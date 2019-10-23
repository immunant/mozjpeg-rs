


















































































































































































































































use libc::{c_void, c_ulong, c_int, c_long, self};pub use crate::jpegint_h::{inverse_DCT_method_ptr, jzero_far, JBUF_CRANK_DEST,
                           JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
                           JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_parser_method,
                           jpeg_marker_reader, jpeg_marker_struct,
                           jpeg_memory_mgr, jpeg_progress_mgr,
                           jpeg_saved_marker_ptr, jpeg_source_mgr,
                           jpeg_upsampler, jvirt_barray_control,
                           jvirt_barray_ptr, jvirt_sarray_control,
                           jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr,
                           JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
                           JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
                           JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
                           J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE};pub use crate::jmorecfg_h::{boolean, rgb_blue, rgb_green, rgb_red,
                            EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_RED,
                            EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_RED,
                            EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_RED,
                            EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_RED,
                            EXT_XBGR_BLUE, EXT_XBGR_GREEN, EXT_XBGR_RED,
                            EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_RED,
                            FALSE, INT16, JCOEF, JDIMENSION, JOCTET, JSAMPLE,
                            MAXJSAMPLE, RGB_BLUE, RGB_GREEN, RGB_RED, TRUE,
                            UINT16, UINT8};pub use crate::stddef_h::{size_t, NULL};pub use crate::jconfig_h::BITS_IN_JSAMPLE;pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
                        JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
                        JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID,
                        JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
                        JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE,
                        JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
                        JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION,
                        JERR_BAD_MCU_SIZE, JERR_BAD_PARAM,
                        JERR_BAD_PARAM_VALUE, JERR_BAD_POOL_ID,
                        JERR_BAD_PRECISION, JERR_BAD_PROGRESSION,
                        JERR_BAD_PROG_SCRIPT, JERR_BAD_SAMPLING,
                        JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE,
                        JERR_BAD_STRUCT_SIZE, JERR_BAD_VIRTUAL_ACCESS,
                        JERR_BUFFER_SIZE, JERR_CANT_SUSPEND,
                        JERR_CCIR601_NOTIMPL, JERR_COMPONENT_COUNT,
                        JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX,
                        JERR_DAC_VALUE, JERR_DHT_INDEX, JERR_DQT_INDEX,
                        JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE,
                        JERR_EOI_EXPECTED, JERR_FILE_READ, JERR_FILE_WRITE,
                        JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
                        JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG,
                        JERR_INPUT_EMPTY, JERR_INPUT_EOF,
                        JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA,
                        JERR_MODE_CHANGE, JERR_NOTIMPL, JERR_NOT_COMPILED,
                        JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                        JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                        JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                        JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                        JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                        JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                        JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
                        JERR_TFILE_SEEK, JERR_TFILE_WRITE,
                        JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
                        JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG,
                        JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
                        JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE,
                        JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
                        JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                        JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN, JTRC_EOI,
                        JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE,
                        JTRC_JFIF_EXTENSION, JTRC_JFIF_THUMBNAIL,
                        JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER,
                        JTRC_QUANTVALS, JTRC_QUANT_3_NCOLORS,
                        JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED,
                        JTRC_RECOVERY_ACTION, JTRC_RST, JTRC_SMOOTH_NOTIMPL,
                        JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                        JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE,
                        JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                        JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                        JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                        JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                        JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR,
                        JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
                        JWRN_TOO_MUCH_DATA};

pub type my_cquantize_ptr = *mut my_cquantizer;

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

pub type FSERROR = INT16;
/* type for the 2nd-level pointers */

pub type hist3d = *mut hist2d;
/* typedefs for the array */

pub type hist2d = *mut hist1d;
/* for pointers to histogram cells */

pub type hist1d = [histcell; 32];

pub type histcell = UINT16;

pub type boxptr = *mut box_0;

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
/* histogram cell; prefer an unsigned type */

pub type histptr = *mut histcell;
/* 16 bits should be enough */

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

pub const R_SCALE: c_int = 2i32;
/* scale R distances by this much */

pub const G_SCALE: c_int = 3i32;
/* scale G distances by this much */

pub const B_SCALE: c_int = 1i32;
/* and B by this much */

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

pub const MAXNUMCOLORS: c_int = MAXJSAMPLE + 1i32;
/* maximum size of colormap */
/* These will do the right thing for either R,G,B or B,G,R color order,
 * but you may not like the results for other color orders.
 */

pub const HIST_C0_BITS: c_int = 5i32;
/* bits of precision in R/B histogram */

pub const HIST_C1_BITS: c_int = 6i32;
/* bits of precision in G histogram */

