#![allow(clippy::missing_safety_doc, clippy::new_without_default)]

use std::{ffi::CStr, marker::PhantomData, ptr, slice};

use libc::{c_char, c_double, c_int};

pub use fontconfig_sys::*;

pub fn version() -> i32 {
    unsafe { FcGetVersion() }
}

pub struct Config {
    raw: *mut FcConfig,
}

impl Config {
    pub fn new() -> Option<Config> {
        let raw = unsafe { FcConfigCreate() };
        if raw.is_null() {
            None
        } else {
            Some(Config { raw })
        }
    }

    pub unsafe fn from_raw(raw: *mut FcConfig) -> Config {
        Config { raw }
    }

    pub unsafe fn from_raw_with_ref(raw: *mut FcConfig) -> Config {
        let raw = FcConfigReference(raw);
        Config { raw }
    }

    pub fn init() -> Option<Config> {
        let result = unsafe { FcInit() };
        if result == FcTrue {
            Some(unsafe { Config::from_raw(ptr::null_mut()) })
        } else {
            None
        }
    }

    pub fn font_dirs(&self) -> StrList {
        let raw_str_list = unsafe { FcConfigGetFontDirs(self.raw) };
        unsafe { StrList::from_raw(raw_str_list) }
    }

    pub fn list_fonts(&self, pattern: &Pattern, object_set: Option<&ObjectSet>) -> FontSet {
        let raw_pattern = pattern.raw;
        let raw_object_set = object_set
            .map(|object_set| object_set.raw)
            .unwrap_or(ptr::null_mut());
        let raw_font_set = unsafe { FcFontList(self.raw, raw_pattern, raw_object_set) };
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

pub struct Pattern {
    raw: *mut FcPattern,
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

    pub unsafe fn from_raw_with_ref(raw: *mut FcPattern) -> Pattern {
        FcPatternReference(raw);
        Pattern { raw }
    }

    pub fn print(&self) {
        unsafe { FcPatternPrint(self.raw) };
    }

    pub fn get_bool(&self, object: &CStr) -> Option<bool> {
        self.get_bool_nth(object, 0)
    }

    pub fn get_bool_nth(&self, object: &CStr, nth: i32) -> Option<bool> {
        let mut value: FcBool = FcFalse;
        let result = unsafe { FcPatternGetBool(self.raw, object.as_ptr(), nth, &mut value) };
        match result {
            FcResultMatch => Some(value == FcTrue),
            _ => None,
        }
    }

    pub fn get_int(&self, object: &CStr) -> Option<i32> {
        self.get_int_nth(object, 0)
    }

    pub fn get_int_nth(&self, object: &CStr, nth: i32) -> Option<i32> {
        let mut value: c_int = 0;
        let result = unsafe { FcPatternGetInteger(self.raw, object.as_ptr(), nth, &mut value) };
        match result {
            FcResultMatch => Some(value),
            _ => None,
        }
    }

    pub fn get_double(&self, object: &CStr) -> Option<f64> {
        self.get_double_nth(object, 0)
    }

    pub fn get_double_nth(&self, object: &CStr, nth: i32) -> Option<f64> {
        let mut value: c_double = 0.0;
        let result = unsafe { FcPatternGetDouble(self.raw, object.as_ptr(), nth, &mut value) };
        match result {
            FcResultMatch => Some(value),
            _ => None,
        }
    }

    pub fn get_string(&self, object: &CStr) -> Option<&str> {
        self.get_string_nth(object, 0)
    }

    pub fn get_string_nth(&self, object: &CStr, nth: i32) -> Option<&str> {
        let mut value: *mut FcChar8 = ptr::null_mut();
        let result = unsafe { FcPatternGetString(self.raw, object.as_ptr(), nth, &mut value) };
        match result {
            FcResultMatch => unsafe { CStr::from_ptr(value as *const c_char).to_str().ok() },
            _ => None,
        }
    }

    pub fn file(&self) -> Option<&str> {
        self.get_string(FC_FILE)
    }

    pub fn fullname(&self) -> Option<&str> {
        self.get_string(FC_FULLNAME)
    }

    pub fn postscript_name(&self) -> Option<&str> {
        self.get_string(FC_POSTSCRIPT_NAME)
    }

    pub fn family(&self) -> Option<&str> {
        self.get_string(FC_FAMILY)
    }

    pub fn style(&self) -> Option<&str> {
        self.get_string(FC_STYLE)
    }

    pub fn weight(&self) -> Option<i32> {
        self.get_int(FC_WEIGHT)
    }

    pub fn opentype_weight(&self) -> Option<i32> {
        let weight = self.weight()?;
        let opentype_weight = unsafe { FcWeightToOpenType(weight) };
        Some(opentype_weight)
    }

    pub fn slant(&self) -> Option<i32> {
        self.get_int(FC_SLANT)
    }

    pub fn width(&self) -> Option<i32> {
        self.get_int(FC_WIDTH)
    }

    pub fn opentype_width(&self) -> Option<i32> {
        let width = self.width()?;
        let opentype_width = match width {
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
        };
        Some(opentype_width)
    }

    pub fn is_variable(&self) -> Option<bool> {
        self.get_bool(FC_VARIABLE)
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Pattern {
        let raw = unsafe { FcPatternDuplicate(self.raw) };
        unsafe { Pattern::from_raw(raw) }
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe { FcPatternDestroy(self.raw) };
    }
}

pub struct FontSet {
    raw: *mut FcFontSet,
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

    pub fn print(&self) {
        unsafe { FcFontSetPrint(self.raw) };
    }

    pub fn iter(&self) -> impl Iterator<Item = Pattern> {
        let raw_patterns =
            unsafe { slice::from_raw_parts((*self.raw).fonts, (*self.raw).nfont as usize) };
        raw_patterns
            .iter()
            .map(|&raw| unsafe { Pattern::from_raw_with_ref(raw) })
    }
}

impl Drop for FontSet {
    fn drop(&mut self) {
        unsafe { FcFontSetDestroy(self.raw) };
    }
}

pub struct ObjectSet {
    raw: *mut FcObjectSet,
}

impl ObjectSet {
    pub fn new() -> ObjectSet {
        let raw = unsafe { FcObjectSetCreate() };
        assert!(!raw.is_null());
        ObjectSet { raw }
    }

    pub unsafe fn from_raw(raw: *mut FcObjectSet) -> ObjectSet {
        ObjectSet { raw }
    }

    pub fn print(&self) {
        unsafe { FcObjectSetPrint(self.raw) };
    }

    pub fn add(&mut self, object: &CStr) {
        let result = unsafe { FcObjectSetAdd(self.raw, object.as_ptr()) };
        assert!(result == FcTrue);
    }
}

impl Drop for ObjectSet {
    fn drop(&mut self) {
        unsafe { FcObjectSetDestroy(self.raw) };
    }
}

pub struct StrSet {
    raw: *mut FcStrSet,
}

impl StrSet {
    pub fn new() -> Option<StrSet> {
        let raw = unsafe { FcStrSetCreate() };
        if raw.is_null() {
            None
        } else {
            Some(StrSet { raw })
        }
    }

    pub unsafe fn from_raw(raw: *mut FcStrSet) -> StrSet {
        StrSet { raw }
    }
}

impl Drop for StrSet {
    fn drop(&mut self) {
        unsafe { FcStrSetDestroy(self.raw) };
    }
}

pub struct StrList<'a> {
    raw: *mut FcStrList,
    _marker: PhantomData<&'a StrSet>,
}

impl<'a> StrList<'a> {
    pub unsafe fn from_raw(raw: *mut FcStrList) -> StrList<'a> {
        StrList {
            raw,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for StrList<'a> {
    type Item = Option<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let str = unsafe { FcStrListNext(self.raw) };
        if str.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(str as *const c_char).to_str().ok() })
        }
    }
}

impl Drop for StrList<'_> {
    fn drop(&mut self) {
        unsafe { FcStrListDone(self.raw) };
    }
}
