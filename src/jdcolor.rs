use libc;

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:19"]
pub mod jmorecfg_h {

    pub static mut rgb_green: [libc::c_int; 17] = [
        -1i32,
        -1i32,
        crate::jmorecfg_h::RGB_GREEN_5,
        -1i32,
        -1i32,
        -1i32,
        crate::jmorecfg_h::EXT_RGB_GREEN,
        crate::jmorecfg_h::EXT_RGBX_GREEN,
        crate::jmorecfg_h::EXT_BGR_GREEN,
        crate::jmorecfg_h::EXT_BGRX_GREEN,
        crate::jmorecfg_h::EXT_XBGR_GREEN,
        crate::jmorecfg_h::EXT_XRGB_GREEN,
        crate::jmorecfg_h::EXT_RGBX_GREEN,
        crate::jmorecfg_h::EXT_BGRX_GREEN,
        crate::jmorecfg_h::EXT_XBGR_GREEN,
        crate::jmorecfg_h::EXT_XRGB_GREEN,
        -1i32,
    ];

    pub static mut rgb_blue: [libc::c_int; 17] = [
        -1i32,
        -1i32,
        crate::jmorecfg_h::RGB_BLUE_5,
        -1i32,
        -1i32,
        -1i32,
        crate::jmorecfg_h::EXT_RGB_BLUE,
        crate::jmorecfg_h::EXT_RGBX_BLUE,
        crate::jmorecfg_h::EXT_BGR_BLUE,
        crate::jmorecfg_h::EXT_BGRX_BLUE,
        crate::jmorecfg_h::EXT_XBGR_BLUE,
        crate::jmorecfg_h::EXT_XRGB_BLUE,
        crate::jmorecfg_h::EXT_RGBX_BLUE,
        crate::jmorecfg_h::EXT_BGRX_BLUE,
        crate::jmorecfg_h::EXT_XBGR_BLUE,
        crate::jmorecfg_h::EXT_XRGB_BLUE,
        -1i32,
    ];

    pub static mut rgb_red: [libc::c_int; 17] = [
        -1i32,
        -1i32,
        crate::jmorecfg_h::RGB_RED_5,
        -1i32,
        -1i32,
        -1i32,
        crate::jmorecfg_h::EXT_RGB_RED,
        crate::jmorecfg_h::EXT_RGBX_RED,
        crate::jmorecfg_h::EXT_BGR_RED,
        crate::jmorecfg_h::EXT_BGRX_RED,
        crate::jmorecfg_h::EXT_XBGR_RED,
        crate::jmorecfg_h::EXT_XRGB_RED,
        crate::jmorecfg_h::EXT_RGBX_RED,
        crate::jmorecfg_h::EXT_BGRX_RED,
        crate::jmorecfg_h::EXT_XBGR_RED,
        crate::jmorecfg_h::EXT_XRGB_RED,
        -1i32,
    ];
    /* JPEG_INTERNAL_OPTIONS */
    /* FAST_FLOAT should be either float or double, whichever is done faster
     * by your compiler.  (Note that this type is only used in the floating point
     * DCT routines, so it only matters if you've defined DCT_FLOAT_SUPPORTED.)
     */
    /* prefer 16-bit with SIMD for parellelism */
    /* On some machines (notably 68000 series) "int" is 32 bits, but multiplying
     * two 16-bit shorts is faster than multiplying two ints.  Define MULTIPLIER
     * as short on such a machine.  MULTIPLIER must be at least 16 bits wide.
     */
    /* Definitions for speed-related optimizations. */
}

pub use crate::stddef_h::size_t;

