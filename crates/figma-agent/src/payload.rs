use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct FontFilesResult {
    pub version: i32,
    pub package: String,
    #[serde(rename = "fontFiles")]
    pub font_files: HashMap<String, Vec<FontFile>>,
}

#[derive(Serialize, Clone)]
pub struct FontFile {
    #[serde(skip_serializing)]
    pub path: String,
    #[serde(skip_serializing)]
    pub index: i32,

    pub user_installed: bool,
    pub modified_at: u64,

    pub postscript: String,
    pub family: String,
    pub style: String,
    pub weight: i32,
    pub italic: bool,
    pub stretch: i32,

    #[serde(skip_serializing)]
    pub is_variable: bool,
    #[serde(rename = "variationAxes", skip_serializing_if = "Option::is_none")]
    pub variation_axes: Option<Vec<VariationAxis>>,
}

#[derive(Serialize, Clone)]
pub struct VariationAxis {
    pub name: String,
    pub tag: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub default: f64,
    pub hidden: bool,
}
