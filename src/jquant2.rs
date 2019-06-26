use libc;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;

pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jerror::C2RustUnnamed_4;
pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
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
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
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
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_3;
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
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub type my_cquantize_ptr = *mut my_cquantizer;
/* Private subobject */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_cquantizer {
    pub pub_0: jpeg_color_quantizer,
    pub sv_colormap: JSAMPARRAY,
    pub desired: c_int,
    pub histogram: hist3d,
    pub needs_zeroed: boolean,
    pub fserrors: FSERRPTR,
    pub on_odd_row: boolean,
    pub error_limiter: *mut c_int,
}
/* pointer to error array */
pub type FSERRPTR = *mut FSERROR;
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
/* 16 bits should be enough */
pub type FSERROR = INT16;
/* type for top-level pointer */
pub type hist3d = *mut hist2d;
/* type for the 2nd-level pointers */
pub type hist2d = *mut hist1d;
/* typedefs for the array */
pub type hist1d = [histcell; 32];
/* histogram cell; prefer an unsigned type */
pub type histcell = UINT16;
pub type boxptr = *mut box_0;
/*
 * Next we have the really interesting routines: selection of a colormap
 * given the completed histogram.
 * These routines work with a list of "boxes", each representing a rectangular
 * subset of the input color space (to histogram precision).
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct box_0 {
    pub c0min: c_int,
    pub c0max: c_int,
    pub c1min: c_int,
    pub c1max: c_int,
    pub c2min: c_int,
    pub c2max: c_int,
    pub volume: JLONG,
    pub colorcount: c_long,
}
/* for pointers to histogram cells */
pub type histptr = *mut histcell;
/* use 'int' for calculation temps */
pub type LOCFSERROR = c_int;
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
/* scale R distances by this much */
pub const R_SCALE: c_int = 2i32;
/* scale G distances by this much */
pub const G_SCALE: c_int = 3i32;
/* and B by this much */
pub const B_SCALE: c_int = 1i32;
static mut c_scales: [c_int; 3] = [R_SCALE, G_SCALE, B_SCALE];
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
/* maximum size of colormap */
pub const MAXNUMCOLORS: c_int = MAXJSAMPLE + 1i32;
/* These will do the right thing for either R,G,B or B,G,R color order,
 * but you may not like the results for other color orders.
 */
/* bits of precision in R/B histogram */
pub const HIST_C0_BITS: c_int = 5i32;
/* bits of precision in G histogram */
pub const HIST_C1_BITS: c_int = 6i32;
/* bits of precision in B/R histogram */
pub const HIST_C2_BITS: c_int = 5i32;
/* Number of elements along histogram axes. */
pub const HIST_C0_ELEMS: c_int = 1i32 << HIST_C0_BITS;
pub const HIST_C1_ELEMS: c_int = 1i32 << HIST_C1_BITS;
pub const HIST_C2_ELEMS: c_int = 1i32 << HIST_C2_BITS;
/* These are the amounts to shift an input value to get a histogram index. */
pub const C0_SHIFT: c_int = BITS_IN_JSAMPLE - HIST_C0_BITS;
pub const C1_SHIFT: c_int = BITS_IN_JSAMPLE - HIST_C1_BITS;
pub const C2_SHIFT: c_int = BITS_IN_JSAMPLE - HIST_C2_BITS;
/*
 * Prescan some rows of pixels.
 * In this module the prescan simply updates the histogram, which has been
 * initialized to zeroes by start_pass.
 * An output_buf parameter is required by the method signature, but no data
 * is actually output (in fact the buffer controller is probably passing a
 * NULL pointer).
 */
