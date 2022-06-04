pub use fontconfig_sys::*;

pub use config::*;
pub use font_set::*;
pub use object_set::*;
pub use pattern::*;
pub use str_list::*;
pub use str_set::*;

pub mod config;
pub mod font_set;
pub mod object_set;
pub mod pattern;
pub mod str_list;
pub mod str_set;

pub fn init() -> Option<Config> {
    let raw = unsafe { FcInitLoadConfigAndFonts() };
    if !raw.is_null() {
        Some(unsafe { Config::from_raw(raw) })
    } else {
        None
    }
}

pub fn finalize() {
    unsafe { FcFini() };
}
