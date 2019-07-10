pub use crate::jdct_h::jpeg_idct_10x10;
pub use crate::jdct_h::jpeg_idct_11x11;
pub use crate::jdct_h::jpeg_idct_12x12;
pub use crate::jdct_h::jpeg_idct_13x13;
pub use crate::jdct_h::jpeg_idct_14x14;
pub use crate::jdct_h::jpeg_idct_15x15;
pub use crate::jdct_h::jpeg_idct_16x16;
pub use crate::jdct_h::jpeg_idct_1x1;
pub use crate::jdct_h::jpeg_idct_2x2;
pub use crate::jdct_h::jpeg_idct_3x3;
pub use crate::jdct_h::jpeg_idct_4x4;
pub use crate::jdct_h::jpeg_idct_5x5;
pub use crate::jdct_h::jpeg_idct_6x6;
pub use crate::jdct_h::jpeg_idct_7x7;
pub use crate::jdct_h::jpeg_idct_9x9;
pub use crate::jdct_h::jpeg_idct_float;
pub use crate::jdct_h::jpeg_idct_ifast;
pub use crate::jdct_h::jpeg_idct_islow;
pub use crate::jdct_h::FLOAT_MULT_TYPE;
pub use crate::jdct_h::IFAST_MULT_TYPE;
pub use crate::jdct_h::ISLOW_MULT_TYPE;
pub use crate::jdmaster::my_decomp_master;
pub use crate::jdmaster::my_master_ptr;
pub use crate::jerror::C2RustUnnamed_3;
pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
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
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_idct_method;
pub use crate::jpeglib_h::jpeg_idct_method_selector;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_EXT_ABGR;
pub use crate::jpeglib_h::JCS_EXT_ARGB;
pub use crate::jpeglib_h::JCS_EXT_BGR;
pub use crate::jpeglib_h::JCS_EXT_BGRA;
pub use crate::jpeglib_h::JCS_EXT_BGRX;
pub use crate::jpeglib_h::JCS_EXT_RGB;
pub use crate::jpeglib_h::JCS_EXT_RGBA;
pub use crate::jpeglib_h::JCS_EXT_RGBX;
pub use crate::jpeglib_h::JCS_EXT_XBGR;
pub use crate::jpeglib_h::JCS_EXT_XRGB;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_RGB565;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
use crate::jsimd::jsimd_can_idct_2x2;
use crate::jsimd::jsimd_can_idct_4x4;
use crate::jsimd::jsimd_can_idct_float;
use crate::jsimd::jsimd_can_idct_ifast;
use crate::jsimd::jsimd_can_idct_islow;
use crate::jsimd::jsimd_idct_2x2;
use crate::jsimd::jsimd_idct_4x4;
use crate::jsimd::jsimd_idct_float;
use crate::jsimd::jsimd_idct_ifast;
use crate::jsimd::jsimd_idct_islow;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memset;
use libc;
use libc::c_double;
use libc::c_int;
use libc::c_uint;
use libc::c_ulong;
use libc::intptr_t;
pub type my_idct_ptr = *mut my_idct_controller;
/*
 * jddctmgr.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * Modified 2002-2010 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2010, 2015, D. R. Commander.
 * Copyright (C) 2013, MIPS Technologies, Inc., California.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains the inverse-DCT management logic.
 * This code selects a particular IDCT implementation to be used,
 * and it performs related housekeeping chores.  No code in this file
 * is executed per IDCT step, only during output pass setup.
 *
 * Note that the IDCT routines are responsible for performing coefficient
 * dequantization as well as the IDCT proper.  This module sets up the
 * dequantization multiplier table needed by the IDCT routine.
 */
