use libc;

pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::rgb_blue;
pub use crate::jmorecfg_h::rgb_green;
pub use crate::jmorecfg_h::rgb_red;
pub use crate::jmorecfg_h::EXT_BGRX_BLUE;
pub use crate::jmorecfg_h::EXT_BGRX_GREEN;
pub use crate::jmorecfg_h::EXT_BGRX_RED;
pub use crate::jmorecfg_h::EXT_BGR_BLUE;
pub use crate::jmorecfg_h::EXT_BGR_GREEN;
pub use crate::jmorecfg_h::EXT_BGR_RED;
pub use crate::jmorecfg_h::EXT_RGBX_BLUE;
pub use crate::jmorecfg_h::EXT_RGBX_GREEN;
pub use crate::jmorecfg_h::EXT_RGBX_RED;
pub use crate::jmorecfg_h::EXT_RGB_BLUE;
pub use crate::jmorecfg_h::EXT_RGB_GREEN;
pub use crate::jmorecfg_h::EXT_RGB_RED;
pub use crate::jmorecfg_h::EXT_XBGR_BLUE;
pub use crate::jmorecfg_h::EXT_XBGR_GREEN;
pub use crate::jmorecfg_h::EXT_XBGR_RED;
pub use crate::jmorecfg_h::EXT_XRGB_BLUE;
pub use crate::jmorecfg_h::EXT_XRGB_GREEN;
pub use crate::jmorecfg_h::EXT_XRGB_RED;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_RED;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jzero_far;
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

pub type my_cquantize_ptr = *mut my_cquantizer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_cquantizer {
    pub pub_0: crate::jpeglib_h::jpeg_color_quantizer,
    pub sv_colormap: crate::jpeglib_h::JSAMPARRAY,
    pub desired: libc::c_int,
    pub histogram: hist3d,
    pub needs_zeroed: crate::jmorecfg_h::boolean,
    pub fserrors: FSERRPTR,
    pub on_odd_row: crate::jmorecfg_h::boolean,
    pub error_limiter: *mut libc::c_int,
}
/* use 'int' for calculation temps */

pub type FSERRPTR = *mut FSERROR;
/* type for top-level pointer */
/* Declarations for Floyd-Steinberg dithering.
 *
 * Errors are accumulated into the array fserrors[], at a resolution of
 * 1/16th of a pixel count.  The error at a given pixel is propagated
 * to its not-yet-processed neighbors using the standard F-S fractions,
 *              ...     (here)  7/16
 *              3/16    5/16    1/16
 * We work left-to-right on even rows, right-to-left on odd rows.
 *
 * We can get away with a single array (holding one row's worth of errors)
 * by using it to store the current row's errors at pixel columns not yet
 * processed, but the next row's errors at columns already processed.  We
 * need only a few extra variables to hold the errors immediately around the
 * current column.  (If we are lucky, those variables are in registers, but
 * even if not, they're probably cheaper to access than array elements are.)
 *
 * The fserrors[] array has (#columns + 2) entries; the extra entry at
 * each end saves us from special-casing the first and last pixels.
 * Each entry is three values long, one value for each color component.
 */

pub type FSERROR = crate::jmorecfg_h::INT16;
/* type for the 2nd-level pointers */

pub type hist3d = *mut hist2d;
/* typedefs for the array */

pub type hist2d = *mut hist1d;
/* for pointers to histogram cells */

pub type hist1d = [histcell; 32];

pub type histcell = crate::jmorecfg_h::UINT16;

pub type boxptr = *mut box_0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct box_0 {
    pub c0min: libc::c_int,
    pub c0max: libc::c_int,
    pub c1min: libc::c_int,
    pub c1max: libc::c_int,
    pub c2min: libc::c_int,
    pub c2max: libc::c_int,
    pub volume: crate::jpegint_h::JLONG,
    pub colorcount: libc::c_long,
}
/* histogram cell; prefer an unsigned type */

pub type histptr = *mut histcell;
/* 16 bits should be enough */

pub type LOCFSERROR = libc::c_int;
/*
 * jquant2.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2014-2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains 2-pass color quantization (color mapping) routines.
 * These routines provide selection of a custom color map for an image,
 * followed by mapping of the image to that color map, with optional
 * Floyd-Steinberg dithering.
 * It is also possible to use just the second pass to map to an arbitrary
 * externally-given color map.
 *
 * Note: ordered dithering is not supported, since there isn't any fast
 * way to compute intercolor distances; it's unclear that ordered dither's
 * fundamental assumptions even hold with an irregularly spaced color map.
 */
/*
 * This module implements the well-known Heckbert paradigm for color
 * quantization.  Most of the ideas used here can be traced back to
 * Heckbert's seminal paper
 *   Heckbert, Paul.  "Color Image Quantization for Frame Buffer Display",
 *   Proc. SIGGRAPH '82, Computer Graphics v.16 #3 (July 1982), pp 297-304.
 *
 * In the first pass over the image, we accumulate a histogram showing the
 * usage count of each possible color.  To keep the histogram to a reasonable
 * size, we reduce the precision of the input; typical practice is to retain
 * 5 or 6 bits per color, so that 8 or 4 different input values are counted
 * in the same histogram cell.
 *
 * Next, the color-selection step begins with a box representing the whole
 * color space, and repeatedly splits the "largest" remaining box until we
 * have as many boxes as desired colors.  Then the mean color in each
 * remaining box becomes one of the possible output colors.
 *
 * The second pass over the image maps each input pixel to the closest output
 * color (optionally after applying a Floyd-Steinberg dithering correction).
 * This mapping is logically trivial, but making it go fast enough requires
 * considerable care.
 *
 * Heckbert-style quantizers vary a good deal in their policies for choosing
 * the "largest" box and deciding where to cut it.  The particular policies
 * used here have proved out well in experimental comparisons, but better ones
 * may yet be found.
 *
 * In earlier versions of the IJG code, this module quantized in YCbCr color
 * space, processing the raw upsampled data without a color conversion step.
 * This allowed the color conversion math to be done only once per colormap
 * entry, not once per pixel.  However, that optimization precluded other
 * useful optimizations (such as merging color conversion with upsampling)
 * and it also interfered with desired capabilities such as quantizing to an
 * externally-supplied colormap.  We have therefore abandoned that approach.
 * The present code works in the post-conversion color space, typically RGB.
 *
 * To improve the visual quality of the results, we actually work in scaled
 * RGB space, giving G distances more weight than R, and R in turn more than
 * B.  To do everything in integer math, we must use integer scale factors.
 * The 2/3/1 scale factors used here correspond loosely to the relative
 * weights of the colors in the NTSC grayscale equation.
 * If you want to use this code to quantize a non-RGB color space, you'll
 * probably need to change these scale factors.
 */

pub const R_SCALE: libc::c_int = 2i32;
/* scale R distances by this much */

pub const G_SCALE: libc::c_int = 3i32;
/* scale G distances by this much */

pub const B_SCALE: libc::c_int = 1i32;
/* and B by this much */

static mut c_scales: [libc::c_int; 3] = [R_SCALE, G_SCALE, B_SCALE];
/*
 * First we have the histogram data structure and routines for creating it.
 *
 * The number of bits of precision can be adjusted by changing these symbols.
 * We recommend keeping 6 bits for G and 5 each for R and B.
 * If you have plenty of memory and cycles, 6 bits all around gives marginally
 * better results; if you are short of memory, 5 bits all around will save
 * some space but degrade the results.
 * To maintain a fully accurate histogram, we'd need to allocate a "long"
 * (preferably unsigned long) for each cell.  In practice this is overkill;
 * we can get by with 16 bits per cell.  Few of the cell counts will overflow,
 * and clamping those that do overflow to the maximum value will give close-
 * enough results.  This reduces the recommended histogram size from 256Kb
 * to 128Kb, which is a useful savings on PC-class machines.
 * (In the second pass the histogram space is re-used for pixel mapping data;
 * in that capacity, each cell must be able to store zero to the number of
 * desired colors.  16 bits/cell is plenty for that too.)
 * Since the JPEG code is intended to run in small memory model on 80x86
 * machines, we can't just allocate the histogram in one chunk.  Instead
 * of a true 3-D array, we use a row of pointers to 2-D arrays.  Each
 * pointer corresponds to a C0 value (typically 2^5 = 32 pointers) and
 * each 2-D array has 2^6*2^5 = 2048 or 2^6*2^6 = 4096 entries.
 */

