use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
extern "C" {
    #[no_mangle]
    pub fn jpeg_write_m_header(cinfo: j_compress_ptr, marker: c_int, datalen: c_uint);

    #[no_mangle]
    pub fn jpeg_write_m_byte(cinfo: j_compress_ptr, val: c_int);
    /* Read or write raw DCT coefficients --- useful for lossless transcoding. */
    #[no_mangle]
    pub fn jpeg_read_coefficients(cinfo: j_decompress_ptr) -> *mut jvirt_barray_ptr;

    #[no_mangle]
    pub fn jpeg_write_coefficients(cinfo: j_compress_ptr, coef_arrays: *mut jvirt_barray_ptr);

    #[no_mangle]
    pub fn jpeg_copy_critical_parameters(srcinfo: j_decompress_ptr, dstinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jpeg_c_int_param_supported(cinfo: j_compress_ptr, param: J_INT_PARAM) -> boolean;

    #[no_mangle]
    pub fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *mut JQUANT_TBL;

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
    pub fn jpeg_suppress_tables(cinfo: j_compress_ptr, suppress: boolean);

    #[no_mangle]
    pub fn jpeg_resync_to_restart(cinfo: j_decompress_ptr, desired: c_int) -> boolean;
    /* Generic versions of jpeg_abort and jpeg_destroy that work on either
     * flavor of JPEG object.  These may be more convenient in some places.
     */
    #[no_mangle]
    pub fn jpeg_destroy(cinfo: j_common_ptr);

    #[no_mangle]
    pub fn jpeg_abort(cinfo: j_common_ptr);

    #[no_mangle]
    pub fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *mut JHUFF_TBL;

    #[no_mangle]
    pub fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;

    #[no_mangle]
    pub fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: c_int, structsize: size_t);

    #[no_mangle]
    pub fn jpeg_destroy_compress(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *mut FILE);

    #[no_mangle]
    pub fn jpeg_mem_dest(
        cinfo: j_compress_ptr,
        outbuffer: *mut *mut c_uchar,
        outsize: *mut c_ulong,
    );

    #[no_mangle]
    pub fn jpeg_set_defaults(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jpeg_set_colorspace(cinfo: j_compress_ptr, colorspace: J_COLOR_SPACE);

    #[no_mangle]
    pub fn jpeg_default_colorspace(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jpeg_set_quality(cinfo: j_compress_ptr, quality: c_int, force_baseline: boolean);

    #[no_mangle]
    pub fn jpeg_simple_progression(cinfo: j_compress_ptr);

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

    #[no_mangle]
    pub fn jpeg_write_marker(
        cinfo: j_compress_ptr,
        marker: c_int,
        dataptr: *const JOCTET,
        datalen: c_uint,
    );

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
    pub fn jpeg_CreateDecompress(cinfo: j_decompress_ptr, version: c_int, structsize: size_t);

    #[no_mangle]
    pub fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *mut FILE);

    #[no_mangle]
    pub fn jpeg_mem_src(cinfo: j_decompress_ptr, inbuffer: *const c_uchar, insize: c_ulong);
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
    pub fn jpeg_skip_scanlines(cinfo: j_decompress_ptr, num_lines: JDIMENSION) -> JDIMENSION;

    #[no_mangle]
    pub fn jpeg_crop_scanline(
        cinfo: j_decompress_ptr,
        xoffset: *mut JDIMENSION,
        width: *mut JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
    /* Control saving of COM and APPn markers into marker_list. */
    #[no_mangle]
    pub fn jpeg_save_markers(cinfo: j_decompress_ptr, marker_code: c_int, length_limit: c_uint);
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

    #[no_mangle]
    pub fn jpeg_write_raw_data(
        cinfo: j_compress_ptr,
        data: JSAMPIMAGE,
        num_lines: JDIMENSION,
    ) -> JDIMENSION;

    #[no_mangle]
    pub fn jpeg_read_raw_data(
        cinfo: j_decompress_ptr,
        data: JSAMPIMAGE,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;

    #[no_mangle]
    pub fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jpeg_abort_compress(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jpeg_abort_decompress(cinfo: j_decompress_ptr);
}
use crate::stdlib::FILE;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_decoder {
    pub start_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub decode_mcu: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: *mut JBLOCKROW) -> boolean>,
    pub insufficient_data: boolean,
}
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_coef_controller {
    pub start_input_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub consume_data: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> c_int>,
    pub start_output_pass: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub decompress_data: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: JSAMPIMAGE) -> c_int>,
    pub coef_arrays: *mut jvirt_barray_ptr,
}
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
/* lasts until master record is destroyed */

/* lasts until done with image/datastream */
pub const JPOOL_NUMPOOLS: c_int = 2i32;
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
pub const JPOOL_IMAGE: c_int = 1i32;

use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
use crate::stddef_h::size_t;
/* Quantization tables are numbered 0..3 */

/* Huffman tables are numbered 0..3 */
pub const NUM_ARITH_TBLS: c_int = 16i32;
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
pub const DCTSIZE: c_int = 8i32;
/* Quantization tables are numbered 0..3 */

/* Huffman tables are numbered 0..3 */

/* Arith-coding tables are numbered 0..15 */
pub const MAX_COMPS_IN_SCAN: c_int = 4i32;
/* JPEG limit on # of components in one scan */
pub const MAX_SAMP_FACTOR: c_int = 4i32;
use crate::jpegint_h::J_BUF_MODE;
/* a 3-D array of coefficient blocks */
pub type JCOEFPTR = *mut JCOEF;
use crate::jpegint_h::inverse_DCT_method_ptr;
/* The basic DCT block is 8x8 samples */
pub const DCTSIZE2: c_int = 64i32;
/* Return value is one of: */

/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */
pub const JPEG_REACHED_SOS: c_int = 1i32;
/* Reached start of new scan */

/* Reached end of image */
pub const JPEG_ROW_COMPLETED: c_int = 3i32;
pub const JPEG_REACHED_EOI: c_int = 2i32;
pub const JPEG_SUSPENDED: c_int = 0i32;
/* These marker codes are exported since applications and data source modules
 * are likely to want to use them.
 */

/* RST0 marker code */
pub const JPEG_EOI: c_int = 0xd9i32;
pub const D_MAX_BLOCKS_IN_MCU: c_int = 10i32;
pub const JPEG_SCAN_COMPLETED: c_int = 4i32;
/* Entropy encoding */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_encoder {
    pub start_pass: Option<unsafe extern "C" fn(_: j_compress_ptr, _: boolean) -> ()>,
    pub encode_mcu: Option<unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean>,
    pub finish_pass: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
}
/* TRUE if need rows above & below */

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

/* Data structures for images (arrays of samples and of DCT coefficients).
 */

/* ptr to one image row of pixel samples. */

/* ptr to some rows (a 2-D sample array) */

/* a 3-D sample array: top index is color */

/* one block of coefficients */

/* pointer to one row of coefficient blocks */

/* a 2-D array of coefficient blocks */

/* a 3-D array of coefficient blocks */

/* useful in a couple of places */

/* Types for JPEG compression parameters and working tables. */

/* DCT coefficient quantization tables. */

/* This array gives the coefficient quantizers in natural array order
 * (not the zigzag order in which they are stored in a JPEG DQT marker).
 * CAUTION: IJG versions prior to v6a kept this array in zigzag order.
 */

/* quantization step for each coefficient */

/* This field is used only during compression.  It's initialized FALSE when
 * the table is created, and set TRUE when it's been output to the file.
 * You could suppress output of a table by setting this to TRUE.
 * (See jpeg_suppress_tables for an example.)
 */

/* TRUE when table has been output */

/* Huffman coding tables. */

/* These two fields directly represent the contents of a JPEG DHT marker */

/* bits[k] = # of symbols with codes of */

/* length k bits; bits[0] is unused */

/* The symbols, in order of incr code length */

/* This field is used only during compression.  It's initialized FALSE when
 * the table is created, and set TRUE when it's been output to the file.
 * You could suppress output of a table by setting this to TRUE.
 * (See jpeg_suppress_tables for an example.)
 */

/* TRUE when table has been output */

/* Basic info about one component (color channel). */

/* These values are fixed over the whole image. */

/* For compression, they must be supplied by parameter setup; */

/* for decompression, they are read from the SOF marker. */

/* identifier for this component (0..255) */

/* its index in SOF or cinfo->comp_info[] */

/* horizontal sampling factor (1..4) */

/* vertical sampling factor (1..4) */

/* quantization table selector (0..3) */

/* These values may vary between scans. */

/* For compression, they must be supplied by parameter setup; */

/* for decompression, they are read from the SOS marker. */

/* The decompressor output side may not use these variables. */

/* DC entropy table selector (0..3) */

/* AC entropy table selector (0..3) */

/* Remaining fields should be treated as private by applications. */

/* These values are computed during compression or decompression startup: */

/* Component's size in DCT blocks.
 * Any dummy blocks added to complete an MCU are not counted; therefore
 * these values do not depend on whether a scan is interleaved or not.
 */

/* Size of a DCT block in samples.  Always DCTSIZE for compression.
 * For decompression this is the size of the output from one DCT block,
 * reflecting any scaling we choose to apply during the IDCT step.
 * Values from 1 to 16 are supported.
 * Note that different components may receive different IDCT scalings.
 */

/* The downsampled dimensions are the component's actual, unpadded number
 * of samples at the main buffer (preprocessing/compression interface), thus
 * downsampled_width = ceil(image_width * Hi/Hmax)
 * and similarly for height.  For decompression, IDCT scaling is included, so
 * downsampled_width = ceil(image_width * Hi/Hmax * DCT_[h_]scaled_size/DCTSIZE)
 */

/* actual width in samples */

/* actual height in samples */

/* This flag is used only for decompression.  In cases where some of the
 * components will be ignored (eg grayscale output from YCbCr image),
 * we can skip most computations for the unused components.
 */

/* do we need the value of this component? */

/* These values are computed before starting a scan of the component. */

/* The decompressor output side may not use these variables. */

/* number of blocks per MCU, horizontally */

/* number of blocks per MCU, vertically */

/* MCU_width * MCU_height */

/* MCU width in samples, MCU_width*DCT_[h_]scaled_size */

/* # of non-dummy blocks across in last MCU */

/* # of non-dummy blocks down in last MCU */

/* Saved quantization table for component; NULL if none yet saved.
 * See jdinput.c comments about the need for this information.
 * This field is currently used only for decompression.
 */

/* Private per-component storage for DCT or IDCT subsystem. */

/* The script for encoding a multiple-scan file is an array of these: */

/* number of components encoded in this scan */

/* their SOF/comp_info[] indexes */

/* progressive JPEG spectral selection parms */

/* progressive JPEG successive approx. parms */

/* The decompressor can save APPn and COM markers in a list of these: */

/* next in list, or NULL */

/* marker code: JPEG_COM, or JPEG_APP0+n */

/* # bytes of data in the file */

/* # bytes of data saved at data[] */

/* the data contained in the marker */

/* the marker length word is not counted in data_length or original_length */

/* Known color spaces. */

/* error/unspecified */

/* monochrome */

/* red/green/blue as specified by the RGB_RED,
RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE macros */

/* Y/Cb/Cr (also known as YUV) */

/* C/M/Y/K */

/* Y/Cb/Cr/K */

/* red/green/blue */

/* red/green/blue/x */

/* blue/green/red */

/* blue/green/red/x */

/* x/blue/green/red */

/* x/red/green/blue */

/* When out_color_space it set to JCS_EXT_RGBX, JCS_EXT_BGRX, JCS_EXT_XBGR,
or JCS_EXT_XRGB during decompression, the X byte is undefined, and in
order to ensure the best performance, libjpeg-turbo can set that byte to
whatever value it wishes.  Use the following colorspace constants to
ensure that the X byte is set to 0xFF, so that it can be interpreted as an
opaque alpha channel. */

/* red/green/blue/alpha */

/* blue/green/red/alpha */

/* alpha/blue/green/red */

/* alpha/red/green/blue */

/* 5-bit red/6-bit green/5-bit blue */

/* DCT/IDCT algorithm options. */

/* slow but accurate integer algorithm */

/* faster, less accurate integer method */

/* floating-point: accurate, fast on fast HW */

/* may be overridden in jconfig.h */

/* may be overridden in jconfig.h */

/* Dithering options for decompression. */

/* no dithering */

/* simple ordered dither */

/* Floyd-Steinberg error diffusion dither */

/* These 32-bit GUIDs and the corresponding jpeg_*_get_*_param()/
 * jpeg_*_set_*_param() functions allow for extending the libjpeg API without
 * breaking backward ABI compatibility.  The actual parameters are stored in
 * the opaque jpeg_comp_master and jpeg_decomp_master structs.
 */

/* Boolean extension parameters */

/* TRUE=optimize progressive coding scans */

/* TRUE=use trellis quantization */

/* TRUE=use trellis quant for DC coefficient */

/* TRUE=optimize for sequences of EOB */

/* TRUE=use lambda weighting table */

/* TRUE=use scans in trellis optimization */

/* TRUE=optimize quant table in trellis loop */

/* TRUE=preprocess input to reduce ringing of edges on white background */

/* Floating point parameters */

/* Integer parameters */

/* compression profile */

/* splitting point for frequency in trellis quantization */

/* number of trellis loops */

/* base quantization table index */

/* DC scan optimization mode */

/* Values for the JINT_COMPRESS_PROFILE parameter (32-bit GUIDs) */

/* best compression ratio (progressive, all mozjpeg extensions) */

/* libjpeg[-turbo] defaults (baseline, no mozjpeg extensions) */

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

/* Fields common to both master struct types */

/* Additional fields follow in an actual jpeg_compress_struct or
 * jpeg_decompress_struct.  All three structs must agree on these
 * initial fields!  (This would be a lot cleaner in C++.)
 */

/* Master record for a compression instance */

/* Fields shared with jpeg_decompress_struct */

/* Destination for compressed data */

/* Description of source image --- these fields must be filled in by
 * outer application before starting compression.  in_color_space must
 * be correct before you can even call jpeg_set_defaults().
 */

/* input image width */

/* input image height */

/* # of color components in input image */

/* colorspace of input image */

/* image gamma of input image */

/* Compression parameters --- these fields must be set before calling
 * jpeg_start_compress().  We recommend calling jpeg_set_defaults() to
 * initialize everything to reasonable defaults, then changing anything
 * the application specifically wants to change.  That way you won't get
 * burnt when new parameters are added.  Also note that there are several
 * helper routines to simplify changing parameters.
 */

/* bits of precision in image data */

/* # of color components in JPEG image */

/* colorspace of JPEG image */

/* comp_info[i] describes component that appears i'th in SOF */

/* ptrs to coefficient quantization tables, or NULL if not defined,
 * and corresponding scale factors (percentage, initialized 100).
 */

/* ptrs to Huffman coding tables, or NULL if not defined */

/* L values for DC arith-coding tables */

/* U values for DC arith-coding tables */

/* Kx values for AC arith-coding tables */

/* # of entries in scan_info array */

/* script for multi-scan file, or NULL */

/* The default value of scan_info is NULL, which causes a single-scan
 * sequential JPEG file to be emitted.  To create a multi-scan file,
 * set num_scans and scan_info to point to an array of scan definitions.
 */

/* TRUE=caller supplies downsampled data */

/* TRUE=arithmetic coding, FALSE=Huffman */

/* TRUE=optimize entropy encoding parms */

/* TRUE=first samples are cosited */

/* 1..100, or 0 for no input smoothing */

/* DCT algorithm selector */

/* The restart interval can be specified in absolute MCUs by setting
 * restart_interval, or in MCU rows by setting restart_in_rows
 * (in which case the correct restart_interval will be figured
 * for each scan).
 */

/* MCUs per restart, or 0 for no restart */

/* if > 0, MCU rows per restart interval */

/* Parameters controlling emission of special markers. */

/* should a JFIF marker be written? */

/* What to write for the JFIF version number */

/* These three values are not used by the JPEG code, merely copied */

/* into the JFIF APP0 marker.  density_unit can be 0 for unknown, */

/* 1 for dots/inch, or 2 for dots/cm.  Note that the pixel aspect */

/* ratio is defined by X_density/Y_density even when density_unit=0. */

/* JFIF code for pixel size units */

/* Horizontal pixel density */

/* Vertical pixel density */

/* should an Adobe marker be written? */

/* State variable: index of next scanline to be written to
 * jpeg_write_scanlines().  Application may use this to control its
 * processing loop, e.g., "while (next_scanline < image_height)".
 */

/* 0 .. image_height-1  */

/* Remaining fields are known throughout compressor, but generally
 * should not be touched by a surrounding application.
 */

/*
 * These fields are computed during compression startup
 */

/* TRUE if scan script uses progressive mode */

/* largest h_samp_factor */

/* largest v_samp_factor */

/* # of iMCU rows to be input to coef ctlr */

/* The coefficient controller receives data in units of MCU rows as defined
 * for fully interleaved scans (whether the JPEG file is interleaved or not).
 * There are v_samp_factor * DCTSIZE sample rows of each component in an
 * "iMCU" (interleaved MCU) row.
 */

/*
 * These fields are valid during any one scan.
 * They describe the components and MCUs actually appearing in the scan.
 */

/* # of JPEG components in this scan */

/* *cur_comp_info[i] describes component that appears i'th in SOS */

/* # of MCUs across the image */

/* # of MCU rows in the image */

/* # of DCT blocks per MCU */

/* MCU_membership[i] is index in cur_comp_info of component owning */

/* i'th block in an MCU */

/* progressive JPEG parameters for scan */

/*
 * Links to compression subobjects (methods and private variables of modules)
 */
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
pub type JSAMPIMAGE = *mut JSAMPARRAY;
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
pub const JPOOL_PERMANENT: c_int = 0i32;
pub const NUM_HUFF_TBLS: c_int = 4i32;
pub const NUM_QUANT_TBLS: c_int = 4i32;
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
pub const JPEG_HEADER_TABLES_ONLY: c_int = 2i32;
pub const JPEG_HEADER_OK: c_int = 1i32;
pub type C2RustUnnamed_1 = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub i: [c_int; 8],
    pub s: [c_char; 80],
}
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
pub type JBLOCK = [JCOEF; 64];
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; 64],
    pub sent_table: boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17],
    pub huffval: [UINT8; 256],
    pub sent_table: boolean,
}
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: UINT8,
    pub original_length: c_uint,
    pub data_length: c_uint,
    pub data: *mut JOCTET,
}
pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;
pub type J_COLOR_SPACE = c_uint;
pub type J_DCT_METHOD = c_uint;
pub type J_BOOLEAN_PARAM = c_uint;
pub type J_FLOAT_PARAM = c_uint;
pub type J_INT_PARAM = c_uint;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub pass_counter: c_long,
    pub pass_limit: c_long,
    pub completed_passes: c_int,
    pub total_passes: c_int,
}
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub emit_message: Option<unsafe extern "C" fn(_: j_common_ptr, _: c_int) -> ()>,
    pub output_message: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub format_message: Option<unsafe extern "C" fn(_: j_common_ptr, _: *mut c_char) -> ()>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub msg_code: c_int,
    pub msg_parm: C2RustUnnamed_2,
    pub trace_level: c_int,
    pub num_warnings: c_long,
    pub jpeg_message_table: *const *const c_char,
    pub last_jpeg_message: c_int,
    pub addon_message_table: *const *const c_char,
    pub first_addon_message: c_int,
    pub last_addon_message: c_int,
}
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(_: j_compress_ptr) -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
}
pub type j_compress_ptr = *mut jpeg_compress_struct;
/* may be overridden in jconfig.h */

