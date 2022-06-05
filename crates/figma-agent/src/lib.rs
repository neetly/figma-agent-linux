use fontconfig::Config;
use freetype::Library;
use lazy_static::lazy_static;

mod helpers;
mod payload;

pub use helpers::*;
pub use payload::*;

lazy_static! {
    pub static ref FC: Config = fontconfig::init().unwrap();
    pub static ref FT: Library = freetype::init().unwrap();
}
