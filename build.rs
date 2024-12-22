use std::{env, path::PathBuf};

fn main() {

    let clay_path = PathBuf::from("clay.h")
        .canonicalize()
        .expect("Cannot canonicalize clay.h path");
    let clay_path_str = clay_path.to_str().expect("clay.h path is not valid string");

    #[cfg(feature = "build-clay")]
    {
        use std::fs;

        let env_provided_path = env::var("CLAY_HEADER_PATH").unwrap_or("clay/clay.h".to_string());

        let clay_repo_path = PathBuf::from(env_provided_path)
            .canonicalize()
            .expect("Cannot canonicalize clay.h path\nEather clone the clay repository at the root of the projet, or set the `CLAY_HEADER_PATH` to a valid clay.h file");
        
        fs::copy(clay_repo_path, &clay_path).expect("Failed to copy clay.h from repo to root\nEather clone the clay repository at the root of the projet, or set the `CLAY_HEADER_PATH` to a valid clay.h file");
    }

    let mut cc_builder = cc::Build::new();
    #[allow(unused_mut)]
    let mut bindgen_builder = bindgen::Builder::default()
        .header(clay_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    
    cc_builder.file("build.c");
    #[cfg(feature = "disable-culling")]
    {
        cc_builder.define("CLAY_DISABLE_CULLING", "");
        bindgen_builder = bindgen_builder.clang_arg("-DCLAY_DISABLE_CULLING");
    }
    #[cfg(feature = "wasm")]
    {
        cc_builder.define("CLAY_WASM", "");
        bindgen_builder = bindgen_builder.clang_arg("-DCLAY_WASM");
    }
    #[cfg(feature = "debug")]
    {
        cc_builder.define("CLAY_DEBUG", "");
        bindgen_builder = bindgen_builder.clang_arg("-DCLAY_DEBUG");
    }
    
    cc_builder.compile("clay");

    let bindings = bindgen_builder.generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
