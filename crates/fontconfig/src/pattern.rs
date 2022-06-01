#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::CStr,
    hash::{Hash, Hasher},
    ptr,
};

use fontconfig_sys::{
    FcChar8, FcFalse, FcPattern, FcPatternCreate, FcPatternDestroy, FcPatternDuplicate,
    FcPatternEqual, FcPatternGetBool, FcPatternGetDouble, FcPatternGetFTFace, FcPatternGetInteger,
    FcPatternGetString, FcPatternHash, FcResultMatch, FcWeightToOpenType, FC_FAMILY, FC_FILE,
    FC_FT_FACE, FC_FULLNAME, FC_POSTSCRIPT_NAME, FC_SLANT, FC_STYLE, FC_VARIABLE, FC_WEIGHT,
    FC_WIDTH, FC_WIDTH_CONDENSED, FC_WIDTH_EXPANDED, FC_WIDTH_EXTRACONDENSED,
    FC_WIDTH_EXTRAEXPANDED, FC_WIDTH_NORMAL, FC_WIDTH_SEMICONDENSED, FC_WIDTH_SEMIEXPANDED,
    FC_WIDTH_ULTRACONDENSED, FC_WIDTH_ULTRAEXPANDED,
};
use libc::{c_double, c_int};

pub struct Pattern {
    pub(crate) raw: *mut FcPattern,
}

impl Default for Pattern {
    fn default() -> Self {
        Self::new()
    }
}

impl Pattern {
    pub fn new() -> Pattern {
        let raw = unsafe { FcPatternCreate() };
        assert!(!raw.is_null());
        Pattern { raw }
    }

    pub unsafe fn from_raw(raw: *mut FcPattern) -> Pattern {
        Pattern { raw }
    }

    pub fn get_bool(&self, object: &[u8]) -> Option<bool> {
        self.get_bool_at(object, 0)
    }

    pub fn get_bool_at(&self, object: &[u8], n: usize) -> Option<bool> {
        let mut value: c_int = Default::default();
        let result =
            unsafe { FcPatternGetBool(self.raw, object.as_ptr() as _, n as _, &mut value) };
        if result == FcResultMatch {
            Some(value != FcFalse)
        } else {
            None
        }
    }

    pub fn get_i32(&self, object: &[u8]) -> Option<i32> {
        self.get_i32_at(object, 0)
    }

    pub fn get_i32_at(&self, object: &[u8], n: usize) -> Option<i32> {
        let mut value: c_int = Default::default();
        let result =
            unsafe { FcPatternGetInteger(self.raw, object.as_ptr() as _, n as _, &mut value) };
        if result == FcResultMatch {
            Some(value as _)
        } else {
            None
        }
    }

    pub fn get_f64(&self, object: &[u8]) -> Option<f64> {
        self.get_f64_at(object, 0)
    }

    pub fn get_f64_at(&self, object: &[u8], n: usize) -> Option<f64> {
        let mut value: c_double = Default::default();
        let result =
            unsafe { FcPatternGetDouble(self.raw, object.as_ptr() as _, n as _, &mut value) };
        if result == FcResultMatch {
            Some(value as _)
        } else {
            None
        }
    }

    pub fn get_str(&self, object: &[u8]) -> Option<&str> {
        self.get_str_at(object, 0)
    }

    pub fn get_str_at(&self, object: &[u8], n: usize) -> Option<&str> {
        let mut value: *mut FcChar8 = ptr::null_mut();
        let result =
            unsafe { FcPatternGetString(self.raw, object.as_ptr() as _, n as _, &mut value) };
        if result == FcResultMatch {
            unsafe { CStr::from_ptr(value as _) }.to_str().ok()
        } else {
            None
        }
    }

    pub fn get_freetype_face(&self, object: &[u8]) -> Option<freetype::Face> {
        self.get_freetype_face_at(object, 0)
    }

    pub fn get_freetype_face_at(&self, object: &[u8], n: usize) -> Option<freetype::Face> {
        let mut value: freetype::FT_Face = ptr::null_mut();
        let result =
            unsafe { FcPatternGetFTFace(self.raw, object.as_ptr() as _, n as _, &mut value) };
        if result == FcResultMatch {
            Some(unsafe { freetype::Face::from_raw(value) })
        } else {
            None
        }
    }

    pub fn file(&self) -> Option<&str> {
        self.get_str(FC_FILE)
    }

    pub fn fullname(&self) -> Option<&str> {
        self.get_str(FC_FULLNAME)
    }

    pub fn family(&self) -> Option<&str> {
        self.get_str(FC_FAMILY)
    }

    pub fn style(&self) -> Option<&str> {
        self.get_str(FC_STYLE)
    }

    pub fn postscript_name(&self) -> Option<&str> {
        self.get_str(FC_POSTSCRIPT_NAME)
    }

    pub fn weight(&self) -> Option<i32> {
        self.get_i32(FC_WEIGHT)
    }

    pub fn weight_opentype(&self) -> Option<i32> {
        let weight = self.weight()?;
        Some(unsafe { FcWeightToOpenType(weight) })
    }

    pub fn slant(&self) -> Option<i32> {
        self.get_i32(FC_SLANT)
    }

    pub fn width(&self) -> Option<i32> {
        self.get_i32(FC_WIDTH)
    }

    pub fn width_opentype(&self) -> Option<i32> {
        let width = self.width()?;
        Some(match width {
            FC_WIDTH_ULTRACONDENSED => 1,
            FC_WIDTH_EXTRACONDENSED => 2,
            FC_WIDTH_CONDENSED => 3,
            FC_WIDTH_SEMICONDENSED => 4,
            FC_WIDTH_NORMAL => 5,
            FC_WIDTH_SEMIEXPANDED => 6,
            FC_WIDTH_EXPANDED => 7,
            FC_WIDTH_EXTRAEXPANDED => 8,
            FC_WIDTH_ULTRAEXPANDED => 9,
            _ => return None,
        })
    }

    pub fn is_variable(&self) -> Option<bool> {
        self.get_bool(FC_VARIABLE)
    }

    pub fn freetype_face(&self) -> Option<freetype::Face> {
        self.get_freetype_face(FC_FT_FACE)
    }
}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = unsafe { FcPatternHash(self.raw) };
        state.write_u32(hash);
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        let result = unsafe { FcPatternEqual(self.raw, other.raw) };
        result != FcFalse
    }
}

impl Eq for Pattern {}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        let raw = unsafe { FcPatternDuplicate(self.raw) };
        assert!(!raw.is_null());
        Pattern { raw }
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe { FcPatternDestroy(self.raw) };
    }
}