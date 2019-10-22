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
use super::simd::x86_64::jsimd::{
    jsimd_can_rgb_gray, jsimd_can_rgb_ycc, jsimd_rgb_gray_convert, jsimd_rgb_ycc_convert,
};
pub use crate::jccolext_c::{
    extbgr_gray_convert_internal, extbgr_rgb_convert_internal, extbgr_ycc_convert_internal,
    extbgrx_gray_convert_internal, extbgrx_rgb_convert_internal, extbgrx_ycc_convert_internal,
    extrgb_gray_convert_internal, extrgb_rgb_convert_internal, extrgb_ycc_convert_internal,
    extrgbx_gray_convert_internal, extrgbx_rgb_convert_internal, extrgbx_ycc_convert_internal,
    extxbgr_gray_convert_internal, extxbgr_rgb_convert_internal, extxbgr_ycc_convert_internal,
    extxrgb_gray_convert_internal, extxrgb_rgb_convert_internal, extxrgb_ycc_convert_internal,
    rgb_gray_convert_internal, rgb_rgb_convert_internal, rgb_ycc_convert_internal,
};
pub use crate::jmorecfg_h::{
    boolean, rgb_blue, rgb_green, rgb_pixelsize, rgb_red, CENTERJSAMPLE, EXT_BGRX_BLUE,
    EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN,
    EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE,
    EXT_RGBX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_XBGR_BLUE,
    EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN,
    EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAXJSAMPLE, RGB_BLUE,
    RGB_GREEN, RGB_PIXELSIZE_5, RGB_RED, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, JLONG,
    J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_writer, jpeg_memory_mgr,
    jpeg_progress_mgr, jpeg_scan_info, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
    JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
    JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_IMAGE,
    JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
};
pub use crate::stddef_h::size_t;
use libc::{self, c_double, c_int, c_long, c_uint, c_ulong};

pub type my_cconvert_ptr = *mut my_color_converter;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_converter {
    pub pub_0: jpeg_color_converter,
    pub rgb_ycc_tab: *mut JLONG,
}
/* *************** RGB -> YCbCr conversion: most common case **************/
/*
 * YCbCr is defined per CCIR 601-1, except that Cb and Cr are
 * normalized to the range 0..MAXJSAMPLE rather than -0.5 .. 0.5.
 * The conversion equations to be implemented are therefore
 *      Y  =  0.29900 * R + 0.58700 * G + 0.11400 * B
 *      Cb = -0.16874 * R - 0.33126 * G + 0.50000 * B  + CENTERJSAMPLE
 *      Cr =  0.50000 * R - 0.41869 * G - 0.08131 * B  + CENTERJSAMPLE
 * (These numbers are derived from TIFF 6.0 section 21, dated 3-June-92.)
 * Note: older versions of the IJG code used a zero offset of MAXJSAMPLE/2,
 * rather than CENTERJSAMPLE, for Cb and Cr.  This gave equal positive and
 * negative swings for Cb/Cr, but meant that grayscale values (Cb=Cr=0)
 * were not represented exactly.  Now we sacrifice exact representation of
 * maximum red and maximum blue in order to get exact grayscales.
 *
 * To avoid floating-point arithmetic, we represent the fractional constants
 * as integers scaled up by 2^16 (about 4 digits precision); we have to divide
 * the products by 2^16, with appropriate rounding, to get the correct answer.
 *
 * For even more speed, we avoid doing any multiplications in the inner loop
 * by precalculating the constants times R,G,B for all possible values.
 * For 8-bit JSAMPLEs this is very reasonable (only 256 entries per table);
 * for 12-bit samples it is still acceptable.  It's not very reasonable for
 * 16-bit samples, but if you want lossless storage you shouldn't be changing
 * colorspace anyway.
 * The CENTERJSAMPLE offsets and the rounding fudge-factor of 0.5 are included
 * in the tables to save adding them separately in the inner loop.
 */

pub const SCALEBITS: c_int = 16i32;
/* speediest right-shift on some machines */

