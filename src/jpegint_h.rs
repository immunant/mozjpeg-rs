#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decomp_master {
    pub prepare_for_output_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub finish_output_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub is_dummy_pass: crate::jmorecfg_h::boolean,
    pub first_iMCU_col: crate::jmorecfg_h::JDIMENSION,
    pub last_iMCU_col: crate::jmorecfg_h::JDIMENSION,
    pub first_MCU_col: [crate::jmorecfg_h::JDIMENSION; 10],
    pub last_MCU_col: [crate::jmorecfg_h::JDIMENSION; 10],
    pub jinit_upsampler_no_alloc: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_main_controller {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpegint_h::J_BUF_MODE,
        ) -> (),
    >,
    pub process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_coef_controller {
    pub start_input_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub consume_data:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int>,
    pub start_output_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub decompress_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
        ) -> libc::c_int,
    >,
    pub coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_post_controller {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpegint_h::J_BUF_MODE,
        ) -> (),
    >,
    pub post_process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_input_controller {
    pub consume_input:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int>,
    pub reset_input_controller:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub start_input_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub finish_input_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub has_multiple_scans: crate::jmorecfg_h::boolean,
    pub eoi_reached: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_reader {
    pub reset_marker_reader:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub read_markers:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int>,
    pub read_restart_marker: crate::jpeglib_h::jpeg_marker_parser_method,
    pub saw_SOI: crate::jmorecfg_h::boolean,
    pub saw_SOF: crate::jmorecfg_h::boolean,
    pub next_restart_num: libc::c_int,
    pub discarded_bytes: libc::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_decoder {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub decode_mcu: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: *mut crate::jpeglib_h::JBLOCKROW,
        ) -> crate::jmorecfg_h::boolean,
    >,
    pub insufficient_data: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_inverse_dct {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub inverse_DCT: [crate::jpegint_h::inverse_DCT_method_ptr; 10],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_upsampler {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub upsample: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
    pub need_context_rows: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_deconverter {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: libc::c_int,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_quantizer {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jmorecfg_h::boolean,
        ) -> (),
    >,
    pub color_quantize: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: libc::c_int,
        ) -> (),
    >,
    pub finish_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub new_color_map: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
}
pub type inverse_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: crate::jpeglib_h::j_decompress_ptr,
        _: *mut crate::jpeglib_h::jpeg_component_info,
        _: crate::jpeglib_h::JCOEFPTR,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
pub const DSTATE_START: libc::c_int = 200 as libc::c_int;
pub type JLONG = libc::c_long;
pub const DSTATE_STOPPING: libc::c_int = 210 as libc::c_int;
pub const DSTATE_RDCOEFS: libc::c_int = 209 as libc::c_int;
pub const DSTATE_SCANNING: libc::c_int = 205 as libc::c_int;
pub const DSTATE_RAW_OK: libc::c_int = 206 as libc::c_int;
pub const DSTATE_PRESCAN: libc::c_int = 204 as libc::c_int;
pub const DSTATE_PRELOAD: libc::c_int = 203 as libc::c_int;
pub const DSTATE_BUFIMAGE: libc::c_int = 207 as libc::c_int;
pub const DSTATE_READY: libc::c_int = 202 as libc::c_int;
pub const DSTATE_BUFPOST: libc::c_int = 208 as libc::c_int;
/*
 * Left shift macro that handles a negative operand without causing any
 * sanitizer warnings
 */

/* Declarations for compression modules */

/* Master control module */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_comp_master {
    pub prepare_for_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub pass_startup: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub finish_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub call_pass_startup: crate::jmorecfg_h::boolean,
    pub is_last_pass: crate::jmorecfg_h::boolean,
    pub optimize_scans: crate::jmorecfg_h::boolean,
    pub trellis_quant: crate::jmorecfg_h::boolean,
    pub trellis_quant_dc: crate::jmorecfg_h::boolean,
    pub trellis_eob_opt: crate::jmorecfg_h::boolean,
    pub use_lambda_weight_tbl: crate::jmorecfg_h::boolean,
    pub use_scans_in_trellis: crate::jmorecfg_h::boolean,
    pub trellis_passes: crate::jmorecfg_h::boolean,
    pub trellis_q_opt: crate::jmorecfg_h::boolean,
    pub overshoot_deringing: crate::jmorecfg_h::boolean,
    pub norm_src: [[libc::c_double; 64]; 4],
    pub norm_coef: [[libc::c_double; 64]; 4],
    pub compress_profile: libc::c_int,
    pub dc_scan_opt_mode: libc::c_int,
    pub quant_tbl_master_idx: libc::c_int,
    pub trellis_freq_split: libc::c_int,
    pub trellis_num_loops: libc::c_int,
    pub num_scans_luma: libc::c_int,
    pub num_scans_luma_dc: libc::c_int,
    pub num_scans_chroma_dc: libc::c_int,
    pub num_frequency_splits: libc::c_int,
    pub Al_max_luma: libc::c_int,
    pub Al_max_chroma: libc::c_int,
    pub lambda_log_scale1: libc::c_float,
    pub lambda_log_scale2: libc::c_float,
    pub trellis_delta_dc_weight: libc::c_float,
}
/* Main buffer control (downsampled-data buffer) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_main_controller {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpegint_h::J_BUF_MODE,
        ) -> (),
    >,
    pub process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
/* Compression preprocessing (downsampling input buffer control) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_prep_controller {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpegint_h::J_BUF_MODE,
        ) -> (),
    >,
    pub pre_process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
/* Coefficient buffer control */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_coef_controller {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpegint_h::J_BUF_MODE,
        ) -> (),
    >,
    pub compress_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
        ) -> crate::jmorecfg_h::boolean,
    >,
}
/* Marker writing */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_writer {
    pub write_file_header: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_frame_header: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_scan_header: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_file_trailer: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_tables_only: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_marker_header: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: libc::c_int,
            _: libc::c_uint,
        ) -> (),
    >,
    pub write_marker_byte:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr, _: libc::c_int) -> ()>,
}
/* Colorspace conversion */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_converter {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: libc::c_int,
        ) -> (),
    >,
}
/* Downsampling */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_downsampler {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub downsample: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
    pub need_context_rows: crate::jmorecfg_h::boolean,
}
/* TRUE if need rows above & below */

