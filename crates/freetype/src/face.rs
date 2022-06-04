#![allow(clippy::missing_safety_doc)]

use freetype_sys::{FT_Done_Face, FT_Err_Ok, FT_Face};

pub struct Face {
    raw: FT_Face,
}

impl Face {
    pub fn raw(&self) -> FT_Face {
        self.raw
    }
}

impl Face {
    pub unsafe fn from_raw(raw: FT_Face) -> Face {
        Face { raw }
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        let result = unsafe { FT_Done_Face(self.raw) };
        assert!(result == FT_Err_Ok);
    }
}
