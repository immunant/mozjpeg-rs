pub use crate::jdcol565_c::{
    gray_rgb565D_convert_be, gray_rgb565D_convert_le, gray_rgb565_convert_be,
    gray_rgb565_convert_le, rgb_rgb565D_convert_be, rgb_rgb565D_convert_le, rgb_rgb565_convert_be,
    rgb_rgb565_convert_le, ycc_rgb565D_convert_be, ycc_rgb565D_convert_le, ycc_rgb565_convert_be,
    ycc_rgb565_convert_le,
};
pub use crate::jdcolext_c::{
    gray_extbgr_convert_internal, gray_extbgrx_convert_internal, gray_extrgb_convert_internal,
    gray_extrgbx_convert_internal, gray_extxbgr_convert_internal, gray_extxrgb_convert_internal,
    gray_rgb_convert_internal, rgb_extbgr_convert_internal, rgb_extbgrx_convert_internal,
    rgb_extrgb_convert_internal, rgb_extrgbx_convert_internal, rgb_extxbgr_convert_internal,
    rgb_extxrgb_convert_internal, rgb_rgb_convert_internal, ycc_extbgr_convert_internal,
    ycc_extbgrx_convert_internal, ycc_extrgb_convert_internal, ycc_extrgbx_convert_internal,
    ycc_extxbgr_convert_internal, ycc_extxrgb_convert_internal, ycc_rgb_convert_internal,
};
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
    boolean, rgb_blue, rgb_green, rgb_pixelsize, rgb_red, CENTERJSAMPLE, EXT_BGRX_BLUE,
    EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN,
    EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE,
    EXT_RGBX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_XBGR_BLUE,
    EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN,
    EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, FALSE, INT16, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAXJSAMPLE,
    RGB_BLUE_5, RGB_GREEN_5, RGB_PIXELSIZE_5, RGB_RED_5, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jcopy_sample_rows, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_d_coef_controller, jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master,
    jpeg_entropy_decoder, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader,
    jpeg_upsampler, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_common_struct, jpeg_component_info,
    jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_parser_method, jpeg_marker_struct,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_source_mgr,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL,
    JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
    J_DITHER_MODE,
};
use crate::jsimd::{
    jsimd_can_ycc_rgb, jsimd_can_ycc_rgb565, jsimd_ycc_rgb565_convert, jsimd_ycc_rgb_convert,
};
pub use crate::stddef_h::size_t;
use libc::{self, c_char, c_double, c_int, c_long, c_uint, c_ulong};
pub type my_cconvert_ptr = *mut my_color_deconverter;
/*
 * jdcolor.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2011 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2009, 2011-2012, 2014-2015, D. R. Commander.
 * Copyright (C) 2013, Linaro Limited.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */
/* Private subobject */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_deconverter {
    pub pub_0: jpeg_color_deconverter,
    pub Cr_r_tab: *mut c_int,
    pub Cb_b_tab: *mut c_int,
    pub Cr_g_tab: *mut JLONG,
    pub Cb_g_tab: *mut JLONG,
    pub rgb_y_tab: *mut JLONG,
}
/* *************** YCbCr -> RGB conversion: most common case **************/
/* ***************   RGB -> Y   conversion: less common case **************/
/*
 * YCbCr is defined per CCIR 601-1, except that Cb and Cr are
 * normalized to the range 0..MAXJSAMPLE rather than -0.5 .. 0.5.
 * The conversion equations to be implemented are therefore
 *
 *      R = Y                + 1.40200 * Cr
 *      G = Y - 0.34414 * Cb - 0.71414 * Cr
 *      B = Y + 1.77200 * Cb
 *
 *      Y = 0.29900 * R + 0.58700 * G + 0.11400 * B
 *
 * where Cb and Cr represent the incoming values less CENTERJSAMPLE.
 * (These numbers are derived from TIFF 6.0 section 21, dated 3-June-92.)
 *
 * To avoid floating-point arithmetic, we represent the fractional constants
 * as integers scaled up by 2^16 (about 4 digits precision); we have to divide
 * the products by 2^16, with appropriate rounding, to get the correct answer.
 * Notice that Y, being an integral input, does not contribute any fraction
 * so it need not participate in the rounding.
 *
 * For even more speed, we avoid doing any multiplications in the inner loop
 * by precalculating the constants times Cb and Cr for all possible values.
 * For 8-bit JSAMPLEs this is very reasonable (only 256 entries per table);
 * for 12-bit samples it is still acceptable.  It's not very reasonable for
 * 16-bit samples, but if you want lossless storage you shouldn't be changing
 * colorspace anyway.
 * The Cr=>R and Cb=>B values can be rounded to integers in advance; the
 * values for the G calculation are left scaled up, since we must add them
 * together before rounding.
 */
