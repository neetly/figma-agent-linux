use fontconfig::{
    FcWeightToOpenType, Pattern, FC_WIDTH_CONDENSED, FC_WIDTH_EXPANDED, FC_WIDTH_EXTRACONDENSED,
    FC_WIDTH_EXTRAEXPANDED, FC_WIDTH_NORMAL, FC_WIDTH_SEMICONDENSED, FC_WIDTH_SEMIEXPANDED,
    FC_WIDTH_ULTRACONDENSED, FC_WIDTH_ULTRAEXPANDED,
};

pub trait OpenTypeHelpers {
    fn os_weight_class(&self) -> Option<i32>;

    fn os_width_class(&self) -> Option<i32>;
}

impl OpenTypeHelpers for Pattern {
    fn os_weight_class(&self) -> Option<i32> {
        let weight = self.weight()?;
        Some(unsafe { FcWeightToOpenType(weight) })
    }

    fn os_width_class(&self) -> Option<i32> {
        let width = self.width()?;
        WIDTHS
            .iter()
            .min_by_key(|item| width.abs_diff(item.1))
            .map(|item| item.0)
    }
}

const WIDTHS: Vec<(i32, i32)> = vec![
    (1, FC_WIDTH_ULTRACONDENSED),
    (2, FC_WIDTH_EXTRACONDENSED),
    (3, FC_WIDTH_CONDENSED),
    (4, FC_WIDTH_SEMICONDENSED),
    (5, FC_WIDTH_NORMAL),
    (6, FC_WIDTH_SEMIEXPANDED),
    (7, FC_WIDTH_EXPANDED),
    (8, FC_WIDTH_EXTRAEXPANDED),
    (9, FC_WIDTH_ULTRAEXPANDED),
];
