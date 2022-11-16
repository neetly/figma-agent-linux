use freetype::{TT_MS_ID_SYMBOL_CS, TT_MS_ID_UNICODE_CS, TT_PLATFORM_MICROSOFT};
use serde::{Deserialize, Serialize};

use crate::FT;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Font {
    pub postscript_name: String,
    pub family_name: String,
    pub style_name: String,
    pub variation_axes: Vec<FontVariationAxis>,
    pub instances: Vec<FontInstance>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationAxis {
    pub name: String,
    pub tag: String,
    pub min: i32,
    pub max: i32,
    pub default: i32,
    pub is_hidden: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontInstance {
    pub name: String,
    pub postscript_name: String,
    pub coordinates: Vec<i32>,
}

impl Font {
    pub fn new<P>(path: P, index: isize) -> Option<Font>
    where
        P: AsRef<str>,
    {
        let face = FT.face_from_file(path, index)?;
        let mm_var = face.mm_var()?;

        let get_name = |name_id| {
            let sfnt_name = face.find_sfnt_name(|item| {
                item.name_id() == name_id
                    && item.platform_id() == TT_PLATFORM_MICROSOFT as u16
                    && (item.encoding_id() == TT_MS_ID_SYMBOL_CS as u16
                        || item.encoding_id() == TT_MS_ID_UNICODE_CS as u16)
            })?;
            sfnt_name.name()
        };

        Some(Font {
            postscript_name: face.postscript_name().unwrap_or("").to_owned(),
            family_name: face.family_name().unwrap_or("").to_owned(),
            style_name: face.style_name().unwrap_or("").to_owned(),

            variation_axes: mm_var
                .axes()
                .map(|axis| FontVariationAxis {
                    name: get_name(axis.name_id()).unwrap_or_else(|| "".to_owned()),
                    tag: axis.tag_string().unwrap_or_else(|| "".to_owned()),
                    min: axis.min(),
                    max: axis.max(),
                    default: axis.default(),
                    is_hidden: axis.is_hidden().unwrap_or(false),
                })
                .collect(),

            instances: mm_var
                .named_styles()
                .map(|named_style| FontInstance {
                    name: get_name(named_style.name_id()).unwrap_or_else(|| "".to_owned()),
                    postscript_name: named_style
                        .postscript_name_id()
                        .and_then(get_name)
                        .unwrap_or_else(|| "".to_owned()),
                    coordinates: named_style.coordinates().collect(),
                })
                .collect(),
        })
    }
}