pub const CBCR_OFFSET: JLONG = (CENTERJSAMPLE as JLONG) << SCALEBITS;

pub const ONE_HALF: JLONG = (1i32 as JLONG) << SCALEBITS - 1i32;
/* We allocate one big table and divide it up into eight parts, instead of
 * doing eight alloc_small requests.  This lets us use a single table base
 * address, which can be held in a register in the inner loops on many
 * machines (more than can hold all eight addresses, anyway).
 */

pub const R_Y_OFF: c_int = 0i32;
/* offset to R => Y section */

pub const G_Y_OFF: c_int = 1i32 * (MAXJSAMPLE + 1i32);
/* offset to G => Y section */

pub const B_Y_OFF: c_int = 2i32 * (MAXJSAMPLE + 1i32);
/* etc. */

pub const R_CB_OFF: c_int = 3i32 * (MAXJSAMPLE + 1i32);

pub const G_CB_OFF: c_int = 4i32 * (MAXJSAMPLE + 1i32);

pub const B_CB_OFF: c_int = 5i32 * (MAXJSAMPLE + 1i32);

pub const R_CR_OFF: c_int = B_CB_OFF;
/* B=>Cb, R=>Cr are the same */

pub const G_CR_OFF: c_int = 6i32 * (MAXJSAMPLE + 1i32);

pub const B_CR_OFF: c_int = 7i32 * (MAXJSAMPLE + 1i32);

pub const TABLE_SIZE: c_int = 8i32 * (MAXJSAMPLE + 1i32);
/* Include inline routines for colorspace extensions */

pub const RGB_PIXELSIZE_4: c_int = EXT_RGB_PIXELSIZE;

pub const RGB_PIXELSIZE_2: c_int = EXT_RGBX_PIXELSIZE;

pub const RGB_PIXELSIZE_3: c_int = EXT_BGR_PIXELSIZE;

pub const RGB_PIXELSIZE_1: c_int = EXT_BGRX_PIXELSIZE;

pub const RGB_PIXELSIZE_0: c_int = EXT_XBGR_PIXELSIZE;

pub const RGB_PIXELSIZE: c_int = EXT_XRGB_PIXELSIZE;
/*
 * Initialize for RGB->YCC colorspace conversion.
 */

