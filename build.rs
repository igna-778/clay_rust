use std::env;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    cc::Build::new()
        .file("clay/clay.c")
        .shared_flag(true)
        .out_dir(out_path)
        .compile("clay");
}
