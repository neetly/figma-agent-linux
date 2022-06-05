use std::collections::HashMap;

use actix_web::{get, web, Responder};
use figma_agent::{FontFile, FontFilesPayload, PatternHelpers, VariationAxis, FC, FONT_CACHE};
use fontconfig::{Pattern, FC_SLANT_ROMAN};
use itertools::Itertools;

#[get("/font-files")]
pub async fn font_files() -> impl Responder {
    let font_cache = FONT_CACHE.lock();
    font_cache.borrow_mut().read();

    let font_files = FC
        .list_fonts(&Pattern::new(), None)
        .iter()
        .flat_map(get_font_file)
        .into_group_map_by(|item| item.path.to_owned());

    let font_files: HashMap<_, _> = font_files
        .into_iter()
        .map(|(path, items)| {
            if items.iter().any(|item| item.is_variable) {
                (
                    path,
                    items
                        .into_iter()
                        .filter(|item| !item.is_variable)
                        .flat_map(get_variable_font_file)
                        .collect(),
                )
            } else {
                (path, items)
            }
        })
        .collect();

    font_cache.borrow_mut().write();

    web::Json(FontFilesPayload {
        version: 20,
        font_files,
    })
}

fn get_font_file(pattern: Pattern) -> Option<FontFile> {
    let path = pattern.file()?;
    let index = pattern.index()?;

    Some(FontFile {
        path: path.to_owned(),
        index,

        postscript: pattern.postscript_name().unwrap_or("").to_owned(),
        family: pattern.family().unwrap_or("").to_owned(),
        style: pattern.style().unwrap_or("").to_owned(),
        weight: pattern.os_weight_class().unwrap_or(400),
        italic: pattern
            .slant()
            .map(|slant| slant != FC_SLANT_ROMAN)
            .unwrap_or(false),
        width: pattern.os_width_class().unwrap_or(5),

        is_variable: pattern.is_variable().unwrap_or(false),
        variation_axes: None,
    })
}

fn get_variable_font_file(mut font_file: FontFile) -> Option<FontFile> {
    let font_cache = FONT_CACHE.lock();

    if font_file.index == 0 {
        font_file.index = 0x10000;
    }

    let font = font_cache
        .borrow_mut()
        .get(&font_file.path, font_file.index as _)?;
    let instance = font.instances.get((font_file.index as usize >> 16) - 1)?;

    font_file.postscript = instance.postscript_name.to_owned();

    let to_f64 = |fixed| fixed as f64 / 65536.0;

    font_file.variation_axes = Some(
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
    );

    Some(font_file)
}
