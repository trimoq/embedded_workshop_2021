extern crate cc;

use std::path::PathBuf;

fn main() {

    // Compile external files

    cc::Build::new()
        .file("lib/point.c")
        .compile("libpoint.a");

    // Rerun only if needed

    println!("cargo:rerun-if-changed=lib/point.c");
    println!("cargo:rerun-if-changed=lib/point.h");


    // Create bindings

    let bindings = bindgen::Builder::default()
        .header("lib/point.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

        
    let out_path = PathBuf::from("./lib/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
