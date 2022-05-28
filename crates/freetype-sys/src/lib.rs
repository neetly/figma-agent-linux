#![allow(non_camel_case_types, non_upper_case_globals)]

use std::os::raw::c_int;

use ffi_utils::opaque;

opaque! {
    #[repr(C)]
    pub struct FT_Library;

    #[repr(C)]
    pub struct FT_Face;
}

pub type FT_Error = c_int;

pub const FT_Err_Ok: FT_Error = 0x00;

#[link(name = "freetype")]
extern "C" {
    pub fn FT_Init_FreeType(library: *mut FT_Library) -> FT_Error;
}
