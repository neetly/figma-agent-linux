use actix_web::{get, web, Responder};
use figma_agent::{
    FontCache, FontFile, FontFilesPayload, PatternHelpers, VariationAxis, FC, FONT_CACHE,
};
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
        .into_group_map_by(|item| item.file.to_owned());

    font_cache.write();

    web::Json(FontFilesPayload {
        version: 20,
        font_files,
    })
}

fn get_font_file(pattern: &Pattern, font_cache: &mut FontCache) -> Option<FontFile> {
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

        is_variable: pattern.is_variable().unwrap_or(false),
        variation_axes: get_variation_axes(path, index as _, font_cache),
    })
}

fn get_variation_axes(
    path: &str,
    index: isize,
    font_cache: &mut FontCache,
) -> Option<Vec<VariationAxis>> {
    let instance_index = index >> 16;
    if instance_index <= 0 {
        return None;
    }

    let font = font_cache.get(path, index)?;
    let instance = font.instances.get(instance_index as usize - 1)?;

    let to_f64 = |fixed| fixed as f64 / 65536.0;

    Some(
        font.variation_axes
            .iter()
            .enumerate()
            .map(|(index, axis)| VariationAxis {
                name: axis.name.to_owned(),
                tag: axis.tag.to_owned(),
                value: to_f64(instance.coordinates[index]),
                min: to_f64(axis.min),
                max: to_f64(axis.max),
                default: to_f64(axis.default),
                hidden: axis.is_hidden,
            })
            .collect(),
    )
}