unsafe extern "C" fn rgb_ycc_start(mut cinfo: j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_ycc_tab: *mut JLONG = 0 as *mut JLONG;
    let mut i: JLONG = 0;
    /* Allocate and fill in the conversion tables. */
    rgb_ycc_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (TABLE_SIZE as c_ulong).wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    (*cconvert).rgb_ycc_tab = rgb_ycc_tab;
    i = 0i32 as JLONG;
    while i <= MAXJSAMPLE as c_long {
        *rgb_ycc_tab.offset((i + R_Y_OFF as c_long) as isize) =
            (0.29900f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i;
        *rgb_ycc_tab.offset((i + G_Y_OFF as c_long) as isize) =
            (0.58700f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i;
        *rgb_ycc_tab.offset((i + B_Y_OFF as c_long) as isize) =
            (0.11400f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i + ONE_HALF;
        *rgb_ycc_tab.offset((i + R_CB_OFF as c_long) as isize) =
            -((0.16874f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * i;
        *rgb_ycc_tab.offset((i + G_CB_OFF as c_long) as isize) =
            -((0.33126f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * i;
        /* We use a rounding fudge-factor of 0.5-epsilon for Cb and Cr.
         * This ensures that the maximum output will round to MAXJSAMPLE
         * not MAXJSAMPLE+1, and thus that we don't have to range-limit.
         */
        *rgb_ycc_tab.offset((i + B_CB_OFF as c_long) as isize) =
            (0.50000f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i
                + CBCR_OFFSET
                + ONE_HALF
                - 1i32 as c_long;
        /*  B=>Cb and R=>Cr tables are the same
            rgb_ycc_tab[i + R_CR_OFF] = FIX(0.50000) * i  + CBCR_OFFSET + ONE_HALF - 1;
        */
        *rgb_ycc_tab.offset((i + G_CR_OFF as c_long) as isize) =
            -((0.41869f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * i;
        *rgb_ycc_tab.offset((i + B_CR_OFF as c_long) as isize) =
            -((0.08131f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * i;
        i += 1
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 */

unsafe extern "C" fn rgb_ycc_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    match (*cinfo).in_color_space as c_uint {
        6 => {
            extrgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
/* *************** Cases other than RGB -> YCbCr **************/
/*
 * Convert some rows of samples to the JPEG colorspace.
 */

unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    match (*cinfo).in_color_space as c_uint {
        6 => {
            extrgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
/*
 * Extended RGB to plain RGB conversion
 */

unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    match (*cinfo).in_color_space as c_uint {
        6 => {
            extrgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles Adobe-style CMYK->YCCK conversion,
 * where we convert R=1-C, G=1-M, and B=1-Y to YCbCr using the same
 * conversion as above, while passing K (black) unchanged.
 * We assume rgb_ycc_start has been called.
 */

unsafe extern "C" fn cmyk_ycck_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: c_int = 0;
    let mut g: c_int = 0;
    let mut b: c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr3: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh21 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh21;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        outptr3 = *(*output_buf.offset(3)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            r = MAXJSAMPLE - *inptr.offset(0) as c_int;
            g = MAXJSAMPLE - *inptr.offset(1) as c_int;
            b = MAXJSAMPLE - *inptr.offset(2) as c_int;
            /* K passes through as-is */
            *outptr3.offset(col as isize) = *inptr.offset(3); /* don't need GETJSAMPLE here */
            inptr = inptr.offset(4);
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            /* Cb */
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            /* Cr */
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles grayscale output with no conversion.
 * The source can be either plain grayscale or YCbCr (since Y == gray).
 */

unsafe extern "C" fn grayscale_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE; /* don't need GETJSAMPLE() here */
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    let mut instride: c_int = (*cinfo).input_components;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh22 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh22;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(col as isize) = *inptr.offset(0);
            inptr = inptr.offset(instride as isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles multi-component colorspaces without conversion.
 * We assume input_components == num_components.
 */

unsafe extern "C" fn null_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr3: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut nc: c_int = (*cinfo).num_components;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    if nc == 3i32 {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            let fresh23 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh23;
            outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0i32 as JDIMENSION;
            while col < num_cols {
                let fresh24 = inptr;
                inptr = inptr.offset(1);
                *outptr0.offset(col as isize) = *fresh24;
                let fresh25 = inptr;
                inptr = inptr.offset(1);
                *outptr1.offset(col as isize) = *fresh25;
                let fresh26 = inptr;
                inptr = inptr.offset(1);
                *outptr2.offset(col as isize) = *fresh26;
                col = col.wrapping_add(1)
            }
        }
    } else if nc == 4i32 {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            let fresh27 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh27;
            outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
            outptr3 = *(*output_buf.offset(3)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0i32 as JDIMENSION;
            while col < num_cols {
                let fresh28 = inptr;
                inptr = inptr.offset(1);
                *outptr0.offset(col as isize) = *fresh28;
                let fresh29 = inptr;
                inptr = inptr.offset(1);
                *outptr1.offset(col as isize) = *fresh29;
                let fresh30 = inptr;
                inptr = inptr.offset(1);
                *outptr2.offset(col as isize) = *fresh30;
                let fresh31 = inptr;
                inptr = inptr.offset(1);
                *outptr3.offset(col as isize) = *fresh31;
                col = col.wrapping_add(1)
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            /* It seems fastest to make a separate pass for each component. */
            ci = 0i32; /* don't need GETJSAMPLE() here */
            while ci < nc {
                inptr = *input_buf;
                outptr = *(*output_buf.offset(ci as isize)).offset(output_row as isize);
                col = 0i32 as JDIMENSION;
                while col < num_cols {
                    *outptr.offset(col as isize) = *inptr.offset(ci as isize);
                    inptr = inptr.offset(nc as isize);
                    col = col.wrapping_add(1)
                }
                ci += 1
            }
            input_buf = input_buf.offset(1);
            output_row = output_row.wrapping_add(1)
        }
    };
}
/*
 * Empty method for start_pass.
 */

unsafe extern "C" fn null_method(mut cinfo: j_compress_ptr) {
    /* no work needed */
}
/*
 * Module initialization routine for input colorspace conversion.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_color_converter(mut cinfo: j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = 0 as *mut my_color_converter;
    cconvert = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_color_converter>() as c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut jpeg_color_converter;
    /* set start_pass to null method until we find out differently */
    (*cconvert).pub_0.start_pass =
        Some(null_method as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    /* Make sure input_components agrees with in_color_space */
    match (*cinfo).in_color_space as c_uint {
        1 => {
            if (*cinfo).input_components != 1i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            if (*cinfo).input_components != rgb_pixelsize[(*cinfo).in_color_space as usize] {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        3 => {
            if (*cinfo).input_components != 3i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 | 5 => {
            if (*cinfo).input_components != 4i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            /* JCS_UNKNOWN can be anything */
            if (*cinfo).input_components < 1i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    /* Check num_components, set conversion method based on requested space */
    match (*cinfo).jpeg_color_space as c_uint {
        1 => {
            if (*cinfo).num_components != 1i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGBX as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGRX as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_XBGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_XRGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGBA as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGRA as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_ABGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_ARGB as c_int as c_uint
            {
                if super::simd::x86_64::jsimd::jsimd_can_rgb_gray() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        super::simd::x86_64::jsimd::jsimd_rgb_gray_convert
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: JSAMPARRAY,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.start_pass =
                        Some(rgb_ycc_start as unsafe extern "C" fn(_: j_compress_ptr) -> ());
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_gray_convert
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: JSAMPARRAY,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: c_int,
                            ) -> (),
                    )
                }
            } else if (*cinfo).in_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 => {
            if (*cinfo).num_components != 3i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if rgb_red[(*cinfo).in_color_space as usize] == 0i32
                && rgb_green[(*cinfo).in_color_space as usize] == 1i32
                && rgb_blue[(*cinfo).in_color_space as usize] == 2i32
                && rgb_pixelsize[(*cinfo).in_color_space as usize] == 3i32
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGBX as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGRX as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_XBGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_XRGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGBA as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGRA as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_ABGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_ARGB as c_int as c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        3 => {
            if (*cinfo).num_components != 3i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGBX as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGRX as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_XBGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_XRGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_RGBA as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_BGRA as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_ABGR as c_int as c_uint
                || (*cinfo).in_color_space as c_uint == JCS_EXT_ARGB as c_int as c_uint
            {
                if super::simd::x86_64::jsimd::jsimd_can_rgb_ycc() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        super::simd::x86_64::jsimd::jsimd_rgb_ycc_convert
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: JSAMPARRAY,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.start_pass =
                        Some(rgb_ycc_start as unsafe extern "C" fn(_: j_compress_ptr) -> ());
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_ycc_convert
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: JSAMPARRAY,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: c_int,
                            ) -> (),
                    )
                }
            } else if (*cinfo).in_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 => {
            if (*cinfo).num_components != 4i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        5 => {
            if (*cinfo).num_components != 4i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*cconvert).pub_0.start_pass =
                    Some(rgb_ycc_start as unsafe extern "C" fn(_: j_compress_ptr) -> ());
                (*cconvert).pub_0.color_convert = Some(
                    cmyk_ycck_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_YCCK as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            /* allow null conversion of JCS_UNKNOWN */
            if (*cinfo).jpeg_color_space as c_uint != (*cinfo).in_color_space as c_uint
                || (*cinfo).num_components != (*cinfo).input_components
            {
                (*(*cinfo).err).msg_code = super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*cconvert).pub_0.color_convert = Some(
                null_convert
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
    };
}
