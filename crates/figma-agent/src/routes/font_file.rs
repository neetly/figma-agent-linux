use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{error, get, web, Responder};
use figma_agent::FC;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    file: PathBuf,
}

#[get("/font-file")]
pub async fn font_file(query: web::Query<Query>) -> impl Responder {
    let is_valid = FC
        .font_dirs()
        .flatten()
        .any(|dir| query.file.starts_with(dir));

    if is_valid {
        Ok(NamedFile::open(&query.file))
    } else {
        Err(error::ErrorNotFound("Not Found"))
    }
}
