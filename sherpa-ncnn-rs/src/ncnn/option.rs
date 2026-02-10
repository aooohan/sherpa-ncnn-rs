/// ncnn inference option configuration.
///
/// Controls thread count, Vulkan compute, allocators, etc.
pub struct Option {
    raw: ncnn_sys::ncnn_option_t,
    owned: bool,
}

impl Option {
    /// Create a new option with default values.
    pub fn new() -> Self {
        let raw = unsafe { ncnn_sys::ncnn_option_create() };
        Self { raw, owned: true }
    }

    /// Wrap a raw option pointer (non-owning).
    ///
    /// # Safety
    /// The caller must ensure `raw` is valid for the lifetime of the returned `Option`.
    #[allow(dead_code)]
    pub(crate) unsafe fn from_raw(raw: ncnn_sys::ncnn_option_t) -> Self {
        Self { raw, owned: false }
    }

    /// Get the raw pointer.
    pub fn as_ptr(&self) -> ncnn_sys::ncnn_option_t {
        self.raw
    }

    /// Get the number of threads.
    pub fn num_threads(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_option_get_num_threads(self.raw) }
    }

    /// Set the number of threads.
    pub fn set_num_threads(&mut self, num_threads: i32) {
        unsafe { ncnn_sys::ncnn_option_set_num_threads(self.raw, num_threads) }
    }

    /// Get whether Vulkan compute is enabled.
    pub fn use_vulkan_compute(&self) -> bool {
        unsafe { ncnn_sys::ncnn_option_get_use_vulkan_compute(self.raw) != 0 }
    }

    /// Enable or disable Vulkan compute.
    pub fn set_use_vulkan_compute(&mut self, enable: bool) {
        unsafe { ncnn_sys::ncnn_option_set_use_vulkan_compute(self.raw, enable as i32) }
    }

    /// Get whether local pool allocator is used.
    pub fn use_local_pool_allocator(&self) -> bool {
        unsafe { ncnn_sys::ncnn_option_get_use_local_pool_allocator(self.raw) != 0 }
    }

    /// Set whether to use local pool allocator.
    pub fn set_use_local_pool_allocator(&mut self, enable: bool) {
        unsafe { ncnn_sys::ncnn_option_set_use_local_pool_allocator(self.raw, enable as i32) }
    }

    /// Set blob allocator.
    pub fn set_blob_allocator(&mut self, allocator: &super::Allocator) {
        unsafe { ncnn_sys::ncnn_option_set_blob_allocator(self.raw, allocator.as_ptr()) }
    }

    /// Set workspace allocator.
    pub fn set_workspace_allocator(&mut self, allocator: &super::Allocator) {
        unsafe { ncnn_sys::ncnn_option_set_workspace_allocator(self.raw, allocator.as_ptr()) }
    }
}

impl Default for Option {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Option {
    fn drop(&mut self) {
        if self.owned && !self.raw.is_null() {
            unsafe { ncnn_sys::ncnn_option_destroy(self.raw) }
        }
    }
}
