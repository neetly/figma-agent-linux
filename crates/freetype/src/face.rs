#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{CStr, CString},
    ptr,
};

use freetype_sys::{
    FT_Done_Face, FT_Err_Ok, FT_Face, FT_Get_Postscript_Name, FT_Get_Sfnt_Name,
    FT_Get_Sfnt_Name_Count, FT_New_Face,
};

use crate::{Library, MMVar, SfntName};

pub struct Face<'a> {
    raw: FT_Face,
    library: &'a Library,
}

impl Face<'_> {
    pub unsafe fn raw(&self) -> FT_Face {
        self.raw
    }
}

impl Face<'_> {
    pub fn from_file<P>(library: &Library, path: P, face_index: isize) -> Option<Face>
    where
        P: AsRef<str>,
    {
        let mut raw = ptr::null_mut();
        let path = CString::new(path.as_ref()).ok()?;
        let result =
            unsafe { FT_New_Face(library.raw(), path.as_ptr(), face_index as _, &mut raw) };
        if result == FT_Err_Ok {
            Some(Face { raw, library })
        } else {
            None
        }
    }

    pub unsafe fn from_raw(raw: FT_Face, library: &Library) -> Face {
        Face { raw, library }
    }
}

impl Drop for Face<'_> {
    fn drop(&mut self) {
        let result = unsafe { FT_Done_Face(self.raw) };
        assert!(result == FT_Err_Ok);
    }
}

impl Face<'_> {
    pub fn postscript_name(&self) -> Option<&str> {
        let raw_postscript_name = unsafe { FT_Get_Postscript_Name(self.raw) };
        if !raw_postscript_name.is_null() {
            unsafe { CStr::from_ptr(raw_postscript_name) }.to_str().ok()
        } else {
            None
        }
    }

    pub fn family_name(&self) -> Option<&str> {
        let raw_family_name = unsafe { (*self.raw).family_name };
        if !raw_family_name.is_null() {
            unsafe { CStr::from_ptr(raw_family_name) }.to_str().ok()
        } else {
            None
        }
    }

    pub fn style_name(&self) -> Option<&str> {
        let raw_style_name = unsafe { (*self.raw).style_name };
        if !raw_style_name.is_null() {
            unsafe { CStr::from_ptr(raw_style_name) }.to_str().ok()
        } else {
            None
        }
    }

    pub fn find_sfnt_name<P>(&self, mut predicate: P) -> Option<SfntName>
    where
        P: FnMut(&SfntName) -> bool,
    {
        let mut sfnt_name = unsafe { SfntName::new() };
        let count = unsafe { FT_Get_Sfnt_Name_Count(self.raw) };
        for index in 0..count {
            let result = unsafe { FT_Get_Sfnt_Name(self.raw, index, sfnt_name.as_mut()) };
            if result == FT_Err_Ok && predicate(&sfnt_name) {
                return Some(sfnt_name);
            }
        }
        None
    }

    pub fn mm_var(&self) -> Option<MMVar> {
        MMVar::from_face(self, self.library)
    }
}
