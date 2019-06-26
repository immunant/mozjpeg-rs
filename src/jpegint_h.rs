use libc::c_double;
use libc::c_float;
use libc::c_int;
use libc::c_long;
use libc::c_uchar;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
extern "C" {
    #[no_mangle]
    pub fn jinit_marker_writer(cinfo: j_compress_ptr);

    /* Memory manager initialization */
    #[no_mangle]
    pub fn jinit_memory_mgr(cinfo: j_common_ptr);

    /* Miscellaneous useful macros */
    /* We assume that right shift corresponds to signed division by 2 with
     * rounding towards minus infinity.  This is correct for typical "arithmetic
     * shift" instructions that shift in copies of the sign bit.  But some
     * C compilers implement >> with an unsigned shift.  For these machines you
     * must define RIGHT_SHIFT_IS_UNSIGNED.
     * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
     * It is only applied with constant shift counts.  SHIFT_TEMPS must be
     * included in the variables of any routine using RIGHT_SHIFT.
     */
    /* Compression module initialization routines */
    #[no_mangle]
    pub fn jinit_compress_master(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jround_up(a: c_long, b: c_long) -> c_long;
    #[no_mangle]
    pub fn jzero_far(target: *mut c_void, bytestozero: size_t);

    /* Constant tables in jutils.c */
    /* This table is not actually needed in v6a */
    /* zigzag coef order to natural order */
    #[no_mangle]
    pub static jpeg_natural_order: [c_int; 0];
    #[no_mangle]
    pub fn jinit_c_master_control(cinfo: j_compress_ptr, transcode_only: boolean);
    #[no_mangle]
    pub fn jinit_c_main_controller(cinfo: j_compress_ptr, need_full_buffer: boolean);
    #[no_mangle]
    pub fn jinit_c_prep_controller(cinfo: j_compress_ptr, need_full_buffer: boolean);
    #[no_mangle]
    pub fn jinit_c_coef_controller(cinfo: j_compress_ptr, need_full_buffer: boolean);
    #[no_mangle]
    pub fn jinit_color_converter(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jinit_downsampler(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jinit_forward_dct(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jinit_huff_encoder(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jinit_phuff_encoder(cinfo: j_compress_ptr);
    #[no_mangle]
    pub fn jpeg_mem_dest_internal(
        cinfo: j_compress_ptr,
        outbuffer: *mut *mut c_uchar,
        outsize: *mut c_ulong,
        pool_id: c_int,
    );

    /* Utility routines in jutils.c */
    #[no_mangle]
    pub fn jdiv_round_up(a: c_long, b: c_long) -> c_long;
    #[no_mangle]
    pub fn jcopy_sample_rows(
        input_array: JSAMPARRAY,
        source_row: c_int,
        output_array: JSAMPARRAY,
        dest_row: c_int,
        num_rows: c_int,
        num_cols: JDIMENSION,
    );
    #[no_mangle]
    pub fn jinit_input_controller(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_marker_reader(cinfo: j_decompress_ptr);

    /* Decompression module initialization routines */
    #[no_mangle]
    pub fn jinit_master_decompress(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_upsampler(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jcopy_block_row(input_row: JBLOCKROW, output_row: JBLOCKROW, num_blocks: JDIMENSION);
    #[no_mangle]
    pub fn jinit_d_main_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean);
    #[no_mangle]
    pub fn jinit_d_coef_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean);
    #[no_mangle]
    pub fn jinit_d_post_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean);
    #[no_mangle]
    pub fn jinit_huff_decoder(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_phuff_decoder(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_inverse_dct(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_color_deconverter(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_1pass_quantizer(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_2pass_quantizer(cinfo: j_decompress_ptr);
    #[no_mangle]
    pub fn jinit_merged_upsampler(cinfo: j_decompress_ptr);
}
/* Entropy encoding */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_encoder {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr, _: boolean) -> ()>,
    pub encode_mcu: Option<unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean>,
    pub finish_pass: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
}
/* Forward DCT (also controls coefficient quantization) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_forward_dct {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub forward_DCT: Option<
        unsafe extern "C" fn(
            _: j_compress_ptr,
            _: *mut jpeg_component_info,
            _: JSAMPARRAY,
            _: JBLOCKROW,
            _: JDIMENSION,
            _: JDIMENSION,
            _: JDIMENSION,
            _: JBLOCKROW,
        ) -> (),
    >,
}
/* Downsampling */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_downsampler {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub downsample: Option<
        unsafe extern "C" fn(
            _: j_compress_ptr,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: JSAMPIMAGE,
            _: JDIMENSION,
        ) -> (),
    >,
    pub need_context_rows: boolean,
}
/* Colorspace conversion */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_converter {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            _: j_compress_ptr,
            _: JSAMPARRAY,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: c_int,
        ) -> (),
    >,
}
/* Marker writing */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_writer {
    pub write_file_header: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub write_frame_header: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub write_scan_header: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub write_file_trailer: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub write_tables_only: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub write_marker_header:
        Option<unsafe extern "C" fn(_: j_compress_ptr, _: c_int, _: c_uint) -> ()>,
    pub write_marker_byte: Option<unsafe extern "C" fn(_: j_compress_ptr, _: c_int) -> ()>,
}
/* Coefficient buffer control */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_coef_controller {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr, _: J_BUF_MODE) -> ()>,
    pub compress_data: Option<unsafe extern "C" fn(_: j_compress_ptr, _: JSAMPIMAGE) -> boolean>,
}
/*
 * jpegint.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 1997-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015-2016, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file provides common declarations for the various JPEG modules.
 * These declarations are considered internal to the JPEG library; most
 * applications using the library shouldn't need to include this file.
 */

/* Declarations for both compression & decompression */

/* Operating modes for buffer controllers */
pub type J_BUF_MODE = c_uint;
/* Compression preprocessing (downsampling input buffer control) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_prep_controller {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr, _: J_BUF_MODE) -> ()>,
    pub pre_process_data: Option<
        unsafe extern "C" fn(
            _: j_compress_ptr,
            _: JSAMPARRAY,
            _: *mut JDIMENSION,
            _: JDIMENSION,
            _: JSAMPIMAGE,
            _: *mut JDIMENSION,
            _: JDIMENSION,
        ) -> (),
    >,
}
/* Main buffer control (downsampled-data buffer) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr, _: J_BUF_MODE) -> ()>,
    pub process_data: Option<
        unsafe extern "C" fn(
            _: j_compress_ptr,
            _: JSAMPARRAY,
            _: *mut JDIMENSION,
            _: JDIMENSION,
        ) -> (),
    >,
}
/*
 * Left shift macro that handles a negative operand without causing any
 * sanitizer warnings
 */

/* Declarations for compression modules */

/* Master control module */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_comp_master {
    pub prepare_for_pass: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub pass_startup: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub finish_pass: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub call_pass_startup: boolean,
    pub is_last_pass: boolean,
    pub optimize_scans: boolean,
    pub trellis_quant: boolean,
    pub trellis_quant_dc: boolean,
    pub trellis_eob_opt: boolean,
    pub use_lambda_weight_tbl: boolean,
    pub use_scans_in_trellis: boolean,
    pub trellis_passes: boolean,
    pub trellis_q_opt: boolean,
    pub overshoot_deringing: boolean,
    pub norm_src: [[c_double; 64]; 4],
    pub norm_coef: [[c_double; 64]; 4],
    pub compress_profile: c_int,
    pub dc_scan_opt_mode: c_int,
    pub quant_tbl_master_idx: c_int,
    pub trellis_freq_split: c_int,
    pub trellis_num_loops: c_int,
    pub num_scans_luma: c_int,
    pub num_scans_luma_dc: c_int,
    pub num_scans_chroma_dc: c_int,
    pub num_frequency_splits: c_int,
    pub Al_max_luma: c_int,
    pub Al_max_chroma: c_int,
    pub lambda_log_scale1: c_float,
    pub lambda_log_scale2: c_float,
    pub trellis_delta_dc_weight: c_float,
}
use crate::jmorecfg_h::boolean;
use crate::jmorecfg_h::JDIMENSION;
use crate::jpeglib_h::j_common_ptr;
use crate::jpeglib_h::j_compress_ptr;
use crate::jpeglib_h::jpeg_common_struct;
use crate::jpeglib_h::jpeg_component_info;
use crate::jpeglib_h::jpeg_compress_struct;
use crate::jpeglib_h::JBLOCKROW;
use crate::jpeglib_h::JSAMPARRAY;
use crate::jpeglib_h::JSAMPIMAGE;
use crate::stddef_h::size_t;
use ::libc;
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */

/* after create_compress */

/* start_compress done, write_scanlines OK */

/* start_compress done, write_raw_data OK */

/* jpeg_write_coefficients done */

/* after create_decompress */

/* reading header markers, no SOS yet */

/* found SOS, ready for start_decompress */

/* reading multiscan file in start_decompress*/

/* performing dummy pass for 2-pass quant */

/* start_decompress done, read_scanlines OK */

/* start_decompress done, read_raw_data OK */

/* expecting jpeg_start_output */

/* looking for SOS/EOI in jpeg_finish_output */

/* reading file in jpeg_read_coefficients */

/* looking for EOI in jpeg_finish_decompress */

/* JLONG must hold at least signed 32-bit values. */
pub type JLONG = c_long;
/* Color quantization or color precision reduction */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_quantizer {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: boolean) -> ()>,
    pub color_quantize: Option<
        unsafe extern "C" fn(_: j_decompress_ptr, _: JSAMPARRAY, _: JSAMPARRAY, _: c_int) -> (),
    >,
    pub finish_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub new_color_map: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
}
/* Colorspace conversion */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_deconverter {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            _: j_decompress_ptr,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: c_int,
        ) -> (),
    >,
}
/* Upsampling (note that upsampler must also call color converter) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_upsampler {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub upsample: Option<
        unsafe extern "C" fn(
            _: j_decompress_ptr,
            _: JSAMPIMAGE,
            _: *mut JDIMENSION,
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: *mut JDIMENSION,
            _: JDIMENSION,
        ) -> (),
    >,
    pub need_context_rows: boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_inverse_dct {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub inverse_DCT: [inverse_DCT_method_ptr; 10],
}
/* Inverse DCT (also performs dequantization) */
pub type inverse_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: j_decompress_ptr,
        _: *mut jpeg_component_info,
        _: JCOEFPTR,
        _: JSAMPARRAY,
        _: JDIMENSION,
    ) -> (),
>;
/* Entropy decoding */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_decoder {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub decode_mcu: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: *mut JBLOCKROW) -> boolean>,
    pub insufficient_data: boolean,
}
/* Marker reading & parsing */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_reader {
    pub reset_marker_reader: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub read_markers: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> c_int>,
    pub read_restart_marker: jpeg_marker_parser_method,
    pub saw_SOI: boolean,
    pub saw_SOF: boolean,
    pub next_restart_num: c_int,
    pub discarded_bytes: c_uint,
}
/* Input control module */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_input_controller {
    pub consume_input: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> c_int>,
    pub reset_input_controller: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub start_input_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub finish_input_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub has_multiple_scans: boolean,
    pub eoi_reached: boolean,
}
/* Decompression postprocessing (color quantization buffer control) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_post_controller {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: J_BUF_MODE) -> ()>,
    pub post_process_data: Option<
        unsafe extern "C" fn(
            _: j_decompress_ptr,
            _: JSAMPIMAGE,
            _: *mut JDIMENSION,
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: *mut JDIMENSION,
            _: JDIMENSION,
        ) -> (),
    >,
}
/* Coefficient buffer control */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_coef_controller {
    pub start_input_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub consume_data: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> c_int>,
    pub start_output_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub decompress_data: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: JSAMPIMAGE) -> c_int>,
    pub coef_arrays: *mut jvirt_barray_ptr,
}
/* Main buffer control (downsampled-data buffer) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: J_BUF_MODE) -> ()>,
    pub process_data: Option<
        unsafe extern "C" fn(
            _: j_decompress_ptr,
            _: JSAMPARRAY,
            _: *mut JDIMENSION,
            _: JDIMENSION,
        ) -> (),
    >,
}
/* Declarations for decompression modules */

/* Master control module */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decomp_master {
    pub prepare_for_output_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub finish_output_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub is_dummy_pass: boolean,
    pub first_iMCU_col: JDIMENSION,
    pub last_iMCU_col: JDIMENSION,
    pub first_MCU_col: [JDIMENSION; 10],
    pub last_MCU_col: [JDIMENSION; 10],
    pub jinit_upsampler_no_alloc: boolean,
}
use crate::jpeglib_h::j_decompress_ptr;
use crate::jpeglib_h::jpeg_decompress_struct;
use crate::jpeglib_h::jpeg_marker_parser_method;
use crate::jpeglib_h::jvirt_barray_ptr;
use crate::jpeglib_h::JBLOCK;
use crate::jpeglib_h::JCOEFPTR;
use crate::jpeglib_h::JSAMPROW;
/* Requantize */
pub const JBUF_REQUANT: J_BUF_MODE = 4;
/* Run both subobjects, save output */
pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = 3;
/* Run dest subobject only, using saved data */
pub const JBUF_CRANK_DEST: J_BUF_MODE = 2;
/* Remaining modes require a full-image buffer to have been created */

/* Run source subobject only, save output */
pub const JBUF_SAVE_SOURCE: J_BUF_MODE = 1;
/* Plain stripwise operation */
pub const JBUF_PASS_THRU: J_BUF_MODE = 0;
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */

/* after create_compress */
pub const CSTATE_START: c_int = 100i32;
/* start_compress done, write_scanlines OK */
pub const CSTATE_SCANNING: c_int = 101i32;
/* start_compress done, write_raw_data OK */
pub const CSTATE_RAW_OK: c_int = 102i32;
/* jpeg_write_coefficients done */
pub const CSTATE_WRCOEFS: c_int = 103i32;
/* start_compress done, write_scanlines OK */

/* start_compress done, write_raw_data OK */

/* jpeg_write_coefficients done */

/* after create_decompress */
pub const DSTATE_START: c_int = 200i32;
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */

/* after create_compress */

/* start_compress done, write_scanlines OK */

/* start_compress done, write_raw_data OK */

/* jpeg_write_coefficients done */

/* after create_decompress */

/* reading header markers, no SOS yet */

/* found SOS, ready for start_decompress */
pub const DSTATE_READY: c_int = 202i32;
/* reading multiscan file in start_decompress*/
pub const DSTATE_PRELOAD: c_int = 203i32;
/* performing dummy pass for 2-pass quant */
pub const DSTATE_PRESCAN: c_int = 204i32;
/* start_decompress done, read_scanlines OK */
pub const DSTATE_SCANNING: c_int = 205i32;
/* start_decompress done, read_raw_data OK */
pub const DSTATE_RAW_OK: c_int = 206i32;
/* expecting jpeg_start_output */
pub const DSTATE_BUFIMAGE: c_int = 207i32;
/* looking for SOS/EOI in jpeg_finish_output */
pub const DSTATE_BUFPOST: c_int = 208i32;
/* looking for SOS/EOI in jpeg_finish_output */

/* reading file in jpeg_read_coefficients */
pub const DSTATE_RDCOEFS: c_int = 209i32;
/* looking for EOI in jpeg_finish_decompress */
pub const DSTATE_STOPPING: c_int = 210i32;