pub use crate::jdcol565_c::gray_rgb565D_convert_be;
pub use crate::jdcol565_c::gray_rgb565D_convert_le;
pub use crate::jdcol565_c::gray_rgb565_convert_be;
pub use crate::jdcol565_c::gray_rgb565_convert_le;
pub use crate::jdcol565_c::rgb_rgb565D_convert_be;
pub use crate::jdcol565_c::rgb_rgb565D_convert_le;
pub use crate::jdcol565_c::rgb_rgb565_convert_be;
pub use crate::jdcol565_c::rgb_rgb565_convert_le;
pub use crate::jdcol565_c::ycc_rgb565D_convert_be;
pub use crate::jdcol565_c::ycc_rgb565D_convert_le;
pub use crate::jdcol565_c::ycc_rgb565_convert_be;
pub use crate::jdcol565_c::ycc_rgb565_convert_le;
pub use crate::jdcolext_c::gray_extbgr_convert_internal;
pub use crate::jdcolext_c::gray_extbgrx_convert_internal;
pub use crate::jdcolext_c::gray_extrgb_convert_internal;
pub use crate::jdcolext_c::gray_extrgbx_convert_internal;
pub use crate::jdcolext_c::gray_extxbgr_convert_internal;
pub use crate::jdcolext_c::gray_extxrgb_convert_internal;
pub use crate::jdcolext_c::gray_rgb_convert_internal;
pub use crate::jdcolext_c::rgb_extbgr_convert_internal;
pub use crate::jdcolext_c::rgb_extbgrx_convert_internal;
pub use crate::jdcolext_c::rgb_extrgb_convert_internal;
pub use crate::jdcolext_c::rgb_extrgbx_convert_internal;
pub use crate::jdcolext_c::rgb_extxbgr_convert_internal;
pub use crate::jdcolext_c::rgb_extxrgb_convert_internal;
pub use crate::jdcolext_c::rgb_rgb_convert_internal;
pub use crate::jdcolext_c::ycc_extbgr_convert_internal;
pub use crate::jdcolext_c::ycc_extbgrx_convert_internal;
pub use crate::jdcolext_c::ycc_extrgb_convert_internal;
pub use crate::jdcolext_c::ycc_extrgbx_convert_internal;
pub use crate::jdcolext_c::ycc_extxbgr_convert_internal;
pub use crate::jdcolext_c::ycc_extxrgb_convert_internal;
pub use crate::jdcolext_c::ycc_rgb_convert_internal;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::rgb_pixelsize;
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
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE_5;
pub use crate::jmorecfg_h::RGB_GREEN_5;
pub use crate::jmorecfg_h::RGB_PIXELSIZE_5;
pub use crate::jmorecfg_h::RGB_RED_5;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jcopy_sample_rows;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
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
pub use crate::src::jdcolor::jmorecfg_h::rgb_blue;
pub use crate::src::jdcolor::jmorecfg_h::rgb_green;
pub use crate::src::jdcolor::jmorecfg_h::rgb_red;
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
use crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb;
use crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb565;
use crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb565_convert;
use crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb_convert;

