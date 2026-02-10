//! Safe Rust bindings for the ncnn neural network inference framework.
//!
//! This module wraps the ncnn C API (matching the version bundled with sherpa-ncnn 2.1.15)
//! and provides safe, idiomatic Rust types for `Net`, `Mat`, `Extractor`, and `Option`.
//!
//! # Example
//!
//! ```no_run
//! use sherpa_ncnn::ncnn::{Net, Mat};
//!
//! let mut net = Net::new();
//! net.load_param("model.param").unwrap();
//! net.load_model("model.bin").unwrap();
//!
//! let input = Mat::new_1d(224, None);
//! let mut ex = net.create_extractor();
//! ex.input("data", &input).unwrap();
//!
//! let mut output = Mat::new();
//! ex.extract("output", &mut output).unwrap();
//! ```

mod allocator;
mod extractor;
mod mat;
mod net;
mod option;

pub use allocator::Allocator;
pub use extractor::Extractor;
pub use mat::Mat;
pub use net::Net;
pub use option::Option as NcnnOption;

/// Re-export the sys crate for advanced usage
pub use ncnn_sys as sys;

/// Get the ncnn library version string.
pub fn version() -> &'static str {
    unsafe {
        let ptr = ncnn_sys::ncnn_version();
        if ptr.is_null() {
            "unknown"
        } else {
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("unknown")
        }
    }
}
