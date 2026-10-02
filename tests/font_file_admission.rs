#![cfg(unix)]

use std::{
    fs,
    os::unix::fs::symlink,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{body::Body, extract::Query, http::Request, response::IntoResponse};
use figma_agent::{
    FONT_FILES,
    font::{FontError, FontFile},
    load_font_file, routes,
    scanner::scan_font_paths,
};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "figma-agent-admission-{}-{nonce}",
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

// A minimal SFNT table directory accepted by the parser, without optional tables.
const FONT: &[u8] = &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn collection(offsets: &[u32]) -> Vec<u8> {
    let mut data = b"ttcf\x00\x01\x00\x00".to_vec();
    data.extend_from_slice(&(offsets.len() as u32).to_be_bytes());
    for offset in offsets {
        data.extend_from_slice(&offset.to_be_bytes());
    }
    data
}

#[tokio::test]
async fn symlinks_only_admit_files_with_readable_fonts() {
    let fixture = Fixture::new();
    let fonts = fixture.0.join("fonts");
    fs::create_dir(&fonts).unwrap();

    let mut partial = collection(&[20, u32::MAX]);
    partial.extend_from_slice(FONT);
    let cases: &[(&str, &[u8], usize)] = &[
        ("private.txt", b"synthetic secret, not a font", 0),
        ("empty", b"", 0),
        ("empty-collection", &collection(&[]), 0),
        ("broken-collection", &collection(&[u32::MAX]), 0),
        ("valid-font", FONT, 1),
        ("partial-collection", &partial, 1),
    ];
    for (name, data, _) in cases {
        let target = fixture.0.join(name);
        fs::write(&target, data).unwrap();
        symlink(&target, fonts.join(format!("{name}.ttf"))).unwrap();
    }

    let paths: Vec<_> = scan_font_paths([&fonts]).collect();
    assert_eq!(paths.len(), cases.len());
    for path in paths {
        if let Some(font_file) = load_font_file(&path) {
            FONT_FILES.write().await.insert(path, font_file);
        }
    }

    for (name, data, expected_fonts) in cases {
        let path = fixture.0.join(name).canonicalize().unwrap();
        if *expected_fonts == 0 {
            assert!(matches!(
                FontFile::from_path(&path),
                Err(FontError::Parse(_, None))
            ));
            assert!(!FONT_FILES.read().await.contains_key(&path));
        } else {
            assert_eq!(FONT_FILES.read().await[&path].fonts.len(), *expected_fonts);
        }
        let response = routes::font_file(
            Query(routes::FontFileQuery { file: path }),
            Request::new(Body::empty()),
        )
        .await
        .into_response();
        assert_eq!(
            response.status().as_u16(),
            if *expected_fonts == 0 { 404 } else { 200 }
        );
        if *expected_fonts != 0 {
            let body = axum::body::to_bytes(response.into_body(), 1024)
                .await
                .unwrap();
            assert_eq!(body.as_ref(), *data);
        }
    }
    FONT_FILES.write().await.clear();
}
