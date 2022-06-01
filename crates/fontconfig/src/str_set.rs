#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
};

use fontconfig_sys::{
    FcFalse, FcStrList, FcStrListCreate, FcStrListDone, FcStrListNext, FcStrSet, FcStrSetAdd,
    FcStrSetCreate, FcStrSetDel, FcStrSetDestroy, FcStrSetEqual, FcStrSetMember,
};

pub struct StrSet {
    pub(crate) raw: *mut FcStrSet,
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

    pub fn iter(&self) -> StrSetIterator {
        let raw_str_list = unsafe { FcStrListCreate(self.raw) };
        assert!(!raw_str_list.is_null());
        unsafe { StrSetIterator::from_raw(raw_str_list) }
    }

    pub fn contains<V>(&self, value: V) -> bool
    where
        V: AsRef<str>,
    {
        let value = CString::new(value.as_ref()).unwrap();
        let result = unsafe { FcStrSetMember(self.raw, value.as_ptr() as _) };
        result != FcFalse
    }

    pub fn insert<V>(&mut self, value: V) -> bool
    where
        V: AsRef<str>,
    {
        let value = CString::new(value.as_ref()).unwrap();
        let result = unsafe { FcStrSetAdd(self.raw, value.as_ptr() as _) };
        result != FcFalse
    }

    pub fn remove<V>(&mut self, value: V) -> bool
    where
        V: AsRef<str>,
    {
        let value = CString::new(value.as_ref()).unwrap();
        let result = unsafe { FcStrSetDel(self.raw, value.as_ptr() as _) };
        result != FcFalse
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

pub struct StrSetIterator<'a> {
    pub(crate) raw: *mut FcStrList,
    _marker: PhantomData<&'a StrSet>,
}

impl<'a> StrSetIterator<'a> {
    pub unsafe fn from_raw(raw: *mut FcStrList) -> StrSetIterator<'a> {
        StrSetIterator {
            raw,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for StrSetIterator<'a> {
    type Item = Option<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_str = unsafe { FcStrListNext(self.raw) };
        if !raw_str.is_null() {
            Some(unsafe { CStr::from_ptr(raw_str as _) }.to_str().ok())
        } else {
            None
        }
    }
}

impl Drop for StrSetIterator<'_> {
    fn drop(&mut self) {
        unsafe { FcStrListDone(self.raw) };
    }
}
