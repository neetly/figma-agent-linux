use std::{
    collections::HashSet,
    iter,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use walkdir::{DirEntry, WalkDir};

const FONT_EXTENSIONS: [&str; 4] = ["ttf", "ttc", "otf", "otc"];

fn walk_font_directories(
    directories: impl IntoIterator<Item = impl AsRef<Path>>,
) -> impl Iterator<Item = Result<DirEntry, walkdir::Error>> {
    let mut walkers = directories
        .into_iter()
        .map(|directory| WalkDir::new(directory).follow_links(true).into_iter())
        .peekable();
    // Share directory identities across roots, but start fresh for each scan.
    let mut visited = HashSet::new();

    iter::from_fn(move || {
        while let Some(walker) = walkers.peek_mut() {
            let Some(entry) = walker.next() else {
                walkers.next();
                continue;
            };
            if let Ok(entry) = &entry
                && entry.file_type().is_dir()
            {
                match entry.metadata() {
                    Ok(metadata) => {
                        if !visited.insert((metadata.dev(), metadata.ino())) {
                            walker.skip_current_dir();
                            continue;
                        }
                    }
                    Err(error) => {
                        walker.skip_current_dir();
                        return Some(Err(error));
                    }
                }
            }
            return Some(entry);
        }
        None
    })
}

pub fn scan_font_paths(
    directories: impl IntoIterator<Item = impl AsRef<Path>>,
) -> impl Iterator<Item = PathBuf> {
    walk_font_directories(directories)
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(error) => {
                tracing::debug!(
                    path = %error.path().unwrap_or_else(|| Path::new("<unknown>")).display(),
                    %error,
                    "Could not scan font path; skipping it"
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
                    path = %entry.path().display(),
                    %error,
                    "Could not resolve font path; skipping it"
                );
                None
            }
        })
        .unique()
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        os::unix::fs::symlink,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "figma-agent-scanner-{}-{nonce}",
                std::process::id(),
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn shared_directory_graph_is_traversed_once_across_roots() {
        let fixture = Fixture::new();
        for level in 0..=12 {
            fs::create_dir(fixture.0.join(level.to_string())).unwrap();
        }
        for level in 0..12 {
            for name in ["a", "b"] {
                symlink(
                    format!("../{}", level + 1),
                    fixture.0.join(level.to_string()).join(name),
                )
                .unwrap();
            }
        }
        let font = fixture.0.join("12/font.ttf");
        fs::write(&font, b"scanner fixture").unwrap();
        let roots = [
            fixture.0.join("0"),
            fixture.0.join("6"),
            fixture.0.join("0"),
        ];

        // Count traversal entries, not the final deduplicated font list:
        // 13 physical directories and one file, despite 4096 paths to the file.
        let entries: Vec<_> = walk_font_directories(&roots)
            .take(100)
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(entries.len(), 14);
        assert_eq!(
            entries
                .iter()
                .filter(|entry| entry.file_type().is_file())
                .count(),
            1
        );
        assert_eq!(
            scan_font_paths(&roots).collect::<Vec<_>>(),
            [font.canonicalize().unwrap()]
        );

        let added = fixture.0.join("12/added.otf");
        fs::write(&added, b"new scanner fixture").unwrap();
        let paths: HashSet<_> = scan_font_paths(&roots).collect();
        assert_eq!(
            paths,
            HashSet::from([font.canonicalize().unwrap(), added.canonicalize().unwrap()])
        );
    }

    #[test]
    fn links_outside_root_survive_cycles_and_broken_links() {
        let fixture = Fixture::new();
        let root = fixture.0.join("fonts");
        let outside = fixture.0.join("outside");
        fs::create_dir(&root).unwrap();
        fs::create_dir(&outside).unwrap();
        let font = outside.join("font.ttf");
        fs::write(&font, b"scanner fixture").unwrap();
        symlink(&outside, root.join("directory")).unwrap();
        symlink(&font, root.join("alias.TTF")).unwrap();
        symlink(&root, root.join("self")).unwrap();
        symlink(&root, outside.join("back")).unwrap();
        symlink(root.join("missing"), root.join("broken.ttf")).unwrap();

        assert_eq!(
            scan_font_paths([fixture.0.join("missing-root"), root]).collect::<Vec<_>>(),
            [font.canonicalize().unwrap()],
        );
    }
}
