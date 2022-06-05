use fontconfig::Config;
use freetype::Library;
use lazy_static::lazy_static;

mod font;
mod font_cache;
mod helpers;
mod payload;

pub use font::*;
pub use font_cache::*;
pub use helpers::*;
pub use payload::*;
use xdg::BaseDirectories;

lazy_static! {
    pub static ref XDG_DIRS: BaseDirectories = BaseDirectories::with_prefix("figma-agent").unwrap();
    pub static ref FONT_CACHE: FontCache =
        FontCache::new(XDG_DIRS.place_cache_file("fonts.json").unwrap());
    pub static ref FC: Config = fontconfig::init().unwrap();
    pub static ref FT: Library = freetype::init().unwrap();
}
