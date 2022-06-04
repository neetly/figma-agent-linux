#![allow(clippy::missing_safety_doc)]

use std::mem;

use freetype_sys::FT_SfntName;

pub struct SfntName {
    raw: FT_SfntName,
}

impl AsRef<FT_SfntName> for SfntName {
    fn as_ref(&self) -> &FT_SfntName {
        &self.raw
    }
}

impl AsMut<FT_SfntName> for SfntName {
    fn as_mut(&mut self) -> &mut FT_SfntName {
        &mut self.raw
    }
}

impl SfntName {
    pub unsafe fn new() -> SfntName {
        let raw = mem::zeroed();
        SfntName { raw }
    }
}