pub const HIST_C2_BITS: c_int = 5i32;
/* bits of precision in B/R histogram */
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
    mut _output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    
    
    let mut histogram: hist3d = (*cquantize).histogram;
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
     let mut row:   c_int =  0i32;
    while row < num_rows {
          
         let mut ptr:   JSAMPROW =  *input_buf.offset(row as isize); let mut col:   JDIMENSION =  width;
        while col > 0u32 {
             let mut histp:   histptr =
     &mut *(*(*histogram
                .offset((*ptr.offset(0) as c_int >> C0_SHIFT) as isize))
            .offset((*ptr.offset(1) as c_int >> C1_SHIFT) as isize))
            .as_mut_ptr()
            .offset((*ptr.offset(2) as c_int >> C2_SHIFT) as isize)
                as *mut histcell;
            /* increment, check for overflow and undo increment if so. */
            *histp = *histp + 1;
            if *histp as c_int <= 0i32 {
                *histp = *histp - 1
            }
            ptr = ptr.offset(3);
            col -=  1
        }
        row += 1
    }
}

unsafe extern "C" fn find_biggest_color_pop(
    mut boxlist: boxptr,
    mut numboxes: c_int,
) -> boxptr
/* Find the splittable box with the largest color population */
/* Returns NULL if no splittable boxes remain */ {
    
    
      
    let mut which: boxptr = NULL as boxptr;
    
     let mut i:   c_int =  0i32; let mut boxp:   boxptr =  boxlist;
    while i < numboxes {
         let mut maxc:  c_long =  0i64;if (*boxp).colorcount > maxc && (*boxp).volume > 0i64 {
            which = boxp;
            maxc = (*boxp).colorcount
        }
        i += 1;
        boxp = boxp.offset(1)
    }
    return which;
}

unsafe extern "C" fn find_biggest_volume(mut boxlist: boxptr, mut numboxes: c_int) -> boxptr
/* Find the splittable box with the largest (scaled) volume */
/* Returns NULL if no splittable boxes remain */ {
    
    
      
    let mut which: boxptr = NULL as boxptr;
    
     let mut i:   c_int =  0i32; let mut boxp:   boxptr =  boxlist;
    while i < numboxes {
         let mut maxv:  JLONG =  0i64;if (*boxp).volume > maxv {
            which = boxp;
            maxv = (*boxp).volume
        }
        i += 1;
        boxp = boxp.offset(1)
    }
    return which;
}

