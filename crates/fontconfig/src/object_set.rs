#![allow(clippy::missing_safety_doc)]

use fontconfig_sys::{FcFalse, FcObjectSet, FcObjectSetAdd, FcObjectSetCreate, FcObjectSetDestroy};

pub struct ObjectSet {
    raw: *mut FcObjectSet,
}

impl ObjectSet {
    pub fn raw(&self) -> *mut FcObjectSet {
        self.raw
    }
}

impl Default for ObjectSet {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn insert(&mut self, object: &[u8]) -> bool {
        let result = unsafe { FcObjectSetAdd(self.raw, object.as_ptr() as _) };
        result != FcFalse
    }
}

impl Drop for ObjectSet {
    fn drop(&mut self) {
        unsafe { FcObjectSetDestroy(self.raw) };
    }
}
