use libc::c_double;use libc::c_ulong;use libc::c_int;use libc::c_long;use libc;

pub use crate::stddef_h::size_t;

pub use crate::jccolext_c::extbgr_gray_convert_internal;
pub use crate::jccolext_c::extbgr_rgb_convert_internal;
pub use crate::jccolext_c::extbgr_ycc_convert_internal;
pub use crate::jccolext_c::extbgrx_gray_convert_internal;
pub use crate::jccolext_c::extbgrx_rgb_convert_internal;
pub use crate::jccolext_c::extbgrx_ycc_convert_internal;
pub use crate::jccolext_c::extrgb_gray_convert_internal;
pub use crate::jccolext_c::extrgb_rgb_convert_internal;
pub use crate::jccolext_c::extrgb_ycc_convert_internal;
pub use crate::jccolext_c::extrgbx_gray_convert_internal;
pub use crate::jccolext_c::extrgbx_rgb_convert_internal;
pub use crate::jccolext_c::extrgbx_ycc_convert_internal;
pub use crate::jccolext_c::extxbgr_gray_convert_internal;
pub use crate::jccolext_c::extxbgr_rgb_convert_internal;
pub use crate::jccolext_c::extxbgr_ycc_convert_internal;
pub use crate::jccolext_c::extxrgb_gray_convert_internal;
pub use crate::jccolext_c::extxrgb_rgb_convert_internal;
pub use crate::jccolext_c::extxrgb_ycc_convert_internal;
pub use crate::jccolext_c::rgb_gray_convert_internal;
pub use crate::jccolext_c::rgb_rgb_convert_internal;
pub use crate::jccolext_c::rgb_ycc_convert_internal;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::rgb_blue;
pub use crate::jmorecfg_h::rgb_green;
pub use crate::jmorecfg_h::rgb_pixelsize;
pub use crate::jmorecfg_h::rgb_red;
pub use crate::jmorecfg_h::CENTERJSAMPLE;
pub use crate::jmorecfg_h::EXT_BGRX_BLUE;
pub use crate::jmorecfg_h::EXT_BGRX_GREEN;
pub use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGRX_RED;
pub use crate::jmorecfg_h::EXT_BGR_BLUE;
pub use crate::jmorecfg_h::EXT_BGR_GREEN;
pub use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGR_RED;
pub use crate::jmorecfg_h::EXT_RGBX_BLUE;
pub use crate::jmorecfg_h::EXT_RGBX_GREEN;
pub use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGBX_RED;
pub use crate::jmorecfg_h::EXT_RGB_BLUE;
pub use crate::jmorecfg_h::EXT_RGB_GREEN;
pub use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGB_RED;
pub use crate::jmorecfg_h::EXT_XBGR_BLUE;
pub use crate::jmorecfg_h::EXT_XBGR_GREEN;
pub use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XBGR_RED;
pub use crate::jmorecfg_h::EXT_XRGB_BLUE;
pub use crate::jmorecfg_h::EXT_XRGB_GREEN;
pub use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XRGB_RED;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_PIXELSIZE_5;
pub use crate::jmorecfg_h::RGB_RED;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_writer;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use super::jerror::C2RustUnnamed_3;
pub use super::jerror::JERR_ARITH_NOTIMPL;
pub use super::jerror::JERR_BAD_ALIGN_TYPE;
pub use super::jerror::JERR_BAD_ALLOC_CHUNK;
pub use super::jerror::JERR_BAD_BUFFER_MODE;
pub use super::jerror::JERR_BAD_COMPONENT_ID;
pub use super::jerror::JERR_BAD_CROP_SPEC;
pub use super::jerror::JERR_BAD_DCTSIZE;
pub use super::jerror::JERR_BAD_DCT_COEF;
pub use super::jerror::JERR_BAD_HUFF_TABLE;
pub use super::jerror::JERR_BAD_IN_COLORSPACE;
pub use super::jerror::JERR_BAD_J_COLORSPACE;
pub use super::jerror::JERR_BAD_LENGTH;
pub use super::jerror::JERR_BAD_LIB_VERSION;
pub use super::jerror::JERR_BAD_MCU_SIZE;
pub use super::jerror::JERR_BAD_PARAM;
pub use super::jerror::JERR_BAD_PARAM_VALUE;
pub use super::jerror::JERR_BAD_POOL_ID;
pub use super::jerror::JERR_BAD_PRECISION;
pub use super::jerror::JERR_BAD_PROGRESSION;
pub use super::jerror::JERR_BAD_PROG_SCRIPT;
pub use super::jerror::JERR_BAD_SAMPLING;
pub use super::jerror::JERR_BAD_SCAN_SCRIPT;
pub use super::jerror::JERR_BAD_STATE;
pub use super::jerror::JERR_BAD_STRUCT_SIZE;
pub use super::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use super::jerror::JERR_BUFFER_SIZE;
pub use super::jerror::JERR_CANT_SUSPEND;
pub use super::jerror::JERR_CCIR601_NOTIMPL;
pub use super::jerror::JERR_COMPONENT_COUNT;
pub use super::jerror::JERR_CONVERSION_NOTIMPL;
pub use super::jerror::JERR_DAC_INDEX;
pub use super::jerror::JERR_DAC_VALUE;
pub use super::jerror::JERR_DHT_INDEX;
pub use super::jerror::JERR_DQT_INDEX;
pub use super::jerror::JERR_EMPTY_IMAGE;
pub use super::jerror::JERR_EMS_READ;
pub use super::jerror::JERR_EMS_WRITE;
pub use super::jerror::JERR_EOI_EXPECTED;
pub use super::jerror::JERR_FILE_READ;
pub use super::jerror::JERR_FILE_WRITE;
pub use super::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use super::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use super::jerror::JERR_HUFF_MISSING_CODE;
pub use super::jerror::JERR_IMAGE_TOO_BIG;
pub use super::jerror::JERR_INPUT_EMPTY;
pub use super::jerror::JERR_INPUT_EOF;
pub use super::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use super::jerror::JERR_MISSING_DATA;
pub use super::jerror::JERR_MODE_CHANGE;
pub use super::jerror::JERR_NOTIMPL;
pub use super::jerror::JERR_NOT_COMPILED;
pub use super::jerror::JERR_NO_BACKING_STORE;
pub use super::jerror::JERR_NO_HUFF_TABLE;
pub use super::jerror::JERR_NO_IMAGE;
pub use super::jerror::JERR_NO_QUANT_TABLE;
pub use super::jerror::JERR_NO_SOI;
pub use super::jerror::JERR_OUT_OF_MEMORY;
pub use super::jerror::JERR_QUANT_COMPONENTS;
pub use super::jerror::JERR_QUANT_FEW_COLORS;
pub use super::jerror::JERR_QUANT_MANY_COLORS;
pub use super::jerror::JERR_SOF_DUPLICATE;
pub use super::jerror::JERR_SOF_NO_SOS;
pub use super::jerror::JERR_SOF_UNSUPPORTED;
pub use super::jerror::JERR_SOI_DUPLICATE;
pub use super::jerror::JERR_SOS_NO_SOF;
pub use super::jerror::JERR_TFILE_CREATE;
pub use super::jerror::JERR_TFILE_READ;
pub use super::jerror::JERR_TFILE_SEEK;
pub use super::jerror::JERR_TFILE_WRITE;
pub use super::jerror::JERR_TOO_LITTLE_DATA;
pub use super::jerror::JERR_UNKNOWN_MARKER;
pub use super::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use super::jerror::JERR_VIRTUAL_BUG;
pub use super::jerror::JERR_WIDTH_OVERFLOW;
pub use super::jerror::JERR_XMS_READ;
pub use super::jerror::JERR_XMS_WRITE;
pub use super::jerror::JMSG_COPYRIGHT;
pub use super::jerror::JMSG_LASTMSGCODE;
pub use super::jerror::JMSG_NOMESSAGE;
pub use super::jerror::JMSG_VERSION;
pub use super::jerror::JTRC_16BIT_TABLES;
pub use super::jerror::JTRC_ADOBE;
pub use super::jerror::JTRC_APP0;
pub use super::jerror::JTRC_APP14;
pub use super::jerror::JTRC_DAC;
pub use super::jerror::JTRC_DHT;
pub use super::jerror::JTRC_DQT;
pub use super::jerror::JTRC_DRI;
pub use super::jerror::JTRC_EMS_CLOSE;
pub use super::jerror::JTRC_EMS_OPEN;
pub use super::jerror::JTRC_EOI;
pub use super::jerror::JTRC_HUFFBITS;
pub use super::jerror::JTRC_JFIF;
pub use super::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use super::jerror::JTRC_JFIF_EXTENSION;
pub use super::jerror::JTRC_JFIF_THUMBNAIL;
pub use super::jerror::JTRC_MISC_MARKER;
pub use super::jerror::JTRC_PARMLESS_MARKER;
pub use super::jerror::JTRC_QUANTVALS;
pub use super::jerror::JTRC_QUANT_3_NCOLORS;
pub use super::jerror::JTRC_QUANT_NCOLORS;
pub use super::jerror::JTRC_QUANT_SELECTED;
pub use super::jerror::JTRC_RECOVERY_ACTION;
pub use super::jerror::JTRC_RST;
pub use super::jerror::JTRC_SMOOTH_NOTIMPL;
pub use super::jerror::JTRC_SOF;
pub use super::jerror::JTRC_SOF_COMPONENT;
pub use super::jerror::JTRC_SOI;
pub use super::jerror::JTRC_SOS;
pub use super::jerror::JTRC_SOS_COMPONENT;
pub use super::jerror::JTRC_SOS_PARAMS;
pub use super::jerror::JTRC_TFILE_CLOSE;
pub use super::jerror::JTRC_TFILE_OPEN;
pub use super::jerror::JTRC_THUMB_JPEG;
pub use super::jerror::JTRC_THUMB_PALETTE;
pub use super::jerror::JTRC_THUMB_RGB;
pub use super::jerror::JTRC_UNKNOWN_IDS;
pub use super::jerror::JTRC_XMS_CLOSE;
pub use super::jerror::JTRC_XMS_OPEN;
pub use super::jerror::JWRN_ADOBE_XFORM;
pub use super::jerror::JWRN_BOGUS_ICC;
pub use super::jerror::JWRN_BOGUS_PROGRESSION;
pub use super::jerror::JWRN_EXTRANEOUS_DATA;
pub use super::jerror::JWRN_HIT_MARKER;
pub use super::jerror::JWRN_HUFF_BAD_CODE;
pub use super::jerror::JWRN_JFIF_MAJOR;
pub use super::jerror::JWRN_JPEG_EOF;
pub use super::jerror::JWRN_MUST_RESYNC;
pub use super::jerror::JWRN_NOT_SEQUENTIAL;
pub use super::jerror::JWRN_TOO_MUCH_DATA;
use super::simd::x86_64::jsimd::jsimd_can_rgb_gray;
use super::simd::x86_64::jsimd::jsimd_can_rgb_ycc;
use super::simd::x86_64::jsimd::jsimd_rgb_gray_convert;
use super::simd::x86_64::jsimd::jsimd_rgb_ycc_convert;

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

