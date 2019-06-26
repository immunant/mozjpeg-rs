use libc::c_char;
use libc::c_double;
use libc::c_float;
use libc::c_int;
use libc::c_long;
use libc::c_uchar;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
extern "C" {

    /* Generic versions of jpeg_abort and jpeg_destroy that work on either
     * flavor of JPEG object.  These may be more convenient in some places.
     */
    #[no_mangle]
    pub fn jpeg_abort(cinfo: j_common_ptr);
    #[no_mangle]
    pub fn jpeg_destroy(cinfo: j_common_ptr);
    #[no_mangle]
    pub fn jpeg_suppress_tables(cinfo: j_compress_ptr, suppress: boolean);
    #[no_mangle]
    pub fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *mut JHUFF_TBL;

    /* Same, but piecemeal. */
    #[no_mangle]
    pub fn jpeg_write_m_header(cinfo: j_compress_ptr, marker: c_int, datalen: c_uint);
    #[no_mangle]
    pub fn jpeg_write_m_byte(cinfo: j_compress_ptr, val: c_int);
    #[no_mangle]
    pub fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *mut JQUANT_TBL;

    /* Default parameter setup for compression */
    #[no_mangle]
    pub fn jpeg_set_defaults(cinfo: j_compress_ptr);

    /* Compression parameter setup aids */
    #[no_mangle]
    pub fn jpeg_set_colorspace(cinfo: j_compress_ptr, colorspace: J_COLOR_SPACE);

    /* Default restart-marker-resync procedure for use by data source modules */
    #[no_mangle]
    pub fn jpeg_resync_to_restart(cinfo: j_decompress_ptr, desired: c_int) -> boolean;

    /* Originally, this macro was used as a way of defining function prototypes
     * for both modern compilers as well as older compilers that did not support
     * prototype parameters.  libjpeg-turbo has never supported these older,
     * non-ANSI compilers, but the macro is still included because there is some
     * software out there that uses it.
     */
    /* Default error-management setup */
    #[no_mangle]
    pub fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;

    /* Initialization of JPEG compression objects.
     * jpeg_create_compress() and jpeg_create_decompress() are the exported
     * names that applications should call.  These expand to calls on
     * jpeg_CreateCompress and jpeg_CreateDecompress with additional information
     * passed for version mismatch checking.
     * NB: you must set up the error-manager BEFORE calling jpeg_create_xxx.
     */
    #[no_mangle]
    pub fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: c_int, structsize: size_t);
    #[no_mangle]
    pub fn jpeg_CreateDecompress(cinfo: j_decompress_ptr, version: c_int, structsize: size_t);

    /* Destruction of JPEG compression objects */
    #[no_mangle]
    pub fn jpeg_destroy_compress(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jpeg_set_quality(cinfo: j_compress_ptr, quality: c_int, force_baseline: boolean);
    #[no_mangle]
    pub fn jpeg_simple_progression(cinfo: j_compress_ptr);

    /* Main entry points for compression */
    #[no_mangle]
    pub fn jpeg_start_compress(cinfo: j_compress_ptr, write_all_tables: boolean);
    #[no_mangle]
    pub fn jpeg_write_scanlines(
        cinfo: j_compress_ptr,
        scanlines: JSAMPARRAY,
        num_lines: JDIMENSION,
    ) -> JDIMENSION;
    #[no_mangle]
    pub fn jpeg_finish_compress(cinfo: j_compress_ptr);

    /* Replaces jpeg_write_scanlines when writing raw downsampled data. */
    #[no_mangle]
    pub fn jpeg_write_raw_data(
        cinfo: j_compress_ptr,
        data: JSAMPIMAGE,
        num_lines: JDIMENSION,
    ) -> JDIMENSION;

    /* Decompression startup: read start of JPEG datastream to see what's there */
    #[no_mangle]
    pub fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> c_int;

    /* Return value is one of: */
    /* Suspended due to lack of input data */
    /* Found valid image datastream */
    /* Found valid table-specs-only datastream */
    /* If you pass require_image = TRUE (normal case), you need not check for
     * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
     * JPEG_SUSPENDED is only possible if you use a data source module that can
     * give a suspension return (the stdio source module doesn't).
     */
    /* Main entry points for decompression */
    #[no_mangle]
    pub fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean;
    #[no_mangle]
    pub fn jpeg_read_scanlines(
        cinfo: j_decompress_ptr,
        scanlines: JSAMPARRAY,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    #[no_mangle]
    pub fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;

    /* Replaces jpeg_read_scanlines when reading raw downsampled data. */
    #[no_mangle]
    pub fn jpeg_read_raw_data(
        cinfo: j_decompress_ptr,
        data: JSAMPIMAGE,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;

    /* Reached end of image */
    /* Completed one iMCU row */
    /* Completed last iMCU row of a scan */
    /* Precalculate output dimensions for current decompression parameters. */
    #[no_mangle]
    pub fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr);

    /* Read or write raw DCT coefficients --- useful for lossless transcoding. */
    #[no_mangle]
    pub fn jpeg_read_coefficients(cinfo: j_decompress_ptr) -> *mut jvirt_barray_ptr;
    #[no_mangle]
    pub fn jpeg_write_coefficients(cinfo: j_compress_ptr, coef_arrays: *mut jvirt_barray_ptr);
    #[no_mangle]
    pub fn jpeg_copy_critical_parameters(srcinfo: j_decompress_ptr, dstinfo: j_compress_ptr);

    /* If you choose to abort compression or decompression before completing
     * jpeg_finish_(de)compress, then you need to clean up to release memory,
     * temporary files, etc.  You can just call jpeg_destroy_(de)compress
     * if you're done with the JPEG object, but if you want to clean it up and
     * reuse it, call this:
     */
    #[no_mangle]
    pub fn jpeg_abort_compress(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jpeg_abort_decompress(cinfo: j_decompress_ptr);

    /* Write a special marker.  See libjpeg.txt concerning safe usage. */
    #[no_mangle]
    pub fn jpeg_write_marker(
        cinfo: j_compress_ptr,
        marker: c_int,
        dataptr: *const JOCTET,
        datalen: c_uint,
    );

    /* Control saving of COM and APPn markers into marker_list. */
    #[no_mangle]
    pub fn jpeg_save_markers(cinfo: j_decompress_ptr, marker_code: c_int, length_limit: c_uint);

    /* Standard data source and destination managers: stdio streams. */
    /* Caller is responsible for opening the file before and closing after. */
    #[no_mangle]
    pub fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *mut FILE);

    /* Data source and destination managers: memory buffers. */
    #[no_mangle]
    pub fn jpeg_mem_dest(
        cinfo: j_compress_ptr,
        outbuffer: *mut *mut c_uchar,
        outsize: *mut c_ulong,
    );
    #[no_mangle]
    pub fn jpeg_default_colorspace(cinfo: j_compress_ptr);

    /* Write ICC profile.  See libjpeg.txt for usage information. */
    #[no_mangle]
    pub fn jpeg_write_icc_profile(
        cinfo: j_compress_ptr,
        icc_data_ptr: *const JOCTET,
        icc_data_len: c_uint,
    );
    #[no_mangle]
    pub fn jpeg_c_set_bool_param(cinfo: j_compress_ptr, param: J_BOOLEAN_PARAM, value: boolean);
    #[no_mangle]
    pub fn jpeg_c_set_float_param(cinfo: j_compress_ptr, param: J_FLOAT_PARAM, value: c_float);
    #[no_mangle]
    pub fn jpeg_c_set_int_param(cinfo: j_compress_ptr, param: J_INT_PARAM, value: c_int);
    #[no_mangle]
    pub fn jpeg_c_get_int_param(cinfo: j_compress_ptr, param: J_INT_PARAM) -> c_int;
    #[no_mangle]
    pub fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *mut FILE);
    #[no_mangle]
    pub fn jpeg_add_quant_table(
        cinfo: j_compress_ptr,
        which_tbl: c_int,
        basic_table: *const c_uint,
        scale_factor: c_int,
        force_baseline: boolean,
    );
    #[no_mangle]
    pub fn jpeg_float_quality_scaling(quality: c_float) -> c_float;
    #[no_mangle]
    pub fn jpeg_c_int_param_supported(cinfo: j_compress_ptr, param: J_INT_PARAM) -> boolean;
    #[no_mangle]
    pub fn jpeg_mem_src(cinfo: j_decompress_ptr, inbuffer: *const c_uchar, insize: c_ulong);
    #[no_mangle]
    pub fn jpeg_skip_scanlines(cinfo: j_decompress_ptr, num_lines: JDIMENSION) -> JDIMENSION;
    #[no_mangle]
    pub fn jpeg_crop_scanline(
        cinfo: j_decompress_ptr,
        xoffset: *mut JDIMENSION,
        width: *mut JDIMENSION,
    );

    /* Install a special processing method for COM or APPn markers. */
    #[no_mangle]
    pub fn jpeg_set_marker_processor(
        cinfo: j_decompress_ptr,
        marker_code: c_int,
        routine: jpeg_marker_parser_method,
    );

    /* Read ICC profile.  See libjpeg.txt for usage information. */
    #[no_mangle]
    pub fn jpeg_read_icc_profile(
        cinfo: j_decompress_ptr,
        icc_data_ptr: *mut *mut JOCTET,
        icc_data_len: *mut c_uint,
    ) -> boolean;
}
/* Values for the JINT_COMPRESS_PROFILE parameter (32-bit GUIDs) */
pub type C2RustUnnamed_2 = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_3 {
    pub i: [c_int; 8],
    pub s: [c_char; 80],
}
use ::libc;
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

