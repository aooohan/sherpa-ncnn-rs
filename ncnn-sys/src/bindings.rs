// Pre-generated FFI bindings for ncnn C API
// Matching ncnn commit c4193aadbbb56582aa87b1850dd3d98fb8fd936d
// (bundled with sherpa-ncnn 2.1.15)
//
// These are used when the C headers are not available (e.g. cross-compilation).

use std::os::raw::{c_char, c_float, c_int, c_uchar, c_void};

// ============================================================
// Allocator API
// ============================================================

pub type ncnn_allocator_t = *mut __ncnn_allocator_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_allocator_t {
    pub pthis: *mut c_void,
    pub fast_malloc: Option<unsafe extern "C" fn(allocator: ncnn_allocator_t, size: usize) -> *mut c_void>,
    pub fast_free: Option<unsafe extern "C" fn(allocator: ncnn_allocator_t, ptr: *mut c_void)>,
}

// ============================================================
// Option API
// ============================================================

pub type ncnn_option_t = *mut __ncnn_option_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_option_t {
    _private: [u8; 0],
}

// ============================================================
// Mat API
// ============================================================

pub type ncnn_mat_t = *mut __ncnn_mat_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_mat_t {
    _private: [u8; 0],
}

// ============================================================
// Blob API
// ============================================================

pub type ncnn_blob_t = *mut __ncnn_blob_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_blob_t {
    _private: [u8; 0],
}

// ============================================================
// ParamDict API
// ============================================================

pub type ncnn_paramdict_t = *mut __ncnn_paramdict_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_paramdict_t {
    _private: [u8; 0],
}

// ============================================================
// DataReader API
// ============================================================

pub type ncnn_datareader_t = *mut __ncnn_datareader_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_datareader_t {
    pub pthis: *mut c_void,
    pub scan: Option<unsafe extern "C" fn(dr: ncnn_datareader_t, format: *const c_char, p: *mut c_void) -> c_int>,
    pub read: Option<unsafe extern "C" fn(dr: ncnn_datareader_t, buf: *mut c_void, size: usize) -> usize>,
}

// ============================================================
// ModelBin API
// ============================================================

pub type ncnn_modelbin_t = *mut __ncnn_modelbin_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_modelbin_t {
    pub pthis: *mut c_void,
    pub load_1d: Option<unsafe extern "C" fn(mb: ncnn_modelbin_t, w: c_int, type_: c_int) -> ncnn_mat_t>,
    pub load_2d: Option<unsafe extern "C" fn(mb: ncnn_modelbin_t, w: c_int, h: c_int, type_: c_int) -> ncnn_mat_t>,
    pub load_3d: Option<unsafe extern "C" fn(mb: ncnn_modelbin_t, w: c_int, h: c_int, c: c_int, type_: c_int) -> ncnn_mat_t>,
}

// ============================================================
// Layer API
// ============================================================

pub type ncnn_layer_t = *mut __ncnn_layer_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_layer_t {
    pub pthis: *mut c_void,

    pub load_param: Option<unsafe extern "C" fn(layer: ncnn_layer_t, pd: ncnn_paramdict_t) -> c_int>,
    pub load_model: Option<unsafe extern "C" fn(layer: ncnn_layer_t, mb: ncnn_modelbin_t) -> c_int>,

    pub create_pipeline: Option<unsafe extern "C" fn(layer: ncnn_layer_t, opt: ncnn_option_t) -> c_int>,
    pub destroy_pipeline: Option<unsafe extern "C" fn(layer: ncnn_layer_t, opt: ncnn_option_t) -> c_int>,

    pub forward_1: Option<unsafe extern "C" fn(layer: ncnn_layer_t, bottom_blob: ncnn_mat_t, top_blob: *mut ncnn_mat_t, opt: ncnn_option_t) -> c_int>,
    pub forward_n: Option<unsafe extern "C" fn(layer: ncnn_layer_t, bottom_blobs: *const ncnn_mat_t, n: c_int, top_blobs: *mut ncnn_mat_t, n2: c_int, opt: ncnn_option_t) -> c_int>,

    pub forward_inplace_1: Option<unsafe extern "C" fn(layer: ncnn_layer_t, bottom_top_blob: ncnn_mat_t, opt: ncnn_option_t) -> c_int>,
    pub forward_inplace_n: Option<unsafe extern "C" fn(layer: ncnn_layer_t, bottom_top_blobs: *mut ncnn_mat_t, n: c_int, opt: ncnn_option_t) -> c_int>,
}

