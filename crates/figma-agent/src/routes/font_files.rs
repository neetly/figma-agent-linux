use std::{collections::HashMap, fs, time::UNIX_EPOCH};

use actix_web::{get, web, Responder};
use figma_agent::{PatternHelpers, CONFIG, FC, FONT_CACHE};
use fontconfig::{Pattern, FC_SLANT_ROMAN};
use itertools::Itertools;

use crate::payload;

#[get("/font-files")]
pub async fn font_files() -> impl Responder {
    let font_cache = FONT_CACHE.lock();
    font_cache.borrow_mut().read();

    let patterns: Vec<_> = FC.list_fonts(&Pattern::new(), None).iter().collect();

    let font_files = patterns
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
                        .map(|item| get_variable_font_file(&item).unwrap_or(item))
                        .collect(),
                )
            } else {
                (path, items)
            }
        })
        .collect();

    font_cache.borrow_mut().write();

    web::Json(payload::FontFilesResult {
        version: 22,
        package: "116.10.8".to_owned(), // latest version as of 2023-06-22
        font_files,
    })
}

fn get_font_file(pattern: &Pattern) -> Option<payload::FontFile> {
    let path = pattern.file()?;
    let index = pattern.index()?;

    Some(payload::FontFile {
        path: path.to_owned(),
        index,

        user_installed: true,
        modified_at: fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .ok()
            .and_then(|modified_time| modified_time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0),

        postscript: pattern.postscript_name().unwrap_or("").to_owned(),
        family: pattern.family().unwrap_or("").to_owned(),
        style: pattern.style().unwrap_or("").to_owned(),
        weight: pattern.os_weight_class().unwrap_or(400),
        italic: pattern
            .slant()
            .map(|slant| slant != FC_SLANT_ROMAN)
            .unwrap_or(false),
        stretch: pattern.os_width_class().unwrap_or(5),

        is_variable: pattern.is_variable().unwrap_or(false),
        variation_axes: None,
    })
}

fn get_variable_font_file(font_file: &payload::FontFile) -> Option<payload::FontFile> {
    if !CONFIG.enable_variable_font {
        return None;
    }

    let font_cache = FONT_CACHE.lock();

    let font_index = font_file.index as isize & 0xFFFF;
    let font = font_cache.borrow_mut().get(&font_file.path, font_index)?;

    let instance_index = (font_file.index as isize >> 16) - 1;
    let instance = if instance_index != -1 {
        font.instances.get(instance_index as usize)
    } else {
        None
    };

    let mut font_file = font_file.to_owned();

    if let Some(instance) = instance {
        font_file.postscript = instance.postscript_name.to_owned();
        font_file.family = font.family_name.to_owned();
        font_file.style = instance.name.to_owned();
    } else {
        font_file.postscript = font.postscript_name.to_owned();
        font_file.family = font.family_name.to_owned();
        font_file.style = font.style_name.to_owned();
    }

    let from_fixed = |fixed| fixed as f64 / 65536.0;

    font_file.variation_axes = Some(
        font.variation_axes
            .iter()
            .enumerate()
            .map(|(index, variation_axis)| payload::VariationAxis {
                name: variation_axis.name.to_owned(),
                tag: variation_axis.tag.to_owned(),
                value: from_fixed(
                    instance
                        .map(|instance| instance.coordinates[index])
                        .unwrap_or(variation_axis.default),
                ),
                min: from_fixed(variation_axis.min),
                max: from_fixed(variation_axis.max),
                default: from_fixed(variation_axis.default),
                hidden: variation_axis.is_hidden,
            })
            .collect(),
    );

    Some(font_file)
}