/* compressor's limit on blocks per MCU */

/* decompressor's limit on blocks per MCU */

/* Data structures for images (arrays of samples and of DCT coefficients).
 */

/* ptr to one image row of pixel samples. */
pub type JSAMPROW = *mut JSAMPLE;
/* ptr to some rows (a 2-D sample array) */
pub type JSAMPARRAY = *mut JSAMPROW;
/* a 3-D sample array: top index is color */
pub type JSAMPIMAGE = *mut JSAMPARRAY;
/* one block of coefficients */
pub type JBLOCK = [JCOEF; 64];
/* pointer to one row of coefficient blocks */
pub type JBLOCKROW = *mut JBLOCK;
/* a 2-D array of coefficient blocks */
pub type JBLOCKARRAY = *mut JBLOCKROW;
/* Types for JPEG compression parameters and working tables. */

/* DCT coefficient quantization tables. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; 64],
    pub sent_table: boolean,
}
/* Huffman coding tables. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17],
    pub huffval: [UINT8; 256],
    pub sent_table: boolean,
}
/* Basic info about one component (color channel). */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_component_info {
    pub component_id: c_int,
    pub component_index: c_int,
    pub h_samp_factor: c_int,
    pub v_samp_factor: c_int,
    pub quant_tbl_no: c_int,
    pub dc_tbl_no: c_int,
    pub ac_tbl_no: c_int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    pub DCT_scaled_size: c_int,
    pub downsampled_width: JDIMENSION,
    pub downsampled_height: JDIMENSION,
    pub component_needed: boolean,
    pub MCU_width: c_int,
    pub MCU_height: c_int,
    pub MCU_blocks: c_int,
    pub MCU_sample_width: c_int,
    pub last_col_width: c_int,
    pub last_row_height: c_int,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut c_void,
}
/* The script for encoding a multiple-scan file is an array of these: */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_scan_info {
    pub comps_in_scan: c_int,
    pub component_index: [c_int; 4],
    pub Ss: c_int,
    pub Se: c_int,
    pub Ah: c_int,
    pub Al: c_int,
}
/* the marker length word is not counted in data_length or original_length */