unsafe extern "C" fn update_box(mut cinfo: j_decompress_ptr, mut boxp: boxptr)
/* Shrink the min/max bounds of a box to enclose only nonzero elements, */
/* and recompute its volume and population */
{
     let mut histp:  histptr =  ::std::ptr::null_mut::< histcell>(); let mut c0:  c_int =  0; let mut c1:  c_int =  0; let mut c2:  c_int =  0;          let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
     let mut c0min:   c_int =  (*boxp).c0min; let mut c0max:   c_int =  (*boxp).c0max; let mut c1min:   c_int =  (*boxp).c1min; let mut c1max:   c_int =  (*boxp).c1max; let mut c2min:   c_int =  (*boxp).c2min; let mut c2max:   c_int =  (*boxp).c2max;
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
    /* Update box volume.
     * We use 2-norm rather than real volume here; this biases the method
     * against making long narrow boxes, and it has the side benefit that
     * a box is splittable iff norm > 0.
     * Since the differences are expressed in histogram-cell units,
     * we have to shift back to JSAMPLE units to get consistent distances;
     * after which, we scale according to the selected distance scale factors.
     */
    
    
     let mut dist0:   JLONG =
     ((c0max - c0min << C0_SHIFT)
        * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
        as JLONG; let mut dist1:   JLONG =
     ((c1max - c1min << C1_SHIFT)
        * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
        as JLONG; let mut dist2:   JLONG =
     ((c2max - c2min << C2_SHIFT)
        * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
        as JLONG;
    (*boxp).volume = dist0 * dist0 + dist1 * dist1 + dist2 * dist2;
     let mut ccount:   c_long =  0i64;
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
                histp = histp.offset(1)
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
) -> c_int
/* Repeatedly select and split the largest box until we have enough boxes */ {
    
    
    
    
    
    
    
    
    while numboxes < desired_colors {
        /* Select box to split.
         * Current algorithm: by population for first half, then by volume.
         */
         let mut n:  c_int =  0; let mut lb:  c_int =  0;    let mut cmax:  c_int =  0; let mut b1:  boxptr =  ::std::ptr::null_mut::< box_0>(); if numboxes * 2i32 <= desired_colors {
            b1 = find_biggest_color_pop(boxlist, numboxes)
        } else {
            b1 = find_biggest_volume(boxlist, numboxes)
        } /* where new box will go */
        if b1.is_null() {
            break;
        }
         let mut b2:   boxptr =  &mut *boxlist.offset(numboxes as isize) as *mut box_0;
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
        
        
         let mut c0:   c_int =
     ((*b1).c0max - (*b1).c0min << C0_SHIFT)
            * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]; let mut c1:   c_int =
     ((*b1).c1max - (*b1).c1min << C1_SHIFT)
            * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]; let mut c2:   c_int =
     ((*b1).c2max - (*b1).c2min << C2_SHIFT)
            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize];
        /* We want to break any ties in favor of green, then red, blue last.
         * This code does the right thing for R,G,B or B,G,R color orders only.
         */
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
    mut cinfo: j_decompress_ptr,
    mut boxp: boxptr,
    mut icolor: c_int,
)
/* Compute representative color for a box, put it in colormap[icolor] */
{
           let mut total:  c_long =  0i64; let mut c0total:  c_long =  0i64; let mut c1total:  c_long =  0i64; let mut c2total:  c_long =  0i64;let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
     let mut c0min:   c_int =  (*boxp).c0min; let mut c0max:   c_int =  (*boxp).c0max; let mut c1min:   c_int =  (*boxp).c1min; let mut c1max:   c_int =  (*boxp).c1max; let mut c2min:   c_int =  (*boxp).c2min; let mut c2max:   c_int =  (*boxp).c2max; let mut c0:   c_int =  c0min;
    while c0 <= c0max {
          let mut c1:   c_int =  c1min;
        while c1 <= c1max {
              
             let mut histp:   histptr =
     &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2min as isize) as *mut histcell; let mut c2:   c_int =  c2min;
            while c2 <= c2max {
                 let fresh4 = histp;
                histp = histp.offset(1);
                 let mut count:   c_long =  *fresh4 as c_long;
                if count != 0i64 {
                    total += count;
                    c0total +=
                        ((c0 << C0_SHIFT) + (1i32 << C0_SHIFT >> 1i32)) as c_long * count;
                    c1total +=
                        ((c1 << C1_SHIFT) + (1i32 << C1_SHIFT >> 1i32)) as c_long * count;
                    c2total +=
                        ((c2 << C2_SHIFT) + (1i32 << C2_SHIFT >> 1i32)) as c_long * count
                }
                c2 += 1
            }
            c1 += 1
        }
        c0 += 1
    }
    *(*(*cinfo).colormap.offset(0)).offset(icolor as isize) =
        ((c0total + (total >> 1i32)) / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(1)).offset(icolor as isize) =
        ((c1total + (total >> 1i32)) / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(2)).offset(icolor as isize) =
        ((c2total + (total >> 1i32)) / total) as JSAMPLE;
}

unsafe extern "C" fn select_colors(
    mut cinfo: j_decompress_ptr,
    mut desired_colors: c_int,
)
/* Master routine for color selection */
{
    
    
       
    
     let mut boxlist:   boxptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        desired_colors as c_ulong *
    ::std::mem::size_of::<box_0>() as c_ulong,
    ) as boxptr; let mut numboxes:   c_int =  1i32;
    (*boxlist.offset(0)).c0min = 0i32;
    (*boxlist.offset(0)).c0max = MAXJSAMPLE >> C0_SHIFT;
    (*boxlist.offset(0)).c1min = 0i32;
    (*boxlist.offset(0)).c1max = MAXJSAMPLE >> C1_SHIFT;
    (*boxlist.offset(0)).c2min = 0i32;
    (*boxlist.offset(0)).c2max = MAXJSAMPLE >> C2_SHIFT;
    /* Shrink it to actually-used volume and set its statistics */
    update_box(cinfo, &mut *boxlist.offset(0));
    /* Perform median-cut to produce final box list */
    numboxes = median_cut(cinfo, boxlist, numboxes, desired_colors);
     let mut i:   c_int =  0i32;
    while i < numboxes {
        compute_color(cinfo, &mut *boxlist.offset(i as isize), i);
        i += 1
    }
    (*cinfo).actual_number_of_colors = numboxes;
    (*(*cinfo).err).msg_code = super::jerror::JTRC_QUANT_SELECTED as c_int;
    (*(*cinfo).err).msg_parm.i[0] = numboxes;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
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

