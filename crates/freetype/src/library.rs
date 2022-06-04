#![allow(clippy::missing_safety_doc)]

use std::ptr;

use freetype_sys::{FT_Done_Library, FT_Err_Ok, FT_Library, FT_New_Library};

use crate::MEMORY;

pub struct Library {
    raw: FT_Library,
}

impl Library {
    pub fn raw(&self) -> FT_Library {
        self.raw
    }
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

impl Library {
    pub fn new() -> Library {
        let mut raw: FT_Library = ptr::null_mut();
        let result = unsafe { FT_New_Library(&mut MEMORY, &mut raw) };
        assert!(result == FT_Err_Ok);
        Library { raw }
    }

    pub unsafe fn from_raw(raw: FT_Library) -> Library {
        Library { raw }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        let result = unsafe { FT_Done_Library(self.raw) };
        assert!(result == FT_Err_Ok);
    }
}