/* Known color spaces. */
pub type J_COLOR_SPACE = c_uint;
/* DCT/IDCT algorithm options. */
pub type J_DCT_METHOD = c_uint;
/* Common fields between JPEG compression and decompression master structs. */

/* Error handler module */

/* Memory manager module */

/* Progress monitor, or NULL if none */

/* Available for use by application */

/* So common code can tell which is which */

/* For checking call sequence validity */

/* Routines that are to be used by both halves of the library are declared
 * to receive a pointer to this structure.  There are no actual instances of
 * jpeg_common_struct, only of jpeg_compress_struct and jpeg_decompress_struct.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut c_void,
    pub is_decompressor: boolean,
    pub global_state: c_int,
}
/* Progress monitor object */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub pass_counter: c_long,
    pub pass_limit: c_long,
    pub completed_passes: c_int,
    pub total_passes: c_int,
}
/* Additional fields follow in an actual jpeg_compress_struct or
 * jpeg_decompress_struct.  All three structs must agree on these
 * initial fields!  (This would be a lot cleaner in C++.)
 */
pub type j_common_ptr = *mut jpeg_common_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_memory_mgr {
    pub alloc_small:
        Option<unsafe extern "C" fn(_: j_common_ptr, _: c_int, _: size_t) -> *mut c_void>,
    pub alloc_large:
        Option<unsafe extern "C" fn(_: j_common_ptr, _: c_int, _: size_t) -> *mut c_void>,
    pub alloc_sarray: Option<
        unsafe extern "C" fn(_: j_common_ptr, _: c_int, _: JDIMENSION, _: JDIMENSION) -> JSAMPARRAY,
    >,
    pub alloc_barray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: c_int,
            _: JDIMENSION,
            _: JDIMENSION,
        ) -> JBLOCKARRAY,
    >,
    pub request_virt_sarray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: c_int,
            _: boolean,
            _: JDIMENSION,
            _: JDIMENSION,
            _: JDIMENSION,
        ) -> jvirt_sarray_ptr,
    >,
    pub request_virt_barray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: c_int,
            _: boolean,
            _: JDIMENSION,
            _: JDIMENSION,
            _: JDIMENSION,
        ) -> jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub access_virt_sarray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: jvirt_sarray_ptr,
            _: JDIMENSION,
            _: JDIMENSION,
            _: boolean,
        ) -> JSAMPARRAY,
    >,
    pub access_virt_barray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: jvirt_barray_ptr,
            _: JDIMENSION,
            _: JDIMENSION,
            _: boolean,
        ) -> JBLOCKARRAY,
    >,
    pub free_pool: Option<unsafe extern "C" fn(_: j_common_ptr, _: c_int) -> ()>,
    pub self_destruct: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub max_memory_to_use: c_long,
    pub max_alloc_chunk: c_long,
}
pub type jvirt_barray_ptr = *mut jvirt_barray_control;
pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
/* "Object" declarations for JPEG modules that may be supplied or called
 * directly by the surrounding application.
 * As with all objects in the JPEG library, these structs only define the
 * publicly visible methods and state variables of a module.  Additional
 * private fields may exist after the public ones.
 */