/*
 * The decompressor input side (jdinput.c) saves away the appropriate
 * quantization table for each component at the start of the first scan
 * involving that component.  (This is necessary in order to correctly
 * decode files that reuse Q-table slots.)
 * When we are ready to make an output pass, the saved Q-table is converted
 * to a multiplier table that will actually be used by the IDCT routine.
 * The multiplier table contents are IDCT-method-dependent.  To support
 * application changes in IDCT method between scans, we can remake the
 * multiplier tables if necessary.
 * In buffered-image mode, the first output pass may occur before any data
 * has been seen for some components, and thus before their Q-tables have
 * been saved away.  To handle this case, multiplier tables are preset
 * to zeroes; the result of the IDCT will be a neutral gray level.
 */
/* Private subobject for this module */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_idct_controller {
    pub pub_0: jpeg_inverse_dct,
    pub cur_method: [c_int; 10],
}
/* Allocated multiplier tables: big enough for any supported variant */
#[repr(C)]
#[derive(Copy, Clone)]
pub union multiplier_table {
    pub islow_array: [ISLOW_MULT_TYPE; 64],
    pub ifast_array: [IFAST_MULT_TYPE; 64],
    pub float_array: [FLOAT_MULT_TYPE; 64],
}
/*
 * Permit users to replace the IDCT method dynamically.
 * The selector callback is called after the default idct implementation was choosen,
 * and is able to override it.
 */
/* The current scaled-IDCT routines require ISLOW-style multiplier tables,
 * so be sure to compile that code if either ISLOW or SCALING is requested.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_set_idct_method_selector(
    mut cinfo: j_decompress_ptr,
    mut selector: jpeg_idct_method_selector,
) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    (*master).custom_idct_selector = selector;
}
/*
 * Prepare for an output pass.
 * Here we select the proper IDCT routine for each component and build
 * a matching multiplier table.
 */
