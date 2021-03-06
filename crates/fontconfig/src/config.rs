#![allow(clippy::missing_safety_doc)]

use std::ptr;

use fontconfig_sys::{FcConfig, FcConfigCreate, FcConfigDestroy, FcConfigGetFontDirs, FcFontList};

use crate::{FontSet, ObjectSet, Pattern, StrList};

pub struct Config {
    raw: *mut FcConfig,
}

unsafe impl Send for Config {}
unsafe impl Sync for Config {}

impl Config {
    pub unsafe fn raw(&self) -> *mut FcConfig {
        self.raw
    }
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

    pub fn font_dirs(&self) -> StrList {
        let raw_str_list = unsafe { FcConfigGetFontDirs(self.raw) };
        assert!(!raw_str_list.is_null());
        unsafe { StrList::from_raw(raw_str_list) }
    }

    pub fn list_fonts(&self, pattern: &Pattern, object_set: Option<&ObjectSet>) -> FontSet {
        let raw_font_set = unsafe {
            FcFontList(
                self.raw,
                pattern.raw(),
                object_set
                    .map(|object_set| object_set.raw())
                    .unwrap_or(ptr::null_mut()),
            )
        };
        assert!(!raw_font_set.is_null());
        unsafe { FontSet::from_raw(raw_font_set) }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { FcConfigDestroy(self.raw) };
        }
    }
}
