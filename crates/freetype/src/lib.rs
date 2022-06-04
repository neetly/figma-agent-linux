use std::ptr;

use libc::{c_long, c_void, free, malloc, realloc};

pub use freetype_sys::*;

mod face;
mod library;
mod memory;
mod mm_var;
mod sfnt_name;
mod var_axis;
mod var_named_style;

pub use crate::face::*;
pub use crate::library::*;
pub use crate::memory::*;
pub use crate::mm_var::*;
pub use crate::sfnt_name::*;
pub use crate::var_axis::*;
pub use crate::var_named_style::*;

pub fn init() -> Option<Library> {
    let library = Library::new();
    unsafe { FT_Add_Default_Modules(library.raw()) };
    Some(library)
}

pub static mut MEMORY: Memory = Memory {
    user: ptr::null_mut(),
    alloc: Some(memory_alloc),
    free: Some(memory_free),
    realloc: Some(memory_realloc),
};

unsafe extern "C" fn memory_alloc(_: FT_Memory, size: c_long) -> *mut c_void {
    malloc(size as _)
}

unsafe extern "C" fn memory_free(_: FT_Memory, block: *mut c_void) {
    free(block)
}

unsafe extern "C" fn memory_realloc(
    _: FT_Memory,
    _: c_long,
    new_size: c_long,
    block: *mut c_void,
) -> *mut c_void {
    realloc(block, new_size as _)
}
