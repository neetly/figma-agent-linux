#![allow(clippy::missing_safety_doc)]

use std::ffi::CString;

use fontconfig_sys::{
    FcFalse, FcStrSet, FcStrSetAdd, FcStrSetCreate, FcStrSetDel, FcStrSetDestroy, FcStrSetMember,
};

use crate::StrList;

pub struct StrSet {
    raw: *mut FcStrSet,
}

impl StrSet {
    pub unsafe fn raw(&self) -> *mut FcStrSet {
        self.raw
    }
}

impl Default for StrSet {
    fn default() -> Self {
        Self::new()
    }
}

impl StrSet {
    pub fn new() -> StrSet {
        let raw = unsafe { FcStrSetCreate() };
        assert!(!raw.is_null());
        StrSet { raw }
    }

    pub unsafe fn from_raw(raw: *mut FcStrSet) -> StrSet {
        StrSet { raw }
    }

    pub fn iter(&self) -> StrList {
        StrList::new(self)
    }

    pub fn contains<V>(&self, value: V) -> bool
    where
        V: AsRef<str>,
    {
        if let Ok(value) = CString::new(value.as_ref()) {
            let result = unsafe { FcStrSetMember(self.raw, value.as_ptr() as _) };
            result != FcFalse
        } else {
            false
        }
    }

    pub fn insert<V>(&mut self, value: V) -> bool
    where
        V: AsRef<str>,
    {
        if let Ok(value) = CString::new(value.as_ref()) {
            let result = unsafe { FcStrSetAdd(self.raw, value.as_ptr() as _) };
            result != FcFalse
        } else {
            false
        }
    }

    pub fn remove<V>(&mut self, value: V) -> bool
    where
        V: AsRef<str>,
    {
        if let Ok(value) = CString::new(value.as_ref()) {
            let result = unsafe { FcStrSetDel(self.raw, value.as_ptr() as _) };
            result != FcFalse
        } else {
            false
        }
    }
}

impl Drop for StrSet {
    fn drop(&mut self) {
        unsafe { FcStrSetDestroy(self.raw) };
    }
}
