#![allow(clippy::missing_safety_doc)]

use std::{ffi::CStr, marker::PhantomData};

use fontconfig_sys::{FcStrList, FcStrListDone, FcStrListNext};

use crate::StrSet;

pub struct StrList<'a> {
    pub(crate) raw: *mut FcStrList,
    _marker: PhantomData<&'a StrSet>,
}

impl<'a> StrList<'a> {
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
