#![allow(clippy::missing_safety_doc)]

use std::slice;

use freetype_sys::FT_SfntName;

pub struct SfntName {
    value: FT_SfntName,
}

impl Default for SfntName {
    fn default() -> Self {
        Self::new()
    }
}

impl SfntName {
    pub fn new() -> SfntName {
        SfntName {
            value: Default::default(),
        }
    }

    pub fn name(&self) -> Option<String> {
        let slice = unsafe { slice::from_raw_parts(self.value.string, self.value.string_len as _) };
        let vec: Vec<_> = slice
            .chunks_exact(2)
            .map(|item| u16::from_be_bytes([item[0], item[1]]))
            .collect();
        String::from_utf16(&vec).ok()
    }

    pub fn platform_id(&self) -> u16 {
        self.value.platform_id
    }

    pub fn encoding_id(&self) -> u16 {
        self.value.encoding_id
    }

    pub fn language_id(&self) -> u16 {
        self.value.language_id
    }
}

impl AsRef<FT_SfntName> for SfntName {
    fn as_ref(&self) -> &FT_SfntName {
        &self.value
    }
}

impl AsMut<FT_SfntName> for SfntName {
    fn as_mut(&mut self) -> &mut FT_SfntName {
        &mut self.value
    }
}