// ============================================================
// Layer Factory
// ============================================================

pub type ncnn_layer_creator_t = Option<unsafe extern "C" fn(userdata: *mut c_void) -> ncnn_layer_t>;
pub type ncnn_layer_destroyer_t = Option<unsafe extern "C" fn(layer: ncnn_layer_t, userdata: *mut c_void)>;

pub type ncnn_net_custom_layer_factory_t = *mut __ncnn_net_custom_layer_factory_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_net_custom_layer_factory_t {
    pub creator: ncnn_layer_creator_t,
    pub destroyer: ncnn_layer_destroyer_t,
    pub userdata: *mut c_void,
    pub next: ncnn_net_custom_layer_factory_t,
}

// ============================================================
// Net API
// ============================================================

pub type ncnn_net_t = *mut __ncnn_net_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_net_t {
    pub pthis: *mut c_void,
    pub custom_layer_factory: ncnn_net_custom_layer_factory_t,
}

// ============================================================
// Extractor API
// ============================================================

pub type ncnn_extractor_t = *mut __ncnn_extractor_t;

#[repr(C)]
#[derive(Debug)]
pub struct __ncnn_extractor_t {
    _private: [u8; 0],
}

// ============================================================
// Pixel format constants
// ============================================================

pub const NCNN_MAT_PIXEL_RGB: c_int = 1;
pub const NCNN_MAT_PIXEL_BGR: c_int = 2;
pub const NCNN_MAT_PIXEL_GRAY: c_int = 3;
pub const NCNN_MAT_PIXEL_RGBA: c_int = 4;
pub const NCNN_MAT_PIXEL_BGRA: c_int = 5;

/// Helper to create pixel conversion type: `NCNN_MAT_PIXEL_X2Y(from, to)`
#[inline]
pub const fn ncnn_mat_pixel_x2y(x: c_int, y: c_int) -> c_int {
    x | (y << 16)
}

// ============================================================
// Border constants
// ============================================================

pub const NCNN_BORDER_CONSTANT: c_int = 0;
pub const NCNN_BORDER_REPLICATE: c_int = 1;
pub const NCNN_BORDER_REFLECT: c_int = 2;
pub const NCNN_BORDER_TRANSPARENT: c_int = -233;

// ============================================================
// Extern Functions
// ============================================================

