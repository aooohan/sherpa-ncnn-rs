use std::ffi::c_int;
use std::ptr;

/// Pixel format for `Mat::from_pixels*` methods.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelType {
    Rgb = ncnn_sys::NCNN_MAT_PIXEL_RGB,
    Bgr = ncnn_sys::NCNN_MAT_PIXEL_BGR,
    Gray = ncnn_sys::NCNN_MAT_PIXEL_GRAY,
    Rgba = ncnn_sys::NCNN_MAT_PIXEL_RGBA,
    Bgra = ncnn_sys::NCNN_MAT_PIXEL_BGRA,
}

/// Border type for `Mat::copy_make_border`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderType {
    Constant = ncnn_sys::NCNN_BORDER_CONSTANT,
    Replicate = ncnn_sys::NCNN_BORDER_REPLICATE,
    Reflect = ncnn_sys::NCNN_BORDER_REFLECT,
    Transparent = ncnn_sys::NCNN_BORDER_TRANSPARENT,
}

/// ncnn Mat — multi-dimensional array (tensor) for network input/output.
pub struct Mat {
    raw: ncnn_sys::ncnn_mat_t,
}

impl Mat {
    /// Create an empty Mat.
    pub fn new() -> Self {
        let raw = unsafe { ncnn_sys::ncnn_mat_create() };
        Self { raw }
    }

