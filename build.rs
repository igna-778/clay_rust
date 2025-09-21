use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Only run the Gitea fetch if this is the primary package
    let is_primary = std::env::var("CARGO_PRIMARY_PACKAGE").is_ok();

    if is_primary {
        // Load token from environment
        let token = std::env::var("GITEA_TOKEN")
            .expect("Set GITEA_TOKEN in your environment");

        let url = "http://192.168.1.158:3000/api/v1/repos/Igna/clay_exp/raw/main/clay.h";
        let local_path = Path::new("clay.h");

        // Fetch clay.h with authorization
        let response = ureq::get(url)
            .set("Authorization", &format!("token {}", token))
            .call()
            .expect("Failed to fetch clay.h");

        let text = response.into_string().expect("Failed to read response");

        // Compare with local copy
        let needs_update = match fs::read_to_string(local_path) {
            Ok(existing) => existing != text,
            Err(_) => true, // missing file
        };

        if needs_update {
            let mut file = fs::File::create(local_path).unwrap();
            file.write_all(text.as_bytes()).unwrap();
            println!("cargo:warning=Updated clay.h from Gitea");
        }

        // Ensure Cargo rebuilds when clay.h changes
        println!("cargo:rerun-if-changed=clay.h");
    }

    // Compile native code
    if target_os == "windows" {
        cc::Build::new()
            .file("build.cpp")
            .warnings(false)
            .std("c++20")
            .compile("clay");
    } else {
        cc::Build::new()
            .file("build.c")
            .warnings(false)
            .compile("clay");
    }
}
