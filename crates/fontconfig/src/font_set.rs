#![allow(clippy::missing_safety_doc)]

use std::slice;

use fontconfig_sys::{FcFontSet, FcFontSetCreate, FcFontSetDestroy, FcPatternReference};

use crate::Pattern;

pub struct FontSet {
    pub(crate) raw: *mut FcFontSet,
}

impl Default for FontSet {
    fn default() -> Self {
        Self::new()
    }
}

impl FontSet {
    pub fn new() -> FontSet {
        let raw = unsafe { FcFontSetCreate() };
        assert!(!raw.is_null());
        FontSet { raw }
    }

    pub unsafe fn from_raw(raw: *mut FcFontSet) -> FontSet {
        FontSet { raw }
    }

    pub fn iter(&self) -> impl Iterator<Item = Pattern> {
        let slice = unsafe { slice::from_raw_parts((*self.raw).fonts, (*self.raw).nfont as _) };
        slice.iter().map(|&raw_pattern| unsafe {
            FcPatternReference(raw_pattern);
            Pattern::from_raw(raw_pattern)
        })
    }
}

impl Drop for FontSet {
    fn drop(&mut self) {
        unsafe { FcFontSetDestroy(self.raw) }
    }
}