/* may be overridden in jconfig.h */

/* Dithering options for decompression. */
pub type J_DITHER_MODE = c_uint;
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
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
/* Routine signature for application-supplied marker processing methods.
 * Need not pass marker code since it is stored in cinfo->unread_marker.
 */
pub type jpeg_marker_parser_method = Option<unsafe extern "C" fn(_: j_decompress_ptr) -> boolean>;
pub const JCS_RGB565: J_COLOR_SPACE = 16;
pub const JCS_EXT_ARGB: J_COLOR_SPACE = 15;
pub const JCS_EXT_ABGR: J_COLOR_SPACE = 14;
pub const JCS_EXT_BGRA: J_COLOR_SPACE = 13;
pub const JCS_EXT_RGBA: J_COLOR_SPACE = 12;
pub const JCS_EXT_XRGB: J_COLOR_SPACE = 11;
pub const JCS_EXT_XBGR: J_COLOR_SPACE = 10;
pub const JCS_EXT_BGRX: J_COLOR_SPACE = 9;
pub const JCS_EXT_BGR: J_COLOR_SPACE = 8;
pub const JCS_EXT_RGBX: J_COLOR_SPACE = 7;
pub const JCS_EXT_RGB: J_COLOR_SPACE = 6;
pub const JCS_YCCK: J_COLOR_SPACE = 5;
pub const JCS_CMYK: J_COLOR_SPACE = 4;
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
pub const JCS_RGB: J_COLOR_SPACE = 2;
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
pub const JDCT_IFAST: J_DCT_METHOD = 1;
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
pub const JBOOLEAN_OVERSHOOT_DERINGING: J_BOOLEAN_PARAM = 1061927929;
pub const JBOOLEAN_TRELLIS_Q_OPT: J_BOOLEAN_PARAM = 3777684073;
pub const JBOOLEAN_USE_SCANS_IN_TRELLIS: J_BOOLEAN_PARAM = 4253291573;
pub const JBOOLEAN_USE_LAMBDA_WEIGHT_TBL: J_BOOLEAN_PARAM = 865973855;
pub const JBOOLEAN_TRELLIS_EOB_OPT: J_BOOLEAN_PARAM = 3623303040;
pub const JBOOLEAN_TRELLIS_QUANT_DC: J_BOOLEAN_PARAM = 865946636;
pub const JBOOLEAN_TRELLIS_QUANT: J_BOOLEAN_PARAM = 3306299443;
pub const JBOOLEAN_OPTIMIZE_SCANS: J_BOOLEAN_PARAM = 1745618462;
pub const JFLOAT_TRELLIS_DELTA_DC_WEIGHT: J_FLOAT_PARAM = 326587475;
pub const JFLOAT_LAMBDA_LOG_SCALE2: J_FLOAT_PARAM = 3116084739;
pub const JFLOAT_LAMBDA_LOG_SCALE1: J_FLOAT_PARAM = 1533126041;
pub const JINT_DC_SCAN_OPT_MODE: J_INT_PARAM = 199732540;
pub const JINT_BASE_QUANT_TBL_IDX: J_INT_PARAM = 1145645745;
pub const JINT_TRELLIS_NUM_LOOPS: J_INT_PARAM = 3057565497;
pub const JINT_TRELLIS_FREQ_SPLIT: J_INT_PARAM = 1873801511;
pub const JINT_COMPRESS_PROFILE: J_INT_PARAM = 3918628389;
pub const JCP_FASTEST: C2RustUnnamed_1 = 720002228;
pub const JCP_MAX_COMPRESSION: C2RustUnnamed_1 = 1560820397;
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

