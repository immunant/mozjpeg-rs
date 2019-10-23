pub use super::jerror::{
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
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAX_COMPONENTS, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, CSTATE_START, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_alloc_huff_table, jpeg_alloc_quant_table,
    jpeg_c_coef_controller, jpeg_c_main_controller, jpeg_c_prep_controller, jpeg_color_converter,
    jpeg_color_deconverter, jpeg_color_quantizer, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_destination_mgr,
    jpeg_downsampler, jpeg_entropy_decoder, jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct,
    jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method, jpeg_marker_reader,
    jpeg_marker_struct, jpeg_marker_writer, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_scan_info, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control,
    jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_1, C2RustUnnamed_2,
    JCS_YCbCr, DCTSIZE2, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCP_FASTEST,
    JCP_MAX_COMPRESSION, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_DEFAULT, JDCT_FLOAT,
    JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPOOL_PERMANENT,
    JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
    MAX_COMPS_IN_SCAN, NUM_ARITH_TBLS, NUM_QUANT_TBLS,
};
pub use crate::jstdhuff_c::{add_huff_table, std_huff_tables};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_float, c_int, c_long, c_uint, c_ulong};
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
    mut cinfo: j_compress_ptr,
    mut which_tbl: c_int,
    mut basic_table: *const c_uint,
    mut scale_factor: c_int,
    mut force_baseline: boolean,
)
/* Define a quantization table equal to the basic_table times
 * a scale factor (given as a percentage).
 * If force_baseline is TRUE, the computed quantization table entries
 * are limited to 1..255 for JPEG baseline compatibility.
 */
{
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if which_tbl < 0i32 || which_tbl >= NUM_QUANT_TBLS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_DQT_INDEX as c_int;
        (*(*cinfo).err).msg_parm.i[0] = which_tbl;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    let mut qtblptr: *mut *mut JQUANT_TBL = &mut *(*cinfo)
        .quant_tbl_ptrs
        .as_mut_ptr()
        .offset(which_tbl as isize)
        as *mut *mut JQUANT_TBL;
    if (*qtblptr).is_null() {
        *qtblptr = jpeg_alloc_quant_table(cinfo as j_common_ptr)
    }
    let mut i: c_int = 0i32;
    while i < DCTSIZE2 {
        let mut temp: c_long =
            (*basic_table.offset(i as isize) as c_long * scale_factor as c_long + 50i64) / 100i64;
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
        (**qtblptr).quantval[i as usize] = temp as UINT16;
        i += 1
    }
    /* Initialize sent_table FALSE so table will be written to JPEG file. */
    (**qtblptr).sent_table = FALSE;
}
/* These are the sample quantization tables given in Annex K (Clause K.1) of
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
 * The spec says that the values given produce "good" quality, and
 * when divided by 2, "very good" quality.
 */

