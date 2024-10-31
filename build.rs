extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system libgpod and glib
    println!("cargo:rustc-link-lib=gpod");
    println!("cargo:rustc-link-lib=glib-2.0");

    // Use pkg-config to find the library path
    let glib = pkg_config::Config::new()
        .probe("glib-2.0")
        .expect("Could not find glib-2.0");

    let mut builder = bindgen::Builder::default()
        .header("./libgpod/src/itdb.h"); // Path to the libgpod header file

    for include_path in glib.include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path.to_string_lossy()));
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
