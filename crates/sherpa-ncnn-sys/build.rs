//! Build script for sherpa-ncnn-sys
//! Handles downloading pre-built binaries and generating FFI bindings

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

#[path = "src/download_binaries.rs"]
#[cfg(feature = "download-binaries")]
mod download_binaries;

macro_rules! debug_log {
    ($($arg:tt)*) => {
        if std::env::var("SHERPA_BUILD_DEBUG").unwrap_or_default() == "1" {
            println!("cargo:warning=[DEBUG] {}", format!($($arg)*));
        }
    };
}

fn link_lib(lib: &str, is_dynamic: bool) {
    let lib_kind = if is_dynamic { "dylib" } else { "static" };
    debug_log!("cargo:rustc-link-lib={}={}", lib_kind, lib);
    println!("cargo:rustc-link-lib={lib_kind}={lib}");
}

fn link_framework(framework: &str) {
    debug_log!("cargo:rustc-link-lib=framework={}", framework);
    println!("cargo:rustc-link-lib=framework={framework}");
}

fn add_search_path<P: AsRef<Path>>(path: P) {
    debug_log!("cargo:rustc-link-search={}", path.as_ref().display());
    println!("cargo:rustc-link-search={}", path.as_ref().display());
}

fn get_cargo_target_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let profile = env::var("PROFILE")?;
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}

fn copy_file(src: &Path, dst: &Path) {
    if let Err(_) = std::fs::hard_link(src, dst) {
        std::fs::copy(src, dst)
            .unwrap_or_else(|_| panic!("Failed to copy {} to {}", src.display(), dst.display()));
    }
}

fn extract_lib_names(lib_dir: &Path, is_dynamic: bool, target_os: &str) -> Vec<String> {
    let lib_pattern = if target_os == "windows" {
        "*.lib"
    } else if target_os == "macos" || target_os == "ios" {
        if is_dynamic { "*.dylib" } else { "*.a" }
    } else {
        if is_dynamic { "*.so" } else { "*.a" }
    };

    let pattern = lib_dir.join(lib_pattern);
    debug_log!("Extract libs from {}", pattern.display());

    let mut lib_names = Vec::new();
    for entry in glob::glob(pattern.to_str().unwrap()).unwrap() {
        if let Ok(path) = entry {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let lib_name = stem.strip_prefix("lib").unwrap_or(stem);
            lib_names.push(lib_name.to_string());
        }
    }
    lib_names
}

fn extract_lib_assets(lib_dir: &Path, target_os: &str) -> Vec<PathBuf> {
    let pattern = if target_os == "windows" {
        "*.dll"
    } else if target_os == "macos" || target_os == "ios" {
        "*.dylib"
    } else {
        "*.so"
    };

    let pattern = lib_dir.join(pattern);
    let mut files = Vec::new();
    for entry in glob::glob(pattern.to_str().unwrap()).unwrap() {
        if let Ok(path) = entry {
            files.push(path);
        }
    }
    files
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=dist.json");
    println!("cargo:rerun-if-env-changed=SHERPA_NCNN_LIB_PATH");
    println!("cargo:rerun-if-env-changed=SHERPA_BUILD_DEBUG");

    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    debug_log!("TARGET: {}", target);
    debug_log!("TARGET_OS: {}", target_os);
    debug_log!("OUT_DIR: {}", out_dir.display());

    let is_dynamic = true; // Always use dynamic linking for sherpa-ncnn

    // Try to get library path from environment or download
    let lib_path: Option<PathBuf> = if let Ok(path) = env::var("SHERPA_NCNN_LIB_PATH") {
        Some(PathBuf::from(path))
    } else {
        #[cfg(feature = "download-binaries")]
        {
            download_binaries::download_and_extract(&target, &out_dir)
        }
        #[cfg(not(feature = "download-binaries"))]
        {
            None
        }
    };

    let lib_path = lib_path.expect(
        "sherpa-ncnn library not found. Set SHERPA_NCNN_LIB_PATH or enable download-binaries feature"
    );

    debug_log!("Using library path: {}", lib_path.display());

    // Add library search path
    let lib_dir = lib_path.join("lib");
    if lib_dir.exists() {
        add_search_path(&lib_dir);
    } else {
        add_search_path(&lib_path);
    }

    // Extract and link libraries
    let search_dir = if lib_dir.exists() { &lib_dir } else { &lib_path };
    let libs = extract_lib_names(search_dir, is_dynamic, &target_os);

    debug_log!("Found libraries: {:?}", libs);

    for lib in &libs {
        link_lib(lib, is_dynamic);
    }

    // Platform-specific linking
    if target_os == "macos" || target_os == "ios" {
        link_framework("Foundation");
        link_lib("c++", true);
    } else if target_os == "linux" || target.contains("android") {
        link_lib("stdc++", true);
    }

    // Generate bindings
    let include_dir = lib_path.join("include");
    let header_path = if include_dir.exists() {
        include_dir.join("sherpa-ncnn/c-api/c-api.h")
    } else {
        lib_path.join("sherpa-ncnn/c-api/c-api.h")
    };

    if header_path.exists() {
        debug_log!("Generating bindings from {}", header_path.display());

        let bindings = bindgen::Builder::default()
            .header(header_path.to_str().unwrap())
            .clang_arg(format!("-I{}", include_dir.display()))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .derive_default(true)
            .generate()
            .expect("Failed to generate bindings");

        let bindings_path = out_dir.join("bindings.rs");
        bindings.write_to_file(&bindings_path).expect("Failed to write bindings");
        debug_log!("Bindings written to {}", bindings_path.display());
    } else {
        debug_log!("Header not found at {}, using pre-generated bindings", header_path.display());
        // Copy pre-generated bindings if header not found
        let src_bindings = PathBuf::from(&manifest_dir).join("src/bindings.rs");
        if src_bindings.exists() {
            std::fs::copy(&src_bindings, out_dir.join("bindings.rs"))
                .expect("Failed to copy pre-generated bindings");
        }
    }

    // Copy dynamic libraries to target directory
    if is_dynamic {
        if let Ok(target_dir) = get_cargo_target_dir() {
            let assets = extract_lib_assets(search_dir, &target_os);
            for asset in assets {
                let filename = asset.file_name().unwrap();
                let dst = target_dir.join(filename);
                if !dst.exists() {
                    copy_file(&asset, &dst);
                }
                // Also copy to deps for tests
                let deps_dst = target_dir.join("deps").join(filename);
                if !deps_dst.exists() {
                    let _ = std::fs::create_dir_all(target_dir.join("deps"));
                    copy_file(&asset, &deps_dst);
                }
            }
        }
    }
}
