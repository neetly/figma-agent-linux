use freetype_sys::{FT_Done_Face, FT_Err_Ok, FT_Face};

pub struct Face {
    pub(crate) raw: FT_Face,
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
