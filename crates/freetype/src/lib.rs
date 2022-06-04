use std::ptr;

use libc::{c_long, c_void, free, malloc, realloc, size_t};

pub use freetype_sys::*;

pub use face::*;
pub use library::*;

pub mod face;
pub mod library;

pub fn init() -> Option<Library> {
    let library = Library::new();
    unsafe { FT_Add_Default_Modules(library.raw) };
    Some(library)
}

pub static mut MEMORY: FT_MemoryRec_ = FT_MemoryRec_ {
    user: ptr::null_mut(),
    alloc: Some(memory_alloc),
    free: Some(memory_free),
    realloc: Some(memory_realloc),
};

unsafe extern "C" fn memory_alloc(_: FT_Memory, size: c_long) -> *mut c_void {
    malloc(size as size_t)
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
    realloc(block, new_size as size_t)
}
