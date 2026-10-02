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
    font::{FileFingerprint, FontError, FontFile},
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

    rescan_font_files(&mut font_files, &EFFECTIVE_FONT_DIRECTORIES);
}

fn rescan_font_files(font_files: &mut HashMap<PathBuf, FontFile>, directories: &[PathBuf]) {
    let (mut added_count, mut updated_count, mut removed_count) = (0, 0, 0);
    let mut font_paths = scan_font_paths(directories).collect::<HashSet<_>>();

    font_files.retain(|path, _| {
        let contains = font_paths.contains(path);
        if !contains {
            removed_count += 1;
        }
        contains
    });

    font_paths.retain(|path| {
        if let Some(font_file) = font_files.get(path) {
            fs::metadata(path)
                .map(|metadata| FileFingerprint::from(&metadata) != font_file.fingerprint)
                .unwrap_or(true)
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
        } else if font_files.remove(&path).is_some() {
            removed_count += 1;
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
            font_file.map(|font_file| *font_file)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::*;

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir()
                .join(format!("figma-agent-rescan-{}-{nonce}", std::process::id()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    // Minimal SFNT table directory accepted by the parser.
    const FONT: &[u8] = &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    #[test]
    fn rescan_removes_replaced_fonts_even_without_a_newer_mtime() {
        let fixture = Fixture::new();
        let path = fixture.0.join("font.ttf");
        let mut files = HashMap::new();
        for case in [
            "older",
            "same-time-larger",
            "same-time-same-size",
            "new-inode",
        ] {
            fs::write(&path, FONT).unwrap();
            files.clear();
            rescan_font_files(&mut files, std::slice::from_ref(&fixture.0));
            assert_eq!(files.len(), 1);
            let original = files[&path].modified_at.unwrap();
            let timestamp = if case == "older" {
                original - Duration::from_secs(3600)
            } else {
                original
            };
            let replacement = if case == "new-inode" {
                fixture.0.join("replacement")
            } else {
                path.clone()
            };
            // Ensure ctime differs even on filesystems with coarse clock resolution.
            if case == "same-time-same-size" {
                std::thread::sleep(Duration::from_millis(1100));
            }
            let size = FONT.len() + usize::from(case == "same-time-larger");
            fs::write(&replacement, vec![0xff; size]).unwrap();
            fs::File::options()
                .write(true)
                .open(&replacement)
                .unwrap()
                .set_modified(timestamp)
                .unwrap();
            if case == "new-inode" {
                fs::rename(&replacement, &path).unwrap();
            }
            rescan_font_files(&mut files, std::slice::from_ref(&fixture.0));
            assert!(files.is_empty(), "{case}");
        }
    }

    #[test]
    fn rescan_discovers_directories_created_after_configuration() {
        let fixture = Fixture::new();
        let directory = fixture.0.join("Fonts");
        let config = Config {
            use_system_fonts: false,
            font_directories: vec![directory.clone()],
            ..Config::default()
        };
        let directories: Vec<_> = config
            .effective_font_directories(&FontConfig::default())
            .collect();
        assert_eq!(directories, [directory.clone()]);
        let mut files = HashMap::new();
        rescan_font_files(&mut files, &directories);
        assert!(files.is_empty());
        fs::create_dir(&directory).unwrap();
        let path = directory.join("font.ttf");
        fs::write(&path, FONT).unwrap();
        rescan_font_files(&mut files, &directories);
        assert!(files.contains_key(&path));
    }
}
