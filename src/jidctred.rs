use libc::c_int;use libc::c_ulong;pub use crate::jdct_h::ISLOW_MULT_TYPE;
pub use crate::jdct_h::RANGE_MASK;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::CENTERJSAMPLE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
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
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
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
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
use libc;
/*
 * jidctred.c
 *
 * This file was part of the Independent JPEG Group's software.
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains inverse-DCT routines that produce reduced-size output:
 * either 4x4, 2x2, or 1x1 pixels from an 8x8 DCT block.
 *
 * The implementation is based on the Loeffler, Ligtenberg and Moschytz (LL&M)
 * algorithm used in jidctint.c.  We simply replace each 8-to-8 1-D IDCT step
 * with an 8-to-4 step that produces the four averages of two adjacent outputs
 * (or an 8-to-2 step producing two averages of four outputs, for 2x2 output).
 * These steps were derived by computing the corresponding values at the end
 * of the normal LL&M code, then simplifying as much as possible.
 *
 * 1x1 is trivial: just take the DC coefficient divided by 8.
 *
 * See jidctint.c for additional comments.
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/* Scaling is the same as in jidctint.c. */
/* Some C compilers fail to reduce "FIX(constant)" at compile time, thus
 * causing a lot of useless floating-point operations at run time.
 * To get around this we use the following pre-calculated constants.
 * If you change CONST_BITS you may want to add appropriate values.
 * (With a reasonable C compiler, you can just rely on the FIX() macro...)
 */
/* FIX(0.211164243) */
/* FIX(0.509795579) */
/* FIX(0.601344887) */
/* FIX(0.720959822) */
/* FIX(0.765366865) */
/* FIX(0.850430095) */
/* FIX(0.899976223) */
/* FIX(1.061594337) */
/* FIX(1.272758580) */
/* FIX(1.451774981) */
/* FIX(1.847759065) */
/* FIX(2.172734803) */
/* FIX(2.562915447) */
/* FIX(3.624509785) */
/* Multiply a JLONG variable by a JLONG constant to yield a JLONG result.
 * For 8-bit samples with the recommended scaling, all the variable
 * and constant values involved are no more than 16 bits wide, so a
 * 16x16->32 bit multiply can be used instead of a full 32x32 multiply.
 * For 12-bit samples, a full 32-bit multiplication will be needed.
 */