/* Error handler object */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub emit_message: Option<unsafe extern "C" fn(_: j_common_ptr, _: c_int) -> ()>,
    pub output_message: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub format_message: Option<unsafe extern "C" fn(_: j_common_ptr, _: *mut c_char) -> ()>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub msg_code: c_int,
    pub msg_parm: C2RustUnnamed_3,
    pub trace_level: c_int,
    pub num_warnings: c_long,
    pub jpeg_message_table: *const *const c_char,
    pub last_jpeg_message: c_int,
    pub addon_message_table: *const *const c_char,
    pub first_addon_message: c_int,
    pub last_addon_message: c_int,
}
/* Master record for a compression instance */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_compress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut c_void,
    pub is_decompressor: boolean,
    pub global_state: c_int,
    pub dest: *mut jpeg_destination_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub input_components: c_int,
    pub in_color_space: J_COLOR_SPACE,
    pub input_gamma: c_double,
    pub data_precision: c_int,
    pub num_components: c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub num_scans: c_int,
    pub scan_info: *const jpeg_scan_info,
    pub raw_data_in: boolean,
    pub arith_code: boolean,
    pub optimize_coding: boolean,
    pub CCIR601_sampling: boolean,
    pub smoothing_factor: c_int,
    pub dct_method: J_DCT_METHOD,
    pub restart_interval: c_uint,
    pub restart_in_rows: c_int,
    pub write_JFIF_header: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub write_Adobe_marker: boolean,
    pub next_scanline: JDIMENSION,
    pub progressive_mode: boolean,
    pub max_h_samp_factor: c_int,
    pub max_v_samp_factor: c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub comps_in_scan: c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: c_int,
    pub MCU_membership: [c_int; 10],
    pub Ss: c_int,
    pub Se: c_int,
    pub Ah: c_int,
    pub Al: c_int,
    pub master: *mut jpeg_comp_master,
    pub main: *mut jpeg_c_main_controller,
    pub prep: *mut jpeg_c_prep_controller,
    pub coef: *mut jpeg_c_coef_controller,
    pub marker: *mut jpeg_marker_writer,
    pub cconvert: *mut jpeg_color_converter,
    pub downsample: *mut jpeg_downsampler,
    pub fdct: *mut jpeg_forward_dct,
    pub entropy: *mut jpeg_entropy_encoder,
    pub script_space: *mut jpeg_scan_info,
    pub script_space_size: c_int,
}
pub type j_compress_ptr = *mut jpeg_compress_struct;
/* Data destination object for compression */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(_: j_compress_ptr) -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
}
pub use crate::jmemmgr::jvirt_barray_control;
pub use crate::jmemmgr::jvirt_sarray_control;
use crate::jmorecfg_h::boolean;
use crate::jmorecfg_h::JCOEF;
use crate::jmorecfg_h::JDIMENSION;
use crate::jmorecfg_h::JOCTET;
use crate::jmorecfg_h::JSAMPLE;
use crate::jmorecfg_h::UINT16;
use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
use crate::stddef_h::size_t;
/* useful in a couple of places */
pub type JCOEFPTR = *mut JCOEF;
/* These 32-bit GUIDs and the corresponding jpeg_*_get_*_param()/
 * jpeg_*_set_*_param() functions allow for extending the libjpeg API without
 * breaking backward ABI compatibility.  The actual parameters are stored in
 * the opaque jpeg_comp_master and jpeg_decomp_master structs.
 */