/* Data structures for images (arrays of samples and of DCT coefficients).
 */

/* ptr to one image row of pixel samples. */

/* ptr to some rows (a 2-D sample array) */

/* a 3-D sample array: top index is color */

/* one block of coefficients */

/* pointer to one row of coefficient blocks */

/* a 2-D array of coefficient blocks */

/* a 3-D array of coefficient blocks */

/* useful in a couple of places */

/* Types for JPEG compression parameters and working tables. */

/* DCT coefficient quantization tables. */

/* This array gives the coefficient quantizers in natural array order
 * (not the zigzag order in which they are stored in a JPEG DQT marker).
 * CAUTION: IJG versions prior to v6a kept this array in zigzag order.
 */

/* quantization step for each coefficient */

/* This field is used only during compression.  It's initialized FALSE when
 * the table is created, and set TRUE when it's been output to the file.
 * You could suppress output of a table by setting this to TRUE.
 * (See jpeg_suppress_tables for an example.)
 */

/* TRUE when table has been output */

/* Huffman coding tables. */

/* These two fields directly represent the contents of a JPEG DHT marker */

/* bits[k] = # of symbols with codes of */

/* length k bits; bits[0] is unused */

/* The symbols, in order of incr code length */

/* This field is used only during compression.  It's initialized FALSE when
 * the table is created, and set TRUE when it's been output to the file.
 * You could suppress output of a table by setting this to TRUE.
 * (See jpeg_suppress_tables for an example.)
 */

