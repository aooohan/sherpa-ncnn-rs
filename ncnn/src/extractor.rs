use std::ffi::CString;

use crate::mat::Mat;

/// ncnn Extractor — runs inference on a loaded Net.
///
/// Feed input blobs and extract output blobs by name or index.
pub struct Extractor {
    raw: ncnn_sys::ncnn_extractor_t,
}

impl Extractor {
    /// Wrap a raw ncnn_extractor_t pointer (takes ownership).
    ///
    /// # Safety
    /// The caller must ensure `raw` is valid and owned.
    pub(crate) unsafe fn from_raw(raw: ncnn_sys::ncnn_extractor_t) -> Self {
        Self { raw }
    }

    /// Get the raw pointer.
    pub fn as_ptr(&self) -> ncnn_sys::ncnn_extractor_t {
        self.raw
    }

    /// Set option for this extractor.
    pub fn set_option(&mut self, opt: &crate::NcnnOption) {
        unsafe { ncnn_sys::ncnn_extractor_set_option(self.raw, opt.as_ptr()) }
    }

    /// Feed input blob by name.
    pub fn input(&mut self, name: &str, mat: &Mat) -> Result<(), i32> {
        let c_name = CString::new(name).map_err(|_| -1)?;
        let ret =
            unsafe { ncnn_sys::ncnn_extractor_input(self.raw, c_name.as_ptr(), mat.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Extract output blob by name.
    pub fn extract(&mut self, name: &str, mat: &mut Mat) -> Result<(), i32> {
        let c_name = CString::new(name).map_err(|_| -1)?;
        let ret = unsafe {
            ncnn_sys::ncnn_extractor_extract(self.raw, c_name.as_ptr(), mat.as_mut_ptr())
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Feed input blob by index.
    pub fn input_index(&mut self, index: i32, mat: &Mat) -> Result<(), i32> {
        let ret = unsafe { ncnn_sys::ncnn_extractor_input_index(self.raw, index, mat.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    /// Extract output blob by index.
    pub fn extract_index(&mut self, index: i32, mat: &mut Mat) -> Result<(), i32> {
        let ret =
            unsafe { ncnn_sys::ncnn_extractor_extract_index(self.raw, index, mat.as_mut_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }
}

impl Drop for Extractor {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ncnn_sys::ncnn_extractor_destroy(self.raw) }
        }
    }
}
