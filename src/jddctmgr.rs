pub use crate::jdct_h::{
    jpeg_idct_10x10, jpeg_idct_11x11, jpeg_idct_12x12, jpeg_idct_13x13, jpeg_idct_14x14,
    jpeg_idct_15x15, jpeg_idct_16x16, jpeg_idct_1x1, jpeg_idct_2x2, jpeg_idct_3x3, jpeg_idct_4x4,
    jpeg_idct_5x5, jpeg_idct_6x6, jpeg_idct_7x7, jpeg_idct_9x9, jpeg_idct_float, jpeg_idct_ifast,
    jpeg_idct_islow, FLOAT_MULT_TYPE, IFAST_MULT_TYPE, ISLOW_MULT_TYPE,
};
pub use crate::jdmaster::{my_decomp_master, my_master_ptr};
pub use crate::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::jmorecfg_h::{boolean, INT16, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jpeg_color_deconverter, jpeg_color_quantizer, jpeg_d_coef_controller,
    jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master, jpeg_entropy_decoder,
    jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader, jpeg_upsampler, JBUF_CRANK_DEST,
    JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_common_struct, jpeg_component_info,
    jpeg_decompress_struct, jpeg_error_mgr, jpeg_idct_method, jpeg_idct_method_selector,
    jpeg_marker_parser_method, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE2, JBLOCK,
    JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
    JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
    JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
    JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
    JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
use crate::jsimd::{
    jsimd_can_idct_2x2, jsimd_can_idct_4x4, jsimd_can_idct_float, jsimd_can_idct_ifast,
    jsimd_can_idct_islow, jsimd_idct_2x2, jsimd_idct_4x4, jsimd_idct_float, jsimd_idct_ifast,
    jsimd_idct_islow,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::memset;
use libc::{self, c_double, c_int, c_uint, c_ulong, intptr_t};
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
