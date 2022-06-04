#![allow(clippy::missing_safety_doc)]

use std::{ffi::CStr, marker::PhantomData};

use fontconfig_sys::{FcStrList, FcStrListCreate, FcStrListDone, FcStrListNext};

use crate::StrSet;

pub struct StrList<'a> {
    raw: *mut FcStrList,
    _marker: PhantomData<&'a StrSet>,
}

impl StrList<'_> {
    pub fn raw(&self) -> *mut FcStrList {
        self.raw
    }
}

impl<'a> StrList<'a> {
    pub fn new(str_set: &StrSet) -> StrList {
        let raw = unsafe { FcStrListCreate(str_set.raw()) };
        assert!(!raw.is_null());
        StrList {
            raw,
            _marker: PhantomData,
        }
    }

    pub unsafe fn from_raw(raw: *mut FcStrList) -> StrList<'a> {
        StrList {
            raw,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for StrList<'a> {
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

impl Drop for StrList<'_> {
    fn drop(&mut self) {
        unsafe { FcStrListDone(self.raw) };
    }
}
