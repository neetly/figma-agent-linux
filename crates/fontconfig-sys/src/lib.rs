#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use freetype_sys::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use _FcResult::{
    FcResultMatch, FcResultNoId, FcResultNoMatch, FcResultOutOfMemory, FcResultTypeMismatch,
};
