
fn main() {    
    cc::Build::new()
        .file("build.c")
        .compile("clay");

    #[cfg(feature = "build-clay")]
    {
        use std::{env, path::PathBuf};

        let clay_path = PathBuf::from("clay/clay.h")
            .canonicalize().expect("Cannot canonicalize clay.h path");
        let clay_path_str = clay_path.to_str().expect("clay.h path is not valid string");
    
        let bindings = bindgen::Builder::default()
            .header(clay_path_str)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");
    
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