pub type my_cconvert_ptr = *mut my_color_deconverter;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_deconverter {
    pub pub_0: crate::jpeglib_h::jpeg_color_deconverter,
    pub Cr_r_tab: *mut libc::c_int,
    pub Cb_b_tab: *mut libc::c_int,
    pub Cr_g_tab: *mut crate::jpegint_h::JLONG,
    pub Cb_g_tab: *mut crate::jpegint_h::JLONG,
    pub rgb_y_tab: *mut crate::jpegint_h::JLONG,
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

pub const SCALEBITS: libc::c_int = 16i32;
/* speediest right-shift on some machines */

pub const ONE_HALF: crate::jpegint_h::JLONG = (1i64) << SCALEBITS - 1i32;
/* We allocate one big table for RGB->Y conversion and divide it up into
 * three parts, instead of doing three alloc_small requests.  This lets us
 * use a single table base address, which can be held in a register in the
 * inner loops on many machines (more than can hold all three addresses,
 * anyway).
 */

pub const R_Y_OFF: libc::c_int = 0i32;
/* offset to R => Y section */

pub const G_Y_OFF: libc::c_int = 1i32 * (crate::jmorecfg_h::MAXJSAMPLE + 1i32);
/* offset to G => Y section */

pub const B_Y_OFF: libc::c_int = 2i32 * (crate::jmorecfg_h::MAXJSAMPLE + 1i32);
/* etc. */

pub const TABLE_SIZE: libc::c_int = 3i32 * (crate::jmorecfg_h::MAXJSAMPLE + 1i32);
/* Include inline routines for colorspace extensions */

pub const RGB_RED_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_RED;

pub const RGB_GREEN_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_GREEN;

pub const RGB_BLUE_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_BLUE;

pub const RGB_PIXELSIZE_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_PIXELSIZE;

pub const RGB_RED_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_RED;

pub const RGB_GREEN_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_GREEN;

pub const RGB_BLUE_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_BLUE;

pub const RGB_ALPHA_2: libc::c_int = 3i32;

pub const RGB_PIXELSIZE_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;

pub const RGB_RED_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_RED;

pub const RGB_GREEN_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_GREEN;

pub const RGB_BLUE_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_BLUE;

pub const RGB_PIXELSIZE_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_PIXELSIZE;

pub const RGB_RED_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_RED;

pub const RGB_GREEN_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_GREEN;

pub const RGB_BLUE_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_BLUE;

pub const RGB_ALPHA_1: libc::c_int = 3i32;

pub const RGB_PIXELSIZE_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;

pub const RGB_RED_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_RED;

pub const RGB_GREEN_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_GREEN;

pub const RGB_BLUE_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_BLUE;

pub const RGB_ALPHA_0: libc::c_int = 0i32;

pub const RGB_PIXELSIZE_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;

pub const RGB_RED: libc::c_int = crate::jmorecfg_h::EXT_XRGB_RED;

pub const RGB_GREEN: libc::c_int = crate::jmorecfg_h::EXT_XRGB_GREEN;

pub const RGB_BLUE: libc::c_int = crate::jmorecfg_h::EXT_XRGB_BLUE;

pub const RGB_ALPHA: libc::c_int = 0i32;

pub const RGB_PIXELSIZE: libc::c_int = crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
/*
 * Initialize tables for YCC->RGB colorspace conversion.
 */

unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
      let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    
    
    (*cconvert).Cr_r_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong *
    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    (*cconvert).Cb_b_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong *
    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    (*cconvert).Cr_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong *
    ::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong,
    ) as *mut crate::jpegint_h::JLONG;
    (*cconvert).Cb_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong *
    ::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong,
    ) as *mut crate::jpegint_h::JLONG;
    
     let mut i:   libc::c_int =  0i32; let mut x:   crate::jpegint_h::JLONG =
     -crate::jmorecfg_h::CENTERJSAMPLE as crate::jpegint_h::JLONG;
    while i <= crate::jmorecfg_h::MAXJSAMPLE {
        /* i is the actual input pixel value, in the range 0..MAXJSAMPLE */
        /* The Cb or Cr value we are thinking of is x = i - CENTERJSAMPLE */
        /* Cr=>R value is nearest int to 1.40200 * x */
        *(*cconvert).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * (1i64 << 16i32) as libc::c_double + 0.5f64) as crate::jpegint_h::JLONG
                * x
                + ((1i64) << 16i32 - 1i32)
                >> 16i32) as libc::c_int;
        /* Cb=>B value is nearest int to 1.77200 * x */
        *(*cconvert).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * (1i64 << 16i32) as libc::c_double + 0.5f64) as crate::jpegint_h::JLONG
                * x
                + ((1i64) << 16i32 - 1i32)
                >> 16i32) as libc::c_int;
        /* Cr=>G value is scaled-up -0.71414 * x */
        *(*cconvert).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * (1i64 << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        *(*cconvert).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * (1i64 << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * x
                + ONE_HALF;
        i += 1;
        x += 1
    }
}
/*
 * Convert some rows of samples to the output colorspace.
 */

