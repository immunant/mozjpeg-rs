use ::libc;

/* JERROR_H */

/* Informational/debugging messages */

/* Nonfatal errors (we can keep going, but the data is probably corrupt) */

/* Fatal errors (print message and exit) */

/* Macros to simplify using the error and trace message stuff */

/* The first parameter is either type of cinfo pointer */

/* Zap JMESSAGE macro so that future re-inclusions do nothing by default */

/* JMAKE_ENUM_LIST */
pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JBOOLEAN_OPTIMIZE_SCANS;
pub use crate::jpeglib_h::JBOOLEAN_OVERSHOOT_DERINGING;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_EOB_OPT;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT_DC;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_Q_OPT;
pub use crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL;
pub use crate::jpeglib_h::JBOOLEAN_USE_SCANS_IN_TRELLIS;
pub use crate::jpeglib_h::JCP_FASTEST;
pub use crate::jpeglib_h::JCP_MAX_COMPRESSION;
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
pub use crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1;
pub use crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2;
pub use crate::jpeglib_h::JFLOAT_TRELLIS_DELTA_DC_WEIGHT;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX;
pub use crate::jpeglib_h::JINT_COMPRESS_PROFILE;
pub use crate::jpeglib_h::JINT_DC_SCAN_OPT_MODE;
pub use crate::jpeglib_h::JINT_TRELLIS_FREQ_SPLIT;
pub use crate::jpeglib_h::JINT_TRELLIS_NUM_LOOPS;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_BOOLEAN_PARAM;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_FLOAT_PARAM;
pub use crate::jpeglib_h::J_INT_PARAM;
pub use crate::src::jerror::JERR_ARITH_NOTIMPL;
pub use crate::src::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_LENGTH;
pub use crate::src::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jerror::JERR_BAD_PARAM;
pub use crate::src::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::src::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jerror::JERR_BAD_PRECISION;
pub use crate::src::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jerror::JERR_BAD_STATE;
pub use crate::src::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jerror::JERR_DAC_INDEX;
pub use crate::src::jerror::JERR_DAC_VALUE;
pub use crate::src::jerror::JERR_DHT_INDEX;
pub use crate::src::jerror::JERR_DQT_INDEX;
pub use crate::src::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jerror::JERR_EMS_READ;
pub use crate::src::jerror::JERR_EMS_WRITE;
pub use crate::src::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jerror::JERR_FILE_READ;
pub use crate::src::jerror::JERR_FILE_WRITE;
pub use crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jerror::JERR_INPUT_EOF;
pub use crate::src::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jerror::JERR_MISSING_DATA;
pub use crate::src::jerror::JERR_MODE_CHANGE;
pub use crate::src::jerror::JERR_NOTIMPL;
pub use crate::src::jerror::JERR_NOT_COMPILED;
pub use crate::src::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jerror::JERR_NO_IMAGE;
pub use crate::src::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jerror::JERR_NO_SOI;
pub use crate::src::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jerror::JERR_TFILE_CREATE;
pub use crate::src::jerror::JERR_TFILE_READ;
pub use crate::src::jerror::JERR_TFILE_SEEK;
pub use crate::src::jerror::JERR_TFILE_WRITE;
pub use crate::src::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::src::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jerror::JERR_XMS_READ;
pub use crate::src::jerror::JERR_XMS_WRITE;
pub use crate::src::jerror::JMSG_COPYRIGHT;
pub use crate::src::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jerror::JMSG_NOMESSAGE;
pub use crate::src::jerror::JMSG_VERSION;
pub use crate::src::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jerror::JTRC_ADOBE;
pub use crate::src::jerror::JTRC_APP0;
pub use crate::src::jerror::JTRC_APP14;
pub use crate::src::jerror::JTRC_DAC;
pub use crate::src::jerror::JTRC_DHT;
pub use crate::src::jerror::JTRC_DQT;
pub use crate::src::jerror::JTRC_DRI;
pub use crate::src::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jerror::JTRC_EMS_OPEN;
pub use crate::src::jerror::JTRC_EOI;
pub use crate::src::jerror::JTRC_HUFFBITS;
pub use crate::src::jerror::JTRC_JFIF;
pub use crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jerror::JTRC_MISC_MARKER;
pub use crate::src::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jerror::JTRC_QUANTVALS;
pub use crate::src::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jerror::JTRC_RST;
pub use crate::src::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jerror::JTRC_SOF;
pub use crate::src::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jerror::JTRC_SOI;
pub use crate::src::jerror::JTRC_SOS;
pub use crate::src::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jerror::JTRC_THUMB_RGB;
pub use crate::src::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jerror::JTRC_XMS_OPEN;
pub use crate::src::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jerror::JWRN_BOGUS_ICC;
pub use crate::src::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jerror::JWRN_HIT_MARKER;
pub use crate::src::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jerror::JWRN_JPEG_EOF;
pub use crate::src::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::stdlib::C2RustUnnamed_0;
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
    cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_BOOLEAN_PARAM,
) -> crate::jmorecfg_h::boolean {
    match param as libc::c_uint {
        1745618462 | 3306299443 | 865946636 | 3623303040 | 865973855 | 4253291573 | 3777684073
        | 1061927929 => return crate::jmorecfg_h::TRUE,
        _ => {}
    }
    return crate::jmorecfg_h::FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_set_bool_param(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_BOOLEAN_PARAM,
    mut value: crate::jmorecfg_h::boolean,
) {
    match param as libc::c_uint {
        1745618462 => (*(*cinfo).master).optimize_scans = value,
        3306299443 => (*(*cinfo).master).trellis_quant = value,
        865946636 => (*(*cinfo).master).trellis_quant_dc = value,
        3623303040 => (*(*cinfo).master).trellis_eob_opt = value,
        865973855 => (*(*cinfo).master).use_lambda_weight_tbl = value,
        4253291573 => (*(*cinfo).master).use_scans_in_trellis = value,
        3777684073 => (*(*cinfo).master).trellis_q_opt = value,
        1061927929 => (*(*cinfo).master).overshoot_deringing = value,
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PARAM as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_get_bool_param(
    cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_BOOLEAN_PARAM,
) -> crate::jmorecfg_h::boolean {
    match param as libc::c_uint {
        1745618462 => return (*(*cinfo).master).optimize_scans,
        3306299443 => return (*(*cinfo).master).trellis_quant,
        865946636 => return (*(*cinfo).master).trellis_quant_dc,
        3623303040 => return (*(*cinfo).master).trellis_eob_opt,
        865973855 => return (*(*cinfo).master).use_lambda_weight_tbl,
        4253291573 => return (*(*cinfo).master).use_scans_in_trellis,
        3777684073 => return (*(*cinfo).master).trellis_q_opt,
        1061927929 => return (*(*cinfo).master).overshoot_deringing,
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PARAM as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    return crate::jmorecfg_h::FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_float_param_supported(
    cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_FLOAT_PARAM,
) -> crate::jmorecfg_h::boolean {
    match param as libc::c_uint {
        1533126041 | 3116084739 | 326587475 => return crate::jmorecfg_h::TRUE,
        _ => {}
    }
    return crate::jmorecfg_h::FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_set_float_param(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_FLOAT_PARAM,
    mut value: libc::c_float,
) {
    match param as libc::c_uint {
        1533126041 => (*(*cinfo).master).lambda_log_scale1 = value,
        3116084739 => (*(*cinfo).master).lambda_log_scale2 = value,
        326587475 => (*(*cinfo).master).trellis_delta_dc_weight = value,
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PARAM as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_get_float_param(
    cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_FLOAT_PARAM,
) -> libc::c_float {
    match param as libc::c_uint {
        1533126041 => return (*(*cinfo).master).lambda_log_scale1,
        3116084739 => return (*(*cinfo).master).lambda_log_scale2,
        326587475 => return (*(*cinfo).master).trellis_delta_dc_weight,
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PARAM as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    return -(1 as libc::c_int) as libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_int_param_supported(
    cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_INT_PARAM,
) -> crate::jmorecfg_h::boolean {
    match param as libc::c_uint {
        3918628389 | 1873801511 | 3057565497 | 1145645745 | 199732540 => {
            return crate::jmorecfg_h::TRUE
        }
        _ => {}
    }
    return crate::jmorecfg_h::FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_c_set_int_param(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_INT_PARAM,
    mut value: libc::c_int,
) {
    match param as libc::c_uint {
        3918628389 => match value {
            1560820397 | 720002228 => (*(*cinfo).master).compress_profile = value,
            _ => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PARAM_VALUE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        },
        1873801511 => (*(*cinfo).master).trellis_freq_split = value,
        3057565497 => (*(*cinfo).master).trellis_num_loops = value,
        1145645745 => {
            if value >= 0 as libc::c_int && value <= 8 as libc::c_int {
                (*(*cinfo).master).quant_tbl_master_idx = value
            }
        }
        199732540 => (*(*cinfo).master).dc_scan_opt_mode = value,
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PARAM as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
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
    cinfo: crate::jpeglib_h::j_compress_ptr,
    mut param: crate::jpeglib_h::J_INT_PARAM,
) -> libc::c_int {
    match param as libc::c_uint {
        3918628389 => return (*(*cinfo).master).compress_profile,
        1873801511 => return (*(*cinfo).master).trellis_freq_split,
        3057565497 => return (*(*cinfo).master).trellis_num_loops,
        1145645745 => return (*(*cinfo).master).quant_tbl_master_idx,
        199732540 => return (*(*cinfo).master).dc_scan_opt_mode,
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PARAM as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    return -(1 as libc::c_int);
}
