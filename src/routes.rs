use std::{path::PathBuf, sync::Arc, time::SystemTime};

use axum::{
    Json,
    extract::{Query, Request},
    http::{StatusCode, header},
    response::IntoResponse,
};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{
    CONFIG, FONT_FILES,
    font::{Font, FontFile, FontQuery, FontQueryResult, to_us_weight_class, to_us_width_class},
    load_font_files,
    payload::{FontFilesEndpointPayload, FontPayload, VariationAxisPayload},
    renderer::{RenderOptions, render_text},
};

#[tracing::instrument]
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
            italic: font.is_italic || font.is_oblique,
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
            user_installed: true,
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
                        let mut is_italic = font.is_italic;
                        let mut is_oblique = font.is_oblique;
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
                                    is_italic = *coordinate != 0.0;
                                }
                                if variation_axis.tag == "slnt" {
                                    is_oblique = *coordinate != 0.0;
                                }
                            });
                        font_payload.italic = is_italic || is_oblique;
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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FontFileQuery {
    pub file: PathBuf,
}

#[tracing::instrument]
pub async fn font_file(
    Query(query): Query<FontFileQuery>,
    request: Request,
) -> Result<impl IntoResponse, StatusCode> {
    let font_files = FONT_FILES.load();

    let font_file = font_files.get(&query.file).ok_or_else(|| {
        tracing::error!("Font file not found: {path:?}", path = query.file);
        StatusCode::NOT_FOUND
    })?;

    Ok(ServeFile::new(&font_file.path).oneshot(request).await)
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FontPreviewQuery {
    pub file: PathBuf,
    pub family: String,
    pub style: String,
    pub postscript: String,
    pub font_size: f32,
}

#[tracing::instrument]
pub async fn font_preview(
    Query(query): Query<FontPreviewQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    if !CONFIG.enable_font_preview {
        return Err(StatusCode::NOT_FOUND);
    }

    let font_files = FONT_FILES.load();

    let font_file = font_files.get(&query.file).ok_or_else(|| {
        tracing::error!("Font file not found: {path:?}", path = query.file);
        StatusCode::NOT_FOUND
    })?;

    let FontQueryResult {
        font,
        named_instance,
    } = font_file
        .query(FontQuery {
            family_name: Some(query.family.as_str()).filter(|family| !family.is_empty()),
            subfamily_name: Some(query.style.as_str()).filter(|style| !style.is_empty()),
            postscript_name: Some(query.postscript.as_str()).filter(|postscript| !postscript.is_empty()),
        })
        .ok_or_else(|| {
            tracing::error!(
                "Font not found: {family_name:?}, subfamily: {subfamily_name:?}, postscript: {postscript_name:?}",
                family_name = query.family,
                subfamily_name = query.style,
                postscript_name = query.postscript,
            );
            StatusCode::NOT_FOUND
        })?;

    let content = render_text(
        &query.family,
        RenderOptions {
            font: (&font_file.path, font.index),
            size: query.font_size,
            named_instance_index: named_instance.map(|named_instance| named_instance.index),
        },
    )
    .map_err(|error| {
        tracing::error!("Failed to render font preview, error: {error:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(content) = content {
        Ok(([(header::CONTENT_TYPE, "image/svg+xml")], content))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
