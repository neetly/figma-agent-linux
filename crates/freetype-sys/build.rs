use std::{env, path::PathBuf};

use bindgen::{builder, CargoCallbacks};
use pkg_config::probe_library;

fn main() {
    println!("cargo:rerun-if-changed=./include/freetype.h");

    let library = probe_library("freetype2").unwrap();
    let clang_args: Vec<_> = library
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.display()))
        .collect();

    let bindings = builder()
        .header("./include/freetype.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .clang_args(clang_args)
        .ctypes_prefix("::libc")
        .allowlist_file(r".+[/\\]freetype[/\\].+")
        .generate()
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_dir.join("bindings.rs")).unwrap();
}