unsafe extern "C" fn prescan_quantize(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut histp: histptr = 0 as *mut histcell;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut row: c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0i32;
    while row < num_rows {
        ptr = *input_buf.offset(row as isize);
        col = width;
        while col > 0i32 as c_uint {
            histp = &mut *(*(*histogram
                .offset((*ptr.offset(0isize) as c_int >> C0_SHIFT) as isize))
            .offset((*ptr.offset(1isize) as c_int >> C1_SHIFT) as isize))
            .as_mut_ptr()
            .offset((*ptr.offset(2isize) as c_int >> C2_SHIFT) as isize)
                as *mut histcell;
            *histp = (*histp).wrapping_add(1);
            if *histp as c_int <= 0i32 {
                *histp = (*histp).wrapping_sub(1)
            }
            ptr = ptr.offset(3isize);
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}
unsafe extern "C" fn find_biggest_color_pop(mut boxlist: boxptr, mut numboxes: c_int) -> boxptr {
    let mut boxp: boxptr = 0 as *mut box_0;
    let mut i: c_int = 0;
    let mut maxc: c_long = 0i32 as c_long;
    let mut which: boxptr = NULL as boxptr;
    i = 0i32;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).colorcount > maxc && (*boxp).volume > 0i32 as c_long {
            which = boxp;
            maxc = (*boxp).colorcount
        }
        i += 1;
        boxp = boxp.offset(1isize)
    }
    return which;
}
unsafe extern "C" fn find_biggest_volume(mut boxlist: boxptr, mut numboxes: c_int) -> boxptr {
    let mut boxp: boxptr = 0 as *mut box_0;
    let mut i: c_int = 0;
    let mut maxv: JLONG = 0i32 as JLONG;
    let mut which: boxptr = NULL as boxptr;
    i = 0i32;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).volume > maxv {
            which = boxp;
            maxv = (*boxp).volume
        }
        i += 1;
        boxp = boxp.offset(1isize)
    }
    return which;
}
unsafe extern "C" fn update_box(mut cinfo: j_decompress_ptr, mut boxp: boxptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = 0 as *mut histcell;
    let mut c0: c_int = 0;
    let mut c1: c_int = 0;
    let mut c2: c_int = 0;
    let mut c0min: c_int = 0;
    let mut c0max: c_int = 0;
    let mut c1min: c_int = 0;
    let mut c1max: c_int = 0;
    let mut c2min: c_int = 0;
    let mut c2max: c_int = 0;
    let mut dist0: JLONG = 0;
    let mut dist1: JLONG = 0;
    let mut dist2: JLONG = 0;
    let mut ccount: c_long = 0;
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
                    if *fresh0 as c_int != 0i32 {
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
                    if *fresh1 as c_int != 0i32 {
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
                    if *fresh2 as c_int != 0i32 {
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
                    if *fresh3 as c_int != 0i32 {
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
                    if *histp as c_int != 0i32 {
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
                    if *histp as c_int != 0i32 {
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
    dist0 = ((c0max - c0min << C0_SHIFT)
        * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]) as JLONG;
    dist1 = ((c1max - c1min << C1_SHIFT)
        * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]) as JLONG;
    dist2 = ((c2max - c2min << C2_SHIFT)
        * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]) as JLONG;
    (*boxp).volume = dist0 * dist0 + dist1 * dist1 + dist2 * dist2;
    ccount = 0i32 as c_long;
    c0 = c0min;
    while c0 <= c0max {
        c1 = c1min;
        while c1 <= c1max {
            histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2min as isize) as *mut histcell;
            c2 = c2min;
            while c2 <= c2max {
                if *histp as c_int != 0i32 {
                    ccount += 1
                }
                c2 += 1;
                histp = histp.offset(1isize)
            }
            c1 += 1
        }
        c0 += 1
    }
    (*boxp).colorcount = ccount;
}
unsafe extern "C" fn median_cut(
    mut cinfo: j_decompress_ptr,
    mut boxlist: boxptr,
    mut numboxes: c_int,
    mut desired_colors: c_int,
) -> c_int {
    let mut n: c_int = 0;
    let mut lb: c_int = 0;
    let mut c0: c_int = 0;
    let mut c1: c_int = 0;
    let mut c2: c_int = 0;
    let mut cmax: c_int = 0;
    let mut b1: boxptr = 0 as *mut box_0;
    let mut b2: boxptr = 0 as *mut box_0;
    while numboxes < desired_colors {
        if numboxes * 2i32 <= desired_colors {
            b1 = find_biggest_color_pop(boxlist, numboxes)
        } else {
            b1 = find_biggest_volume(boxlist, numboxes)
        }
        /* no splittable boxes left! */
        if b1.is_null() {
            break;
        }
        b2 = &mut *boxlist.offset(numboxes as isize) as *mut box_0;
        (*b2).c0max = (*b1).c0max;
        (*b2).c1max = (*b1).c1max;
        (*b2).c2max = (*b1).c2max;
        (*b2).c0min = (*b1).c0min;
        (*b2).c1min = (*b1).c1min;
        (*b2).c2min = (*b1).c2min;
        c0 = ((*b1).c0max - (*b1).c0min << C0_SHIFT)
            * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize];
        c1 = ((*b1).c1max - (*b1).c1min << C1_SHIFT)
            * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize];
        c2 = ((*b1).c2max - (*b1).c2min << C2_SHIFT)
            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize];
        if rgb_red[(*cinfo).out_color_space as usize] == 0i32 {
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
        update_box(cinfo, b1);
        update_box(cinfo, b2);
        numboxes += 1
    }
    return numboxes;
}
unsafe extern "C" fn compute_color(
    mut cinfo: j_decompress_ptr,
    mut boxp: boxptr,
    mut icolor: c_int,
) {
    /* Current algorithm: mean weighted by pixels (not colors) */
    /* Note it is important to get the rounding correct! */
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = 0 as *mut histcell;
    let mut c0: c_int = 0;
    let mut c1: c_int = 0;
    let mut c2: c_int = 0;
    let mut c0min: c_int = 0;
    let mut c0max: c_int = 0;
    let mut c1min: c_int = 0;
    let mut c1max: c_int = 0;
    let mut c2min: c_int = 0;
    let mut c2max: c_int = 0;
    let mut count: c_long = 0;
    let mut total: c_long = 0i32 as c_long;
    let mut c0total: c_long = 0i32 as c_long;
    let mut c1total: c_long = 0i32 as c_long;
    let mut c2total: c_long = 0i32 as c_long;
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
                count = *fresh4 as c_long;
                if count != 0i32 as c_long {
                    total += count;
                    c0total += ((c0 << C0_SHIFT) + (1i32 << C0_SHIFT >> 1i32)) as c_long * count;
                    c1total += ((c1 << C1_SHIFT) + (1i32 << C1_SHIFT >> 1i32)) as c_long * count;
                    c2total += ((c2 << C2_SHIFT) + (1i32 << C2_SHIFT >> 1i32)) as c_long * count
                }
                c2 += 1
            }
            c1 += 1
        }
        c0 += 1
    }
    *(*(*cinfo).colormap.offset(0isize)).offset(icolor as isize) =
        ((c0total + (total >> 1i32)) / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(1isize)).offset(icolor as isize) =
        ((c1total + (total >> 1i32)) / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(2isize)).offset(icolor as isize) =
        ((c2total + (total >> 1i32)) / total) as JSAMPLE;
}
unsafe extern "C" fn select_colors(mut cinfo: j_decompress_ptr, mut desired_colors: c_int) {
    let mut boxlist: boxptr = 0 as *mut box_0;
    let mut numboxes: c_int = 0;
    let mut i: c_int = 0;
    boxlist = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (desired_colors as c_ulong).wrapping_mul(::std::mem::size_of::<box_0>() as c_ulong),
    ) as boxptr;
    numboxes = 1i32;
    (*boxlist.offset(0isize)).c0min = 0i32;
    (*boxlist.offset(0isize)).c0max = MAXJSAMPLE >> C0_SHIFT;
    (*boxlist.offset(0isize)).c1min = 0i32;
    (*boxlist.offset(0isize)).c1max = MAXJSAMPLE >> C1_SHIFT;
    (*boxlist.offset(0isize)).c2min = 0i32;
    (*boxlist.offset(0isize)).c2max = MAXJSAMPLE >> C2_SHIFT;
    update_box(cinfo, &mut *boxlist.offset(0isize));
    numboxes = median_cut(cinfo, boxlist, numboxes, desired_colors);
    i = 0i32;
    while i < numboxes {
        compute_color(cinfo, &mut *boxlist.offset(i as isize), i);
        i += 1
    }
    (*cinfo).actual_number_of_colors = numboxes;
    (*(*cinfo).err).msg_code = JTRC_QUANT_SELECTED as c_int;
    (*(*cinfo).err).msg_parm.i[0usize] = numboxes;
    (*(*cinfo).err)
        .emit_message
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
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
pub const BOX_C0_LOG: c_int = HIST_C0_BITS - 3i32;
pub const BOX_C1_LOG: c_int = HIST_C1_BITS - 3i32;
pub const BOX_C2_LOG: c_int = HIST_C2_BITS - 3i32;
/* # of hist cells in update box */
pub const BOX_C0_ELEMS: c_int = 1i32 << BOX_C0_LOG;
pub const BOX_C1_ELEMS: c_int = 1i32 << BOX_C1_LOG;
pub const BOX_C2_ELEMS: c_int = 1i32 << BOX_C2_LOG;
pub const BOX_C0_SHIFT: c_int = C0_SHIFT + BOX_C0_LOG;
pub const BOX_C1_SHIFT: c_int = C1_SHIFT + BOX_C1_LOG;
pub const BOX_C2_SHIFT: c_int = C2_SHIFT + BOX_C2_LOG;
/*
 * The next three routines implement inverse colormap filling.  They could
 * all be folded into one big routine, but splitting them up this way saves
 * some stack space (the mindist[] and bestdist[] arrays need not coexist)
 * and may allow some compilers to produce better code by registerizing more
 * inner-loop variables.
 */