/* Boolean extension parameters */
pub type J_BOOLEAN_PARAM = c_uint;
/* Floating point parameters */
pub type J_FLOAT_PARAM = c_uint;
/* Integer parameters */
pub type J_INT_PARAM = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: UINT8,
    pub original_length: c_uint,
    pub data_length: c_uint,
    pub data: *mut JOCTET,
}
/* The decompressor can save APPn and COM markers in a list of these: */
pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;
/* may be overridden in jconfig.h */

/* may be overridden in jconfig.h */

/* Dithering options for decompression. */
pub type J_DITHER_MODE = c_uint;
/* Master record for a decompression instance */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decompress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut c_void,
    pub is_decompressor: boolean,
    pub global_state: c_int,
    pub src: *mut jpeg_source_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub num_components: c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: c_uint,
    pub scale_denom: c_uint,
    pub output_gamma: c_double,
    pub buffered_image: boolean,
    pub raw_data_out: boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: boolean,
    pub do_block_smoothing: boolean,
    pub quantize_colors: boolean,
    pub dither_mode: J_DITHER_MODE,
    pub two_pass_quantize: boolean,
    pub desired_number_of_colors: c_int,
    pub enable_1pass_quant: boolean,
    pub enable_external_quant: boolean,
    pub enable_2pass_quant: boolean,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub out_color_components: c_int,
    pub output_components: c_int,
    pub rec_outbuf_height: c_int,
    pub actual_number_of_colors: c_int,
    pub colormap: JSAMPARRAY,
    pub output_scanline: JDIMENSION,
    pub input_scan_number: c_int,
    pub input_iMCU_row: JDIMENSION,
    pub output_scan_number: c_int,
    pub output_iMCU_row: JDIMENSION,
    pub coef_bits: *mut [c_int; 64],
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub data_precision: c_int,
    pub comp_info: *mut jpeg_component_info,
    pub progressive_mode: boolean,
    pub arith_code: boolean,
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub restart_interval: c_uint,
    pub saw_JFIF_marker: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub saw_Adobe_marker: boolean,
    pub Adobe_transform: UINT8,
    pub CCIR601_sampling: boolean,
    pub marker_list: jpeg_saved_marker_ptr,
    pub max_h_samp_factor: c_int,
    pub max_v_samp_factor: c_int,
    pub min_DCT_scaled_size: c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub sample_range_limit: *mut JSAMPLE,
    pub comps_in_scan: c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: c_int,
    pub MCU_membership: [c_int; 10],
    pub Ss: c_int,
    pub Se: c_int,
    pub Ah: c_int,
    pub Al: c_int,
    pub unread_marker: c_int,
    pub master: *mut jpeg_decomp_master,
    pub main: *mut jpeg_d_main_controller,
    pub coef: *mut jpeg_d_coef_controller,
    pub post: *mut jpeg_d_post_controller,
    pub inputctl: *mut jpeg_input_controller,
    pub marker: *mut jpeg_marker_reader,
    pub entropy: *mut jpeg_entropy_decoder,
    pub idct: *mut jpeg_inverse_dct,
    pub upsample: *mut jpeg_upsampler,
    pub cconvert: *mut jpeg_color_deconverter,
    pub cquantize: *mut jpeg_color_quantizer,
}
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
/* Routine signature for application-supplied marker processing methods.
 * Need not pass marker code since it is stored in cinfo->unread_marker.
 */
