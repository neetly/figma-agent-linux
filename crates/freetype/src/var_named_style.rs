use std::slice;

use freetype_sys::FT_Var_Named_Style;

pub struct VarNamedStyle<'a> {
    raw: &'a FT_Var_Named_Style,
    axis_count: usize,
}

impl VarNamedStyle<'_> {
    pub fn new(raw: &FT_Var_Named_Style, axis_count: usize) -> VarNamedStyle {
        VarNamedStyle { raw, axis_count }
    }

    pub fn name_id(&self) -> u16 {
        self.raw.strid as _
    }

    pub fn postscript_name_id(&self) -> Option<u16> {
        if self.raw.psid != 0xFFFF {
            Some(self.raw.psid as _)
        } else {
            None
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = i32> {
        let slice = unsafe { slice::from_raw_parts(self.raw.coords, self.axis_count) };
        slice.iter().map(|&item| item as _)
    }
}
