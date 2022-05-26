use std::{collections::HashMap, fs::File};

use fontconfig::{Config, Pattern, FC_SLANT_ROMAN};
use itertools::Itertools;
use serde_json::json;
use tiny_http::{Header, Response, Server};
use uriparse::URIReference;

macro_rules! header {
    ($name:literal: $value:literal) => {
        concat!($name, ": ", $value).parse::<Header>().unwrap()
    };
}

fn main() {
    let server = Server::http("127.0.0.1:18412").unwrap();
    let config = Config::new();

    for request in server.incoming_requests() {
        let uri = URIReference::try_from(request.url()).unwrap();
        match uri.path().to_string().as_str() {
            "/figma/font-files" => {
                let font_set = config.list_fonts(&Pattern::new(), None);

                let font_files = font_set
                    .iter()
                    .map(|pattern| {
                        (
                            pattern.file().unwrap_or("").to_owned(),
                            json!({
                                "family": pattern.family().unwrap_or(""),
                                "postscript": pattern.postscript_name().unwrap_or(""),
                                "style": pattern.style().unwrap_or(""),
                                "weight": pattern.opentype_weight().unwrap_or(400),
                                "italic": pattern.slant().map(|slant| slant != FC_SLANT_ROMAN).unwrap_or(false),
                                "width": pattern.opentype_width().unwrap_or(5),
                            }),
                        )
                    })
                    .into_group_map();

                let result = json!({
                    "version": 20,
                    "fontFiles": font_files,
                });

                if let Ok(payload) = serde_json::to_string(&result) {
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
                    if let Ok(file) = File::open(path.as_ref()) {
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
