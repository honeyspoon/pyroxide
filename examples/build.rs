use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mojo_dir = manifest_dir.join("mojo");
    let out_dir = manifest_dir.join("target").join("mojo-libs");

    std::fs::create_dir_all(&out_dir).unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    let mojo = find_mojo();

    for entry in std::fs::read_dir(&mojo_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "mojo") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let dylib = out_dir.join(format!("lib{stem}.dylib"));

            println!("cargo:rerun-if-changed={}", path.display());

            let status = Command::new(&mojo)
                .args(["build", "--emit", "shared-lib", "-o"])
                .arg(&dylib)
                .arg(&path)
                .status()
                .unwrap_or_else(|e| panic!("failed to run {}: {e}", mojo.display()));

            if !status.success() {
                panic!("mojo build failed for {}", path.display());
            }

            println!("cargo:rustc-link-lib=dylib={stem}");
        }
    }
}

/// Find the `mojo` binary. Checks `MOJO_PATH` env, then common locations, then PATH.
fn find_mojo() -> PathBuf {
    if let Ok(p) = std::env::var("MOJO_PATH") {
        return PathBuf::from(p);
    }

    let home = PathBuf::from(std::env::var("HOME").unwrap_or_default());
    for subpath in ["/.pixi/bin/mojo", "/.modular/bin/mojo"] {
        let candidate = home.join(&subpath[1..]); // strip leading /
        if candidate.is_file() {
            return candidate;
        }
    }

    PathBuf::from("mojo")
}
