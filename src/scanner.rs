use std::path::{Path, PathBuf};

use itertools::Itertools;
use walkdir::WalkDir;

pub fn scan_font_paths(
    directories: impl IntoIterator<Item = impl AsRef<Path>>,
) -> impl Iterator<Item = PathBuf> {
    static FONT_EXTENSIONS: [&str; 4] = ["ttf", "ttc", "otf", "otc"];

    directories
        .into_iter()
        .flat_map(|directory| WalkDir::new(directory))
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(error) => {
                tracing::debug!(
                    "Skipped font file/directory: {path:?}, error: {error:?}",
                    path = error.path().unwrap_or_else(|| Path::new("<unknown>")),
                );
                None
            }
        })
        .filter(|entry| {
            entry.file_type().is_file()
                && match entry.path().extension() {
                    Some(extension) => FONT_EXTENSIONS
                        .iter()
                        .any(|item| extension.eq_ignore_ascii_case(item)),
                    None => false,
                }
        })
        .filter_map(|entry| match entry.path().canonicalize() {
            Ok(path) => Some(path),
            Err(error) => {
                tracing::debug!(
                    "Skipped font file: {path:?}, error: {error:?}",
                    path = entry.path(),
                );
                None
            }
        })
        .unique()
}
