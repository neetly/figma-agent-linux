#![allow(clippy::missing_safety_doc)]

use std::{ptr, slice};

use freetype_sys::{FT_Done_MM_Var, FT_Err_Ok, FT_Get_MM_Var, FT_MM_Var};

use crate::{Face, Library, VarAxis, VarNamedStyle};

pub struct MMVar<'a> {
    raw: *mut FT_MM_Var,
    library: &'a Library,
}

impl MMVar<'_> {
    pub unsafe fn raw(&self) -> *mut FT_MM_Var {
        self.raw
    }
}

impl<'a> MMVar<'a> {
    pub fn from_face(face: &Face, library: &'a Library) -> Option<MMVar<'a>> {
        let mut raw = ptr::null_mut();
        let result = unsafe { FT_Get_MM_Var(face.raw(), &mut raw) };
        if result == FT_Err_Ok {
            Some(MMVar { raw, library })
        } else {
            None
        }
    }

    pub unsafe fn from_raw(raw: *mut FT_MM_Var, library: &Library) -> MMVar {
        MMVar { raw, library }
    }

    pub fn axis_count(&self) -> usize {
        unsafe { (*self.raw).num_axis as _ }
    }

    pub fn named_style_count(&self) -> usize {
        unsafe { (*self.raw).num_namedstyles as _ }
    }

    pub fn axes(&self) -> impl Iterator<Item = VarAxis> {
        let slice = unsafe { slice::from_raw_parts((*self.raw).axis, (*self.raw).num_axis as _) };
        slice
            .iter()
            .enumerate()
            .map(|(index, item)| VarAxis::new(item, self, index))
    }

    pub fn named_styles(&self) -> impl Iterator<Item = VarNamedStyle> {
        let slice = unsafe {
            slice::from_raw_parts((*self.raw).namedstyle, (*self.raw).num_namedstyles as _)
        };
        slice.iter().map(|item| VarNamedStyle::new(item, self))
    }
}

impl Drop for MMVar<'_> {
    fn drop(&mut self) {
        let result = unsafe { FT_Done_MM_Var(self.library.raw(), self.raw) };
        assert!(result == FT_Err_Ok);
    }
}