unsafe extern "C" fn find_nearby_colors(
    mut cinfo: j_decompress_ptr,
    mut minc0: c_int,
    mut minc1: c_int,
    mut minc2: c_int,
    mut colorlist: *mut JSAMPLE,
) -> c_int {
    let mut numcolors: c_int = (*cinfo).actual_number_of_colors;
    let mut maxc0: c_int = 0;
    let mut maxc1: c_int = 0;
    let mut maxc2: c_int = 0;
    let mut centerc0: c_int = 0;
    let mut centerc1: c_int = 0;
    let mut centerc2: c_int = 0;
    let mut i: c_int = 0;
    let mut x: c_int = 0;
    let mut ncolors: c_int = 0;
    let mut minmaxdist: JLONG = 0;
    let mut min_dist: JLONG = 0;
    let mut max_dist: JLONG = 0;
    let mut tdist: JLONG = 0;
    /* min distance to colormap entry i */
    let mut mindist: [JLONG; 256] = [0; 256];
    maxc0 = minc0 + ((1i32 << BOX_C0_SHIFT) - (1i32 << C0_SHIFT));
    centerc0 = minc0 + maxc0 >> 1i32;
    maxc1 = minc1 + ((1i32 << BOX_C1_SHIFT) - (1i32 << C1_SHIFT));
    centerc1 = minc1 + maxc1 >> 1i32;
    maxc2 = minc2 + ((1i32 << BOX_C2_SHIFT) - (1i32 << C2_SHIFT));
    centerc2 = minc2 + maxc2 >> 1i32;
    minmaxdist = 0x7fffffffi64;
    i = 0i32;
    while i < numcolors {
        x = *(*(*cinfo).colormap.offset(0isize)).offset(i as isize) as c_int;
        if x < minc0 {
            tdist = ((x - minc0) * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - maxc0) * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist = tdist * tdist
        } else if x > maxc0 {
            tdist = ((x - maxc0) * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - minc0) * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist = tdist * tdist
        } else {
            min_dist = 0i32 as JLONG;
            if x <= centerc0 {
                tdist = ((x - maxc0)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as JLONG;
                max_dist = tdist * tdist
            } else {
                tdist = ((x - minc0)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as JLONG;
                max_dist = tdist * tdist
            }
        }
        x = *(*(*cinfo).colormap.offset(1isize)).offset(i as isize) as c_int;
        if x < minc1 {
            tdist = ((x - minc1) * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc1) * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x > maxc1 {
            tdist = ((x - maxc1) * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc1) * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x <= centerc1 {
            tdist = ((x - maxc1) * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc1) * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        }
        x = *(*(*cinfo).colormap.offset(2isize)).offset(i as isize) as c_int;
        if x < minc2 {
            tdist = ((x - minc2) * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc2) * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x > maxc2 {
            tdist = ((x - maxc2) * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc2) * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x <= centerc2 {
            tdist = ((x - maxc2) * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc2) * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        }
        mindist[i as usize] = min_dist;
        if max_dist < minmaxdist {
            minmaxdist = max_dist
        }
        i += 1
    }
    ncolors = 0i32;
    i = 0i32;
    while i < numcolors {
        if mindist[i as usize] <= minmaxdist {
            let fresh5 = ncolors;
            ncolors = ncolors + 1;
            *colorlist.offset(fresh5 as isize) = i as JSAMPLE
        }
        i += 1
    }
    return ncolors;
}
unsafe extern "C" fn find_best_colors(
    mut cinfo: j_decompress_ptr,
    mut minc0: c_int,
    mut minc1: c_int,
    mut minc2: c_int,
    mut numcolors: c_int,
    mut colorlist: *mut JSAMPLE,
    mut bestcolor: *mut JSAMPLE,
) {
    let mut ic0: c_int = 0;
    let mut ic1: c_int = 0;
    let mut ic2: c_int = 0;
    let mut i: c_int = 0;
    let mut icolor: c_int = 0;
    /* pointer into bestdist[] array */
    let mut bptr: *mut JLONG = 0 as *mut JLONG;
    /* pointer into bestcolor[] array */
    let mut cptr: *mut JSAMPLE = 0 as *mut JSAMPLE;
    /* initial distance values */
    let mut dist0: JLONG = 0;
    let mut dist1: JLONG = 0;
    /* current distance in inner loop */
    let mut dist2: JLONG = 0;
    /* distance increments */
    let mut xx0: JLONG = 0;
    let mut xx1: JLONG = 0;
    let mut xx2: JLONG = 0;
    /* initial values for increments */
    let mut inc0: JLONG = 0;
    let mut inc1: JLONG = 0;
    let mut inc2: JLONG = 0;
    /* This array holds the distance to the nearest-so-far color for each cell */
    let mut bestdist: [JLONG; 128] = [0; 128];
    bptr = bestdist.as_mut_ptr();
    i = BOX_C0_ELEMS * BOX_C1_ELEMS * BOX_C2_ELEMS - 1i32;
    while i >= 0i32 {
        let fresh6 = bptr;
        bptr = bptr.offset(1);
        *fresh6 = 0x7fffffffi64;
        i -= 1
    }
    i = 0i32;
    while i < numcolors {
        icolor = *colorlist.offset(i as isize) as c_int;
        inc0 = ((minc0 - *(*(*cinfo).colormap.offset(0isize)).offset(icolor as isize) as c_int)
            * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
            as JLONG;
        dist0 = inc0 * inc0;
        inc1 = ((minc1 - *(*(*cinfo).colormap.offset(1isize)).offset(icolor as isize) as c_int)
            * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
            as JLONG;
        dist0 += inc1 * inc1;
        inc2 = ((minc2 - *(*(*cinfo).colormap.offset(2isize)).offset(icolor as isize) as c_int)
            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
            as JLONG;
        dist0 += inc2 * inc2;
        inc0 = inc0
            * (2i32
                * ((1i32 << C0_SHIFT)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as c_long
            + ((1i32 << C0_SHIFT)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]
                * ((1i32 << C0_SHIFT)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as c_long;
        inc1 = inc1
            * (2i32
                * ((1i32 << C1_SHIFT)
                    * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]))
                as c_long
            + ((1i32 << C1_SHIFT)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]
                * ((1i32 << C1_SHIFT)
                    * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]))
                as c_long;
        inc2 = inc2
            * (2i32
                * ((1i32 << C2_SHIFT)
                    * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as c_long
            + ((1i32 << C2_SHIFT)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]
                * ((1i32 << C2_SHIFT)
                    * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as c_long;
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
                        *cptr = icolor as JSAMPLE
                    }
                    dist2 += xx2;
                    xx2 += (2i32
                        * ((1i32 << C2_SHIFT)
                            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                        * ((1i32 << C2_SHIFT)
                            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                        as c_long;
                    bptr = bptr.offset(1isize);
                    cptr = cptr.offset(1isize);
                    ic2 -= 1
                }
                dist1 += xx1;
                xx1 += (2i32
                    * ((1i32 << C1_SHIFT)
                        * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                    * ((1i32 << C1_SHIFT)
                        * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]))
                    as c_long;
                ic1 -= 1
            }
            dist0 += xx0;
            xx0 += (2i32
                * ((1i32 << C0_SHIFT)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                * ((1i32 << C0_SHIFT)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as c_long;
            ic0 -= 1
        }
        i += 1
    }
}
unsafe extern "C" fn fill_inverse_cmap(
    mut cinfo: j_decompress_ptr,
    mut c0: c_int,
    mut c1: c_int,
    mut c2: c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    /* lower left corner of update box */
    let mut minc0: c_int = 0;
    let mut minc1: c_int = 0;
    let mut minc2: c_int = 0;
    let mut ic0: c_int = 0;
    let mut ic1: c_int = 0;
    let mut ic2: c_int = 0;
    /* pointer into bestcolor[] array */
    let mut cptr: *mut JSAMPLE = 0 as *mut JSAMPLE;
    /* pointer into main cache array */
    let mut cachep: histptr = 0 as *mut histcell;
    /* This array lists the candidate colormap indexes. */
    let mut colorlist: [JSAMPLE; 256] = [0; 256];
    /* number of candidate colors */
    let mut numcolors: c_int = 0;
    /* This array holds the actually closest colormap index for each cell. */
    let mut bestcolor: [JSAMPLE; 128] = [0; 128];
    c0 >>= BOX_C0_LOG;
    c1 >>= BOX_C1_LOG;
    c2 >>= BOX_C2_LOG;
    minc0 = (c0 << BOX_C0_SHIFT) + (1i32 << C0_SHIFT >> 1i32);
    minc1 = (c1 << BOX_C1_SHIFT) + (1i32 << C1_SHIFT >> 1i32);
    minc2 = (c2 << BOX_C2_SHIFT) + (1i32 << C2_SHIFT >> 1i32);
    numcolors = find_nearby_colors(cinfo, minc0, minc1, minc2, colorlist.as_mut_ptr());
    find_best_colors(
        cinfo,
        minc0,
        minc1,
        minc2,
        numcolors,
        colorlist.as_mut_ptr(),
        bestcolor.as_mut_ptr(),
    );
    c0 <<= BOX_C0_LOG;
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
                let fresh8 = cachep;
                cachep = cachep.offset(1);
                let fresh7 = cptr;
                cptr = cptr.offset(1);
                *fresh8 = (*fresh7 as c_int + 1i32) as histcell;
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
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut cachep: histptr = 0 as *mut histcell;
    let mut c0: c_int = 0;
    let mut c1: c_int = 0;
    let mut c2: c_int = 0;
    let mut row: c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0i32;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        col = width;
        while col > 0i32 as c_uint {
            let fresh9 = inptr;
            inptr = inptr.offset(1);
            c0 = *fresh9 as c_int >> C0_SHIFT;
            let fresh10 = inptr;
            inptr = inptr.offset(1);
            c1 = *fresh10 as c_int >> C1_SHIFT;
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            c2 = *fresh11 as c_int >> C2_SHIFT;
            cachep = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2 as isize) as *mut histcell;
            if *cachep as c_int == 0i32 {
                fill_inverse_cmap(cinfo, c0, c1, c2);
            }
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 = (*cachep as c_int - 1i32) as JSAMPLE;
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}
unsafe extern "C" fn pass2_fs_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    /* current error or pixel value */
    let mut cur0: LOCFSERROR = 0;
    let mut cur1: LOCFSERROR = 0;
    let mut cur2: LOCFSERROR = 0;
    /* error for pixel below cur */
    let mut belowerr0: LOCFSERROR = 0;
    let mut belowerr1: LOCFSERROR = 0;
    let mut belowerr2: LOCFSERROR = 0;
    /* error for below/prev col */
    let mut bpreverr0: LOCFSERROR = 0;
    let mut bpreverr1: LOCFSERROR = 0;
    let mut bpreverr2: LOCFSERROR = 0;
    /* => fserrors[] at column before current */
    let mut errorptr: FSERRPTR = 0 as *mut FSERROR;
    /* => current input pixel */
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    /* => current output pixel */
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut cachep: histptr = 0 as *mut histcell;
    /* +1 or -1 depending on direction */
    let mut dir: c_int = 0;
    /* 3*dir, for advancing inptr & errorptr */
    let mut dir3: c_int = 0;
    let mut row: c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut error_limit: *mut c_int = (*cquantize).error_limiter;
    let mut colormap0: JSAMPROW = *(*cinfo).colormap.offset(0isize);
    let mut colormap1: JSAMPROW = *(*cinfo).colormap.offset(1isize);
    let mut colormap2: JSAMPROW = *(*cinfo).colormap.offset(2isize);
    row = 0i32;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        if 0 != (*cquantize).on_odd_row {
            inptr = inptr.offset(
                width
                    .wrapping_sub(1i32 as c_uint)
                    .wrapping_mul(3i32 as c_uint) as isize,
            );
            outptr = outptr.offset(width.wrapping_sub(1i32 as c_uint) as isize);
            dir = -1i32;
            dir3 = -3i32;
            errorptr = (*cquantize).fserrors.offset(
                width
                    .wrapping_add(1i32 as c_uint)
                    .wrapping_mul(3i32 as c_uint) as isize,
            );
            (*cquantize).on_odd_row = FALSE
        } else {
            dir = 1i32;
            dir3 = 3i32;
            errorptr = (*cquantize).fserrors;
            (*cquantize).on_odd_row = TRUE
        }
        cur2 = 0i32;
        cur1 = cur2;
        cur0 = cur1;
        belowerr2 = 0i32;
        belowerr1 = belowerr2;
        belowerr0 = belowerr1;
        bpreverr2 = 0i32;
        bpreverr1 = bpreverr2;
        bpreverr0 = bpreverr1;
        col = width;
        while col > 0i32 as c_uint {
            cur0 = cur0 + *errorptr.offset((dir3 + 0i32) as isize) as c_int + 8i32 >> 4i32;
            cur1 = cur1 + *errorptr.offset((dir3 + 1i32) as isize) as c_int + 8i32 >> 4i32;
            cur2 = cur2 + *errorptr.offset((dir3 + 2i32) as isize) as c_int + 8i32 >> 4i32;
            cur0 = *error_limit.offset(cur0 as isize);
            cur1 = *error_limit.offset(cur1 as isize);
            cur2 = *error_limit.offset(cur2 as isize);
            cur0 += *inptr.offset(0isize) as c_int;
            cur1 += *inptr.offset(1isize) as c_int;
            cur2 += *inptr.offset(2isize) as c_int;
            cur0 = *range_limit.offset(cur0 as isize) as c_int;
            cur1 = *range_limit.offset(cur1 as isize) as c_int;
            cur2 = *range_limit.offset(cur2 as isize) as c_int;
            cachep = &mut *(*(*histogram.offset((cur0 >> C0_SHIFT) as isize))
                .offset((cur1 >> C1_SHIFT) as isize))
            .as_mut_ptr()
            .offset((cur2 >> C2_SHIFT) as isize) as *mut histcell;
            if *cachep as c_int == 0i32 {
                fill_inverse_cmap(cinfo, cur0 >> C0_SHIFT, cur1 >> C1_SHIFT, cur2 >> C2_SHIFT);
            }
            let mut pixcode: c_int = *cachep as c_int - 1i32;
            *outptr = pixcode as JSAMPLE;
            cur0 -= *colormap0.offset(pixcode as isize) as c_int;
            cur1 -= *colormap1.offset(pixcode as isize) as c_int;
            cur2 -= *colormap2.offset(pixcode as isize) as c_int;
            let mut bnexterr: LOCFSERROR = 0;
            bnexterr = cur0;
            *errorptr.offset(0isize) = (bpreverr0 + cur0 * 3i32) as FSERROR;
            bpreverr0 = belowerr0 + cur0 * 5i32;
            belowerr0 = bnexterr;
            cur0 *= 7i32;
            bnexterr = cur1;
            *errorptr.offset(1isize) = (bpreverr1 + cur1 * 3i32) as FSERROR;
            bpreverr1 = belowerr1 + cur1 * 5i32;
            belowerr1 = bnexterr;
            cur1 *= 7i32;
            bnexterr = cur2;
            *errorptr.offset(2isize) = (bpreverr2 + cur2 * 3i32) as FSERROR;
            bpreverr2 = belowerr2 + cur2 * 5i32;
            belowerr2 = bnexterr;
            cur2 *= 7i32;
            inptr = inptr.offset(dir3 as isize);
            outptr = outptr.offset(dir as isize);
            errorptr = errorptr.offset(dir3 as isize);
            col = col.wrapping_sub(1)
        }
        *errorptr.offset(0isize) = bpreverr0 as FSERROR;
        *errorptr.offset(1isize) = bpreverr1 as FSERROR;
        *errorptr.offset(2isize) = bpreverr2 as FSERROR;
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
unsafe extern "C" fn init_error_limit(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut table: *mut c_int = 0 as *mut c_int;
    let mut in_0: c_int = 0;
    let mut out: c_int = 0;
    table = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE * 2i32 + 1i32) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut c_int;
    table = table.offset(MAXJSAMPLE as isize);
    (*cquantize).error_limiter = table;
    out = 0i32;
    in_0 = 0i32;
    while in_0 < STEPSIZE {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out += 1
    }
    while in_0 < STEPSIZE * 3i32 {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out += (if 0 != in_0 & 1i32 { 0i32 } else { 1i32 })
    }
    while in_0 <= MAXJSAMPLE {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1
    }
}
pub const STEPSIZE: c_int = (MAXJSAMPLE + 1i32) / 16i32;
/*
 * Finish up at the end of each pass.
 */
unsafe extern "C" fn finish_pass1(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    (*cinfo).colormap = (*cquantize).sv_colormap;
    select_colors(cinfo, (*cquantize).desired);
    (*cquantize).needs_zeroed = TRUE;
}
unsafe extern "C" fn finish_pass2(mut cinfo: j_decompress_ptr) {}
/* no work */
/*
 * Initialize for each processing pass.
 */
unsafe extern "C" fn start_pass_2_quant(mut cinfo: j_decompress_ptr, mut is_pre_scan: boolean) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut i: c_int = 0;
    if (*cinfo).dither_mode as c_uint != JDITHER_NONE as c_int as c_uint {
        (*cinfo).dither_mode = JDITHER_FS
    }
    if 0 != is_pre_scan {
        (*cquantize).pub_0.color_quantize = Some(
            prescan_quantize
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPARRAY,
                    _: JSAMPARRAY,
                    _: c_int,
                ) -> (),
        );
        (*cquantize).pub_0.finish_pass =
            Some(finish_pass1 as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
        (*cquantize).needs_zeroed = TRUE
    } else {
        if (*cinfo).dither_mode as c_uint == JDITHER_FS as c_int as c_uint {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_fs_dither
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPARRAY,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        } else {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_no_dither
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPARRAY,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
        (*cquantize).pub_0.finish_pass =
            Some(finish_pass2 as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
        i = (*cinfo).actual_number_of_colors;
        if i < 1i32 {
            (*(*cinfo).err).msg_code = JERR_QUANT_FEW_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = 1i32;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if i > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = 255i32 + 1i32;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if (*cinfo).dither_mode as c_uint == JDITHER_FS as c_int as c_uint {
            let mut arraysize: size_t =
                ((*cinfo).output_width.wrapping_add(2i32 as c_uint) as c_ulong).wrapping_mul(
                    (3i32 as c_ulong).wrapping_mul(::std::mem::size_of::<FSERROR>() as c_ulong),
                );
            if (*cquantize).fserrors.is_null() {
                (*cquantize).fserrors = (*(*cinfo).mem)
                    .alloc_large
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    JPOOL_IMAGE,
                    arraysize,
                ) as FSERRPTR
            }
            jzero_far((*cquantize).fserrors as *mut c_void, arraysize);
            if (*cquantize).error_limiter.is_null() {
                init_error_limit(cinfo);
            }
            (*cquantize).on_odd_row = FALSE
        }
    }
    if 0 != (*cquantize).needs_zeroed {
        i = 0i32;
        while i < HIST_C0_ELEMS {
            jzero_far(
                *histogram.offset(i as isize) as *mut c_void,
                ((HIST_C1_ELEMS * HIST_C2_ELEMS) as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<histcell>() as c_ulong),
            );
            i += 1
        }
        (*cquantize).needs_zeroed = FALSE
    };
}
/*
 * Switch to a new external colormap between output passes.
 */
unsafe extern "C" fn new_color_map_2_quant(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    (*cquantize).needs_zeroed = TRUE;
}
/*
 * Module initialization routine for 2-pass color quantization.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_2pass_quantizer(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = 0 as *mut my_cquantizer;
    let mut i: c_int = 0;
    cquantize = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_cquantizer>() as c_ulong,
    ) as my_cquantize_ptr;
    (*cinfo).cquantize = cquantize as *mut jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass =
        Some(start_pass_2_quant as unsafe extern "C" fn(_: j_decompress_ptr, _: boolean) -> ());
    (*cquantize).pub_0.new_color_map =
        Some(new_color_map_2_quant as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*cquantize).fserrors = NULL as FSERRPTR;
    (*cquantize).error_limiter = NULL as *mut c_int;
    if (*cinfo).out_color_components != 3i32 {
        (*(*cinfo).err).msg_code = JERR_NOTIMPL as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cquantize).histogram = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (HIST_C0_ELEMS as c_ulong).wrapping_mul(::std::mem::size_of::<hist2d>() as c_ulong),
    ) as hist3d;
    i = 0i32;
    while i < HIST_C0_ELEMS {
        let ref mut fresh13 = *(*cquantize).histogram.offset(i as isize);
        *fresh13 = (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((HIST_C1_ELEMS * HIST_C2_ELEMS) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<histcell>() as c_ulong),
        ) as hist2d;
        i += 1
    }
    (*cquantize).needs_zeroed = TRUE;
    if 0 != (*cinfo).enable_2pass_quant {
        let mut desired: c_int = (*cinfo).desired_number_of_colors;
        if desired < 8i32 {
            (*(*cinfo).err).msg_code = JERR_QUANT_FEW_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = 8i32;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if desired > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = 255i32 + 1i32;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*cquantize).sv_colormap = (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            desired as JDIMENSION,
            3i32 as JDIMENSION,
        );
        (*cquantize).desired = desired
    } else {
        (*cquantize).sv_colormap = NULL as JSAMPARRAY
    }
    if (*cinfo).dither_mode as c_uint != JDITHER_NONE as c_int as c_uint {
        (*cinfo).dither_mode = JDITHER_FS
    }
    if (*cinfo).dither_mode as c_uint == JDITHER_FS as c_int as c_uint {
        (*cquantize).fserrors = (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*cinfo).output_width.wrapping_add(2i32 as c_uint) as c_ulong).wrapping_mul(
                (3i32 as c_ulong).wrapping_mul(::std::mem::size_of::<FSERROR>() as c_ulong),
            ),
        ) as FSERRPTR;
        init_error_limit(cinfo);
    };
}
