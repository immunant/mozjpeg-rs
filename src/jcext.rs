
































































































































































































































use libc::{c_int, c_float, self};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::stddef_h::size_t;pub use crate::jpegint_h::{JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
                           JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE};pub use crate::jpeglib_h::{j_common_ptr, j_compress_ptr,
                           jpeg_c_coef_controller, jpeg_c_main_controller,
                           jpeg_c_prep_controller, jpeg_color_converter,
                           jpeg_common_struct, jpeg_comp_master,
                           jpeg_component_info, jpeg_compress_struct,
                           jpeg_destination_mgr, jpeg_downsampler,
                           jpeg_entropy_encoder, jpeg_error_mgr,
                           jpeg_forward_dct, jpeg_marker_writer,
                           jpeg_memory_mgr, jpeg_progress_mgr, jpeg_scan_info,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_1, C2RustUnnamed_2, JCS_YCbCr,
                           JBLOCK, JBLOCKARRAY, JBLOCKROW,
                           JBOOLEAN_OPTIMIZE_SCANS,
                           JBOOLEAN_OVERSHOOT_DERINGING,
                           JBOOLEAN_TRELLIS_EOB_OPT, JBOOLEAN_TRELLIS_QUANT,
                           JBOOLEAN_TRELLIS_QUANT_DC, JBOOLEAN_TRELLIS_Q_OPT,
                           JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                           JBOOLEAN_USE_SCANS_IN_TRELLIS, JCP_FASTEST,
                           JCP_MAX_COMPRESSION, JCS_CMYK, JCS_EXT_ABGR,
                           JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
                           JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
                           JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
                           JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
                           JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
                           JFLOAT_LAMBDA_LOG_SCALE1, JFLOAT_LAMBDA_LOG_SCALE2,
                           JFLOAT_TRELLIS_DELTA_DC_WEIGHT, JHUFF_TBL,
                           JINT_BASE_QUANT_TBL_IDX, JINT_COMPRESS_PROFILE,
                           JINT_DC_SCAN_OPT_MODE, JINT_TRELLIS_FREQ_SPLIT,
                           JINT_TRELLIS_NUM_LOOPS, JQUANT_TBL, JSAMPARRAY,
                           JSAMPIMAGE, JSAMPROW, J_BOOLEAN_PARAM,
                           J_COLOR_SPACE, J_DCT_METHOD, J_FLOAT_PARAM,
                           J_INT_PARAM};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
                        JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
                        JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID,
                        JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
                        JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE,
                        JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
                        JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION,
                        JERR_BAD_MCU_SIZE, JERR_BAD_PARAM,
                        JERR_BAD_PARAM_VALUE, JERR_BAD_POOL_ID,
                        JERR_BAD_PRECISION, JERR_BAD_PROGRESSION,
                        JERR_BAD_PROG_SCRIPT, JERR_BAD_SAMPLING,
                        JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE,
                        JERR_BAD_STRUCT_SIZE, JERR_BAD_VIRTUAL_ACCESS,
                        JERR_BUFFER_SIZE, JERR_CANT_SUSPEND,
                        JERR_CCIR601_NOTIMPL, JERR_COMPONENT_COUNT,
                        JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX,
                        JERR_DAC_VALUE, JERR_DHT_INDEX, JERR_DQT_INDEX,
                        JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE,
                        JERR_EOI_EXPECTED, JERR_FILE_READ, JERR_FILE_WRITE,
                        JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
                        JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG,
                        JERR_INPUT_EMPTY, JERR_INPUT_EOF,
                        JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA,
                        JERR_MODE_CHANGE, JERR_NOTIMPL, JERR_NOT_COMPILED,
                        JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                        JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                        JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                        JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                        JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                        JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                        JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
                        JERR_TFILE_SEEK, JERR_TFILE_WRITE,
                        JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
                        JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG,
                        JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
                        JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE,
                        JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
                        JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                        JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN, JTRC_EOI,
                        JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE,
                        JTRC_JFIF_EXTENSION, JTRC_JFIF_THUMBNAIL,
                        JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER,
                        JTRC_QUANTVALS, JTRC_QUANT_3_NCOLORS,
                        JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED,
                        JTRC_RECOVERY_ACTION, JTRC_RST, JTRC_SMOOTH_NOTIMPL,
                        JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                        JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE,
                        JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                        JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                        JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                        JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                        JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR,
                        JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
                        JWRN_TOO_MUCH_DATA};