/* TRUE when table has been output */

/* Basic info about one component (color channel). */

/* These values are fixed over the whole image. */

/* For compression, they must be supplied by parameter setup; */

/* for decompression, they are read from the SOF marker. */

/* identifier for this component (0..255) */

/* its index in SOF or cinfo->comp_info[] */

/* horizontal sampling factor (1..4) */

/* vertical sampling factor (1..4) */

/* quantization table selector (0..3) */

/* These values may vary between scans. */

/* For compression, they must be supplied by parameter setup; */

/* for decompression, they are read from the SOS marker. */

/* The decompressor output side may not use these variables. */

/* DC entropy table selector (0..3) */

/* AC entropy table selector (0..3) */

/* Remaining fields should be treated as private by applications. */

/* These values are computed during compression or decompression startup: */

/* Component's size in DCT blocks.
 * Any dummy blocks added to complete an MCU are not counted; therefore
 * these values do not depend on whether a scan is interleaved or not.
 */

/* Size of a DCT block in samples.  Always DCTSIZE for compression.
 * For decompression this is the size of the output from one DCT block,
 * reflecting any scaling we choose to apply during the IDCT step.
 * Values from 1 to 16 are supported.
 * Note that different components may receive different IDCT scalings.
 */

/* The downsampled dimensions are the component's actual, unpadded number
 * of samples at the main buffer (preprocessing/compression interface), thus
 * downsampled_width = ceil(image_width * Hi/Hmax)
 * and similarly for height.  For decompression, IDCT scaling is included, so
 * downsampled_width = ceil(image_width * Hi/Hmax * DCT_[h_]scaled_size/DCTSIZE)
 */

