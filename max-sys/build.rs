use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header(crate_dir.join("wrapper.h").to_str().unwrap())
        .clang_arg(format!("-I{}/headers", crate_dir.display()))
        .allowlist_type("M_.*")
        .allowlist_function("M_.*")
        .allowlist_var("M_.*")
        // Derive common traits
        .derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        .derive_eq(true)
        // Use core types
        .use_core()
        .generate()
        .expect("Failed to generate MAX C API bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write bindings");
}
