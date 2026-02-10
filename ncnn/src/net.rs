use std::ffi::CString;

use crate::extractor::Extractor;

/// ncnn Net — neural network model container.
///
/// Load param and model files, then create an `Extractor` to run inference.
pub struct Net {
    raw: ncnn_sys::ncnn_net_t,
}

impl Net {
    /// Create a new empty Net.
    pub fn new() -> Self {
        let raw = unsafe { ncnn_sys::ncnn_net_create() };
        Self { raw }
    }

    /// Get the raw pointer.
    pub fn as_ptr(&self) -> ncnn_sys::ncnn_net_t {
        self.raw
    }

    /// Set option for this net.
    pub fn set_option(&mut self, opt: &crate::NcnnOption) {
        unsafe { ncnn_sys::ncnn_net_set_option(self.raw, opt.as_ptr()) }
    }

    /// Load network param from a text file.
    pub fn load_param(&mut self, path: &str) -> Result<(), i32> {
        let c_path = CString::new(path).map_err(|_| -1)?;
        let ret = unsafe { ncnn_sys::ncnn_net_load_param(self.raw, c_path.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Load network param from a binary file.
    pub fn load_param_bin(&mut self, path: &str) -> Result<(), i32> {
        let c_path = CString::new(path).map_err(|_| -1)?;
        let ret = unsafe { ncnn_sys::ncnn_net_load_param_bin(self.raw, c_path.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Load network model (weights) from a file.
    pub fn load_model(&mut self, path: &str) -> Result<(), i32> {
        let c_path = CString::new(path).map_err(|_| -1)?;
        let ret = unsafe { ncnn_sys::ncnn_net_load_model(self.raw, c_path.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Load param from a string in memory.
    pub fn load_param_memory(&mut self, mem: &str) -> Result<(), i32> {
        let c_mem = CString::new(mem).map_err(|_| -1)?;
        let ret = unsafe { ncnn_sys::ncnn_net_load_param_memory(self.raw, c_mem.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Load binary param from memory.
    pub fn load_param_bin_memory(&mut self, mem: &[u8]) -> Result<(), i32> {
        let ret = unsafe { ncnn_sys::ncnn_net_load_param_bin_memory(self.raw, mem.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Load model (weights) from memory.
    pub fn load_model_memory(&mut self, mem: &[u8]) -> Result<(), i32> {
        let ret = unsafe { ncnn_sys::ncnn_net_load_model_memory(self.raw, mem.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Clear all loaded data.
    pub fn clear(&mut self) {
        unsafe { ncnn_sys::ncnn_net_clear(self.raw) }
    }

    /// Get the number of input blobs.
    pub fn input_count(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_net_get_input_count(self.raw) }
    }

    /// Get the number of output blobs.
    pub fn output_count(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_net_get_output_count(self.raw) }
    }

    /// Get the name of the i-th input blob.
    pub fn input_name(&self, i: i32) -> Option<&str> {
        unsafe {
            let ptr = ncnn_sys::ncnn_net_get_input_name(self.raw, i);
            if ptr.is_null() {
                None
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    /// Get the name of the i-th output blob.
    pub fn output_name(&self, i: i32) -> Option<&str> {
        unsafe {
            let ptr = ncnn_sys::ncnn_net_get_output_name(self.raw, i);
            if ptr.is_null() {
                None
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    /// Create an extractor for running inference.
    pub fn create_extractor(&self) -> Extractor {
        let raw = unsafe { ncnn_sys::ncnn_extractor_create(self.raw) };
        unsafe { Extractor::from_raw(raw) }
    }
}

impl Default for Net {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ncnn_sys::ncnn_net_destroy(self.raw) }
        }
    }
}
