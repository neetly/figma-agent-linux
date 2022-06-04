use freetype_sys::FT_Var_Axis;

pub struct VarAxis<'a> {
    raw: &'a FT_Var_Axis,
}

impl VarAxis<'_> {
    pub fn new(raw: &FT_Var_Axis) -> VarAxis {
        VarAxis { raw }
    }

    pub fn tag(&self) -> u32 {
        self.raw.tag as _
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
}
