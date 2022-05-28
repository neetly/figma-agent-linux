#![allow(
    non_upper_case_globals,
    clippy::missing_safety_doc,
    clippy::new_without_default
)]

use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::size_of,
    ptr, slice, str,
};

use libc::{c_long, c_void, free, malloc, realloc};

pub use freetype_sys::*;

#[macro_export]
macro_rules! dispatch {
    ($expr:expr) => {{
        let result = $expr;
        assert!(result == FT_Err_Ok);
    }};
}

#[macro_export]
macro_rules! try_dispatch {
    ($expr:expr) => {{
        let result = $expr;
        if result == FT_Err_Ok {
            Some(())
        } else {
            None
        }
    }};
}

pub struct Library {
    raw: FT_Library,
}

impl Library {
    pub fn new() -> Option<Library> {
        let mut raw: FT_Library = ptr::null_mut();
        try_dispatch!(unsafe { FT_New_Library(&mut MEMORY, &mut raw) })?;
        Some(Library { raw })
    }

    pub unsafe fn from_raw(raw: FT_Library) -> Library {
        Library { raw }
    }

    pub unsafe fn from_raw_with_ref(raw: FT_Library) -> Library {
        dispatch!(FT_Reference_Library(raw));
        Library { raw }
    }

    pub fn init() -> Option<Library> {
        let library = Library::new()?;
        try_dispatch!(unsafe { FT_Add_Default_Modules(library.raw) })?;
        Some(library)
    }

    pub fn open_face(&self, path: &str, face_index: i64) -> Option<Face> {
        Face::new(self, path, face_index)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        dispatch!(unsafe { FT_Done_Library(self.raw) });
    }
}

pub struct Face<'a> {
    library: &'a Library,
    raw: FT_Face,
}

impl<'a> Face<'a> {
    pub fn new(library: &'a Library, path: &str, face_index: i64) -> Option<Face<'a>> {
        let mut raw: FT_Face = ptr::null_mut();
        let path = CString::new(path).ok()?;
        try_dispatch!(unsafe { FT_New_Face(library.raw, path.as_ptr(), face_index, &mut raw) })?;
        Some(Face { library, raw })
    }

    pub unsafe fn from_raw(library: &Library, raw: FT_Face) -> Face {
        Face { library, raw }
    }

    pub unsafe fn from_raw_with_ref(library: &Library, raw: FT_Face) -> Face {
        dispatch!(FT_Reference_Face(raw));
        Face { library, raw }
    }

    pub fn sfnt_name_count(&self) -> usize {
        unsafe { FT_Get_Sfnt_Name_Count(self.raw) as usize }
    }

    pub fn sfnt_name_at(&self, index: usize) -> Option<SfntName> {
        let sfnt_name = SfntName::new();
        try_dispatch!(unsafe { FT_Get_Sfnt_Name(self.raw, index as FT_UInt, sfnt_name.raw) })?;
        Some(sfnt_name)
    }

    pub fn mm_var(&self) -> Option<MMVar> {
        let mut raw_mm_var: *mut FT_MM_Var = ptr::null_mut();
        try_dispatch!(unsafe { FT_Get_MM_Var(self.raw, &mut raw_mm_var) })?;
        Some(MMVar {
            raw: raw_mm_var,
            library: self.library,
            face: self,
        })
    }
}

impl Drop for Face<'_> {
    fn drop(&mut self) {
        dispatch!(unsafe { FT_Done_Face(self.raw) });
    }
}

pub struct SfntName<'a> {
    raw: *mut FT_SfntName,
    _marker: PhantomData<&'a Face<'a>>,
}

impl<'a> SfntName<'a> {
    pub fn new() -> SfntName<'a> {
        let raw = unsafe { malloc(size_of::<FT_SfntName>()) as *mut FT_SfntName };
        SfntName {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn name(&self) -> Option<&'a str> {
        let slice =
            unsafe { slice::from_raw_parts((*self.raw).string, (*self.raw).string_len as usize) };
        str::from_utf8(slice).ok()
    }
}

impl Drop for SfntName<'_> {
    fn drop(&mut self) {
        unsafe { free(self.raw as *mut c_void) };
    }
}

pub struct MMVar<'a> {
    raw: *mut FT_MM_Var,
    library: &'a Library,
    face: &'a Face<'a>,
}

impl MMVar<'_> {
    pub fn num_axes(&self) -> usize {
        unsafe { (*self.raw).num_axis as usize }
    }

    pub fn num_designs(&self) -> usize {
        unsafe { (*self.raw).num_designs as usize }
    }

    pub fn num_named_styles(&self) -> usize {
        unsafe { (*self.raw).num_namedstyles as usize }
    }

    pub fn axes(&self) -> impl Iterator<Item = VarAxis> {
        let raw_axes =
            unsafe { slice::from_raw_parts((*self.raw).axis, (*self.raw).num_axis as usize) };
        raw_axes.iter().map(|axis| VarAxis {
            raw: axis,
            face: self.face,
        })
    }

    pub fn named_styles(&self) -> impl Iterator<Item = VarNamedStyle> {
        let raw_named_styles = unsafe {
            slice::from_raw_parts((*self.raw).namedstyle, (*self.raw).num_namedstyles as usize)
        };
        raw_named_styles.iter().map(|named_style| VarNamedStyle {
            raw: named_style,
            mm_var: self,
        })
    }
}

impl Drop for MMVar<'_> {
    fn drop(&mut self) {
        dispatch!(unsafe { FT_Done_MM_Var(self.library.raw, self.raw) });
    }
}

pub struct VarAxis<'a> {
    raw: &'a FT_Var_Axis,
    face: &'a Face<'a>,
}

impl VarAxis<'_> {
    pub fn name(&self) -> Option<&str> {
        unsafe { CStr::from_ptr(self.raw.name).to_str().ok() }
    }

    pub fn sfnt_name(&self) -> Option<&str> {
        self.face.sfnt_name_at(self.raw.strid as usize)?.name()
    }

    pub fn default(&self) -> i64 {
        self.raw.def
    }

    pub fn min(&self) -> i64 {
        self.raw.minimum
    }

    pub fn max(&self) -> i64 {
        self.raw.maximum
    }
}

pub struct VarNamedStyle<'a> {
    raw: &'a FT_Var_Named_Style,
    mm_var: &'a MMVar<'a>,
}

impl VarNamedStyle<'_> {
    pub fn coords(&self) -> impl Iterator<Item = &i64> {
        let raw_coords = unsafe { slice::from_raw_parts(self.raw.coords, self.mm_var.num_axes()) };
        raw_coords.iter()
    }
}

static mut MEMORY: FT_MemoryRec = FT_MemoryRec {
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
