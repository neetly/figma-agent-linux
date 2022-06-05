use std::slice;

use freetype_sys::FT_Var_Named_Style;

use crate::MMVar;

pub struct VarNamedStyle<'a> {
    raw: &'a FT_Var_Named_Style,
    mm_var: &'a MMVar<'a>,
}

impl<'a> VarNamedStyle<'a> {
    pub fn new(raw: &'a FT_Var_Named_Style, mm_var: &'a MMVar) -> VarNamedStyle<'a> {
        VarNamedStyle { raw, mm_var }
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
        let slice = unsafe { slice::from_raw_parts(self.raw.coords, self.mm_var.axis_count()) };
        slice.iter().map(|&coordinate| coordinate as _)
    }
}