/* speediest right-shift on some machines */
pub const SCALEBITS: c_int = 16i32;
pub const ONE_HALF: JLONG = (1i32 as JLONG) << SCALEBITS - 1i32;
/* We allocate one big table for RGB->Y conversion and divide it up into
 * three parts, instead of doing three alloc_small requests.  This lets us
 * use a single table base address, which can be held in a register in the
 * inner loops on many machines (more than can hold all three addresses,
 * anyway).
 */
/* offset to R => Y section */
pub const R_Y_OFF: c_int = 0i32;
/* offset to G => Y section */
pub const G_Y_OFF: c_int = 1i32 * (MAXJSAMPLE + 1i32);
/* etc. */
pub const B_Y_OFF: c_int = 2i32 * (MAXJSAMPLE + 1i32);
pub const TABLE_SIZE: c_int = 3i32 * (MAXJSAMPLE + 1i32);
/* Include inline routines for colorspace extensions */
pub const RGB_RED_4: c_int = EXT_RGB_RED;
pub const RGB_GREEN_4: c_int = EXT_RGB_GREEN;
pub const RGB_BLUE_4: c_int = EXT_RGB_BLUE;
pub const RGB_PIXELSIZE_4: c_int = EXT_RGB_PIXELSIZE;
pub const RGB_RED_2: c_int = EXT_RGBX_RED;
pub const RGB_GREEN_2: c_int = EXT_RGBX_GREEN;
pub const RGB_BLUE_2: c_int = EXT_RGBX_BLUE;
pub const RGB_ALPHA_2: c_int = 3i32;
pub const RGB_PIXELSIZE_2: c_int = EXT_RGBX_PIXELSIZE;
pub const RGB_RED_3: c_int = EXT_BGR_RED;
pub const RGB_GREEN_3: c_int = EXT_BGR_GREEN;
pub const RGB_BLUE_3: c_int = EXT_BGR_BLUE;
pub const RGB_PIXELSIZE_3: c_int = EXT_BGR_PIXELSIZE;
pub const RGB_RED_1: c_int = EXT_BGRX_RED;
pub const RGB_GREEN_1: c_int = EXT_BGRX_GREEN;
pub const RGB_BLUE_1: c_int = EXT_BGRX_BLUE;
pub const RGB_ALPHA_1: c_int = 3i32;
pub const RGB_PIXELSIZE_1: c_int = EXT_BGRX_PIXELSIZE;
pub const RGB_RED_0: c_int = EXT_XBGR_RED;
pub const RGB_GREEN_0: c_int = EXT_XBGR_GREEN;
pub const RGB_BLUE_0: c_int = EXT_XBGR_BLUE;
pub const RGB_ALPHA_0: c_int = 0i32;
pub const RGB_PIXELSIZE_0: c_int = EXT_XBGR_PIXELSIZE;
pub const RGB_RED: c_int = EXT_XRGB_RED;
pub const RGB_GREEN: c_int = EXT_XRGB_GREEN;
pub const RGB_BLUE: c_int = EXT_XRGB_BLUE;
pub const RGB_ALPHA: c_int = 0i32;
pub const RGB_PIXELSIZE: c_int = EXT_XRGB_PIXELSIZE;
/*
 * Initialize tables for YCC->RGB colorspace conversion.
 */
unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut i: c_int = 0;
    let mut x: JLONG = 0;
    (*cconvert).Cr_r_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut c_int;
    (*cconvert).Cb_b_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut c_int;
    (*cconvert).Cr_g_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    (*cconvert).Cb_g_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    i = 0i32;
    x = -CENTERJSAMPLE as JLONG;
    while i <= MAXJSAMPLE {
        *(*cconvert).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG * x
                + ((1i32 as JLONG) << 16i32 - 1i32)
                >> 16i32) as c_int;
        *(*cconvert).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG * x
                + ((1i32 as JLONG) << 16i32 - 1i32)
                >> 16i32) as c_int;
        *(*cconvert).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * x;
        *(*cconvert).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * x + ONE_HALF;
        i += 1;
        x += 1
    }
}
/*
 * Convert some rows of samples to the output colorspace.
 */
unsafe extern "C" fn ycc_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            ycc_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            ycc_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            ycc_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            ycc_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            ycc_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            ycc_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            ycc_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/* *************** Cases other than YCbCr -> RGB **************/
/*
 * Initialize for RGB->grayscale colorspace conversion.
 */