/* actual width in samples */

/* actual height in samples */

/* This flag is used only for decompression.  In cases where some of the
 * components will be ignored (eg grayscale output from YCbCr image),
 * we can skip most computations for the unused components.
 */

/* do we need the value of this component? */

/* These values are computed before starting a scan of the component. */

/* The decompressor output side may not use these variables. */

/* number of blocks per MCU, horizontally */

/* number of blocks per MCU, vertically */

/* MCU_width * MCU_height */

/* MCU width in samples, MCU_width*DCT_[h_]scaled_size */

/* # of non-dummy blocks across in last MCU */

/* # of non-dummy blocks down in last MCU */

/* Saved quantization table for component; NULL if none yet saved.
 * See jdinput.c comments about the need for this information.
 * This field is currently used only for decompression.
 */

/* Private per-component storage for DCT or IDCT subsystem. */

/* The script for encoding a multiple-scan file is an array of these: */

/* number of components encoded in this scan */

/* their SOF/comp_info[] indexes */

/* progressive JPEG spectral selection parms */

/* progressive JPEG successive approx. parms */

/* The decompressor can save APPn and COM markers in a list of these: */

/* next in list, or NULL */

/* marker code: JPEG_COM, or JPEG_APP0+n */

/* # bytes of data in the file */

/* # bytes of data saved at data[] */

/* the data contained in the marker */

/* the marker length word is not counted in data_length or original_length */

/* Known color spaces. */

/* error/unspecified */

/* monochrome */

/* red/green/blue as specified by the RGB_RED,
RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE macros */

/* Y/Cb/Cr (also known as YUV) */

/* C/M/Y/K */

/* Y/Cb/Cr/K */

/* red/green/blue */

/* red/green/blue/x */

/* blue/green/red */

/* blue/green/red/x */

/* x/blue/green/red */

/* x/red/green/blue */

/* When out_color_space it set to JCS_EXT_RGBX, JCS_EXT_BGRX, JCS_EXT_XBGR,
or JCS_EXT_XRGB during decompression, the X byte is undefined, and in
order to ensure the best performance, libjpeg-turbo can set that byte to
whatever value it wishes.  Use the following colorspace constants to
ensure that the X byte is set to 0xFF, so that it can be interpreted as an
opaque alpha channel. */

/* red/green/blue/alpha */

/* blue/green/red/alpha */

/* alpha/blue/green/red */

/* alpha/red/green/blue */

/* 5-bit red/6-bit green/5-bit blue */

/* DCT/IDCT algorithm options. */

/* slow but accurate integer algorithm */

/* faster, less accurate integer method */

/* floating-point: accurate, fast on fast HW */

/* may be overridden in jconfig.h */

/* may be overridden in jconfig.h */