pub type jpeg_marker_parser_method = Option<unsafe extern "C" fn(_: j_decompress_ptr) -> boolean>;
/* Data source object for decompression */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: size_t,
    pub init_source: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub fill_input_buffer: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> boolean>,
    pub skip_input_data: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: c_long) -> ()>,
    pub resync_to_restart: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: c_int) -> boolean>,
    pub term_source: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
}
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub type jpeg_idct_method = Option<
    unsafe extern "C" fn(
        _: j_decompress_ptr,
        _: *mut jpeg_component_info,
        _: JCOEFPTR,
        _: JSAMPARRAY,
        _: JDIMENSION,
    ) -> (),
>;
pub type jpeg_idct_method_selector = Option<
    unsafe extern "C" fn(
        _: j_decompress_ptr,
        _: *mut jpeg_component_info,
        _: *mut jpeg_idct_method,
        _: *mut c_int,
    ) -> (),
>;
use crate::stdlib::FILE;
/* recommended size of format_message buffer */
pub const JMSG_LENGTH_MAX: c_int = 200i32;
/* 5-bit red/6-bit green/5-bit blue */
pub const JCS_RGB565: J_COLOR_SPACE = 16;
/* alpha/red/green/blue */
pub const JCS_EXT_ARGB: J_COLOR_SPACE = 15;
/* alpha/blue/green/red */
pub const JCS_EXT_ABGR: J_COLOR_SPACE = 14;
/* blue/green/red/alpha */
pub const JCS_EXT_BGRA: J_COLOR_SPACE = 13;
/* When out_color_space it set to JCS_EXT_RGBX, JCS_EXT_BGRX, JCS_EXT_XBGR,
or JCS_EXT_XRGB during decompression, the X byte is undefined, and in
order to ensure the best performance, libjpeg-turbo can set that byte to
whatever value it wishes.  Use the following colorspace constants to
ensure that the X byte is set to 0xFF, so that it can be interpreted as an
opaque alpha channel. */

