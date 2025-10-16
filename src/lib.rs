use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use arc_swap::ArcSwap;
use fontconfig_parser::FontConfig;

use crate::{
    config::Config,
    font::{FontError, FontFile},
    scanner::scan_font_paths,
};

pub mod config;
pub mod font;
pub mod path;
pub mod payload;
pub mod renderer;
pub mod routes;
pub mod scanner;

pub static XDG_DIRECTORIES: LazyLock<xdg::BaseDirectories> =
    LazyLock::new(|| xdg::BaseDirectories::with_prefix("figma-agent"));

pub static FONTCONFIG: LazyLock<FontConfig> = LazyLock::new(|| {
    let mut font_config = FontConfig::default();
    if let Err(error) = font_config.merge_config("/etc/fonts/fonts.conf") {
        tracing::warn!(
            "Failed to load Fontconfig config file: /etc/fonts/fonts.conf, error: {error:?}"
        );
    }
    font_config
});

pub static CONFIG: LazyLock<ArcSwap<Config>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(load_config())));

pub static FONT_FILES: LazyLock<ArcSwap<HashMap<PathBuf, FontFile>>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(load_font_files())));

#[tracing::instrument]
pub fn load_config() -> Config {
    XDG_DIRECTORIES
        .find_config_file("config.json")
        .and_then(|path| {
            tracing::info!("Use config file: {path:?}");
            match Config::from_path(&path) {
                Ok(config) => {
                    tracing::info!("Use config: {config:?}");
                    Some(config)
                }
                Err(error) => {
                    tracing::error!("Failed to load config file: {path:?}, error: {error:?}");
                    None
                }
            }
        })
        .unwrap_or_else(|| {
            let config = Config::default();
            tracing::info!("Use default config: {config:?}");
            config
        })
}

#[tracing::instrument]
pub fn load_font_files() -> HashMap<PathBuf, FontFile> {
    tracing::debug!("Scanning font files...");
    let directories = CONFIG
        .load()
        .effective_font_directories(&FONTCONFIG)
        .collect::<Vec<_>>();
    tracing::debug!("Use font directories: {directories:?}");
    let font_files = scan_font_paths(directories)
        .filter_map(|path| match FontFile::from_path(&path) {
            Ok(font_file) => Some((path, font_file)),
            Err(FontError::Read(error)) => {
                tracing::debug!("Failed to load font file: {path:?}, error: {error:?}");
                None
            }
            Err(FontError::Parse(errors, font_file)) => {
                for (index, error) in errors {
                    tracing::debug!(
                        "Failed to load font file: {path:?} ({index}), error: {error:?}",
                    );
                }
                font_file.map(|font_file| (path, font_file))
            }
        })
        .collect::<HashMap<_, _>>();
    tracing::debug!("Loaded {count} font files", count = font_files.len());
    font_files
}