pub const MAXNUMCOLORS: libc::c_int = crate::jmorecfg_h::MAXJSAMPLE + 1i32;
/* maximum size of colormap */
/* These will do the right thing for either R,G,B or B,G,R color order,
 * but you may not like the results for other color orders.
 */

pub const HIST_C0_BITS: libc::c_int = 5i32;
/* bits of precision in R/B histogram */

pub const HIST_C1_BITS: libc::c_int = 6i32;
/* bits of precision in G histogram */

pub const HIST_C2_BITS: libc::c_int = 5i32;
/* bits of precision in B/R histogram */
/* Number of elements along histogram axes. */

pub const HIST_C0_ELEMS: libc::c_int = 1i32 << HIST_C0_BITS;

pub const HIST_C1_ELEMS: libc::c_int = 1i32 << HIST_C1_BITS;

pub const HIST_C2_ELEMS: libc::c_int = 1i32 << HIST_C2_BITS;
/* These are the amounts to shift an input value to get a histogram index. */

pub const C0_SHIFT: libc::c_int = crate::jconfig_h::BITS_IN_JSAMPLE - HIST_C0_BITS;

pub const C1_SHIFT: libc::c_int = crate::jconfig_h::BITS_IN_JSAMPLE - HIST_C1_BITS;

pub const C2_SHIFT: libc::c_int = crate::jconfig_h::BITS_IN_JSAMPLE - HIST_C2_BITS;
/*
 * Prescan some rows of pixels.
 * In this module the prescan simply updates the histogram, which has been
 * initialized to zeroes by start_pass.
 * An output_buf parameter is required by the method signature, but no data
 * is actually output (in fact the buffer controller is probably passing a
 * NULL pointer).
 */

unsafe extern "C" fn prescan_quantize(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut histp: histptr = ::std::ptr::null_mut::< histcell>();
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0i32;
    while row < num_rows {
        ptr = *input_buf.offset(row as isize);
        col = width;
        while col > 0i32 as libc::c_uint {
            /* get pixel value and index into the histogram */
            histp = &mut *(*(*histogram
                .offset((*ptr.offset(0) as libc::c_int >> C0_SHIFT) as isize))
            .offset((*ptr.offset(1) as libc::c_int >> C1_SHIFT) as isize))
            .as_mut_ptr()
            .offset((*ptr.offset(2) as libc::c_int >> C2_SHIFT) as isize)
                as *mut histcell;
            /* increment, check for overflow and undo increment if so. */
            *histp = *histp + 1;
            if *histp as libc::c_int <= 0i32 {
                *histp = *histp - 1
            }
            ptr = ptr.offset(3);
            col =  col - 1
        }
        row += 1
    }
}

unsafe extern "C" fn find_biggest_color_pop(
    mut boxlist: boxptr,
    mut numboxes: libc::c_int,
) -> boxptr
/* Find the splittable box with the largest color population */
/* Returns NULL if no splittable boxes remain */ {
    let mut boxp: boxptr = ::std::ptr::null_mut::< box_0>();
    let mut i: libc::c_int = 0;
    let mut maxc: libc::c_long = 0i32 as libc::c_long;
    let mut which: boxptr = crate::stddef_h::NULL as boxptr;
    i = 0i32;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).colorcount > maxc && (*boxp).volume > 0i32 as libc::c_long {
            which = boxp;
            maxc = (*boxp).colorcount
        }
        i += 1;
        boxp = boxp.offset(1)
    }
    return which;
}

unsafe extern "C" fn find_biggest_volume(mut boxlist: boxptr, mut numboxes: libc::c_int) -> boxptr
/* Find the splittable box with the largest (scaled) volume */
/* Returns NULL if no splittable boxes remain */ {
    let mut boxp: boxptr = ::std::ptr::null_mut::< box_0>();
    let mut i: libc::c_int = 0;
    let mut maxv: crate::jpegint_h::JLONG = 0i32 as crate::jpegint_h::JLONG;
    let mut which: boxptr = crate::stddef_h::NULL as boxptr;
    i = 0i32;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).volume > maxv {
            which = boxp;
            maxv = (*boxp).volume
        }
        i += 1;
        boxp = boxp.offset(1)
    }
    return which;
}

