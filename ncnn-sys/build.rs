//! Build script for ncnn-sys
//! Links against libncnn (with NCNN_C_API enabled) from the sherpa-ncnn prebuilt package.

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=NCNN_LIB_PATH");
    println!("cargo:rerun-if-env-changed=SHERPA_NCNN_LIB_PATH");

    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // ncnn-sys reuses the same library directory as sherpa-ncnn-sys.
    // The libncnn.{a,dylib,so} is already present there.
    // It just needs to be rebuilt with NCNN_C_API=ON.
    //
    // Library search path priority:
    //   1. NCNN_LIB_PATH env var
    //   2. SHERPA_NCNN_LIB_PATH env var (shared with sherpa-ncnn-sys)
    //   3. DEP_SHERPA_NCNN_LIB_PATH (set by sherpa-ncnn-sys via cargo:lib-path)
    let lib_path = env::var("NCNN_LIB_PATH")
        .or_else(|_| env::var("SHERPA_NCNN_LIB_PATH"))
        .or_else(|_| env::var("DEP_SHERPA_NCNN_LIB_PATH"))
        .ok()
        .map(PathBuf::from);

    if let Some(ref path) = lib_path {
        let lib_dir = path.join("lib");
        if lib_dir.exists() {
            println!("cargo:rustc-link-search={}", lib_dir.display());
        } else {
            println!("cargo:rustc-link-search={}", path.display());
        }
    }

    // ncnn-sys does NOT link libncnn itself — that's already done by sherpa-ncnn-sys.
    // We only need to generate/provide the bindings.
    // If you use ncnn-sys standalone (without sherpa-ncnn-sys), uncomment:
    // println!("cargo:rustc-link-lib=dylib=ncnn");

    // Generate or copy bindings
    let is_cross_compiling = target != host;

    let include_dir = lib_path
        .as_ref()
        .map(|p| p.join("include"))
        .unwrap_or_default();

    let header_path = if include_dir.join("ncnn/c_api.h").exists() {
        Some(include_dir.join("ncnn/c_api.h"))
    } else {
        None
    };

    if is_cross_compiling || header_path.is_none() {
        // Use pre-generated bindings
        let src_bindings = PathBuf::from(&manifest_dir).join("src/bindings.rs");
        if src_bindings.exists() {
            std::fs::copy(&src_bindings, out_dir.join("bindings.rs"))
                .expect("Failed to copy pre-generated bindings");
        } else {
            panic!("Pre-generated bindings not found at {:?}", src_bindings);
        }
    } else if let Some(header) = header_path {
        let bindings = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .clang_arg(format!("-I{}", include_dir.display()))
            // Define NCNN_C_API=1 so the header is active
            .clang_arg("-DNCNN_C_API=1")
            .clang_arg("-DNCNN_STRING=1")
            .clang_arg("-DNCNN_STDIO=1")
            .clang_arg("-DNCNN_PIXEL=1")
            .clang_arg("-DNCNN_EXPORT=")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .derive_default(true)
            .generate()
            .expect("Failed to generate ncnn bindings");

        bindings
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Failed to write ncnn bindings");
    }
}