/* Dithering options for decompression. */

/* no dithering */

/* simple ordered dither */

/* Floyd-Steinberg error diffusion dither */

/* These 32-bit GUIDs and the corresponding jpeg_*_get_*_param()/
 * jpeg_*_set_*_param() functions allow for extending the libjpeg API without
 * breaking backward ABI compatibility.  The actual parameters are stored in
 * the opaque jpeg_comp_master and jpeg_decomp_master structs.
 */

/* Boolean extension parameters */

/* TRUE=optimize progressive coding scans */

/* TRUE=use trellis quantization */

/* TRUE=use trellis quant for DC coefficient */

/* TRUE=optimize for sequences of EOB */

/* TRUE=use lambda weighting table */

/* TRUE=use scans in trellis optimization */

/* TRUE=optimize quant table in trellis loop */

/* TRUE=preprocess input to reduce ringing of edges on white background */

/* Floating point parameters */

/* Integer parameters */

/* compression profile */

/* splitting point for frequency in trellis quantization */

/* number of trellis loops */

/* base quantization table index */

/* DC scan optimization mode */

/* Values for the JINT_COMPRESS_PROFILE parameter (32-bit GUIDs) */

/* best compression ratio (progressive, all mozjpeg extensions) */

/* libjpeg[-turbo] defaults (baseline, no mozjpeg extensions) */

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

/* Fields common to both master struct types */

/* Additional fields follow in an actual jpeg_compress_struct or
 * jpeg_decompress_struct.  All three structs must agree on these
 * initial fields!  (This would be a lot cleaner in C++.)
 */

/* Master record for a compression instance */

/* Fields shared with jpeg_decompress_struct */

/* Destination for compressed data */

/* Description of source image --- these fields must be filled in by
 * outer application before starting compression.  in_color_space must
 * be correct before you can even call jpeg_set_defaults().
 */

/* input image width */

/* input image height */

/* # of color components in input image */

/* colorspace of input image */

/* image gamma of input image */

/* Compression parameters --- these fields must be set before calling
 * jpeg_start_compress().  We recommend calling jpeg_set_defaults() to
 * initialize everything to reasonable defaults, then changing anything
 * the application specifically wants to change.  That way you won't get
 * burnt when new parameters are added.  Also note that there are several
 * helper routines to simplify changing parameters.
 */

/* bits of precision in image data */

/* # of color components in JPEG image */

/* colorspace of JPEG image */

/* comp_info[i] describes component that appears i'th in SOF */

/* ptrs to coefficient quantization tables, or NULL if not defined,
 * and corresponding scale factors (percentage, initialized 100).
 */

/* ptrs to Huffman coding tables, or NULL if not defined */

/* L values for DC arith-coding tables */

/* U values for DC arith-coding tables */

/* Kx values for AC arith-coding tables */

/* # of entries in scan_info array */

/* script for multi-scan file, or NULL */

/* The default value of scan_info is NULL, which causes a single-scan
 * sequential JPEG file to be emitted.  To create a multi-scan file,
 * set num_scans and scan_info to point to an array of scan definitions.
 */

/* TRUE=caller supplies downsampled data */

/* TRUE=arithmetic coding, FALSE=Huffman */

/* TRUE=optimize entropy encoding parms */

/* TRUE=first samples are cosited */

/* 1..100, or 0 for no input smoothing */

/* DCT algorithm selector */

/* The restart interval can be specified in absolute MCUs by setting
 * restart_interval, or in MCU rows by setting restart_in_rows
 * (in which case the correct restart_interval will be figured
 * for each scan).
 */

/* MCUs per restart, or 0 for no restart */

/* if > 0, MCU rows per restart interval */

/* Parameters controlling emission of special markers. */

/* should a JFIF marker be written? */

/* What to write for the JFIF version number */

/* These three values are not used by the JPEG code, merely copied */

/* into the JFIF APP0 marker.  density_unit can be 0 for unknown, */

/* 1 for dots/inch, or 2 for dots/cm.  Note that the pixel aspect */

/* ratio is defined by X_density/Y_density even when density_unit=0. */

/* JFIF code for pixel size units */

/* Horizontal pixel density */

/* Vertical pixel density */

/* should an Adobe marker be written? */

/* State variable: index of next scanline to be written to
 * jpeg_write_scanlines().  Application may use this to control its
 * processing loop, e.g., "while (next_scanline < image_height)".
 */

/* 0 .. image_height-1  */

/* Remaining fields are known throughout compressor, but generally
 * should not be touched by a surrounding application.
 */

/*
 * These fields are computed during compression startup
 */

/* TRUE if scan script uses progressive mode */

/* largest h_samp_factor */

/* largest v_samp_factor */

/* # of iMCU rows to be input to coef ctlr */

/* The coefficient controller receives data in units of MCU rows as defined
 * for fully interleaved scans (whether the JPEG file is interleaved or not).
 * There are v_samp_factor * DCTSIZE sample rows of each component in an
 * "iMCU" (interleaved MCU) row.
 */

/*
 * These fields are valid during any one scan.
 * They describe the components and MCUs actually appearing in the scan.
 */

/* # of JPEG components in this scan */

/* *cur_comp_info[i] describes component that appears i'th in SOS */

/* # of MCUs across the image */

/* # of MCU rows in the image */

/* # of DCT blocks per MCU */

/* MCU_membership[i] is index in cur_comp_info of component owning */

/* i'th block in an MCU */

/* progressive JPEG parameters for scan */

/*
 * Links to compression subobjects (methods and private variables of modules)
 */

/* workspace for jpeg_simple_progression */

/* Master record for a decompression instance */

/* Fields shared with jpeg_compress_struct */

/* Source of compressed data */

/* Basic description of image --- filled in by jpeg_read_header(). */

/* Application may inspect these values to decide how to process image. */

/* nominal image width (from SOF marker) */

/* nominal image height */

/* # of color components in JPEG image */