unsafe extern "C" fn build_rgb_y_table(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_y_tab: *mut JLONG = 0 as *mut JLONG;
    let mut i: JLONG = 0;
    rgb_y_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (TABLE_SIZE as c_ulong).wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    (*cconvert).rgb_y_tab = rgb_y_tab;
    i = 0i32 as JLONG;
    while i <= MAXJSAMPLE as c_long {
        *rgb_y_tab.offset((i + R_Y_OFF as c_long) as isize) =
            (0.29900f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i;
        *rgb_y_tab.offset((i + G_Y_OFF as c_long) as isize) =
            (0.58700f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i;
        *rgb_y_tab.offset((i + B_Y_OFF as c_long) as isize) =
            (0.11400f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i + ONE_HALF;
        i += 1
    }
}
/*
 * Convert RGB to grayscale.
 */
unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: c_int = 0;
    let mut g: c_int = 0;
    let mut b: c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_y_tab;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh42 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh42;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            r = *inptr0.offset(col as isize) as c_int;
            g = *inptr1.offset(col as isize) as c_int;
            b = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Color conversion for no colorspace change: just copy the data,
 * converting from separate-planes to interleaved representation.
 */
unsafe extern "C" fn null_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr3: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_components: c_int = (*cinfo).num_components;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut ci: c_int = 0;
    if num_components == 3i32 {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh43 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh43;
            col = 0i32 as JDIMENSION;
            while col < num_cols {
                let fresh44 = outptr;
                outptr = outptr.offset(1);
                *fresh44 = *inptr0.offset(col as isize);
                let fresh45 = outptr;
                outptr = outptr.offset(1);
                *fresh45 = *inptr1.offset(col as isize);
                let fresh46 = outptr;
                outptr = outptr.offset(1);
                *fresh46 = *inptr2.offset(col as isize);
                col = col.wrapping_add(1)
            }
        }
    } else if num_components == 4i32 {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
            inptr3 = *(*input_buf.offset(3isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh47 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh47;
            col = 0i32 as JDIMENSION;
            while col < num_cols {
                let fresh48 = outptr;
                outptr = outptr.offset(1);
                *fresh48 = *inptr0.offset(col as isize);
                let fresh49 = outptr;
                outptr = outptr.offset(1);
                *fresh49 = *inptr1.offset(col as isize);
                let fresh50 = outptr;
                outptr = outptr.offset(1);
                *fresh50 = *inptr2.offset(col as isize);
                let fresh51 = outptr;
                outptr = outptr.offset(1);
                *fresh51 = *inptr3.offset(col as isize);
                col = col.wrapping_add(1)
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            ci = 0i32;
            while ci < num_components {
                inptr = *(*input_buf.offset(ci as isize)).offset(input_row as isize);
                outptr = *output_buf;
                col = 0i32 as JDIMENSION;
                while col < num_cols {
                    *outptr.offset(ci as isize) = *inptr.offset(col as isize);
                    outptr = outptr.offset(num_components as isize);
                    col = col.wrapping_add(1)
                }
                ci += 1
            }
            output_buf = output_buf.offset(1isize);
            input_row = input_row.wrapping_add(1)
        }
    };
}
/*
 * Color conversion for grayscale: just copy the data.
 * This also works for YCbCr -> grayscale conversion, in which
 * we just copy the Y (luminance) component and ignore chrominance.
 */
unsafe extern "C" fn grayscale_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    jcopy_sample_rows(
        *input_buf.offset(0isize),
        input_row as c_int,
        output_buf,
        0i32,
        num_rows,
        (*cinfo).output_width,
    );
}
/*
 * Convert grayscale to RGB
 */
unsafe extern "C" fn gray_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            gray_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            gray_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            gray_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            gray_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            gray_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            gray_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            gray_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/*
 * Convert plain RGB to extended RGB
 */
unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            rgb_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            rgb_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            rgb_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            rgb_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            rgb_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            rgb_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            rgb_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/*
 * Adobe-style YCCK->CMYK conversion.
 * We convert YCbCr to R=1-C, G=1-M, and B=1-Y using the same
 * conversion as above, while passing K (black) unchanged.
 * We assume build_ycc_rgb_table has been called.
 */
unsafe extern "C" fn ycck_cmyk_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr3: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        inptr3 = *(*input_buf.offset(3isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh52 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh52;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(0isize) =
                *range_limit.offset((MAXJSAMPLE - (y + *Crrtab.offset(cr as isize))) as isize);
            *outptr.offset(1isize) = *range_limit.offset(
                (MAXJSAMPLE
                    - (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                        as c_int)) as isize,
            );
            *outptr.offset(2isize) =
                *range_limit.offset((MAXJSAMPLE - (y + *Cbbtab.offset(cb as isize))) as isize);
            *outptr.offset(3isize) = *inptr3.offset(col as isize);
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * RGB565 conversion
 */
/* Declarations for ordered dithering
 *
 * We use a 4x4 ordered dither array packed into 32 bits.  This array is
 * sufficent for dithering RGB888 to RGB565.
 */
pub const DITHER_MASK: c_int = 0x3i32;
pub(crate) static mut dither_matrix: [JLONG; 4] = [
    0x8020ai32 as JLONG,
    0xc040e06i32 as JLONG,
    0x30b0109i32 as JLONG,
    0xf070d05i32 as JLONG,
];
#[inline(always)]
unsafe extern "C" fn is_big_endian() -> boolean {
    let mut test_value: c_int = 1i32;
    if *(&mut test_value as *mut c_int as *mut c_char) as c_int != 1i32 {
        return TRUE;
    }
    return FALSE;
}
/* Include inline routines for RGB565 conversion */
unsafe extern "C" fn ycc_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        ycc_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn ycc_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        ycc_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn rgb_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        rgb_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn rgb_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        rgb_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn gray_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        gray_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn gray_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        gray_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
/*
 * Empty method for start_pass.
 */
unsafe extern "C" fn start_pass_dcolor(mut _cinfo: j_decompress_ptr) {}
/* no work needed */
/*
 * Module initialization routine for output colorspace conversion.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_color_deconverter(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = 0 as *mut my_color_deconverter;
    let mut ci: c_int = 0;
    cconvert = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_color_deconverter>() as c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut jpeg_color_deconverter;
    (*cconvert).pub_0.start_pass =
        Some(start_pass_dcolor as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    match (*cinfo).jpeg_color_space as c_uint {
        1 => {
            if (*cinfo).num_components != 1i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 3 => {
            if (*cinfo).num_components != 3i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 | 5 => {
            if (*cinfo).num_components != 4i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).num_components < 1i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    match (*cinfo).out_color_space as c_uint {
        1 => {
            (*cinfo).out_color_components = 1i32;
            if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint
                || (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                ci = 1i32;
                while ci < (*cinfo).num_components {
                    (*(*cinfo).comp_info.offset(ci as isize)).component_needed = FALSE;
                    ci += 1
                }
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_gray_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                build_rgb_y_table(cinfo);
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            (*cinfo).out_color_components = rgb_pixelsize[(*cinfo).out_color_space as usize];
            if (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                if 0 != jsimd_can_ycc_rgb() {
                    (*cconvert).pub_0.color_convert = Some(
                        jsimd_ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    );
                    build_ycc_rgb_table(cinfo);
                }
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                if rgb_red[(*cinfo).out_color_space as usize] == 0i32
                    && rgb_green[(*cinfo).out_color_space as usize] == 1i32
                    && rgb_blue[(*cinfo).out_color_space as usize] == 2i32
                    && rgb_pixelsize[(*cinfo).out_color_space as usize] == 3i32
                {
                    (*cconvert).pub_0.color_convert = Some(
                        null_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                }
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        16 => {
            (*cinfo).out_color_components = 3i32;
            if (*cinfo).dither_mode as c_uint == JDITHER_NONE as c_int as c_uint {
                if (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                    if 0 != jsimd_can_ycc_rgb565() {
                        (*cconvert).pub_0.color_convert = Some(
                            jsimd_ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: JSAMPIMAGE,
                                    _: JDIMENSION,
                                    _: JSAMPARRAY,
                                    _: c_int,
                                ) -> (),
                        )
                    } else {
                        (*cconvert).pub_0.color_convert = Some(
                            ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: JSAMPIMAGE,
                                    _: JDIMENSION,
                                    _: JSAMPARRAY,
                                    _: c_int,
                                ) -> (),
                        );
                        build_ycc_rgb_table(cinfo);
                    }
                } else if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                    (*cconvert).pub_0.color_convert = Some(
                        gray_rgb565_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb565_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    ycc_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 => {
            (*cinfo).out_color_components = 4i32;
            if (*cinfo).jpeg_color_space as c_uint == JCS_YCCK as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    ycck_cmyk_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).out_color_space as c_uint == (*cinfo).jpeg_color_space as c_uint {
                (*cinfo).out_color_components = (*cinfo).num_components;
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    if 0 != (*cinfo).quantize_colors {
        (*cinfo).output_components = 1i32
    } else {
        (*cinfo).output_components = (*cinfo).out_color_components
    };
}
