pub use crate::jdct_h::{ISLOW_MULT_TYPE, ONE, RANGE_MASK};
pub use crate::jmorecfg_h::{
    boolean, CENTERJSAMPLE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAXJSAMPLE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jpeg_color_deconverter, jpeg_color_quantizer, jpeg_d_coef_controller,
    jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master, jpeg_entropy_decoder,
    jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader, jpeg_upsampler, JBUF_CRANK_DEST,
    JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_common_struct, jpeg_component_info,
    jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_parser_method, jpeg_marker_struct,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_source_mgr,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
    JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
    JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
    JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED,
    JHUFF_TBL, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
    J_DITHER_MODE,
};
pub use crate::stddef_h::size_t;
use libc::{self, c_double, c_int, c_long, c_ulong};
/*
 * jidctint.c
 *
 * This file was part of the Independent JPEG Group's software.
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modification developed 2002-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a slow-but-accurate integer implementation of the
 * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
 * must also perform dequantization of the input coefficients.
 *
 * A 2-D IDCT can be done by 1-D IDCT on each column followed by 1-D IDCT
 * on each row (or vice versa, but it's more convenient to emit a row at
 * a time).  Direct algorithms are also available, but they are much more
 * complex and seem not to be any faster when reduced to code.
 *
 * This implementation is based on an algorithm described in
 *   C. Loeffler, A. Ligtenberg and G. Moschytz, "Practical Fast 1-D DCT
 *   Algorithms with 11 Multiplications", Proc. Int'l. Conf. on Acoustics,
 *   Speech, and Signal Processing 1989 (ICASSP '89), pp. 988-991.
 * The primary algorithm described there uses 11 multiplies and 29 adds.
 * We use their alternate method with 12 multiplies and 32 adds.
 * The advantage of this method is that no data path contains more than one
 * multiplication; this allows a very simple and accurate implementation in
 * scaled fixed-point arithmetic, with a minimal number of shifts.
 *
 * We also provide IDCT routines with various output sample block sizes for
 * direct resolution reduction or enlargement without additional resampling:
 * NxN (N=1...16) pixels for one 8x8 input DCT block.
 *
 * For N<8 we simply take the corresponding low-frequency coefficients of
 * the 8x8 input DCT block and apply an NxN point IDCT on the sub-block
 * to yield the downscaled outputs.
 * This can be seen as direct low-pass downsampling from the DCT domain
 * point of view rather than the usual spatial domain point of view,
 * yielding significant computational savings and results at least
 * as good as common bilinear (averaging) spatial downsampling.
 *
 * For N>8 we apply a partial NxN IDCT on the 8 input coefficients as
 * lower frequencies and higher frequencies assumed to be zero.
 * It turns out that the computational effort is similar to the 8x8 IDCT
 * regarding the output size.
 * Furthermore, the scaling and descaling is the same for all IDCT sizes.
 *
 * CAUTION: We rely on the FIX() macro except for the N=1,2,4,8 cases
 * since there would be too many additional constants to pre-calculate.
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/*
 * The poop on this scaling stuff is as follows:
 *
 * Each 1-D IDCT step produces outputs which are a factor of sqrt(N)
 * larger than the true IDCT outputs.  The final outputs are therefore
 * a factor of N larger than desired; since N=8 this can be cured by
 * a simple right shift at the end of the algorithm.  The advantage of
 * this arrangement is that we save two multiplications per 1-D IDCT,
 * because the y0 and y4 inputs need not be divided by sqrt(N).
 *
 * We have to do addition and subtraction of the integer inputs, which
 * is no problem, and multiplication by fractional constants, which is
 * a problem to do in integer arithmetic.  We multiply all the constants
 * by CONST_SCALE and convert them to integer constants (thus retaining
 * CONST_BITS bits of precision in the constants).  After doing a
 * multiplication we have to divide the product by CONST_SCALE, with proper
 * rounding, to produce the correct output.  This division can be done
 * cheaply as a right shift of CONST_BITS bits.  We postpone shifting
 * as long as possible so that partial sums can be added together with
 * full fractional precision.
 *
 * The outputs of the first pass are scaled up by PASS1_BITS bits so that
 * they are represented to better-than-integral precision.  These outputs
 * require BITS_IN_JSAMPLE + PASS1_BITS + 3 bits; this fits in a 16-bit word
 * with the recommended scaling.  (To scale up 12-bit sample data further, an
 * intermediate JLONG array would be needed.)
 *
 * To avoid overflow of the 32-bit intermediate results in pass 2, we must
 * have BITS_IN_JSAMPLE + CONST_BITS + PASS1_BITS <= 26.  Error analysis
 * shows that the values given below are the most effective.
 */
pub const CONST_BITS: c_int = 13i32;
pub const PASS1_BITS: c_int = 2i32;
/* Some C compilers fail to reduce "FIX(constant)" at compile time, thus
 * causing a lot of useless floating-point operations at run time.
 * To get around this we use the following pre-calculated constants.
 * If you change CONST_BITS you may want to add appropriate values.
 * (With a reasonable C compiler, you can just rely on the FIX() macro...)
 */