extern "C" {
    // --- version ---
    pub fn ncnn_version() -> *const c_char;

    // --- allocator ---
    pub fn ncnn_allocator_create_pool_allocator() -> ncnn_allocator_t;
    pub fn ncnn_allocator_create_unlocked_pool_allocator() -> ncnn_allocator_t;
    pub fn ncnn_allocator_destroy(allocator: ncnn_allocator_t);

    // --- option ---
    pub fn ncnn_option_create() -> ncnn_option_t;
    pub fn ncnn_option_destroy(opt: ncnn_option_t);

    pub fn ncnn_option_get_num_threads(opt: ncnn_option_t) -> c_int;
    pub fn ncnn_option_set_num_threads(opt: ncnn_option_t, num_threads: c_int);

    pub fn ncnn_option_get_use_local_pool_allocator(opt: ncnn_option_t) -> c_int;
    pub fn ncnn_option_set_use_local_pool_allocator(opt: ncnn_option_t, use_local_pool_allocator: c_int);

    pub fn ncnn_option_set_blob_allocator(opt: ncnn_option_t, allocator: ncnn_allocator_t);
    pub fn ncnn_option_set_workspace_allocator(opt: ncnn_option_t, allocator: ncnn_allocator_t);

    pub fn ncnn_option_get_use_vulkan_compute(opt: ncnn_option_t) -> c_int;
    pub fn ncnn_option_set_use_vulkan_compute(opt: ncnn_option_t, use_vulkan_compute: c_int);

    // --- mat creation ---
    pub fn ncnn_mat_create() -> ncnn_mat_t;
    pub fn ncnn_mat_create_1d(w: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_2d(w: c_int, h: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_3d(w: c_int, h: c_int, c: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_4d(w: c_int, h: c_int, d: c_int, c: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;

    pub fn ncnn_mat_create_external_1d(w: c_int, data: *mut c_void, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_external_2d(w: c_int, h: c_int, data: *mut c_void, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_external_3d(w: c_int, h: c_int, c: c_int, data: *mut c_void, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_external_4d(w: c_int, h: c_int, d: c_int, c: c_int, data: *mut c_void, allocator: ncnn_allocator_t) -> ncnn_mat_t;

    pub fn ncnn_mat_create_1d_elem(w: c_int, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_2d_elem(w: c_int, h: c_int, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_3d_elem(w: c_int, h: c_int, c: c_int, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_4d_elem(w: c_int, h: c_int, d: c_int, c: c_int, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;

    pub fn ncnn_mat_create_external_1d_elem(w: c_int, data: *mut c_void, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_external_2d_elem(w: c_int, h: c_int, data: *mut c_void, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_external_3d_elem(w: c_int, h: c_int, c: c_int, data: *mut c_void, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_create_external_4d_elem(w: c_int, h: c_int, d: c_int, c: c_int, data: *mut c_void, elemsize: usize, elempack: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;

    pub fn ncnn_mat_destroy(mat: ncnn_mat_t);

    // --- mat operations ---
    pub fn ncnn_mat_fill_float(mat: ncnn_mat_t, v: c_float);
    pub fn ncnn_mat_clone(mat: ncnn_mat_t, allocator: ncnn_allocator_t) -> ncnn_mat_t;

    pub fn ncnn_mat_reshape_1d(mat: ncnn_mat_t, w: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_reshape_2d(mat: ncnn_mat_t, w: c_int, h: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_reshape_3d(mat: ncnn_mat_t, w: c_int, h: c_int, c: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;
    pub fn ncnn_mat_reshape_4d(mat: ncnn_mat_t, w: c_int, h: c_int, d: c_int, c: c_int, allocator: ncnn_allocator_t) -> ncnn_mat_t;

    // --- mat getters ---
    pub fn ncnn_mat_get_dims(mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_mat_get_w(mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_mat_get_h(mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_mat_get_d(mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_mat_get_c(mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_mat_get_elemsize(mat: ncnn_mat_t) -> usize;
    pub fn ncnn_mat_get_elempack(mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_mat_get_cstep(mat: ncnn_mat_t) -> usize;
    pub fn ncnn_mat_get_data(mat: ncnn_mat_t) -> *mut c_void;
    pub fn ncnn_mat_get_channel_data(mat: ncnn_mat_t, c: c_int) -> *mut c_void;

    // --- mat pixel (NCNN_PIXEL) ---
    pub fn ncnn_mat_from_pixels(
        pixels: *const c_uchar, type_: c_int,
        w: c_int, h: c_int, stride: c_int,
        allocator: ncnn_allocator_t,
    ) -> ncnn_mat_t;
    pub fn ncnn_mat_from_pixels_resize(
        pixels: *const c_uchar, type_: c_int,
        w: c_int, h: c_int, stride: c_int,
        target_width: c_int, target_height: c_int,
        allocator: ncnn_allocator_t,
    ) -> ncnn_mat_t;
    pub fn ncnn_mat_from_pixels_roi(
        pixels: *const c_uchar, type_: c_int,
        w: c_int, h: c_int, stride: c_int,
        roix: c_int, roiy: c_int, roiw: c_int, roih: c_int,
        allocator: ncnn_allocator_t,
    ) -> ncnn_mat_t;
    pub fn ncnn_mat_from_pixels_roi_resize(
        pixels: *const c_uchar, type_: c_int,
        w: c_int, h: c_int, stride: c_int,
        roix: c_int, roiy: c_int, roiw: c_int, roih: c_int,
        target_width: c_int, target_height: c_int,
        allocator: ncnn_allocator_t,
    ) -> ncnn_mat_t;
    pub fn ncnn_mat_to_pixels(
        mat: ncnn_mat_t, pixels: *mut c_uchar,
        type_: c_int, stride: c_int,
    );
    pub fn ncnn_mat_to_pixels_resize(
        mat: ncnn_mat_t, pixels: *mut c_uchar,
        type_: c_int,
        target_width: c_int, target_height: c_int, target_stride: c_int,
    );

    // --- mat processing ---
    pub fn ncnn_mat_substract_mean_normalize(
        mat: ncnn_mat_t, mean_vals: *const c_float, norm_vals: *const c_float,
    );
    pub fn ncnn_convert_packing(
        src: ncnn_mat_t, dst: *mut ncnn_mat_t,
        elempack: c_int, opt: ncnn_option_t,
    );
    pub fn ncnn_flatten(
        src: ncnn_mat_t, dst: *mut ncnn_mat_t, opt: ncnn_option_t,
    );

    // --- blob ---
    pub fn ncnn_blob_get_name(blob: ncnn_blob_t) -> *const c_char;
    pub fn ncnn_blob_get_producer(blob: ncnn_blob_t) -> c_int;
    pub fn ncnn_blob_get_consumer(blob: ncnn_blob_t) -> c_int;
    pub fn ncnn_blob_get_shape(
        blob: ncnn_blob_t,
        dims: *mut c_int, w: *mut c_int, h: *mut c_int, c: *mut c_int,
    );

    // --- paramdict ---
    pub fn ncnn_paramdict_create() -> ncnn_paramdict_t;
    pub fn ncnn_paramdict_destroy(pd: ncnn_paramdict_t);
    pub fn ncnn_paramdict_get_type(pd: ncnn_paramdict_t, id: c_int) -> c_int;
    pub fn ncnn_paramdict_get_int(pd: ncnn_paramdict_t, id: c_int, def: c_int) -> c_int;
    pub fn ncnn_paramdict_get_float(pd: ncnn_paramdict_t, id: c_int, def: c_float) -> c_float;
    pub fn ncnn_paramdict_get_array(pd: ncnn_paramdict_t, id: c_int, def: ncnn_mat_t) -> ncnn_mat_t;
    pub fn ncnn_paramdict_set_int(pd: ncnn_paramdict_t, id: c_int, i: c_int);
    pub fn ncnn_paramdict_set_float(pd: ncnn_paramdict_t, id: c_int, f: c_float);
    pub fn ncnn_paramdict_set_array(pd: ncnn_paramdict_t, id: c_int, v: ncnn_mat_t);

    // --- datareader ---
    pub fn ncnn_datareader_create() -> ncnn_datareader_t;
    pub fn ncnn_datareader_create_from_memory(mem: *const *const c_uchar) -> ncnn_datareader_t;
    pub fn ncnn_datareader_destroy(dr: ncnn_datareader_t);

    // --- modelbin ---
    pub fn ncnn_modelbin_create_from_datareader(dr: ncnn_datareader_t) -> ncnn_modelbin_t;
    pub fn ncnn_modelbin_create_from_mat_array(weights: *const ncnn_mat_t, n: c_int) -> ncnn_modelbin_t;
    pub fn ncnn_modelbin_destroy(mb: ncnn_modelbin_t);

    // --- layer ---
    pub fn ncnn_layer_create() -> ncnn_layer_t;
    pub fn ncnn_layer_create_by_typeindex(typeindex: c_int) -> ncnn_layer_t;
    pub fn ncnn_layer_create_by_type(type_: *const c_char) -> ncnn_layer_t;
    pub fn ncnn_layer_type_to_index(type_: *const c_char) -> c_int;
    pub fn ncnn_layer_destroy(layer: ncnn_layer_t);

    pub fn ncnn_layer_get_name(layer: ncnn_layer_t) -> *const c_char;
    pub fn ncnn_layer_get_typeindex(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_type(layer: ncnn_layer_t) -> *const c_char;

    pub fn ncnn_layer_get_one_blob_only(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_support_inplace(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_support_vulkan(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_support_packing(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_support_bf16_storage(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_support_fp16_storage(layer: ncnn_layer_t) -> c_int;

    pub fn ncnn_layer_set_one_blob_only(layer: ncnn_layer_t, enable: c_int);
    pub fn ncnn_layer_set_support_inplace(layer: ncnn_layer_t, enable: c_int);
    pub fn ncnn_layer_set_support_vulkan(layer: ncnn_layer_t, enable: c_int);
    pub fn ncnn_layer_set_support_packing(layer: ncnn_layer_t, enable: c_int);
    pub fn ncnn_layer_set_support_bf16_storage(layer: ncnn_layer_t, enable: c_int);
    pub fn ncnn_layer_set_support_fp16_storage(layer: ncnn_layer_t, enable: c_int);

    pub fn ncnn_layer_get_bottom_count(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_bottom(layer: ncnn_layer_t, i: c_int) -> c_int;
    pub fn ncnn_layer_get_top_count(layer: ncnn_layer_t) -> c_int;
    pub fn ncnn_layer_get_top(layer: ncnn_layer_t, i: c_int) -> c_int;

    pub fn ncnn_blob_get_bottom_shape(
        layer: ncnn_layer_t, i: c_int,
        dims: *mut c_int, w: *mut c_int, h: *mut c_int, c: *mut c_int,
    );
    pub fn ncnn_blob_get_top_shape(
        layer: ncnn_layer_t, i: c_int,
        dims: *mut c_int, w: *mut c_int, h: *mut c_int, c: *mut c_int,
    );

    // --- net ---
    pub fn ncnn_net_create() -> ncnn_net_t;
    pub fn ncnn_net_destroy(net: ncnn_net_t);

    pub fn ncnn_net_get_option(net: ncnn_net_t) -> ncnn_option_t;
    pub fn ncnn_net_set_option(net: ncnn_net_t, opt: ncnn_option_t);

    pub fn ncnn_net_register_custom_layer_by_type(
        net: ncnn_net_t, type_: *const c_char,
        creator: ncnn_layer_creator_t, destroyer: ncnn_layer_destroyer_t,
        userdata: *mut c_void,
    );
    pub fn ncnn_net_register_custom_layer_by_typeindex(
        net: ncnn_net_t, typeindex: c_int,
        creator: ncnn_layer_creator_t, destroyer: ncnn_layer_destroyer_t,
        userdata: *mut c_void,
    );

    pub fn ncnn_net_load_param(net: ncnn_net_t, path: *const c_char) -> c_int;
    pub fn ncnn_net_load_param_bin(net: ncnn_net_t, path: *const c_char) -> c_int;
    pub fn ncnn_net_load_model(net: ncnn_net_t, path: *const c_char) -> c_int;

    pub fn ncnn_net_load_param_memory(net: ncnn_net_t, mem: *const c_char) -> c_int;
    pub fn ncnn_net_load_param_bin_memory(net: ncnn_net_t, mem: *const c_uchar) -> c_int;
    pub fn ncnn_net_load_model_memory(net: ncnn_net_t, mem: *const c_uchar) -> c_int;

    pub fn ncnn_net_load_param_datareader(net: ncnn_net_t, dr: ncnn_datareader_t) -> c_int;
    pub fn ncnn_net_load_param_bin_datareader(net: ncnn_net_t, dr: ncnn_datareader_t) -> c_int;
    pub fn ncnn_net_load_model_datareader(net: ncnn_net_t, dr: ncnn_datareader_t) -> c_int;

    pub fn ncnn_net_clear(net: ncnn_net_t);

    pub fn ncnn_net_get_input_count(net: ncnn_net_t) -> c_int;
    pub fn ncnn_net_get_output_count(net: ncnn_net_t) -> c_int;
    pub fn ncnn_net_get_input_name(net: ncnn_net_t, i: c_int) -> *const c_char;
    pub fn ncnn_net_get_output_name(net: ncnn_net_t, i: c_int) -> *const c_char;
    pub fn ncnn_net_get_input_index(net: ncnn_net_t, i: c_int) -> c_int;
    pub fn ncnn_net_get_output_index(net: ncnn_net_t, i: c_int) -> c_int;

    // --- extractor ---
    pub fn ncnn_extractor_create(net: ncnn_net_t) -> ncnn_extractor_t;
    pub fn ncnn_extractor_destroy(ex: ncnn_extractor_t);

    pub fn ncnn_extractor_set_option(ex: ncnn_extractor_t, opt: ncnn_option_t);

    pub fn ncnn_extractor_input(ex: ncnn_extractor_t, name: *const c_char, mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_extractor_extract(ex: ncnn_extractor_t, name: *const c_char, mat: *mut ncnn_mat_t) -> c_int;
    pub fn ncnn_extractor_input_index(ex: ncnn_extractor_t, index: c_int, mat: ncnn_mat_t) -> c_int;
    pub fn ncnn_extractor_extract_index(ex: ncnn_extractor_t, index: c_int, mat: *mut ncnn_mat_t) -> c_int;

    // --- mat border operations ---
    pub fn ncnn_copy_make_border(
        src: ncnn_mat_t, dst: ncnn_mat_t,
        top: c_int, bottom: c_int, left: c_int, right: c_int,
        type_: c_int, v: c_float, opt: ncnn_option_t,
    );
    pub fn ncnn_copy_make_border_3d(
        src: ncnn_mat_t, dst: ncnn_mat_t,
        top: c_int, bottom: c_int, left: c_int, right: c_int,
        front: c_int, behind: c_int,
        type_: c_int, v: c_float, opt: ncnn_option_t,
    );
    pub fn ncnn_copy_cut_border(
        src: ncnn_mat_t, dst: ncnn_mat_t,
        top: c_int, bottom: c_int, left: c_int, right: c_int,
        opt: ncnn_option_t,
    );
    pub fn ncnn_copy_cut_border_3d(
        src: ncnn_mat_t, dst: ncnn_mat_t,
        top: c_int, bottom: c_int, left: c_int, right: c_int,
        front: c_int, behind: c_int,
        opt: ncnn_option_t,
    );
}