pub const CBCR_OFFSET: JLONG =
    (CENTERJSAMPLE as JLONG) << SCALEBITS;

pub const ONE_HALF: JLONG = (1i64) << SCALEBITS - 1i32;
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
    
    
     let mut rgb_ycc_tab:   *mut JLONG =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        TABLE_SIZE as c_ulong *
    ::std::mem::size_of::<JLONG>() as c_ulong,
    ) as *mut JLONG;
    (*cconvert).rgb_ycc_tab = rgb_ycc_tab;
     let mut i:   JLONG =  0i64;
    while i <= MAXJSAMPLE as c_long {
        *rgb_ycc_tab.offset((i + R_Y_OFF as c_long) as isize) =
            (0.29900f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG
                * i;
        *rgb_ycc_tab.offset((i + G_Y_OFF as c_long) as isize) =
            (0.58700f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG
                * i;
        *rgb_ycc_tab.offset((i + B_Y_OFF as c_long) as isize) =
            (0.11400f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG
                * i
                + ONE_HALF;
        *rgb_ycc_tab.offset((i + R_CB_OFF as c_long) as isize) =
            -((0.16874f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG)
                * i;
        *rgb_ycc_tab.offset((i + G_CB_OFF as c_long) as isize) =
            -((0.33126f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG)
                * i;
        /* We use a rounding fudge-factor of 0.5-epsilon for Cb and Cr.
         * This ensures that the maximum output will round to MAXJSAMPLE
         * not MAXJSAMPLE+1, and thus that we don't have to range-limit.
         */
        *rgb_ycc_tab.offset((i + B_CB_OFF as c_long) as isize) =
            (0.50000f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG
                * i
                + CBCR_OFFSET
                + ONE_HALF
                - 1i64;
        /*  B=>Cb and R=>Cr tables are the same
            rgb_ycc_tab[i + R_CR_OFF] = FIX(0.50000) * i  + CBCR_OFFSET + ONE_HALF - 1;
        */
        *rgb_ycc_tab.offset((i + G_CR_OFF as c_long) as isize) =
            -((0.41869f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG)
                * i;
        *rgb_ycc_tab.offset((i + B_CR_OFF as c_long) as isize) =
            -((0.08131f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG)
                * i;
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
    match  (*cinfo).in_color_space {
        6 => {
            extrgb_ycc_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        7 | 12 => {
            extrgbx_ycc_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        8 => {
            extbgr_ycc_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        9 | 13 => {
            extbgrx_ycc_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        10 | 14 => {
            extxbgr_ycc_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        11 | 15 => {
            extxrgb_ycc_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        _ => {
            rgb_ycc_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
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
    match  (*cinfo).in_color_space {
        6 => {
            extrgb_gray_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        7 | 12 => {
            extrgbx_gray_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        8 => {
            extbgr_gray_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        9 | 13 => {
            extbgrx_gray_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        10 | 14 => {
            extxbgr_gray_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        11 | 15 => {
            extxrgb_gray_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        _ => {
            rgb_gray_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
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
    match  (*cinfo).in_color_space {
        6 => {
            extrgb_rgb_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        7 | 12 => {
            extrgbx_rgb_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        8 => {
            extbgr_rgb_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        9 | 13 => {
            extbgrx_rgb_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        10 | 14 => {
            extxbgr_rgb_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        11 | 15 => {
            extxrgb_rgb_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
        }
        _ => {
            rgb_rgb_convert_internal(
                cinfo, input_buf, output_buf, output_row, num_rows,
            );
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
    
    
    
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    
    
    
    
    
    
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
              num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh21 = input_buf;
        input_buf = input_buf.offset(1);
        
        
        
        
         let mut inptr:   JSAMPROW =  *fresh21; let mut outptr0:   JSAMPROW =
     *(*output_buf.offset(0)).offset(output_row as isize); let mut outptr1:   JSAMPROW =
     *(*output_buf.offset(1)).offset(output_row as isize); let mut outptr2:   JSAMPROW =
     *(*output_buf.offset(2)).offset(output_row as isize); let mut outptr3:   JSAMPROW =
     *(*output_buf.offset(3)).offset(output_row as isize);
        output_row +=  1;
         let mut col:   JDIMENSION =  0u32;
        while col < num_cols {
               
            
             let mut r:   c_int =
     MAXJSAMPLE - *inptr.offset(0) as c_int; let mut g:   c_int =
     MAXJSAMPLE - *inptr.offset(1) as c_int; let mut b:   c_int =
     MAXJSAMPLE - *inptr.offset(2) as c_int;
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
                >> SCALEBITS)
                as JSAMPLE;
            /* Cb */
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as JSAMPLE;
            /* Cr */
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as JSAMPLE;
            col +=  1
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
     /* don't need GETJSAMPLE() here */
    
    
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    let mut instride: c_int = (*cinfo).input_components;
    loop {
           num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh22 = input_buf;
        input_buf = input_buf.offset(1);
        
         let mut inptr:   JSAMPROW =  *fresh22; let mut outptr:   JSAMPROW =
     *(*output_buf.offset(0)).offset(output_row as isize);
        output_row +=  1;
         let mut col:   JDIMENSION =  0u32;
        while col < num_cols {
            *outptr.offset(col as isize) = *inptr.offset(0);
            inptr = inptr.offset(instride as isize);
            col +=  1
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
    
    
    
    
    
    
    
     let mut inptr:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>(); let mut outptr0:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>(); let mut outptr1:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>(); let mut outptr2:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>(); let mut col:  JDIMENSION =  0;
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
            output_row +=  1;
            col = 0u32;
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
                col +=  1
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
             let mut outptr3:   JSAMPROW =
     *(*output_buf.offset(3)).offset(output_row as isize);
            output_row +=  1;
            col = 0u32;
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
                col +=  1
            }
        }
    } else {
        loop {
             num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
             let mut ci:   c_int =  0i32; /* don't need GETJSAMPLE() here */
            while ci < nc {
                 inptr = *input_buf;
                 let mut outptr:   JSAMPROW =
     *(*output_buf.offset(ci as isize)).offset(output_row as isize);
                col = 0u32;
                while col < num_cols {
                    *outptr.offset(col as isize) = *inptr.offset(ci as isize);
                    inptr = inptr.offset(nc as isize);
                    col +=  1
                }
                ci += 1
            }
            input_buf = input_buf.offset(1);
            output_row +=  1
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
     
     let mut cconvert:   my_cconvert_ptr =
     Some(
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
    match  (*cinfo).in_color_space {
        1 => {
            if (*cinfo).input_components != 1i32 {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
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
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            if (*cinfo).input_components
                != rgb_pixelsize[(*cinfo).in_color_space as usize]
            {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
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
        3 => {
            if (*cinfo).input_components != 3i32 {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
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
        4 | 5 => {
            if (*cinfo).input_components != 4i32 {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
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
        _ => {
            /* JCS_UNKNOWN can be anything */
            if (*cinfo).input_components < 1i32 {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
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
    }
    /* Check num_components, set conversion method based on requested space */
    match  (*cinfo).jpeg_color_space {
        1 => {
            if (*cinfo).num_components != 1i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            if  (*cinfo).in_color_space
                ==  JCS_GRAYSCALE
            {
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
            } else if  (*cinfo).in_color_space
                ==  JCS_RGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGBX
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGRX
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_XBGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_XRGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGBA
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGRA
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_ABGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_ARGB
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
                    (*cconvert).pub_0.start_pass = Some(
                        rgb_ycc_start
                            as unsafe extern "C" fn(_: j_compress_ptr) -> (),
                    );
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
            } else if  (*cinfo).in_color_space
                ==  JCS_YCbCr
            {
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
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
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
        2 => {
            if (*cinfo).num_components != 3i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
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
            } else if  (*cinfo).in_color_space
                ==  JCS_RGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGBX
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGRX
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_XBGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_XRGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGBA
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGRA
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_ABGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_ARGB
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
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
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
        3 => {
            if (*cinfo).num_components != 3i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            if  (*cinfo).in_color_space
                ==  JCS_RGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGBX
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGRX
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_XBGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_XRGB
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_RGBA
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_BGRA
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_ABGR
                ||  (*cinfo).in_color_space
                    ==  JCS_EXT_ARGB
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
                    (*cconvert).pub_0.start_pass = Some(
                        rgb_ycc_start
                            as unsafe extern "C" fn(_: j_compress_ptr) -> (),
                    );
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
            } else if  (*cinfo).in_color_space
                ==  JCS_YCbCr
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
            } else {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
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
        4 => {
            if (*cinfo).num_components != 4i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            if  (*cinfo).in_color_space
                ==  JCS_CMYK
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
            } else {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
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
        5 => {
            if (*cinfo).num_components != 4i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_J_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            if  (*cinfo).in_color_space
                ==  JCS_CMYK
            {
                (*cconvert).pub_0.start_pass = Some(
                    rgb_ycc_start
                        as unsafe extern "C" fn(_: j_compress_ptr) -> (),
                );
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
            } else if  (*cinfo).in_color_space
                ==  JCS_YCCK
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
            } else {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
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
        _ => {
            /* allow null conversion of JCS_UNKNOWN */
            if  (*cinfo).jpeg_color_space !=  (*cinfo).in_color_space
                || (*cinfo).num_components != (*cinfo).input_components
            {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
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