/* Dequantize a coefficient by multiplying it by the multiplier-table
 * entry; produce an int result.  In this module, both inputs and result
 * are 16 bits or less, so either int or short multiply will work.
 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 4x4 output block.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_4x4(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE =
        0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 32] = [0; 32];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = DCTSIZE;
    while ctr > 0i32 {
        /* Don't bother to process column 4, because second pass won't use it */
        if !(ctr == DCTSIZE - 4i32) {
            if *inptr.offset((DCTSIZE * 1i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 2i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 3i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 5i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 6i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 7i32) as isize) as c_int == 0i32
            {
                /* AC terms all zero; we need not examine term 4 for 4x4 output */
                let mut dcval: c_int =
                    (((*inptr.offset((8i32 * 0i32) as isize) as c_int
                        * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                        as c_ulong)
                        << 2i32) as JLONG as c_int;
                *wsptr.offset((DCTSIZE * 0i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 1i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 2i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 3i32) as isize) = dcval
            } else {
                tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                    as JLONG;
                tmp0 = ((tmp0 as c_ulong) << 13i32 + 1i32) as JLONG;
                z2 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 2i32) as isize) as c_int)
                    as JLONG;
                z3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 6i32) as isize) as c_int)
                    as JLONG;
                tmp2 = z2 * 15137i32 as JLONG
                    + z3 * -(6270i32 as JLONG);
                tmp10 = tmp0 + tmp2;
                tmp12 = tmp0 - tmp2;
                z1 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 7i32) as isize) as c_int)
                    as JLONG;
                z2 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 5i32) as isize) as c_int)
                    as JLONG;
                z3 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 3i32) as isize) as c_int)
                    as JLONG;
                z4 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 1i32) as isize) as c_int)
                    as JLONG;
                tmp0 = z1 * -(1730i32 as JLONG)
                    + z2 * 11893i32 as JLONG
                    + z3 * -(17799i32 as JLONG)
                    + z4 * 8697i32 as JLONG;
                tmp2 = z1 * -(4176i32 as JLONG)
                    + z2 * -(4926i32 as JLONG)
                    + z3 * 7373i32 as JLONG
                    + z4 * 20995i32 as JLONG;
                *wsptr.offset((DCTSIZE * 0i32) as isize) = (tmp10
                    + tmp2
                    + ((1i32 as JLONG) << 13i32 - 2i32 + 1i32 - 1i32)
                    >> 13i32 - 2i32 + 1i32)
                    as c_int;
                *wsptr.offset((DCTSIZE * 3i32) as isize) = (tmp10 - tmp2
                    + ((1i32 as JLONG) << 13i32 - 2i32 + 1i32 - 1i32)
                    >> 13i32 - 2i32 + 1i32)
                    as c_int;
                *wsptr.offset((DCTSIZE * 1i32) as isize) = (tmp12
                    + tmp0
                    + ((1i32 as JLONG) << 13i32 - 2i32 + 1i32 - 1i32)
                    >> 13i32 - 2i32 + 1i32)
                    as c_int;
                *wsptr.offset((DCTSIZE * 2i32) as isize) = (tmp12 - tmp0
                    + ((1i32 as JLONG) << 13i32 - 2i32 + 1i32 - 1i32)
                    >> 13i32 - 2i32 + 1i32)
                    as c_int
            }
        }
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize);
        ctr -= 1
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 4i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* It's not clear whether a zero row test is worthwhile here ... */
        if *wsptr.offset(1isize) == 0i32
            && *wsptr.offset(2isize) == 0i32
            && *wsptr.offset(3isize) == 0i32
            && *wsptr.offset(5isize) == 0i32
            && *wsptr.offset(6isize) == 0i32
            && *wsptr.offset(7isize) == 0i32
        {
            /* AC terms all zero */
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0isize) as JLONG
                    + ((1i32 as JLONG) << 2i32 + 3i32 - 1i32)
                    >> 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(0isize) = dcval_0;
            *outptr.offset(1isize) = dcval_0;
            *outptr.offset(2isize) = dcval_0;
            *outptr.offset(3isize) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize)
        } else {
            tmp0 = ((*wsptr.offset(0isize) as JLONG as c_ulong)
                << 13i32 + 1i32) as JLONG;
            tmp2 = *wsptr.offset(2isize) as JLONG
                * 15137i32 as JLONG
                + *wsptr.offset(6isize) as JLONG
                    * -(6270i32 as JLONG);
            tmp10 = tmp0 + tmp2;
            tmp12 = tmp0 - tmp2;
            z1 = *wsptr.offset(7isize) as JLONG;
            z2 = *wsptr.offset(5isize) as JLONG;
            z3 = *wsptr.offset(3isize) as JLONG;
            z4 = *wsptr.offset(1isize) as JLONG;
            tmp0 = z1 * -(1730i32 as JLONG)
                + z2 * 11893i32 as JLONG
                + z3 * -(17799i32 as JLONG)
                + z4 * 8697i32 as JLONG;
            tmp2 = z1 * -(4176i32 as JLONG)
                + z2 * -(4926i32 as JLONG)
                + z3 * 7373i32 as JLONG
                + z4 * 20995i32 as JLONG;
            *outptr.offset(0isize) = *range_limit.offset(
                ((tmp10
                    + tmp2
                    + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 1i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(3isize) = *range_limit.offset(
                ((tmp10 - tmp2
                    + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 1i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1isize) = *range_limit.offset(
                ((tmp12
                    + tmp0
                    + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 1i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(2isize) = *range_limit.offset(
                ((tmp12 - tmp0
                    + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 1i32) as c_int
                    & RANGE_MASK) as isize,
            );
            wsptr = wsptr.offset(DCTSIZE as isize)
        }
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 2x2 output block.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_2x2(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE =
        0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 16] = [0; 16];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = DCTSIZE;
    while ctr > 0i32 {
        /* Don't bother to process columns 2,4,6 */
        if !(ctr == DCTSIZE - 2i32
            || ctr == DCTSIZE - 4i32
            || ctr == DCTSIZE - 6i32)
        {
            if *inptr.offset((DCTSIZE * 1i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 3i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 5i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 7i32) as isize) as c_int == 0i32
            {
                /* AC terms all zero; we need not examine terms 2,4,6 for 2x2 output */
                let mut dcval: c_int =
                    (((*inptr.offset((8i32 * 0i32) as isize) as c_int
                        * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                        as c_ulong)
                        << 2i32) as JLONG as c_int;
                *wsptr.offset((DCTSIZE * 0i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 1i32) as isize) = dcval
            } else {
                z1 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                    as JLONG;
                tmp10 = ((z1 as c_ulong) << 13i32 + 2i32) as JLONG;
                z1 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 7i32) as isize) as c_int)
                    as JLONG;
                tmp0 = z1 * -(5906i32 as JLONG);
                z1 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 5i32) as isize) as c_int)
                    as JLONG;
                tmp0 += z1 * 6967i32 as JLONG;
                z1 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 3i32) as isize) as c_int)
                    as JLONG;
                tmp0 += z1 * -(10426i32 as JLONG);
                z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 1i32) as isize) as c_int)
                    as JLONG;
                tmp0 += z1 * 29692i32 as JLONG;
                *wsptr.offset((DCTSIZE * 0i32) as isize) = (tmp10
                    + tmp0
                    + ((1i32 as JLONG) << 13i32 - 2i32 + 2i32 - 1i32)
                    >> 13i32 - 2i32 + 2i32)
                    as c_int;
                *wsptr.offset((DCTSIZE * 1i32) as isize) = (tmp10 - tmp0
                    + ((1i32 as JLONG) << 13i32 - 2i32 + 2i32 - 1i32)
                    >> 13i32 - 2i32 + 2i32)
                    as c_int
            }
        }
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize);
        ctr -= 1
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 2i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* It's not clear whether a zero row test is worthwhile here ... */
        if *wsptr.offset(1isize) == 0i32
            && *wsptr.offset(3isize) == 0i32
            && *wsptr.offset(5isize) == 0i32
            && *wsptr.offset(7isize) == 0i32
        {
            /* AC terms all zero */
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0isize) as JLONG
                    + ((1i32 as JLONG) << 2i32 + 3i32 - 1i32)
                    >> 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(0isize) = dcval_0;
            *outptr.offset(1isize) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize)
        } else {
            tmp10 = ((*wsptr.offset(0isize) as JLONG as c_ulong)
                << 13i32 + 2i32) as JLONG;
            tmp0 = *wsptr.offset(7isize) as JLONG
                * -(5906i32 as JLONG)
                + *wsptr.offset(5isize) as JLONG
                    * 6967i32 as JLONG
                + *wsptr.offset(3isize) as JLONG
                    * -(10426i32 as JLONG)
                + *wsptr.offset(1isize) as JLONG
                    * 29692i32 as JLONG;
            *outptr.offset(0isize) = *range_limit.offset(
                ((tmp10
                    + tmp0
                    + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 + 2i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 2i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1isize) = *range_limit.offset(
                ((tmp10 - tmp0
                    + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 + 2i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 2i32) as c_int
                    & RANGE_MASK) as isize,
            );
            wsptr = wsptr.offset(DCTSIZE as isize)
        }
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 1x1 output block.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_1x1(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut dcval: c_int = 0;
    let mut quantptr: *mut ISLOW_MULT_TYPE =
        0 as *mut ISLOW_MULT_TYPE;
    let mut range_limit: *mut JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(CENTERJSAMPLE as isize);
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    dcval = *coef_block.offset(0isize) as c_int * *quantptr.offset(0isize) as c_int;
    dcval = (dcval as JLONG + ((1i32 as JLONG) << 3i32 - 1i32)
        >> 3i32) as c_int;
    *(*output_buf.offset(0isize)).offset(output_col as isize) =
        *range_limit.offset((dcval & RANGE_MASK) as isize);
}
