pub use crate::jdct_h::{ISLOW_MULT_TYPE, RANGE_MASK};
pub use crate::jmorecfg_h::{
    boolean, CENTERJSAMPLE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAXJSAMPLE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, JBLOCK,
    JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
    JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
    JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
    JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JQUANT_TBL,
    JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::size_t;
use libc::{self, c_int, c_ulong};
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
    /* buffers data between passes */

    let mut tmp0: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut workspace: [c_int; 32] = [0; 32];
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);

    let mut inptr: JCOEFPTR = coef_block;
    let mut quantptr: *mut ISLOW_MULT_TYPE = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = workspace.as_mut_ptr();
    let mut ctr: c_int = DCTSIZE;
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
                let mut dcval: c_int = (((*inptr.offset((8i32 * 0i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                    as c_ulong)
                    << 2i32) as c_int;
                *wsptr.offset((DCTSIZE * 0i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 1i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 2i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 3i32) as isize) = dcval
            } else {
                /* Even part */
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
                tmp2 = z2 * 15137i64 + z3 * -(6270i64);
                tmp10 = tmp0 + tmp2;
                tmp12 = tmp0 - tmp2;
                /* Odd part */
                z1 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 7i32) as isize) as c_int)
                    as JLONG; /* sqrt(2) * ( c5+c7) */
                z2 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 5i32) as isize) as c_int)
                    as JLONG; /* sqrt(2) * (c1+c3) */
                z3 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 3i32) as isize) as c_int)
                    as JLONG;
                z4 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 1i32) as isize) as c_int)
                    as JLONG;
                tmp0 = z1 * -(1730i64) + z2 * 11893i64 + z3 * -(17799i64) + z4 * 8697i64;
                tmp2 = z1 * -(4176i64) + z2 * -(4926i64) + z3 * 7373i64 + z4 * 20995i64;
                /* Final output stage */
                *wsptr.offset((DCTSIZE * 0i32) as isize) =
                    (tmp10 + tmp2 + ((1i64) << 13i32 - 2i32 + 1i32 - 1i32) >> 13i32 - 2i32 + 1i32)
                        as c_int;
                *wsptr.offset((DCTSIZE * 3i32) as isize) =
                    (tmp10 - tmp2 + ((1i64) << 13i32 - 2i32 + 1i32 - 1i32) >> 13i32 - 2i32 + 1i32)
                        as c_int;
                *wsptr.offset((DCTSIZE * 1i32) as isize) =
                    (tmp12 + tmp0 + ((1i64) << 13i32 - 2i32 + 1i32 - 1i32) >> 13i32 - 2i32 + 1i32)
                        as c_int;
                *wsptr.offset((DCTSIZE * 2i32) as isize) =
                    (tmp12 - tmp0 + ((1i64) << 13i32 - 2i32 + 1i32 - 1i32) >> 13i32 - 2i32 + 1i32)
                        as c_int
            }
        }
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
    /* Pass 2: process 4 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 4i32 {
        let mut outptr: JSAMPROW = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        /* It's not clear whether a zero row test is worthwhile here ... */
        if *wsptr.offset(1) == 0i32
            && *wsptr.offset(2) == 0i32
            && *wsptr.offset(3) == 0i32
            && *wsptr.offset(5) == 0i32
            && *wsptr.offset(6) == 0i32
            && *wsptr.offset(7) == 0i32
        {
            /* AC terms all zero */
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0) as JLONG + ((1i64) << 2i32 + 3i32 - 1i32) >> 2i32 + 3i32)
                    as c_int
                    & RANGE_MASK) as isize,
            ); /* advance pointer to next row */
            *outptr.offset(0) = dcval_0;
            *outptr.offset(1) = dcval_0;
            *outptr.offset(2) = dcval_0;
            *outptr.offset(3) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize)
        } else {
            /* Even part */
            tmp0 = ((*wsptr.offset(0) as c_ulong) << 13i32 + 1i32) as JLONG;
            tmp2 = *wsptr.offset(2) as JLONG * 15137i64 + *wsptr.offset(6) as JLONG * -(6270i64);
            tmp10 = tmp0 + tmp2;
            tmp12 = tmp0 - tmp2;
            /* Odd part */
            z1 = *wsptr.offset(7) as JLONG; /* sqrt(2) * ( c5+c7) */
            z2 = *wsptr.offset(5) as JLONG; /* sqrt(2) * (c1+c3) */
            z3 = *wsptr.offset(3) as JLONG;
            z4 = *wsptr.offset(1) as JLONG;
            tmp0 = z1 * -(1730i64) + z2 * 11893i64 + z3 * -(17799i64) + z4 * 8697i64;
            tmp2 = z1 * -(4176i64) + z2 * -(4926i64) + z3 * 7373i64 + z4 * 20995i64;
            /* Final output stage */
            *outptr.offset(0) = *range_limit.offset(
                ((tmp10 + tmp2 + ((1i64) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 1i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(3) = *range_limit.offset(
                ((tmp10 - tmp2 + ((1i64) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 1i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1) = *range_limit.offset(
                ((tmp12 + tmp0 + ((1i64) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 1i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(2) = *range_limit.offset(
                ((tmp12 - tmp0 + ((1i64) << 13i32 + 2i32 + 3i32 + 1i32 - 1i32)
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
    /* buffers data between passes */

    let mut tmp0: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut workspace: [c_int; 16] = [0; 16];
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);

    let mut inptr: JCOEFPTR = coef_block;
    let mut quantptr: *mut ISLOW_MULT_TYPE = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = workspace.as_mut_ptr();
    let mut ctr: c_int = DCTSIZE;
    while ctr > 0i32 {
        /* Don't bother to process columns 2,4,6 */
        if !(ctr == DCTSIZE - 2i32 || ctr == DCTSIZE - 4i32 || ctr == DCTSIZE - 6i32) {
            if *inptr.offset((DCTSIZE * 1i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 3i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 5i32) as isize) as c_int == 0i32
                && *inptr.offset((DCTSIZE * 7i32) as isize) as c_int == 0i32
            {
                /* AC terms all zero; we need not examine terms 2,4,6 for 2x2 output */
                let mut dcval: c_int = (((*inptr.offset((8i32 * 0i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                    as c_ulong)
                    << 2i32) as c_int;
                *wsptr.offset((DCTSIZE * 0i32) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 1i32) as isize) = dcval
            } else {
                let mut z1: JLONG = (*inptr.offset((8i32 * 0i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                    as JLONG;
                tmp10 = ((z1 as c_ulong) << 13i32 + 2i32) as JLONG;
                /* Odd part */
                z1 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 7i32) as isize) as c_int)
                    as JLONG; /* sqrt(2) * ( c7-c5+c3-c1) */
                tmp0 = z1 * -(5906i64); /* sqrt(2) * (-c1+c3+c5+c7) */
                z1 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 5i32) as isize) as c_int)
                    as JLONG; /* sqrt(2) * (-c1+c3-c5-c7) */
                tmp0 += z1 * 6967i64; /* sqrt(2) * ( c1+c3+c5+c7) */
                z1 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 3i32) as isize) as c_int)
                    as JLONG;
                tmp0 += z1 * -(10426i64);
                z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
                    * *quantptr.offset((8i32 * 1i32) as isize) as c_int)
                    as JLONG;
                tmp0 += z1 * 29692i64;
                /* Final output stage */
                *wsptr.offset((DCTSIZE * 0i32) as isize) =
                    (tmp10 + tmp0 + ((1i64) << 13i32 - 2i32 + 2i32 - 1i32) >> 13i32 - 2i32 + 2i32)
                        as c_int;
                *wsptr.offset((DCTSIZE * 1i32) as isize) =
                    (tmp10 - tmp0 + ((1i64) << 13i32 - 2i32 + 2i32 - 1i32) >> 13i32 - 2i32 + 2i32)
                        as c_int
            }
        }
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
    /* Pass 2: process 2 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 2i32 {
        let mut outptr: JSAMPROW = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        /* It's not clear whether a zero row test is worthwhile here ... */
        if *wsptr.offset(1) == 0i32
            && *wsptr.offset(3) == 0i32
            && *wsptr.offset(5) == 0i32
            && *wsptr.offset(7) == 0i32
        {
            /* AC terms all zero */
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0) as JLONG + ((1i64) << 2i32 + 3i32 - 1i32) >> 2i32 + 3i32)
                    as c_int
                    & RANGE_MASK) as isize,
            ); /* advance pointer to next row */
            *outptr.offset(0) = dcval_0;
            *outptr.offset(1) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize)
        } else {
            /* Even part */
            tmp10 = ((*wsptr.offset(0) as c_ulong) << 13i32 + 2i32) as JLONG;
            /* Odd part */
            tmp0 = *wsptr.offset(7) as JLONG * -(5906i64)
                + *wsptr.offset(5) as JLONG * 6967i64
                + *wsptr.offset(3) as JLONG * -(10426i64)
                + *wsptr.offset(1) as JLONG * 29692i64; /* sqrt(2) * ( c1+c3+c5+c7) */
            /* Final output stage */
            *outptr.offset(0) = *range_limit.offset(
                ((tmp10 + tmp0 + ((1i64) << 13i32 + 2i32 + 3i32 + 2i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32 + 2i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1) = *range_limit.offset(
                ((tmp10 - tmp0 + ((1i64) << 13i32 + 2i32 + 3i32 + 2i32 - 1i32)
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
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    /* We hardly need an inverse DCT routine for this: just take the
     * average pixel value, which is one-eighth of the DC coefficient.
     */

    let mut quantptr: *mut ISLOW_MULT_TYPE = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    let mut dcval: c_int = *coef_block.offset(0) as c_int * *quantptr.offset(0) as c_int;
    dcval = (dcval as JLONG + ((1i64) << 3i32 - 1i32) >> 3i32) as c_int;
    *(*output_buf.offset(0)).offset(output_col as isize) =
        *range_limit.offset((dcval & RANGE_MASK) as isize);
}
/* IDCT_SCALING_SUPPORTED */
