use freetype_sys::FT_Var_Named_Style;

pub struct VarNamedStyle<'a> {
    raw: &'a FT_Var_Named_Style,
}

impl VarNamedStyle<'_> {
    pub fn from_raw(raw: &FT_Var_Named_Style) -> VarNamedStyle {
        VarNamedStyle { raw }
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
}