/* colorspace of JPEG image */

/* Decompression processing parameters --- these fields must be set before
 * calling jpeg_start_decompress().  Note that jpeg_read_header() initializes
 * them to default values.
 */

/* colorspace for output */

/* fraction by which to scale image */

/* image gamma wanted in output */

/* TRUE=multiple output passes */

/* TRUE=downsampled data wanted */

/* IDCT algorithm selector */

/* TRUE=apply fancy upsampling */

/* TRUE=apply interblock smoothing */

/* TRUE=colormapped output wanted */

/* the following are ignored if not quantize_colors: */

/* type of color dithering to use */

/* TRUE=use two-pass color quantization */

/* max # colors to use in created colormap */

/* these are significant only in buffered-image mode: */

/* enable future use of 1-pass quantizer */

/* enable future use of external colormap */

/* enable future use of 2-pass quantizer */

/* Description of actual output image that will be returned to application.
 * These fields are computed by jpeg_start_decompress().
 * You can also use jpeg_calc_output_dimensions() to determine these values
 * in advance of calling jpeg_start_decompress().
 */

/* scaled image width */

/* scaled image height */

/* # of color components in out_color_space */

/* # of color components returned */

/* output_components is 1 (a colormap index) when quantizing colors;
 * otherwise it equals out_color_components.
 */

/* min recommended height of scanline buffer */

/* If the buffer passed to jpeg_read_scanlines() is less than this many rows
 * high, space and time will be wasted due to unnecessary data copying.
 * Usually rec_outbuf_height will be 1 or 2, at most 4.
 */

/* When quantizing colors, the output colormap is described by these fields.
 * The application can supply a colormap by setting colormap non-NULL before
 * calling jpeg_start_decompress; otherwise a colormap is created during
 * jpeg_start_decompress or jpeg_start_output.
 * The map has out_color_components rows and actual_number_of_colors columns.
 */

/* number of entries in use */

/* The color map as a 2-D pixel array */

/* State variables: these variables indicate the progress of decompression.
 * The application may examine these but must not modify them.
 */

/* Row index of next scanline to be read from jpeg_read_scanlines().
 * Application may use this to control its processing loop, e.g.,
 * "while (output_scanline < output_height)".
 */

/* 0 .. output_height-1  */

/* Current input scan number and number of iMCU rows completed in scan.
 * These indicate the progress of the decompressor input side.
 */

/* Number of SOS markers seen so far */

/* Number of iMCU rows completed */

/* The "output scan number" is the notional scan being displayed by the
 * output side.  The decompressor will not allow output scan/row number
 * to get ahead of input scan/row, but it can fall arbitrarily far behind.
 */

/* Nominal scan number being displayed */

/* Number of iMCU rows read */

/* Current progression status.  coef_bits[c][i] indicates the precision
 * with which component c's DCT coefficient i (in zigzag order) is known.
 * It is -1 when no data has yet been received, otherwise it is the point
 * transform (shift) value for the most recent scan of the coefficient
 * (thus, 0 at completion of the progression).
 * This pointer is NULL when reading a non-progressive file.
 */

/* -1 or current Al value for each coef */

/* Internal JPEG parameters --- the application usually need not look at
 * these fields.  Note that the decompressor output side may not use
 * any parameters that can change between scans.
 */

/* Quantization and Huffman tables are carried forward across input
 * datastreams when processing abbreviated JPEG datastreams.
 */

/* ptrs to coefficient quantization tables, or NULL if not defined */

/* ptrs to Huffman coding tables, or NULL if not defined */

/* These parameters are never carried across datastreams, since they
 * are given in SOF/SOS markers or defined to be reset by SOI.
 */

/* bits of precision in image data */

/* comp_info[i] describes component that appears i'th in SOF */

/* TRUE if SOFn specifies progressive mode */

/* TRUE=arithmetic coding, FALSE=Huffman */

/* L values for DC arith-coding tables */

/* U values for DC arith-coding tables */

/* Kx values for AC arith-coding tables */

/* MCUs per restart interval, or 0 for no restart */

/* These fields record data obtained from optional markers recognized by
 * the JPEG library.
 */

/* TRUE iff a JFIF APP0 marker was found */

/* Data copied from JFIF marker; only valid if saw_JFIF_marker is TRUE: */

/* JFIF version number */

/* JFIF code for pixel size units */

/* Horizontal pixel density */

/* Vertical pixel density */

/* TRUE iff an Adobe APP14 marker was found */

/* Color transform code from Adobe marker */

/* TRUE=first samples are cosited */

/* Aside from the specific data retained from APPn markers known to the
 * library, the uninterpreted contents of any or all APPn and COM markers
 * can be saved in a list for examination by the application.
 */

/* Head of list of saved markers */

/* Remaining fields are known throughout decompressor, but generally
 * should not be touched by a surrounding application.
 */

/*
 * These fields are computed during decompression startup
 */

/* largest h_samp_factor */

/* largest v_samp_factor */

/* smallest DCT_scaled_size of any component */

/* # of iMCU rows in image */

/* The coefficient controller's input and output progress is measured in
 * units of "iMCU" (interleaved MCU) rows.  These are the same as MCU rows
 * in fully interleaved JPEG scans, but are used whether the scan is
 * interleaved or not.  We define an iMCU row as v_samp_factor DCT block
 * rows of each component.  Therefore, the IDCT output contains
 * v_samp_factor*DCT_[v_]scaled_size sample rows of a component per iMCU row.
 */

/* table for fast range-limiting */

/*
 * These fields are valid during any one scan.
 * They describe the components and MCUs actually appearing in the scan.
 * Note that the decompressor output side must not use these fields.
 */

/* # of JPEG components in this scan */

/* *cur_comp_info[i] describes component that appears i'th in SOS */

/* # of MCUs across the image */

/* # of MCU rows in the image */

/* # of DCT blocks per MCU */

