#![allow(clippy::missing_safety_doc)]

use std::ptr;

use fontconfig_sys::{FcConfig, FcConfigCreate, FcConfigDestroy, FcConfigGetFontDirs, FcFontList};

use crate::{FontSet, ObjectSet, Pattern, StrList};

pub struct Config {
    pub(crate) raw: *mut FcConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let raw = unsafe { FcConfigCreate() };
        assert!(!raw.is_null());
        Config { raw }
    }

    pub unsafe fn from_raw(raw: *mut FcConfig) -> Config {
        Config { raw }
    }

    pub fn font_dirs(&self) -> impl Iterator<Item = Option<&str>> {
        let raw_str_list = unsafe { FcConfigGetFontDirs(self.raw) };
        assert!(!raw_str_list.is_null());
        unsafe { StrList::from_raw(raw_str_list) }
    }

    pub fn fonts(&self, pattern: &Pattern, object_set: Option<&ObjectSet>) -> FontSet {
        let raw_font_set = unsafe {
            FcFontList(
                self.raw,
                pattern.raw,
                object_set
                    .map(|object_set| object_set.raw)
                    .unwrap_or(ptr::null_mut()),
            )
        };
        assert!(!raw_font_set.is_null());
        unsafe { FontSet::from_raw(raw_font_set) }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { FcConfigDestroy(self.raw) };
    }
}