/* Forward DCT (also controls coefficient quantization) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_forward_dct {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub forward_DCT: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: *mut crate::jpeglib_h::jpeg_component_info,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JBLOCKROW,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JBLOCKROW,
        ) -> (),
    >,
}
/* Entropy encoding */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_encoder {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jmorecfg_h::boolean,
        ) -> (),
    >,
    pub encode_mcu: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: *mut crate::jpeglib_h::JBLOCKROW,
        ) -> crate::jmorecfg_h::boolean,
    >,
    pub finish_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
}
pub type J_BUF_MODE = libc::c_uint;
pub const JBUF_REQUANT: crate::jpegint_h::J_BUF_MODE = 4;
pub const JBUF_SAVE_AND_PASS: crate::jpegint_h::J_BUF_MODE = 3;
pub const JBUF_CRANK_DEST: crate::jpegint_h::J_BUF_MODE = 2;
pub const JBUF_SAVE_SOURCE: crate::jpegint_h::J_BUF_MODE = 1;
pub const JBUF_PASS_THRU: crate::jpegint_h::J_BUF_MODE = 0;
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */
pub const CSTATE_START: libc::c_int = 100 as libc::c_int;
/* after create_compress */
pub const CSTATE_SCANNING: libc::c_int = 101 as libc::c_int;
/* start_compress done, write_scanlines OK */
pub const CSTATE_RAW_OK: libc::c_int = 102 as libc::c_int;
/* start_compress done, write_raw_data OK */
pub const CSTATE_WRCOEFS: libc::c_int = 103 as libc::c_int;
