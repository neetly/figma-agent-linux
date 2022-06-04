use freetype_sys::FT_Var_Axis;

pub struct VarAxis<'a> {
    raw: &'a FT_Var_Axis,
}

impl VarAxis<'_> {
    pub fn from_raw(raw: &FT_Var_Axis) -> VarAxis {
        VarAxis { raw }
    }
}
