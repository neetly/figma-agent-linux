use actix_web::{get, web, Responder};
use figma_agent::{FontCache, FontFile, FontFilesPayload, PatternHelpers, FC, FONT_CACHE};
use fontconfig::{Pattern, FC_SLANT_ROMAN};
use itertools::Itertools;

#[get("/font-files")]
pub async fn font_files() -> impl Responder {
    let mut font_cache = FONT_CACHE.lock().unwrap();
    font_cache.read();

    let font_files = FC
        .list_fonts(&Pattern::new(), None)
        .iter()
        .flat_map(|pattern| get_font_file(&pattern, &mut font_cache))
        .into_group_map_by(|item| item.file.clone());

    font_cache.write();

    web::Json(FontFilesPayload {
        version: 20,
        font_files,
    })
}

fn get_font_file(pattern: &Pattern, _: &mut FontCache) -> Option<FontFile> {
    let path = pattern.file()?;
    let index = pattern.index()?;

    Some(FontFile {
        file: path.into(),
        index,

        postscript: pattern.postscript_name().unwrap_or("").into(),
        family: pattern.family().unwrap_or("").into(),
        style: pattern.style().unwrap_or("").into(),
        weight: pattern.os_weight_class().unwrap_or(400),
        italic: pattern
            .slant()
            .map(|slant| slant != FC_SLANT_ROMAN)
            .unwrap_or(false),
        width: pattern.os_width_class().unwrap_or(5),

        variation_axes: None,
    })
}
