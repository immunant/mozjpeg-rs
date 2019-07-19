pub use crate::jconfig_h::BITS_IN_JSAMPLE;
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
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAX_COMPONENTS, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jpeg_c_coef_controller, jpeg_c_main_controller, jpeg_c_prep_controller,
    jpeg_color_converter, jpeg_color_deconverter, jpeg_color_quantizer, jpeg_comp_master,
    jpeg_d_coef_controller, jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master,
    jpeg_downsampler, jpeg_entropy_decoder, jpeg_entropy_encoder, jpeg_forward_dct,
    jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader, jpeg_marker_writer,
    jpeg_upsampler, CSTATE_START, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_alloc_huff_table, jpeg_alloc_quant_table,
    jpeg_common_struct, jpeg_component_info, jpeg_compress_struct, jpeg_decompress_struct,
    jpeg_destination_mgr, jpeg_error_mgr, jpeg_marker_parser_method, jpeg_marker_struct,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_scan_info, jpeg_source_mgr,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_1, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE2, JBLOCK, JBLOCKARRAY, JBLOCKROW,
    JCOEFPTR, JCP_FASTEST, JCP_MAX_COMPRESSION, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
    JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
    JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_DEFAULT,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL,
    JPOOL_PERMANENT, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
    J_DITHER_MODE, MAX_COMPS_IN_SCAN, NUM_ARITH_TBLS, NUM_QUANT_TBLS,
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
) {
    let mut qtblptr: *mut *mut JQUANT_TBL = 0 as *mut *mut JQUANT_TBL;
    let mut i: c_int = 0;
    let mut temp: c_long = 0;
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if which_tbl < 0i32 || which_tbl >= NUM_QUANT_TBLS {
        (*(*cinfo).err).msg_code = JERR_DQT_INDEX as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = which_tbl;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    qtblptr = &mut *(*cinfo)
        .quant_tbl_ptrs
        .as_mut_ptr()
        .offset(which_tbl as isize) as *mut *mut JQUANT_TBL;
    if (*qtblptr).is_null() {
        *qtblptr = jpeg_alloc_quant_table(cinfo as j_common_ptr)
    }
    i = 0i32;
    while i < DCTSIZE2 {
        temp =
            (*basic_table.offset(i as isize) as c_long * scale_factor as c_long + 50i64) / 100i64;
        if temp <= 0i64 {
            temp = 1i64
        }
        if temp > 32767i64 {
            temp = 32767i64
        }
        if 0 != force_baseline && temp > 255i64 {
            temp = 255i64
        }
        (**qtblptr).quantval[i as usize] = temp as UINT16;
        i += 1
    }
    (**qtblptr).sent_table = FALSE;
}
/* These are the sample quantization tables given in Annex K (Clause K.1) of
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
 * The spec says that the values given produce "good" quality, and
 * when divided by 2, "very good" quality.
 */
static mut std_luminance_quant_tbl: [[c_uint; 64]; 9] = [
    [
        16i32 as c_uint,
        11i32 as c_uint,
        10i32 as c_uint,
        16i32 as c_uint,
        24i32 as c_uint,
        40i32 as c_uint,
        51i32 as c_uint,
        61i32 as c_uint,
        12i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        26i32 as c_uint,
        58i32 as c_uint,
        60i32 as c_uint,
        55i32 as c_uint,
        14i32 as c_uint,
        13i32 as c_uint,
        16i32 as c_uint,
        24i32 as c_uint,
        40i32 as c_uint,
        57i32 as c_uint,
        69i32 as c_uint,
        56i32 as c_uint,
        14i32 as c_uint,
        17i32 as c_uint,
        22i32 as c_uint,
        29i32 as c_uint,
        51i32 as c_uint,
        87i32 as c_uint,
        80i32 as c_uint,
        62i32 as c_uint,
        18i32 as c_uint,
        22i32 as c_uint,
        37i32 as c_uint,
        56i32 as c_uint,
        68i32 as c_uint,
        109i32 as c_uint,
        103i32 as c_uint,
        77i32 as c_uint,
        24i32 as c_uint,
        35i32 as c_uint,
        55i32 as c_uint,
        64i32 as c_uint,
        81i32 as c_uint,
        104i32 as c_uint,
        113i32 as c_uint,
        92i32 as c_uint,
        49i32 as c_uint,
        64i32 as c_uint,
        78i32 as c_uint,
        87i32 as c_uint,
        103i32 as c_uint,
        121i32 as c_uint,
        120i32 as c_uint,
        101i32 as c_uint,
        72i32 as c_uint,
        92i32 as c_uint,
        95i32 as c_uint,
        98i32 as c_uint,
        112i32 as c_uint,
        100i32 as c_uint,
        103i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
    ],
    [
        12i32 as c_uint,
        17i32 as c_uint,
        20i32 as c_uint,
        21i32 as c_uint,
        30i32 as c_uint,
        34i32 as c_uint,
        56i32 as c_uint,
        63i32 as c_uint,
        18i32 as c_uint,
        20i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        28i32 as c_uint,
        51i32 as c_uint,
        61i32 as c_uint,
        55i32 as c_uint,
        19i32 as c_uint,
        20i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        33i32 as c_uint,
        58i32 as c_uint,
        69i32 as c_uint,
        55i32 as c_uint,
        26i32 as c_uint,
        26i32 as c_uint,
        26i32 as c_uint,
        30i32 as c_uint,
        46i32 as c_uint,
        87i32 as c_uint,
        86i32 as c_uint,
        66i32 as c_uint,
        31i32 as c_uint,
        33i32 as c_uint,
        36i32 as c_uint,
        40i32 as c_uint,
        46i32 as c_uint,
        96i32 as c_uint,
        100i32 as c_uint,
        73i32 as c_uint,
        40i32 as c_uint,
        35i32 as c_uint,
        46i32 as c_uint,
        62i32 as c_uint,
        81i32 as c_uint,
        100i32 as c_uint,
        111i32 as c_uint,
        91i32 as c_uint,
        46i32 as c_uint,
        66i32 as c_uint,
        76i32 as c_uint,
        86i32 as c_uint,
        102i32 as c_uint,
        121i32 as c_uint,
        120i32 as c_uint,
        101i32 as c_uint,
        68i32 as c_uint,
        90i32 as c_uint,
        90i32 as c_uint,
        96i32 as c_uint,
        113i32 as c_uint,
        102i32 as c_uint,
        105i32 as c_uint,
        103i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        37i32 as c_uint,
        56i32 as c_uint,
        85i32 as c_uint,
        16i32 as c_uint,
        17i32 as c_uint,
        20i32 as c_uint,
        27i32 as c_uint,
        34i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        75i32 as c_uint,
        16i32 as c_uint,
        20i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        43i32 as c_uint,
        62i32 as c_uint,
        91i32 as c_uint,
        135i32 as c_uint,
        18i32 as c_uint,
        27i32 as c_uint,
        31i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        74i32 as c_uint,
        106i32 as c_uint,
        156i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        43i32 as c_uint,
        53i32 as c_uint,
        69i32 as c_uint,
        94i32 as c_uint,
        131i32 as c_uint,
        189i32 as c_uint,
        37i32 as c_uint,
        40i32 as c_uint,
        62i32 as c_uint,
        74i32 as c_uint,
        94i32 as c_uint,
        124i32 as c_uint,
        169i32 as c_uint,
        238i32 as c_uint,
        56i32 as c_uint,
        53i32 as c_uint,
        91i32 as c_uint,
        106i32 as c_uint,
        131i32 as c_uint,
        169i32 as c_uint,
        226i32 as c_uint,
        311i32 as c_uint,
        85i32 as c_uint,
        75i32 as c_uint,
        135i32 as c_uint,
        156i32 as c_uint,
        189i32 as c_uint,
        238i32 as c_uint,
        311i32 as c_uint,
        418i32 as c_uint,
    ],
    [
        9i32 as c_uint,
        10i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        27i32 as c_uint,
        32i32 as c_uint,
        51i32 as c_uint,
        62i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        27i32 as c_uint,
        44i32 as c_uint,
        59i32 as c_uint,
        73i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        42i32 as c_uint,
        59i32 as c_uint,
        79i32 as c_uint,
        78i32 as c_uint,
        17i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        42i32 as c_uint,
        61i32 as c_uint,
        92i32 as c_uint,
        87i32 as c_uint,
        92i32 as c_uint,
        23i32 as c_uint,
        28i32 as c_uint,
        42i32 as c_uint,
        75i32 as c_uint,
        79i32 as c_uint,
        112i32 as c_uint,
        112i32 as c_uint,
        99i32 as c_uint,
        40i32 as c_uint,
        42i32 as c_uint,
        59i32 as c_uint,
        84i32 as c_uint,
        88i32 as c_uint,
        124i32 as c_uint,
        132i32 as c_uint,
        111i32 as c_uint,
        42i32 as c_uint,
        64i32 as c_uint,
        78i32 as c_uint,
        95i32 as c_uint,
        105i32 as c_uint,
        126i32 as c_uint,
        125i32 as c_uint,
        99i32 as c_uint,
        70i32 as c_uint,
        75i32 as c_uint,
        100i32 as c_uint,
        102i32 as c_uint,
        116i32 as c_uint,
        100i32 as c_uint,
        107i32 as c_uint,
        98i32 as c_uint,
    ],
    [
        10i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        26i32 as c_uint,
        38i32 as c_uint,
        57i32 as c_uint,
        86i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        35i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        76i32 as c_uint,
        14i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        44i32 as c_uint,
        63i32 as c_uint,
        92i32 as c_uint,
        136i32 as c_uint,
        19i32 as c_uint,
        28i32 as c_uint,
        32i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        75i32 as c_uint,
        107i32 as c_uint,
        157i32 as c_uint,
        26i32 as c_uint,
        35i32 as c_uint,
        44i32 as c_uint,
        54i32 as c_uint,
        70i32 as c_uint,
        95i32 as c_uint,
        132i32 as c_uint,
        190i32 as c_uint,
        38i32 as c_uint,
        41i32 as c_uint,
        63i32 as c_uint,
        75i32 as c_uint,
        95i32 as c_uint,
        125i32 as c_uint,
        170i32 as c_uint,
        239i32 as c_uint,
        57i32 as c_uint,
        54i32 as c_uint,
        92i32 as c_uint,
        107i32 as c_uint,
        132i32 as c_uint,
        170i32 as c_uint,
        227i32 as c_uint,
        312i32 as c_uint,
        86i32 as c_uint,
        76i32 as c_uint,
        136i32 as c_uint,
        157i32 as c_uint,
        190i32 as c_uint,
        239i32 as c_uint,
        312i32 as c_uint,
        419i32 as c_uint,
    ],
    [
        7i32 as c_uint,
        8i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        23i32 as c_uint,
        44i32 as c_uint,
        95i32 as c_uint,
        241i32 as c_uint,
        8i32 as c_uint,
        8i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        25i32 as c_uint,
        47i32 as c_uint,
        102i32 as c_uint,
        255i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        19i32 as c_uint,
        31i32 as c_uint,
        58i32 as c_uint,
        127i32 as c_uint,
        255i32 as c_uint,
        14i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        27i32 as c_uint,
        44i32 as c_uint,
        83i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        23i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        44i32 as c_uint,
        72i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        44i32 as c_uint,
        47i32 as c_uint,
        58i32 as c_uint,
        83i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        95i32 as c_uint,
        102i32 as c_uint,
        127i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        241i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
    ],
    [
        15i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        10i32 as c_uint,
        10i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        24i32 as c_uint,
        11i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        14i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        22i32 as c_uint,
        27i32 as c_uint,
        12i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        15i32 as c_uint,
        12i32 as c_uint,
        16i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        36i32 as c_uint,
        42i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        45i32 as c_uint,
        53i32 as c_uint,
        25i32 as c_uint,
        19i32 as c_uint,
        22i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        45i32 as c_uint,
        55i32 as c_uint,
        65i32 as c_uint,
        32i32 as c_uint,
        24i32 as c_uint,
        27i32 as c_uint,
        33i32 as c_uint,
        42i32 as c_uint,
        53i32 as c_uint,
        65i32 as c_uint,
        77i32 as c_uint,
    ],
    [
        14i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        45i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        33i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        14i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        39i32 as c_uint,
        47i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        43i32 as c_uint,
        51i32 as c_uint,
        59i32 as c_uint,
        25i32 as c_uint,
        20i32 as c_uint,
        25i32 as c_uint,
        33i32 as c_uint,
        43i32 as c_uint,
        54i32 as c_uint,
        64i32 as c_uint,
        74i32 as c_uint,
        34i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        39i32 as c_uint,
        51i32 as c_uint,
        64i32 as c_uint,
        77i32 as c_uint,
        91i32 as c_uint,
        45i32 as c_uint,
        33i32 as c_uint,
        38i32 as c_uint,
        47i32 as c_uint,
        59i32 as c_uint,
        74i32 as c_uint,
        91i32 as c_uint,
        108i32 as c_uint,
    ],
];
/* JPEG Annex K
 */
/* flat
 */
/* From http://www.imagemagick.org/discourse-server/viewtopic.php?f=22&t=20333&p=98008#p98008
 */
/* Relevance of human vision to JPEG-DCT compression (1992) Klein, Silverstein and Carney.
 */
/* DCTune perceptual optimization of compressed dental X-Rays (1997) Watson, Taylor, Borthwick
 */
/* A visual detection model for DCT coefficient quantization (12/9/93) Ahumada, Watson, Peterson
 */
/* An improved detection model for DCT coefficient quantization (1993) Peterson, Ahumada and Watson
 */
static mut std_chrominance_quant_tbl: [[c_uint; 64]; 9] = [
    [
        17i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        47i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        66i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        24i32 as c_uint,
        26i32 as c_uint,
        56i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        47i32 as c_uint,
        66i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
    ],
    [
        8i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        15i32 as c_uint,
        86i32 as c_uint,
        96i32 as c_uint,
        96i32 as c_uint,
        98i32 as c_uint,
        13i32 as c_uint,
        13i32 as c_uint,
        15i32 as c_uint,
        26i32 as c_uint,
        90i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        98i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        17i32 as c_uint,
        16i32 as c_uint,
        90i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        96i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        37i32 as c_uint,
        56i32 as c_uint,
        85i32 as c_uint,
        16i32 as c_uint,
        17i32 as c_uint,
        20i32 as c_uint,
        27i32 as c_uint,
        34i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        75i32 as c_uint,
        16i32 as c_uint,
        20i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        43i32 as c_uint,
        62i32 as c_uint,
        91i32 as c_uint,
        135i32 as c_uint,
        18i32 as c_uint,
        27i32 as c_uint,
        31i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        74i32 as c_uint,
        106i32 as c_uint,
        156i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        43i32 as c_uint,
        53i32 as c_uint,
        69i32 as c_uint,
        94i32 as c_uint,
        131i32 as c_uint,
        189i32 as c_uint,
        37i32 as c_uint,
        40i32 as c_uint,
        62i32 as c_uint,
        74i32 as c_uint,
        94i32 as c_uint,
        124i32 as c_uint,
        169i32 as c_uint,
        238i32 as c_uint,
        56i32 as c_uint,
        53i32 as c_uint,
        91i32 as c_uint,
        106i32 as c_uint,
        131i32 as c_uint,
        169i32 as c_uint,
        226i32 as c_uint,
        311i32 as c_uint,
        85i32 as c_uint,
        75i32 as c_uint,
        135i32 as c_uint,
        156i32 as c_uint,
        189i32 as c_uint,
        238i32 as c_uint,
        311i32 as c_uint,
        418i32 as c_uint,
    ],
    [
        9i32 as c_uint,
        10i32 as c_uint,
        17i32 as c_uint,
        19i32 as c_uint,
        62i32 as c_uint,
        89i32 as c_uint,
        91i32 as c_uint,
        97i32 as c_uint,
        12i32 as c_uint,
        13i32 as c_uint,
        18i32 as c_uint,
        29i32 as c_uint,
        84i32 as c_uint,
        91i32 as c_uint,
        88i32 as c_uint,
        98i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        29i32 as c_uint,
        93i32 as c_uint,
        95i32 as c_uint,
        95i32 as c_uint,
        98i32 as c_uint,
        97i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        84i32 as c_uint,
        88i32 as c_uint,
        95i32 as c_uint,
        95i32 as c_uint,
        98i32 as c_uint,
        94i32 as c_uint,
        26i32 as c_uint,
        86i32 as c_uint,
        91i32 as c_uint,
        93i32 as c_uint,
        97i32 as c_uint,
        99i32 as c_uint,
        98i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        100i32 as c_uint,
        98i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        97i32 as c_uint,
        97i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        97i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        10i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        26i32 as c_uint,
        38i32 as c_uint,
        57i32 as c_uint,
        86i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        35i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        76i32 as c_uint,
        14i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        44i32 as c_uint,
        63i32 as c_uint,
        92i32 as c_uint,
        136i32 as c_uint,
        19i32 as c_uint,
        28i32 as c_uint,
        32i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        75i32 as c_uint,
        107i32 as c_uint,
        157i32 as c_uint,
        26i32 as c_uint,
        35i32 as c_uint,
        44i32 as c_uint,
        54i32 as c_uint,
        70i32 as c_uint,
        95i32 as c_uint,
        132i32 as c_uint,
        190i32 as c_uint,
        38i32 as c_uint,
        41i32 as c_uint,
        63i32 as c_uint,
        75i32 as c_uint,
        95i32 as c_uint,
        125i32 as c_uint,
        170i32 as c_uint,
        239i32 as c_uint,
        57i32 as c_uint,
        54i32 as c_uint,
        92i32 as c_uint,
        107i32 as c_uint,
        132i32 as c_uint,
        170i32 as c_uint,
        227i32 as c_uint,
        312i32 as c_uint,
        86i32 as c_uint,
        76i32 as c_uint,
        136i32 as c_uint,
        157i32 as c_uint,
        190i32 as c_uint,
        239i32 as c_uint,
        312i32 as c_uint,
        419i32 as c_uint,
    ],
    [
        7i32 as c_uint,
        8i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        23i32 as c_uint,
        44i32 as c_uint,
        95i32 as c_uint,
        241i32 as c_uint,
        8i32 as c_uint,
        8i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        25i32 as c_uint,
        47i32 as c_uint,
        102i32 as c_uint,
        255i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        19i32 as c_uint,
        31i32 as c_uint,
        58i32 as c_uint,
        127i32 as c_uint,
        255i32 as c_uint,
        14i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        27i32 as c_uint,
        44i32 as c_uint,
        83i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        23i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        44i32 as c_uint,
        72i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        44i32 as c_uint,
        47i32 as c_uint,
        58i32 as c_uint,
        83i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        95i32 as c_uint,
        102i32 as c_uint,
        127i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        241i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
    ],
    [
        15i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        10i32 as c_uint,
        10i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        24i32 as c_uint,
        11i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        14i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        22i32 as c_uint,
        27i32 as c_uint,
        12i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        15i32 as c_uint,
        12i32 as c_uint,
        16i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        36i32 as c_uint,
        42i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        45i32 as c_uint,
        53i32 as c_uint,
        25i32 as c_uint,
        19i32 as c_uint,
        22i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        45i32 as c_uint,
        55i32 as c_uint,
        65i32 as c_uint,
        32i32 as c_uint,
        24i32 as c_uint,
        27i32 as c_uint,
        33i32 as c_uint,
        42i32 as c_uint,
        53i32 as c_uint,
        65i32 as c_uint,
        77i32 as c_uint,
    ],
    [
        14i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        45i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        33i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        14i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        39i32 as c_uint,
        47i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        43i32 as c_uint,
        51i32 as c_uint,
        59i32 as c_uint,
        25i32 as c_uint,
        20i32 as c_uint,
        25i32 as c_uint,
        33i32 as c_uint,
        43i32 as c_uint,
        54i32 as c_uint,
        64i32 as c_uint,
        74i32 as c_uint,
        34i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        39i32 as c_uint,
        51i32 as c_uint,
        64i32 as c_uint,
        77i32 as c_uint,
        91i32 as c_uint,
        45i32 as c_uint,
        33i32 as c_uint,
        38i32 as c_uint,
        47i32 as c_uint,
        59i32 as c_uint,
        74i32 as c_uint,
        91i32 as c_uint,
        108i32 as c_uint,
    ],
];
/* JPEG Annex K
 */
/* flat
 */
/* From http://www.imagemagick.org/discourse-server/viewtopic.php?f=22&t=20333&p=98008#p98008
 */
/* Relevance of human vision to JPEG-DCT compression (1992) Klein, Silverstein and Carney.
 * Copied from luma
 */
/* DCTune perceptual optimization of compressed dental X-Rays (1997) Watson, Taylor, Borthwick
 * Copied from luma
 */
/* A visual detection model for DCT coefficient quantization (12/9/93) Ahumada, Watson, Peterson
 * Copied from luma
 */
/* An improved detection model for DCT coefficient quantization (1993) Peterson, Ahumada and Watson
 * Copied from luma
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_set_linear_quality(
    mut cinfo: j_compress_ptr,
    mut scale_factor: c_int,
    mut force_baseline: boolean,
) {
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
pub unsafe extern "C" fn jpeg_float_quality_scaling(mut quality: c_float) -> c_float {
    if quality <= 0.0f32 {
        quality = 1.0f32
    }
    if quality > 100.0f32 {
        quality = 100.0f32
    }
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
) {
    quality = jpeg_quality_scaling(quality);
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
    let mut i: c_int = 0;
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).comp_info.is_null() {
        (*cinfo).comp_info = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            (MAX_COMPONENTS as c_ulong)
                .wrapping_mul(::std::mem::size_of::<jpeg_component_info>() as c_ulong),
        ) as *mut jpeg_component_info
    }
    (*cinfo).data_precision = BITS_IN_JSAMPLE;
    jpeg_set_quality(cinfo, 75i32, TRUE);
    std_huff_tables(cinfo as j_common_ptr);
    i = 0i32;
    while i < NUM_ARITH_TBLS {
        (*cinfo).arith_dc_L[i as usize] = 0i32 as UINT8;
        (*cinfo).arith_dc_U[i as usize] = 1i32 as UINT8;
        (*cinfo).arith_ac_K[i as usize] = 5i32 as UINT8;
        i += 1
    }
    (*cinfo).scan_info = NULL as *const jpeg_scan_info;
    (*cinfo).num_scans = 0i32;
    (*cinfo).raw_data_in = FALSE;
    (*cinfo).arith_code = FALSE;
    if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
        (*cinfo).optimize_coding = TRUE
    } else {
        (*cinfo).optimize_coding = FALSE
    }
    if (*cinfo).data_precision > 8i32 {
        (*cinfo).optimize_coding = TRUE
    }
    (*cinfo).CCIR601_sampling = FALSE;
    (*(*cinfo).master).overshoot_deringing =
        ((*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int) as c_int;
    (*cinfo).smoothing_factor = 0i32;
    (*cinfo).dct_method = JDCT_DEFAULT as J_DCT_METHOD;
    (*cinfo).restart_interval = 0i32 as c_uint;
    (*cinfo).restart_in_rows = 0i32;
    (*cinfo).JFIF_major_version = 1i32 as UINT8;
    (*cinfo).JFIF_minor_version = 1i32 as UINT8;
    (*cinfo).density_unit = 0i32 as UINT8;
    (*cinfo).X_density = 1i32 as UINT16;
    (*cinfo).Y_density = 1i32 as UINT16;
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
    (*(*cinfo).master).lambda_log_scale1 = 14.75f64 as c_float;
    (*(*cinfo).master).lambda_log_scale2 = 16.5f64 as c_float;
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
    (*(*cinfo).master).trellis_delta_dc_weight = 0.0f64 as c_float;
}
/*
 * Select an appropriate JPEG colorspace for in_color_space.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_default_colorspace(mut cinfo: j_compress_ptr) {
    match (*cinfo).in_color_space as c_uint {
        1 => {
            jpeg_set_colorspace(cinfo, JCS_GRAYSCALE);
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
            (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
            (*(*cinfo).err)
                .error_exit
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
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut ci: c_int = 0;
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cinfo).jpeg_color_space = colorspace;
    (*cinfo).write_JFIF_header = FALSE;
    (*cinfo).write_Adobe_marker = FALSE;
    match colorspace as c_uint {
        1 => {
            (*cinfo).write_JFIF_header = TRUE;
            (*cinfo).num_components = 1i32;
            compptr = &mut *(*cinfo).comp_info.offset(0isize) as *mut jpeg_component_info;
            (*compptr).component_id = 1i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32
        }
        2 => {
            (*cinfo).write_Adobe_marker = TRUE;
            (*cinfo).num_components = 3i32;
            compptr = &mut *(*cinfo).comp_info.offset(0isize) as *mut jpeg_component_info;
            (*compptr).component_id = 0x52i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1isize) as *mut jpeg_component_info;
            (*compptr).component_id = 0x47i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(2isize) as *mut jpeg_component_info;
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
            compptr = &mut *(*cinfo).comp_info.offset(0isize) as *mut jpeg_component_info;
            (*compptr).component_id = 1i32;
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1isize) as *mut jpeg_component_info;
            (*compptr).component_id = 2i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr = &mut *(*cinfo).comp_info.offset(2isize) as *mut jpeg_component_info;
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
            compptr = &mut *(*cinfo).comp_info.offset(0isize) as *mut jpeg_component_info;
            (*compptr).component_id = 0x43i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1isize) as *mut jpeg_component_info;
            (*compptr).component_id = 0x4di32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(2isize) as *mut jpeg_component_info;
            (*compptr).component_id = 0x59i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(3isize) as *mut jpeg_component_info;
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
            compptr = &mut *(*cinfo).comp_info.offset(0isize) as *mut jpeg_component_info;
            (*compptr).component_id = 1i32;
            (*compptr).h_samp_factor = 2i32;
            (*compptr).v_samp_factor = 2i32;
            (*compptr).quant_tbl_no = 0i32;
            (*compptr).dc_tbl_no = 0i32;
            (*compptr).ac_tbl_no = 0i32;
            compptr = &mut *(*cinfo).comp_info.offset(1isize) as *mut jpeg_component_info;
            (*compptr).component_id = 2i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr = &mut *(*cinfo).comp_info.offset(2isize) as *mut jpeg_component_info;
            (*compptr).component_id = 3i32;
            (*compptr).h_samp_factor = 1i32;
            (*compptr).v_samp_factor = 1i32;
            (*compptr).quant_tbl_no = 1i32;
            (*compptr).dc_tbl_no = 1i32;
            (*compptr).ac_tbl_no = 1i32;
            compptr = &mut *(*cinfo).comp_info.offset(3isize) as *mut jpeg_component_info;
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
                (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).num_components;
                (*(*cinfo).err).msg_parm.i[1usize] = 10i32;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            ci = 0i32;
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
            (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
            (*(*cinfo).err)
                .error_exit
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
) -> *mut jpeg_scan_info {
    (*scanptr).comps_in_scan = 1i32;
    (*scanptr).component_index[0usize] = ci;
    (*scanptr).Ss = Ss;
    (*scanptr).Se = Se;
    (*scanptr).Ah = Ah;
    (*scanptr).Al = Al;
    scanptr = scanptr.offset(1isize);
    return scanptr;
}
unsafe extern "C" fn fill_a_scan_pair(
    mut scanptr: *mut jpeg_scan_info,
    mut ci: c_int,
    mut Ss: c_int,
    mut Se: c_int,
    mut Ah: c_int,
    mut Al: c_int,
) -> *mut jpeg_scan_info {
    (*scanptr).comps_in_scan = 2i32;
    (*scanptr).component_index[0usize] = ci;
    (*scanptr).component_index[1usize] = ci + 1i32;
    (*scanptr).Ss = Ss;
    (*scanptr).Se = Se;
    (*scanptr).Ah = Ah;
    (*scanptr).Al = Al;
    scanptr = scanptr.offset(1isize);
    return scanptr;
}
unsafe extern "C" fn fill_scans(
    mut scanptr: *mut jpeg_scan_info,
    mut ncomps: c_int,
    mut Ss: c_int,
    mut Se: c_int,
    mut Ah: c_int,
    mut Al: c_int,
) -> *mut jpeg_scan_info {
    let mut ci: c_int = 0;
    ci = 0i32;
    while ci < ncomps {
        (*scanptr).comps_in_scan = 1i32;
        (*scanptr).component_index[0usize] = ci;
        (*scanptr).Ss = Ss;
        (*scanptr).Se = Se;
        (*scanptr).Ah = Ah;
        (*scanptr).Al = Al;
        scanptr = scanptr.offset(1isize);
        ci += 1
    }
    return scanptr;
}
unsafe extern "C" fn fill_dc_scans(
    mut scanptr: *mut jpeg_scan_info,
    mut ncomps: c_int,
    mut Ah: c_int,
    mut Al: c_int,
) -> *mut jpeg_scan_info {
    let mut ci: c_int = 0;
    if ncomps <= MAX_COMPS_IN_SCAN {
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
        scanptr = scanptr.offset(1isize)
    } else {
        scanptr = fill_scans(scanptr, ncomps, 0i32, 0i32, Ah, Al)
    }
    return scanptr;
}
/*
 * List of scans to be tested
 * cinfo->num_components and cinfo->jpeg_color_space must be correct.
 */
unsafe extern "C" fn jpeg_search_progression(mut cinfo: j_compress_ptr) -> boolean {
    let mut ncomps: c_int = (*cinfo).num_components;
    let mut nscans: c_int = 0;
    let mut scanptr: *mut jpeg_scan_info = 0 as *mut jpeg_scan_info;
    let mut Al: c_int = 0;
    let mut frequency_split: [c_int; 5] = [2i32, 8i32, 5i32, 12i32, 18i32];
    let mut i: c_int = 0;
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if ncomps == 3i32 && (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
        nscans = 64i32
    } else if ncomps == 1i32 {
        nscans = 23i32
    } else {
        (*(*cinfo).master).num_scans_luma = 0i32;
        return FALSE;
    }
    if (*cinfo).script_space.is_null() || (*cinfo).script_space_size < nscans {
        (*cinfo).script_space_size = if nscans > 64i32 { nscans } else { 64i32 };
        (*cinfo).script_space = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ((*cinfo).script_space_size as c_ulong)
                .wrapping_mul(::std::mem::size_of::<jpeg_scan_info>() as c_ulong),
        ) as *mut jpeg_scan_info
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
        scanptr = fill_a_scan_pair(scanptr, 1i32, 0i32, 0i32, 0i32, 0i32);
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
    let mut ncomps: c_int = 0;
    let mut nscans: c_int = 0;
    let mut scanptr: *mut jpeg_scan_info = 0 as *mut jpeg_scan_info;
    if 0 != (*(*cinfo).master).optimize_scans {
        if jpeg_search_progression(cinfo) == TRUE {
            return;
        }
    }
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ncomps = (*cinfo).num_components;
    if ncomps == 3i32 && (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
        if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
            if (*(*cinfo).master).dc_scan_opt_mode == 0i32 {
                nscans = 9i32
            } else if (*(*cinfo).master).dc_scan_opt_mode == 1i32 {
                nscans = 11i32
            } else {
                nscans = 10i32
            }
        } else {
            nscans = 10i32
        }
    } else if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
        if ncomps > MAX_COMPS_IN_SCAN {
            nscans = 5i32 * ncomps
        } else {
            nscans = 1i32 + 4i32 * ncomps
        }
    } else if ncomps > MAX_COMPS_IN_SCAN {
        nscans = 6i32 * ncomps
    } else {
        nscans = 2i32 + 4i32 * ncomps
    }
    if (*cinfo).script_space.is_null() || (*cinfo).script_space_size < nscans {
        (*cinfo).script_space_size = if nscans > 10i32 { nscans } else { 10i32 };
        (*cinfo).script_space = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ((*cinfo).script_space_size as c_ulong)
                .wrapping_mul(::std::mem::size_of::<jpeg_scan_info>() as c_ulong),
        ) as *mut jpeg_scan_info
    }
    scanptr = (*cinfo).script_space;
    (*cinfo).scan_info = scanptr;
    (*cinfo).num_scans = nscans;
    if ncomps == 3i32 && (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
        if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
            if (*(*cinfo).master).dc_scan_opt_mode == 0i32 {
                scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 0i32)
            } else if (*(*cinfo).master).dc_scan_opt_mode == 1i32 {
                scanptr = fill_a_scan(scanptr, 0i32, 0i32, 0i32, 0i32, 0i32);
                scanptr = fill_a_scan(scanptr, 1i32, 0i32, 0i32, 0i32, 0i32);
                scanptr = fill_a_scan(scanptr, 2i32, 0i32, 0i32, 0i32, 0i32)
            } else {
                scanptr = fill_dc_scans(scanptr, 1i32, 0i32, 0i32);
                scanptr = fill_a_scan_pair(scanptr, 1i32, 0i32, 0i32, 0i32, 0i32)
            }
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 8i32, 0i32, 2i32);
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 8i32, 0i32, 0i32);
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 8i32, 0i32, 0i32);
            scanptr = fill_a_scan(scanptr, 0i32, 9i32, 63i32, 0i32, 2i32);
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 2i32, 1i32);
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 1i32, 0i32);
            scanptr = fill_a_scan(scanptr, 1i32, 9i32, 63i32, 0i32, 0i32);
            scanptr = fill_a_scan(scanptr, 2i32, 9i32, 63i32, 0i32, 0i32)
        } else {
            scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 1i32);
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 5i32, 0i32, 2i32);
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 63i32, 0i32, 1i32);
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 63i32, 0i32, 1i32);
            scanptr = fill_a_scan(scanptr, 0i32, 6i32, 63i32, 0i32, 2i32);
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 2i32, 1i32);
            scanptr = fill_dc_scans(scanptr, ncomps, 1i32, 0i32);
            scanptr = fill_a_scan(scanptr, 2i32, 1i32, 63i32, 1i32, 0i32);
            scanptr = fill_a_scan(scanptr, 1i32, 1i32, 63i32, 1i32, 0i32);
            scanptr = fill_a_scan(scanptr, 0i32, 1i32, 63i32, 1i32, 0i32)
        }
    } else if (*(*cinfo).master).compress_profile == JCP_MAX_COMPRESSION as c_int {
        scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 0i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 8i32, 0i32, 2i32);
        scanptr = fill_scans(scanptr, ncomps, 9i32, 63i32, 0i32, 2i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 2i32, 1i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 1i32, 0i32)
    } else {
        scanptr = fill_dc_scans(scanptr, ncomps, 0i32, 1i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 5i32, 0i32, 2i32);
        scanptr = fill_scans(scanptr, ncomps, 6i32, 63i32, 0i32, 2i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 2i32, 1i32);
        scanptr = fill_dc_scans(scanptr, ncomps, 1i32, 0i32);
        scanptr = fill_scans(scanptr, ncomps, 1i32, 63i32, 1i32, 0i32)
    };
}
