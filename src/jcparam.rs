use libc;

pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAX_COMPONENTS;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::CSTATE_START;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_alloc_huff_table;
pub use crate::jpeglib_h::jpeg_alloc_quant_table;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_1;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
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
pub use crate::jpeglib_h::JDCT_DEFAULT;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::jpeglib_h::MAX_COMPS_IN_SCAN;
pub use crate::jpeglib_h::NUM_ARITH_TBLS;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
pub use crate::src::jerror::C2RustUnnamed_3;
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
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;

pub use crate::jstdhuff_c::add_huff_table;
pub use crate::jstdhuff_c::std_huff_tables;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
/*
 * jcparam.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modified 2003-2008 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2011, 2018, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains optional default-setting code for the JPEG compressor.
 * Applications do not have to use this file, but those that don't use it
 * must know a lot more about the innards of the JPEG code.
 */
/*
 * Quantization table setup routines
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_add_quant_table(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut which_tbl: libc::c_int,
    mut basic_table: *const libc::c_uint,
    mut scale_factor: libc::c_int,
    mut force_baseline: crate::jmorecfg_h::boolean,
)
/* Define a quantization table equal to the basic_table times
 * a scale factor (given as a percentage).
 * If force_baseline is TRUE, the computed quantization table entries
 * are limited to 1..255 for JPEG baseline compatibility.
 */
{
    let mut qtblptr: *mut *mut crate::jpeglib_h::JQUANT_TBL =
        0 as *mut *mut crate::jpeglib_h::JQUANT_TBL;
    let mut i: libc::c_int = 0;
    let mut temp: libc::c_long = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != crate::jpegint_h::CSTATE_START {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if which_tbl < 0i32 || which_tbl >= crate::jpeglib_h::NUM_QUANT_TBLS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_DQT_INDEX as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = which_tbl;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    qtblptr = &mut *(*cinfo)
        .quant_tbl_ptrs
        .as_mut_ptr()
        .offset(which_tbl as isize) as *mut *mut crate::jpeglib_h::JQUANT_TBL;
    if (*qtblptr).is_null() {
        *qtblptr = crate::jpeglib_h::jpeg_alloc_quant_table(cinfo as crate::jpeglib_h::j_common_ptr)
    }
    i = 0i32;
    while i < crate::jpeglib_h::DCTSIZE2 {
        temp = (*basic_table.offset(i as isize) as libc::c_long * scale_factor as libc::c_long
            + 50i64)
            / 100i64;
        /* limit the values to the valid range */
        if temp <= 0i64 {
            temp = 1i64
        } /* max quantizer needed for 12 bits */
        if temp > 32767i64 {
            temp = 32767i64
        } /* limit to baseline range if requested */
        if force_baseline != 0 && temp > 255i64 {
            temp = 255i64
        }
        (**qtblptr).quantval[i as usize] = temp as crate::jmorecfg_h::UINT16;
        i += 1
    }
    /* Initialize sent_table FALSE so table will be written to JPEG file. */
    (**qtblptr).sent_table = crate::jmorecfg_h::FALSE;
}
/* These are the sample quantization tables given in Annex K (Clause K.1) of
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
 * The spec says that the values given produce "good" quality, and
 * when divided by 2, "very good" quality.
 */

static mut std_luminance_quant_tbl: [[libc::c_uint; 64]; 9] = [
    [
        16i32 as libc::c_uint,
        11i32 as libc::c_uint,
        10i32 as libc::c_uint,
        16i32 as libc::c_uint,
        24i32 as libc::c_uint,
        40i32 as libc::c_uint,
        51i32 as libc::c_uint,
        61i32 as libc::c_uint,
        12i32 as libc::c_uint,
        12i32 as libc::c_uint,
        14i32 as libc::c_uint,
        19i32 as libc::c_uint,
        26i32 as libc::c_uint,
        58i32 as libc::c_uint,
        60i32 as libc::c_uint,
        55i32 as libc::c_uint,
        14i32 as libc::c_uint,
        13i32 as libc::c_uint,
        16i32 as libc::c_uint,
        24i32 as libc::c_uint,
        40i32 as libc::c_uint,
        57i32 as libc::c_uint,
        69i32 as libc::c_uint,
        56i32 as libc::c_uint,
        14i32 as libc::c_uint,
        17i32 as libc::c_uint,
        22i32 as libc::c_uint,
        29i32 as libc::c_uint,
        51i32 as libc::c_uint,
        87i32 as libc::c_uint,
        80i32 as libc::c_uint,
        62i32 as libc::c_uint,
        18i32 as libc::c_uint,
        22i32 as libc::c_uint,
        37i32 as libc::c_uint,
        56i32 as libc::c_uint,
        68i32 as libc::c_uint,
        109i32 as libc::c_uint,
        103i32 as libc::c_uint,
        77i32 as libc::c_uint,
        24i32 as libc::c_uint,
        35i32 as libc::c_uint,
        55i32 as libc::c_uint,
        64i32 as libc::c_uint,
        81i32 as libc::c_uint,
        104i32 as libc::c_uint,
        113i32 as libc::c_uint,
        92i32 as libc::c_uint,
        49i32 as libc::c_uint,
        64i32 as libc::c_uint,
        78i32 as libc::c_uint,
        87i32 as libc::c_uint,
        103i32 as libc::c_uint,
        121i32 as libc::c_uint,
        120i32 as libc::c_uint,
        101i32 as libc::c_uint,
        72i32 as libc::c_uint,
        92i32 as libc::c_uint,
        95i32 as libc::c_uint,
        98i32 as libc::c_uint,
        112i32 as libc::c_uint,
        100i32 as libc::c_uint,
        103i32 as libc::c_uint,
        99i32 as libc::c_uint,
    ],
    [
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
    ],
    [
        12i32 as libc::c_uint,
        17i32 as libc::c_uint,
        20i32 as libc::c_uint,
        21i32 as libc::c_uint,
        30i32 as libc::c_uint,
        34i32 as libc::c_uint,
        56i32 as libc::c_uint,
        63i32 as libc::c_uint,
        18i32 as libc::c_uint,
        20i32 as libc::c_uint,
        20i32 as libc::c_uint,
        26i32 as libc::c_uint,
        28i32 as libc::c_uint,
        51i32 as libc::c_uint,
        61i32 as libc::c_uint,
        55i32 as libc::c_uint,
        19i32 as libc::c_uint,
        20i32 as libc::c_uint,
        21i32 as libc::c_uint,
        26i32 as libc::c_uint,
        33i32 as libc::c_uint,
        58i32 as libc::c_uint,
        69i32 as libc::c_uint,
        55i32 as libc::c_uint,
        26i32 as libc::c_uint,
        26i32 as libc::c_uint,
        26i32 as libc::c_uint,
        30i32 as libc::c_uint,
        46i32 as libc::c_uint,
        87i32 as libc::c_uint,
        86i32 as libc::c_uint,
        66i32 as libc::c_uint,
        31i32 as libc::c_uint,
        33i32 as libc::c_uint,
        36i32 as libc::c_uint,
        40i32 as libc::c_uint,
        46i32 as libc::c_uint,
        96i32 as libc::c_uint,
        100i32 as libc::c_uint,
        73i32 as libc::c_uint,
        40i32 as libc::c_uint,
        35i32 as libc::c_uint,
        46i32 as libc::c_uint,
        62i32 as libc::c_uint,
        81i32 as libc::c_uint,
        100i32 as libc::c_uint,
        111i32 as libc::c_uint,
        91i32 as libc::c_uint,
        46i32 as libc::c_uint,
        66i32 as libc::c_uint,
        76i32 as libc::c_uint,
        86i32 as libc::c_uint,
        102i32 as libc::c_uint,
        121i32 as libc::c_uint,
        120i32 as libc::c_uint,
        101i32 as libc::c_uint,
        68i32 as libc::c_uint,
        90i32 as libc::c_uint,
        90i32 as libc::c_uint,
        96i32 as libc::c_uint,
        113i32 as libc::c_uint,
        102i32 as libc::c_uint,
        105i32 as libc::c_uint,
        103i32 as libc::c_uint,
    ],
    [
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        18i32 as libc::c_uint,
        25i32 as libc::c_uint,
        37i32 as libc::c_uint,
        56i32 as libc::c_uint,
        85i32 as libc::c_uint,
        16i32 as libc::c_uint,
        17i32 as libc::c_uint,
        20i32 as libc::c_uint,
        27i32 as libc::c_uint,
        34i32 as libc::c_uint,
        40i32 as libc::c_uint,
        53i32 as libc::c_uint,
        75i32 as libc::c_uint,
        16i32 as libc::c_uint,
        20i32 as libc::c_uint,
        24i32 as libc::c_uint,
        31i32 as libc::c_uint,
        43i32 as libc::c_uint,
        62i32 as libc::c_uint,
        91i32 as libc::c_uint,
        135i32 as libc::c_uint,
        18i32 as libc::c_uint,
        27i32 as libc::c_uint,
        31i32 as libc::c_uint,
        40i32 as libc::c_uint,
        53i32 as libc::c_uint,
        74i32 as libc::c_uint,
        106i32 as libc::c_uint,
        156i32 as libc::c_uint,
        25i32 as libc::c_uint,
        34i32 as libc::c_uint,
        43i32 as libc::c_uint,
        53i32 as libc::c_uint,
        69i32 as libc::c_uint,
        94i32 as libc::c_uint,
        131i32 as libc::c_uint,
        189i32 as libc::c_uint,
        37i32 as libc::c_uint,
        40i32 as libc::c_uint,
        62i32 as libc::c_uint,
        74i32 as libc::c_uint,
        94i32 as libc::c_uint,
        124i32 as libc::c_uint,
        169i32 as libc::c_uint,
        238i32 as libc::c_uint,
        56i32 as libc::c_uint,
        53i32 as libc::c_uint,
        91i32 as libc::c_uint,
        106i32 as libc::c_uint,
        131i32 as libc::c_uint,
        169i32 as libc::c_uint,
        226i32 as libc::c_uint,
        311i32 as libc::c_uint,
        85i32 as libc::c_uint,
        75i32 as libc::c_uint,
        135i32 as libc::c_uint,
        156i32 as libc::c_uint,
        189i32 as libc::c_uint,
        238i32 as libc::c_uint,
        311i32 as libc::c_uint,
        418i32 as libc::c_uint,
    ],
    [
        9i32 as libc::c_uint,
        10i32 as libc::c_uint,
        12i32 as libc::c_uint,
        14i32 as libc::c_uint,
        27i32 as libc::c_uint,
        32i32 as libc::c_uint,
        51i32 as libc::c_uint,
        62i32 as libc::c_uint,
        11i32 as libc::c_uint,
        12i32 as libc::c_uint,
        14i32 as libc::c_uint,
        19i32 as libc::c_uint,
        27i32 as libc::c_uint,
        44i32 as libc::c_uint,
        59i32 as libc::c_uint,
        73i32 as libc::c_uint,
        12i32 as libc::c_uint,
        14i32 as libc::c_uint,
        18i32 as libc::c_uint,
        25i32 as libc::c_uint,
        42i32 as libc::c_uint,
        59i32 as libc::c_uint,
        79i32 as libc::c_uint,
        78i32 as libc::c_uint,
        17i32 as libc::c_uint,
        18i32 as libc::c_uint,
        25i32 as libc::c_uint,
        42i32 as libc::c_uint,
        61i32 as libc::c_uint,
        92i32 as libc::c_uint,
        87i32 as libc::c_uint,
        92i32 as libc::c_uint,
        23i32 as libc::c_uint,
        28i32 as libc::c_uint,
        42i32 as libc::c_uint,
        75i32 as libc::c_uint,
        79i32 as libc::c_uint,
        112i32 as libc::c_uint,
        112i32 as libc::c_uint,
        99i32 as libc::c_uint,
        40i32 as libc::c_uint,
        42i32 as libc::c_uint,
        59i32 as libc::c_uint,
        84i32 as libc::c_uint,
        88i32 as libc::c_uint,
        124i32 as libc::c_uint,
        132i32 as libc::c_uint,
        111i32 as libc::c_uint,
        42i32 as libc::c_uint,
        64i32 as libc::c_uint,
        78i32 as libc::c_uint,
        95i32 as libc::c_uint,
        105i32 as libc::c_uint,
        126i32 as libc::c_uint,
        125i32 as libc::c_uint,
        99i32 as libc::c_uint,
        70i32 as libc::c_uint,
        75i32 as libc::c_uint,
        100i32 as libc::c_uint,
        102i32 as libc::c_uint,
        116i32 as libc::c_uint,
        100i32 as libc::c_uint,
        107i32 as libc::c_uint,
        98i32 as libc::c_uint,
    ],
    [
        10i32 as libc::c_uint,
        12i32 as libc::c_uint,
        14i32 as libc::c_uint,
        19i32 as libc::c_uint,
        26i32 as libc::c_uint,
        38i32 as libc::c_uint,
        57i32 as libc::c_uint,
        86i32 as libc::c_uint,
        12i32 as libc::c_uint,
        18i32 as libc::c_uint,
        21i32 as libc::c_uint,
        28i32 as libc::c_uint,
        35i32 as libc::c_uint,
        41i32 as libc::c_uint,
        54i32 as libc::c_uint,
        76i32 as libc::c_uint,
        14i32 as libc::c_uint,
        21i32 as libc::c_uint,
        25i32 as libc::c_uint,
        32i32 as libc::c_uint,
        44i32 as libc::c_uint,
        63i32 as libc::c_uint,
        92i32 as libc::c_uint,
        136i32 as libc::c_uint,
        19i32 as libc::c_uint,
        28i32 as libc::c_uint,
        32i32 as libc::c_uint,
        41i32 as libc::c_uint,
        54i32 as libc::c_uint,
        75i32 as libc::c_uint,
        107i32 as libc::c_uint,
        157i32 as libc::c_uint,
        26i32 as libc::c_uint,
        35i32 as libc::c_uint,
        44i32 as libc::c_uint,
        54i32 as libc::c_uint,
        70i32 as libc::c_uint,
        95i32 as libc::c_uint,
        132i32 as libc::c_uint,
        190i32 as libc::c_uint,
        38i32 as libc::c_uint,
        41i32 as libc::c_uint,
        63i32 as libc::c_uint,
        75i32 as libc::c_uint,
        95i32 as libc::c_uint,
        125i32 as libc::c_uint,
        170i32 as libc::c_uint,
        239i32 as libc::c_uint,
        57i32 as libc::c_uint,
        54i32 as libc::c_uint,
        92i32 as libc::c_uint,
        107i32 as libc::c_uint,
        132i32 as libc::c_uint,
        170i32 as libc::c_uint,
        227i32 as libc::c_uint,
        312i32 as libc::c_uint,
        86i32 as libc::c_uint,
        76i32 as libc::c_uint,
        136i32 as libc::c_uint,
        157i32 as libc::c_uint,
        190i32 as libc::c_uint,
        239i32 as libc::c_uint,
        312i32 as libc::c_uint,
        419i32 as libc::c_uint,
    ],
    [
        7i32 as libc::c_uint,
        8i32 as libc::c_uint,
        10i32 as libc::c_uint,
        14i32 as libc::c_uint,
        23i32 as libc::c_uint,
        44i32 as libc::c_uint,
        95i32 as libc::c_uint,
        241i32 as libc::c_uint,
        8i32 as libc::c_uint,
        8i32 as libc::c_uint,
        11i32 as libc::c_uint,
        15i32 as libc::c_uint,
        25i32 as libc::c_uint,
        47i32 as libc::c_uint,
        102i32 as libc::c_uint,
        255i32 as libc::c_uint,
        10i32 as libc::c_uint,
        11i32 as libc::c_uint,
        13i32 as libc::c_uint,
        19i32 as libc::c_uint,
        31i32 as libc::c_uint,
        58i32 as libc::c_uint,
        127i32 as libc::c_uint,
        255i32 as libc::c_uint,
        14i32 as libc::c_uint,
        15i32 as libc::c_uint,
        19i32 as libc::c_uint,
        27i32 as libc::c_uint,
        44i32 as libc::c_uint,
        83i32 as libc::c_uint,
        181i32 as libc::c_uint,
        255i32 as libc::c_uint,
        23i32 as libc::c_uint,
        25i32 as libc::c_uint,
        31i32 as libc::c_uint,
        44i32 as libc::c_uint,
        72i32 as libc::c_uint,
        136i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        44i32 as libc::c_uint,
        47i32 as libc::c_uint,
        58i32 as libc::c_uint,
        83i32 as libc::c_uint,
        136i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        95i32 as libc::c_uint,
        102i32 as libc::c_uint,
        127i32 as libc::c_uint,
        181i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        241i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
    ],
    [
        15i32 as libc::c_uint,
        11i32 as libc::c_uint,
        11i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        19i32 as libc::c_uint,
        25i32 as libc::c_uint,
        32i32 as libc::c_uint,
        11i32 as libc::c_uint,
        13i32 as libc::c_uint,
        10i32 as libc::c_uint,
        10i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        19i32 as libc::c_uint,
        24i32 as libc::c_uint,
        11i32 as libc::c_uint,
        10i32 as libc::c_uint,
        14i32 as libc::c_uint,
        14i32 as libc::c_uint,
        16i32 as libc::c_uint,
        18i32 as libc::c_uint,
        22i32 as libc::c_uint,
        27i32 as libc::c_uint,
        12i32 as libc::c_uint,
        10i32 as libc::c_uint,
        14i32 as libc::c_uint,
        18i32 as libc::c_uint,
        21i32 as libc::c_uint,
        24i32 as libc::c_uint,
        28i32 as libc::c_uint,
        33i32 as libc::c_uint,
        15i32 as libc::c_uint,
        12i32 as libc::c_uint,
        16i32 as libc::c_uint,
        21i32 as libc::c_uint,
        26i32 as libc::c_uint,
        31i32 as libc::c_uint,
        36i32 as libc::c_uint,
        42i32 as libc::c_uint,
        19i32 as libc::c_uint,
        15i32 as libc::c_uint,
        18i32 as libc::c_uint,
        24i32 as libc::c_uint,
        31i32 as libc::c_uint,
        38i32 as libc::c_uint,
        45i32 as libc::c_uint,
        53i32 as libc::c_uint,
        25i32 as libc::c_uint,
        19i32 as libc::c_uint,
        22i32 as libc::c_uint,
        28i32 as libc::c_uint,
        36i32 as libc::c_uint,
        45i32 as libc::c_uint,
        55i32 as libc::c_uint,
        65i32 as libc::c_uint,
        32i32 as libc::c_uint,
        24i32 as libc::c_uint,
        27i32 as libc::c_uint,
        33i32 as libc::c_uint,
        42i32 as libc::c_uint,
        53i32 as libc::c_uint,
        65i32 as libc::c_uint,
        77i32 as libc::c_uint,
    ],
    [
        14i32 as libc::c_uint,
        10i32 as libc::c_uint,
        11i32 as libc::c_uint,
        14i32 as libc::c_uint,
        19i32 as libc::c_uint,
        25i32 as libc::c_uint,
        34i32 as libc::c_uint,
        45i32 as libc::c_uint,
        10i32 as libc::c_uint,
        11i32 as libc::c_uint,
        11i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        20i32 as libc::c_uint,
        26i32 as libc::c_uint,
        33i32 as libc::c_uint,
        11i32 as libc::c_uint,
        11i32 as libc::c_uint,
        15i32 as libc::c_uint,
        18i32 as libc::c_uint,
        21i32 as libc::c_uint,
        25i32 as libc::c_uint,
        31i32 as libc::c_uint,
        38i32 as libc::c_uint,
        14i32 as libc::c_uint,
        12i32 as libc::c_uint,
        18i32 as libc::c_uint,
        24i32 as libc::c_uint,
        28i32 as libc::c_uint,
        33i32 as libc::c_uint,
        39i32 as libc::c_uint,
        47i32 as libc::c_uint,
        19i32 as libc::c_uint,
        15i32 as libc::c_uint,
        21i32 as libc::c_uint,
        28i32 as libc::c_uint,
        36i32 as libc::c_uint,
        43i32 as libc::c_uint,
        51i32 as libc::c_uint,
        59i32 as libc::c_uint,
        25i32 as libc::c_uint,
        20i32 as libc::c_uint,
        25i32 as libc::c_uint,
        33i32 as libc::c_uint,
        43i32 as libc::c_uint,
        54i32 as libc::c_uint,
        64i32 as libc::c_uint,
        74i32 as libc::c_uint,
        34i32 as libc::c_uint,
        26i32 as libc::c_uint,
        31i32 as libc::c_uint,
        39i32 as libc::c_uint,
        51i32 as libc::c_uint,
        64i32 as libc::c_uint,
        77i32 as libc::c_uint,
        91i32 as libc::c_uint,
        45i32 as libc::c_uint,
        33i32 as libc::c_uint,
        38i32 as libc::c_uint,
        47i32 as libc::c_uint,
        59i32 as libc::c_uint,
        74i32 as libc::c_uint,
        91i32 as libc::c_uint,
        108i32 as libc::c_uint,
    ],
];

static mut std_chrominance_quant_tbl: [[libc::c_uint; 64]; 9] = [
    [
        17i32 as libc::c_uint,
        18i32 as libc::c_uint,
        24i32 as libc::c_uint,
        47i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        18i32 as libc::c_uint,
        21i32 as libc::c_uint,
        26i32 as libc::c_uint,
        66i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        24i32 as libc::c_uint,
        26i32 as libc::c_uint,
        56i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        47i32 as libc::c_uint,
        66i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
    ],
    [
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
    ],
    [
        8i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        15i32 as libc::c_uint,
        86i32 as libc::c_uint,
        96i32 as libc::c_uint,
        96i32 as libc::c_uint,
        98i32 as libc::c_uint,
        13i32 as libc::c_uint,
        13i32 as libc::c_uint,
        15i32 as libc::c_uint,
        26i32 as libc::c_uint,
        90i32 as libc::c_uint,
        96i32 as libc::c_uint,
        99i32 as libc::c_uint,
        98i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        18i32 as libc::c_uint,
        96i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        17i32 as libc::c_uint,
        16i32 as libc::c_uint,
        90i32 as libc::c_uint,
        96i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        96i32 as libc::c_uint,
        96i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
    ],
    [
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        16i32 as libc::c_uint,
        18i32 as libc::c_uint,
        25i32 as libc::c_uint,
        37i32 as libc::c_uint,
        56i32 as libc::c_uint,
        85i32 as libc::c_uint,
        16i32 as libc::c_uint,
        17i32 as libc::c_uint,
        20i32 as libc::c_uint,
        27i32 as libc::c_uint,
        34i32 as libc::c_uint,
        40i32 as libc::c_uint,
        53i32 as libc::c_uint,
        75i32 as libc::c_uint,
        16i32 as libc::c_uint,
        20i32 as libc::c_uint,
        24i32 as libc::c_uint,
        31i32 as libc::c_uint,
        43i32 as libc::c_uint,
        62i32 as libc::c_uint,
        91i32 as libc::c_uint,
        135i32 as libc::c_uint,
        18i32 as libc::c_uint,
        27i32 as libc::c_uint,
        31i32 as libc::c_uint,
        40i32 as libc::c_uint,
        53i32 as libc::c_uint,
        74i32 as libc::c_uint,
        106i32 as libc::c_uint,
        156i32 as libc::c_uint,
        25i32 as libc::c_uint,
        34i32 as libc::c_uint,
        43i32 as libc::c_uint,
        53i32 as libc::c_uint,
        69i32 as libc::c_uint,
        94i32 as libc::c_uint,
        131i32 as libc::c_uint,
        189i32 as libc::c_uint,
        37i32 as libc::c_uint,
        40i32 as libc::c_uint,
        62i32 as libc::c_uint,
        74i32 as libc::c_uint,
        94i32 as libc::c_uint,
        124i32 as libc::c_uint,
        169i32 as libc::c_uint,
        238i32 as libc::c_uint,
        56i32 as libc::c_uint,
        53i32 as libc::c_uint,
        91i32 as libc::c_uint,
        106i32 as libc::c_uint,
        131i32 as libc::c_uint,
        169i32 as libc::c_uint,
        226i32 as libc::c_uint,
        311i32 as libc::c_uint,
        85i32 as libc::c_uint,
        75i32 as libc::c_uint,
        135i32 as libc::c_uint,
        156i32 as libc::c_uint,
        189i32 as libc::c_uint,
        238i32 as libc::c_uint,
        311i32 as libc::c_uint,
        418i32 as libc::c_uint,
    ],
    [
        9i32 as libc::c_uint,
        10i32 as libc::c_uint,
        17i32 as libc::c_uint,
        19i32 as libc::c_uint,
        62i32 as libc::c_uint,
        89i32 as libc::c_uint,
        91i32 as libc::c_uint,
        97i32 as libc::c_uint,
        12i32 as libc::c_uint,
        13i32 as libc::c_uint,
        18i32 as libc::c_uint,
        29i32 as libc::c_uint,
        84i32 as libc::c_uint,
        91i32 as libc::c_uint,
        88i32 as libc::c_uint,
        98i32 as libc::c_uint,
        14i32 as libc::c_uint,
        19i32 as libc::c_uint,
        29i32 as libc::c_uint,
        93i32 as libc::c_uint,
        95i32 as libc::c_uint,
        95i32 as libc::c_uint,
        98i32 as libc::c_uint,
        97i32 as libc::c_uint,
        20i32 as libc::c_uint,
        26i32 as libc::c_uint,
        84i32 as libc::c_uint,
        88i32 as libc::c_uint,
        95i32 as libc::c_uint,
        95i32 as libc::c_uint,
        98i32 as libc::c_uint,
        94i32 as libc::c_uint,
        26i32 as libc::c_uint,
        86i32 as libc::c_uint,
        91i32 as libc::c_uint,
        93i32 as libc::c_uint,
        97i32 as libc::c_uint,
        99i32 as libc::c_uint,
        98i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        100i32 as libc::c_uint,
        98i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        97i32 as libc::c_uint,
        97i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        99i32 as libc::c_uint,
        97i32 as libc::c_uint,
        99i32 as libc::c_uint,
    ],
    [
        10i32 as libc::c_uint,
        12i32 as libc::c_uint,
        14i32 as libc::c_uint,
        19i32 as libc::c_uint,
        26i32 as libc::c_uint,
        38i32 as libc::c_uint,
        57i32 as libc::c_uint,
        86i32 as libc::c_uint,
        12i32 as libc::c_uint,
        18i32 as libc::c_uint,
        21i32 as libc::c_uint,
        28i32 as libc::c_uint,
        35i32 as libc::c_uint,
        41i32 as libc::c_uint,
        54i32 as libc::c_uint,
        76i32 as libc::c_uint,
        14i32 as libc::c_uint,
        21i32 as libc::c_uint,
        25i32 as libc::c_uint,
        32i32 as libc::c_uint,
        44i32 as libc::c_uint,
        63i32 as libc::c_uint,
        92i32 as libc::c_uint,
        136i32 as libc::c_uint,
        19i32 as libc::c_uint,
        28i32 as libc::c_uint,
        32i32 as libc::c_uint,
        41i32 as libc::c_uint,
        54i32 as libc::c_uint,
        75i32 as libc::c_uint,
        107i32 as libc::c_uint,
        157i32 as libc::c_uint,
        26i32 as libc::c_uint,
        35i32 as libc::c_uint,
        44i32 as libc::c_uint,
        54i32 as libc::c_uint,
        70i32 as libc::c_uint,
        95i32 as libc::c_uint,
        132i32 as libc::c_uint,
        190i32 as libc::c_uint,
        38i32 as libc::c_uint,
        41i32 as libc::c_uint,
        63i32 as libc::c_uint,
        75i32 as libc::c_uint,
        95i32 as libc::c_uint,
        125i32 as libc::c_uint,
        170i32 as libc::c_uint,
        239i32 as libc::c_uint,
        57i32 as libc::c_uint,
        54i32 as libc::c_uint,
        92i32 as libc::c_uint,
        107i32 as libc::c_uint,
        132i32 as libc::c_uint,
        170i32 as libc::c_uint,
        227i32 as libc::c_uint,
        312i32 as libc::c_uint,
        86i32 as libc::c_uint,
        76i32 as libc::c_uint,
        136i32 as libc::c_uint,
        157i32 as libc::c_uint,
        190i32 as libc::c_uint,
        239i32 as libc::c_uint,
        312i32 as libc::c_uint,
        419i32 as libc::c_uint,
    ],
    [
        7i32 as libc::c_uint,
        8i32 as libc::c_uint,
        10i32 as libc::c_uint,
        14i32 as libc::c_uint,
        23i32 as libc::c_uint,
        44i32 as libc::c_uint,
        95i32 as libc::c_uint,
        241i32 as libc::c_uint,
        8i32 as libc::c_uint,
        8i32 as libc::c_uint,
        11i32 as libc::c_uint,
        15i32 as libc::c_uint,
        25i32 as libc::c_uint,
        47i32 as libc::c_uint,
        102i32 as libc::c_uint,
        255i32 as libc::c_uint,
        10i32 as libc::c_uint,
        11i32 as libc::c_uint,
        13i32 as libc::c_uint,
        19i32 as libc::c_uint,
        31i32 as libc::c_uint,
        58i32 as libc::c_uint,
        127i32 as libc::c_uint,
        255i32 as libc::c_uint,
        14i32 as libc::c_uint,
        15i32 as libc::c_uint,
        19i32 as libc::c_uint,
        27i32 as libc::c_uint,
        44i32 as libc::c_uint,
        83i32 as libc::c_uint,
        181i32 as libc::c_uint,
        255i32 as libc::c_uint,
        23i32 as libc::c_uint,
        25i32 as libc::c_uint,
        31i32 as libc::c_uint,
        44i32 as libc::c_uint,
        72i32 as libc::c_uint,
        136i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        44i32 as libc::c_uint,
        47i32 as libc::c_uint,
        58i32 as libc::c_uint,
        83i32 as libc::c_uint,
        136i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        95i32 as libc::c_uint,
        102i32 as libc::c_uint,
        127i32 as libc::c_uint,
        181i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        241i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
        255i32 as libc::c_uint,
    ],
    [
        15i32 as libc::c_uint,
        11i32 as libc::c_uint,
        11i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        19i32 as libc::c_uint,
        25i32 as libc::c_uint,
        32i32 as libc::c_uint,
        11i32 as libc::c_uint,
        13i32 as libc::c_uint,
        10i32 as libc::c_uint,
        10i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        19i32 as libc::c_uint,
        24i32 as libc::c_uint,
        11i32 as libc::c_uint,
        10i32 as libc::c_uint,
        14i32 as libc::c_uint,
        14i32 as libc::c_uint,
        16i32 as libc::c_uint,
        18i32 as libc::c_uint,
        22i32 as libc::c_uint,
        27i32 as libc::c_uint,
        12i32 as libc::c_uint,
        10i32 as libc::c_uint,
        14i32 as libc::c_uint,
        18i32 as libc::c_uint,
        21i32 as libc::c_uint,
        24i32 as libc::c_uint,
        28i32 as libc::c_uint,
        33i32 as libc::c_uint,
        15i32 as libc::c_uint,
        12i32 as libc::c_uint,
        16i32 as libc::c_uint,
        21i32 as libc::c_uint,
        26i32 as libc::c_uint,
        31i32 as libc::c_uint,
        36i32 as libc::c_uint,
        42i32 as libc::c_uint,
        19i32 as libc::c_uint,
        15i32 as libc::c_uint,
        18i32 as libc::c_uint,
        24i32 as libc::c_uint,
        31i32 as libc::c_uint,
        38i32 as libc::c_uint,
        45i32 as libc::c_uint,
        53i32 as libc::c_uint,
        25i32 as libc::c_uint,
        19i32 as libc::c_uint,
        22i32 as libc::c_uint,
        28i32 as libc::c_uint,
        36i32 as libc::c_uint,
        45i32 as libc::c_uint,
        55i32 as libc::c_uint,
        65i32 as libc::c_uint,
        32i32 as libc::c_uint,
        24i32 as libc::c_uint,
        27i32 as libc::c_uint,
        33i32 as libc::c_uint,
        42i32 as libc::c_uint,
        53i32 as libc::c_uint,
        65i32 as libc::c_uint,
        77i32 as libc::c_uint,
    ],
    [
        14i32 as libc::c_uint,
        10i32 as libc::c_uint,
        11i32 as libc::c_uint,
        14i32 as libc::c_uint,
        19i32 as libc::c_uint,
        25i32 as libc::c_uint,
        34i32 as libc::c_uint,
        45i32 as libc::c_uint,
        10i32 as libc::c_uint,
        11i32 as libc::c_uint,
        11i32 as libc::c_uint,
        12i32 as libc::c_uint,
        15i32 as libc::c_uint,
        20i32 as libc::c_uint,
        26i32 as libc::c_uint,
        33i32 as libc::c_uint,
        11i32 as libc::c_uint,
        11i32 as libc::c_uint,
        15i32 as libc::c_uint,
        18i32 as libc::c_uint,
        21i32 as libc::c_uint,
        25i32 as libc::c_uint,
        31i32 as libc::c_uint,
        38i32 as libc::c_uint,
        14i32 as libc::c_uint,
        12i32 as libc::c_uint,
        18i32 as libc::c_uint,
        24i32 as libc::c_uint,
        28i32 as libc::c_uint,
        33i32 as libc::c_uint,
        39i32 as libc::c_uint,
        47i32 as libc::c_uint,
        19i32 as libc::c_uint,
        15i32 as libc::c_uint,
        21i32 as libc::c_uint,
        28i32 as libc::c_uint,
        36i32 as libc::c_uint,
        43i32 as libc::c_uint,
        51i32 as libc::c_uint,
        59i32 as libc::c_uint,
        25i32 as libc::c_uint,
        20i32 as libc::c_uint,
        25i32 as libc::c_uint,
        33i32 as libc::c_uint,
        43i32 as libc::c_uint,
        54i32 as libc::c_uint,
        64i32 as libc::c_uint,
        74i32 as libc::c_uint,
        34i32 as libc::c_uint,
        26i32 as libc::c_uint,
        31i32 as libc::c_uint,
        39i32 as libc::c_uint,
        51i32 as libc::c_uint,
        64i32 as libc::c_uint,
        77i32 as libc::c_uint,
        91i32 as libc::c_uint,
        45i32 as libc::c_uint,
        33i32 as libc::c_uint,
        38i32 as libc::c_uint,
        47i32 as libc::c_uint,
        59i32 as libc::c_uint,
        74i32 as libc::c_uint,
        91i32 as libc::c_uint,
        108i32 as libc::c_uint,
    ],
];
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_linear_quality(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut scale_factor: libc::c_int,
    mut force_baseline: crate::jmorecfg_h::boolean,
)
/* Set or change the 'quality' (quantization) setting, using default tables
 * and a straight percentage-scaling quality scale.  In most cases it's better
 * to use jpeg_set_quality (below); this entry point is provided for
 * applications that insist on a linear percentage scaling.
 */
{
    /* Set up two quantization tables using the specified scaling */
    jpeg_add_quant_table(
        cinfo,
        0i32,
        std_luminance_quant_tbl[(*(*cinfo).master).quant_tbl_master_idx as usize].as_ptr(),
        scale_factor,
        force_baseline,
    );
    jpeg_add_quant_table(
        cinfo,
        1i32,
        std_chrominance_quant_tbl[(*(*cinfo).master).quant_tbl_master_idx as usize].as_ptr(),
        scale_factor,
        force_baseline,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_quality_scaling(mut quality: libc::c_int) -> libc::c_int {
    return jpeg_float_quality_scaling(quality as libc::c_float) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_float_quality_scaling(mut quality: libc::c_float) -> libc::c_float
/* Convert a user-specified quality rating to a percentage scaling factor
 * for an underlying quantization table, using our recommended scaling curve.
 * The input 'quality' factor should be 0 (terrible) to 100 (very good).
 */ {
    /* Safety limit on quality factor.  Convert 0 to 1 to avoid zero divide. */
    if quality <= 0.0f32 {
        quality = 1.0f32
    }
    if quality > 100.0f32 {
        quality = 100.0f32
    }
    /* The basic table is used as-is (scaling 100) for a quality of 50.
     * Qualities 50..100 are converted to scaling percentage 200 - 2*Q;
     * note that at Q=100 the scaling is 0, which will cause jpeg_add_quant_table
     * to make all the table entries 1 (hence, minimum quantization loss).
     * Qualities 1..50 are converted to scaling percentage 5000/Q.
     */
    if quality < 50.0f32 {
        quality = 5000.0f32 / quality
    } else {
        quality = 200.0f32 - quality * 2.0f32
    }
    return quality;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_quality(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut quality: libc::c_int,
    mut force_baseline: crate::jmorecfg_h::boolean,
)
/* Set or change the 'quality' (quantization) setting, using default tables.
 * This is the standard quality-adjusting entry point for typical user
 * interfaces; only those who want detailed control over quantization tables
 * would use the preceding three routines directly.
 */
{
    /* Convert user 0-100 rating to percentage scaling */
    quality = jpeg_quality_scaling(quality);
    /* Set up standard quality tables */
    jpeg_set_linear_quality(cinfo, quality, force_baseline);
}
/* Default parameter setup for compression */
/*
 * Default parameter setup for compression.
 *
 * Applications that don't choose to use this routine must do their
 * own setup of all these parameters.  Alternately, you can call this
 * to establish defaults and then alter parameters selectively.  This
 * is the recommended approach since, if we add any new parameters,
 * your code will still work (they'll be set to reasonable defaults).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_defaults(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut i: libc::c_int = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != crate::jpegint_h::CSTATE_START {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Allocate comp_info array large enough for maximum component count.
     * Array is made permanent in case application wants to compress
     * multiple images at same param settings.
     */
    if (*cinfo).comp_info.is_null() {
        (*cinfo).comp_info = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_PERMANENT,
            (crate::jmorecfg_h::MAX_COMPONENTS as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<crate::jpeglib_h::jpeg_component_info>() as libc::c_ulong,
            ),
        ) as *mut crate::jpeglib_h::jpeg_component_info
    }
    /* Initialize everything not dependent on the color space */
    (*cinfo).data_precision = crate::jconfig_h::BITS_IN_JSAMPLE;
    /* Set up two quantization tables using default quality of 75 */
    jpeg_set_quality(cinfo, 75i32, crate::jmorecfg_h::TRUE);
    /* Set up two Huffman tables */
    crate::jstdhuff_c::std_huff_tables(cinfo as crate::jpeglib_h::j_common_ptr);
    /* Initialize default arithmetic coding conditioning */
    i = 0i32;
    while i < crate::jpeglib_h::NUM_ARITH_TBLS {
        (*cinfo).arith_dc_L[i as usize] = 0i32 as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_dc_U[i as usize] = 1i32 as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_ac_K[i as usize] = 5i32 as crate::jmorecfg_h::UINT8;
        i += 1
    }
    /* Default is no multiple-scan output */
    (*cinfo).scan_info = crate::stddef_h::NULL as *const crate::jpeglib_h::jpeg_scan_info;
    (*cinfo).num_scans = 0i32;
    /* Expect normal source image, not raw downsampled data */
    (*cinfo).raw_data_in = crate::jmorecfg_h::FALSE;
    /* Use Huffman coding, not arithmetic coding, by default */
    (*cinfo).arith_code = crate::jmorecfg_h::FALSE;
    if (*(*cinfo).master).compress_profile == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int {
        /* By default, do extra passes to optimize entropy coding */
        (*cinfo).optimize_coding = crate::jmorecfg_h::TRUE
    } else {
        /* By default, don't do extra passes to optimize entropy coding */
        (*cinfo).optimize_coding = crate::jmorecfg_h::FALSE
    }
    /* The standard Huffman tables are only valid for 8-bit data precision.
     * If the precision is higher, force optimization on so that usable
     * tables will be computed.  This test can be removed if default tables
     * are supplied that are valid for the desired precision.
     */
    if (*cinfo).data_precision > 8i32 {
        (*cinfo).optimize_coding = crate::jmorecfg_h::TRUE
    }
    /* By default, use the simpler non-cosited sampling alignment */
    (*cinfo).CCIR601_sampling = crate::jmorecfg_h::FALSE;
    (*(*cinfo).master).overshoot_deringing = ((*(*cinfo).master).compress_profile
        == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int)
        as libc::c_int;
    /* No input smoothing */
    (*cinfo).smoothing_factor = 0i32;
    /* DCT algorithm preference */
    (*cinfo).dct_method = crate::jpeglib_h::JDCT_DEFAULT as crate::jpeglib_h::J_DCT_METHOD;
    /* No restart markers */
    (*cinfo).restart_interval = 0i32 as libc::c_uint;
    (*cinfo).restart_in_rows = 0i32;
    /* Fill in default JFIF marker parameters.  Note that whether the marker
     * will actually be written is determined by jpeg_set_colorspace.
     *
     * By default, the library emits JFIF version code 1.01.
     * An application that wants to emit JFIF 1.02 extension markers should set
     * JFIF_minor_version to 2.  We could probably get away with just defaulting
     * to 1.02, but there may still be some decoders in use that will complain
     * about that; saying 1.01 should minimize compatibility problems.
     */
    (*cinfo).JFIF_major_version = 1i32 as crate::jmorecfg_h::UINT8; /* Default JFIF version = 1.01 */
    (*cinfo).JFIF_minor_version = 1i32 as crate::jmorecfg_h::UINT8; /* Pixel size is unknown by default */
    (*cinfo).density_unit = 0i32 as crate::jmorecfg_h::UINT8; /* Pixel aspect ratio is square by default */
    (*cinfo).X_density = 1i32 as crate::jmorecfg_h::UINT16;
    (*cinfo).Y_density = 1i32 as crate::jmorecfg_h::UINT16;
    /* Choose JPEG colorspace based on input space, set defaults accordingly */
    jpeg_default_colorspace(cinfo);
    (*(*cinfo).master).dc_scan_opt_mode = 1i32;
    if (*(*cinfo).master).compress_profile == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int {
        (*(*cinfo).master).optimize_scans = crate::jmorecfg_h::TRUE;
        jpeg_simple_progression(cinfo);
    } else {
        (*(*cinfo).master).optimize_scans = crate::jmorecfg_h::FALSE
    }
    (*(*cinfo).master).trellis_quant = ((*(*cinfo).master).compress_profile
        == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int)
        as libc::c_int;
    (*(*cinfo).master).lambda_log_scale1 = 14.75f64 as libc::c_float;
    (*(*cinfo).master).lambda_log_scale2 = 16.5f64 as libc::c_float;
    (*(*cinfo).master).quant_tbl_master_idx = if (*(*cinfo).master).compress_profile
        == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int
    {
        3i32
    } else {
        0i32
    };
    (*(*cinfo).master).use_lambda_weight_tbl = crate::jmorecfg_h::TRUE;
    (*(*cinfo).master).use_scans_in_trellis = crate::jmorecfg_h::FALSE;
    (*(*cinfo).master).trellis_freq_split = 8i32;
    (*(*cinfo).master).trellis_num_loops = 1i32;
    (*(*cinfo).master).trellis_q_opt = crate::jmorecfg_h::FALSE;
    (*(*cinfo).master).trellis_quant_dc = crate::jmorecfg_h::TRUE;
    (*(*cinfo).master).trellis_delta_dc_weight = 0.0f64 as libc::c_float;
}
/*
 * Select an appropriate JPEG colorspace for in_color_space.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_default_colorspace(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    match (*cinfo).in_color_space as libc::c_uint {
        1 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_GRAYSCALE); /* By default, no translation */
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCbCr);
        }
        3 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCbCr);
        }
        4 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_CMYK);
        }
        5 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCCK);
        }
        0 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_UNKNOWN);
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_IN_COLORSPACE as libc::c_int;
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
/* Compression parameter setup aids */
/*
 * Set the JPEG colorspace, and choose colorspace-dependent default values.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_colorspace(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut colorspace: crate::jpeglib_h::J_COLOR_SPACE,
) {
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut ci: libc::c_int = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != crate::jpegint_h::CSTATE_START {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For all colorspaces, we use Q and Huff tables 0 for luminance components,
     * tables 1 for chrominance components.
     */
    (*cinfo).jpeg_color_space = colorspace; /* No marker for non-JFIF colorspaces */
    (*cinfo).write_JFIF_header = crate::jmorecfg_h::FALSE; /* write no Adobe marker by default */
    (*cinfo).write_Adobe_marker = crate::jmorecfg_h::FALSE; /* Write a JFIF marker */
    match colorspace as libc::c_uint {
        1 => {
            (*cinfo).write_JFIF_header = crate::jmorecfg_h::TRUE;
            (*cinfo).num_components = 1i32;
            /* JFIF specifies component ID 1 */
            compptr =
                &mut *(*cinfo).comp_info.offset(0) as *mut crate::jpeglib_h::jpeg_component_info; /* write Adobe marker to flag RGB */
            (*compptr).component_id = 1i32; /* Write a JFIF marker */
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        2 => {
            (*cinfo).write_Adobe_marker = crate::jmorecfg_h::TRUE;
            (*cinfo).num_components = 3i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(0) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x52i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(1) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x47i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(2) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x42i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        3 => {
            (*cinfo).write_JFIF_header = crate::jmorecfg_h::TRUE;
            (*cinfo).num_components = 3i32;
            /* JFIF specifies component IDs 1,2,3 */
            /* We default to 2x2 subsamples of chrominance */
            compptr =
                &mut *(*cinfo).comp_info.offset(0) as *mut crate::jpeglib_h::jpeg_component_info; /* write Adobe marker to flag CMYK */
            (*compptr).component_id = 1i32; /* write Adobe marker to flag YCCK */
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(1) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 2i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(2) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 3i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32
        }
        4 => {
            (*cinfo).write_Adobe_marker = crate::jmorecfg_h::TRUE;
            (*cinfo).num_components = 4i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(0) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x43i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(1) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x4di32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(2) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x59i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(3) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x4bi32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        5 => {
            (*cinfo).write_Adobe_marker = crate::jmorecfg_h::TRUE;
            (*cinfo).num_components = 4i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(0) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 1i32;
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(1) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 2i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(2) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 3i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr =
                &mut *(*cinfo).comp_info.offset(3) as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 4i32;
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        0 => {
            (*cinfo).num_components = (*cinfo).input_components;
            if (*cinfo).num_components < 1i32
                || (*cinfo).num_components > crate::jmorecfg_h::MAX_COMPONENTS
            {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_COMPONENT_COUNT as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = (*cinfo).num_components;
                (*(*cinfo).err).msg_parm.i[1] = 10i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci = 0i32;
            while ci < (*cinfo).num_components {
                compptr = &mut *(*cinfo).comp_info.offset(ci as isize)
                    as *mut crate::jpeglib_h::jpeg_component_info;
                (*compptr).component_id = ci;
                (*compptr).h_samp_factor = 1i32;
                (*compptr).v_samp_factor = 1i32;
                (*compptr).quant_tbl_no = 0i32;
                (*compptr).dc_tbl_no = 0i32;
                (*compptr).ac_tbl_no = 0i32;
                ci += 1
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
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

unsafe extern "C" fn fill_a_scan(
    mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info,
    mut ci: libc::c_int,
    mut Ss: libc::c_int,
    mut Se: libc::c_int,
    mut Ah: libc::c_int,
    mut Al: libc::c_int,
) -> *mut crate::jpeglib_h::jpeg_scan_info
/* Support routine: generate one scan for specified component */ {
    (*scanptr).comps_in_scan = 1i32;
    (*scanptr).component_index[0] = ci;
    (*scanptr).Ss = Ss;
    (*scanptr).Se = Se;
    (*scanptr).Ah = Ah;
    (*scanptr).Al = Al;
    scanptr = scanptr.offset(1);
    return scanptr;
}

unsafe extern "C" fn fill_a_scan_pair(
    mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info,
    mut ci: libc::c_int,
    mut Ss: libc::c_int,
    mut Se: libc::c_int,
    mut Ah: libc::c_int,
    mut Al: libc::c_int,
) -> *mut crate::jpeglib_h::jpeg_scan_info
/* Support routine: generate one scan for pair of components */ {
    (*scanptr).comps_in_scan = 2i32;
    (*scanptr).component_index[0] = ci;
    (*scanptr).component_index[1] = ci + 1i32;
    (*scanptr).Ss = Ss;
    (*scanptr).Se = Se;
    (*scanptr).Ah = Ah;
    (*scanptr).Al = Al;
    scanptr = scanptr.offset(1);
    return scanptr;
}

unsafe extern "C" fn fill_scans(
    mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info,
    mut ncomps: libc::c_int,
    mut Ss: libc::c_int,
    mut Se: libc::c_int,
    mut Ah: libc::c_int,
    mut Al: libc::c_int,
) -> *mut crate::jpeglib_h::jpeg_scan_info
/* Support routine: generate one scan for each component */ {
    let mut ci: libc::c_int = 0;
    ci = 0i32;
    while ci < ncomps {
        (*scanptr).comps_in_scan = 1i32;
        (*scanptr).component_index[0] = ci;
        (*scanptr).Ss = Ss;
        (*scanptr).Se = Se;
        (*scanptr).Ah = Ah;
        (*scanptr).Al = Al;
        scanptr = scanptr.offset(1);
        ci += 1
    }
    return scanptr;
}

unsafe extern "C" fn fill_dc_scans(
    mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info,
    mut ncomps: libc::c_int,
    mut Ah: libc::c_int,
    mut Al: libc::c_int,
) -> *mut crate::jpeglib_h::jpeg_scan_info
/* Support routine: generate interleaved DC scan if possible, else N scans */ {
    let mut ci: libc::c_int = 0;
    if ncomps <= crate::jpeglib_h::MAX_COMPS_IN_SCAN {
        /* Single interleaved DC scan */
        (*scanptr).comps_in_scan = ncomps;
        ci = 0i32;
        while ci < ncomps {
            (*scanptr).component_index[ci as usize] = ci;
            ci += 1
        }
        (*scanptr).Se = 0i32;
        (*scanptr).Ss = (*scanptr).Se;
        (*scanptr).Ah = Ah;
        (*scanptr).Al = Al;
        scanptr = scanptr.offset(1)
    } else {
        /* Noninterleaved DC scan for each component */
        scanptr = fill_scans(scanptr, ncomps, 0i32, 0i32, Ah, Al)
    }
    return scanptr;
}
/*
 * List of scans to be tested
 * cinfo->num_components and cinfo->jpeg_color_space must be correct.
 */

unsafe extern "C" fn jpeg_search_progression(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut ncomps: libc::c_int = (*cinfo).num_components;
    let mut nscans: libc::c_int = 0;
    let mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info =
        0 as *mut crate::jpeglib_h::jpeg_scan_info;
    let mut Al: libc::c_int = 0;
    let mut frequency_split: [libc::c_int; 5] = [2i32, 8i32, 5i32, 12i32, 18i32];
    let mut i: libc::c_int = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != crate::jpegint_h::CSTATE_START {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Figure space needed for script.  Calculation must match code below! */
    if ncomps == 3i32
        && (*cinfo).jpeg_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
    {
        /* Custom script for YCbCr color images. */
        nscans = 64i32
    } else if ncomps == 1i32 {
        nscans = 23i32
    } else {
        (*(*cinfo).master).num_scans_luma = 0i32;
        return crate::jmorecfg_h::FALSE;
    }
    /* Allocate space for script.
     * We need to put it in the permanent pool in case the application performs
     * multiple compressions without changing the settings.  To avoid a memory
     * leak if jpeg_simple_progression is called repeatedly for the same JPEG
     * object, we try to re-use previously allocated space, and we allocate
     * enough space to handle YCbCr even if initially asked for grayscale.
     */
    if (*cinfo).script_space.is_null() || (*cinfo).script_space_size < nscans {
        (*cinfo).script_space_size = if nscans > 64i32 { nscans } else { 64i32 };
        (*cinfo).script_space = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_PERMANENT,
            ((*cinfo).script_space_size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::jpeglib_h::jpeg_scan_info>() as libc::c_ulong
                ),
        ) as *mut crate::jpeglib_h::jpeg_scan_info
    }
    scanptr = (*cinfo).script_space;
    (*cinfo).scan_info = scanptr;
    (*cinfo).num_scans = nscans;
    (*(*cinfo).master).Al_max_luma = 3i32;
    (*(*cinfo).master).num_scans_luma_dc = 1i32;
    (*(*cinfo).master).num_frequency_splits = 5i32;
    (*(*cinfo).master).num_scans_luma = (*(*cinfo).master).num_scans_luma_dc
        + (3i32 * (*(*cinfo).master).Al_max_luma + 2i32)
        + (2i32 * (*(*cinfo).master).num_frequency_splits + 1i32);
    /* 23 scans for luma */
    /* 1 scan for DC */
    /* 11 scans to determine successive approximation */
    /* 11 scans to determine frequency approximation */
    /* after 12 scans need to update following 11 */
    /* after 23 scans need to determine which to keep */
    /* last 4 done conditionally */
    /* luma DC by itself */
    if (*(*cinfo).master).dc_scan_opt_mode == 0i32 {
        scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 0i32)
    } else {
        scanptr = fill_dc_scans(scanptr, 1i32, 0i32, 0i32)
    }
    scanptr = fill_a_scan(scanptr, 0i32, 1i32, 8i32, 0i32, 0i32);
    scanptr = fill_a_scan(scanptr, 0i32, 9i32, 63i32, 0i32, 0i32);
    Al = 0i32;
    while Al < (*(*cinfo).master).Al_max_luma {
        scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, Al + 1i32, Al);
        scanptr = fill_a_scan(scanptr, 0i32, 1i32, 8i32, 0i32, Al + 1i32);
        scanptr = fill_a_scan(scanptr, 0i32, 9i32, 63i32, 0i32, Al + 1i32);
        Al += 1
    }
    scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 0i32, 0i32);
    i = 0i32;
    while i < (*(*cinfo).master).num_frequency_splits {
        scanptr = fill_a_scan(scanptr, 0i32, 1i32, frequency_split[i as usize], 0i32, 0i32);
        scanptr = fill_a_scan(
            scanptr,
            0i32,
            frequency_split[i as usize] + 1i32,
            63i32,
            0i32,
            0i32,
        );
        i += 1
    }
    if ncomps == 1i32 {
        (*(*cinfo).master).Al_max_chroma = 0i32;
        (*(*cinfo).master).num_scans_chroma_dc = 0i32
    } else {
        (*(*cinfo).master).Al_max_chroma = 2i32;
        (*(*cinfo).master).num_scans_chroma_dc = 3i32;
        /* 41 scans for chroma */
        /* chroma DC combined */
        scanptr = fill_a_scan_pair(scanptr, 1i32, 0i32, 0i32, 0i32, 0i32);
        /* chroma DC separate */
        scanptr = fill_a_scan(scanptr, 1i32, 0i32, 0i32, 0i32, 0i32);
        scanptr = fill_a_scan(scanptr, 2i32, 0i32, 0i32, 0i32, 0i32);
        scanptr = fill_a_scan(scanptr, 1i32, 1i32, 8i32, 0i32, 0i32);
        scanptr = fill_a_scan(scanptr, 1i32, 9i32, 63i32, 0i32, 0i32);
        scanptr = fill_a_scan(scanptr, 2i32, 1i32, 8i32, 0i32, 0i32);
        scanptr = fill_a_scan(scanptr, 2i32, 9i32, 63i32, 0i32, 0i32);
        Al = 0i32;
        while Al < (*(*cinfo).master).Al_max_chroma {
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 63i32, Al + 1i32, Al);
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 63i32, Al + 1i32, Al);
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 8i32, 0i32, Al + 1i32);
            scanptr = fill_a_scan(scanptr, 1i32, 9i32, 63i32, 0i32, Al + 1i32);
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 8i32, 0i32, Al + 1i32);
            scanptr = fill_a_scan(scanptr, 2i32, 9i32, 63i32, 0i32, Al + 1i32);
            Al += 1
        }
        scanptr = fill_a_scan(scanptr, 1i32, 1i32, 63i32, 0i32, 0i32);
        scanptr = fill_a_scan(scanptr, 2i32, 1i32, 63i32, 0i32, 0i32);
        i = 0i32;
        while i < (*(*cinfo).master).num_frequency_splits {
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, frequency_split[i as usize], 0i32, 0i32);
            scanptr = fill_a_scan(
                scanptr,
                1i32,
                frequency_split[i as usize] + 1i32,
                63i32,
                0i32,
                0i32,
            );
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, frequency_split[i as usize], 0i32, 0i32);
            scanptr = fill_a_scan(
                scanptr,
                2i32,
                frequency_split[i as usize] + 1i32,
                63i32,
                0i32,
                0i32,
            );
            i += 1
        }
    }
    return crate::jmorecfg_h::TRUE;
}
/*
 * Create a recommended progressive-JPEG script.
 * cinfo->num_components and cinfo->jpeg_color_space must be correct.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_simple_progression(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut ncomps: libc::c_int = 0;
    let mut nscans: libc::c_int = 0;
    let mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info =
        0 as *mut crate::jpeglib_h::jpeg_scan_info;
    if (*(*cinfo).master).optimize_scans != 0 {
        if jpeg_search_progression(cinfo) == crate::jmorecfg_h::TRUE {
            return;
        }
    }
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != crate::jpegint_h::CSTATE_START {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Figure space needed for script.  Calculation must match code below! */
    ncomps = (*cinfo).num_components;
    if ncomps == 3i32
        && (*cinfo).jpeg_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
    {
        /* Custom script for YCbCr color images. */
        if (*(*cinfo).master).compress_profile
            == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int
        {
            if (*(*cinfo).master).dc_scan_opt_mode == 0i32 {
                nscans = 9i32
            /* 1 DC scan for all components */
            } else if (*(*cinfo).master).dc_scan_opt_mode == 1i32 {
                nscans = 11i32
            /* 1 DC scan for each component */
            } else {
                nscans = 10i32
                /* 1 DC scan for luminance and 1 DC scan for chroma */
            }
        } else {
            nscans = 10i32
            /* 2 DC scans and 8 AC scans */
        }
    } else if (*(*cinfo).master).compress_profile
        == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int
    {
        if ncomps > crate::jpeglib_h::MAX_COMPS_IN_SCAN {
            /* All-purpose script for other color spaces. */
            nscans = 5i32 * ncomps
        } else {
            nscans = 1i32 + 4i32 * ncomps
        } /* 2 DC + 4 AC scans per component */
    /* 2 DC scans; 4 AC scans per component */
    } else if ncomps > crate::jpeglib_h::MAX_COMPS_IN_SCAN {
        nscans = 6i32 * ncomps
    } else {
        nscans = 2i32 + 4i32 * ncomps
    } /* 2 DC + 4 AC scans per component */
    /* 2 DC scans; 4 AC scans per component */
    /* Allocate space for script.
     * We need to put it in the permanent pool in case the application performs
     * multiple compressions without changing the settings.  To avoid a memory
     * leak if jpeg_simple_progression is called repeatedly for the same JPEG
     * object, we try to re-use previously allocated space, and we allocate
     * enough space to handle YCbCr even if initially asked for grayscale.
     */
    if (*cinfo).script_space.is_null() || (*cinfo).script_space_size < nscans {
        (*cinfo).script_space_size = if nscans > 10i32 { nscans } else { 10i32 };
        (*cinfo).script_space = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_PERMANENT,
            ((*cinfo).script_space_size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::jpeglib_h::jpeg_scan_info>() as libc::c_ulong
                ),
        ) as *mut crate::jpeglib_h::jpeg_scan_info
    }
    scanptr = (*cinfo).script_space;
    (*cinfo).scan_info = scanptr;
    (*cinfo).num_scans = nscans;
    if ncomps == 3i32
        && (*cinfo).jpeg_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
    {
        /* Custom script for YCbCr color images. */
        if (*(*cinfo).master).compress_profile
            == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int
        {
            /* scan defined in jpeg_scan_rgb.txt in jpgcrush */
            /* Initial DC scan */
            if (*(*cinfo).master).dc_scan_opt_mode == 0i32 {
                /* 1 DC scan for all components */
                scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 0i32)
            } else if (*(*cinfo).master).dc_scan_opt_mode == 1i32 {
                /* 1 DC scan for each component */
                scanptr = fill_a_scan(scanptr, 0i32, 0i32, 0i32, 0i32, 0i32);
                scanptr = fill_a_scan(scanptr, 1i32, 0i32, 0i32, 0i32, 0i32);
                scanptr = fill_a_scan(scanptr, 2i32, 0i32, 0i32, 0i32, 0i32)
            } else {
                /* 1 DC scan for luminance and 1 DC scan for chroma */
                scanptr = fill_dc_scans(scanptr, 1i32, 0i32, 0i32);
                scanptr = fill_a_scan_pair(scanptr, 1i32, 0i32, 0i32, 0i32, 0i32)
            }
            /* Low frequency AC scans */
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 8i32, 0i32, 2i32);
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 8i32, 0i32, 0i32);
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 8i32, 0i32, 0i32);
            /* Complete spectral selection for luma AC */
            scanptr = fill_a_scan(scanptr, 0i32, 9i32, 63i32, 0i32, 2i32);
            /* Finish luma AC successive approximation */
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 2i32, 1i32);
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 1i32, 0i32);
            /* Complete spectral selection for chroma AC */
            scanptr = fill_a_scan(scanptr, 1i32, 9i32, 63i32, 0i32, 0i32);
            scanptr = fill_a_scan(scanptr, 2i32, 9i32, 63i32, 0i32, 0i32)
        } else {
            /* Initial DC scan */
            scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 1i32);
            /* Initial AC scan: get some luma data out in a hurry */
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 5i32, 0i32, 2i32);
            /* Chroma data is too small to be worth expending many scans on */
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 63i32, 0i32, 1i32);
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 63i32, 0i32, 1i32);
            /* Complete spectral selection for luma AC */
            scanptr = fill_a_scan(scanptr, 0i32, 6i32, 63i32, 0i32, 2i32);
            /* Refine next bit of luma AC */
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 2i32, 1i32);
            /* Finish DC successive approximation */
            scanptr = fill_dc_scans(scanptr, ncomps, 1i32, 0i32);
            /* Finish AC successive approximation */
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 63i32, 1i32, 0i32);
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 63i32, 1i32, 0i32);
            /* Luma bottom bit comes last since it's usually largest scan */
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 1i32, 0i32)
        }
    } else if (*(*cinfo).master).compress_profile
        == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int
    {
        /* All-purpose script for other color spaces. */
        /* scan defined in jpeg_scan_bw.txt in jpgcrush */
        /* DC component, no successive approximation */
        scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 0i32);
        /* Successive approximation first pass */
        scanptr = fill_scans(scanptr, ncomps, 1i32, 8i32, 0i32, 2i32);
        scanptr = fill_scans(scanptr, ncomps, 9i32, 63i32, 0i32, 2i32);
        /* Successive approximation second pass */
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 2i32, 1i32);
        /* Successive approximation final pass */
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 1i32, 0i32)
    } else {
        /* Successive approximation first pass */
        scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 1i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 5i32, 0i32, 2i32);
        scanptr = fill_scans(scanptr, ncomps, 6i32, 63i32, 0i32, 2i32);
        /* Successive approximation second pass */
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 2i32, 1i32);
        /* Successive approximation final pass */
        scanptr = fill_dc_scans(scanptr, ncomps, 1i32, 0i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 1i32, 0i32)
    };
}
/* C_PROGRESSIVE_SUPPORTED */
