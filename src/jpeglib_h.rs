pub const JPEG_COM: libc::c_int = 0xfe as libc::c_int;
pub type J_BOOLEAN_PARAM = libc::c_uint;
pub type J_FLOAT_PARAM = libc::c_uint;
pub type J_INT_PARAM = libc::c_uint;
pub const JBOOLEAN_OVERSHOOT_DERINGING: crate::jpeglib_h::J_BOOLEAN_PARAM = 1061927929;
pub const JBOOLEAN_TRELLIS_Q_OPT: crate::jpeglib_h::J_BOOLEAN_PARAM = 3777684073;
pub const JBOOLEAN_USE_SCANS_IN_TRELLIS: crate::jpeglib_h::J_BOOLEAN_PARAM = 4253291573;
pub const JBOOLEAN_USE_LAMBDA_WEIGHT_TBL: crate::jpeglib_h::J_BOOLEAN_PARAM = 865973855;
pub const JBOOLEAN_TRELLIS_EOB_OPT: crate::jpeglib_h::J_BOOLEAN_PARAM = 3623303040;
pub const JBOOLEAN_TRELLIS_QUANT_DC: crate::jpeglib_h::J_BOOLEAN_PARAM = 865946636;
pub const JBOOLEAN_TRELLIS_QUANT: crate::jpeglib_h::J_BOOLEAN_PARAM = 3306299443;
pub const JBOOLEAN_OPTIMIZE_SCANS: crate::jpeglib_h::J_BOOLEAN_PARAM = 1745618462;
pub const JFLOAT_TRELLIS_DELTA_DC_WEIGHT: crate::jpeglib_h::J_FLOAT_PARAM = 326587475;
pub const JFLOAT_LAMBDA_LOG_SCALE2: crate::jpeglib_h::J_FLOAT_PARAM = 3116084739;
pub const JFLOAT_LAMBDA_LOG_SCALE1: crate::jpeglib_h::J_FLOAT_PARAM = 1533126041;
pub const JINT_DC_SCAN_OPT_MODE: crate::jpeglib_h::J_INT_PARAM = 199732540;
pub const JINT_BASE_QUANT_TBL_IDX: crate::jpeglib_h::J_INT_PARAM = 1145645745;
pub const JINT_TRELLIS_NUM_LOOPS: crate::jpeglib_h::J_INT_PARAM = 3057565497;
pub const JINT_TRELLIS_FREQ_SPLIT: crate::jpeglib_h::J_INT_PARAM = 1873801511;
pub const JINT_COMPRESS_PROFILE: crate::jpeglib_h::J_INT_PARAM = 3918628389;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_struct {
    pub next: crate::jpeglib_h::jpeg_saved_marker_ptr,
    pub marker: crate::jmorecfg_h::UINT8,
    pub original_length: libc::c_uint,
    pub data_length: libc::c_uint,
    pub data: *mut crate::jmorecfg_h::JOCTET,
}
/* The decompressor can save APPn and COM markers in a list of these: */
pub type jpeg_saved_marker_ptr = *mut crate::jpeglib_h::jpeg_marker_struct;
pub type J_DITHER_MODE = libc::c_uint;
/* Master record for a decompression instance */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decompress_struct {
    pub err: *mut crate::jpeglib_h::jpeg_error_mgr,
    pub mem: *mut crate::jpeglib_h::jpeg_memory_mgr,
    pub progress: *mut crate::jpeglib_h::jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: crate::jmorecfg_h::boolean,
    pub global_state: libc::c_int,
    pub src: *mut crate::jpeglib_h::jpeg_source_mgr,
    pub image_width: crate::jmorecfg_h::JDIMENSION,
    pub image_height: crate::jmorecfg_h::JDIMENSION,
    pub num_components: libc::c_int,
    pub jpeg_color_space: crate::jpeglib_h::J_COLOR_SPACE,
    pub out_color_space: crate::jpeglib_h::J_COLOR_SPACE,
    pub scale_num: libc::c_uint,
    pub scale_denom: libc::c_uint,
    pub output_gamma: libc::c_double,
    pub buffered_image: crate::jmorecfg_h::boolean,
    pub raw_data_out: crate::jmorecfg_h::boolean,
    pub dct_method: crate::jpeglib_h::J_DCT_METHOD,
    pub do_fancy_upsampling: crate::jmorecfg_h::boolean,
    pub do_block_smoothing: crate::jmorecfg_h::boolean,
    pub quantize_colors: crate::jmorecfg_h::boolean,
    pub dither_mode: crate::jpeglib_h::J_DITHER_MODE,
    pub two_pass_quantize: crate::jmorecfg_h::boolean,
    pub desired_number_of_colors: libc::c_int,
    pub enable_1pass_quant: crate::jmorecfg_h::boolean,
    pub enable_external_quant: crate::jmorecfg_h::boolean,
    pub enable_2pass_quant: crate::jmorecfg_h::boolean,
    pub output_width: crate::jmorecfg_h::JDIMENSION,
    pub output_height: crate::jmorecfg_h::JDIMENSION,
    pub out_color_components: libc::c_int,
    pub output_components: libc::c_int,
    pub rec_outbuf_height: libc::c_int,
    pub actual_number_of_colors: libc::c_int,
    pub colormap: crate::jpeglib_h::JSAMPARRAY,
    pub output_scanline: crate::jmorecfg_h::JDIMENSION,
    pub input_scan_number: libc::c_int,
    pub input_iMCU_row: crate::jmorecfg_h::JDIMENSION,
    pub output_scan_number: libc::c_int,
    pub output_iMCU_row: crate::jmorecfg_h::JDIMENSION,
    pub coef_bits: *mut [libc::c_int; 64],
    pub quant_tbl_ptrs: [*mut crate::jpeglib_h::JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
    pub data_precision: libc::c_int,
    pub comp_info: *mut crate::jpeglib_h::jpeg_component_info,
    pub progressive_mode: crate::jmorecfg_h::boolean,
    pub arith_code: crate::jmorecfg_h::boolean,
    pub arith_dc_L: [crate::jmorecfg_h::UINT8; 16],
    pub arith_dc_U: [crate::jmorecfg_h::UINT8; 16],
    pub arith_ac_K: [crate::jmorecfg_h::UINT8; 16],
    pub restart_interval: libc::c_uint,
    pub saw_JFIF_marker: crate::jmorecfg_h::boolean,
    pub JFIF_major_version: crate::jmorecfg_h::UINT8,
    pub JFIF_minor_version: crate::jmorecfg_h::UINT8,
    pub density_unit: crate::jmorecfg_h::UINT8,
    pub X_density: crate::jmorecfg_h::UINT16,
    pub Y_density: crate::jmorecfg_h::UINT16,
    pub saw_Adobe_marker: crate::jmorecfg_h::boolean,
    pub Adobe_transform: crate::jmorecfg_h::UINT8,
    pub CCIR601_sampling: crate::jmorecfg_h::boolean,
    pub marker_list: crate::jpeglib_h::jpeg_saved_marker_ptr,
    pub max_h_samp_factor: libc::c_int,
    pub max_v_samp_factor: libc::c_int,
    pub min_DCT_scaled_size: libc::c_int,
    pub total_iMCU_rows: crate::jmorecfg_h::JDIMENSION,
    pub sample_range_limit: *mut crate::jmorecfg_h::JSAMPLE,
    pub comps_in_scan: libc::c_int,
    pub cur_comp_info: [*mut crate::jpeglib_h::jpeg_component_info; 4],
    pub MCUs_per_row: crate::jmorecfg_h::JDIMENSION,
    pub MCU_rows_in_scan: crate::jmorecfg_h::JDIMENSION,
    pub blocks_in_MCU: libc::c_int,
    pub MCU_membership: [libc::c_int; 10],
    pub Ss: libc::c_int,
    pub Se: libc::c_int,
    pub Ah: libc::c_int,
    pub Al: libc::c_int,
    pub unread_marker: libc::c_int,
    pub master: *mut crate::jpegint_h::jpeg_decomp_master,
    pub main: *mut crate::jpegint_h::jpeg_d_main_controller,
    pub coef: *mut crate::jpegint_h::jpeg_d_coef_controller,
    pub post: *mut crate::jpegint_h::jpeg_d_post_controller,
    pub inputctl: *mut crate::jpegint_h::jpeg_input_controller,
    pub marker: *mut crate::jpegint_h::jpeg_marker_reader,
    pub entropy: *mut crate::jpegint_h::jpeg_entropy_decoder,
    pub idct: *mut crate::jpegint_h::jpeg_inverse_dct,
    pub upsample: *mut crate::jpegint_h::jpeg_upsampler,
    pub cconvert: *mut crate::jpegint_h::jpeg_color_deconverter,
    pub cquantize: *mut crate::jpegint_h::jpeg_color_quantizer,
}
pub type j_decompress_ptr = *mut crate::jpeglib_h::jpeg_decompress_struct;
/* Routine signature for application-supplied marker processing methods.
 * Need not pass marker code since it is stored in cinfo->unread_marker.
 */
pub type jpeg_marker_parser_method = Option<
    unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> crate::jmorecfg_h::boolean,
>;
/* Data source object for decompression */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const crate::jmorecfg_h::JOCTET,
    pub bytes_in_buffer: crate::stddef_h::size_t,
    pub init_source: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub fill_input_buffer: Option<
        unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> crate::jmorecfg_h::boolean,
    >,
    pub skip_input_data:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr, _: libc::c_long) -> ()>,
    pub resync_to_restart: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: libc::c_int,
        ) -> crate::jmorecfg_h::boolean,
    >,
    pub term_source: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
}
pub const JDITHER_FS: crate::jpeglib_h::J_DITHER_MODE = 2;
pub const JDITHER_ORDERED: crate::jpeglib_h::J_DITHER_MODE = 1;
pub const JDITHER_NONE: crate::jpeglib_h::J_DITHER_MODE = 0;
/* lasts until master record is destroyed */

