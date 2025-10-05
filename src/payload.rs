use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize)]
pub struct FontFilesEndpointPayload {
    #[serde(rename = "fontFiles")]
    pub font_files: HashMap<PathBuf, Vec<FontPayload>>,
    pub modified_at: Option<u64>,
    pub modified_fonts: Option<HashMap<PathBuf, Vec<FontPayload>>>,
    pub package: String,
    pub version: u32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FontPayload {
    pub family: String,     // Family name
    pub style: String,      // Subfamily name
    pub postscript: String, // PostScript name
    pub weight: u16,        // Weight (OS/2 usWeightClass)
    pub stretch: u16,       // Width (OS/2 usWidthClass)
    pub italic: bool,
    #[serde(rename = "variationAxes", skip_serializing_if = "Option::is_none")]
    pub variation_axes: Option<Vec<VariationAxisPayload>>,
    pub modified_at: u64,
    pub user_installed: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct VariationAxisPayload {
    pub tag: String,
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    pub hidden: bool,
}