/* red/green/blue/alpha */
pub const JCS_EXT_RGBA: J_COLOR_SPACE = 12;
/* x/red/green/blue */
pub const JCS_EXT_XRGB: J_COLOR_SPACE = 11;
/* x/blue/green/red */
pub const JCS_EXT_XBGR: J_COLOR_SPACE = 10;
/* blue/green/red/x */
pub const JCS_EXT_BGRX: J_COLOR_SPACE = 9;
/* blue/green/red */
pub const JCS_EXT_BGR: J_COLOR_SPACE = 8;
/* red/green/blue/x */
pub const JCS_EXT_RGBX: J_COLOR_SPACE = 7;
/* red/green/blue */
pub const JCS_EXT_RGB: J_COLOR_SPACE = 6;
/* Y/Cb/Cr/K */
pub const JCS_YCCK: J_COLOR_SPACE = 5;
/* C/M/Y/K */
pub const JCS_CMYK: J_COLOR_SPACE = 4;
/* Y/Cb/Cr (also known as YUV) */
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
/* red/green/blue as specified by the RGB_RED,
RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE macros */
pub const JCS_RGB: J_COLOR_SPACE = 2;
/* monochrome */
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
/* error/unspecified */
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
/* floating-point: accurate, fast on fast HW */
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
/* faster, less accurate integer method */
pub const JDCT_IFAST: J_DCT_METHOD = 1;
/* slow but accurate integer algorithm */
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
/* libjpeg[-turbo] defaults (baseline, no mozjpeg extensions) */
pub const JCP_FASTEST: C2RustUnnamed_2 = 720002228;
/* best compression ratio (progressive, all mozjpeg extensions) */
pub const JCP_MAX_COMPRESSION: C2RustUnnamed_2 = 1560820397;
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

/* The basic DCT block is 8x8 samples */

/* DCTSIZE squared; # of elements in a block */

/* Quantization tables are numbered 0..3 */
pub const NUM_QUANT_TBLS: c_int = 4i32;
/* Huffman tables are numbered 0..3 */
pub const NUM_HUFF_TBLS: c_int = 4i32;
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
pub const JPOOL_PERMANENT: c_int = 0i32;
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

/* The basic DCT block is 8x8 samples */
pub const DCTSIZE: c_int = 8i32;
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

/* compressor's limit on blocks per MCU */
pub const C_MAX_BLOCKS_IN_MCU: c_int = 10i32;
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

/* lasts until done with image/datastream */
pub const JPOOL_IMAGE: c_int = 1i32;
/* DCTSIZE squared; # of elements in a block */
pub const DCTSIZE2: c_int = 64i32;
/* TRUE=preprocess input to reduce ringing of edges on white background */
pub const JBOOLEAN_OVERSHOOT_DERINGING: J_BOOLEAN_PARAM = 1061927929;
/* TRUE=optimize quant table in trellis loop */
pub const JBOOLEAN_TRELLIS_Q_OPT: J_BOOLEAN_PARAM = 3777684073;
/* TRUE=use scans in trellis optimization */
pub const JBOOLEAN_USE_SCANS_IN_TRELLIS: J_BOOLEAN_PARAM = 4253291573;
/* TRUE=use lambda weighting table */
pub const JBOOLEAN_USE_LAMBDA_WEIGHT_TBL: J_BOOLEAN_PARAM = 865973855;
/* TRUE=optimize for sequences of EOB */
pub const JBOOLEAN_TRELLIS_EOB_OPT: J_BOOLEAN_PARAM = 3623303040;
/* TRUE=use trellis quant for DC coefficient */
pub const JBOOLEAN_TRELLIS_QUANT_DC: J_BOOLEAN_PARAM = 865946636;
/* TRUE=use trellis quantization */
pub const JBOOLEAN_TRELLIS_QUANT: J_BOOLEAN_PARAM = 3306299443;
/* TRUE=optimize progressive coding scans */
pub const JBOOLEAN_OPTIMIZE_SCANS: J_BOOLEAN_PARAM = 1745618462;
pub const JFLOAT_TRELLIS_DELTA_DC_WEIGHT: J_FLOAT_PARAM = 326587475;
pub const JFLOAT_LAMBDA_LOG_SCALE2: J_FLOAT_PARAM = 3116084739;
pub const JFLOAT_LAMBDA_LOG_SCALE1: J_FLOAT_PARAM = 1533126041;
/* DC scan optimization mode */
pub const JINT_DC_SCAN_OPT_MODE: J_INT_PARAM = 199732540;
/* base quantization table index */
pub const JINT_BASE_QUANT_TBL_IDX: J_INT_PARAM = 1145645745;
/* number of trellis loops */
pub const JINT_TRELLIS_NUM_LOOPS: J_INT_PARAM = 3057565497;
/* splitting point for frequency in trellis quantization */
pub const JINT_TRELLIS_FREQ_SPLIT: J_INT_PARAM = 1873801511;
/* compression profile */
pub const JINT_COMPRESS_PROFILE: J_INT_PARAM = 3918628389;
/* These marker codes are exported since applications and data source modules
 * are likely to want to use them.
 */