static mut std_luminance_quant_tbl: [[c_uint; 64]; 9] = [
    [
        16u32, 11u32, 10u32, 16u32, 24u32, 40u32, 51u32, 61u32, 12u32, 12u32, 14u32, 19u32, 26u32,
        58u32, 60u32, 55u32, 14u32, 13u32, 16u32, 24u32, 40u32, 57u32, 69u32, 56u32, 14u32, 17u32,
        22u32, 29u32, 51u32, 87u32, 80u32, 62u32, 18u32, 22u32, 37u32, 56u32, 68u32, 109u32,
        103u32, 77u32, 24u32, 35u32, 55u32, 64u32, 81u32, 104u32, 113u32, 92u32, 49u32, 64u32,
        78u32, 87u32, 103u32, 121u32, 120u32, 101u32, 72u32, 92u32, 95u32, 98u32, 112u32, 100u32,
        103u32, 99u32,
    ],
    [
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
    ],
    [
        12u32, 17u32, 20u32, 21u32, 30u32, 34u32, 56u32, 63u32, 18u32, 20u32, 20u32, 26u32, 28u32,
        51u32, 61u32, 55u32, 19u32, 20u32, 21u32, 26u32, 33u32, 58u32, 69u32, 55u32, 26u32, 26u32,
        26u32, 30u32, 46u32, 87u32, 86u32, 66u32, 31u32, 33u32, 36u32, 40u32, 46u32, 96u32, 100u32,
        73u32, 40u32, 35u32, 46u32, 62u32, 81u32, 100u32, 111u32, 91u32, 46u32, 66u32, 76u32,
        86u32, 102u32, 121u32, 120u32, 101u32, 68u32, 90u32, 90u32, 96u32, 113u32, 102u32, 105u32,
        103u32,
    ],
    [
        16u32, 16u32, 16u32, 18u32, 25u32, 37u32, 56u32, 85u32, 16u32, 17u32, 20u32, 27u32, 34u32,
        40u32, 53u32, 75u32, 16u32, 20u32, 24u32, 31u32, 43u32, 62u32, 91u32, 135u32, 18u32, 27u32,
        31u32, 40u32, 53u32, 74u32, 106u32, 156u32, 25u32, 34u32, 43u32, 53u32, 69u32, 94u32,
        131u32, 189u32, 37u32, 40u32, 62u32, 74u32, 94u32, 124u32, 169u32, 238u32, 56u32, 53u32,
        91u32, 106u32, 131u32, 169u32, 226u32, 311u32, 85u32, 75u32, 135u32, 156u32, 189u32,
        238u32, 311u32, 418u32,
    ],
    [
        9u32, 10u32, 12u32, 14u32, 27u32, 32u32, 51u32, 62u32, 11u32, 12u32, 14u32, 19u32, 27u32,
        44u32, 59u32, 73u32, 12u32, 14u32, 18u32, 25u32, 42u32, 59u32, 79u32, 78u32, 17u32, 18u32,
        25u32, 42u32, 61u32, 92u32, 87u32, 92u32, 23u32, 28u32, 42u32, 75u32, 79u32, 112u32,
        112u32, 99u32, 40u32, 42u32, 59u32, 84u32, 88u32, 124u32, 132u32, 111u32, 42u32, 64u32,
        78u32, 95u32, 105u32, 126u32, 125u32, 99u32, 70u32, 75u32, 100u32, 102u32, 116u32, 100u32,
        107u32, 98u32,
    ],
    [
        10u32, 12u32, 14u32, 19u32, 26u32, 38u32, 57u32, 86u32, 12u32, 18u32, 21u32, 28u32, 35u32,
        41u32, 54u32, 76u32, 14u32, 21u32, 25u32, 32u32, 44u32, 63u32, 92u32, 136u32, 19u32, 28u32,
        32u32, 41u32, 54u32, 75u32, 107u32, 157u32, 26u32, 35u32, 44u32, 54u32, 70u32, 95u32,
        132u32, 190u32, 38u32, 41u32, 63u32, 75u32, 95u32, 125u32, 170u32, 239u32, 57u32, 54u32,
        92u32, 107u32, 132u32, 170u32, 227u32, 312u32, 86u32, 76u32, 136u32, 157u32, 190u32,
        239u32, 312u32, 419u32,
    ],
    [
        7u32, 8u32, 10u32, 14u32, 23u32, 44u32, 95u32, 241u32, 8u32, 8u32, 11u32, 15u32, 25u32,
        47u32, 102u32, 255u32, 10u32, 11u32, 13u32, 19u32, 31u32, 58u32, 127u32, 255u32, 14u32,
        15u32, 19u32, 27u32, 44u32, 83u32, 181u32, 255u32, 23u32, 25u32, 31u32, 44u32, 72u32,
        136u32, 255u32, 255u32, 44u32, 47u32, 58u32, 83u32, 136u32, 255u32, 255u32, 255u32, 95u32,
        102u32, 127u32, 181u32, 255u32, 255u32, 255u32, 255u32, 241u32, 255u32, 255u32, 255u32,
        255u32, 255u32, 255u32, 255u32,
    ],
    [
        15u32, 11u32, 11u32, 12u32, 15u32, 19u32, 25u32, 32u32, 11u32, 13u32, 10u32, 10u32, 12u32,
        15u32, 19u32, 24u32, 11u32, 10u32, 14u32, 14u32, 16u32, 18u32, 22u32, 27u32, 12u32, 10u32,
        14u32, 18u32, 21u32, 24u32, 28u32, 33u32, 15u32, 12u32, 16u32, 21u32, 26u32, 31u32, 36u32,
        42u32, 19u32, 15u32, 18u32, 24u32, 31u32, 38u32, 45u32, 53u32, 25u32, 19u32, 22u32, 28u32,
        36u32, 45u32, 55u32, 65u32, 32u32, 24u32, 27u32, 33u32, 42u32, 53u32, 65u32, 77u32,
    ],
    [
        14u32, 10u32, 11u32, 14u32, 19u32, 25u32, 34u32, 45u32, 10u32, 11u32, 11u32, 12u32, 15u32,
        20u32, 26u32, 33u32, 11u32, 11u32, 15u32, 18u32, 21u32, 25u32, 31u32, 38u32, 14u32, 12u32,
        18u32, 24u32, 28u32, 33u32, 39u32, 47u32, 19u32, 15u32, 21u32, 28u32, 36u32, 43u32, 51u32,
        59u32, 25u32, 20u32, 25u32, 33u32, 43u32, 54u32, 64u32, 74u32, 34u32, 26u32, 31u32, 39u32,
        51u32, 64u32, 77u32, 91u32, 45u32, 33u32, 38u32, 47u32, 59u32, 74u32, 91u32, 108u32,
    ],
];