pub const BOX_C0_ELEMS: c_int = 1i32 << BOX_C0_LOG;
/* # of hist cells in update box */

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
) -> c_int
/* Locate the colormap entries close enough to an update box to be candidates
 * for the nearest entry to some cell(s) in the update box.  The update box
 * is specified by the center coordinates of its first cell.  The number of
 * candidate colormap entries is returned, and their colormap indexes are
 * placed in colorlist[].
 * This routine uses Heckbert's "locally sorted search" criterion to select
 * the colors that need further consideration.
 */ {
              let mut mindist:  [JLONG; 256] =  [0; 256];let mut numcolors: c_int = (*cinfo).actual_number_of_colors; /* min distance to colormap entry i */
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    /* Compute true coordinates of update box's upper corner and center.
     * Actually we compute the coordinates of the center of the upper-corner
     * histogram cell, which are the upper bounds of the volume we care about.
     * Note that since ">>" rounds down, the "center" values may be closer to
     * min than to max; hence comparisons to them must be "<=", not "<".
     */
    
    
    
    
    
    
    /* For each color in colormap, find:
     *  1. its minimum squared-distance to any point in the update box
     *     (zero if color is within update box);
     *  2. its maximum squared-distance to any point in the update box.
     * Both of these can be found by considering only the corners of the box.
     * We save the minimum distance for each color in mindist[];
     * only the smallest maximum distance is of interest.
     */
    
     let mut maxc0:   c_int =
     minc0 + ((1i32 << BOX_C0_SHIFT) - (1i32 << C0_SHIFT)); let mut centerc0:   c_int =  minc0 + maxc0 >> 1i32; let mut maxc1:   c_int =
     minc1 + ((1i32 << BOX_C1_SHIFT) - (1i32 << C1_SHIFT)); let mut centerc1:   c_int =  minc1 + maxc1 >> 1i32; let mut maxc2:   c_int =
     minc2 + ((1i32 << BOX_C2_SHIFT) - (1i32 << C2_SHIFT)); let mut centerc2:   c_int =  minc2 + maxc2 >> 1i32; let mut minmaxdist:   JLONG =  0x7fffffffi64; let mut i:   c_int =  0i32;
    while i < numcolors {
         let mut min_dist:  JLONG =  0; let mut max_dist:  JLONG =  0; let mut tdist:  JLONG =  0; let mut x:   c_int =
     *(*(*cinfo).colormap.offset(0)).offset(i as isize) as c_int;
        if x < minc0 {
            tdist = ((x - minc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - maxc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist = tdist * tdist
        } else if x > maxc0 {
            tdist = ((x - maxc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - minc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist = tdist * tdist
        } else {
            /* within cell range so no contribution to min_dist */
            min_dist = 0i64;
            if x <= centerc0 {
                tdist = ((x - maxc0)
                    * c_scales
                        [rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as JLONG;
                max_dist = tdist * tdist
            } else {
                tdist = ((x - minc0)
                    * c_scales
                        [rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as JLONG;
                max_dist = tdist * tdist
            }
        }
        x = *(*(*cinfo).colormap.offset(1)).offset(i as isize) as c_int;
        if x < minc1 {
            tdist = ((x - minc1)
                * c_scales
                    [rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc1)
                * c_scales
                    [rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x > maxc1 {
            tdist = ((x - maxc1)
                * c_scales
                    [rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc1)
                * c_scales
                    [rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x <= centerc1 {
            tdist = ((x - maxc1)
                * c_scales
                    [rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc1)
                * c_scales
                    [rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        }
        x = *(*(*cinfo).colormap.offset(2)).offset(i as isize) as c_int;
        if x < minc2 {
            tdist = ((x - minc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x > maxc2 {
            tdist = ((x - maxc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else if x <= centerc2 {
            tdist = ((x - maxc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
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
     let mut ncolors:   c_int =  0i32;
    i = 0i32;
    while i < numcolors {
        if mindist[i as usize] <= minmaxdist {
            let fresh5 = ncolors;
            ncolors +=  1;
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
)
/* Find the closest colormap entry for each cell in the update box,
 * given the list of candidate colors prepared by find_nearby_colors.
 * Return the indexes of the closest entries in the bestcolor[] array.
 * This routine uses Thomas' incremental distance calculation method to
 * find the distance from a colormap entry to successive cells in the box.
 */
{
       let mut bestdist:  [JLONG; 128] =  [0; 128];
    
     let mut bptr:   *mut JLONG =  bestdist.as_mut_ptr(); let mut i:   c_int =  BOX_C0_ELEMS * BOX_C1_ELEMS * BOX_C2_ELEMS - 1i32;
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
                
        
        
         let mut icolor:   c_int =  *colorlist.offset(i as isize) as c_int; let mut inc0:   JLONG =
     ((minc0 - *(*(*cinfo).colormap.offset(0)).offset(icolor as isize) as c_int)
            * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
            as JLONG; let mut dist0:   JLONG =  inc0 * inc0; let mut inc1:   JLONG =
     ((minc1 - *(*(*cinfo).colormap.offset(1)).offset(icolor as isize) as c_int)
            * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
            as JLONG;
        dist0 += inc1 * inc1;
         let mut inc2:   JLONG =
     ((minc2 - *(*(*cinfo).colormap.offset(2)).offset(icolor as isize) as c_int)
            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
            as JLONG;
        dist0 += inc2 * inc2;
        /* Form the initial difference increments */
        inc0 = inc0
            * (2i32
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as c_long
            + ((1i32 << C0_SHIFT)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as c_long;
        inc1 =
            inc1 * (2i32
                * ((1i32 << C1_SHIFT)
                    * c_scales
                        [rgb_green[(*cinfo).out_color_space as usize] as usize]))
                as c_long
                + ((1i32 << C1_SHIFT)
                    * c_scales
                        [rgb_green[(*cinfo).out_color_space as usize] as usize]
                    * ((1i32 << C1_SHIFT)
                        * c_scales[rgb_green[(*cinfo).out_color_space as usize]
                            as usize])) as c_long;
        inc2 = inc2
            * (2i32
                * ((1i32 << C2_SHIFT)
                    * c_scales
                        [rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as c_long
            + ((1i32 << C2_SHIFT)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]
                * ((1i32 << C2_SHIFT)
                    * c_scales
                        [rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as c_long;
        /* Now loop over all cells in box, updating distance per Thomas method */
        bptr = bestdist.as_mut_ptr();
        
        
         let mut cptr:   *mut JSAMPLE =  bestcolor; let mut xx0:   JLONG =  inc0; let mut ic0:   c_int =  BOX_C0_ELEMS - 1i32;
        while ic0 >= 0i32 {
               
            
             let mut dist1:   JLONG =  dist0; let mut xx1:   JLONG =  inc1; let mut ic1:   c_int =  BOX_C1_ELEMS - 1i32;
            while ic1 >= 0i32 {
                   
                
                 let mut dist2:   JLONG =  dist1; let mut xx2:   JLONG =  inc2; let mut ic2:   c_int =  BOX_C2_ELEMS - 1i32;
                while ic2 >= 0i32 {
                    if dist2 < *bptr {
                        *bptr = dist2;
                        *cptr = icolor as JSAMPLE
                    }
                    dist2 += xx2;
                    xx2 += (2i32
                        * ((1i32 << C2_SHIFT)
                            * c_scales[rgb_blue
                                [(*cinfo).out_color_space as usize]
                                as usize])
                        * ((1i32 << C2_SHIFT)
                            * c_scales[rgb_blue
                                [(*cinfo).out_color_space as usize]
                                as usize])) as c_long;
                    bptr = bptr.offset(1);
                    cptr = cptr.offset(1);
                    ic2 -= 1
                }
                dist1 += xx1;
                xx1 += (2i32
                    * ((1i32 << C1_SHIFT)
                        * c_scales[rgb_green[(*cinfo).out_color_space as usize]
                            as usize])
                    * ((1i32 << C1_SHIFT)
                        * c_scales[rgb_green[(*cinfo).out_color_space as usize]
                            as usize])) as c_long;
                ic1 -= 1
            }
            dist0 += xx0;
            xx0 += (2i32
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [rgb_red[(*cinfo).out_color_space as usize] as usize])
                * ((1i32 << C0_SHIFT)
                    * c_scales
                        [rgb_red[(*cinfo).out_color_space as usize] as usize]))
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
)
/* Fill the inverse-colormap entries in the update box that contains */
/* histogram cell c0/c1/c2.  (Only that one cell MUST be filled, but */
/* we can fill as many others as we wish.) */
{
          let mut colorlist:  [JSAMPLE; 256] =  [0; 256];  let mut bestcolor:  [JSAMPLE; 128] =  [0; 128];let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* lower left corner of update box */
    let mut histogram: hist3d = (*cquantize).histogram; /* pointer into bestcolor[] array */
     /* pointer into main cache array */
    
    
    
    
    
    
    
     /* number of candidate colors */
    
    
    /* Convert cell coordinates to update box ID */
    c0 >>= BOX_C0_LOG;
    c1 >>= BOX_C1_LOG;
    c2 >>= BOX_C2_LOG;
    /* Compute true coordinates of update box's origin corner.
     * Actually we compute the coordinates of the center of the corner
     * histogram cell, which are the lower bounds of the volume we care about.
     */
    
    
    
    /* Determine which colormap entries are close enough to be candidates
     * for the nearest entry to some cell in the update box.
     */
     let mut minc0:   c_int =
     (c0 << BOX_C0_SHIFT) + (1i32 << C0_SHIFT >> 1i32); let mut minc1:   c_int =
     (c1 << BOX_C1_SHIFT) + (1i32 << C1_SHIFT >> 1i32); let mut minc2:   c_int =
     (c2 << BOX_C2_SHIFT) + (1i32 << C2_SHIFT >> 1i32); let mut numcolors:   c_int =
     find_nearby_colors(cinfo, minc0, minc1, minc2, colorlist.as_mut_ptr());
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
    
     let mut cptr:   *mut JSAMPLE =  bestcolor.as_mut_ptr(); let mut ic0:   c_int =  0i32;
    while ic0 < BOX_C0_ELEMS {
          let mut ic1:   c_int =  0i32;
        while ic1 < BOX_C1_ELEMS {
              
             let mut cachep:   histptr =
     &mut *(*(*histogram.offset((c0 + ic0) as isize)).offset((c1 + ic1) as isize))
                .as_mut_ptr()
                .offset(c2 as isize) as *mut histcell; let mut ic2:   c_int =  0i32;
            while ic2 < BOX_C2_ELEMS {
                let fresh7 = cptr;
                cptr = cptr.offset(1);
                let fresh8 = cachep;
                cachep = cachep.offset(1);
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
)
/* This version performs no dithering */
{
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    
    
    
    
    
    
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
     let mut row:   c_int =  0i32;
    while row < num_rows {
           
        
         let mut inptr:   JSAMPROW =  *input_buf.offset(row as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(row as isize); let mut col:   JDIMENSION =  width;
        while col > 0u32 {
               let fresh9 = inptr;
            inptr = inptr.offset(1);
             let mut c0:   c_int =  *fresh9 as c_int >> C0_SHIFT;
            let fresh10 = inptr;
            inptr = inptr.offset(1);
             let mut c1:   c_int =  *fresh10 as c_int >> C1_SHIFT;
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            
             let mut c2:   c_int =  *fresh11 as c_int >> C2_SHIFT; let mut cachep:   histptr =
     &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2 as isize) as *mut histcell;
            /* If we have not seen this color before, find nearest colormap entry */
            /* and update the cache */
            if *cachep as c_int == 0i32 {
                fill_inverse_cmap(cinfo, c0, c1, c2);
            }
            /* Now emit the colormap index for this cell */
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 = (*cachep as c_int - 1i32) as JSAMPLE;
            col -=  1
        }
        row += 1
    }
}

unsafe extern "C" fn pass2_fs_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
)
/* This version performs Floyd-Steinberg dithering */
{
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* current error or pixel value */
    let mut histogram: hist3d = (*cquantize).histogram; /* error for pixel below cur */
     /* error for below/prev col */
     /* => fserrors[] at column before current */
     /* => current input pixel */
     /* => current output pixel */
     /* +1 or -1 depending on direction */
     /* 3*dir, for advancing inptr & errorptr */
    
    
    
    
    
    
    
    
    
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut error_limit: *mut c_int = (*cquantize).error_limiter;
    let mut colormap0: JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut colormap1: JSAMPROW = *(*cinfo).colormap.offset(1);
    let mut colormap2: JSAMPROW = *(*cinfo).colormap.offset(2);
     let mut row:   c_int =  0i32;
    while row < num_rows {
                  let mut errorptr:  FSERRPTR =  ::std::ptr::null_mut::< FSERROR>();   let mut dir:  c_int =  0; let mut dir3:  c_int =  0; 
         let mut inptr:   JSAMPROW =  *input_buf.offset(row as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(row as isize);
        if (*cquantize).on_odd_row != 0 {
            /* work right to left in this row */
            inptr = inptr.offset(
                ((
                width - 1u32) * 3u32) as isize,
            );
            outptr = outptr.offset((width - 1u32) as isize);
            dir = -1i32;
            dir3 = -3i32; /* so point to rightmost pixel */
            /* flip for next time */
            errorptr = (*cquantize).fserrors.offset(
                ((
                width + 1u32) * 3u32) as isize,
            ); /* => entry after last column */
            (*cquantize).on_odd_row = FALSE
        } else {
            /* work left to right in this row */
            dir = 1i32;
            dir3 = 3i32;
            /* flip for next time */
            errorptr = (*cquantize).fserrors; /* => entry before first real column */
            (*cquantize).on_odd_row = TRUE
        }
        
        
        
        
        
        
        
        
        
         let mut cur2:   LOCFSERROR =  0i32; let mut cur1:   LOCFSERROR =  cur2; let mut cur0:   LOCFSERROR =  cur1; let mut belowerr2:   LOCFSERROR =  0i32; let mut belowerr1:   LOCFSERROR =  belowerr2; let mut belowerr0:   LOCFSERROR =  belowerr1; let mut bpreverr2:   LOCFSERROR =  0i32; let mut bpreverr1:   LOCFSERROR =  bpreverr2; let mut bpreverr0:   LOCFSERROR =  bpreverr1; let mut col:   JDIMENSION =  width;
        while col > 0u32 {
            /* curN holds the error propagated from the previous pixel on the
             * current line.  Add the error propagated from the previous line
             * to form the complete error correction term for this pixel, and
             * round the error term (which is expressed * 16) to an integer.
             * RIGHT_SHIFT rounds towards minus infinity, so adding 8 is correct
             * for either sign of the error value.
             * Note: errorptr points to *previous* column's array entry.
             */
              cur0 = cur0 + *errorptr.offset((dir3 + 0i32) as isize) as c_int + 8i32 >> 4i32;
            cur1 = cur1 + *errorptr.offset((dir3 + 1i32) as isize) as c_int + 8i32 >> 4i32;
            cur2 = cur2 + *errorptr.offset((dir3 + 2i32) as isize) as c_int + 8i32 >> 4i32;
            /* advance errorptr to current column */
            cur0 = *error_limit.offset(cur0 as isize);
            cur1 = *error_limit.offset(cur1 as isize);
            cur2 = *error_limit.offset(cur2 as isize);
            cur0 += *inptr.offset(0) as c_int;
            cur1 += *inptr.offset(1) as c_int;
            cur2 += *inptr.offset(2) as c_int;
            cur0 = *range_limit.offset(cur0 as isize) as c_int;
            cur1 = *range_limit.offset(cur1 as isize) as c_int;
            cur2 = *range_limit.offset(cur2 as isize) as c_int;
             let mut cachep:   histptr =
     &mut *(*(*histogram.offset((cur0 >> C0_SHIFT) as isize))
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
            
             let mut bnexterr:   LOCFSERROR =  cur0;
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
            col -=  1
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

unsafe extern "C" fn init_error_limit(mut cinfo: j_decompress_ptr)
/* Allocate and fill in the error_limiter table */
{
       let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* so can index -MAXJSAMPLE .. +MAXJSAMPLE */
    
    
    
     let mut table:   *mut c_int =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE * 2i32 + 1i32) as c_ulong *
    ::std::mem::size_of::<c_int>() as c_ulong,
    ) as *mut c_int;
    table = table.offset(MAXJSAMPLE as isize);
    (*cquantize).error_limiter = table;
    
     let mut out:   c_int =  0i32; let mut in_0:   c_int =  0i32;
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
        out += if in_0 & 1i32 != 0 { 0i32 } else { 1i32 }
    }
    /* Clamp the rest to final out value (which is (MAXJSAMPLE+1)/8) */
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
    /* Select the representative colors and fill in cinfo->colormap */
    (*cinfo).colormap = (*cquantize).sv_colormap;
    select_colors(cinfo, (*cquantize).desired);
    /* Force next pass to zero the color index table */
    (*cquantize).needs_zeroed = TRUE;
}

unsafe extern "C" fn finish_pass2(mut _cinfo: j_decompress_ptr) {
    /* no work */
}
/*
 * Initialize for each processing pass.
 */

unsafe extern "C" fn start_pass_2_quant(
    mut cinfo: j_decompress_ptr,
    mut is_pre_scan: boolean,
) {
     let mut i:  c_int =  0;let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    
    /* Only F-S dithering or no dithering is supported. */
    /* If user asks for ordered dither, give him F-S. */
    if  (*cinfo).dither_mode
        !=  JDITHER_NONE
    {
        (*cinfo).dither_mode = JDITHER_FS
    }
    if is_pre_scan != 0 {
        /* Set up method pointers */
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
    /* Always zero histogram */
    } else {
        /* Set up method pointers */
        if  (*cinfo).dither_mode
            ==  JDITHER_FS
        {
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
        /* Make sure color count is acceptable */
        i = (*cinfo).actual_number_of_colors;
        if i < 1i32 {
            (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_FEW_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0] = 1i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        if i > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_MANY_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        if  (*cinfo).dither_mode
            ==  JDITHER_FS
        {
            let mut arraysize: size_t =
                ((*cinfo).output_width + 2u32) as c_ulong *
    (3u64 *
         ::std::mem::size_of::<FSERROR>() as c_ulong);
            /* Allocate Floyd-Steinberg workspace if we didn't already. */
            if (*cquantize).fserrors.is_null() {
                (*cquantize).fserrors = Some(
                    (*(*cinfo).mem)
                        .alloc_large
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    JPOOL_IMAGE,
                    arraysize,
                ) as FSERRPTR
            }
            /* Initialize the propagated errors to zero. */
            jzero_far((*cquantize).fserrors as *mut c_void, arraysize);
            /* Make the error-limit table if we didn't already. */
            if (*cquantize).error_limiter.is_null() {
                init_error_limit(cinfo);
            }
            (*cquantize).on_odd_row = FALSE
        }
    }
    /* Zero the histogram or inverse color map, if necessary */
    if (*cquantize).needs_zeroed != 0 {
        i = 0i32;
        while i < HIST_C0_ELEMS {
            jzero_far(
                *histogram.offset(i as isize) as *mut c_void,
                (HIST_C1_ELEMS * HIST_C2_ELEMS) as c_ulong *
    ::std::mem::size_of::<histcell>() as c_ulong,
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
    /* Reset the inverse color map */
    (*cquantize).needs_zeroed = TRUE;
}
/*
 * Module initialization routine for 2-pass color quantization.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_2pass_quantizer(mut cinfo: j_decompress_ptr) {
      
     let mut cquantize:   my_cquantize_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_cquantizer>() as c_ulong,
    ) as my_cquantize_ptr;
    (*cinfo).cquantize = cquantize as *mut jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass = Some(
        start_pass_2_quant
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: boolean,
            ) -> (),
    );
    (*cquantize).pub_0.new_color_map = Some(
        new_color_map_2_quant as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
    (*cquantize).fserrors = NULL as FSERRPTR;
    (*cquantize).error_limiter = NULL as *mut c_int;
    /* Make sure jdmaster didn't give me a case I can't handle */
    if (*cinfo).out_color_components != 3i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_NOTIMPL as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Allocate the histogram/inverse colormap storage */
    (*cquantize).histogram = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        HIST_C0_ELEMS as c_ulong *
    ::std::mem::size_of::<hist2d>() as c_ulong,
    ) as hist3d; /* histogram is garbage now */
     let mut i:   c_int =  0i32;
    while i < HIST_C0_ELEMS {
        let ref mut fresh13 = *(*cquantize).histogram.offset(i as isize);
        *fresh13 = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (HIST_C1_ELEMS * HIST_C2_ELEMS) as c_ulong *
    ::std::mem::size_of::<histcell>() as c_ulong,
        ) as hist2d;
        i += 1
    }
    (*cquantize).needs_zeroed = TRUE;
    /* Allocate storage for the completed colormap, if required.
     * We do this now since it may affect the memory manager's space
     * calculations.
     */
    if (*cinfo).enable_2pass_quant != 0 {
        /* Make sure color count is acceptable */
        let mut desired: c_int = (*cinfo).desired_number_of_colors;
        /* Lower bound on # of colors ... somewhat arbitrary as long as > 0 */
        if desired < 8i32 {
            (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_FEW_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0] = 8i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        /* Make sure colormap indexes can be represented by JSAMPLEs */
        if desired > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_MANY_COLORS as c_int;
            (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        (*cquantize).sv_colormap = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            desired as JDIMENSION,
            3u32,
        );
        (*cquantize).desired = desired
    } else {
        (*cquantize).sv_colormap = NULL as JSAMPARRAY
    }
    /* Only F-S dithering or no dithering is supported. */
    /* If user asks for ordered dither, give him F-S. */
    if  (*cinfo).dither_mode
        !=  JDITHER_NONE
    {
        (*cinfo).dither_mode = JDITHER_FS
    }
    /* Allocate Floyd-Steinberg workspace if necessary.
     * This isn't really needed until pass 2, but again it may affect the memory
     * manager's space calculations.  Although we will cope with a later change
     * in dither_mode, we do not promise to honor max_memory_to_use if
     * dither_mode changes.
     */
    if  (*cinfo).dither_mode
        ==  JDITHER_FS
    {
        (*cquantize).fserrors = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*cinfo).output_width + 2u32) as c_ulong *
    (3u64 *
         ::std::mem::size_of::<FSERROR>() as c_ulong),
        ) as FSERRPTR;
        /* Might as well create the error-limiting table too. */
        init_error_limit(cinfo);
    };
}
/* QUANT_2PASS_SUPPORTED */