/*
 * jcext.c
 *
 * Copyright (C) 2014, D. R. Commander.
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains accessor functions for extension parameters.  These
 * allow for extending the functionality of the libjpeg API without breaking
 * backward ABI compatibility.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_bool_param_supported(
    cinfo: j_compress_ptr,
    mut param: J_BOOLEAN_PARAM,
) -> boolean {
    match  param {
        1745618462 | 3306299443 | 865946636 | 3623303040 | 865973855 | 4253291573 | 3777684073
        | 1061927929 => return TRUE,
        _ => {}
    }
    return FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_set_bool_param(
    mut cinfo: j_compress_ptr,
    mut param: J_BOOLEAN_PARAM,
    mut value: boolean,
) {
    match  param {
        1745618462 => (*(*cinfo).master).optimize_scans = value,
        3306299443 => (*(*cinfo).master).trellis_quant = value,
        865946636 => (*(*cinfo).master).trellis_quant_dc = value,
        3623303040 => (*(*cinfo).master).trellis_eob_opt = value,
        865973855 => (*(*cinfo).master).use_lambda_weight_tbl = value,
        4253291573 => (*(*cinfo).master).use_scans_in_trellis = value,
        3777684073 => (*(*cinfo).master).trellis_q_opt = value,
        1061927929 => (*(*cinfo).master).overshoot_deringing = value,
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PARAM as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_get_bool_param(
    cinfo: j_compress_ptr,
    mut param: J_BOOLEAN_PARAM,
) -> boolean {
    match  param {
        1745618462 => return (*(*cinfo).master).optimize_scans,
        3306299443 => return (*(*cinfo).master).trellis_quant,
        865946636 => return (*(*cinfo).master).trellis_quant_dc,
        3623303040 => return (*(*cinfo).master).trellis_eob_opt,
        865973855 => return (*(*cinfo).master).use_lambda_weight_tbl,
        4253291573 => return (*(*cinfo).master).use_scans_in_trellis,
        3777684073 => return (*(*cinfo).master).trellis_q_opt,
        1061927929 => return (*(*cinfo).master).overshoot_deringing,
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PARAM as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    }
    return FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_float_param_supported(
    cinfo: j_compress_ptr,
    mut param: J_FLOAT_PARAM,
) -> boolean {
    match  param {
        1533126041 | 3116084739 | 326587475 => return TRUE,
        _ => {}
    }
    return FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_set_float_param(
    mut cinfo: j_compress_ptr,
    mut param: J_FLOAT_PARAM,
    mut value: c_float,
) {
    match  param {
        1533126041 => (*(*cinfo).master).lambda_log_scale1 = value,
        3116084739 => (*(*cinfo).master).lambda_log_scale2 = value,
        326587475 => (*(*cinfo).master).trellis_delta_dc_weight = value,
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PARAM as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_get_float_param(
    cinfo: j_compress_ptr,
    mut param: J_FLOAT_PARAM,
) -> c_float {
    match  param {
        1533126041 => return (*(*cinfo).master).lambda_log_scale1,
        3116084739 => return (*(*cinfo).master).lambda_log_scale2,
        326587475 => return (*(*cinfo).master).trellis_delta_dc_weight,
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PARAM as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    }
    return -1f32;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_int_param_supported(
    cinfo: j_compress_ptr,
    mut param: J_INT_PARAM,
) -> boolean {
    match  param {
        3918628389 | 1873801511 | 3057565497 | 1145645745 | 199732540 => {
            return TRUE
        }
        _ => {}
    }
    return FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_set_int_param(
    mut cinfo: j_compress_ptr,
    mut param: J_INT_PARAM,
    mut value: c_int,
) {
    match  param {
        3918628389 => match value {
            1560820397 | 720002228 => (*(*cinfo).master).compress_profile = value,
            _ => {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PARAM_VALUE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
        },
        1873801511 => (*(*cinfo).master).trellis_freq_split = value,
        3057565497 => (*(*cinfo).master).trellis_num_loops = value,
        1145645745 => {
            if value >= 0i32 && value <= 8i32 {
                (*(*cinfo).master).quant_tbl_master_idx = value
            }
        }
        199732540 => (*(*cinfo).master).dc_scan_opt_mode = value,
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PARAM as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
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
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_get_int_param(
    cinfo: j_compress_ptr,
    mut param: J_INT_PARAM,
) -> c_int {
    match  param {
        3918628389 => return (*(*cinfo).master).compress_profile,
        1873801511 => return (*(*cinfo).master).trellis_freq_split,
        3057565497 => return (*(*cinfo).master).trellis_num_loops,
        1145645745 => return (*(*cinfo).master).quant_tbl_master_idx,
        199732540 => return (*(*cinfo).master).dc_scan_opt_mode,
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PARAM as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    }
    return -1i32;
}
