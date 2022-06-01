#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use freetype_sys::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use FcEndian::{FcEndianBig, FcEndianLittle};
pub use _FcLangResult::{
    FcLangDifferentCountry, FcLangDifferentLang, FcLangDifferentTerritory, FcLangEqual,
};
pub use _FcMatchKind::{
    FcMatchFont, FcMatchKindBegin, FcMatchKindEnd, FcMatchPattern, FcMatchScan,
};
pub use _FcResult::{
    FcResultMatch, FcResultNoId, FcResultNoMatch, FcResultOutOfMemory, FcResultTypeMismatch,
};
pub use _FcSetName::{FcSetApplication, FcSetSystem};
pub use _FcType::{
    FcTypeBool, FcTypeCharSet, FcTypeDouble, FcTypeFTFace, FcTypeInteger, FcTypeLangSet,
    FcTypeMatrix, FcTypeRange, FcTypeString, FcTypeUnknown, FcTypeVoid,
};
pub use _FcValueBinding::{
    FcValueBindingEnd, FcValueBindingSame, FcValueBindingStrong, FcValueBindingWeak,
};
