#![allow(non_upper_case_globals, clippy::missing_safety_doc)]

use std::ptr;

use libc::{c_long, c_void, free, malloc, realloc};

pub use freetype_sys::*;

#[macro_export]
macro_rules! dispatch {
    ($expr:expr) => {
        dispatch!($expr, ())
    };

    ($expr:expr, $result:expr) => {{
        let result = $expr;
        match result {
            FT_Err_Ok => Ok($result),
            _ => Err(result),
        }
    }};
}

pub struct Library {
    raw: FT_Library,
}

impl Library {
    pub fn new() -> Result<Library, FT_Error> {
        let mut raw: FT_Library = ptr::null_mut();
        unsafe { dispatch!(FT_New_Library(&mut MEMORY, &mut raw), Library { raw }) }
    }

    pub unsafe fn from_raw(raw: FT_Library) -> Result<Library, FT_Error> {
        Ok(Library { raw })
    }

    pub unsafe fn from_raw_with_ref(raw: FT_Library) -> Result<Library, FT_Error> {
        dispatch!(FT_Reference_Library(raw), Library { raw })
    }

    pub fn init() -> Result<Library, FT_Error> {
        let library = Library::new()?;
        unsafe { dispatch!(FT_Add_Default_Modules(library.raw), library) }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { FT_Done_Library(self.raw) };
    }
}

pub static mut MEMORY: FT_MemoryRec = FT_MemoryRec {
    user: ptr::null_mut(),
    alloc: memory_alloc,
    realloc: memory_realloc,
    free: memory_free,
};

extern "C" fn memory_alloc(_: FT_Memory, size: c_long) -> *mut c_void {
    unsafe { malloc(size as usize) }
}

extern "C" fn memory_realloc(
    _: FT_Memory,
    _: c_long,
    size: c_long,
    pointer: *mut c_void,
) -> *mut c_void {
    unsafe { realloc(pointer, size as usize) }
}

extern "C" fn memory_free(_: FT_Memory, pointer: *mut c_void) {
    unsafe { free(pointer) }
}
