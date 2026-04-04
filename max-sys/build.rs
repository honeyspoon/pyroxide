use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let crate_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    let bindings = bindgen::Builder::default()
        .header(
            crate_dir
                .join("wrapper.h")
                .to_str()
                .expect("wrapper.h path is not valid UTF-8"),
        )
        .clang_arg(format!("-I{}/headers", crate_dir.display()))
        .allowlist_type("M_.*")
        .allowlist_function("M_.*")
        .allowlist_var("M_.*")
        .derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        .derive_eq(true)
        .use_core()
        .generate()
        .expect("failed to generate MAX C API bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings");
}
