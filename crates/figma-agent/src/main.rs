use std::{collections::HashMap, fs::File, path::Path};

use fontconfig::{Pattern, FC_SLANT_ROMAN};
use itertools::Itertools;
use serde::Serialize;
use tiny_http::{Header, Response, Server};
use uriparse::URIReference;

mod opentype;

use crate::opentype::OpenTypeHelpers;

macro_rules! header {
    ($name:literal: $value:literal) => {
        concat!($name, ": ", $value).parse::<Header>().unwrap()
    };
}

fn main() {
    let server = Server::http("127.0.0.1:18412").unwrap();
    let fc = fontconfig::init().unwrap();

    let get_font_file = |pattern: &Pattern| -> Option<FontFile> {
        pattern.file().map(|path| FontFile {
            file: path.into(),
            family: pattern.family().unwrap_or("").into(),
            style: pattern.style().unwrap_or("").into(),
            postscript: pattern.postscript_name().unwrap_or("").into(),
            weight: pattern.os_weight_class().unwrap_or(400),
            italic: pattern
                .slant()
                .map(|slant| slant != FC_SLANT_ROMAN)
                .unwrap_or(false),
            width: pattern.os_width_class().unwrap_or(5),
            variation_axes: None,
        })
    };

    let open_font_file = |path: &str| -> Option<File> {
        let path = Path::new(path);
        if fc.font_dirs().flatten().any(|dir| path.starts_with(dir)) {
            File::open(path).ok()
        } else {
            None
        }
    };

    for request in server.incoming_requests() {
        let uri = URIReference::try_from(request.url()).unwrap();
        match uri.path().to_string().as_str() {
            "/figma/font-files" => {
                let font_set = fc.list_fonts(&Pattern::new(), None);
                let font_files = font_set
                    .iter()
                    .flat_map(|pattern| get_font_file(&pattern))
                    .into_group_map_by(|font_file| font_file.file.to_owned());

                let payload = FontFilesPayload {
                    version: 20,
                    font_files,
                };

                if let Ok(payload) = serde_json::to_string(&payload) {
                    let response = Response::from_string(payload)
                        .with_header(header!("Content-Type": "application/json"))
                        .with_header(
                            header!("Access-Control-Allow-Origin": "https://www.figma.com"),
                        );
                    let _ = request.respond(response);
                } else {
                    let _ = request.respond(Response::empty(500));
                }
            }

            "/figma/font-file" => {
                let query = uri.query().map(|query| query.as_bytes()).unwrap_or(&[]);
                let params: HashMap<_, _> = form_urlencoded::parse(query).collect();

                if let Some(path) = params.get("file") {
                    if let Some(file) = open_font_file(path) {
                        let response = Response::from_file(file)
                            .with_header(header!("Content-Type": "application/octet-stream"))
                            .with_header(
                                header!("Access-Control-Allow-Origin": "https://www.figma.com"),
                            );
                        let _ = request.respond(response);
                    } else {
                        let _ = request.respond(Response::empty(404));
                    }
                } else {
                    let _ = request.respond(Response::empty(400));
                }
            }

            _ => {
                let _ = request.respond(Response::empty(404));
            }
        };
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FontFilesPayload {
    version: i32,
    font_files: HashMap<String, Vec<FontFile>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FontFile {
    #[serde(skip)]
    file: String,
    family: String,
    postscript: String,
    style: String,
    weight: i32,
    italic: bool,
    width: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    variation_axes: Option<Vec<VariationAxis>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VariationAxis {
    name: String,
    tag: String,
    value: f64,
    min: f64,
    max: f64,
    default: f64,
    hidden: bool,
}