unsafe extern "C" fn update_box(mut cinfo: crate::jpeglib_h::j_decompress_ptr, mut boxp: boxptr)
/* Shrink the min/max bounds of a box to enclose only nonzero elements, */
/* and recompute its volume and population */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = ::std::ptr::null_mut::< histcell>();
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut c0min: libc::c_int = 0;
    let mut c0max: libc::c_int = 0;
    let mut c1min: libc::c_int = 0;
    let mut c1max: libc::c_int = 0;
    let mut c2min: libc::c_int = 0;
    let mut c2max: libc::c_int = 0;
    let mut dist0: crate::jpegint_h::JLONG = 0;
    let mut dist1: crate::jpegint_h::JLONG = 0;
    let mut dist2: crate::jpegint_h::JLONG = 0;
    let mut ccount: libc::c_long = 0;
    c0min = (*boxp).c0min;
    c0max = (*boxp).c0max;
    c1min = (*boxp).c1min;
    c1max = (*boxp).c1max;
    c2min = (*boxp).c2min;
    c2max = (*boxp).c2max;
    if c0max > c0min {
        c0 = c0min;
        's_50: while c0 <= c0max {
            c1 = c1min;
            while c1 <= c1max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh0 = histp;
                    histp = histp.offset(1);
                    if *fresh0 as libc::c_int != 0i32 {
                        c0min = c0;
                        (*boxp).c0min = c0min;
                        break 's_50;
                    } else {
                        c2 += 1
                    }
                }
                c1 += 1
            }
            c0 += 1
        }
    }
    if c0max > c0min {
        c0 = c0max;
        's_105: while c0 >= c0min {
            c1 = c1min;
            while c1 <= c1max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh1 = histp;
                    histp = histp.offset(1);
                    if *fresh1 as libc::c_int != 0i32 {
                        c0max = c0;
                        (*boxp).c0max = c0max;
                        break 's_105;
                    } else {
                        c2 += 1
                    }
                }
                c1 += 1
            }
            c0 -= 1
        }
    }
    if c1max > c1min {
        c1 = c1min;
        's_162: while c1 <= c1max {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh2 = histp;
                    histp = histp.offset(1);
                    if *fresh2 as libc::c_int != 0i32 {
                        c1min = c1;
                        (*boxp).c1min = c1min;
                        break 's_162;
                    } else {
                        c2 += 1
                    }
                }
                c0 += 1
            }
            c1 += 1
        }
    }
    if c1max > c1min {
        c1 = c1max;
        's_219: while c1 >= c1min {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh3 = histp;
                    histp = histp.offset(1);
                    if *fresh3 as libc::c_int != 0i32 {
                        c1max = c1;
                        (*boxp).c1max = c1max;
                        break 's_219;
                    } else {
                        c2 += 1
                    }
                }
                c0 += 1
            }
            c1 -= 1
        }
    }
    if c2max > c2min {
        c2 = c2min;
        's_276: while c2 <= c2max {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1min as isize))
                    .as_mut_ptr()
                    .offset(c2 as isize) as *mut histcell;
                c1 = c1min;
                while c1 <= c1max {
                    if *histp as libc::c_int != 0i32 {
                        c2min = c2;
                        (*boxp).c2min = c2min;
                        break 's_276;
                    } else {
                        c1 += 1;
                        histp = histp.offset(HIST_C2_ELEMS as isize)
                    }
                }
                c0 += 1
            }
            c2 += 1
        }
    }
    if c2max > c2min {
        c2 = c2max;
        's_333: while c2 >= c2min {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1min as isize))
                    .as_mut_ptr()
                    .offset(c2 as isize) as *mut histcell;
                c1 = c1min;
                while c1 <= c1max {
                    if *histp as libc::c_int != 0i32 {
                        c2max = c2;
                        (*boxp).c2max = c2max;
                        break 's_333;
                    } else {
                        c1 += 1;
                        histp = histp.offset(HIST_C2_ELEMS as isize)
                    }
                }
                c0 += 1
            }
            c2 -= 1
        }
    }
    /* Update box volume.
     * We use 2-norm rather than real volume here; this biases the method
     * against making long narrow boxes, and it has the side benefit that
     * a box is splittable iff norm > 0.
     * Since the differences are expressed in histogram-cell units,
     * we have to shift back to JSAMPLE units to get consistent distances;
     * after which, we scale according to the selected distance scale factors.
     */
    dist0 = ((c0max - c0min << C0_SHIFT)
        * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
        as crate::jpegint_h::JLONG;
    dist1 = ((c1max - c1min << C1_SHIFT)
        * c_scales[crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
        as crate::jpegint_h::JLONG;
    dist2 = ((c2max - c2min << C2_SHIFT)
        * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
        as crate::jpegint_h::JLONG;
    (*boxp).volume = dist0 * dist0 + dist1 * dist1 + dist2 * dist2;
    /* Now scan remaining volume of box and compute population */
    ccount = 0i32 as libc::c_long;
    c0 = c0min;
    while c0 <= c0max {
        c1 = c1min;
        while c1 <= c1max {
            histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2min as isize) as *mut histcell;
            c2 = c2min;
            while c2 <= c2max {
                if *histp as libc::c_int != 0i32 {
                    ccount += 1
                }
                c2 += 1;
                histp = histp.offset(1)
            }
            c1 += 1
        }
        c0 += 1
    }
    (*boxp).colorcount = ccount;
}

unsafe extern "C" fn median_cut(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut boxlist: boxptr,
    mut numboxes: libc::c_int,
    mut desired_colors: libc::c_int,
) -> libc::c_int
/* Repeatedly select and split the largest box until we have enough boxes */ {
    let mut n: libc::c_int = 0;
    let mut lb: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut cmax: libc::c_int = 0;
    let mut b1: boxptr = ::std::ptr::null_mut::< box_0>();
    let mut b2: boxptr = ::std::ptr::null_mut::< box_0>();
    while numboxes < desired_colors {
        /* Select box to split.
         * Current algorithm: by population for first half, then by volume.
         */
        if numboxes * 2i32 <= desired_colors {
            b1 = find_biggest_color_pop(boxlist, numboxes)
        } else {
            b1 = find_biggest_volume(boxlist, numboxes)
        } /* where new box will go */
        if b1.is_null() {
            break;
        }
        b2 = &mut *boxlist.offset(numboxes as isize) as *mut box_0;
        /* Copy the color bounds to the new box. */
        (*b2).c0max = (*b1).c0max;
        (*b2).c1max = (*b1).c1max;
        (*b2).c2max = (*b1).c2max;
        (*b2).c0min = (*b1).c0min;
        (*b2).c1min = (*b1).c1min;
        (*b2).c2min = (*b1).c2min;
        /* Choose which axis to split the box on.
         * Current algorithm: longest scaled axis.
         * See notes in update_box about scaling distances.
         */
        c0 = ((*b1).c0max - (*b1).c0min << C0_SHIFT)
            * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize];
        c1 = ((*b1).c1max - (*b1).c1min << C1_SHIFT)
            * c_scales[crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize];
        c2 = ((*b1).c2max - (*b1).c2min << C2_SHIFT)
            * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize];
        /* We want to break any ties in favor of green, then red, blue last.
         * This code does the right thing for R,G,B or B,G,R color orders only.
         */
        if crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] == 0i32 {
            cmax = c1;
            n = 1i32;
            if c0 > cmax {
                cmax = c0;
                n = 0i32
            }
            if c2 > cmax {
                n = 2i32
            }
        } else {
            cmax = c1;
            n = 1i32;
            if c2 > cmax {
                cmax = c2;
                n = 2i32
            }
            if c0 > cmax {
                n = 0i32
            }
        }
        /* Choose split point along selected axis, and update box bounds.
         * Current algorithm: split at halfway point.
         * (Since the box has been shrunk to minimum volume,
         * any split will produce two nonempty subboxes.)
         * Note that lb value is max for lower box, so must be < old max.
         */
        match n {
            0 => {
                lb = ((*b1).c0max + (*b1).c0min) / 2i32;
                (*b1).c0max = lb;
                (*b2).c0min = lb + 1i32
            }
            1 => {
                lb = ((*b1).c1max + (*b1).c1min) / 2i32;
                (*b1).c1max = lb;
                (*b2).c1min = lb + 1i32
            }
            2 => {
                lb = ((*b1).c2max + (*b1).c2min) / 2i32;
                (*b1).c2max = lb;
                (*b2).c2min = lb + 1i32
            }
            _ => {}
        }
        /* Update stats for boxes */
        update_box(cinfo, b1);
        update_box(cinfo, b2);
        numboxes += 1
    }
    return numboxes;
}

unsafe extern "C" fn compute_color(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut boxp: boxptr,
    mut icolor: libc::c_int,
)
/* Compute representative color for a box, put it in colormap[icolor] */
{
    /* Current algorithm: mean weighted by pixels (not colors) */
    /* Note it is important to get the rounding correct! */
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = ::std::ptr::null_mut::< histcell>();
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut c0min: libc::c_int = 0;
    let mut c0max: libc::c_int = 0;
    let mut c1min: libc::c_int = 0;
    let mut c1max: libc::c_int = 0;
    let mut c2min: libc::c_int = 0;
    let mut c2max: libc::c_int = 0;
    let mut count: libc::c_long = 0;
    let mut total: libc::c_long = 0i32 as libc::c_long;
    let mut c0total: libc::c_long = 0i32 as libc::c_long;
    let mut c1total: libc::c_long = 0i32 as libc::c_long;
    let mut c2total: libc::c_long = 0i32 as libc::c_long;
    c0min = (*boxp).c0min;
    c0max = (*boxp).c0max;
    c1min = (*boxp).c1min;
    c1max = (*boxp).c1max;
    c2min = (*boxp).c2min;
    c2max = (*boxp).c2max;
    c0 = c0min;
    while c0 <= c0max {
        c1 = c1min;
        while c1 <= c1max {
            histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2min as isize) as *mut histcell;
            c2 = c2min;
            while c2 <= c2max {
                let fresh4 = histp;
                histp = histp.offset(1);
                count = *fresh4 as libc::c_long;
                if count != 0i32 as libc::c_long {
                    total += count;
                    c0total +=
                        ((c0 << C0_SHIFT) + (1i32 << C0_SHIFT >> 1i32)) as libc::c_long * count;
                    c1total +=
                        ((c1 << C1_SHIFT) + (1i32 << C1_SHIFT >> 1i32)) as libc::c_long * count;
                    c2total +=
                        ((c2 << C2_SHIFT) + (1i32 << C2_SHIFT >> 1i32)) as libc::c_long * count
                }
                c2 += 1
            }
            c1 += 1
        }
        c0 += 1
    }
    *(*(*cinfo).colormap.offset(0)).offset(icolor as isize) =
        ((c0total + (total >> 1i32)) / total) as crate::jmorecfg_h::JSAMPLE;
    *(*(*cinfo).colormap.offset(1)).offset(icolor as isize) =
        ((c1total + (total >> 1i32)) / total) as crate::jmorecfg_h::JSAMPLE;
    *(*(*cinfo).colormap.offset(2)).offset(icolor as isize) =
        ((c2total + (total >> 1i32)) / total) as crate::jmorecfg_h::JSAMPLE;
}

