#![allow(clippy::missing_safety_doc)]

use std::{mem, slice};

use freetype_sys::FT_SfntName;

pub struct SfntName {
    raw: FT_SfntName,
}

impl SfntName {
    pub unsafe fn new() -> SfntName {
        SfntName { raw: mem::zeroed() }
    }

    pub fn name(&self) -> Option<String> {
        let slice = unsafe { slice::from_raw_parts(self.raw.string, self.raw.string_len as _) };
        let vec: Vec<_> = slice
            .chunks_exact(2)
            .map(|item| u16::from_be_bytes([item[0], item[1]]))
            .collect();
        String::from_utf16(&vec).ok()
    }

    pub fn platform_id(&self) -> u16 {
        self.raw.platform_id
    }

    pub fn encoding_id(&self) -> u16 {
        self.raw.encoding_id
    }

    pub fn language_id(&self) -> u16 {
        self.raw.language_id
    }

    pub fn name_id(&self) -> u16 {
        self.raw.name_id
    }
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
