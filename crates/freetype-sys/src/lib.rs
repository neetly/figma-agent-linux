#![allow(non_camel_case_types, non_upper_case_globals)]

use ffi_utils::opaque;
use libc::{c_int, c_long, c_void};

pub type FT_Library = *mut FT_LibraryRec;
pub type FT_Face = *mut FT_FaceRec;

opaque! {
    #[repr(C)]
    pub struct FT_LibraryRec;

    #[repr(C)]
    pub struct FT_FaceRec;
}

pub type FT_Memory = *mut FT_MemoryRec;

#[repr(C)]
pub struct FT_MemoryRec {
    pub user: *mut c_void,
    pub alloc: FT_Alloc_Func,
    pub realloc: FT_Realloc_Func,
    pub free: FT_Free_Func,
}

pub type FT_Alloc_Func = extern "C" fn(FT_Memory, c_long) -> *mut c_void;
pub type FT_Realloc_Func = extern "C" fn(FT_Memory, c_long, c_long, *mut c_void) -> *mut c_void;
pub type FT_Free_Func = extern "C" fn(FT_Memory, *mut c_void);

pub type FT_Error = c_int;

pub const FT_Err_Ok: FT_Error = 0x00;

#[link(name = "freetype")]
extern "C" {
    pub fn FT_Init_FreeType(library: *mut FT_Library) -> FT_Error;

    pub fn FT_New_Library(memory: FT_Memory, library: *mut FT_Library) -> FT_Error;
    pub fn FT_Reference_Library(library: FT_Library) -> FT_Error;
    pub fn FT_Done_Library(library: FT_Library) -> FT_Error;

    pub fn FT_Add_Default_Modules(library: FT_Library) -> FT_Error;
}
