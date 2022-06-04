#![allow(clippy::missing_safety_doc)]

use std::ffi::CString;

use fontconfig_sys::{
    FcFalse, FcStrListCreate, FcStrSet, FcStrSetAdd, FcStrSetCreate, FcStrSetDel, FcStrSetDestroy,
    FcStrSetEqual, FcStrSetMember,
};

use crate::StrList;

pub struct StrSet {
    raw: *mut FcStrSet,
}

impl StrSet {
    pub fn raw(&self) -> *mut FcStrSet {
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
        let raw_str_list = unsafe { FcStrListCreate(self.raw) };
        assert!(!raw_str_list.is_null());
        unsafe { StrList::from_raw(raw_str_list) }
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

impl PartialEq for StrSet {
    fn eq(&self, other: &Self) -> bool {
        let result = unsafe { FcStrSetEqual(self.raw, other.raw) };
        result != FcFalse
    }
}

impl Eq for StrSet {}

impl Drop for StrSet {
    fn drop(&mut self) {
        unsafe { FcStrSetDestroy(self.raw) };
    }
}
