use serde::{Deserialize, Serialize};

use crate::FT;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Font {
    variation_axes: Vec<FontVariationAxis>,
    instances: Vec<FontInstance>,
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
            let sfnt_name = face.find_sfnt_name(|item| item.name_id() == name_id)?;
            sfnt_name.name()
        };

        Some(Font {
            variation_axes: mm_var
                .axes()
                .map(|item| FontVariationAxis {
                    name: get_name(item.name_id()).unwrap_or("".into()),
                    tag: item.tag_string().unwrap_or("".into()),
                    min: item.min(),
                    max: item.max(),
                    default: item.default(),
                    is_hidden: item.is_hidden().unwrap_or(false),
                })
                .collect(),

            instances: mm_var
                .named_styles()
                .map(|item| FontInstance {
                    name: get_name(item.name_id()).unwrap_or("".into()),
                    postscript_name: item
                        .postscript_name_id()
                        .and_then(get_name)
                        .unwrap_or("".into()),
                    coordinates: item.coordinates().collect(),
                })
                .collect(),
        })
    }
}
