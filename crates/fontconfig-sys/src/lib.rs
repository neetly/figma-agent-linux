#![allow(non_upper_case_globals)]

use std::{
    ffi::CStr,
    os::raw::{c_char, c_double, c_int, c_uchar, c_uint, c_ushort},
};

macro_rules! cstr {
    ($($(#[$meta:meta])* $vis:vis const $name:ident = $str:literal;)*) => {
        $(
            $(#[$meta])*
            $vis const $name: &CStr =
                unsafe { CStr::from_bytes_with_nul_unchecked(concat!($str, "\0").as_bytes()) };
        )*
    };
}

macro_rules! opaque {
    ($($(#[$meta:meta])* $vis:vis struct $name:ident;)*) => {
        $(
            $(#[$meta])*
            $vis struct $name {
                _data: [u8; 0],
                _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
            }
        )*
    };
}

pub type FcChar8 = c_uchar;
pub type FcChar16 = c_ushort;
pub type FcChar32 = c_uint;

pub type FcBool = c_int;
pub const FcFalse: c_int = 0;
pub const FcTrue: c_int = 1;
pub const FcDontCare: c_int = 2;

cstr! {
    pub const FC_FILE = "file";
    pub const FC_FULLNAME = "fullname";
    pub const FC_POSTSCRIPT_NAME = "postscriptname";
    pub const FC_FAMILY = "family";
    pub const FC_STYLE = "style";
    pub const FC_WEIGHT = "weight";
    pub const FC_SLANT = "slant";
    pub const FC_WIDTH = "width";
}

pub const FC_SLANT_ROMAN: c_int = 0;
pub const FC_SLANT_ITALIC: c_int = 100;
pub const FC_SLANT_OBLIQUE: c_int = 110;

pub const FC_WIDTH_ULTRACONDENSED: c_int = 50;
pub const FC_WIDTH_EXTRACONDENSED: c_int = 63;
pub const FC_WIDTH_CONDENSED: c_int = 75;
pub const FC_WIDTH_SEMICONDENSED: c_int = 87;
pub const FC_WIDTH_NORMAL: c_int = 100;
pub const FC_WIDTH_SEMIEXPANDED: c_int = 113;
pub const FC_WIDTH_EXPANDED: c_int = 125;
pub const FC_WIDTH_EXTRAEXPANDED: c_int = 150;
pub const FC_WIDTH_ULTRAEXPANDED: c_int = 200;

opaque! {
    #[repr(C)]
    pub struct FcConfig;

    #[repr(C)]
    pub struct FcPattern;
}

#[repr(C)]
pub struct FcFontSet {
    pub nfont: c_int,
    pub sfont: c_int,
    pub fonts: *mut *mut FcPattern,
}

#[repr(C)]
pub struct FcObjectSet {
    pub nobject: c_int,
    pub sobject: c_int,
    pub objects: *mut *const c_char,
}

#[repr(C)]
pub enum FcResult {
    FcResultMatch,
    FcResultNoMatch,
    FcResultTypeMismatch,
    FcResultNoId,
    FcResultOutOfMemory,
}

pub use FcResult::*;

#[link(name = "fontconfig")]
extern "C" {
    pub fn FcGetVersion() -> c_int;

    pub fn FcInit() -> FcBool;
    pub fn FcInitLoadConfig() -> *mut FcConfig;
    pub fn FcInitLoadConfigAndFonts() -> *mut FcConfig;

    pub fn FcPatternCreate() -> *mut FcPattern;
    pub fn FcPatternDuplicate(pattern: *const FcPattern) -> *mut FcPattern;
    pub fn FcPatternReference(pattern: *mut FcPattern);
    pub fn FcPatternDestroy(pattern: *mut FcPattern);
    pub fn FcPatternPrint(pattern: *mut FcPattern);

    pub fn FcPatternGetInteger(
        pattern: *mut FcPattern,
        object: *const c_char,
        nth: c_int,
        value: *mut c_int,
    ) -> FcResult;
    pub fn FcPatternGetDouble(
        pattern: *mut FcPattern,
        object: *const c_char,
        nth: c_int,
        value: *mut c_double,
    ) -> FcResult;
    pub fn FcPatternGetString(
        pattern: *mut FcPattern,
        object: *const c_char,
        nth: c_int,
        value: *mut *mut FcChar8,
    ) -> FcResult;

    pub fn FcFontSetCreate() -> *mut FcFontSet;
    pub fn FcFontSetReference(font_set: *mut FcFontSet);
    pub fn FcFontSetDestroy(font_set: *mut FcFontSet);
    pub fn FcFontSetPrint(font_set: *mut FcFontSet);

    pub fn FcObjectSetCreate() -> *mut FcObjectSet;
    pub fn FcObjectSetReference(object_set: *mut FcObjectSet);
    pub fn FcObjectSetDestroy(object_set: *mut FcObjectSet);
    pub fn FcObjectSetPrint(object_set: *mut FcObjectSet);
    pub fn FcObjectSetAdd(object_set: *mut FcObjectSet, object: *const c_char) -> FcBool;

    pub fn FcFontList(
        config: *mut FcConfig,
        pattern: *mut FcPattern,
        object_set: *mut FcObjectSet,
    ) -> *mut FcFontSet;

    pub fn FcWeightFromOpenType(weight: c_int) -> c_int;
    pub fn FcWeightFromOpenTypeDouble(weight: c_double) -> c_double;
    pub fn FcWeightToOpenType(weight: c_int) -> c_int;
    pub fn FcWeightToOpenTypeDouble(weight: c_double) -> c_double;
}
