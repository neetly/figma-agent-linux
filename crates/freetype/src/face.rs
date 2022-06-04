#![allow(clippy::missing_safety_doc)]

use std::{ffi::CString, ptr};

use freetype_sys::{FT_Done_Face, FT_Err_Ok, FT_Face, FT_New_Face};

use crate::Library;

pub struct Face {
    raw: FT_Face,
}

impl Face {
    pub fn raw(&self) -> FT_Face {
        self.raw
    }
}

impl Face {
    pub fn from_file<P>(library: &Library, path: P, face_index: i32) -> Option<Face>
    where
        P: AsRef<str>,
    {
        let mut raw = ptr::null_mut();
        let path = CString::new(path.as_ref()).ok()?;
        let result =
            unsafe { FT_New_Face(library.raw(), path.as_ptr(), face_index as _, &mut raw) };
        if result == FT_Err_Ok {
            Some(Face { raw })
        } else {
            None
        }
    }

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
