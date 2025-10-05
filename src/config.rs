use std::{
    fs, io, iter,
    path::{Path, PathBuf},
};

use fontconfig_parser::FontConfig;
use itertools::{Either, Itertools};

use crate::path::expand_home;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    Read(#[from] io::Error),
    #[error("Failed to parse config file")]
    Parse(#[from] jsonc_parser::errors::ParseError),
    #[error("Failed to parse config file")]
    Deserialize(#[from] serde_json::Error),
    #[error("Failed to parse config file")]
    Invalid,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_bool::<true>")]
    pub use_system_fonts: bool,
    #[serde(default)]
    pub font_directories: Vec<PathBuf>,
}

fn default_bind() -> String {
    "127.0.0.1:44950".into()
}

fn default_bool<const V: bool>() -> bool {
    V
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_value(serde_json::json!({})).unwrap()
    }
}

impl Config {
    pub fn parse(text: impl AsRef<str>) -> Result<Self, ConfigError> {
        let value = jsonc_parser::parse_to_serde_value(
            text.as_ref(),
            &jsonc_parser::ParseOptions::default(),
        )?;
        let value = value.ok_or(ConfigError::Invalid)?;
        let config = serde_json::from_value(value)?;
        Ok(config)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let text = fs::read_to_string(path)?;
        Config::parse(text)
    }
}

#[test]
fn test_default() {
    assert_eq!(
        Config::default(),
        Config {
            bind: "127.0.0.1:44950".into(),
            use_system_fonts: true,
            font_directories: vec![],
        },
    );
}

#[test]
fn test_parse() {
    assert_eq!(Config::parse("{}").unwrap(), Config::default());
    assert_eq!(Config::parse("{} // comment").unwrap(), Config::default());
    assert_eq!(
        Config::parse(
            r#"{ "bind": "0.0.0.0:44950", "use_system_fonts": false, "font_directories": ["/usr/share/fonts"] }"#,
        )
        .unwrap(),
        Config {
            bind: "0.0.0.0:44950".into(),
            use_system_fonts: false,
            font_directories: vec![PathBuf::from("/usr/share/fonts")],
        },
    );
}

impl Config {
    pub fn effective_font_directories(
        &self,
        fontconfig: &FontConfig,
    ) -> impl Iterator<Item = PathBuf> {
        self.font_directories
            .iter()
            .filter_map(|directory| match expand_home(directory) {
                Ok(directory) => Some(directory),
                Err(error) => {
                    tracing::debug!("Skipped font directory: {directory:?}, error: {error:?}");
                    None
                }
            })
            .chain(if self.use_system_fonts {
                Either::Left(fontconfig.dirs.iter().map(|dir| dir.path.clone()))
            } else {
                Either::Right(iter::empty())
            })
            .filter_map(|directory| match directory.canonicalize() {
                Ok(directory) => Some(directory),
                Err(error) => {
                    tracing::debug!("Skipped font directory: {directory:?}, error: {error:?}");
                    None
                }
            })
            .unique()
    }
}
