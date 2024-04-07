#![allow(clippy::missing_safety_doc)]

use std::ptr;

use freetype_sys::{FT_Done_Library, FT_Err_Ok, FT_Library, FT_New_Library};

use crate::{Face, MEMORY};

pub struct Library {
    raw: FT_Library,
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Library {
    pub unsafe fn raw(&self) -> FT_Library {
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
        let mut raw = ptr::null_mut();
        #[allow(static_mut_refs)]
        let result = unsafe { FT_New_Library(&mut MEMORY, &mut raw) };
        assert!(result == FT_Err_Ok);
        Library { raw }
    }

    pub unsafe fn from_raw(raw: FT_Library) -> Library {
        Library { raw }
    }

    pub fn face_from_file<P>(&self, path: P, face_index: isize) -> Option<Face>
    where
        P: AsRef<str>,
    {
        Face::from_file(self, path, face_index)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        let result = unsafe { FT_Done_Library(self.raw) };
        assert!(result == FT_Err_Ok);
    }
}
