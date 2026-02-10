//! Raw FFI bindings to sherpa-ncnn C API
//!
//! This crate provides low-level FFI bindings to the sherpa-ncnn library.
//! For a safe, idiomatic Rust API, use the `sherpa_ncnn` crate (package `sherpa-ncnn-rs`).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Include auto-generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_exist() {
        // Basic sanity check that bindings were generated
        // The actual struct sizes will vary by platform
        assert!(std::mem::size_of::<SherpaNcnnRecognizerConfig>() > 0);
    }
}
