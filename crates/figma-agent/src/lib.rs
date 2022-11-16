use std::cell::RefCell;

use lazy_static::lazy_static;
use parking_lot::ReentrantMutex;
use xdg::BaseDirectories;

mod config;
mod font;
mod font_cache;
mod helpers;

pub use config::*;
pub use font::*;
pub use font_cache::*;
pub use helpers::*;

lazy_static! {
    pub static ref XDG_DIRS: BaseDirectories = BaseDirectories::with_prefix("figma-agent").unwrap();
    pub static ref CONFIG: Config = Config::new();
    pub static ref FONT_CACHE: ReentrantMutex<RefCell<FontCache>> =
        ReentrantMutex::new(RefCell::new(FontCache::new(
            XDG_DIRS.place_cache_file("fonts.json").unwrap()
        )));
    pub static ref FC: fontconfig::Config = fontconfig::init().unwrap();
    pub static ref FT: freetype::Library = freetype::init().unwrap();
}