/* FIX(0.298631336) */
/* FIX(0.390180644) */
/* FIX(0.541196100) */
/* FIX(0.765366865) */
/* FIX(0.899976223) */
/* FIX(1.175875602) */
/* FIX(1.501321110) */
/* FIX(1.847759065) */
/* FIX(1.961570560) */
/* FIX(2.053119869) */
/* FIX(2.562915447) */
/* FIX(3.072711026) */
/* Multiply an JLONG variable by an JLONG constant to yield an JLONG result.
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
 * Perform dequantization and inverse DCT on one block of coefficients.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_islow(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp1: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp3: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut z5: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 64] = [0; 64];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = DCTSIZE;
    while ctr > 0i32 {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */
        if *inptr.offset((DCTSIZE * 1i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 2i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 3i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 4i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 5i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 6i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 7i32) as isize) as c_int == 0i32
        {
            /* AC terms all zero */
            let mut dcval: c_int = (((*inptr.offset((8i32 * 0i32) as isize) as c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                as c_ulong)
                << 2i32) as JLONG as c_int;
            *wsptr.offset((DCTSIZE * 0i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 1i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 2i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 3i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 4i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 5i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 6i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 7i32) as isize) = dcval;
            inptr = inptr.offset(1isize);
            quantptr = quantptr.offset(1isize);
            wsptr = wsptr.offset(1isize)
        } else {
            z2 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
                * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
            z3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
                * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
            z1 = (z2 + z3) * 4433i32 as JLONG;
            tmp2 = z1 + z3 * -(15137i32 as JLONG);
            tmp3 = z1 + z2 * 6270i32 as JLONG;
            z2 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
            z3 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
                * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
            tmp0 = (((z2 + z3) as c_ulong) << 13i32) as JLONG;
            tmp1 = (((z2 - z3) as c_ulong) << 13i32) as JLONG;
            tmp10 = tmp0 + tmp3;
            tmp13 = tmp0 - tmp3;
            tmp11 = tmp1 + tmp2;
            tmp12 = tmp1 - tmp2;
            tmp0 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
                * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
            tmp1 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
                * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
            tmp2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
                * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
            tmp3 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
                * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
            z1 = tmp0 + tmp3;
            z2 = tmp1 + tmp2;
            z3 = tmp0 + tmp2;
            z4 = tmp1 + tmp3;
            z5 = (z3 + z4) * 9633i32 as JLONG;
            tmp0 = tmp0 * 2446i32 as JLONG;
            tmp1 = tmp1 * 16819i32 as JLONG;
            tmp2 = tmp2 * 25172i32 as JLONG;
            tmp3 = tmp3 * 12299i32 as JLONG;
            z1 = z1 * -(7373i32 as JLONG);
            z2 = z2 * -(20995i32 as JLONG);
            z3 = z3 * -(16069i32 as JLONG);
            z4 = z4 * -(3196i32 as JLONG);
            z3 += z5;
            z4 += z5;
            tmp0 += z1 + z3;
            tmp1 += z2 + z4;
            tmp2 += z2 + z3;
            tmp3 += z1 + z4;
            *wsptr.offset((DCTSIZE * 0i32) as isize) =
                (tmp10 + tmp3 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            *wsptr.offset((DCTSIZE * 7i32) as isize) =
                (tmp10 - tmp3 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            *wsptr.offset((DCTSIZE * 1i32) as isize) =
                (tmp11 + tmp2 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            *wsptr.offset((DCTSIZE * 6i32) as isize) =
                (tmp11 - tmp2 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            *wsptr.offset((DCTSIZE * 2i32) as isize) =
                (tmp12 + tmp1 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            *wsptr.offset((DCTSIZE * 5i32) as isize) =
                (tmp12 - tmp1 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            *wsptr.offset((DCTSIZE * 3i32) as isize) =
                (tmp13 + tmp0 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            *wsptr.offset((DCTSIZE * 4i32) as isize) =
                (tmp13 - tmp0 + ((1i32 as JLONG) << 13i32 - 2i32 - 1i32) >> 13i32 - 2i32) as c_int;
            inptr = inptr.offset(1isize);
            quantptr = quantptr.offset(1isize);
            wsptr = wsptr.offset(1isize)
        }
        ctr -= 1
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < DCTSIZE {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* Rows of zeroes can be exploited in the same way as we did with columns.
         * However, the column calculation has created many nonzero AC terms, so
         * the simplification applies less often (typically 5% to 10% of the time).
         * On machines with very fast multiplication, it's possible that the
         * test takes more time than it's worth.  In that case this section
         * may be commented out.
         */
        if *wsptr.offset(1isize) == 0i32
            && *wsptr.offset(2isize) == 0i32
            && *wsptr.offset(3isize) == 0i32
            && *wsptr.offset(4isize) == 0i32
            && *wsptr.offset(5isize) == 0i32
            && *wsptr.offset(6isize) == 0i32
            && *wsptr.offset(7isize) == 0i32
        {
            /* AC terms all zero */
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0isize) as JLONG + ((1i32 as JLONG) << 2i32 + 3i32 - 1i32)
                    >> 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(0isize) = dcval_0;
            *outptr.offset(1isize) = dcval_0;
            *outptr.offset(2isize) = dcval_0;
            *outptr.offset(3isize) = dcval_0;
            *outptr.offset(4isize) = dcval_0;
            *outptr.offset(5isize) = dcval_0;
            *outptr.offset(6isize) = dcval_0;
            *outptr.offset(7isize) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize)
        } else {
            z2 = *wsptr.offset(2isize) as JLONG;
            z3 = *wsptr.offset(6isize) as JLONG;
            z1 = (z2 + z3) * 4433i32 as JLONG;
            tmp2 = z1 + z3 * -(15137i32 as JLONG);
            tmp3 = z1 + z2 * 6270i32 as JLONG;
            tmp0 = (((*wsptr.offset(0isize) as JLONG + *wsptr.offset(4isize) as JLONG) as c_ulong)
                << 13i32) as JLONG;
            tmp1 = (((*wsptr.offset(0isize) as JLONG - *wsptr.offset(4isize) as JLONG) as c_ulong)
                << 13i32) as JLONG;
            tmp10 = tmp0 + tmp3;
            tmp13 = tmp0 - tmp3;
            tmp11 = tmp1 + tmp2;
            tmp12 = tmp1 - tmp2;
            tmp0 = *wsptr.offset(7isize) as JLONG;
            tmp1 = *wsptr.offset(5isize) as JLONG;
            tmp2 = *wsptr.offset(3isize) as JLONG;
            tmp3 = *wsptr.offset(1isize) as JLONG;
            z1 = tmp0 + tmp3;
            z2 = tmp1 + tmp2;
            z3 = tmp0 + tmp2;
            z4 = tmp1 + tmp3;
            z5 = (z3 + z4) * 9633i32 as JLONG;
            tmp0 = tmp0 * 2446i32 as JLONG;
            tmp1 = tmp1 * 16819i32 as JLONG;
            tmp2 = tmp2 * 25172i32 as JLONG;
            tmp3 = tmp3 * 12299i32 as JLONG;
            z1 = z1 * -(7373i32 as JLONG);
            z2 = z2 * -(20995i32 as JLONG);
            z3 = z3 * -(16069i32 as JLONG);
            z4 = z4 * -(3196i32 as JLONG);
            z3 += z5;
            z4 += z5;
            tmp0 += z1 + z3;
            tmp1 += z2 + z4;
            tmp2 += z2 + z3;
            tmp3 += z1 + z4;
            *outptr.offset(0isize) = *range_limit.offset(
                ((tmp10 + tmp3 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(7isize) = *range_limit.offset(
                ((tmp10 - tmp3 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1isize) = *range_limit.offset(
                ((tmp11 + tmp2 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(6isize) = *range_limit.offset(
                ((tmp11 - tmp2 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(2isize) = *range_limit.offset(
                ((tmp12 + tmp1 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(5isize) = *range_limit.offset(
                ((tmp12 - tmp1 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(3isize) = *range_limit.offset(
                ((tmp13 + tmp0 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(4isize) = *range_limit.offset(
                ((tmp13 - tmp0 + ((1i32 as JLONG) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as c_int
                    & RANGE_MASK) as isize,
            );
            wsptr = wsptr.offset(DCTSIZE as isize)
        }
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 7x7 output block.
 *
 * Optimized algorithm with 12 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/14).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_7x7(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp1: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 49] = [0; 49];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 7i32 {
        tmp13 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        tmp13 = ((tmp13 as c_ulong) << 13i32) as JLONG;
        tmp13 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        tmp10 =
            (z2 - z3) * (0.881747734f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 - z2) * (0.314692123f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp10 + tmp12 + tmp13
            - z2 * (1.841218003f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = tmp0 * (1.274162392f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + tmp13;
        tmp10 +=
            tmp0 - z3 * (0.077722536f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 +=
            tmp0 - z1 * (2.470602249f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 += z2 * (1.414213562f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        tmp1 =
            (z1 + z2) * (0.935414347f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 =
            (z1 - z2) * (0.170262339f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (z2 + z3)
            * -((1.378756276f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp1 += tmp2;
        z2 =
            (z1 + z3) * (0.613604268f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 += z2;
        tmp2 +=
            z2 + z3 * (1.870828693f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *wsptr.offset((7i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((7i32 * 6i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((7i32 * 1i32) as isize) = (tmp11 + tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((7i32 * 5i32) as isize) = (tmp11 - tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((7i32 * 2i32) as isize) = (tmp12 + tmp2 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((7i32 * 4i32) as isize) = (tmp12 - tmp2 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((7i32 * 3i32) as isize) = (tmp13 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 7i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        tmp13 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        tmp13 = ((tmp13 as c_ulong) << 13i32) as JLONG;
        z1 = *wsptr.offset(2isize) as JLONG;
        z2 = *wsptr.offset(4isize) as JLONG;
        z3 = *wsptr.offset(6isize) as JLONG;
        tmp10 =
            (z2 - z3) * (0.881747734f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 - z2) * (0.314692123f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp10 + tmp12 + tmp13
            - z2 * (1.841218003f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = tmp0 * (1.274162392f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + tmp13;
        tmp10 +=
            tmp0 - z3 * (0.077722536f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 +=
            tmp0 - z1 * (2.470602249f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 += z2 * (1.414213562f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        tmp1 =
            (z1 + z2) * (0.935414347f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 =
            (z1 - z2) * (0.170262339f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (z2 + z3)
            * -((1.378756276f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp1 += tmp2;
        z2 =
            (z1 + z3) * (0.613604268f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 += z2;
        tmp2 +=
            z2 + z3 * (1.870828693f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp12 + tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp12 - tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) =
            *range_limit.offset(((tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(7isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 6x6 output block.
 *
 * Optimized algorithm with 3 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/12).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_6x6(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp1: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 36] = [0; 36];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 6i32 {
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        tmp0 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        tmp2 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        tmp10 = tmp2 * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10 >> 13i32 - 2i32;
        tmp10 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        tmp0 = tmp10 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        tmp1 =
            (z1 + z3) * (0.366025404f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp1 + (((z1 + z2) as c_ulong) << 13i32) as JLONG;
        tmp2 = tmp1 + (((z3 - z2) as c_ulong) << 13i32) as JLONG;
        tmp1 = (((z1 - z2 - z3) as c_ulong) << 2i32) as JLONG;
        *wsptr.offset((6i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((6i32 * 5i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((6i32 * 1i32) as isize) = (tmp11 + tmp1) as c_int;
        *wsptr.offset((6i32 * 4i32) as isize) = (tmp11 - tmp1) as c_int;
        *wsptr.offset((6i32 * 2i32) as isize) = (tmp12 + tmp2 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((6i32 * 3i32) as isize) = (tmp12 - tmp2 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 6i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        tmp0 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        tmp2 = *wsptr.offset(4isize) as JLONG;
        tmp10 = tmp2 * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10;
        tmp10 = *wsptr.offset(2isize) as JLONG;
        tmp0 = tmp10 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        tmp1 =
            (z1 + z3) * (0.366025404f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp1 + (((z1 + z2) as c_ulong) << 13i32) as JLONG;
        tmp2 = tmp1 + (((z3 - z2) as c_ulong) << 13i32) as JLONG;
        tmp1 = (((z1 - z2 - z3) as c_ulong) << 13i32) as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp12 + tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp12 - tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(6isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 5x5 output block.
 *
 * Optimized algorithm with 5 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/10).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_5x5(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp1: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 25] = [0; 25];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 5i32 {
        tmp12 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        tmp12 = ((tmp12 as c_ulong) << 13i32) as JLONG;
        tmp12 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        tmp0 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        tmp1 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z1 = (tmp0 + tmp1)
            * (0.790569415f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = (tmp0 - tmp1)
            * (0.353553391f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= ((z2 as c_ulong) << 2i32) as JLONG;
        z2 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z1 =
            (z2 + z3) * (0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 =
            z1 + z2 * (0.513743148f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 =
            z1 - z3 * (2.176250899f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *wsptr.offset((5i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((5i32 * 4i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((5i32 * 1i32) as isize) = (tmp11 + tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((5i32 * 3i32) as isize) = (tmp11 - tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((5i32 * 2i32) as isize) = (tmp12 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 5i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        tmp12 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        tmp12 = ((tmp12 as c_ulong) << 13i32) as JLONG;
        tmp0 = *wsptr.offset(2isize) as JLONG;
        tmp1 = *wsptr.offset(4isize) as JLONG;
        z1 = (tmp0 + tmp1)
            * (0.790569415f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = (tmp0 - tmp1)
            * (0.353553391f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= ((z2 as c_ulong) << 2i32) as JLONG;
        z2 = *wsptr.offset(1isize) as JLONG;
        z3 = *wsptr.offset(3isize) as JLONG;
        z1 =
            (z2 + z3) * (0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 =
            z1 + z2 * (0.513743148f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 =
            z1 - z3 * (2.176250899f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) =
            *range_limit.offset(((tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(5isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 3x3 output block.
 *
 * Optimized algorithm with 2 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/6).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_3x3(
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
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 9] = [0; 9];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 3i32 {
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        tmp0 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        tmp2 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        tmp12 = tmp2 * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        tmp12 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        tmp0 = tmp12 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *wsptr.offset((3i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((3i32 * 2i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((3i32 * 1i32) as isize) = (tmp2 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 3i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        tmp0 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        tmp2 = *wsptr.offset(2isize) as JLONG;
        tmp12 = tmp2 * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        tmp12 = *wsptr.offset(1isize) as JLONG;
        tmp0 = tmp12 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) =
            *range_limit.offset(((tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(3isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 9x9 output block.
 *
 * Optimized algorithm with 10 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/18).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_9x9(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp1: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp3: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp14: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 72] = [0; 72];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        tmp0 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        tmp3 = z3 * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;
        tmp0 =
            (z1 - z2) * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;
        tmp0 =
            (z1 + z2) * (1.328926049f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 = z1 * (1.083350441f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 = z2 * (0.245575608f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        z2 = z2 * -((1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp2 =
            (z1 + z3) * (0.909038955f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 =
            (z1 + z4) * (0.483689525f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 =
            (z3 - z4) * (1.392728481f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = (z1 - z3 - z4)
            * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp11 + tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp11 - tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp12 + tmp2 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp12 - tmp2 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp13 + tmp3 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp13 - tmp3 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp14 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 9i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        tmp0 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        z1 = *wsptr.offset(2isize) as JLONG;
        z2 = *wsptr.offset(4isize) as JLONG;
        z3 = *wsptr.offset(6isize) as JLONG;
        tmp3 = z3 * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;
        tmp0 =
            (z1 - z2) * (0.707106781f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;
        tmp0 =
            (z1 + z2) * (1.328926049f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 = z1 * (1.083350441f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 = z2 * (0.245575608f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        z2 = z2 * -((1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp2 =
            (z1 + z3) * (0.909038955f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 =
            (z1 + z4) * (0.483689525f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 =
            (z3 - z4) * (1.392728481f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = (z1 - z3 - z4)
            * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) = *range_limit
            .offset(((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp12 + tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp12 - tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp13 + tmp3 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp13 - tmp3 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) =
            *range_limit.offset(((tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 10x10 output block.
 *
 * Optimized algorithm with 12 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/20).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_10x10(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp14: JLONG = 0;
    let mut tmp20: JLONG = 0;
    let mut tmp21: JLONG = 0;
    let mut tmp22: JLONG = 0;
    let mut tmp23: JLONG = 0;
    let mut tmp24: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut z5: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 80] = [0; 80];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        z3 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        z3 = ((z3 as c_ulong) << 13i32) as JLONG;
        z3 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z4 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z1 = z4 * (1.144122806f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = z4 * (0.437016024f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;
        tmp22 = z3 - (((z1 - z2) as c_ulong) << 1i32) as JLONG >> 13i32 - 2i32;
        z2 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        z1 =
            (z2 + z3) * (0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            z1 + z2 * (0.513743148f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 =
            z1 - z3 * (2.176250899f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        tmp11 = z2 + z4;
        tmp13 = z2 - z4;
        tmp12 = tmp13 * (0.309016994f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z5 = ((z3 as c_ulong) << 13i32) as JLONG;
        z2 = tmp11 * (0.951056516f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z5 + tmp12;
        tmp10 = z1 * (1.396802247f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z2
            + z4;
        tmp14 = z1 * (0.221231742f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2
            + z4;
        z2 = tmp11 * (0.587785252f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z5 - tmp12 - ((tmp13 as c_ulong) << 13i32 - 1i32) as JLONG;
        tmp12 = (((z1 - tmp13 - z3) as c_ulong) << 2i32) as JLONG;
        tmp11 = z1 * (1.260073511f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2
            - z4;
        tmp13 = z1 * (0.642039522f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2
            + z4;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp22 - tmp12) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 10i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        z3 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        z3 = ((z3 as c_ulong) << 13i32) as JLONG;
        z4 = *wsptr.offset(4isize) as JLONG;
        z1 = z4 * (1.144122806f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = z4 * (0.437016024f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;
        tmp22 = z3 - (((z1 - z2) as c_ulong) << 1i32) as JLONG;
        z2 = *wsptr.offset(2isize) as JLONG;
        z3 = *wsptr.offset(6isize) as JLONG;
        z1 =
            (z2 + z3) * (0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            z1 + z2 * (0.513743148f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 =
            z1 - z3 * (2.176250899f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        z3 = ((z3 as c_ulong) << 13i32) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        tmp11 = z2 + z4;
        tmp13 = z2 - z4;
        tmp12 = tmp13 * (0.309016994f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = tmp11 * (0.951056516f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z3 + tmp12;
        tmp10 = z1 * (1.396802247f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z2
            + z4;
        tmp14 = z1 * (0.221231742f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2
            + z4;
        z2 = tmp11 * (0.587785252f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z3 - tmp12 - ((tmp13 as c_ulong) << 13i32 - 1i32) as JLONG;
        tmp12 = (((z1 - tmp13) as c_ulong) << 13i32) as JLONG - z3;
        tmp11 = z1 * (1.260073511f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2
            - z4;
        tmp13 = z1 * (0.642039522f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2
            + z4;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(9isize) = *range_limit
            .offset(((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) = *range_limit
            .offset(((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 11x11 output block.
 *
 * Optimized algorithm with 24 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/22).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_11x11(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp14: JLONG = 0;
    let mut tmp20: JLONG = 0;
    let mut tmp21: JLONG = 0;
    let mut tmp22: JLONG = 0;
    let mut tmp23: JLONG = 0;
    let mut tmp24: JLONG = 0;
    let mut tmp25: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 88] = [0; 88];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        tmp10 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        tmp10 = ((tmp10 as c_ulong) << 13i32) as JLONG;
        tmp10 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        tmp20 =
            (z2 - z3) * (2.546640132f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp23 =
            (z2 - z1) * (0.430815045f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z1 + z3;
        tmp24 = z4 * -((1.155664402f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        z4 -= z2;
        tmp25 = tmp10
            + z4 * (1.356927976f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp21 = tmp20 + tmp23 + tmp25
            - z2 * (1.821790775f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 += tmp25
            + z3 * (2.115825087f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp23 += tmp25
            - z1 * (1.513598477f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp24 += tmp25;
        tmp22 = tmp24
            - z3 * (0.788749120f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp24 += z2 * (1.944413522f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z1 * (1.390975730f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp25 = tmp10
            - z4 * (1.414213562f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        tmp11 = z1 + z2;
        tmp14 = (tmp11 + z3 + z4)
            * (0.398430003f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp11 * (0.887983902f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 + z3) * (0.670361295f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp14
            + (z1 + z4)
                * (0.366151574f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (0.923107866f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = tmp14
            - (z2 + z3)
                * (1.163011579f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 +=
            z1 + z2 * (2.073276588f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 +=
            z1 - z3 * (1.192193623f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = (z2 + z4)
            * -((1.798248910f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp11 += z1;
        tmp13 +=
            z1 + z4 * (2.102458632f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 += z2
            * -((1.467221301f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            + z3 * (1.001388905f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z4 * (1.684843907f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 11i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        tmp10 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        tmp10 = ((tmp10 as c_ulong) << 13i32) as JLONG;
        z1 = *wsptr.offset(2isize) as JLONG;
        z2 = *wsptr.offset(4isize) as JLONG;
        z3 = *wsptr.offset(6isize) as JLONG;
        tmp20 =
            (z2 - z3) * (2.546640132f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp23 =
            (z2 - z1) * (0.430815045f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z1 + z3;
        tmp24 = z4 * -((1.155664402f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        z4 -= z2;
        tmp25 = tmp10
            + z4 * (1.356927976f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp21 = tmp20 + tmp23 + tmp25
            - z2 * (1.821790775f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 += tmp25
            + z3 * (2.115825087f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp23 += tmp25
            - z1 * (1.513598477f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp24 += tmp25;
        tmp22 = tmp24
            - z3 * (0.788749120f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp24 += z2 * (1.944413522f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z1 * (1.390975730f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp25 = tmp10
            - z4 * (1.414213562f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        tmp11 = z1 + z2;
        tmp14 = (tmp11 + z3 + z4)
            * (0.398430003f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp11 * (0.887983902f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 + z3) * (0.670361295f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp14
            + (z1 + z4)
                * (0.366151574f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (0.923107866f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = tmp14
            - (z2 + z3)
                * (1.163011579f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 +=
            z1 + z2 * (2.073276588f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 +=
            z1 - z3 * (1.192193623f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = (z2 + z4)
            * -((1.798248910f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp11 += z1;
        tmp13 +=
            z1 + z4 * (2.102458632f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 += z2
            * -((1.467221301f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            + z3 * (1.001388905f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z4 * (1.684843907f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(10isize) = *range_limit
            .offset(((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(9isize) = *range_limit
            .offset(((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) = *range_limit
            .offset(((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) =
            *range_limit.offset(((tmp25 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 12x12 output block.
 *
 * Optimized algorithm with 15 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/24).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_12x12(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp14: JLONG = 0;
    let mut tmp15: JLONG = 0;
    let mut tmp20: JLONG = 0;
    let mut tmp21: JLONG = 0;
    let mut tmp22: JLONG = 0;
    let mut tmp23: JLONG = 0;
    let mut tmp24: JLONG = 0;
    let mut tmp25: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 96] = [0; 96];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        z3 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        z3 = ((z3 as c_ulong) << 13i32) as JLONG;
        z3 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z4 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z4 = z4 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z4 = z1 * (1.366025404f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z2 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        z2 = ((z2 as c_ulong) << 13i32) as JLONG;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        tmp11 = z2 * (1.306562965f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = z2 * -(4433i32 as JLONG);
        tmp10 = z1 + z3;
        tmp15 = (tmp10 + z4)
            * (0.860918669f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 = tmp15
            + tmp10 * (0.261052384f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = (z3 + z4)
            * -((1.045510580f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 += tmp14
            - z1 * (0.676326758f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z4 * (1.982889723f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433i32 as JLONG;
        tmp11 = z3 + z1 * 6270i32 as JLONG;
        tmp14 = z3 - z2 * 15137i32 as JLONG;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 12i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        z3 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        z3 = ((z3 as c_ulong) << 13i32) as JLONG;
        z4 = *wsptr.offset(4isize) as JLONG;
        z4 = z4 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = *wsptr.offset(2isize) as JLONG;
        z4 = z1 * (1.366025404f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z2 = *wsptr.offset(6isize) as JLONG;
        z2 = ((z2 as c_ulong) << 13i32) as JLONG;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        tmp11 = z2 * (1.306562965f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = z2 * -(4433i32 as JLONG);
        tmp10 = z1 + z3;
        tmp15 = (tmp10 + z4)
            * (0.860918669f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 = tmp15
            + tmp10 * (0.261052384f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = (z3 + z4)
            * -((1.045510580f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 += tmp14
            - z1 * (0.676326758f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z4 * (1.982889723f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433i32 as JLONG;
        tmp11 = z3 + z1 * 6270i32 as JLONG;
        tmp14 = z3 - z2 * 15137i32 as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(11isize) = *range_limit
            .offset(((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(10isize) = *range_limit
            .offset(((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(9isize) = *range_limit
            .offset(((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) = *range_limit
            .offset(((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 13x13 output block.
 *
 * Optimized algorithm with 29 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/26).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_13x13(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp14: JLONG = 0;
    let mut tmp15: JLONG = 0;
    let mut tmp20: JLONG = 0;
    let mut tmp21: JLONG = 0;
    let mut tmp22: JLONG = 0;
    let mut tmp23: JLONG = 0;
    let mut tmp24: JLONG = 0;
    let mut tmp25: JLONG = 0;
    let mut tmp26: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 104] = [0; 104];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        z1 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z1 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z2 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        tmp12 = tmp10 * (1.155388986f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp11 * (0.096834934f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z1;
        tmp20 = z2 * (1.373119086f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + tmp12
            + tmp13;
        tmp22 = z2 * (0.501487041f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - tmp12
            + tmp13;
        tmp12 = tmp10 * (0.316450131f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp11 * (0.486914739f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z1;
        tmp21 = z2 * (1.058554052f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - tmp12
            + tmp13;
        tmp25 = z2 * -((1.252223920f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            + tmp12
            + tmp13;
        tmp12 = tmp10 * (0.435816023f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp11 * (0.937303064f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z1;
        tmp23 = z2 * -((0.170464608f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            - tmp12
            - tmp13;
        tmp24 = z2 * -((0.803364869f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            + tmp12
            - tmp13;
        tmp26 = (tmp11 - z2)
            * (1.414213562f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z1;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        tmp11 =
            (z1 + z2) * (1.322312651f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 + z3) * (1.163874945f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 = z1 + z4;
        tmp13 = tmp15 * (0.937797057f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (2.020082300f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = (z2 + z3)
            * -((0.338443458f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp11 += tmp14
            + z2 * (0.837223564f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 += tmp14
            - z3 * (1.572116027f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = (z2 + z4)
            * -((1.163874945f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp11 += tmp14;
        tmp13 += tmp14
            + z4 * (2.205608352f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = (z3 + z4)
            * -((0.657217813f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = tmp15 * (0.338443458f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = tmp15
            + z1 * (0.318774355f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2 * (0.466105296f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 =
            (z3 - z2) * (0.937797057f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 += z1;
        tmp15 += z1
            + z3 * (0.384515595f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z4 * (1.742345811f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 13i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        z1 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z2 = *wsptr.offset(2isize) as JLONG;
        z3 = *wsptr.offset(4isize) as JLONG;
        z4 = *wsptr.offset(6isize) as JLONG;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        tmp12 = tmp10 * (1.155388986f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp11 * (0.096834934f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z1;
        tmp20 = z2 * (1.373119086f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + tmp12
            + tmp13;
        tmp22 = z2 * (0.501487041f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - tmp12
            + tmp13;
        tmp12 = tmp10 * (0.316450131f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp11 * (0.486914739f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z1;
        tmp21 = z2 * (1.058554052f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - tmp12
            + tmp13;
        tmp25 = z2 * -((1.252223920f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            + tmp12
            + tmp13;
        tmp12 = tmp10 * (0.435816023f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp11 * (0.937303064f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z1;
        tmp23 = z2 * -((0.170464608f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            - tmp12
            - tmp13;
        tmp24 = z2 * -((0.803364869f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            + tmp12
            - tmp13;
        tmp26 = (tmp11 - z2)
            * (1.414213562f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z1;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        tmp11 =
            (z1 + z2) * (1.322312651f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 + z3) * (1.163874945f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 = z1 + z4;
        tmp13 = tmp15 * (0.937797057f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (2.020082300f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = (z2 + z3)
            * -((0.338443458f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp11 += tmp14
            + z2 * (0.837223564f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 += tmp14
            - z3 * (1.572116027f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = (z2 + z4)
            * -((1.163874945f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp11 += tmp14;
        tmp13 += tmp14
            + z4 * (2.205608352f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = (z3 + z4)
            * -((0.657217813f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = tmp15 * (0.338443458f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = tmp15
            + z1 * (0.318774355f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2 * (0.466105296f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 =
            (z3 - z2) * (0.937797057f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 += z1;
        tmp15 += z1
            + z3 * (0.384515595f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z4 * (1.742345811f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(12isize) = *range_limit
            .offset(((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(11isize) = *range_limit
            .offset(((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(10isize) = *range_limit
            .offset(((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(9isize) = *range_limit
            .offset(((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) = *range_limit
            .offset(((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) =
            *range_limit.offset(((tmp26 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 14x14 output block.
 *
 * Optimized algorithm with 20 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/28).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_14x14(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp14: JLONG = 0;
    let mut tmp15: JLONG = 0;
    let mut tmp16: JLONG = 0;
    let mut tmp20: JLONG = 0;
    let mut tmp21: JLONG = 0;
    let mut tmp22: JLONG = 0;
    let mut tmp23: JLONG = 0;
    let mut tmp24: JLONG = 0;
    let mut tmp25: JLONG = 0;
    let mut tmp26: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 112] = [0; 112];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        z1 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z1 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z4 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z2 = z4 * (1.274162392f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z3 = z4 * (0.314692123f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z4 * (0.881747734f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;
        tmp23 = z1 - (((z2 + z3 - z4) as c_ulong) << 1i32) as JLONG >> 13i32 - 2i32;
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        z3 =
            (z1 + z2) * (1.105676686f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 =
            z3 + z1 * (0.273079590f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 =
            z3 - z2 * (1.719280954f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 = z1 * (0.613604268f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2 * (1.378756276f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        tmp13 = ((z4 as c_ulong) << 13i32) as JLONG;
        tmp14 = z1 + z3;
        tmp11 =
            (z1 + z2) * (1.334852607f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 = tmp14 * (1.197448846f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (1.126980169f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = tmp14 * (0.752406978f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp16 = tmp14
            - z1 * (1.061150426f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 -= z2;
        tmp15 = z1 * (0.467085129f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - tmp13;
        tmp16 += tmp15;
        z1 += z4;
        z4 = (z2 + z3)
            * -((0.158341681f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            - tmp13;
        tmp11 +=
            z4 - z2 * (0.424103948f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 +=
            z4 - z3 * (2.373959773f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 =
            (z3 - z2) * (1.405321284f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 += z4 + tmp13
            - z3 * (1.6906431334f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 +=
            z4 + z2 * (0.674957567f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = (((z1 - z3) as c_ulong) << 2i32) as JLONG;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 13i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13) as c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp23 - tmp13) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 + tmp16 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp26 - tmp16 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 14i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        z1 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z4 = *wsptr.offset(4isize) as JLONG;
        z2 = z4 * (1.274162392f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z3 = z4 * (0.314692123f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = z4 * (0.881747734f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;
        tmp23 = z1 - (((z2 + z3 - z4) as c_ulong) << 1i32) as JLONG;
        z1 = *wsptr.offset(2isize) as JLONG;
        z2 = *wsptr.offset(6isize) as JLONG;
        z3 =
            (z1 + z2) * (1.105676686f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 =
            z3 + z1 * (0.273079590f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 =
            z3 - z2 * (1.719280954f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 = z1 * (0.613604268f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z2 * (1.378756276f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        z4 = ((z4 as c_ulong) << 13i32) as JLONG;
        tmp14 = z1 + z3;
        tmp11 =
            (z1 + z2) * (1.334852607f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 = tmp14 * (1.197448846f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp11 + tmp12 + z4
            - z1 * (1.126980169f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = tmp14 * (0.752406978f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp16 = tmp14
            - z1 * (1.061150426f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 -= z2;
        tmp15 =
            z1 * (0.467085129f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG - z4;
        tmp16 += tmp15;
        tmp13 = (z2 + z3)
            * -((0.158341681f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG)
            - z4;
        tmp11 += tmp13
            - z2 * (0.424103948f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 += tmp13
            - z3 * (2.373959773f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 =
            (z3 - z2) * (1.405321284f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 += tmp13 + z4
            - z3 * (1.6906431334f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp15 += tmp13
            + z2 * (0.674957567f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = (((z1 - z3) as c_ulong) << 13i32) as JLONG + z4;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(13isize) = *range_limit
            .offset(((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(12isize) = *range_limit
            .offset(((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(11isize) = *range_limit
            .offset(((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(10isize) = *range_limit
            .offset(((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(9isize) = *range_limit
            .offset(((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp26 + tmp16 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) = *range_limit
            .offset(((tmp26 - tmp16 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 15x15 output block.
 *
 * Optimized algorithm with 22 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/30).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_15x15(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp14: JLONG = 0;
    let mut tmp15: JLONG = 0;
    let mut tmp16: JLONG = 0;
    let mut tmp20: JLONG = 0;
    let mut tmp21: JLONG = 0;
    let mut tmp22: JLONG = 0;
    let mut tmp23: JLONG = 0;
    let mut tmp24: JLONG = 0;
    let mut tmp25: JLONG = 0;
    let mut tmp26: JLONG = 0;
    let mut tmp27: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 120] = [0; 120];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        z1 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z1 += (ONE as JLONG) << CONST_BITS - PASS1_BITS - 1i32;
        z2 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        tmp10 = z4 * (0.437016024f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (1.144122806f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 = z1 - tmp10;
        tmp13 = z1 + tmp11;
        z1 -= (((tmp11 - tmp10) as c_ulong) << 1i32) as JLONG;
        z4 = z2 - z3;
        z3 += z2;
        tmp10 = z3 * (1.337628990f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (0.045680613f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = z2 * (1.439773946f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;
        tmp10 = z3 * (0.547059574f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (0.399234004f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;
        tmp10 = z3 * (0.790569415f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (0.353553391f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11;
        tmp27 = z1 - tmp11 - tmp11;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z3 = z4 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        tmp13 = z2 - z4;
        tmp15 = (z1 + tmp13)
            * (0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp15
            + z1 * (0.513743148f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = tmp15
            - tmp13 * (2.176250899f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = z2 * -((0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp15 = z2 * -((1.344997024f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        z2 = z1 - z4;
        tmp12 =
            z3 + z2 * (1.406466353f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp12
            + z4 * (2.457431844f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - tmp15;
        tmp16 = tmp12
            - z1 * (1.112434820f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + tmp13;
        tmp12 =
            z2 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG - z3;
        z2 =
            (z1 + z4) * (0.575212477f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 += z2
            + z1 * (0.475753014f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z3;
        tmp15 += z2
            - z4 * (0.869244010f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z3;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 14i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 13i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 + tmp16 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp26 - tmp16 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp27 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 15i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        z1 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        z1 = ((z1 as c_ulong) << 13i32) as JLONG;
        z2 = *wsptr.offset(2isize) as JLONG;
        z3 = *wsptr.offset(4isize) as JLONG;
        z4 = *wsptr.offset(6isize) as JLONG;
        tmp10 = z4 * (0.437016024f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (1.144122806f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 = z1 - tmp10;
        tmp13 = z1 + tmp11;
        z1 -= (((tmp11 - tmp10) as c_ulong) << 1i32) as JLONG;
        z4 = z2 - z3;
        z3 += z2;
        tmp10 = z3 * (1.337628990f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (0.045680613f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = z2 * (1.439773946f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;
        tmp10 = z3 * (0.547059574f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (0.399234004f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;
        tmp10 = z3 * (0.790569415f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = z4 * (0.353553391f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11;
        tmp27 = z1 - tmp11 - tmp11;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z4 = *wsptr.offset(5isize) as JLONG;
        z3 = z4 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        tmp13 = z2 - z4;
        tmp15 = (z1 + tmp13)
            * (0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp15
            + z1 * (0.513743148f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp14 = tmp15
            - tmp13 * (2.176250899f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = z2 * -((0.831253876f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp15 = z2 * -((1.344997024f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        z2 = z1 - z4;
        tmp12 =
            z3 + z2 * (1.406466353f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 = tmp12
            + z4 * (2.457431844f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - tmp15;
        tmp16 = tmp12
            - z1 * (1.112434820f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + tmp13;
        tmp12 =
            z2 * (1.224744871f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG - z3;
        z2 =
            (z1 + z4) * (0.575212477f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 += z2
            + z1 * (0.475753014f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            - z3;
        tmp15 += z2
            - z4 * (0.869244010f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG
            + z3;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(14isize) = *range_limit
            .offset(((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(13isize) = *range_limit
            .offset(((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(12isize) = *range_limit
            .offset(((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(11isize) = *range_limit
            .offset(((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(10isize) = *range_limit
            .offset(((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(9isize) = *range_limit
            .offset(((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp26 + tmp16 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp26 - tmp16 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) =
            *range_limit.offset(((tmp27 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 16x16 output block.
 *
 * Optimized algorithm with 28 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/32).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_16x16(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp1: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp3: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut tmp20: JLONG = 0;
    let mut tmp21: JLONG = 0;
    let mut tmp22: JLONG = 0;
    let mut tmp23: JLONG = 0;
    let mut tmp24: JLONG = 0;
    let mut tmp25: JLONG = 0;
    let mut tmp26: JLONG = 0;
    let mut tmp27: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut ISLOW_MULT_TYPE = 0 as *mut ISLOW_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 128] = [0; 128];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as JLONG;
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        tmp0 += (1i32 << CONST_BITS - PASS1_BITS - 1i32) as c_long;
        z1 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as JLONG;
        tmp1 = z1 * (1.306562965f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 = z1 * 4433i32 as JLONG;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as JLONG;
        z3 = z1 - z2;
        z4 = z3 * (0.275899379f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z3 = z3 * (1.387039845f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = z3 + z2 * 20995i32 as JLONG;
        tmp1 = z4 + z1 * 7373i32 as JLONG;
        tmp2 =
            z3 - z1 * (0.601344887f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 =
            z4 - z2 * (0.509795579f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as JLONG;
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as JLONG;
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as JLONG;
        tmp11 = z1 + z3;
        tmp1 =
            (z1 + z2) * (1.353318001f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 = tmp11 * (1.247225013f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 =
            (z1 + z4) * (1.093201867f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 =
            (z1 - z4) * (0.897167586f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp11 * (0.666655658f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 - z2) * (0.410524528f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 =
            (z2 + z3) * (0.138617169f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 +=
            z1 + z2 * (0.071888074f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 +=
            z1 - z3 * (1.125726048f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 =
            (z3 - z2) * (1.407403738f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 +=
            z1 - z3 * (0.766367282f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 +=
            z1 + z2 * (1.971951411f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 += z4;
        z1 = z2 * -((0.666655658f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp1 += z1;
        tmp3 +=
            z1 + z4 * (1.065388962f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = z2 * -((1.247225013f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp10 +=
            z2 + z4 * (3.141271809f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp2 += z2;
        tmp3 += z2;
        z2 =
            (z4 - z3) * (0.410524528f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 += z2;
        tmp11 += z2;
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 15i32) as isize) = (tmp20 - tmp0 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 14i32) as isize) = (tmp21 - tmp1 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp2 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 13i32) as isize) = (tmp22 - tmp2 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp3 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp23 - tmp3 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp24 - tmp10 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp25 - tmp11 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 + tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp26 - tmp12 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp27 + tmp13 >> 13i32 - 2i32) as c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp27 - tmp13 >> 13i32 - 2i32) as c_int;
        ctr += 1;
        inptr = inptr.offset(1isize);
        quantptr = quantptr.offset(1isize);
        wsptr = wsptr.offset(1isize)
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 16i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        tmp0 = *wsptr.offset(0isize) as JLONG + ((ONE as JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as c_ulong) << 13i32) as JLONG;
        z1 = *wsptr.offset(4isize) as JLONG;
        tmp1 = z1 * (1.306562965f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 = z1 * 4433i32 as JLONG;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;
        z1 = *wsptr.offset(2isize) as JLONG;
        z2 = *wsptr.offset(6isize) as JLONG;
        z3 = z1 - z2;
        z4 = z3 * (0.275899379f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z3 = z3 * (1.387039845f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = z3 + z2 * 20995i32 as JLONG;
        tmp1 = z4 + z1 * 7373i32 as JLONG;
        tmp2 =
            z3 - z1 * (0.601344887f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 =
            z4 - z2 * (0.509795579f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        z1 = *wsptr.offset(1isize) as JLONG;
        z2 = *wsptr.offset(3isize) as JLONG;
        z3 = *wsptr.offset(5isize) as JLONG;
        z4 = *wsptr.offset(7isize) as JLONG;
        tmp11 = z1 + z3;
        tmp1 =
            (z1 + z2) * (1.353318001f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 = tmp11 * (1.247225013f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp3 =
            (z1 + z4) * (1.093201867f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 =
            (z1 - z4) * (0.897167586f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 = tmp11 * (0.666655658f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 =
            (z1 - z2) * (0.410524528f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 =
            (z2 + z3) * (0.138617169f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp1 +=
            z1 + z2 * (0.071888074f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp2 +=
            z1 - z3 * (1.125726048f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z1 =
            (z3 - z2) * (1.407403738f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp11 +=
            z1 - z3 * (0.766367282f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 +=
            z1 + z2 * (1.971951411f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 += z4;
        z1 = z2 * -((0.666655658f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp1 += z1;
        tmp3 +=
            z1 + z4 * (1.065388962f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        z2 = z2 * -((1.247225013f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp10 +=
            z2 + z4 * (3.141271809f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG);
        tmp2 += z2;
        tmp3 += z2;
        z2 =
            (z4 - z3) * (0.410524528f64 * ((1i32 as JLONG) << 13i32) as c_double + 0.5f64) as JLONG;
        tmp10 += z2;
        tmp11 += z2;
        *outptr.offset(0isize) = *range_limit
            .offset(((tmp20 + tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(15isize) = *range_limit
            .offset(((tmp20 - tmp0 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) = *range_limit
            .offset(((tmp21 + tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(14isize) = *range_limit
            .offset(((tmp21 - tmp1 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) = *range_limit
            .offset(((tmp22 + tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(13isize) = *range_limit
            .offset(((tmp22 - tmp2 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) = *range_limit
            .offset(((tmp23 + tmp3 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(12isize) = *range_limit
            .offset(((tmp23 - tmp3 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) = *range_limit
            .offset(((tmp24 + tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(11isize) = *range_limit
            .offset(((tmp24 - tmp10 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) = *range_limit
            .offset(((tmp25 + tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(10isize) = *range_limit
            .offset(((tmp25 - tmp11 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) = *range_limit
            .offset(((tmp26 + tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(9isize) = *range_limit
            .offset(((tmp26 - tmp12 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) = *range_limit
            .offset(((tmp27 + tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        *outptr.offset(8isize) = *range_limit
            .offset(((tmp27 - tmp13 >> 13i32 + 2i32 + 3i32) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(8isize);
        ctr += 1
    }
}
