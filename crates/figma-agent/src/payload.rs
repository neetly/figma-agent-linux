use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FontFilesPayload {
    pub version: i32,
    pub font_files: HashMap<String, Vec<FontFile>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FontFile {
    #[serde(skip_serializing)]
    pub path: String,
    #[serde(skip_serializing)]
    pub index: i32,

    pub postscript: String,
    pub family: String,
    pub style: String,
    pub weight: i32,
    pub italic: bool,
    pub width: i32,

    #[serde(skip_serializing)]
    pub is_variable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_axes: Option<Vec<VariationAxis>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariationAxis {
    pub name: String,
    pub tag: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub default: f64,
    pub hidden: bool,
}
