use std::ptr;

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
    let result = unsafe { FcInit() };
    if result != FcFalse {
        Some(unsafe { Config::from_raw(ptr::null_mut()) })
    } else {
        None
    }
}

pub fn finalize() {
    unsafe { FcFini() };
}