/* lasts until done with image/datastream */
pub const JPOOL_NUMPOOLS: libc::c_int = 2 as libc::c_int;
/* Read ICC profile.  See libjpeg.txt for usage information. */

/*
 * Permit users to replace the IDCT method dynamically.
 * The selector callback is called after the default idct implementation was choosen,
 * and is able to override it.
 */

/* These marker codes are exported since applications and data source modules
 * are likely to want to use them.
 */

/* RST0 marker code */

/* EOI marker code */
pub const JPEG_APP0: libc::c_int = 0xe0 as libc::c_int;
/* The basic DCT block is 8x8 samples */

/* DCTSIZE squared; # of elements in a block */

/* Quantization tables are numbered 0..3 */

/* Huffman tables are numbered 0..3 */

/* Arith-coding tables are numbered 0..15 */

/* JPEG limit on # of components in one scan */

/* JPEG limit on sampling factors */

/* Unfortunately, some bozo at Adobe saw no reason to be bound by the standard;
 * the PostScript DCT filter can emit files with many more than 10 blocks/MCU.
 * If you happen to run across such a file, you can up D_MAX_BLOCKS_IN_MCU
 * to handle it.  We even let you do this from the jconfig.h file.  However,
 * we strongly discourage changing C_MAX_BLOCKS_IN_MCU; just because Adobe
 * sometimes emits noncompliant files doesn't mean you should too.
 */