unsafe extern "C" fn start_pass(mut cinfo: j_decompress_ptr) {
    let mut idct: my_idct_ptr = (*cinfo).idct as my_idct_ptr;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut method: c_int = 0i32;
    let mut method_ptr: inverse_DCT_method_ptr =
        ::std::mem::transmute::<intptr_t, inverse_DCT_method_ptr>(NULL as intptr_t);
    let mut qtbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut master: my_master_ptr = 0 as *mut my_decomp_master;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        match (*compptr).DCT_scaled_size {
            1 => {
                method_ptr = Some(
                    jpeg_idct_1x1
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            2 => {
                if 0 != jsimd_can_idct_2x2() {
                    method_ptr = Some(
                        jsimd_idct_2x2
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                } else {
                    method_ptr = Some(
                        jpeg_idct_2x2
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                }
                method = JDCT_ISLOW as c_int
            }
            3 => {
                method_ptr = Some(
                    jpeg_idct_3x3
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            4 => {
                if 0 != jsimd_can_idct_4x4() {
                    method_ptr = Some(
                        jsimd_idct_4x4
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                } else {
                    method_ptr = Some(
                        jpeg_idct_4x4
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                }
                method = JDCT_ISLOW as c_int
            }
            5 => {
                method_ptr = Some(
                    jpeg_idct_5x5
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            6 => {
                method_ptr = Some(
                    jpeg_idct_6x6
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            7 => {
                method_ptr = Some(
                    jpeg_idct_7x7
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            8 => match (*cinfo).dct_method as c_uint {
                0 => {
                    if 0 != jsimd_can_idct_islow() {
                        method_ptr = Some(
                            jsimd_idct_islow
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: *mut jpeg_component_info,
                                    _: JCOEFPTR,
                                    _: JSAMPARRAY,
                                    _: JDIMENSION,
                                ) -> (),
                        )
                    } else {
                        method_ptr = Some(
                            jpeg_idct_islow
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: *mut jpeg_component_info,
                                    _: JCOEFPTR,
                                    _: JSAMPARRAY,
                                    _: JDIMENSION,
                                ) -> (),
                        )
                    }
                    method = JDCT_ISLOW as c_int
                }
                1 => {
                    if 0 != jsimd_can_idct_ifast() {
                        method_ptr = Some(
                            jsimd_idct_ifast
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: *mut jpeg_component_info,
                                    _: JCOEFPTR,
                                    _: JSAMPARRAY,
                                    _: JDIMENSION,
                                ) -> (),
                        )
                    } else {
                        method_ptr = Some(
                            jpeg_idct_ifast
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: *mut jpeg_component_info,
                                    _: JCOEFPTR,
                                    _: JSAMPARRAY,
                                    _: JDIMENSION,
                                ) -> (),
                        )
                    }
                    method = JDCT_IFAST as c_int
                }
                2 => {
                    if 0 != jsimd_can_idct_float() {
                        method_ptr = Some(
                            jsimd_idct_float
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: *mut jpeg_component_info,
                                    _: JCOEFPTR,
                                    _: JSAMPARRAY,
                                    _: JDIMENSION,
                                ) -> (),
                        )
                    } else {
                        method_ptr = Some(
                            jpeg_idct_float
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: *mut jpeg_component_info,
                                    _: JCOEFPTR,
                                    _: JSAMPARRAY,
                                    _: JDIMENSION,
                                ) -> (),
                        )
                    }
                    method = JDCT_FLOAT as c_int
                }
                _ => {
                    (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            },
            9 => {
                method_ptr = Some(
                    jpeg_idct_9x9
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            10 => {
                method_ptr = Some(
                    jpeg_idct_10x10
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            11 => {
                method_ptr = Some(
                    jpeg_idct_11x11
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            12 => {
                method_ptr = Some(
                    jpeg_idct_12x12
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            13 => {
                method_ptr = Some(
                    jpeg_idct_13x13
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            14 => {
                method_ptr = Some(
                    jpeg_idct_14x14
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            15 => {
                method_ptr = Some(
                    jpeg_idct_15x15
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            16 => {
                method_ptr = Some(
                    jpeg_idct_16x16
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_BAD_DCTSIZE as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = (*compptr).DCT_scaled_size;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        master = (*cinfo).master as my_master_ptr;
        if (*master).custom_idct_selector.is_some() {
            (*master)
                .custom_idct_selector
                .expect("non-null function pointer")(
                cinfo, compptr, &mut method_ptr, &mut method
            );
        }
        (*idct).pub_0.inverse_DCT[ci as usize] = method_ptr;
        /* Create multiplier table from quant table.
         * However, we can skip this if the component is uninteresting
         * or if we already built the table.  Also, if no quant table
         * has yet been saved for the component, we leave the
         * multiplier table all-zero; we'll be reading zeroes from the
         * coefficient controller's buffer anyway.
         */
        if !(0 == (*compptr).component_needed || (*idct).cur_method[ci as usize] == method) {
            qtbl = (*compptr).quant_table;
            /* happens if no data yet for component */
            if !qtbl.is_null() {
                (*idct).cur_method[ci as usize] = method;
                match method {
                    0 => {
                        let mut ismtbl: *mut ISLOW_MULT_TYPE =
                            (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
                        i = 0i32;
                        while i < DCTSIZE2 {
                            *ismtbl.offset(i as isize) =
                                (*qtbl).quantval[i as usize] as ISLOW_MULT_TYPE;
                            i += 1
                        }
                    }
                    1 => {
                        let mut ifmtbl: *mut IFAST_MULT_TYPE =
                            (*compptr).dct_table as *mut IFAST_MULT_TYPE;
                        static mut aanscales: [INT16; 64] = [
                            16384i32 as INT16,
                            22725i32 as INT16,
                            21407i32 as INT16,
                            19266i32 as INT16,
                            16384i32 as INT16,
                            12873i32 as INT16,
                            8867i32 as INT16,
                            4520i32 as INT16,
                            22725i32 as INT16,
                            31521i32 as INT16,
                            29692i32 as INT16,
                            26722i32 as INT16,
                            22725i32 as INT16,
                            17855i32 as INT16,
                            12299i32 as INT16,
                            6270i32 as INT16,
                            21407i32 as INT16,
                            29692i32 as INT16,
                            27969i32 as INT16,
                            25172i32 as INT16,
                            21407i32 as INT16,
                            16819i32 as INT16,
                            11585i32 as INT16,
                            5906i32 as INT16,
                            19266i32 as INT16,
                            26722i32 as INT16,
                            25172i32 as INT16,
                            22654i32 as INT16,
                            19266i32 as INT16,
                            15137i32 as INT16,
                            10426i32 as INT16,
                            5315i32 as INT16,
                            16384i32 as INT16,
                            22725i32 as INT16,
                            21407i32 as INT16,
                            19266i32 as INT16,
                            16384i32 as INT16,
                            12873i32 as INT16,
                            8867i32 as INT16,
                            4520i32 as INT16,
                            12873i32 as INT16,
                            17855i32 as INT16,
                            16819i32 as INT16,
                            15137i32 as INT16,
                            12873i32 as INT16,
                            10114i32 as INT16,
                            6967i32 as INT16,
                            3552i32 as INT16,
                            8867i32 as INT16,
                            12299i32 as INT16,
                            11585i32 as INT16,
                            10426i32 as INT16,
                            8867i32 as INT16,
                            6967i32 as INT16,
                            4799i32 as INT16,
                            2446i32 as INT16,
                            4520i32 as INT16,
                            6270i32 as INT16,
                            5906i32 as INT16,
                            5315i32 as INT16,
                            4520i32 as INT16,
                            3552i32 as INT16,
                            2446i32 as INT16,
                            1247i32 as INT16,
                        ];
                        i = 0i32;
                        while i < DCTSIZE2 {
                            *ifmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize] as JLONG
                                * aanscales[i as usize] as JLONG
                                + ((1i32 as JLONG) << 14i32 - 2i32 - 1i32)
                                >> 14i32 - 2i32)
                                as IFAST_MULT_TYPE;
                            i += 1
                        }
                    }
                    2 => {
                        let mut fmtbl: *mut FLOAT_MULT_TYPE =
                            (*compptr).dct_table as *mut FLOAT_MULT_TYPE;
                        let mut row: c_int = 0;
                        let mut col: c_int = 0;
                        static mut aanscalefactor: [c_double; 8] = [
                            1.0f64,
                            1.387039845f64,
                            1.306562965f64,
                            1.175875602f64,
                            1.0f64,
                            0.785694958f64,
                            0.541196100f64,
                            0.275899379f64,
                        ];
                        i = 0i32;
                        row = 0i32;
                        while row < 8i32 {
                            col = 0i32;
                            while col < 8i32 {
                                *fmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize]
                                    as c_double
                                    * aanscalefactor[row as usize]
                                    * aanscalefactor[col as usize])
                                    as FLOAT_MULT_TYPE;
                                i += 1;
                                col += 1
                            }
                            row += 1
                        }
                    }
                    _ => {
                        (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as c_int;
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                    }
                }
            }
        }
        ci += 1;
        compptr = compptr.offset(1isize)
    }
}
/*
 * Initialize IDCT manager.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_inverse_dct(mut cinfo: j_decompress_ptr) {
    let mut idct: my_idct_ptr = 0 as *mut my_idct_controller;
    let mut ci: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    idct = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_idct_controller>() as c_ulong,
    ) as my_idct_ptr;
    (*cinfo).idct = idct as *mut jpeg_inverse_dct;
    (*idct).pub_0.start_pass = Some(start_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).dct_table = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ::std::mem::size_of::<multiplier_table>() as c_ulong,
        );
        memset(
            (*compptr).dct_table,
            0i32,
            ::std::mem::size_of::<multiplier_table>() as c_ulong,
        );
        (*idct).cur_method[ci as usize] = -1i32;
        ci += 1;
        compptr = compptr.offset(1isize)
    }
}