/* MCU_membership[i] is index in cur_comp_info of component owning */

/* i'th block in an MCU */

/* progressive JPEG parameters for scan */

/* This field is shared between entropy decoder and marker parser.
 * It is either zero or the code of a JPEG marker that has been
 * read from the data source, but has not yet been processed.
 */

/*
 * Links to decompression subobjects (methods, private variables of modules)
 */

/* "Object" declarations for JPEG modules that may be supplied or called
 * directly by the surrounding application.
 * As with all objects in the JPEG library, these structs only define the
 * publicly visible methods and state variables of a module.  Additional
 * private fields may exist after the public ones.
 */

/* Error handler object */

/* Error exit handler: does not return to caller */

/* Conditionally emit a trace or warning message */

/* Routine that actually outputs a trace or error message */

/* Format a message string for the most recent JPEG error or message */

/* recommended size of format_message buffer */

/* Reset error state variables at start of a new image */

/* The message ID code and any parameters are saved here.
 * A message can have one string parameter or up to 8 int parameters.
 */

/* Standard state variables for error facility */

/* max msg_level that will be displayed */

/* For recoverable corrupt-data errors, we emit a warning message,
 * but keep going unless emit_message chooses to abort.  emit_message
 * should count warnings in num_warnings.  The surrounding application
 * can check for bad data by seeing if num_warnings is nonzero at the
 * end of processing.
 */

/* number of corrupt-data warnings */

/* These fields point to the table(s) of error message strings.
 * An application can change the table pointer to switch to a different
 * message list (typically, to change the language in which errors are
 * reported).  Some applications may wish to add additional error codes
 * that will be handled by the JPEG library error mechanism; the second
 * table pointer is used for this purpose.
 *
 * First table includes all errors generated by JPEG library itself.
 * Error code 0 is reserved for a "no such error string" message.
 */

/* Library errors */

/* Table contains strings 0..last_jpeg_message */

/* Second table can be added by application (see cjpeg/djpeg for example).
 * It contains strings numbered first_addon_message..last_addon_message.
 */

/* Non-library errors */

/* code for first string in addon table */

/* code for last string in addon table */

/* Progress monitor object */

/* work units completed in this pass */

/* total number of work units in this pass */

/* passes completed so far */

/* total number of passes expected */

/* Data destination object for compression */

/* => next byte to write in buffer */

/* # of byte spaces remaining in buffer */

/* Data source object for decompression */

/* => next byte to read from buffer */

/* # of bytes remaining in buffer */

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

/* Found valid table-specs-only datastream */

/* If you pass require_image = TRUE (normal case), you need not check for
 * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
 * JPEG_SUSPENDED is only possible if you use a data source module that can
 * give a suspension return (the stdio source module doesn't).
 */

/* Main entry points for decompression */

/* Replaces jpeg_read_scanlines when reading raw downsampled data. */

/* Additional entry points for buffered-image mode. */

/* Return value is one of: */

/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */

/* Reached start of new scan */

/* Reached end of image */

/* Completed one iMCU row */

/* Completed last iMCU row of a scan */

/* Precalculate output dimensions for current decompression parameters. */

/* Control saving of COM and APPn markers into marker_list. */

/* Install a special processing method for COM or APPn markers. */

/* Read or write raw DCT coefficients --- useful for lossless transcoding. */

/* If you choose to abort compression or decompression before completing
 * jpeg_finish_(de)compress, then you need to clean up to release memory,
 * temporary files, etc.  You can just call jpeg_destroy_(de)compress
 * if you're done with the JPEG object, but if you want to clean it up and
 * reuse it, call this:
 */

/* Generic versions of jpeg_abort and jpeg_destroy that work on either
 * flavor of JPEG object.  These may be more convenient in some places.
 */

/* Default restart-marker-resync procedure for use by data source modules */

/* Accessor functions for extension parameters */

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
pub const JPEG_APP0: c_int = 0xe0i32;
pub const JDCT_DEFAULT: c_int = JDCT_ISLOW as c_int;
/* Floyd-Steinberg error diffusion dither */

/* simple ordered dither */
pub const JDITHER_FS: J_DITHER_MODE = 2;
/* no dithering */
pub const JDITHER_ORDERED: J_DITHER_MODE = 1;
pub const JDITHER_NONE: J_DITHER_MODE = 0;
/* These marker codes are exported since applications and data source modules
 * are likely to want to use them.
 */

/* RST0 marker code */

/* EOI marker code */

/* APP0 marker code */
pub const JPEG_COM: c_int = 0xfei32;
pub const JDCT_FASTEST: c_int = JDCT_IFAST as c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvirt_barray_control {
    pub mem_buffer: JBLOCKARRAY,
    pub rows_in_array: JDIMENSION,
    pub blocksperrow: JDIMENSION,
    pub maxaccess: JDIMENSION,
    pub rows_in_mem: JDIMENSION,
    pub rowsperchunk: JDIMENSION,
    pub cur_start_row: JDIMENSION,
    pub first_undef_row: JDIMENSION,
    pub pre_zero: boolean,
    pub dirty: boolean,
    pub b_s_open: boolean,
    pub next: jvirt_barray_ptr,
    pub b_s_info: backing_store_info,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvirt_sarray_control {
    pub mem_buffer: JSAMPARRAY,
    pub rows_in_array: JDIMENSION,
    pub samplesperrow: JDIMENSION,
    pub maxaccess: JDIMENSION,
    pub rows_in_mem: JDIMENSION,
    pub rowsperchunk: JDIMENSION,
    pub cur_start_row: JDIMENSION,
    pub first_undef_row: JDIMENSION,
    pub pre_zero: boolean,
    pub dirty: boolean,
    pub b_s_open: boolean,
    pub next: jvirt_sarray_ptr,
    pub b_s_info: backing_store_info,
}
use crate::jmemsys_h::backing_store_info;
pub const JMSG_LENGTH_MAX: c_int = 200i32;