unsafe extern "C" fn select_colors(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut desired_colors: libc::c_int,
)
/* Master routine for color selection */
{
    let mut boxlist: boxptr = ::std::ptr::null_mut::< box_0>();
    let mut numboxes: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Allocate workspace for box list */
    boxlist = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        desired_colors as libc::c_ulong *
    ::std::mem::size_of::<box_0>() as libc::c_ulong,
    ) as boxptr;
    /* Initialize one box containing whole space */
    numboxes = 1i32;
    (*boxlist.offset(0)).c0min = 0i32;
    (*boxlist.offset(0)).c0max = crate::jmorecfg_h::MAXJSAMPLE >> C0_SHIFT;
    (*boxlist.offset(0)).c1min = 0i32;
    (*boxlist.offset(0)).c1max = crate::jmorecfg_h::MAXJSAMPLE >> C1_SHIFT;
    (*boxlist.offset(0)).c2min = 0i32;
    (*boxlist.offset(0)).c2max = crate::jmorecfg_h::MAXJSAMPLE >> C2_SHIFT;
    /* Shrink it to actually-used volume and set its statistics */
    update_box(cinfo, &mut *boxlist.offset(0));
    /* Perform median-cut to produce final box list */
    numboxes = median_cut(cinfo, boxlist, numboxes, desired_colors);
    /* Compute the representative color for each box, fill colormap */
    i = 0i32;
    while i < numboxes {
        compute_color(cinfo, &mut *boxlist.offset(i as isize), i);
        i += 1
    }
    (*cinfo).actual_number_of_colors = numboxes;
    (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_QUANT_SELECTED as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0] = numboxes;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
}
/*
 * These routines are concerned with the time-critical task of mapping input
 * colors to the nearest color in the selected colormap.
 *
 * We re-use the histogram space as an "inverse color map", essentially a
 * cache for the results of nearest-color searches.  All colors within a
 * histogram cell will be mapped to the same colormap entry, namely the one
 * closest to the cell's center.  This may not be quite the closest entry to
 * the actual input color, but it's almost as good.  A zero in the cache
 * indicates we haven't found the nearest color for that cell yet; the array
 * is cleared to zeroes before starting the mapping pass.  When we find the
 * nearest color for a cell, its colormap index plus one is recorded in the
 * cache for future use.  The pass2 scanning routines call fill_inverse_cmap
 * when they need to use an unfilled entry in the cache.
 *
 * Our method of efficiently finding nearest colors is based on the "locally
 * sorted search" idea described by Heckbert and on the incremental distance
 * calculation described by Spencer W. Thomas in chapter III.1 of Graphics
 * Gems II (James Arvo, ed.  Academic Press, 1991).  Thomas points out that
 * the distances from a given colormap entry to each cell of the histogram can
 * be computed quickly using an incremental method: the differences between
 * distances to adjacent cells themselves differ by a constant.  This allows a
 * fairly fast implementation of the "brute force" approach of computing the
 * distance from every colormap entry to every histogram cell.  Unfortunately,
 * it needs a work array to hold the best-distance-so-far for each histogram
 * cell (because the inner loop has to be over cells, not colormap entries).
 * The work array elements have to be JLONGs, so the work array would need
 * 256Kb at our recommended precision.  This is not feasible in DOS machines.
 *
 * To get around these problems, we apply Thomas' method to compute the
 * nearest colors for only the cells within a small subbox of the histogram.
 * The work array need be only as big as the subbox, so the memory usage
 * problem is solved.  Furthermore, we need not fill subboxes that are never
 * referenced in pass2; many images use only part of the color gamut, so a
 * fair amount of work is saved.  An additional advantage of this
 * approach is that we can apply Heckbert's locality criterion to quickly
 * eliminate colormap entries that are far away from the subbox; typically
 * three-fourths of the colormap entries are rejected by Heckbert's criterion,
 * and we need not compute their distances to individual cells in the subbox.
 * The speed of this approach is heavily influenced by the subbox size: too
 * small means too much overhead, too big loses because Heckbert's criterion
 * can't eliminate as many colormap entries.  Empirically the best subbox
 * size seems to be about 1/512th of the histogram (1/8th in each direction).
 *
 * Thomas' article also describes a refined method which is asymptotically
 * faster than the brute-force method, but it is also far more complex and
 * cannot efficiently be applied to small subboxes.  It is therefore not
 * useful for programs intended to be portable to DOS machines.  On machines
 * with plenty of memory, filling the whole histogram in one shot with Thomas'
 * refined method might be faster than the present code --- but then again,
 * it might not be any faster, and it's certainly more complicated.
 */
/* log2(histogram cells in update box) for each axis; this can be adjusted */

pub const BOX_C0_LOG: libc::c_int = HIST_C0_BITS - 3i32;

pub const BOX_C1_LOG: libc::c_int = HIST_C1_BITS - 3i32;

pub const BOX_C2_LOG: libc::c_int = HIST_C2_BITS - 3i32;

pub const BOX_C0_ELEMS: libc::c_int = 1i32 << BOX_C0_LOG;
/* # of hist cells in update box */

pub const BOX_C1_ELEMS: libc::c_int = 1i32 << BOX_C1_LOG;

pub const BOX_C2_ELEMS: libc::c_int = 1i32 << BOX_C2_LOG;

pub const BOX_C0_SHIFT: libc::c_int = C0_SHIFT + BOX_C0_LOG;

pub const BOX_C1_SHIFT: libc::c_int = C1_SHIFT + BOX_C1_LOG;

pub const BOX_C2_SHIFT: libc::c_int = C2_SHIFT + BOX_C2_LOG;
/*
 * The next three routines implement inverse colormap filling.  They could
 * all be folded into one big routine, but splitting them up this way saves
 * some stack space (the mindist[] and bestdist[] arrays need not coexist)
 * and may allow some compilers to produce better code by registerizing more
 * inner-loop variables.
 */

