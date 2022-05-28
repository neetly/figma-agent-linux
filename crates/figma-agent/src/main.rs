use std::{collections::HashMap, fs::File, path::Path};

use fontconfig::{Config, Pattern, FC_SLANT_ROMAN};
use freetype::Library;
use itertools::Itertools;
use serde::Serialize;
use tiny_http::{Header, Response, Server};
use uriparse::URIReference;

macro_rules! header {
    ($name:literal: $value:literal) => {
        concat!($name, ": ", $value).parse::<Header>().unwrap()
    };
}

fn main() {
    let server = Server::http("127.0.0.1:18412").unwrap();
    let fc = Config::init().unwrap();
    let _ft = Library::init().unwrap();

    for request in server.incoming_requests() {
        let uri = URIReference::try_from(request.url()).unwrap();
        match uri.path().to_string().as_str() {
            "/figma/font-files" => {
                let font_set = fc.list_fonts(&Pattern::new(), None);

                let font_files = font_set
                    .iter()
                    .map(|pattern| {
                        (
                            pattern.file().unwrap_or("").to_string(),
                            FontFile {
                                family: pattern.family().unwrap_or("").to_string(),
                                postscript: pattern.postscript_name().unwrap_or("").to_string(),
                                style: pattern.style().unwrap_or("").to_string(),
                                weight: pattern.opentype_weight().unwrap_or(400),
                                italic: pattern
                                    .slant()
                                    .map(|slant| slant != FC_SLANT_ROMAN)
                                    .unwrap_or(false),
                                width: pattern.opentype_width().unwrap_or(5),
                                variation_axes: None,
                            },
                        )
                    })
                    .into_group_map();

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
                    let path = Path::new(path.as_ref());
                    let is_path_valid = fc.font_dirs().flatten().any(|dir| path.starts_with(dir));

                    if is_path_valid {
                        if let Ok(file) = File::open(path) {
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
    family: String,
    postscript: String,
    style: String,
    weight: i32,
    italic: bool,
    width: i32,
    variation_axes: Option<Vec<VariationAxis>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VariationAxis {
    name: String,
    tag: String,
    value: i32,
    default: i32,
    min: i32,
    max: i32,
    hidden: bool,
}
