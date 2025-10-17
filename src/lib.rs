use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use fontconfig_parser::FontConfig;
use tokio::sync::RwLock;

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

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
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
});

pub static EFFECTIVE_FONT_DIRECTORIES: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let directories = CONFIG.effective_font_directories(&FONTCONFIG).collect();
    tracing::info!("Use effective font directories: {directories:?}");
    directories
});

pub static FONT_FILES: LazyLock<RwLock<HashMap<PathBuf, FontFile>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[tracing::instrument]
pub async fn scan_font_files() {
    tracing::debug!("Scanning font files...");

    let mut font_files = FONT_FILES.write().await;

    let (mut added_count, mut updated_count, mut removed_count) = (0, 0, 0);
    let mut font_paths = scan_font_paths(&*EFFECTIVE_FONT_DIRECTORIES).collect::<HashSet<_>>();

    font_files.retain(|path, _| {
        let contains = font_paths.contains(path);
        if !contains {
            removed_count += 1;
        }
        contains
    });

    font_paths.retain(|path| {
        if let Some(font_file) = font_files.get(path) {
            let modified_at = fs::metadata(path)
                .and_then(|metadata| metadata.modified())
                .ok();
            modified_at > font_file.modified_at
        } else {
            true
        }
    });

    for path in font_paths {
        if let Some(font_file) = load_font_file(&path) {
            if font_files.insert(path, font_file).is_none() {
                added_count += 1;
            } else {
                updated_count += 1;
            }
        }
    }

    tracing::debug!(
        "{count} font files loaded ({added_count} added, {updated_count} updated, {removed_count} removed)",
        count = font_files.len(),
    );
}

pub fn load_font_file(path: impl AsRef<Path>) -> Option<FontFile> {
    let path = path.as_ref();

    match FontFile::from_path(path) {
        Ok(font_file) => Some(font_file),
        Err(FontError::Read(error)) => {
            tracing::debug!("Failed to load font file: {path:?}, error: {error:?}");
            None
        }
        Err(FontError::Parse(errors, font_file)) => {
            for (index, error) in errors {
                tracing::debug!("Failed to load font file: {path:?} ({index}), error: {error:?}",);
            }
            font_file
        }
    }
}