static mut std_chrominance_quant_tbl: [[c_uint; 64]; 9] = [
    [
        17u32, 18u32, 24u32, 47u32, 99u32, 99u32, 99u32, 99u32, 18u32, 21u32, 26u32, 66u32, 99u32,
        99u32, 99u32, 99u32, 24u32, 26u32, 56u32, 99u32, 99u32, 99u32, 99u32, 99u32, 47u32, 66u32,
        99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32,
        99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32,
        99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32,
    ],
    [
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
        16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32, 16u32,
    ],
    [
        8u32, 12u32, 15u32, 15u32, 86u32, 96u32, 96u32, 98u32, 13u32, 13u32, 15u32, 26u32, 90u32,
        96u32, 99u32, 98u32, 12u32, 15u32, 18u32, 96u32, 99u32, 99u32, 99u32, 99u32, 17u32, 16u32,
        90u32, 96u32, 99u32, 99u32, 99u32, 99u32, 96u32, 96u32, 99u32, 99u32, 99u32, 99u32, 99u32,
        99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32,
        99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32,
    ],
    [
        16u32, 16u32, 16u32, 18u32, 25u32, 37u32, 56u32, 85u32, 16u32, 17u32, 20u32, 27u32, 34u32,
        40u32, 53u32, 75u32, 16u32, 20u32, 24u32, 31u32, 43u32, 62u32, 91u32, 135u32, 18u32, 27u32,
        31u32, 40u32, 53u32, 74u32, 106u32, 156u32, 25u32, 34u32, 43u32, 53u32, 69u32, 94u32,
        131u32, 189u32, 37u32, 40u32, 62u32, 74u32, 94u32, 124u32, 169u32, 238u32, 56u32, 53u32,
        91u32, 106u32, 131u32, 169u32, 226u32, 311u32, 85u32, 75u32, 135u32, 156u32, 189u32,
        238u32, 311u32, 418u32,
    ],
    [
        9u32, 10u32, 17u32, 19u32, 62u32, 89u32, 91u32, 97u32, 12u32, 13u32, 18u32, 29u32, 84u32,
        91u32, 88u32, 98u32, 14u32, 19u32, 29u32, 93u32, 95u32, 95u32, 98u32, 97u32, 20u32, 26u32,
        84u32, 88u32, 95u32, 95u32, 98u32, 94u32, 26u32, 86u32, 91u32, 93u32, 97u32, 99u32, 98u32,
        99u32, 99u32, 100u32, 98u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32, 99u32,
        99u32, 99u32, 99u32, 99u32, 97u32, 97u32, 99u32, 99u32, 99u32, 99u32, 97u32, 99u32,
    ],
    [
        10u32, 12u32, 14u32, 19u32, 26u32, 38u32, 57u32, 86u32, 12u32, 18u32, 21u32, 28u32, 35u32,
        41u32, 54u32, 76u32, 14u32, 21u32, 25u32, 32u32, 44u32, 63u32, 92u32, 136u32, 19u32, 28u32,
        32u32, 41u32, 54u32, 75u32, 107u32, 157u32, 26u32, 35u32, 44u32, 54u32, 70u32, 95u32,
        132u32, 190u32, 38u32, 41u32, 63u32, 75u32, 95u32, 125u32, 170u32, 239u32, 57u32, 54u32,
        92u32, 107u32, 132u32, 170u32, 227u32, 312u32, 86u32, 76u32, 136u32, 157u32, 190u32,
        239u32, 312u32, 419u32,
    ],
    [
        7u32, 8u32, 10u32, 14u32, 23u32, 44u32, 95u32, 241u32, 8u32, 8u32, 11u32, 15u32, 25u32,
        47u32, 102u32, 255u32, 10u32, 11u32, 13u32, 19u32, 31u32, 58u32, 127u32, 255u32, 14u32,
        15u32, 19u32, 27u32, 44u32, 83u32, 181u32, 255u32, 23u32, 25u32, 31u32, 44u32, 72u32,
        136u32, 255u32, 255u32, 44u32, 47u32, 58u32, 83u32, 136u32, 255u32, 255u32, 255u32, 95u32,
        102u32, 127u32, 181u32, 255u32, 255u32, 255u32, 255u32, 241u32, 255u32, 255u32, 255u32,
        255u32, 255u32, 255u32, 255u32,
    ],
    [
        15u32, 11u32, 11u32, 12u32, 15u32, 19u32, 25u32, 32u32, 11u32, 13u32, 10u32, 10u32, 12u32,
        15u32, 19u32, 24u32, 11u32, 10u32, 14u32, 14u32, 16u32, 18u32, 22u32, 27u32, 12u32, 10u32,
        14u32, 18u32, 21u32, 24u32, 28u32, 33u32, 15u32, 12u32, 16u32, 21u32, 26u32, 31u32, 36u32,
        42u32, 19u32, 15u32, 18u32, 24u32, 31u32, 38u32, 45u32, 53u32, 25u32, 19u32, 22u32, 28u32,
        36u32, 45u32, 55u32, 65u32, 32u32, 24u32, 27u32, 33u32, 42u32, 53u32, 65u32, 77u32,
    ],
    [
        14u32, 10u32, 11u32, 14u32, 19u32, 25u32, 34u32, 45u32, 10u32, 11u32, 11u32, 12u32, 15u32,
        20u32, 26u32, 33u32, 11u32, 11u32, 15u32, 18u32, 21u32, 25u32, 31u32, 38u32, 14u32, 12u32,
        18u32, 24u32, 28u32, 33u32, 39u32, 47u32, 19u32, 15u32, 21u32, 28u32, 36u32, 43u32, 51u32,
        59u32, 25u32, 20u32, 25u32, 33u32, 43u32, 54u32, 64u32, 74u32, 34u32, 26u32, 31u32, 39u32,
        51u32, 64u32, 77u32, 91u32, 45u32, 33u32, 38u32, 47u32, 59u32, 74u32, 91u32, 108u32,
    ],
];
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_linear_quality(
    mut cinfo: j_compress_ptr,
    mut scale_factor: c_int,
    mut force_baseline: boolean,
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

pub unsafe extern "C" fn jpeg_quality_scaling(mut quality: c_int) -> c_int {
    return jpeg_float_quality_scaling(quality as c_float) as c_int;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_float_quality_scaling(mut quality: c_float) -> c_float
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
    mut cinfo: j_compress_ptr,
    mut quality: c_int,
    mut force_baseline: boolean,
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

pub unsafe extern "C" fn jpeg_set_defaults(mut cinfo: j_compress_ptr) {
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
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
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            MAX_COMPONENTS as c_ulong * ::std::mem::size_of::<jpeg_component_info>() as c_ulong,
        ) as *mut jpeg_component_info
    }
    /* Initialize everything not dependent on the color space */
    (*cinfo).data_precision = BITS_IN_JSAMPLE;
    /* Set up two quantization tables using default quality of 75 */
    jpeg_set_quality(cinfo, 75i32, TRUE);
    /* Set up two Huffman tables */
    std_huff_tables(cinfo as j_common_ptr);
    let mut i: c_int = 0i32;
    while i < NUM_ARITH_TBLS {
        (*cinfo).arith_dc_L[i as usize] = 0u8;
        (*cinfo).arith_dc_U[i as usize] = 1u8;
        (*cinfo).arith_ac_K[i as usize] = 5u8;
        i += 1
    }
    /* Default is no multiple-scan output */
    (*cinfo).scan_info = NULL as *const jpeg_scan_info;
    (*cinfo).num_scans = 0i32;
    /* Expect normal source image, not raw downsampled data */
    (*cinfo).raw_data_in = FALSE;
    /* Use Huffman coding, not arithmetic coding, by default */
    (*cinfo).arith_code = FALSE;
    if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
        /* By default, do extra passes to optimize entropy coding */
        (*cinfo).optimize_coding = TRUE
    } else {
        /* By default, don't do extra passes to optimize entropy coding */
        (*cinfo).optimize_coding = FALSE
    }
    /* The standard Huffman tables are only valid for 8-bit data precision.
     * If the precision is higher, force optimization on so that usable
     * tables will be computed.  This test can be removed if default tables
     * are supplied that are valid for the desired precision.
     */
    if (*cinfo).data_precision > 8i32 {
        (*cinfo).optimize_coding = TRUE
    }
    /* By default, use the simpler non-cosited sampling alignment */
    (*cinfo).CCIR601_sampling = FALSE;
    (*(*cinfo).master).overshoot_deringing =
        ((*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int) as c_int;
    /* No input smoothing */
    (*cinfo).smoothing_factor = 0i32;
    /* DCT algorithm preference */
    (*cinfo).dct_method = JDCT_DEFAULT as J_DCT_METHOD;
    /* No restart markers */
    (*cinfo).restart_interval = 0u32;
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
    (*cinfo).JFIF_major_version = 1u8; /* Default JFIF version = 1.01 */
    (*cinfo).JFIF_minor_version = 1u8; /* Pixel size is unknown by default */
    (*cinfo).density_unit = 0u8; /* Pixel aspect ratio is square by default */
    (*cinfo).X_density = 1u16;
    (*cinfo).Y_density = 1u16;
    /* Choose JPEG colorspace based on input space, set defaults accordingly */
    jpeg_default_colorspace(cinfo);
    (*(*cinfo).master).dc_scan_opt_mode = 1i32;
    if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
        (*(*cinfo).master).optimize_scans = TRUE;
        jpeg_simple_progression(cinfo);
    } else {
        (*(*cinfo).master).optimize_scans = FALSE
    }
    (*(*cinfo).master).trellis_quant =
        ((*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int) as c_int;
    (*(*cinfo).master).lambda_log_scale1 = 14.75f32;
    (*(*cinfo).master).lambda_log_scale2 = 16.5f32;
    (*(*cinfo).master).quant_tbl_master_idx =
        if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
            3i32
        } else {
            0i32
        };
    (*(*cinfo).master).use_lambda_weight_tbl = TRUE;
    (*(*cinfo).master).use_scans_in_trellis = FALSE;
    (*(*cinfo).master).trellis_freq_split = 8i32;
    (*(*cinfo).master).trellis_num_loops = 1i32;
    (*(*cinfo).master).trellis_q_opt = FALSE;
    (*(*cinfo).master).trellis_quant_dc = TRUE;
    (*(*cinfo).master).trellis_delta_dc_weight = 0f32;
}
/*
 * Select an appropriate JPEG colorspace for in_color_space.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_default_colorspace(mut cinfo: j_compress_ptr) {
    match (*cinfo).in_color_space {
        1 => {
            jpeg_set_colorspace(cinfo, JCS_GRAYSCALE); /* By default, no translation */
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            jpeg_set_colorspace(cinfo, JCS_YCbCr);
        }
        3 => {
            jpeg_set_colorspace(cinfo, JCS_YCbCr);
        }
        4 => {
            jpeg_set_colorspace(cinfo, JCS_CMYK);
        }
        5 => {
            jpeg_set_colorspace(cinfo, JCS_YCCK);
        }
        0 => {
            jpeg_set_colorspace(cinfo, JCS_UNKNOWN);
        }
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}
/* Compression parameter setup aids */
/*
 * Set the JPEG colorspace, and choose colorspace-dependent default values.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_colorspace(
    mut cinfo: j_compress_ptr,
    mut colorspace: J_COLOR_SPACE,
) {
    let mut compptr: *mut jpeg_component_info = ::std::ptr::null_mut::<jpeg_component_info>();
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For all colorspaces, we use Q and Huff tables 0 for luminance components,
     * tables 1 for chrominance components.
     */
    (*cinfo).jpeg_color_space = colorspace; /* No marker for non-JFIF colorspaces */
    (*cinfo).write_JFIF_header = FALSE; /* write no Adobe marker by default */
    (*cinfo).write_Adobe_marker = FALSE; /* Write a JFIF marker */
    match colorspace {
        1 => {
            (*cinfo).write_JFIF_header = TRUE;
            (*cinfo).num_components = 1i32;
            /* JFIF specifies component ID 1 */
            compptr = &mut *(*cinfo).comp_info.offset(0) as *mut jpeg_component_info; /* write Adobe marker to flag RGB */
            (*compptr).component_id = 1i32; /* Write a JFIF marker */
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        2 => {
            (*cinfo).write_Adobe_marker = TRUE;
            (*cinfo).num_components = 3i32;
            compptr = &mut *(*cinfo).comp_info.offset(0) as *mut jpeg_component_info;
            (*compptr).component_id = 0x52i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1) as *mut jpeg_component_info;
            (*compptr).component_id = 0x47i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(2) as *mut jpeg_component_info;
            (*compptr).component_id = 0x42i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        3 => {
            (*cinfo).write_JFIF_header = TRUE;
            (*cinfo).num_components = 3i32;
            /* JFIF specifies component IDs 1,2,3 */
            /* We default to 2x2 subsamples of chrominance */
            compptr = &mut *(*cinfo).comp_info.offset(0) as *mut jpeg_component_info; /* write Adobe marker to flag CMYK */
            (*compptr).component_id = 1i32; /* write Adobe marker to flag YCCK */
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1) as *mut jpeg_component_info;
            (*compptr).component_id = 2i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr = &mut *(*cinfo).comp_info.offset(2) as *mut jpeg_component_info;
            (*compptr).component_id = 3i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32
        }
        4 => {
            (*cinfo).write_Adobe_marker = TRUE;
            (*cinfo).num_components = 4i32;
            compptr = &mut *(*cinfo).comp_info.offset(0) as *mut jpeg_component_info;
            (*compptr).component_id = 0x43i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1) as *mut jpeg_component_info;
            (*compptr).component_id = 0x4di32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(2) as *mut jpeg_component_info;
            (*compptr).component_id = 0x59i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(3) as *mut jpeg_component_info;
            (*compptr).component_id = 0x4bi32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        5 => {
            (*cinfo).write_Adobe_marker = TRUE;
            (*cinfo).num_components = 4i32;
            compptr = &mut *(*cinfo).comp_info.offset(0) as *mut jpeg_component_info;
            (*compptr).component_id = 1i32;
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1) as *mut jpeg_component_info;
            (*compptr).component_id = 2i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr = &mut *(*cinfo).comp_info.offset(2) as *mut jpeg_component_info;
            (*compptr).component_id = 3i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr = &mut *(*cinfo).comp_info.offset(3) as *mut jpeg_component_info;
            (*compptr).component_id = 4i32;
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        0 => {
            (*cinfo).num_components = (*cinfo).input_components;
            if (*cinfo).num_components < 1i32 || (*cinfo).num_components > MAX_COMPONENTS {
                (*(*cinfo).err).msg_code = super::jerror::JERR_COMPONENT_COUNT as c_int;
                (*(*cinfo).err).msg_parm.i[0] = (*cinfo).num_components;
                (*(*cinfo).err).msg_parm.i[1] = 10i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            let mut ci: c_int = 0i32;
            while ci < (*cinfo).num_components {
                compptr = &mut *(*cinfo).comp_info.offset(ci as isize) as *mut jpeg_component_info;
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
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}

unsafe extern "C" fn fill_a_scan(
    mut scanptr: *mut jpeg_scan_info,
    mut ci: c_int,
    mut Ss: c_int,
    mut Se: c_int,
    mut Ah: c_int,
    mut Al: c_int,
) -> *mut jpeg_scan_info
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
    mut scanptr: *mut jpeg_scan_info,
    mut ci: c_int,
    mut Ss: c_int,
    mut Se: c_int,
    mut Ah: c_int,
    mut Al: c_int,
) -> *mut jpeg_scan_info
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
    mut scanptr: *mut jpeg_scan_info,
    mut ncomps: c_int,
    mut Ss: c_int,
    mut Se: c_int,
    mut Ah: c_int,
    mut Al: c_int,
) -> *mut jpeg_scan_info
/* Support routine: generate one scan for each component */ {
    let mut ci: c_int = 0i32;
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
    mut scanptr: *mut jpeg_scan_info,
    mut ncomps: c_int,
    mut Ah: c_int,
    mut Al: c_int,
) -> *mut jpeg_scan_info
/* Support routine: generate interleaved DC scan if possible, else N scans */ {
    if ncomps <= MAX_COMPS_IN_SCAN {
        (*scanptr).comps_in_scan = ncomps;
        let mut ci: c_int = 0i32;
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

unsafe extern "C" fn jpeg_search_progression(mut cinfo: j_compress_ptr) -> boolean {
    let mut nscans: c_int = 0;
    let mut frequency_split: [c_int; 5] = [2i32, 8i32, 5i32, 12i32, 18i32];
    let mut ncomps: c_int = (*cinfo).num_components;

    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Figure space needed for script.  Calculation must match code below! */
    if ncomps == 3i32 && (*cinfo).jpeg_color_space == JCS_YCbCr {
        /* Custom script for YCbCr color images. */
        nscans = 64i32
    } else if ncomps == 1i32 {
        nscans = 23i32
    } else {
        (*(*cinfo).master).num_scans_luma = 0i32;
        return FALSE;
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
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            (*cinfo).script_space_size as c_ulong
                * ::std::mem::size_of::<jpeg_scan_info>() as c_ulong,
        ) as *mut jpeg_scan_info
    }
    let mut scanptr: *mut jpeg_scan_info = (*cinfo).script_space;
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
    let mut Al: c_int = 0i32;
    while Al < (*(*cinfo).master).Al_max_luma {
        scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, Al + 1i32, Al);
        scanptr = fill_a_scan(scanptr, 0i32, 1i32, 8i32, 0i32, Al + 1i32);
        scanptr = fill_a_scan(scanptr, 0i32, 9i32, 63i32, 0i32, Al + 1i32);
        Al += 1
    }
    scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 0i32, 0i32);
    let mut i: c_int = 0i32;
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
    return TRUE;
}
/*
 * Create a recommended progressive-JPEG script.
 * cinfo->num_components and cinfo->jpeg_color_space must be correct.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_simple_progression(mut cinfo: j_compress_ptr) {
    let mut nscans: c_int = 0;
    if (*(*cinfo).master).optimize_scans != 0 {
        if jpeg_search_progression(cinfo) == TRUE {
            return;
        }
    }
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    let mut ncomps: c_int = (*cinfo).num_components;
    if ncomps == 3i32 && (*cinfo).jpeg_color_space == JCS_YCbCr {
        /* Custom script for YCbCr color images. */
        if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
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
    } else if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
        if ncomps > MAX_COMPS_IN_SCAN {
            /* All-purpose script for other color spaces. */
            nscans = 5i32 * ncomps
        } else {
            nscans = 1i32 + 4i32 * ncomps
        } /* 2 DC + 4 AC scans per component */
    /* 2 DC scans; 4 AC scans per component */
    } else if ncomps > MAX_COMPS_IN_SCAN {
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
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            (*cinfo).script_space_size as c_ulong
                * ::std::mem::size_of::<jpeg_scan_info>() as c_ulong,
        ) as *mut jpeg_scan_info
    }
    let mut scanptr: *mut jpeg_scan_info = (*cinfo).script_space;
    (*cinfo).scan_info = scanptr;
    (*cinfo).num_scans = nscans;
    if ncomps == 3i32 && (*cinfo).jpeg_color_space == JCS_YCbCr {
        /* Custom script for YCbCr color images. */
        if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
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
    } else if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
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