    /// Create a 1D Mat with width `w`.
    pub fn new_1d(w: i32, allocator: std::option::Option<&super::Allocator>) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_create_1d(w, alloc) };
        Self { raw }
    }

    /// Create a 2D Mat with width `w` and height `h`.
    pub fn new_2d(w: i32, h: i32, allocator: std::option::Option<&super::Allocator>) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_create_2d(w, h, alloc) };
        Self { raw }
    }

    /// Create a 3D Mat with width `w`, height `h`, and channels `c`.
    pub fn new_3d(
        w: i32,
        h: i32,
        c: i32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_create_3d(w, h, c, alloc) };
        Self { raw }
    }

    /// Create a 4D Mat with width `w`, height `h`, depth `d`, and channels `c`.
    pub fn new_4d(
        w: i32,
        h: i32,
        d: i32,
        c: i32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_create_4d(w, h, d, c, alloc) };
        Self { raw }
    }

    /// Create a 1D Mat wrapping external data. The data must outlive the Mat.
    ///
    /// # Safety
    /// `data` must point to valid memory of at least `w * sizeof(f32)` bytes.
    pub unsafe fn from_external_1d(
        w: i32,
        data: *mut f32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = ncnn_sys::ncnn_mat_create_external_1d(w, data as *mut _, alloc);
        Self { raw }
    }

    /// Create a 3D Mat wrapping external data. The data must outlive the Mat.
    ///
    /// # Safety
    /// `data` must point to valid memory of at least `w * h * c * sizeof(f32)` bytes.
    pub unsafe fn from_external_3d(
        w: i32,
        h: i32,
        c: i32,
        data: *mut f32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = ncnn_sys::ncnn_mat_create_external_3d(w, h, c, data as *mut _, alloc);
        Self { raw }
    }

    /// Wrap a raw ncnn_mat_t pointer (takes ownership).
    ///
    /// # Safety
    /// The caller must ensure `raw` is valid and owned.
    #[allow(dead_code)]
    pub(crate) unsafe fn from_raw(raw: ncnn_sys::ncnn_mat_t) -> Self {
        Self { raw }
    }

    /// Get the raw pointer.
    pub fn as_ptr(&self) -> ncnn_sys::ncnn_mat_t {
        self.raw
    }

    /// Get a mutable raw pointer.
    pub fn as_mut_ptr(&mut self) -> *mut ncnn_sys::ncnn_mat_t {
        &mut self.raw
    }

    /// Fill the Mat with a float value.
    pub fn fill(&mut self, v: f32) {
        unsafe { ncnn_sys::ncnn_mat_fill_float(self.raw, v) }
    }

    /// Clone this Mat.
    pub fn try_clone(&self, allocator: std::option::Option<&super::Allocator>) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_clone(self.raw, alloc) };
        Self { raw }
    }

    /// Number of dimensions (0, 1, 2, 3, or 4).
    pub fn dims(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_mat_get_dims(self.raw) }
    }

    /// Width.
    pub fn w(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_mat_get_w(self.raw) }
    }

    /// Height.
    pub fn h(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_mat_get_h(self.raw) }
    }

    /// Depth.
    pub fn d(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_mat_get_d(self.raw) }
    }

    /// Channels.
    pub fn c(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_mat_get_c(self.raw) }
    }

    /// Element size in bytes.
    pub fn elemsize(&self) -> usize {
        unsafe { ncnn_sys::ncnn_mat_get_elemsize(self.raw) }
    }

    /// Element packing count.
    pub fn elempack(&self) -> i32 {
        unsafe { ncnn_sys::ncnn_mat_get_elempack(self.raw) }
    }

    /// Per-channel step in element count.
    pub fn cstep(&self) -> usize {
        unsafe { ncnn_sys::ncnn_mat_get_cstep(self.raw) }
    }

    /// Whether this Mat is empty (no dimensions).
    pub fn is_empty(&self) -> bool {
        self.dims() == 0
    }

    /// Get raw data pointer.
    pub fn data(&self) -> *mut std::ffi::c_void {
        unsafe { ncnn_sys::ncnn_mat_get_data(self.raw) }
    }

    /// Get raw data pointer for a specific channel.
    pub fn channel_data(&self, c: i32) -> *mut std::ffi::c_void {
        unsafe { ncnn_sys::ncnn_mat_get_channel_data(self.raw, c) }
    }

    /// Get data as a float slice (only valid if elemsize == 4 and elempack == 1).
    ///
    /// # Safety
    /// Caller must ensure the Mat contains f32 data.
    pub unsafe fn as_float_slice(&self) -> &[f32] {
        let total = match self.dims() {
            1 => self.w() as usize,
            2 => (self.w() * self.h()) as usize,
            3 => self.cstep() * self.c() as usize,
            4 => self.cstep() * self.c() as usize,
            _ => 0,
        };
        if total == 0 {
            return &[];
        }
        std::slice::from_raw_parts(self.data() as *const f32, total)
    }

    /// Subtract mean and normalize.
    pub fn substract_mean_normalize(
        &mut self,
        mean_vals: std::option::Option<&[f32]>,
        norm_vals: std::option::Option<&[f32]>,
    ) {
        let mean_ptr = mean_vals.map_or(ptr::null(), |v| v.as_ptr());
        let norm_ptr = norm_vals.map_or(ptr::null(), |v| v.as_ptr());
        unsafe { ncnn_sys::ncnn_mat_substract_mean_normalize(self.raw, mean_ptr, norm_ptr) }
    }

    /// Reshape to 1D.
    pub fn reshape_1d(&self, w: i32, allocator: std::option::Option<&super::Allocator>) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_reshape_1d(self.raw, w, alloc) };
        Self { raw }
    }

    /// Reshape to 2D.
    pub fn reshape_2d(
        &self,
        w: i32,
        h: i32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_reshape_2d(self.raw, w, h, alloc) };
        Self { raw }
    }

    /// Reshape to 3D.
    pub fn reshape_3d(
        &self,
        w: i32,
        h: i32,
        c: i32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_reshape_3d(self.raw, w, h, c, alloc) };
        Self { raw }
    }

    /// Reshape to 4D.
    pub fn reshape_4d(
        &self,
        w: i32,
        h: i32,
        d: i32,
        c: i32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe { ncnn_sys::ncnn_mat_reshape_4d(self.raw, w, h, d, c, alloc) };
        Self { raw }
    }

    /// Create a Mat from pixel data.
    ///
    /// `pixels` must have at least `h * stride` bytes.
    /// `stride` is the number of bytes per row (often `w * channels`).
    pub fn from_pixels(
        pixels: &[u8],
        pixel_type: PixelType,
        w: i32,
        h: i32,
        stride: i32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe {
            ncnn_sys::ncnn_mat_from_pixels(
                pixels.as_ptr(),
                pixel_type as c_int,
                w,
                h,
                stride,
                alloc,
            )
        };
        Self { raw }
    }

    /// Create a Mat from pixel data with resize.
    ///
    /// `pixels` must have at least `h * stride` bytes.
    /// The result Mat will have dimensions `target_width x target_height`.
    pub fn from_pixels_resize(
        pixels: &[u8],
        pixel_type: PixelType,
        w: i32,
        h: i32,
        stride: i32,
        target_width: i32,
        target_height: i32,
        allocator: std::option::Option<&super::Allocator>,
    ) -> Self {
        let alloc = allocator.map_or(ptr::null_mut(), |a| a.as_ptr());
        let raw = unsafe {
            ncnn_sys::ncnn_mat_from_pixels_resize(
                pixels.as_ptr(),
                pixel_type as c_int,
                w,
                h,
                stride,
                target_width,
                target_height,
                alloc,
            )
        };
        Self { raw }
    }

    /// Copy this Mat with border padding.
    ///
    /// `border_type` controls padding mode, `v` is the fill value for `BorderType::Constant`.
    pub fn copy_make_border(
        &self,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
        border_type: BorderType,
        v: f32,
        opt: &super::NcnnOption,
    ) -> Self {
        let dst = Self::new();
        unsafe {
            ncnn_sys::ncnn_copy_make_border(
                self.raw,
                dst.raw,
                top,
                bottom,
                left,
                right,
                border_type as c_int,
                v,
                opt.as_ptr(),
            );
        }
        dst
    }

    /// Copy this Mat with 3D border padding.
    ///
    /// Adds padding in all 3 dimensions (top/bottom, left/right, front/behind).
    pub fn copy_make_border_3d(
        &self,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
        front: i32,
        behind: i32,
        border_type: BorderType,
        v: f32,
        opt: &super::NcnnOption,
    ) -> Self {
        let dst = Self::new();
        unsafe {
            ncnn_sys::ncnn_copy_make_border_3d(
                self.raw,
                dst.raw,
                top,
                bottom,
                left,
                right,
                front,
                behind,
                border_type as c_int,
                v,
                opt.as_ptr(),
            );
        }
        dst
    }
}

impl Default for Mat {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ncnn_sys::ncnn_mat_destroy(self.raw) }
        }
    }
}
