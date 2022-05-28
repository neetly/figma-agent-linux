#![allow(non_camel_case_types, non_upper_case_globals)]

use ffi_utils::opaque;
use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type FT_Int = c_int;
pub type FT_UInt = c_uint;
pub type FT_Long = c_long;
pub type FT_ULong = c_ulong;
pub type FT_Fixed = c_long;
pub type FT_String = c_char;

pub type FT_Library = *mut FT_LibraryRec;
pub type FT_Face = *mut FT_FaceRec;

opaque! {
    #[repr(C)]
    pub struct FT_LibraryRec;

    #[repr(C)]
    pub struct FT_FaceRec;
}

#[repr(C)]
pub struct FT_MM_Var {
    pub num_axis: FT_UInt,
    pub num_designs: FT_UInt,
    pub num_namedstyles: FT_UInt,
    pub axis: *mut FT_Var_Axis,
    pub namedstyle: *mut FT_Var_Named_Style,
}

#[repr(C)]
pub struct FT_Var_Axis {
    pub name: *mut FT_String,
    pub minimum: FT_Fixed,
    pub def: FT_Fixed,
    pub maximum: FT_Fixed,
    pub tag: FT_ULong,
    pub strid: FT_UInt,
}

#[repr(C)]
pub struct FT_Var_Named_Style {
    pub coords: *mut FT_Fixed,
    pub strid: FT_UInt,
    pub psid: FT_UInt,
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
    pub fn FT_Add_Default_Modules(library: FT_Library) -> FT_Error;

    pub fn FT_New_Library(memory: FT_Memory, library: *mut FT_Library) -> FT_Error;
    pub fn FT_Reference_Library(library: FT_Library) -> FT_Error;
    pub fn FT_Done_Library(library: FT_Library) -> FT_Error;

    pub fn FT_New_Face(
        library: FT_Library,
        pathname: *const c_char,
        face_index: FT_Long,
        face: *mut FT_Face,
    ) -> FT_Error;
    pub fn FT_Reference_Face(face: FT_Face) -> FT_Error;
    pub fn FT_Done_Face(face: FT_Face) -> FT_Error;

    pub fn FT_Get_MM_Var(face: FT_Face, mm_var: *mut *mut FT_MM_Var) -> FT_Error;
    pub fn FT_Done_MM_Var(library: FT_Library, mm_var: *mut FT_MM_Var) -> FT_Error;
}