unsafe extern "C" fn ycc_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    match  (*cinfo).out_color_space {
        6 => {
            crate::jdcolext_c::ycc_extrgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        7 | 12 => {
            crate::jdcolext_c::ycc_extrgbx_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        8 => {
            crate::jdcolext_c::ycc_extbgr_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        9 | 13 => {
            crate::jdcolext_c::ycc_extbgrx_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        10 | 14 => {
            crate::jdcolext_c::ycc_extxbgr_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        11 | 15 => {
            crate::jdcolext_c::ycc_extxrgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        _ => {
            crate::jdcolext_c::ycc_rgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
    };
}
/* *************** Cases other than YCbCr -> RGB **************/
/*
 * Initialize for RGB->grayscale colorspace conversion.
 */

unsafe extern "C" fn build_rgb_y_table(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
      let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    
    
     let mut rgb_y_tab:   *mut crate::jpegint_h::JLONG =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        TABLE_SIZE as libc::c_ulong *
    ::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong,
    ) as *mut crate::jpegint_h::JLONG;
    (*cconvert).rgb_y_tab = rgb_y_tab;
     let mut i:   crate::jpegint_h::JLONG =  0i64;
    while i <= crate::jmorecfg_h::MAXJSAMPLE as libc::c_long {
        *rgb_y_tab.offset((i + R_Y_OFF as libc::c_long) as isize) =
            (0.29900f64 * (1i64 << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i;
        *rgb_y_tab.offset((i + G_Y_OFF as libc::c_long) as isize) =
            (0.58700f64 * (1i64 << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i;
        *rgb_y_tab.offset((i + B_Y_OFF as libc::c_long) as isize) =
            (0.11400f64 * (1i64 << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i
                + ONE_HALF;
        i += 1
    }
}
/*
 * Convert RGB to grayscale.
 */

unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    
    
    
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_y_tab;
    
    
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
             num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh42 = output_buf;
        output_buf = output_buf.offset(1);
        
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh42; let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols {
               
            
             let mut r:   libc::c_int =  *inptr0.offset(col as isize) as libc::c_int; let mut g:   libc::c_int =  *inptr1.offset(col as isize) as libc::c_int; let mut b:   libc::c_int =  *inptr2.offset(col as isize) as libc::c_int;
            /* Y */
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col +=  1
        }
    }
}
/*
 * Color conversion for no colorspace change: just copy the data,
 * converting from separate-planes to interleaved representation.
 */

unsafe extern "C" fn null_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    
    
    
    
    
    
     let mut inptr0:  crate::jpeglib_h::JSAMPROW =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>(); let mut inptr1:  crate::jpeglib_h::JSAMPROW =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>(); let mut inptr2:  crate::jpeglib_h::JSAMPROW =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>(); let mut outptr:  crate::jpeglib_h::JSAMPROW =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>(); let mut col:  crate::jmorecfg_h::JDIMENSION =  0;
    let mut num_components: libc::c_int = (*cinfo).num_components;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    
    if num_components == 3i32 {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
            input_row +=  1;
            let fresh43 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh43;
            col = 0u32;
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
                col +=  1
            }
        }
    } else if num_components == 4i32 {
        loop {
             num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
             let mut inptr3:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(3)).offset(input_row as isize);
            input_row +=  1;
            let fresh47 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh47;
            col = 0u32;
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
                col +=  1
            }
        }
    } else {
        loop {
             num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
             let mut ci:   libc::c_int =  0i32;
            while ci < num_components {
                  let mut inptr:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(ci as isize)).offset(input_row as isize);
                outptr = *output_buf;
                col = 0u32;
                while col < num_cols {
                    *outptr.offset(ci as isize) = *inptr.offset(col as isize);
                    outptr = outptr.offset(num_components as isize);
                    col +=  1
                }
                ci += 1
            }
            output_buf = output_buf.offset(1);
            input_row +=  1
        }
    };
}
/*
 * Color conversion for grayscale: just copy the data.
 * This also works for YCbCr -> grayscale conversion, in which
 * we just copy the Y (luminance) component and ignore chrominance.
 */

unsafe extern "C" fn grayscale_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    crate::jpegint_h::jcopy_sample_rows(
        *input_buf.offset(0),
        input_row as libc::c_int,
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    match  (*cinfo).out_color_space {
        6 => {
            crate::jdcolext_c::gray_extrgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        7 | 12 => {
            crate::jdcolext_c::gray_extrgbx_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        8 => {
            crate::jdcolext_c::gray_extbgr_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        9 | 13 => {
            crate::jdcolext_c::gray_extbgrx_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        10 | 14 => {
            crate::jdcolext_c::gray_extxbgr_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        11 | 15 => {
            crate::jdcolext_c::gray_extxrgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        _ => {
            crate::jdcolext_c::gray_rgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
    };
}
/*
 * Convert plain RGB to extended RGB
 */

unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    match  (*cinfo).out_color_space {
        6 => {
            crate::jdcolext_c::rgb_extrgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        7 | 12 => {
            crate::jdcolext_c::rgb_extrgbx_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        8 => {
            crate::jdcolext_c::rgb_extbgr_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        9 | 13 => {
            crate::jdcolext_c::rgb_extbgrx_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        10 | 14 => {
            crate::jdcolext_c::rgb_extxbgr_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        11 | 15 => {
            crate::jdcolext_c::rgb_extxrgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
        }
        _ => {
            crate::jdcolext_c::rgb_rgb_convert_internal(
                cinfo, input_buf, input_row, output_buf, num_rows,
            );
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    
    
    
    
    
    
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
              num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize); let mut inptr3:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(3)).offset(input_row as isize);
        input_row +=  1;
        let fresh52 = output_buf;
        output_buf = output_buf.offset(1);
        
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh52; let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols {
               
            
             let mut y:   libc::c_int =  *inptr0.offset(col as isize) as libc::c_int; let mut cb:   libc::c_int =  *inptr1.offset(col as isize) as libc::c_int; let mut cr:   libc::c_int =  *inptr2.offset(col as isize) as libc::c_int;
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            *outptr.offset(0) = *range_limit.offset(
                (crate::jmorecfg_h::MAXJSAMPLE - (y + *Crrtab.offset(cr as isize))) as isize,
            ); /* red */
            *outptr.offset(1) = *range_limit.offset(
                (crate::jmorecfg_h::MAXJSAMPLE
                    - (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                        as libc::c_int)) as isize,
            ); /* blue */
            *outptr.offset(2) = *range_limit.offset(
                (crate::jmorecfg_h::MAXJSAMPLE - (y + *Cbbtab.offset(cb as isize))) as isize,
            );
            /* K passes through unchanged */
            *outptr.offset(3) = *inptr3.offset(col as isize); /* don't need GETJSAMPLE here */
            outptr = outptr.offset(4);
            col +=  1
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

pub const DITHER_MASK: libc::c_int = 0x3i32;

pub(crate) static mut dither_matrix: [crate::jpegint_h::JLONG; 4] = [
    0x8020ai64,
    0xc040e06i64,
    0x30b0109i64,
    0xf070d05i64,
];
#[inline(always)]

unsafe extern "C" fn is_big_endian() -> crate::jmorecfg_h::boolean {
     let mut test_value:  libc::c_int =  1i32;
    if *(&mut test_value as *mut libc::c_int as *mut libc::c_char) as libc::c_int != 1i32 {
        return crate::jmorecfg_h::TRUE;
    }
    return crate::jmorecfg_h::FALSE;
}
/* Include inline routines for RGB565 conversion */

unsafe extern "C" fn ycc_rgb565_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        crate::jdcol565_c::ycc_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        crate::jdcol565_c::ycc_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}

unsafe extern "C" fn ycc_rgb565D_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        crate::jdcol565_c::ycc_rgb565D_convert_be(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    } else {
        crate::jdcol565_c::ycc_rgb565D_convert_le(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    };
}

unsafe extern "C" fn rgb_rgb565_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        crate::jdcol565_c::rgb_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        crate::jdcol565_c::rgb_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}

unsafe extern "C" fn rgb_rgb565D_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        crate::jdcol565_c::rgb_rgb565D_convert_be(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    } else {
        crate::jdcol565_c::rgb_rgb565D_convert_le(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    };
}

unsafe extern "C" fn gray_rgb565_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        crate::jdcol565_c::gray_rgb565_convert_be(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    } else {
        crate::jdcol565_c::gray_rgb565_convert_le(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    };
}

unsafe extern "C" fn gray_rgb565D_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        crate::jdcol565_c::gray_rgb565D_convert_be(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    } else {
        crate::jdcol565_c::gray_rgb565D_convert_le(
            cinfo, input_buf, input_row, output_buf, num_rows,
        );
    };
}
/*
 * Empty method for start_pass.
 */

unsafe extern "C" fn start_pass_dcolor(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    /* no work needed */
}
/*
 * jpegint.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 1997-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015-2016, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file provides common declarations for the various JPEG modules.
 * These declarations are considered internal to the JPEG library; most
 * applications using the library shouldn't need to include this file.
 */
/* Declarations for both compression & decompression */
/* Operating modes for buffer controllers */
/* Plain stripwise operation */
/* Remaining modes require a full-image buffer to have been created */
/* Run source subobject only, save output */
/* Run dest subobject only, using saved data */
/* Run both subobjects, save output */
/* Requantize */
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */
/* after create_compress */
/* start_compress done, write_scanlines OK */
/* start_compress done, write_raw_data OK */
/* jpeg_write_coefficients done */
/* after create_decompress */
/* reading header markers, no SOS yet */
/* found SOS, ready for start_decompress */
/* reading multiscan file in start_decompress*/
/* performing dummy pass for 2-pass quant */
/* start_decompress done, read_scanlines OK */
/* start_decompress done, read_raw_data OK */
/* expecting jpeg_start_output */
/* looking for SOS/EOI in jpeg_finish_output */
/* reading file in jpeg_read_coefficients */
/* looking for EOI in jpeg_finish_decompress */
/* JLONG must hold at least signed 32-bit values. */
/*
 * Left shift macro that handles a negative operand without causing any
 * sanitizer warnings
 */
/* Declarations for compression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True if pass_startup must be called */
/* True during last pass */
/* Extension parameters */
/* TRUE=optimize progressive coding scans */
/* TRUE=use trellis quantization */
/* TRUE=use trellis quant for DC coefficient */
/* TRUE=optimize for sequences of EOB */
/* TRUE=use lambda weighting table */
/* TRUE=use scans in trellis optimization */
/* TRUE=currently doing trellis-related passes [not exposed] */
/* TRUE=optimize quant table in trellis loop */
/* TRUE=preprocess input to reduce ringing of edges on white background */
/* compression profile */
/* DC scan optimization mode */
/* Quantization table master index */
/* splitting point for frequency in trellis quantization */
/* number of trellis loops */
/* # of entries in scan_info array pertaining to luma (used when optimize_scans is TRUE */
/* maximum value of Al tested when optimizing scans (luma) */
/* maximum value of Al tested when optimizing scans (chroma) */
/* Main buffer control (downsampled-data buffer) */
/* Compression preprocessing (downsampling input buffer control) */
/* Coefficient buffer control */
/* Colorspace conversion */
/* Downsampling */
/* TRUE if need rows above & below */
/* Forward DCT (also controls coefficient quantization) */
/* perhaps this should be an array??? */
/* Entropy encoding */
/* Marker writing */
/* These routines are exported to allow insertion of extra markers */
/* Probably only COM and APPn markers should be written this way */
/* Declarations for decompression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True during 1st pass for 2-pass quant */
/* Partial decompression variables */
/* Input control module */
/* State variables made visible to other modules */
/* True if file has multiple scans */
/* True when EOI has been consumed */
/* Main buffer control (downsampled-data buffer) */
/* Coefficient buffer control */
/* Pointer to array of coefficient virtual arrays, or NULL if none */
/* Decompression postprocessing (color quantization buffer control) */
/* Marker reading & parsing */
/* Read markers until SOS or EOI.
 * Returns same codes as are defined for jpeg_consume_input:
 * JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 */
/* Read a restart marker --- exported for use by entropy decoder only */
/* State of marker reader --- nominally internal, but applications
 * supplying COM or APPn handlers might like to know the state.
 */
/* found SOI? */
/* found SOF? */
/* next restart number expected (0-7) */
/* # of bytes skipped looking for a marker */
/* Entropy decoding */
/* This is here to share code between baseline and progressive decoders; */
/* other modules probably should not use it */
/* set TRUE after emitting warning */
/* Inverse DCT (also performs dequantization) */
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Compression module initialization routines */
/* Decompression module initialization routines */
/*
 * Module initialization routine for output colorspace conversion.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_color_deconverter(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    
     
     let mut cconvert:   my_cconvert_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_color_deconverter>() as libc::c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut crate::jpeglib_h::jpeg_color_deconverter;
    (*cconvert).pub_0.start_pass = Some(
        start_pass_dcolor as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    /* Make sure num_components agrees with jpeg_color_space */
    match  (*cinfo).jpeg_color_space {
        1 => {
            if (*cinfo).num_components != 1i32 {
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
        }
        2 | 3 => {
            if (*cinfo).num_components != 3i32 {
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
        }
        4 | 5 => {
            if (*cinfo).num_components != 4i32 {
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
        }
        _ => {
            /* JCS_UNKNOWN can be anything */
            if (*cinfo).num_components < 1i32 {
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
        }
    }
    /* Set out_color_components and conversion method based on requested space.
     * Also clear the component_needed flags for any unused components,
     * so that earlier pipeline stages can avoid useless computation.
     */
    match  (*cinfo).out_color_space {
        1 => {
            (*cinfo).out_color_components = 1i32;
            if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_GRAYSCALE
                ||  (*cinfo).jpeg_color_space
                    ==  crate::jpeglib_h::JCS_YCbCr
            {
                 (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                 let mut ci:   libc::c_int =  1i32;
                while ci < (*cinfo).num_components {
                    (*(*cinfo).comp_info.offset(ci as isize)).component_needed =
                        crate::jmorecfg_h::FALSE;
                    ci += 1
                }
            } else if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_RGB
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_gray_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                build_rgb_y_table(cinfo);
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            (*cinfo).out_color_components =
                crate::jmorecfg_h::rgb_pixelsize[(*cinfo).out_color_space as usize];
            if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_YCbCr
            {
                if crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    );
                    build_ycc_rgb_table(cinfo);
                }
            } else if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_GRAYSCALE
            {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_RGB
            {
                if rgb_red[(*cinfo).out_color_space as usize] == 0i32
                    && rgb_green[(*cinfo).out_color_space as usize] == 1i32
                    && rgb_blue[(*cinfo).out_color_space as usize] == 2i32
                    && crate::jmorecfg_h::rgb_pixelsize[(*cinfo).out_color_space as usize] == 3i32
                {
                    (*cconvert).pub_0.color_convert = Some(
                        null_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                }
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        16 => {
            (*cinfo).out_color_components = 3i32;
            if  (*cinfo).dither_mode
                ==  crate::jpeglib_h::JDITHER_NONE
            {
                if  (*cinfo).jpeg_color_space
                    ==  crate::jpeglib_h::JCS_YCbCr
                {
                    if crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb565() != 0 {
                        (*cconvert).pub_0.color_convert = Some(
                            crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: crate::jpeglib_h::j_decompress_ptr,
                                    _: crate::jpeglib_h::JSAMPIMAGE,
                                    _: crate::jmorecfg_h::JDIMENSION,
                                    _: crate::jpeglib_h::JSAMPARRAY,
                                    _: libc::c_int,
                                ) -> (),
                        )
                    } else {
                        (*cconvert).pub_0.color_convert = Some(
                            ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: crate::jpeglib_h::j_decompress_ptr,
                                    _: crate::jpeglib_h::JSAMPIMAGE,
                                    _: crate::jmorecfg_h::JDIMENSION,
                                    _: crate::jpeglib_h::JSAMPARRAY,
                                    _: libc::c_int,
                                ) -> (),
                        );
                        build_ycc_rgb_table(cinfo);
                    }
                } else if  (*cinfo).jpeg_color_space
                    ==  crate::jpeglib_h::JCS_GRAYSCALE
                {
                    (*cconvert).pub_0.color_convert = Some(
                        gray_rgb565_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else if  (*cinfo).jpeg_color_space
                    ==  crate::jpeglib_h::JCS_RGB
                {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb565_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*(*cinfo).err).msg_code =
                        crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
            } else if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_YCbCr
            {
                (*cconvert).pub_0.color_convert = Some(
                    ycc_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_GRAYSCALE
            {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_RGB
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        4 => {
            (*cinfo).out_color_components = 4i32;
            if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_YCCK
            {
                (*cconvert).pub_0.color_convert = Some(
                    ycck_cmyk_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if  (*cinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_CMYK
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        _ => {
            /* only ordered dithering is supported */
            /* Permit null conversion to same output space */
            if  (*cinfo).out_color_space ==  (*cinfo).jpeg_color_space
            {
                (*cinfo).out_color_components = (*cinfo).num_components; /* unsupported non-null conversion */
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int; /* single colormapped output component */
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
    }
    if (*cinfo).quantize_colors != 0 {
        (*cinfo).output_components = 1i32
    } else {
        (*cinfo).output_components = (*cinfo).out_color_components
    };
}
