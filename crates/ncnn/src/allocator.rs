use std::ptr;

/// Memory allocator for ncnn Mat operations.
pub struct Allocator {
    raw: ncnn_sys::ncnn_allocator_t,
}

/// Allocator type selection.
pub enum AllocatorType {
    /// Pool allocator (thread-safe with mutex).
    Pool,
    /// Unlocked pool allocator (no mutex, faster but not thread-safe).
    UnlockedPool,
}

impl Allocator {
    /// Create a new allocator of the specified type.
    pub fn new(typ: AllocatorType) -> Self {
        let raw = unsafe {
            match typ {
                AllocatorType::Pool => ncnn_sys::ncnn_allocator_create_pool_allocator(),
                AllocatorType::UnlockedPool => {
                    ncnn_sys::ncnn_allocator_create_unlocked_pool_allocator()
                }
            }
        };
        Self { raw }
    }

    /// Get the raw pointer (for passing to ncnn C API).
    pub fn as_ptr(&self) -> ncnn_sys::ncnn_allocator_t {
        self.raw
    }

    /// Return a null allocator pointer (uses ncnn default allocator).
    pub fn null() -> ncnn_sys::ncnn_allocator_t {
        ptr::null_mut()
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ncnn_sys::ncnn_allocator_destroy(self.raw) }
        }
    }
}
