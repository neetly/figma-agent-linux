use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

use crate::Font;

const VERSION: i32 = 2;

pub struct FontCache {
    path: PathBuf,
    data: FontCacheData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FontCacheData {
    version: i32,
    fonts: HashMap<String, FontCacheFontData>,
}

impl Default for FontCacheData {
    fn default() -> Self {
        FontCacheData {
            version: VERSION,
            fonts: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FontCacheFontData {
    time: SystemTime,
    font: Font,
}

impl FontCacheFontData {
    fn new(font: Font) -> FontCacheFontData {
        FontCacheFontData {
            time: SystemTime::now(),
            font,
        }
    }
}

impl FontCache {
    pub fn new<P>(path: P) -> FontCache
    where
        P: AsRef<Path>,
    {
        FontCache {
            path: path.as_ref().to_owned(),
            data: Default::default(),
        }
    }

    pub fn get<P>(&mut self, path: P, index: isize) -> Option<Font>
    where
        P: AsRef<str>,
    {
        let key = format!("{}:{}", path.as_ref(), index);

        if let Some(font_data) = self.data.fonts.get(&key) {
            let modified_time =
                fs::metadata(path.as_ref()).and_then(|metadata| metadata.modified());
            if let Ok(modified_time) = modified_time {
                if modified_time <= font_data.time {
                    return Some(font_data.font.to_owned());
                }
            }
        }

        let font = Font::new(path, index)?;
        let font_data = FontCacheFontData::new(font.to_owned());
        self.data.fonts.insert(key, font_data);
        Some(font)
    }

    pub fn read(&mut self) {
        self.data = self.try_read().unwrap_or_default();
    }

    pub fn write(&self) {
        let _ = self.try_write(&self.data);
    }

    fn try_read(&self) -> Option<FontCacheData> {
        let file = File::open(&self.path).ok()?;
        let data: FontCacheData = serde_json::from_reader(file).ok()?;
        if data.version == VERSION {
            Some(data)
        } else {
            None
        }
    }

    fn try_write(&self, data: &FontCacheData) -> Option<()> {
        let file = File::create(&self.path).ok()?;
        serde_json::to_writer(file, data).ok()
    }
}