pub const C_MAX_BLOCKS_IN_MCU: libc::c_int = 10 as libc::c_int;
/* Memory manager object.
 * Allocates "small" objects (a few K total), "large" objects (tens of K),
 * and "really big" objects (virtual arrays with backing store if needed).
 * The memory manager does not allow individual objects to be freed; rather,
 * each created object is assigned to a pool, and whole pools can be freed
 * at once.  This is faster and more convenient than remembering exactly what
 * to free, especially where malloc()/free() are not too speedy.
 * NB: alloc routines never return NULL.  They exit to error_exit if not
 * successful.
 */

/* lasts until master record is destroyed */
pub const JPOOL_IMAGE: libc::c_int = 1 as libc::c_int;
/* Quantization tables are numbered 0..3 */

/* Huffman tables are numbered 0..3 */
pub const NUM_ARITH_TBLS: libc::c_int = 16 as libc::c_int;
/* may be overridden in jconfig.h */
pub const JDCT_DEFAULT: libc::c_int = crate::jpeglib_h::JDCT_ISLOW as libc::c_int;
/*
 * jpeglib.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modified 2002-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2011, 2013-2014, 2016-2017, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file defines the application interface for the JPEG library.
 * Most applications using the library need only include this file,
 * and perhaps jerror.h if they want to know the exact error codes.
 */

