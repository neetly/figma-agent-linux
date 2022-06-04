use freetype_sys::FT_Var_Named_Style;

pub struct VarNamedStyle<'a> {
    raw: &'a FT_Var_Named_Style,
}

impl VarNamedStyle<'_> {
    pub fn from_raw(raw: &FT_Var_Named_Style) -> VarNamedStyle {
        VarNamedStyle { raw }
    }
}
