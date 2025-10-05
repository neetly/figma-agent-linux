use std::{path::PathBuf, sync::Arc, time::SystemTime};

use axum::{
    Json,
    extract::{Query, Request},
    http::StatusCode,
    response::IntoResponse,
};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{
    FONT_FILES,
    font::{Font, FontFile, to_us_weight_class, to_us_width_class},
    load_font_files,
    payload::{FontFilesEndpointPayload, FontPayload, VariationAxisPayload},
};

pub async fn font_files() -> impl IntoResponse {
    let font_files = Arc::new(load_font_files());
    FONT_FILES.store(font_files.clone());

    fn map_font(font: &Font, font_file: &FontFile) -> Vec<FontPayload> {
        let font_payload = FontPayload {
            family: font.family_name.clone().unwrap_or_default(),
            style: font.subfamily_name.clone().unwrap_or_default(),
            postscript: font.postscript_name.clone().unwrap_or_default(),
            weight: to_us_weight_class(font.weight),
            stretch: to_us_width_class(font.width),
            italic: font.is_italic,
            variation_axes: if font.axes.is_empty() {
                None
            } else {
                Some(
                    font.axes
                        .iter()
                        .map(|axis| VariationAxisPayload {
                            tag: axis.tag.clone(),
                            name: axis.name.clone().unwrap_or_default(),
                            value: axis.default_value,
                            min: axis.min_value,
                            max: axis.max_value,
                            default: axis.default_value,
                            hidden: axis.is_hidden,
                        })
                        .collect(),
                )
            },
            modified_at: font_file
                .modified_at
                .and_then(|modified_at| modified_at.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs())
                .unwrap_or_default(),
            user_installed: false,
        };

        if font.named_instances.is_empty() {
            vec![font_payload]
        } else {
            font.named_instances
                .iter()
                .map(|named_instance| {
                    let mut font_payload = font_payload.clone();
                    font_payload.style = named_instance.subfamily_name.clone().unwrap_or_default();
                    font_payload.postscript =
                        named_instance.postscript_name.clone().unwrap_or_default();
                    if let Some(variation_axes) = &mut font_payload.variation_axes {
                        variation_axes
                            .iter_mut()
                            .zip(&named_instance.coordinates)
                            .for_each(|(variation_axis, coordinate)| {
                                variation_axis.value = *coordinate;
                                if variation_axis.tag == "wght" {
                                    font_payload.weight = to_us_weight_class(*coordinate);
                                }
                                if variation_axis.tag == "wdth" {
                                    font_payload.stretch = to_us_width_class(*coordinate);
                                }
                                if variation_axis.tag == "ital" {
                                    font_payload.italic = *coordinate != 0.0;
                                }
                            });
                    }
                    font_payload
                })
                .collect()
        }
    }

    Json(FontFilesEndpointPayload {
        font_files: font_files
            .iter()
            .map(|(path, font_file)| {
                (
                    path.clone(),
                    font_file
                        .fonts
                        .iter()
                        .flat_map(|font| map_font(font, font_file))
                        .collect(),
                )
            })
            .collect(),
        modified_at: None,
        modified_fonts: None,
        package: "125.8.8".into(),
        version: 23,
    })
}

#[derive(Debug, serde::Deserialize)]
pub struct FontFileQuery {
    pub file: PathBuf,
}

pub async fn font_file(Query(query): Query<FontFileQuery>, request: Request) -> impl IntoResponse {
    let font_files = FONT_FILES.load();
    if font_files.contains_key(&query.file) {
        Ok(ServeFile::new(query.file).oneshot(request).await)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