/*
 * First we include the configuration files that record how this
 * installation of the JPEG library is set up.  jconfig.h can be
 * generated automatically for many systems.  jmorecfg.h contains
 * manual configuration options that most people need not worry about.
 */

/* in case jinclude.h already did */

/* Various constants determining the sizes of things.
 * All of these are specified by the JPEG standard, so don't change them
 * if you want to be compatible.
 */
pub const DCTSIZE: libc::c_int = 8 as libc::c_int;
/* Quantization tables are numbered 0..3 */

/* Huffman tables are numbered 0..3 */

/* Arith-coding tables are numbered 0..15 */
pub const MAX_COMPS_IN_SCAN: libc::c_int = 4 as libc::c_int;
/* JPEG limit on # of components in one scan */
pub const MAX_SAMP_FACTOR: libc::c_int = 4 as libc::c_int;
/* a 3-D array of coefficient blocks */
pub type JCOEFPTR = *mut crate::jmorecfg_h::JCOEF;
/* The basic DCT block is 8x8 samples */
pub const DCTSIZE2: libc::c_int = 64 as libc::c_int;
/* Return value is one of: */

/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */
pub const JPEG_REACHED_SOS: libc::c_int = 1 as libc::c_int;
/* Reached start of new scan */

/* Reached end of image */
pub const JPEG_ROW_COMPLETED: libc::c_int = 3 as libc::c_int;
pub const JPEG_REACHED_EOI: libc::c_int = 2 as libc::c_int;
pub const JPEG_SUSPENDED: libc::c_int = 0 as libc::c_int;
/* These marker codes are exported since applications and data source modules
 * are likely to want to use them.
 */