/* RST0 marker code */

/* EOI marker code */

/* APP0 marker code */
pub const JPEG_APP0: c_int = 0xe0i32;
/* Huffman tables are numbered 0..3 */

/* Arith-coding tables are numbered 0..15 */

/* JPEG limit on # of components in one scan */
pub const MAX_COMPS_IN_SCAN: c_int = 4i32;
/* JPEG limit on sampling factors */
pub const MAX_SAMP_FACTOR: c_int = 4i32;
/* Floyd-Steinberg error diffusion dither */
pub const JDITHER_FS: J_DITHER_MODE = 2;
/* simple ordered dither */
pub const JDITHER_ORDERED: J_DITHER_MODE = 1;
/* no dithering */
pub const JDITHER_NONE: J_DITHER_MODE = 0;
/* lasts until done with image/datastream */
pub const JPOOL_NUMPOOLS: c_int = 2i32;
/* Huffman tables are numbered 0..3 */

/* Arith-coding tables are numbered 0..15 */
pub const NUM_ARITH_TBLS: c_int = 16i32;
/* may be overridden in jconfig.h */
pub const JDCT_DEFAULT: c_int = JDCT_ISLOW as c_int;
/* Found valid image datastream */
pub const JPEG_HEADER_OK: c_int = 1i32;
/* Found valid table-specs-only datastream */
pub const JPEG_HEADER_TABLES_ONLY: c_int = 2i32;
/* Return value is one of: */

/* Suspended due to lack of input data */
pub const JPEG_SUSPENDED: c_int = 0i32;
/* Return value is one of: */

/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */

/* Reached start of new scan */
pub const JPEG_REACHED_SOS: c_int = 1i32;
/* Reached end of image */
pub const JPEG_REACHED_EOI: c_int = 2i32;
/* Completed one iMCU row */
pub const JPEG_ROW_COMPLETED: c_int = 3i32;
/* These marker codes are exported since applications and data source modules
 * are likely to want to use them.
 */

/* RST0 marker code */

/* EOI marker code */
pub const JPEG_EOI: c_int = 0xd9i32;
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

/* compressor's limit on blocks per MCU */

/* decompressor's limit on blocks per MCU */
pub const D_MAX_BLOCKS_IN_MCU: c_int = 10i32;
/* Completed last iMCU row of a scan */
pub const JPEG_SCAN_COMPLETED: c_int = 4i32;
/* may be overridden in jconfig.h */

/* may be overridden in jconfig.h */
pub const JDCT_FASTEST: c_int = JDCT_IFAST as c_int;
/* COM marker code */
pub const JPEG_COM: c_int = 0xfei32;
