//! Raw FFI bindings to ncnn C API
//!
//! This crate provides low-level FFI bindings to the ncnn inference framework's C API.
//! The bindings match ncnn commit c4193aadbbb56582aa87b1850dd3d98fb8fd936d,
//! which is the version bundled with sherpa-ncnn 2.1.15.
//!
//! For a safe, idiomatic Rust API, use the `ncnn` crate instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_exist() {
        // Verify key opaque pointer types are defined
        assert!(std::mem::size_of::<*mut __ncnn_net_t>() > 0);
        assert!(std::mem::size_of::<*mut __ncnn_mat_t>() > 0);
    }
}
