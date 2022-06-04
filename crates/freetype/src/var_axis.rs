use freetype_sys::{FT_Err_Ok, FT_Get_Var_Axis_Flags, FT_Var_Axis, FT_VAR_AXIS_FLAG_HIDDEN};

use crate::MMVar;

pub struct VarAxis<'a> {
    raw: &'a FT_Var_Axis,
    mm_var: &'a MMVar<'a>,
    index: usize,
}

impl<'a> VarAxis<'a> {
    pub fn new(raw: &'a FT_Var_Axis, mm_var: &'a MMVar, index: usize) -> VarAxis<'a> {
        VarAxis { raw, mm_var, index }
    }

    pub fn tag(&self) -> u32 {
        self.raw.tag as _
    }

    pub fn tag_string(&self) -> Option<String> {
        String::from_utf8(self.tag().to_be_bytes().into()).ok()
    }

    pub fn name_id(&self) -> u16 {
        self.raw.strid as _
    }

    pub fn min(&self) -> i32 {
        self.raw.minimum as _
    }

    pub fn max(&self) -> i32 {
        self.raw.maximum as _
    }

    pub fn default(&self) -> i32 {
        self.raw.def as _
    }

    pub fn flags(&self) -> Option<u16> {
        let mut flags = Default::default();
        let result =
            unsafe { FT_Get_Var_Axis_Flags(self.mm_var.raw(), self.index as _, &mut flags) };
        if result == FT_Err_Ok {
            Some(flags as _)
        } else {
            None
        }
    }

    pub fn is_hidden(&self) -> Option<bool> {
        let flags = self.flags()?;
        Some(flags & (FT_VAR_AXIS_FLAG_HIDDEN as u16) != 0)
    }
}