/* RST0 marker code */
pub const JPEG_EOI: libc::c_int = 0xd9 as libc::c_int;
pub const D_MAX_BLOCKS_IN_MCU: libc::c_int = 10 as libc::c_int;
pub const JPEG_SCAN_COMPLETED: libc::c_int = 4 as libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub i: [libc::c_int; 8],
    pub s: [libc::c_char; 80],
}
pub type JSAMPROW = *mut crate::jmorecfg_h::JSAMPLE;
pub type JSAMPARRAY = *mut crate::jpeglib_h::JSAMPROW;
pub type JSAMPIMAGE = *mut crate::jpeglib_h::JSAMPARRAY;
pub type JBLOCK = [crate::jmorecfg_h::JCOEF; 64];
pub type JBLOCKROW = *mut crate::jpeglib_h::JBLOCK;
pub type JBLOCKARRAY = *mut crate::jpeglib_h::JBLOCKROW;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JQUANT_TBL {
    pub quantval: [crate::jmorecfg_h::UINT16; 64],
    pub sent_table: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JHUFF_TBL {
    pub bits: [crate::jmorecfg_h::UINT8; 17],
    pub huffval: [crate::jmorecfg_h::UINT8; 256],
    pub sent_table: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_component_info {
    pub component_id: libc::c_int,
    pub component_index: libc::c_int,
    pub h_samp_factor: libc::c_int,
    pub v_samp_factor: libc::c_int,
    pub quant_tbl_no: libc::c_int,
    pub dc_tbl_no: libc::c_int,
    pub ac_tbl_no: libc::c_int,
    pub width_in_blocks: crate::jmorecfg_h::JDIMENSION,
    pub height_in_blocks: crate::jmorecfg_h::JDIMENSION,
    pub DCT_scaled_size: libc::c_int,
    pub downsampled_width: crate::jmorecfg_h::JDIMENSION,
    pub downsampled_height: crate::jmorecfg_h::JDIMENSION,
    pub component_needed: crate::jmorecfg_h::boolean,
    pub MCU_width: libc::c_int,
    pub MCU_height: libc::c_int,
    pub MCU_blocks: libc::c_int,
    pub MCU_sample_width: libc::c_int,
    pub last_col_width: libc::c_int,
    pub last_row_height: libc::c_int,
    pub quant_table: *mut crate::jpeglib_h::JQUANT_TBL,
    pub dct_table: *mut libc::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_scan_info {
    pub comps_in_scan: libc::c_int,
    pub component_index: [libc::c_int; 4],
    pub Ss: libc::c_int,
    pub Se: libc::c_int,
    pub Ah: libc::c_int,
    pub Al: libc::c_int,
}
pub type J_COLOR_SPACE = libc::c_uint;
pub type J_DCT_METHOD = libc::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_common_struct {
    pub err: *mut crate::jpeglib_h::jpeg_error_mgr,
    pub mem: *mut crate::jpeglib_h::jpeg_memory_mgr,
    pub progress: *mut crate::jpeglib_h::jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: crate::jmorecfg_h::boolean,
    pub global_state: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
    pub pass_counter: libc::c_long,
    pub pass_limit: libc::c_long,
    pub completed_passes: libc::c_int,
    pub total_passes: libc::c_int,
}
pub type j_common_ptr = *mut crate::jpeglib_h::jpeg_common_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: libc::c_int,
            _: crate::stddef_h::size_t,
        ) -> *mut libc::c_void,
    >,
    pub alloc_large: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: libc::c_int,
            _: crate::stddef_h::size_t,
        ) -> *mut libc::c_void,
    >,
    pub alloc_sarray: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: libc::c_int,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> crate::jpeglib_h::JSAMPARRAY,
    >,
    pub alloc_barray: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: libc::c_int,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> crate::jpeglib_h::JBLOCKARRAY,
    >,
    pub request_virt_sarray: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: libc::c_int,
            _: crate::jmorecfg_h::boolean,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> crate::jpeglib_h::jvirt_sarray_ptr,
    >,
    pub request_virt_barray: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: libc::c_int,
            _: crate::jmorecfg_h::boolean,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> crate::jpeglib_h::jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
    pub access_virt_sarray: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jpeglib_h::jvirt_sarray_ptr,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::boolean,
        ) -> crate::jpeglib_h::JSAMPARRAY,
    >,
    pub access_virt_barray: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jpeglib_h::jvirt_barray_ptr,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::boolean,
        ) -> crate::jpeglib_h::JBLOCKARRAY,
    >,
    pub free_pool:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> ()>,
    pub self_destruct: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
    pub max_memory_to_use: libc::c_long,
    pub max_alloc_chunk: libc::c_long,
}
pub type jvirt_barray_ptr = *mut crate::jpeglib_h::jvirt_barray_control;
pub type jvirt_sarray_ptr = *mut crate::jpeglib_h::jvirt_sarray_control;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
    pub emit_message:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> ()>,
    pub output_message: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
    pub format_message:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: *mut libc::c_char) -> ()>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
    pub msg_code: libc::c_int,
    pub msg_parm: crate::jpeglib_h::C2RustUnnamed_2,
    pub trace_level: libc::c_int,
    pub num_warnings: libc::c_long,
    pub jpeg_message_table: *const *const libc::c_char,
    pub last_jpeg_message: libc::c_int,
    pub addon_message_table: *const *const libc::c_char,
    pub first_addon_message: libc::c_int,
    pub last_addon_message: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_compress_struct {
    pub err: *mut crate::jpeglib_h::jpeg_error_mgr,
    pub mem: *mut crate::jpeglib_h::jpeg_memory_mgr,
    pub progress: *mut crate::jpeglib_h::jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: crate::jmorecfg_h::boolean,
    pub global_state: libc::c_int,
    pub dest: *mut crate::jpeglib_h::jpeg_destination_mgr,
    pub image_width: crate::jmorecfg_h::JDIMENSION,
    pub image_height: crate::jmorecfg_h::JDIMENSION,
    pub input_components: libc::c_int,
    pub in_color_space: crate::jpeglib_h::J_COLOR_SPACE,
    pub input_gamma: libc::c_double,
    pub data_precision: libc::c_int,
    pub num_components: libc::c_int,
    pub jpeg_color_space: crate::jpeglib_h::J_COLOR_SPACE,
    pub comp_info: *mut crate::jpeglib_h::jpeg_component_info,
    pub quant_tbl_ptrs: [*mut crate::jpeglib_h::JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
    pub arith_dc_L: [crate::jmorecfg_h::UINT8; 16],
    pub arith_dc_U: [crate::jmorecfg_h::UINT8; 16],
    pub arith_ac_K: [crate::jmorecfg_h::UINT8; 16],
    pub num_scans: libc::c_int,
    pub scan_info: *const crate::jpeglib_h::jpeg_scan_info,
    pub raw_data_in: crate::jmorecfg_h::boolean,
    pub arith_code: crate::jmorecfg_h::boolean,
    pub optimize_coding: crate::jmorecfg_h::boolean,
    pub CCIR601_sampling: crate::jmorecfg_h::boolean,
    pub smoothing_factor: libc::c_int,
    pub dct_method: crate::jpeglib_h::J_DCT_METHOD,
    pub restart_interval: libc::c_uint,
    pub restart_in_rows: libc::c_int,
    pub write_JFIF_header: crate::jmorecfg_h::boolean,
    pub JFIF_major_version: crate::jmorecfg_h::UINT8,
    pub JFIF_minor_version: crate::jmorecfg_h::UINT8,
    pub density_unit: crate::jmorecfg_h::UINT8,
    pub X_density: crate::jmorecfg_h::UINT16,
    pub Y_density: crate::jmorecfg_h::UINT16,
    pub write_Adobe_marker: crate::jmorecfg_h::boolean,
    pub next_scanline: crate::jmorecfg_h::JDIMENSION,
    pub progressive_mode: crate::jmorecfg_h::boolean,
    pub max_h_samp_factor: libc::c_int,
    pub max_v_samp_factor: libc::c_int,
    pub total_iMCU_rows: crate::jmorecfg_h::JDIMENSION,
    pub comps_in_scan: libc::c_int,
    pub cur_comp_info: [*mut crate::jpeglib_h::jpeg_component_info; 4],
    pub MCUs_per_row: crate::jmorecfg_h::JDIMENSION,
    pub MCU_rows_in_scan: crate::jmorecfg_h::JDIMENSION,
    pub blocks_in_MCU: libc::c_int,
    pub MCU_membership: [libc::c_int; 10],
    pub Ss: libc::c_int,
    pub Se: libc::c_int,
    pub Ah: libc::c_int,
    pub Al: libc::c_int,
    pub master: *mut crate::jpegint_h::jpeg_comp_master,
    pub main: *mut crate::jpegint_h::jpeg_c_main_controller,
    pub prep: *mut crate::jpegint_h::jpeg_c_prep_controller,
    pub coef: *mut crate::jpegint_h::jpeg_c_coef_controller,
    pub marker: *mut crate::jpegint_h::jpeg_marker_writer,
    pub cconvert: *mut crate::jpegint_h::jpeg_color_converter,
    pub downsample: *mut crate::jpegint_h::jpeg_downsampler,
    pub fdct: *mut crate::jpegint_h::jpeg_forward_dct,
    pub entropy: *mut crate::jpegint_h::jpeg_entropy_encoder,
    pub script_space: *mut crate::jpeglib_h::jpeg_scan_info,
    pub script_space_size: libc::c_int,
}
pub type j_compress_ptr = *mut crate::jpeglib_h::jpeg_compress_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut crate::jmorecfg_h::JOCTET,
    pub free_in_buffer: crate::stddef_h::size_t,
    pub init_destination: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub empty_output_buffer: Option<
        unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> crate::jmorecfg_h::boolean,
    >,
    pub term_destination: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
}
pub type jpeg_idct_method = Option<
    unsafe extern "C" fn(
        _: crate::jpeglib_h::j_decompress_ptr,
        _: *mut crate::jpeglib_h::jpeg_component_info,
        _: crate::jpeglib_h::JCOEFPTR,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
pub type jpeg_idct_method_selector = Option<
    unsafe extern "C" fn(
        _: crate::jpeglib_h::j_decompress_ptr,
        _: *mut crate::jpeglib_h::jpeg_component_info,
        _: *mut crate::jpeglib_h::jpeg_idct_method,
        _: *mut libc::c_int,
    ) -> (),
>;
pub const JCS_RGB565: crate::jpeglib_h::J_COLOR_SPACE = 16;
pub const JCS_EXT_ARGB: crate::jpeglib_h::J_COLOR_SPACE = 15;
pub const JCS_EXT_ABGR: crate::jpeglib_h::J_COLOR_SPACE = 14;
pub const JCS_EXT_BGRA: crate::jpeglib_h::J_COLOR_SPACE = 13;
pub const JCS_EXT_RGBA: crate::jpeglib_h::J_COLOR_SPACE = 12;
pub const JCS_EXT_XRGB: crate::jpeglib_h::J_COLOR_SPACE = 11;
pub const JCS_EXT_XBGR: crate::jpeglib_h::J_COLOR_SPACE = 10;
pub const JCS_EXT_BGRX: crate::jpeglib_h::J_COLOR_SPACE = 9;
pub const JCS_EXT_BGR: crate::jpeglib_h::J_COLOR_SPACE = 8;
pub const JCS_EXT_RGBX: crate::jpeglib_h::J_COLOR_SPACE = 7;
pub const JCS_EXT_RGB: crate::jpeglib_h::J_COLOR_SPACE = 6;
pub const JCS_YCCK: crate::jpeglib_h::J_COLOR_SPACE = 5;
pub const JCS_CMYK: crate::jpeglib_h::J_COLOR_SPACE = 4;
pub const JCS_YCbCr: crate::jpeglib_h::J_COLOR_SPACE = 3;
pub const JCS_RGB: crate::jpeglib_h::J_COLOR_SPACE = 2;
pub const JCS_GRAYSCALE: crate::jpeglib_h::J_COLOR_SPACE = 1;
pub const JCS_UNKNOWN: crate::jpeglib_h::J_COLOR_SPACE = 0;
pub const JDCT_FLOAT: crate::jpeglib_h::J_DCT_METHOD = 2;
pub const JDCT_IFAST: crate::jpeglib_h::J_DCT_METHOD = 1;
pub const JDCT_ISLOW: crate::jpeglib_h::J_DCT_METHOD = 0;
pub const JCP_FASTEST: crate::stdlib::C2RustUnnamed_0 = 720002228;
pub const JCP_MAX_COMPRESSION: crate::stdlib::C2RustUnnamed_0 = 1560820397;
pub const JPOOL_PERMANENT: libc::c_int = 0 as libc::c_int;
pub const NUM_HUFF_TBLS: libc::c_int = 4 as libc::c_int;
pub const NUM_QUANT_TBLS: libc::c_int = 4 as libc::c_int;
/* Method pointers */

/* Limit on memory allocation for this JPEG object.  (Note that this is
 * merely advisory, not a guaranteed maximum; it only affects the space
 * used for virtual-array buffers.)  May be changed by outer application
 * after creating the JPEG object.
 */

/* Maximum allocation request accepted by alloc_large. */

/* Routine signature for application-supplied marker processing methods.
 * Need not pass marker code since it is stored in cinfo->unread_marker.
 */

/* Originally, this macro was used as a way of defining function prototypes
 * for both modern compilers as well as older compilers that did not support
 * prototype parameters.  libjpeg-turbo has never supported these older,
 * non-ANSI compilers, but the macro is still included because there is some
 * software out there that uses it.
 */

/* Default error-management setup */

/* Initialization of JPEG compression objects.
 * jpeg_create_compress() and jpeg_create_decompress() are the exported
 * names that applications should call.  These expand to calls on
 * jpeg_CreateCompress and jpeg_CreateDecompress with additional information
 * passed for version mismatch checking.
 * NB: you must set up the error-manager BEFORE calling jpeg_create_xxx.
 */

/* Destruction of JPEG compression objects */

/* Standard data source and destination managers: stdio streams. */

/* Caller is responsible for opening the file before and closing after. */

/* Data source and destination managers: memory buffers. */

/* Default parameter setup for compression */

/* Compression parameter setup aids */

/* Main entry points for compression */

/* Replaces jpeg_write_scanlines when writing raw downsampled data. */

/* Write a special marker.  See libjpeg.txt concerning safe usage. */

/* Same, but piecemeal. */

/* Alternate compression function: just write an abbreviated table file */

/* Write ICC profile.  See libjpeg.txt for usage information. */

/* Decompression startup: read start of JPEG datastream to see what's there */

/* Return value is one of: */

/* Suspended due to lack of input data */

/* Found valid image datastream */
pub const JPEG_HEADER_TABLES_ONLY: libc::c_int = 2 as libc::c_int;
pub const JPEG_HEADER_OK: libc::c_int = 1 as libc::c_int;
pub use crate::src::jmemmgr::jvirt_barray_control;
pub use crate::src::jmemmgr::jvirt_sarray_control;
pub const JDCT_FASTEST: libc::c_int = crate::jpeglib_h::JDCT_IFAST as libc::c_int;
pub const JMSG_LENGTH_MAX: libc::c_int = 200 as libc::c_int;