unsafe extern "C" fn find_nearby_colors(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut minc0: libc::c_int,
    mut minc1: libc::c_int,
    mut minc2: libc::c_int,
    mut colorlist: *mut crate::jmorecfg_h::JSAMPLE,
) -> libc::c_int
/* Locate the colormap entries close enough to an update box to be candidates
 * for the nearest entry to some cell(s) in the update box.  The update box
 * is specified by the center coordinates of its first cell.  The number of
 * candidate colormap entries is returned, and their colormap indexes are
 * placed in colorlist[].
 * This routine uses Heckbert's "locally sorted search" criterion to select
 * the colors that need further consideration.
 */ {
    let mut numcolors: libc::c_int = (*cinfo).actual_number_of_colors; /* min distance to colormap entry i */
    let mut maxc0: libc::c_int = 0;
    let mut maxc1: libc::c_int = 0;
    let mut maxc2: libc::c_int = 0;
    let mut centerc0: libc::c_int = 0;
    let mut centerc1: libc::c_int = 0;
    let mut centerc2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ncolors: libc::c_int = 0;
    let mut minmaxdist: crate::jpegint_h::JLONG = 0;
    let mut min_dist: crate::jpegint_h::JLONG = 0;
    let mut max_dist: crate::jpegint_h::JLONG = 0;
    let mut tdist: crate::jpegint_h::JLONG = 0;
    let mut mindist: [crate::jpegint_h::JLONG; 256] = [0; 256];
    /* Compute true coordinates of update box's upper corner and center.
     * Actually we compute the coordinates of the center of the upper-corner
     * histogram cell, which are the upper bounds of the volume we care about.
     * Note that since ">>" rounds down, the "center" values may be closer to
     * min than to max; hence comparisons to them must be "<=", not "<".
     */
    maxc0 = minc0 + ((1i32 << BOX_C0_SHIFT) - (1i32 << C0_SHIFT));
    centerc0 = minc0 + maxc0 >> 1i32;
    maxc1 = minc1 + ((1i32 << BOX_C1_SHIFT) - (1i32 << C1_SHIFT));
    centerc1 = minc1 + maxc1 >> 1i32;
    maxc2 = minc2 + ((1i32 << BOX_C2_SHIFT) - (1i32 << C2_SHIFT));
    centerc2 = minc2 + maxc2 >> 1i32;
    /* For each color in colormap, find:
     *  1. its minimum squared-distance to any point in the update box
     *     (zero if color is within update box);
     *  2. its maximum squared-distance to any point in the update box.
     * Both of these can be found by considering only the corners of the box.
     * We save the minimum distance for each color in mindist[];
     * only the smallest maximum distance is of interest.
     */
    minmaxdist = 0x7fffffffi64;
    i = 0i32;
    while i < numcolors {
        /* We compute the squared-c0-distance term, then add in the other two. */
        x = *(*(*cinfo).colormap.offset(0)).offset(i as isize) as libc::c_int;
        if x < minc0 {
            tdist = ((x - minc0)
                * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - maxc0)
                * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist = tdist * tdist
        } else if x > maxc0 {
            tdist = ((x - maxc0)
                * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - minc0)
                * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist = tdist * tdist
        } else {
            /* within cell range so no contribution to min_dist */
            min_dist = 0i32 as crate::jpegint_h::JLONG;
            if x <= centerc0 {
                tdist = ((x - maxc0)
                    * c_scales
                        [crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as crate::jpegint_h::JLONG;
                max_dist = tdist * tdist
            } else {
                tdist = ((x - minc0)
                    * c_scales
                        [crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as crate::jpegint_h::JLONG;
                max_dist = tdist * tdist
            }
        }
        x = *(*(*cinfo).colormap.offset(1)).offset(i as isize) as libc::c_int;
        if x < minc1 {
            tdist = ((x - minc1)
                * c_scales
                    [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc1)
                * c_scales
                    [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        } else if x > maxc1 {
            tdist = ((x - maxc1)
                * c_scales
                    [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc1)
                * c_scales
                    [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        } else if x <= centerc1 {
            tdist = ((x - maxc1)
                * c_scales
                    [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc1)
                * c_scales
                    [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        }
        x = *(*(*cinfo).colormap.offset(2)).offset(i as isize) as libc::c_int;
        if x < minc2 {
            tdist = ((x - minc2)
                * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc2)
                * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        } else if x > maxc2 {
            tdist = ((x - maxc2)
                * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc2)
                * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        } else if x <= centerc2 {
            tdist = ((x - maxc2)
                * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc2)
                * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as crate::jpegint_h::JLONG;
            max_dist += tdist * tdist
        }
        /* within cell range so no contribution to min_dist */
        /* within cell range so no contribution to min_dist */
        mindist[i as usize] = min_dist; /* save away the results */
        if max_dist < minmaxdist {
            minmaxdist = max_dist
        }
        i += 1
    }
    /* Now we know that no cell in the update box is more than minmaxdist
     * away from some colormap entry.  Therefore, only colors that are
     * within minmaxdist of some part of the box need be considered.
     */
    ncolors = 0i32;
    i = 0i32;
    while i < numcolors {
        if mindist[i as usize] <= minmaxdist {
            let fresh5 = ncolors;
            ncolors = ncolors + 1;
            *colorlist.offset(fresh5 as isize) = i as crate::jmorecfg_h::JSAMPLE
        }
        i += 1
    }
    return ncolors;
}

unsafe extern "C" fn find_best_colors(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut minc0: libc::c_int,
    mut minc1: libc::c_int,
    mut minc2: libc::c_int,
    mut numcolors: libc::c_int,
    mut colorlist: *mut crate::jmorecfg_h::JSAMPLE,
    mut bestcolor: *mut crate::jmorecfg_h::JSAMPLE,
)
/* Find the closest colormap entry for each cell in the update box,
 * given the list of candidate colors prepared by find_nearby_colors.
 * Return the indexes of the closest entries in the bestcolor[] array.
 * This routine uses Thomas' incremental distance calculation method to
 * find the distance from a colormap entry to successive cells in the box.
 */
{
    let mut ic0: libc::c_int = 0; /* pointer into bestdist[] array */
    let mut ic1: libc::c_int = 0; /* pointer into bestcolor[] array */
    let mut ic2: libc::c_int = 0; /* initial distance values */
    let mut i: libc::c_int = 0; /* current distance in inner loop */
    let mut icolor: libc::c_int = 0; /* distance increments */
    let mut bptr: *mut crate::jpegint_h::JLONG = ::std::ptr::null_mut::< crate::jpegint_h::JLONG>(); /* initial values for increments */
    let mut cptr: *mut crate::jmorecfg_h::JSAMPLE = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut dist0: crate::jpegint_h::JLONG = 0;
    let mut dist1: crate::jpegint_h::JLONG = 0;
    let mut dist2: crate::jpegint_h::JLONG = 0;
    let mut xx0: crate::jpegint_h::JLONG = 0;
    let mut xx1: crate::jpegint_h::JLONG = 0;
    let mut xx2: crate::jpegint_h::JLONG = 0;
    let mut inc0: crate::jpegint_h::JLONG = 0;
    let mut inc1: crate::jpegint_h::JLONG = 0;
    let mut inc2: crate::jpegint_h::JLONG = 0;
    /* This array holds the distance to the nearest-so-far color for each cell */
    let mut bestdist: [crate::jpegint_h::JLONG; 128] = [0; 128];
    /* Initialize best-distance for each cell of the update box */
    bptr = bestdist.as_mut_ptr();
    i = BOX_C0_ELEMS * BOX_C1_ELEMS * BOX_C2_ELEMS - 1i32;
    while i >= 0i32 {
        let fresh6 = bptr;
        bptr = bptr.offset(1);
        *fresh6 = 0x7fffffffi64;
        i -= 1
    }
    /* For each color selected by find_nearby_colors,
     * compute its distance to the center of each cell in the box.
     * If that's less than best-so-far, update best distance and color number.
     */
    /* Nominal steps between cell centers ("x" in Thomas article) */
    i = 0i32;
    while i < numcolors {
        icolor = *colorlist.offset(i as isize) as libc::c_int;
        /* Compute (square of) distance from minc0/c1/c2 to this color */
        inc0 = ((minc0 - *(*(*cinfo).colormap.offset(0)).offset(icolor as isize) as libc::c_int)
            * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
            as crate::jpegint_h::JLONG;
        dist0 = inc0 * inc0;
        inc1 = ((minc1 - *(*(*cinfo).colormap.offset(1)).offset(icolor as isize) as libc::c_int)
            * c_scales[crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize])
            as crate::jpegint_h::JLONG;
        dist0 += inc1 * inc1;
        inc2 = ((minc2 - *(*(*cinfo).colormap.offset(2)).offset(icolor as isize) as libc::c_int)
            * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize])
            as crate::jpegint_h::JLONG;
        dist0 += inc2 * inc2;
        /* Form the initial difference increments */
        inc0 = inc0
            * (2i32
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as libc::c_long
            + ((1i32 << C0_SHIFT)
                * c_scales[crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize]
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as libc::c_long;
        inc1 =
            inc1 * (2i32
                * ((1i32 << C1_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize]))
                as libc::c_long
                + ((1i32 << C1_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize] as usize]
                    * ((1i32 << C1_SHIFT)
                        * c_scales[crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize]
                            as usize])) as libc::c_long;
        inc2 = inc2
            * (2i32
                * ((1i32 << C2_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as libc::c_long
            + ((1i32 << C2_SHIFT)
                * c_scales[crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize]
                * ((1i32 << C2_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as libc::c_long;
        /* Now loop over all cells in box, updating distance per Thomas method */
        bptr = bestdist.as_mut_ptr();
        cptr = bestcolor;
        xx0 = inc0;
        ic0 = BOX_C0_ELEMS - 1i32;
        while ic0 >= 0i32 {
            dist1 = dist0;
            xx1 = inc1;
            ic1 = BOX_C1_ELEMS - 1i32;
            while ic1 >= 0i32 {
                dist2 = dist1;
                xx2 = inc2;
                ic2 = BOX_C2_ELEMS - 1i32;
                while ic2 >= 0i32 {
                    if dist2 < *bptr {
                        *bptr = dist2;
                        *cptr = icolor as crate::jmorecfg_h::JSAMPLE
                    }
                    dist2 += xx2;
                    xx2 += (2i32
                        * ((1i32 << C2_SHIFT)
                            * c_scales[crate::jmorecfg_h::rgb_blue
                                [(*cinfo).out_color_space as usize]
                                as usize])
                        * ((1i32 << C2_SHIFT)
                            * c_scales[crate::jmorecfg_h::rgb_blue
                                [(*cinfo).out_color_space as usize]
                                as usize])) as libc::c_long;
                    bptr = bptr.offset(1);
                    cptr = cptr.offset(1);
                    ic2 -= 1
                }
                dist1 += xx1;
                xx1 += (2i32
                    * ((1i32 << C1_SHIFT)
                        * c_scales[crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize]
                            as usize])
                    * ((1i32 << C1_SHIFT)
                        * c_scales[crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize]
                            as usize])) as libc::c_long;
                ic1 -= 1
            }
            dist0 += xx0;
            xx0 += (2i32
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize])
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as libc::c_long;
            ic0 -= 1
        }
        i += 1
    }
}

unsafe extern "C" fn fill_inverse_cmap(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut c0: libc::c_int,
    mut c1: libc::c_int,
    mut c2: libc::c_int,
)
/* Fill the inverse-colormap entries in the update box that contains */
/* histogram cell c0/c1/c2.  (Only that one cell MUST be filled, but */
/* we can fill as many others as we wish.) */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* lower left corner of update box */
    let mut histogram: hist3d = (*cquantize).histogram; /* pointer into bestcolor[] array */
    let mut minc0: libc::c_int = 0; /* pointer into main cache array */
    let mut minc1: libc::c_int = 0;
    let mut minc2: libc::c_int = 0;
    let mut ic0: libc::c_int = 0;
    let mut ic1: libc::c_int = 0;
    let mut ic2: libc::c_int = 0;
    let mut cptr: *mut crate::jmorecfg_h::JSAMPLE = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut cachep: histptr = ::std::ptr::null_mut::< histcell>();
    /* This array lists the candidate colormap indexes. */
    let mut colorlist: [crate::jmorecfg_h::JSAMPLE; 256] = [0; 256]; /* number of candidate colors */
    let mut numcolors: libc::c_int = 0;
    /* This array holds the actually closest colormap index for each cell. */
    let mut bestcolor: [crate::jmorecfg_h::JSAMPLE; 128] = [0; 128];
    /* Convert cell coordinates to update box ID */
    c0 >>= BOX_C0_LOG;
    c1 >>= BOX_C1_LOG;
    c2 >>= BOX_C2_LOG;
    /* Compute true coordinates of update box's origin corner.
     * Actually we compute the coordinates of the center of the corner
     * histogram cell, which are the lower bounds of the volume we care about.
     */
    minc0 = (c0 << BOX_C0_SHIFT) + (1i32 << C0_SHIFT >> 1i32);
    minc1 = (c1 << BOX_C1_SHIFT) + (1i32 << C1_SHIFT >> 1i32);
    minc2 = (c2 << BOX_C2_SHIFT) + (1i32 << C2_SHIFT >> 1i32);
    /* Determine which colormap entries are close enough to be candidates
     * for the nearest entry to some cell in the update box.
     */
    numcolors = find_nearby_colors(cinfo, minc0, minc1, minc2, colorlist.as_mut_ptr());
    /* Determine the actually nearest colors. */
    find_best_colors(
        cinfo,
        minc0,
        minc1,
        minc2,
        numcolors,
        colorlist.as_mut_ptr(),
        bestcolor.as_mut_ptr(),
    );
    /* Save the best color numbers (plus 1) in the main cache array */
    c0 <<= BOX_C0_LOG; /* convert ID back to base cell indexes */
    c1 <<= BOX_C1_LOG;
    c2 <<= BOX_C2_LOG;
    cptr = bestcolor.as_mut_ptr();
    ic0 = 0i32;
    while ic0 < BOX_C0_ELEMS {
        ic1 = 0i32;
        while ic1 < BOX_C1_ELEMS {
            cachep = &mut *(*(*histogram.offset((c0 + ic0) as isize)).offset((c1 + ic1) as isize))
                .as_mut_ptr()
                .offset(c2 as isize) as *mut histcell;
            ic2 = 0i32;
            while ic2 < BOX_C2_ELEMS {
                let fresh7 = cptr;
                cptr = cptr.offset(1);
                let fresh8 = cachep;
                cachep = cachep.offset(1);
                *fresh8 = (*fresh7 as libc::c_int + 1i32) as histcell;
                ic2 += 1
            }
            ic1 += 1
        }
        ic0 += 1
    }
}
/*
 * Map some rows of pixels to the output colormapped representation.
 */

unsafe extern "C" fn pass2_no_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
)
/* This version performs no dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut cachep: histptr = ::std::ptr::null_mut::< histcell>();
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0i32;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        col = width;
        while col > 0i32 as libc::c_uint {
            /* get pixel value and index into the cache */
            let fresh9 = inptr;
            inptr = inptr.offset(1);
            c0 = *fresh9 as libc::c_int >> C0_SHIFT;
            let fresh10 = inptr;
            inptr = inptr.offset(1);
            c1 = *fresh10 as libc::c_int >> C1_SHIFT;
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            c2 = *fresh11 as libc::c_int >> C2_SHIFT;
            cachep = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2 as isize) as *mut histcell;
            /* If we have not seen this color before, find nearest colormap entry */
            /* and update the cache */
            if *cachep as libc::c_int == 0i32 {
                fill_inverse_cmap(cinfo, c0, c1, c2);
            }
            /* Now emit the colormap index for this cell */
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 = (*cachep as libc::c_int - 1i32) as crate::jmorecfg_h::JSAMPLE;
            col =  col - 1
        }
        row += 1
    }
}

unsafe extern "C" fn pass2_fs_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
)
/* This version performs Floyd-Steinberg dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* current error or pixel value */
    let mut histogram: hist3d = (*cquantize).histogram; /* error for pixel below cur */
    let mut cur0: LOCFSERROR = 0; /* error for below/prev col */
    let mut cur1: LOCFSERROR = 0; /* => fserrors[] at column before current */
    let mut cur2: LOCFSERROR = 0; /* => current input pixel */
    let mut belowerr0: LOCFSERROR = 0; /* => current output pixel */
    let mut belowerr1: LOCFSERROR = 0; /* +1 or -1 depending on direction */
    let mut belowerr2: LOCFSERROR = 0; /* 3*dir, for advancing inptr & errorptr */
    let mut bpreverr0: LOCFSERROR = 0;
    let mut bpreverr1: LOCFSERROR = 0;
    let mut bpreverr2: LOCFSERROR = 0;
    let mut errorptr: FSERRPTR = ::std::ptr::null_mut::< FSERROR>();
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut cachep: histptr = ::std::ptr::null_mut::< histcell>();
    let mut dir: libc::c_int = 0;
    let mut dir3: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut error_limit: *mut libc::c_int = (*cquantize).error_limiter;
    let mut colormap0: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut colormap1: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(1);
    let mut colormap2: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(2);
    row = 0i32;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        if (*cquantize).on_odd_row != 0 {
            /* work right to left in this row */
            inptr = inptr.offset(
                ((
                width - 1i32 as libc::c_uint) * 3i32 as libc::c_uint) as isize,
            );
            outptr = outptr.offset((width - 1i32 as libc::c_uint) as isize);
            dir = -1i32;
            dir3 = -3i32; /* so point to rightmost pixel */
            /* flip for next time */
            errorptr = (*cquantize).fserrors.offset(
                ((
                width + 1i32 as libc::c_uint) * 3i32 as libc::c_uint) as isize,
            ); /* => entry after last column */
            (*cquantize).on_odd_row = crate::jmorecfg_h::FALSE
        } else {
            /* work left to right in this row */
            dir = 1i32;
            dir3 = 3i32;
            /* flip for next time */
            errorptr = (*cquantize).fserrors; /* => entry before first real column */
            (*cquantize).on_odd_row = crate::jmorecfg_h::TRUE
        }
        /* Preset error values: no error propagated to first pixel from left */
        cur2 = 0i32;
        cur1 = cur2;
        cur0 = cur1;
        /* and no error propagated to row below yet */
        belowerr2 = 0i32;
        belowerr1 = belowerr2;
        belowerr0 = belowerr1;
        bpreverr2 = 0i32;
        bpreverr1 = bpreverr2;
        bpreverr0 = bpreverr1;
        col = width;
        while col > 0i32 as libc::c_uint {
            /* curN holds the error propagated from the previous pixel on the
             * current line.  Add the error propagated from the previous line
             * to form the complete error correction term for this pixel, and
             * round the error term (which is expressed * 16) to an integer.
             * RIGHT_SHIFT rounds towards minus infinity, so adding 8 is correct
             * for either sign of the error value.
             * Note: errorptr points to *previous* column's array entry.
             */
            cur0 = cur0 + *errorptr.offset((dir3 + 0i32) as isize) as libc::c_int + 8i32 >> 4i32;
            cur1 = cur1 + *errorptr.offset((dir3 + 1i32) as isize) as libc::c_int + 8i32 >> 4i32;
            cur2 = cur2 + *errorptr.offset((dir3 + 2i32) as isize) as libc::c_int + 8i32 >> 4i32;
            /* advance errorptr to current column */
            cur0 = *error_limit.offset(cur0 as isize);
            cur1 = *error_limit.offset(cur1 as isize);
            cur2 = *error_limit.offset(cur2 as isize);
            cur0 += *inptr.offset(0) as libc::c_int;
            cur1 += *inptr.offset(1) as libc::c_int;
            cur2 += *inptr.offset(2) as libc::c_int;
            cur0 = *range_limit.offset(cur0 as isize) as libc::c_int;
            cur1 = *range_limit.offset(cur1 as isize) as libc::c_int;
            cur2 = *range_limit.offset(cur2 as isize) as libc::c_int;
            cachep = &mut *(*(*histogram.offset((cur0 >> C0_SHIFT) as isize))
                .offset((cur1 >> C1_SHIFT) as isize))
            .as_mut_ptr()
            .offset((cur2 >> C2_SHIFT) as isize) as *mut histcell;
            if *cachep as libc::c_int == 0i32 {
                fill_inverse_cmap(cinfo, cur0 >> C0_SHIFT, cur1 >> C1_SHIFT, cur2 >> C2_SHIFT);
            }
            let mut pixcode: libc::c_int = *cachep as libc::c_int - 1i32;
            *outptr = pixcode as crate::jmorecfg_h::JSAMPLE;
            cur0 -= *colormap0.offset(pixcode as isize) as libc::c_int;
            cur1 -= *colormap1.offset(pixcode as isize) as libc::c_int;
            cur2 -= *colormap2.offset(pixcode as isize) as libc::c_int;
            let mut bnexterr: LOCFSERROR = 0;
            bnexterr = cur0;
            *errorptr.offset(0) = (bpreverr0 + cur0 * 3i32) as FSERROR;
            bpreverr0 = belowerr0 + cur0 * 5i32;
            belowerr0 = bnexterr;
            cur0 *= 7i32;
            bnexterr = cur1;
            *errorptr.offset(1) = (bpreverr1 + cur1 * 3i32) as FSERROR;
            bpreverr1 = belowerr1 + cur1 * 5i32;
            belowerr1 = bnexterr;
            cur1 *= 7i32;
            bnexterr = cur2;
            *errorptr.offset(2) = (bpreverr2 + cur2 * 3i32) as FSERROR;
            bpreverr2 = belowerr2 + cur2 * 5i32;
            belowerr2 = bnexterr;
            cur2 *= 7i32;
            inptr = inptr.offset(dir3 as isize);
            outptr = outptr.offset(dir as isize);
            errorptr = errorptr.offset(dir3 as isize);
            col =  col - 1
        }
        /* Limit the error using transfer function set by init_error_limit.
         * See comments with init_error_limit for rationale.
         */
        /* Form pixel value + error, and range-limit to 0..MAXJSAMPLE.
         * The maximum error is +- MAXJSAMPLE (or less with error limiting);
         * this sets the required size of the range_limit array.
         */
        /* Index into the cache with adjusted pixel value */
        /* If we have not seen this color before, find nearest colormap */
        /* entry and update the cache */
        /* Now emit the colormap index for this cell */
        /* Compute representation error for this pixel */
        /* Compute error fractions to be propagated to adjacent pixels.
         * Add these into the running sums, and simultaneously shift the
         * next-line error sums left by 1 column.
         */
        /* Process component 0 */
        /* Process component 1 */
        /* Process component 2 */
        /* At this point curN contains the 7/16 error value to be propagated
         * to the next pixel on the current line, and all the errors for the
         * next line have been shifted over.  We are therefore ready to move on.
         */
        /* Advance pixel pointers to next column */
        /* Post-loop cleanup: we must unload the final error values into the
         * final fserrors[] entry.  Note we need not unload belowerrN because
         * it is for the dummy column before or after the actual array.
         */
        *errorptr.offset(0) = bpreverr0 as FSERROR; /* unload prev errs into array */
        *errorptr.offset(1) = bpreverr1 as FSERROR;
        *errorptr.offset(2) = bpreverr2 as FSERROR;
        row += 1
    }
}
/*
 * Initialize the error-limiting transfer function (lookup table).
 * The raw F-S error computation can potentially compute error values of up to
 * +- MAXJSAMPLE.  But we want the maximum correction applied to a pixel to be
 * much less, otherwise obviously wrong pixels will be created.  (Typical
 * effects include weird fringes at color-area boundaries, isolated bright
 * pixels in a dark area, etc.)  The standard advice for avoiding this problem
 * is to ensure that the "corners" of the color cube are allocated as output
 * colors; then repeated errors in the same direction cannot cause cascading
 * error buildup.  However, that only prevents the error from getting
 * completely out of hand; Aaron Giles reports that error limiting improves
 * the results even with corner colors allocated.
 * A simple clamping of the error values to about +- MAXJSAMPLE/8 works pretty
 * well, but the smoother transfer function used below is even better.  Thanks
 * to Aaron Giles for this idea.
 */

unsafe extern "C" fn init_error_limit(mut cinfo: crate::jpeglib_h::j_decompress_ptr)
/* Allocate and fill in the error_limiter table */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* so can index -MAXJSAMPLE .. +MAXJSAMPLE */
    let mut table: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut in_0: libc::c_int = 0;
    let mut out: libc::c_int = 0;
    table = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (crate::jmorecfg_h::MAXJSAMPLE * 2i32 + 1i32) as libc::c_ulong *
    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    table = table.offset(crate::jmorecfg_h::MAXJSAMPLE as isize);
    (*cquantize).error_limiter = table;
    /* Map errors 1:1 up to +- MAXJSAMPLE/16 */
    out = 0i32;
    in_0 = 0i32;
    while in_0 < STEPSIZE {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out += 1
    }
    /* Map errors 1:2 up to +- 3*MAXJSAMPLE/16 */
    while in_0 < STEPSIZE * 3i32 {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out += (if in_0 & 1i32 != 0 { 0i32 } else { 1i32 })
    }
    /* Clamp the rest to final out value (which is (MAXJSAMPLE+1)/8) */
    while in_0 <= crate::jmorecfg_h::MAXJSAMPLE {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1
    }
}

pub const STEPSIZE: libc::c_int = (crate::jmorecfg_h::MAXJSAMPLE + 1i32) / 16i32;
/*
 * Finish up at the end of each pass.
 */

unsafe extern "C" fn finish_pass1(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    /* Select the representative colors and fill in cinfo->colormap */
    (*cinfo).colormap = (*cquantize).sv_colormap;
    select_colors(cinfo, (*cquantize).desired);
    /* Force next pass to zero the color index table */
    (*cquantize).needs_zeroed = crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn finish_pass2(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    /* no work */
}
/*
 * Initialize for each processing pass.
 */

unsafe extern "C" fn start_pass_2_quant(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut is_pre_scan: crate::jmorecfg_h::boolean,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut i: libc::c_int = 0;
    /* Only F-S dithering or no dithering is supported. */
    /* If user asks for ordered dither, give him F-S. */
    if (*cinfo).dither_mode as libc::c_uint
        != crate::jpeglib_h::JDITHER_NONE as libc::c_int as libc::c_uint
    {
        (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_FS
    }
    if is_pre_scan != 0 {
        /* Set up method pointers */
        (*cquantize).pub_0.color_quantize = Some(
            prescan_quantize
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPARRAY,
                    _: crate::jpeglib_h::JSAMPARRAY,
                    _: libc::c_int,
                ) -> (),
        );
        (*cquantize).pub_0.finish_pass =
            Some(finish_pass1 as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ());
        (*cquantize).needs_zeroed = crate::jmorecfg_h::TRUE
    /* Always zero histogram */
    } else {
        /* Set up method pointers */
        if (*cinfo).dither_mode as libc::c_uint
            == crate::jpeglib_h::JDITHER_FS as libc::c_int as libc::c_uint
        {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_fs_dither
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        } else {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_no_dither
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        (*cquantize).pub_0.finish_pass =
            Some(finish_pass2 as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ());
        /* Make sure color count is acceptable */
        i = (*cinfo).actual_number_of_colors;
        if i < 1i32 {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_FEW_COLORS as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = 1i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if i > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_MANY_COLORS as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if (*cinfo).dither_mode as libc::c_uint
            == crate::jpeglib_h::JDITHER_FS as libc::c_int as libc::c_uint
        {
            let mut arraysize: crate::stddef_h::size_t =
                ((*cinfo).output_width + 2i32 as libc::c_uint) as libc::c_ulong *
    (3i32 as libc::c_ulong *
         ::std::mem::size_of::<FSERROR>() as libc::c_ulong);
            /* Allocate Floyd-Steinberg workspace if we didn't already. */
            if (*cquantize).fserrors.is_null() {
                (*cquantize).fserrors = Some(
                    (*(*cinfo).mem)
                        .alloc_large
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    crate::jpeglib_h::JPOOL_IMAGE,
                    arraysize,
                ) as FSERRPTR
            }
            /* Initialize the propagated errors to zero. */
            crate::jpegint_h::jzero_far((*cquantize).fserrors as *mut libc::c_void, arraysize);
            /* Make the error-limit table if we didn't already. */
            if (*cquantize).error_limiter.is_null() {
                init_error_limit(cinfo);
            }
            (*cquantize).on_odd_row = crate::jmorecfg_h::FALSE
        }
    }
    /* Zero the histogram or inverse color map, if necessary */
    if (*cquantize).needs_zeroed != 0 {
        i = 0i32;
        while i < HIST_C0_ELEMS {
            crate::jpegint_h::jzero_far(
                *histogram.offset(i as isize) as *mut libc::c_void,
                (HIST_C1_ELEMS * HIST_C2_ELEMS) as libc::c_ulong *
    ::std::mem::size_of::<histcell>() as libc::c_ulong,
            );
            i += 1
        }
        (*cquantize).needs_zeroed = crate::jmorecfg_h::FALSE
    };
}
/*
 * Switch to a new external colormap between output passes.
 */

unsafe extern "C" fn new_color_map_2_quant(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    /* Reset the inverse color map */
    (*cquantize).needs_zeroed = crate::jmorecfg_h::TRUE;
}
/*
 * Module initialization routine for 2-pass color quantization.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_2pass_quantizer(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = ::std::ptr::null_mut::< my_cquantizer>(); /* flag optional arrays not allocated */
    let mut i: libc::c_int = 0;
    cquantize = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_cquantizer>() as libc::c_ulong,
    ) as my_cquantize_ptr;
    (*cinfo).cquantize = cquantize as *mut crate::jpeglib_h::jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass = Some(
        start_pass_2_quant
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::jmorecfg_h::boolean,
            ) -> (),
    );
    (*cquantize).pub_0.new_color_map = Some(
        new_color_map_2_quant as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*cquantize).fserrors = crate::stddef_h::NULL as FSERRPTR;
    (*cquantize).error_limiter = crate::stddef_h::NULL as *mut libc::c_int;
    /* Make sure jdmaster didn't give me a case I can't handle */
    if (*cinfo).out_color_components != 3i32 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_NOTIMPL as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Allocate the histogram/inverse colormap storage */
    (*cquantize).histogram = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        HIST_C0_ELEMS as libc::c_ulong *
    ::std::mem::size_of::<hist2d>() as libc::c_ulong,
    ) as hist3d; /* histogram is garbage now */
    i = 0i32;
    while i < HIST_C0_ELEMS {
        let ref mut fresh13 = *(*cquantize).histogram.offset(i as isize);
        *fresh13 = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            (HIST_C1_ELEMS * HIST_C2_ELEMS) as libc::c_ulong *
    ::std::mem::size_of::<histcell>() as libc::c_ulong,
        ) as hist2d;
        i += 1
    }
    (*cquantize).needs_zeroed = crate::jmorecfg_h::TRUE;
    /* Allocate storage for the completed colormap, if required.
     * We do this now since it may affect the memory manager's space
     * calculations.
     */
    if (*cinfo).enable_2pass_quant != 0 {
        /* Make sure color count is acceptable */
        let mut desired: libc::c_int = (*cinfo).desired_number_of_colors;
        /* Lower bound on # of colors ... somewhat arbitrary as long as > 0 */
        if desired < 8i32 {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_FEW_COLORS as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = 8i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        /* Make sure colormap indexes can be represented by JSAMPLEs */
        if desired > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_MANY_COLORS as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        (*cquantize).sv_colormap = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            desired as crate::jmorecfg_h::JDIMENSION,
            3i32 as crate::jmorecfg_h::JDIMENSION,
        );
        (*cquantize).desired = desired
    } else {
        (*cquantize).sv_colormap = crate::stddef_h::NULL as crate::jpeglib_h::JSAMPARRAY
    }
    /* Only F-S dithering or no dithering is supported. */
    /* If user asks for ordered dither, give him F-S. */
    if (*cinfo).dither_mode as libc::c_uint
        != crate::jpeglib_h::JDITHER_NONE as libc::c_int as libc::c_uint
    {
        (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_FS
    }
    /* Allocate Floyd-Steinberg workspace if necessary.
     * This isn't really needed until pass 2, but again it may affect the memory
     * manager's space calculations.  Although we will cope with a later change
     * in dither_mode, we do not promise to honor max_memory_to_use if
     * dither_mode changes.
     */
    if (*cinfo).dither_mode as libc::c_uint
        == crate::jpeglib_h::JDITHER_FS as libc::c_int as libc::c_uint
    {
        (*cquantize).fserrors = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            ((*cinfo).output_width + 2i32 as libc::c_uint) as libc::c_ulong *
    (3i32 as libc::c_ulong *
         ::std::mem::size_of::<FSERROR>() as libc::c_ulong),
        ) as FSERRPTR;
        /* Might as well create the error-limiting table too. */
        init_error_limit(cinfo);
    };
}
/* QUANT_2PASS_SUPPORTED */
