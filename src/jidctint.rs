use libc;

/* default definition */

/* may work if 'int' is 32 bits */

/* Same except both inputs are variables. */

/* default definition */

/* known to work with Microsoft C 6.0 */

/* may work if 'int' is 32 bits */

/* Multiply a JLONG variable by a JLONG constant to yield a JLONG result.
 * This macro is used only when the two inputs will actually be no more than
 * 16 bits wide, so that a 16x16->32 bit multiply can be used instead of a
 * full 32x32 multiply.  This provides a useful speedup on many machines.
 * Unfortunately there is no way to specify a 16x16->32 multiply portably
 * in C, but some C compilers will do the right thing if you provide the
 * correct combination of casts.
 */

/* Descale and correctly round a JLONG value that's scaled by N bits.
 * We assume RIGHT_SHIFT rounds towards minus infinity, so adding
 * the fudge factor is correct for either sign of X.
 */

/* Convert a positive real constant to an integer scaled by CONST_SCALE.
 * Caution: some C compilers fail to reduce "FIX(constant)" at compile time,
 * thus causing a lot of useless floating-point operations at run time.
 */
pub use crate::stddef_h::size_t;

pub use crate::jdct_h::ISLOW_MULT_TYPE;
pub use crate::jdct_h::ONE;
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

pub const CONST_BITS: libc::c_int = 13i32;

pub const PASS1_BITS: libc::c_int = 2i32;
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp1: crate::jpegint_h::JLONG = 0;
    let mut tmp2: crate::jpegint_h::JLONG = 0;
    let mut tmp3: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut z5: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 64] = [0; 64];
    /* Pass 1: process columns from input, store into work array. */
    /* Note results are scaled up by sqrt(8) compared to a true IDCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = crate::jpeglib_h::DCTSIZE;
    while ctr > 0i32 {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */
        if *inptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) as libc::c_int == 0i32
            && *inptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) as libc::c_int == 0i32
            && *inptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) as libc::c_int == 0i32
            && *inptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) as libc::c_int == 0i32
            && *inptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) as libc::c_int == 0i32
            && *inptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) as libc::c_int == 0i32
            && *inptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) as libc::c_int == 0i32
        {
            /* AC terms all zero */
            let mut dcval: libc::c_int = ((((*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
                as libc::c_ulong)
                << 2i32))
                as libc::c_int; /* advance pointers to next column */
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) = dcval;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) = dcval;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) = dcval;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) = dcval;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) = dcval;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) = dcval;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) = dcval;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) = dcval;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        } else {
            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */
            z2 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG;
            z3 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG;
            z1 = (z2 + z3) * 4433i64;
            tmp2 = z1 + z3 * -(15137i64);
            tmp3 = z1 + z2 * 6270i64;
            z2 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG;
            z3 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG;
            tmp0 = (((z2 + z3) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
            tmp1 = (((z2 - z3) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
            tmp10 = tmp0 + tmp3;
            tmp13 = tmp0 - tmp3;
            tmp11 = tmp1 + tmp2;
            tmp12 = tmp1 - tmp2;
            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */
            tmp0 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG; /* sqrt(2) * c3 */
            tmp1 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG; /* sqrt(2) * (-c1+c3+c5-c7) */
            tmp2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG; /* sqrt(2) * ( c1+c3-c5+c7) */
            tmp3 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
                * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
                as crate::jpegint_h::JLONG; /* sqrt(2) * ( c1+c3+c5-c7) */
            z1 = tmp0 + tmp3; /* sqrt(2) * ( c1+c3-c5-c7) */
            z2 = tmp1 + tmp2; /* sqrt(2) * ( c7-c3) */
            z3 = tmp0 + tmp2; /* sqrt(2) * (-c1-c3) */
            z4 = tmp1 + tmp3; /* sqrt(2) * (-c3-c5) */
            z5 = (z3 + z4) * 9633i64; /* sqrt(2) * ( c5-c3) */
            tmp0 = tmp0 * 2446i64;
            tmp1 = tmp1 * 16819i64;
            tmp2 = tmp2 * 25172i64;
            tmp3 = tmp3 * 12299i64;
            z1 = z1 * -(7373i64);
            z2 = z2 * -(20995i64);
            z3 = z3 * -(16069i64);
            z4 = z4 * -(3196i64);
            z3 += z5;
            z4 += z5;
            tmp0 += z1 + z3;
            tmp1 += z2 + z4;
            tmp2 += z2 + z3;
            tmp3 += z1 + z4;
            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) =
                (tmp10 + tmp3 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int; /* advance pointers to next column */
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) =
                (tmp10 - tmp3 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) =
                (tmp11 + tmp2 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) =
                (tmp11 - tmp2 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) =
                (tmp12 + tmp1 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) =
                (tmp12 - tmp1 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) =
                (tmp13 + tmp0 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int;
            *wsptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) =
                (tmp13 - tmp0 + ((1i64) << 13i32 - 2i32 - 1i32)
                    >> 13i32 - 2i32) as libc::c_int;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        }
        ctr -= 1
    }
    /* Pass 2: process rows from work array, store into output array. */
    /* Note that we must descale the results by a factor of 8 == 2**3, */
    /* and also undo the PASS1_BITS scaling. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < crate::jpeglib_h::DCTSIZE {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        /* Rows of zeroes can be exploited in the same way as we did with columns.
         * However, the column calculation has created many nonzero AC terms, so
         * the simplification applies less often (typically 5% to 10% of the time).
         * On machines with very fast multiplication, it's possible that the
         * test takes more time than it's worth.  In that case this section
         * may be commented out.
         */
        if *wsptr.offset(1) == 0i32
            && *wsptr.offset(2) == 0i32
            && *wsptr.offset(3) == 0i32
            && *wsptr.offset(4) == 0i32
            && *wsptr.offset(5) == 0i32
            && *wsptr.offset(6) == 0i32
            && *wsptr.offset(7) == 0i32
        {
            /* AC terms all zero */
            let mut dcval_0: crate::jmorecfg_h::JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0) as crate::jpegint_h::JLONG
                    + ((1i64) << 2i32 + 3i32 - 1i32)
                    >> 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            ); /* advance pointer to next row */
            *outptr.offset(0) = dcval_0;
            *outptr.offset(1) = dcval_0;
            *outptr.offset(2) = dcval_0;
            *outptr.offset(3) = dcval_0;
            *outptr.offset(4) = dcval_0;
            *outptr.offset(5) = dcval_0;
            *outptr.offset(6) = dcval_0;
            *outptr.offset(7) = dcval_0;
            wsptr = wsptr.offset(crate::jpeglib_h::DCTSIZE as isize)
        } else {
            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */
            z2 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
            z3 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
            z1 = (z2 + z3) * 4433i64;
            tmp2 = z1 + z3 * -(15137i64);
            tmp3 = z1 + z2 * 6270i64;
            tmp0 = (((*wsptr.offset(0) as crate::jpegint_h::JLONG
                + *wsptr.offset(4) as crate::jpegint_h::JLONG)
                as libc::c_ulong)
                << 13i32) as crate::jpegint_h::JLONG;
            tmp1 = (((*wsptr.offset(0) as crate::jpegint_h::JLONG
                - *wsptr.offset(4) as crate::jpegint_h::JLONG)
                as libc::c_ulong)
                << 13i32) as crate::jpegint_h::JLONG;
            tmp10 = tmp0 + tmp3;
            tmp13 = tmp0 - tmp3;
            tmp11 = tmp1 + tmp2;
            tmp12 = tmp1 - tmp2;
            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */
            tmp0 = *wsptr.offset(7) as crate::jpegint_h::JLONG; /* sqrt(2) * c3 */
            tmp1 = *wsptr.offset(5) as crate::jpegint_h::JLONG; /* sqrt(2) * (-c1+c3+c5-c7) */
            tmp2 = *wsptr.offset(3) as crate::jpegint_h::JLONG; /* sqrt(2) * ( c1+c3-c5+c7) */
            tmp3 = *wsptr.offset(1) as crate::jpegint_h::JLONG; /* sqrt(2) * ( c1+c3+c5-c7) */
            z1 = tmp0 + tmp3; /* sqrt(2) * ( c1+c3-c5-c7) */
            z2 = tmp1 + tmp2; /* sqrt(2) * ( c7-c3) */
            z3 = tmp0 + tmp2; /* sqrt(2) * (-c1-c3) */
            z4 = tmp1 + tmp3; /* sqrt(2) * (-c3-c5) */
            z5 = (z3 + z4) * 9633i64; /* sqrt(2) * ( c5-c3) */
            tmp0 = tmp0 * 2446i64;
            tmp1 = tmp1 * 16819i64;
            tmp2 = tmp2 * 25172i64;
            tmp3 = tmp3 * 12299i64;
            z1 = z1 * -(7373i64);
            z2 = z2 * -(20995i64);
            z3 = z3 * -(16069i64);
            z4 = z4 * -(3196i64);
            z3 += z5;
            z4 += z5;
            tmp0 += z1 + z3;
            tmp1 += z2 + z4;
            tmp2 += z2 + z3;
            tmp3 += z1 + z4;
            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
            *outptr.offset(0) = *range_limit.offset(
                ((tmp10 + tmp3 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            *outptr.offset(7) = *range_limit.offset(
                ((tmp10 - tmp3 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            *outptr.offset(1) = *range_limit.offset(
                ((tmp11 + tmp2 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            *outptr.offset(6) = *range_limit.offset(
                ((tmp11 - tmp2 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            *outptr.offset(2) = *range_limit.offset(
                ((tmp12 + tmp1 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            *outptr.offset(5) = *range_limit.offset(
                ((tmp12 - tmp1 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            *outptr.offset(3) = *range_limit.offset(
                ((tmp13 + tmp0 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            *outptr.offset(4) = *range_limit.offset(
                ((tmp13 - tmp0 + ((1i64) << 13i32 + 2i32 + 3i32 - 1i32)
                    >> 13i32 + 2i32 + 3i32) as libc::c_int
                    & crate::jdct_h::RANGE_MASK) as isize,
            );
            wsptr = wsptr.offset(crate::jpeglib_h::DCTSIZE as isize)
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp1: crate::jpegint_h::JLONG = 0;
    let mut tmp2: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 49] = [0; 49];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 7i32 {
        /* Even part */
        tmp13 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp13 = ((tmp13 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        tmp13 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c4 */
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c6 */
        z2 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2+c4-c6 */
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2 */
        tmp10 = (z2 - z3)
            * (0.881747734f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c2-c4-c6 */
        tmp12 = (z1 - z2)
            * (0.314692123f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c2+c4+c6 */
        tmp11 = tmp10 + tmp12 + tmp13
            - z2 * (1.841218003f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c0 */
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = tmp0
            * (1.274162392f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + tmp13;
        tmp10 += tmp0
            - z3 * (0.077722536f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += tmp0
            - z1 * (2.470602249f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 += z2
            * (1.414213562f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* (c3+c1-c5)/2 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* (c3+c5-c1)/2 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* -c1 */
        tmp1 = (z1 + z2)
            * (0.935414347f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c5 */
        tmp2 = (z1 - z2)
            * (0.170262339f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c1-c5 */
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (z2 + z3)
            * -((1.378756276f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp1 += tmp2;
        z2 = (z1 + z3)
            * (0.613604268f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 += z2;
        tmp2 += z2
            + z3 * (1.870828693f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((7i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((7i32 * 6i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((7i32 * 1i32) as isize) = (tmp11 + tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((7i32 * 5i32) as isize) = (tmp11 - tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((7i32 * 2i32) as isize) = (tmp12 + tmp2 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((7i32 * 4i32) as isize) = (tmp12 - tmp2 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((7i32 * 3i32) as isize) = (tmp13 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 7 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 7i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp13 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        tmp13 = ((tmp13 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z1 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        tmp10 = (z2 - z3)
            * (0.881747734f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = (z1 - z2)
            * (0.314692123f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = tmp10 + tmp12 + tmp13
            - z2 * (1.841218003f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = tmp0
            * (1.274162392f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + tmp13;
        tmp10 += tmp0
            - z3 * (0.077722536f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += tmp0
            - z1 * (2.470602249f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 += z2
            * (1.414213562f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        tmp1 = (z1 + z2)
            * (0.935414347f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 = (z1 - z2)
            * (0.170262339f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (z2 + z3)
            * -((1.378756276f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp1 += tmp2;
        z2 = (z1 + z3)
            * (0.613604268f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 += z2;
        tmp2 += z2
            + z3 * (1.870828693f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp12 + tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp12 - tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK) as isize,
        );
        wsptr = wsptr.offset(7);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c6 */
/* c2+c4-c6 */
/* c2 */
/* c2-c4-c6 */
/* c2+c4+c6 */
/* c0 */
/* Odd part */
/* (c3+c1-c5)/2 */
/* (c3+c5-c1)/2 */
/* -c1 */
/* c5 */
/* c3+c1-c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 6x6 output block.
 *
 * Optimized algorithm with 3 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/12).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_6x6(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp1: crate::jpegint_h::JLONG = 0;
    let mut tmp2: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 36] = [0; 36];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 6i32 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        tmp0 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c4 */
        tmp2 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2 */
        tmp10 = tmp2
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10 >> 13i32 - 2i32;
        tmp10 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp0 = tmp10
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp1 = (z1 + z3)
            * (0.366025404f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = tmp1 + (((z1 + z2) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp2 = tmp1 + (((z3 - z2) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp1 = (((z1 - z2 - z3) as libc::c_ulong) << 2i32) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((6i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((6i32 * 5i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((6i32 * 1i32) as isize) = (tmp11 + tmp1) as libc::c_int;
        *wsptr.offset((6i32 * 4i32) as isize) = (tmp11 - tmp1) as libc::c_int;
        *wsptr.offset((6i32 * 2i32) as isize) = (tmp12 + tmp2 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((6i32 * 3i32) as isize) = (tmp12 - tmp2 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 6 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 6i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp2 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        tmp10 = tmp2
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10;
        tmp10 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        tmp0 = tmp10
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        tmp1 = (z1 + z3)
            * (0.366025404f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = tmp1 + (((z1 + z2) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp2 = tmp1 + (((z3 - z2) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp1 = (((z1 - z2 - z3) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp12 + tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp12 - tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        wsptr = wsptr.offset(6);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c2 */
/* Odd part */
/* c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 5x5 output block.
 *
 * Optimized algorithm with 5 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/10).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_5x5(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp1: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 25] = [0; 25];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 5i32 {
        /* Even part */
        tmp12 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp12 = ((tmp12 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        tmp12 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* (c2+c4)/2 */
        tmp0 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* (c2-c4)/2 */
        tmp1 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z1 = (tmp0 + tmp1)
            * (0.790569415f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = (tmp0 - tmp1)
            * (0.353553391f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= ((z2 as libc::c_ulong) << 2i32) as crate::jpegint_h::JLONG;
        /* Odd part */
        z2 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3 */
        z3 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c1-c3 */
        z1 = (z2 + z3)
            * (0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c3 */
        tmp0 = z1
            + z2 * (0.513743148f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp1 = z1
            - z3 * (2.176250899f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((5i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((5i32 * 4i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((5i32 * 1i32) as isize) = (tmp11 + tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((5i32 * 3i32) as isize) = (tmp11 - tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((5i32 * 2i32) as isize) = (tmp12 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 5 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 5i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp12 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        tmp12 = ((tmp12 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp0 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        tmp1 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z1 = (tmp0 + tmp1)
            * (0.790569415f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = (tmp0 - tmp1)
            * (0.353553391f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= ((z2 as libc::c_ulong) << 2i32) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z1 = (z2 + z3)
            * (0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = z1
            + z2 * (0.513743148f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp1 = z1
            - z3 * (2.176250899f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK) as isize,
        );
        wsptr = wsptr.offset(5);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* (c2+c4)/2 */
/* (c2-c4)/2 */
/* Odd part */
/* c3 */
/* c1-c3 */
/* c1+c3 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 3x3 output block.
 *
 * Optimized algorithm with 2 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/6).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_3x3(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp2: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 9] = [0; 9];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 3i32 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        tmp0 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c2 */
        tmp2 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp12 = tmp2
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        /* Odd part */
        tmp12 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c1 */
        tmp0 = tmp12
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((3i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((3i32 * 2i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((3i32 * 1i32) as isize) = (tmp2 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 3 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 3i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp2 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        tmp12 = tmp2
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        tmp12 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        tmp0 = tmp12
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK) as isize,
        );
        wsptr = wsptr.offset(3);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c2 */
/* Odd part */
/* c1 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 9x9 output block.
 *
 * Optimized algorithm with 10 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/18).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_9x9(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp1: crate::jpegint_h::JLONG = 0;
    let mut tmp2: crate::jpegint_h::JLONG = 0;
    let mut tmp3: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp14: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 72] = [0; 72];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        tmp0 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c6 */
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c6 */
        z2 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2 */
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c4 */
        tmp3 = z3
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c8 */
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;
        tmp0 = (z1 - z2)
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;
        tmp0 = (z1 + z2)
            * (1.328926049f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 = z1
            * (1.083350441f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp3 = z2
            * (0.245575608f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* -c3 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c7 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c1 */
        z2 = z2
            * -((1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG); /* c3 */
        tmp2 = (z1 + z3)
            * (0.909038955f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp3 = (z1 + z4)
            * (0.483689525f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 = (z3 - z4)
            * (1.392728481f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = (z1 - z3 - z4)
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp10 + tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp10 - tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp11 + tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp11 - tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp12 + tmp2 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp12 - tmp2 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp13 + tmp3 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp13 - tmp3 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp14 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 9 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 9i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z1 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        tmp3 = z3
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;
        tmp0 = (z1 - z2)
            * (0.707106781f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;
        tmp0 = (z1 + z2)
            * (1.328926049f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 = z1
            * (1.083350441f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp3 = z2
            * (0.245575608f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        z2 = z2
            * -((1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp2 = (z1 + z3)
            * (0.909038955f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp3 = (z1 + z4)
            * (0.483689525f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 = (z3 - z4)
            * (1.392728481f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = (z1 - z3 - z4)
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp10 + tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp10 - tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp11 + tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp11 - tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp12 + tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp12 - tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp13 + tmp3 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp13 - tmp3 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK) as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c6 */
/* c6 */
/* c2 */
/* c4 */
/* c8 */
/* Odd part */
/* -c3 */
/* c5 */
/* c7 */
/* c1 */
/* c3 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 10x10 output block.
 *
 * Optimized algorithm with 12 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/20).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_10x10(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp14: crate::jpegint_h::JLONG = 0;
    let mut tmp20: crate::jpegint_h::JLONG = 0;
    let mut tmp21: crate::jpegint_h::JLONG = 0;
    let mut tmp22: crate::jpegint_h::JLONG = 0;
    let mut tmp23: crate::jpegint_h::JLONG = 0;
    let mut tmp24: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut z5: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 80] = [0; 80];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        z3 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z3 = ((z3 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        z3 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c4 */
        z4 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c8 */
        z1 = z4
            * (1.144122806f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = z4
            * (0.437016024f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;
        tmp22 =
            z3 - (((z1 - z2) as libc::c_ulong) << 1i32) as crate::jpegint_h::JLONG >> 13i32 - 2i32;
        /* c0 = (c4-c8)*2 */
        z2 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c6 */
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2-c6 */
        z1 = (z2 + z3)
            * (0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c2+c6 */
        tmp12 = z1
            + z2 * (0.513743148f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = z1
            - z3 * (2.176250899f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* (c3-c7)/2 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* (c3+c7)/2 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c1 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c9 */
        tmp11 = z2 + z4; /* (c1-c9)/2 */
        tmp13 = z2 - z4; /* c3 */
        tmp12 = tmp13
            * (0.309016994f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c7 */
        z5 = ((z3 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z2 = tmp11
            * (0.951056516f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = z5 + tmp12;
        tmp10 = z1
            * (1.396802247f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z2
            + z4;
        tmp14 = z1
            * (0.221231742f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2
            + z4;
        z2 = tmp11
            * (0.587785252f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = z5 - tmp12 - ((tmp13 as libc::c_ulong) << 13i32 - 1i32) as crate::jpegint_h::JLONG;
        tmp12 = (((z1 - tmp13 - z3) as libc::c_ulong) << 2i32) as crate::jpegint_h::JLONG;
        tmp11 = z1
            * (1.260073511f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2
            - z4;
        tmp13 = z1
            * (0.642039522f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2
            + z4;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp22 - tmp12) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 10 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 10i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z3 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        z3 = ((z3 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z1 = z4
            * (1.144122806f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = z4
            * (0.437016024f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;
        tmp22 = z3 - (((z1 - z2) as libc::c_ulong) << 1i32) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        z1 = (z2 + z3)
            * (0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = z1
            + z2 * (0.513743148f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = z1
            - z3 * (2.176250899f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z3 = ((z3 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        tmp11 = z2 + z4;
        tmp13 = z2 - z4;
        tmp12 = tmp13
            * (0.309016994f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = tmp11
            * (0.951056516f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = z3 + tmp12;
        tmp10 = z1
            * (1.396802247f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z2
            + z4;
        tmp14 = z1
            * (0.221231742f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2
            + z4;
        z2 = tmp11
            * (0.587785252f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = z3 - tmp12 - ((tmp13 as libc::c_ulong) << 13i32 - 1i32) as crate::jpegint_h::JLONG;
        tmp12 = (((z1 - tmp13) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG - z3;
        tmp11 = z1
            * (1.260073511f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2
            - z4;
        tmp13 = z1
            * (0.642039522f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2
            + z4;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(9) = *range_limit.offset(
            ((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c8 */
/* c0 = (c4-c8)*2 */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* Odd part */
/* (c3-c7)/2 */
/* (c3+c7)/2 */
/* c1 */
/* c9 */
/* (c1-c9)/2 */
/* c3 */
/* c7 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 11x11 output block.
 *
 * Optimized algorithm with 24 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/22).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_11x11(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp14: crate::jpegint_h::JLONG = 0;
    let mut tmp20: crate::jpegint_h::JLONG = 0;
    let mut tmp21: crate::jpegint_h::JLONG = 0;
    let mut tmp22: crate::jpegint_h::JLONG = 0;
    let mut tmp23: crate::jpegint_h::JLONG = 0;
    let mut tmp24: crate::jpegint_h::JLONG = 0;
    let mut tmp25: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 88] = [0; 88];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        tmp10 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp10 = ((tmp10 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        tmp10 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c2+c4 */
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2-c6 */
        z2 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* -(c2-c10) */
        z3 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2 */
        tmp20 = (z2 - z3)
            * (2.546640132f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c2+c4+c10-c6 */
        tmp23 = (z2 - z1)
            * (0.430815045f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c4+c6 */
        z4 = z1 + z3; /* c6+c8 */
        tmp24 = z4
            * -((1.155664402f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG); /* c8+c10 */
        z4 -= z2; /* c4+c10 */
        tmp25 = tmp10
            + z4 * (1.356927976f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c0 */
        tmp21 = tmp20 + tmp23 + tmp25
            - z2 * (1.821790775f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 += tmp25
            + z3 * (2.115825087f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp23 += tmp25
            - z1 * (1.513598477f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp24 += tmp25;
        tmp22 = tmp24
            - z3 * (0.788749120f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp24 += z2
            * (1.944413522f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z1 * (1.390975730f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp25 = tmp10
            - z4 * (1.414213562f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c9 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3-c9 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5-c9 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c7-c9 */
        tmp11 = z1 + z2; /* c7+c5+c3-c1-2*c9 */
        tmp14 = (tmp11 + z3 + z4)
            * (0.398430003f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c7+c9 */
        tmp11 = tmp11
            * (0.887983902f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c7+3*c9-c3 */
        tmp12 = (z1 + z3)
            * (0.670361295f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c5-c7-c9 */
        tmp13 = tmp14
            + (z1 + z4)
                * (0.366151574f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG; /* -(c1+c9) */
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (0.923107866f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c5+c9-c7 */
        z1 = tmp14
            - (z2 + z3)
                * (1.163011579f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c9 */
        tmp11 += z1
            + z2 * (2.073276588f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += z1
            - z3 * (1.192193623f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = (z2 + z4)
            * -((1.798248910f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp11 += z1;
        tmp13 += z1
            + z4 * (2.102458632f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 += z2
            * -((1.467221301f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            + z3 * (1.001388905f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z4 * (1.684843907f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 11 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 11i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp10 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        tmp10 = ((tmp10 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z1 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        tmp20 = (z2 - z3)
            * (2.546640132f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp23 = (z2 - z1)
            * (0.430815045f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = z1 + z3;
        tmp24 = z4
            * -((1.155664402f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        z4 -= z2;
        tmp25 = tmp10
            + z4 * (1.356927976f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp21 = tmp20 + tmp23 + tmp25
            - z2 * (1.821790775f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 += tmp25
            + z3 * (2.115825087f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp23 += tmp25
            - z1 * (1.513598477f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp24 += tmp25;
        tmp22 = tmp24
            - z3 * (0.788749120f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp24 += z2
            * (1.944413522f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z1 * (1.390975730f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp25 = tmp10
            - z4 * (1.414213562f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        tmp11 = z1 + z2;
        tmp14 = (tmp11 + z3 + z4)
            * (0.398430003f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = tmp11
            * (0.887983902f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = (z1 + z3)
            * (0.670361295f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = tmp14
            + (z1 + z4)
                * (0.366151574f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (0.923107866f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = tmp14
            - (z2 + z3)
                * (1.163011579f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 += z1
            + z2 * (2.073276588f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += z1
            - z3 * (1.192193623f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = (z2 + z4)
            * -((1.798248910f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp11 += z1;
        tmp13 += z1
            + z4 * (2.102458632f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 += z2
            * -((1.467221301f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            + z3 * (1.001388905f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z4 * (1.684843907f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(10) = *range_limit.offset(
            ((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(9) = *range_limit.offset(
            ((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp25 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK) as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c2+c4 */
/* c2-c6 */
/* -(c2-c10) */
/* c2 */
/* c2+c4+c10-c6 */
/* c4+c6 */
/* c6+c8 */
/* c8+c10 */
/* c4+c10 */
/* c0 */
/* Odd part */
/* c9 */
/* c3-c9 */
/* c5-c9 */
/* c7-c9 */
/* c7+c5+c3-c1-2*c9 */
/* c7+c9 */
/* c1+c7+3*c9-c3 */
/* c3+c5-c7-c9 */
/* -(c1+c9) */
/* c1+c5+c9-c7 */
/* c3+c9 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 12x12 output block.
 *
 * Optimized algorithm with 15 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/24).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_12x12(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp14: crate::jpegint_h::JLONG = 0;
    let mut tmp15: crate::jpegint_h::JLONG = 0;
    let mut tmp20: crate::jpegint_h::JLONG = 0;
    let mut tmp21: crate::jpegint_h::JLONG = 0;
    let mut tmp22: crate::jpegint_h::JLONG = 0;
    let mut tmp23: crate::jpegint_h::JLONG = 0;
    let mut tmp24: crate::jpegint_h::JLONG = 0;
    let mut tmp25: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 96] = [0; 96];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        z3 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z3 = ((z3 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        z3 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c4 */
        z4 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2 */
        z4 = z4
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z4 = z1
            * (1.366025404f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z2 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z2 = ((z2 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* -c9 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c7 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5-c7 */
        tmp11 = z2
            * (1.306562965f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1-c5 */
        tmp14 = z2 * -(4433i64); /* -(c7+c11) */
        tmp10 = z1 + z3; /* c1+c5-c7-c11 */
        tmp15 = (tmp10 + z4)
            * (0.860918669f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c11 */
        tmp12 = tmp15
            + tmp10
                * (0.261052384f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG; /* c5+c7 */
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c9 */
        tmp13 = (z3 + z4)
            * -((1.045510580f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG); /* c3-c9 */
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c9 */
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp15 += tmp14
            - z1 * (0.676326758f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z4 * (1.982889723f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433i64;
        tmp11 = z3 + z1 * 6270i64;
        tmp14 = z3 - z2 * 15137i64;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 12 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 12i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z3 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        z3 = ((z3 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z4 = z4
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z4 = z1
            * (1.366025404f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        z2 = ((z2 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        tmp11 = z2
            * (1.306562965f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = z2 * -(4433i64);
        tmp10 = z1 + z3;
        tmp15 = (tmp10 + z4)
            * (0.860918669f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = tmp15
            + tmp10
                * (0.261052384f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = (z3 + z4)
            * -((1.045510580f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp15 += tmp14
            - z1 * (0.676326758f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z4 * (1.982889723f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433i64;
        tmp11 = z3 + z1 * 6270i64;
        tmp14 = z3 - z2 * 15137i64;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(11) = *range_limit.offset(
            ((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(10) = *range_limit.offset(
            ((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(9) = *range_limit.offset(
            ((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c2 */
/* Odd part */
/* c3 */
/* -c9 */
/* c7 */
/* c5-c7 */
/* c1-c5 */
/* -(c7+c11) */
/* c1+c5-c7-c11 */
/* c1+c11 */
/* c5+c7 */
/* c9 */
/* c3-c9 */
/* c3+c9 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 13x13 output block.
 *
 * Optimized algorithm with 29 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/26).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_13x13(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp14: crate::jpegint_h::JLONG = 0;
    let mut tmp15: crate::jpegint_h::JLONG = 0;
    let mut tmp20: crate::jpegint_h::JLONG = 0;
    let mut tmp21: crate::jpegint_h::JLONG = 0;
    let mut tmp22: crate::jpegint_h::JLONG = 0;
    let mut tmp23: crate::jpegint_h::JLONG = 0;
    let mut tmp24: crate::jpegint_h::JLONG = 0;
    let mut tmp25: crate::jpegint_h::JLONG = 0;
    let mut tmp26: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 104] = [0; 104];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        z1 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        z1 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* (c4+c6)/2 */
        z2 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* (c4-c6)/2 */
        z3 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2 */
        z4 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c10 */
        tmp10 = z3 + z4; /* (c8-c12)/2 */
        tmp11 = z3 - z4; /* (c8+c12)/2 */
        tmp12 = tmp10
            * (1.155388986f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c6 */
        tmp13 = tmp11
            * (0.096834934f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z1; /* c4 */
        tmp20 = z2
            * (1.373119086f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + tmp12
            + tmp13; /* (c2-c10)/2 */
        tmp22 = z2
            * (0.501487041f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - tmp12
            + tmp13; /* (c2+c10)/2 */
        tmp12 = tmp10
            * (0.316450131f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c12 */
        tmp13 = tmp11
            * (0.486914739f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z1; /* c8 */
        tmp21 = z2
            * (1.058554052f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - tmp12
            + tmp13; /* c0 */
        tmp25 = z2
            * -((1.252223920f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            + tmp12
            + tmp13;
        tmp12 = tmp10
            * (0.435816023f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = tmp11
            * (0.937303064f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z1;
        tmp23 = z2
            * -((0.170464608f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            - tmp12
            - tmp13;
        tmp24 = z2
            * -((0.803364869f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            + tmp12
            - tmp13;
        tmp26 = (tmp11 - z2)
            * (1.414213562f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z1;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c7 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c7+c5+c3-c1 */
        tmp11 = (z1 + z2)
            * (1.322312651f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* -c11 */
        tmp12 = (z1 + z3)
            * (1.163874945f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c5+c9+c11-c3 */
        tmp15 = z1 + z4; /* c1+c5-c9-c11 */
        tmp13 = tmp15
            * (0.937797057f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* -c5 */
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (2.020082300f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c5+c9-c7 */
        tmp14 = (z2 + z3)
            * -((0.338443458f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG); /* -c9 */
        tmp11 += tmp14
            + z2 * (0.837223564f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c11 */
        tmp12 += tmp14
            - z3 * (1.572116027f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1-c7 */
        tmp14 = (z2 + z4)
            * -((1.163874945f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG); /* c7 */
        tmp11 += tmp14; /* c1+c11 */
        tmp13 += tmp14
            + z4 * (2.205608352f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = (z3 + z4)
            * -((0.657217813f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = tmp15
            * (0.338443458f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = tmp15
            + z1 * (0.318774355f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2 * (0.466105296f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = (z3 - z2)
            * (0.937797057f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 += z1;
        tmp15 += z1
            + z3 * (0.384515595f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z4 * (1.742345811f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 13 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 13i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z1 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        tmp12 = tmp10
            * (1.155388986f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = tmp11
            * (0.096834934f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z1;
        tmp20 = z2
            * (1.373119086f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + tmp12
            + tmp13;
        tmp22 = z2
            * (0.501487041f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - tmp12
            + tmp13;
        tmp12 = tmp10
            * (0.316450131f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = tmp11
            * (0.486914739f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z1;
        tmp21 = z2
            * (1.058554052f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - tmp12
            + tmp13;
        tmp25 = z2
            * -((1.252223920f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            + tmp12
            + tmp13;
        tmp12 = tmp10
            * (0.435816023f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = tmp11
            * (0.937303064f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z1;
        tmp23 = z2
            * -((0.170464608f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            - tmp12
            - tmp13;
        tmp24 = z2
            * -((0.803364869f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            + tmp12
            - tmp13;
        tmp26 = (tmp11 - z2)
            * (1.414213562f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z1;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        tmp11 = (z1 + z2)
            * (1.322312651f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = (z1 + z3)
            * (1.163874945f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp15 = z1 + z4;
        tmp13 = tmp15
            * (0.937797057f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (2.020082300f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = (z2 + z3)
            * -((0.338443458f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp11 += tmp14
            + z2 * (0.837223564f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += tmp14
            - z3 * (1.572116027f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = (z2 + z4)
            * -((1.163874945f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp11 += tmp14;
        tmp13 += tmp14
            + z4 * (2.205608352f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = (z3 + z4)
            * -((0.657217813f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = tmp15
            * (0.338443458f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = tmp15
            + z1 * (0.318774355f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2 * (0.466105296f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = (z3 - z2)
            * (0.937797057f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 += z1;
        tmp15 += z1
            + z3 * (0.384515595f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z4 * (1.742345811f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(12) = *range_limit.offset(
            ((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(11) = *range_limit.offset(
            ((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(10) = *range_limit.offset(
            ((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(9) = *range_limit.offset(
            ((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp26 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK) as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* (c4+c6)/2 */
/* (c4-c6)/2 */
/* c2 */
/* c10 */
/* (c8-c12)/2 */
/* (c8+c12)/2 */
/* c6 */
/* c4 */
/* (c2-c10)/2 */
/* (c2+c10)/2 */
/* c12 */
/* c8 */
/* c0 */
/* Odd part */
/* c3 */
/* c5 */
/* c7 */
/* c7+c5+c3-c1 */
/* -c11 */
/* c5+c9+c11-c3 */
/* c1+c5-c9-c11 */
/* -c5 */
/* c3+c5+c9-c7 */
/* -c9 */
/* c11 */
/* c1-c7 */
/* c7 */
/* c1+c11 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 14x14 output block.
 *
 * Optimized algorithm with 20 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/28).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_14x14(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp14: crate::jpegint_h::JLONG = 0;
    let mut tmp15: crate::jpegint_h::JLONG = 0;
    let mut tmp16: crate::jpegint_h::JLONG = 0;
    let mut tmp20: crate::jpegint_h::JLONG = 0;
    let mut tmp21: crate::jpegint_h::JLONG = 0;
    let mut tmp22: crate::jpegint_h::JLONG = 0;
    let mut tmp23: crate::jpegint_h::JLONG = 0;
    let mut tmp24: crate::jpegint_h::JLONG = 0;
    let mut tmp25: crate::jpegint_h::JLONG = 0;
    let mut tmp26: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 112] = [0; 112];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        z1 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        z1 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c4 */
        z4 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c12 */
        z2 = z4
            * (1.274162392f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c8 */
        z3 = z4
            * (0.314692123f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = z4
            * (0.881747734f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;
        tmp23 = z1 - (((z2 + z3 - z4) as libc::c_ulong) << 1i32) as crate::jpegint_h::JLONG
            >> 13i32 - 2i32;
        /* c0 = (c4+c12-c8)*2 */
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c6 */
        z2 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c2-c6 */
        z3 = (z1 + z2)
            * (1.105676686f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c6+c10 */
        tmp13 = z3
            + z1 * (0.273079590f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c2 */
        tmp14 = z3
            - z2 * (1.719280954f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp15 = z1
            * (0.613604268f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2 * (1.378756276f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3+c5-c1 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c9 */
        tmp13 = ((z4 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG; /* c9+c11-c13 */
        tmp14 = z1 + z3; /* c11 */
        tmp11 = (z1 + z2)
            * (1.334852607f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* -c13 */
        tmp12 = tmp14
            * (1.197448846f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3-c9-c13 */
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (1.126980169f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c5-c13 */
        tmp14 = tmp14
            * (0.752406978f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1 */
        tmp16 = tmp14
            - z1 * (1.061150426f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c9-c11 */
        z1 -= z2; /* c1+c11-c5 */
        tmp15 = z1
            * (0.467085129f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - tmp13;
        tmp16 += tmp15;
        z1 += z4;
        z4 = (z2 + z3)
            * -((0.158341681f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            - tmp13;
        tmp11 += z4
            - z2 * (0.424103948f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += z4
            - z3 * (2.373959773f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = (z3 - z2)
            * (1.405321284f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 += z4 + tmp13
            - z3 * (1.6906431334f64
                * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp15 += z4
            + z2 * (0.674957567f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = (((z1 - z3) as libc::c_ulong) << 2i32) as crate::jpegint_h::JLONG;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 13i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13) as libc::c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp23 - tmp13) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 + tmp16 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp26 - tmp16 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 14 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 14i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z1 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z2 = z4
            * (1.274162392f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z3 = z4
            * (0.314692123f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = z4
            * (0.881747734f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;
        tmp23 = z1 - (((z2 + z3 - z4) as libc::c_ulong) << 1i32) as crate::jpegint_h::JLONG;
        z1 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        z3 = (z1 + z2)
            * (1.105676686f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = z3
            + z1 * (0.273079590f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = z3
            - z2 * (1.719280954f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp15 = z1
            * (0.613604268f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z2 * (1.378756276f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        z4 = ((z4 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        tmp14 = z1 + z3;
        tmp11 = (z1 + z2)
            * (1.334852607f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = tmp14
            * (1.197448846f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp11 + tmp12 + z4
            - z1 * (1.126980169f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = tmp14
            * (0.752406978f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp16 = tmp14
            - z1 * (1.061150426f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 -= z2;
        tmp15 = z1
            * (0.467085129f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z4;
        tmp16 += tmp15;
        tmp13 = (z2 + z3)
            * -((0.158341681f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG)
            - z4;
        tmp11 += tmp13
            - z2 * (0.424103948f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += tmp13
            - z3 * (2.373959773f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = (z3 - z2)
            * (1.405321284f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 += tmp13 + z4
            - z3 * (1.6906431334f64
                * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp15 += tmp13
            + z2 * (0.674957567f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = (((z1 - z3) as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG + z4;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(13) = *range_limit.offset(
            ((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(12) = *range_limit.offset(
            ((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(11) = *range_limit.offset(
            ((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(10) = *range_limit.offset(
            ((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(9) = *range_limit.offset(
            ((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp26 + tmp16 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp26 - tmp16 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c12 */
/* c8 */
/* c0 = (c4+c12-c8)*2 */
/* c6 */
/* c2-c6 */
/* c6+c10 */
/* c2 */
/* Odd part */
/* c3 */
/* c5 */
/* c3+c5-c1 */
/* c9 */
/* c9+c11-c13 */
/* c11 */
/* -c13 */
/* c3-c9-c13 */
/* c3+c5-c13 */
/* c1 */
/* c1+c9-c11 */
/* c1+c11-c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 15x15 output block.
 *
 * Optimized algorithm with 22 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/30).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_15x15(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp14: crate::jpegint_h::JLONG = 0;
    let mut tmp15: crate::jpegint_h::JLONG = 0;
    let mut tmp16: crate::jpegint_h::JLONG = 0;
    let mut tmp20: crate::jpegint_h::JLONG = 0;
    let mut tmp21: crate::jpegint_h::JLONG = 0;
    let mut tmp22: crate::jpegint_h::JLONG = 0;
    let mut tmp23: crate::jpegint_h::JLONG = 0;
    let mut tmp24: crate::jpegint_h::JLONG = 0;
    let mut tmp25: crate::jpegint_h::JLONG = 0;
    let mut tmp26: crate::jpegint_h::JLONG = 0;
    let mut tmp27: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 120] = [0; 120];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        z1 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        z1 += (crate::jdct_h::ONE as crate::jpegint_h::JLONG) << CONST_BITS - PASS1_BITS - 1i32; /* c12 */
        z2 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c6 */
        z3 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c0 = (c6-c12)*2 */
        z4 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* (c2+c4)/2 */
        tmp10 = z4
            * (0.437016024f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* (c2-c4)/2 */
        tmp11 = z4
            * (1.144122806f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c4+c14 */
        tmp12 = z1 - tmp10; /* (c8+c14)/2 */
        tmp13 = z1 + tmp11; /* (c8-c14)/2 */
        z1 -= (((tmp11 - tmp10) as libc::c_ulong) << 1i32) as crate::jpegint_h::JLONG; /* (c6+c12)/2 */
        z4 = z2 - z3; /* (c6-c12)/2 */
        z3 += z2; /* c10 = c6-c12 */
        tmp10 = z3
            * (1.337628990f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c0 = (c6-c12)*2 */
        tmp11 = z4
            * (0.045680613f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = z2
            * (1.439773946f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;
        tmp10 = z3
            * (0.547059574f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = z4
            * (0.399234004f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;
        tmp10 = z3
            * (0.790569415f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = z4
            * (0.353553391f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11;
        tmp27 = z1 - tmp11 - tmp11;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c9 */
        z4 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3-c9 */
        z3 = z4
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c9 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* -c9 */
        tmp13 = z2 - z4; /* -c3 */
        tmp15 = (z1 + tmp13)
            * (0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1 */
        tmp11 = tmp15
            + z1 * (0.513743148f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c7 */
        tmp14 = tmp15
            - tmp13
                * (2.176250899f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG; /* c1-c13 */
        tmp13 = z2
            * -((0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG); /* c5 */
        tmp15 = z2
            * -((1.344997024f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG); /* c11 */
        z2 = z1 - z4; /* c7-c11 */
        tmp12 = z3
            + z2 * (1.406466353f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c11+c13 */
        tmp10 = tmp12
            + z4 * (2.457431844f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - tmp15;
        tmp16 = tmp12
            - z1 * (1.112434820f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + tmp13;
        tmp12 = z2
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z3;
        z2 = (z1 + z4)
            * (0.575212477f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 += z2
            + z1 * (0.475753014f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z3;
        tmp15 += z2
            - z4 * (0.869244010f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z3;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 14i32) as isize) = (tmp20 - tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 13i32) as isize) = (tmp21 - tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp22 - tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp23 - tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp24 - tmp14 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp15 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp25 - tmp15 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 + tmp16 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp26 - tmp16 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp27 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 15 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 15i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z1 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        z1 = ((z1 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        tmp10 = z4
            * (0.437016024f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = z4
            * (1.144122806f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = z1 - tmp10;
        tmp13 = z1 + tmp11;
        z1 -= (((tmp11 - tmp10) as libc::c_ulong) << 1i32) as crate::jpegint_h::JLONG;
        z4 = z2 - z3;
        z3 += z2;
        tmp10 = z3
            * (1.337628990f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = z4
            * (0.045680613f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = z2
            * (1.439773946f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;
        tmp10 = z3
            * (0.547059574f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = z4
            * (0.399234004f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;
        tmp10 = z3
            * (0.790569415f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = z4
            * (0.353553391f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11;
        tmp27 = z1 - tmp11 - tmp11;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z3 = z4
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        tmp13 = z2 - z4;
        tmp15 = (z1 + tmp13)
            * (0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = tmp15
            + z1 * (0.513743148f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp14 = tmp15
            - tmp13
                * (2.176250899f64 * ((1i64) << 13i32) as libc::c_double
                    + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = z2
            * -((0.831253876f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp15 = z2
            * -((1.344997024f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        z2 = z1 - z4;
        tmp12 = z3
            + z2 * (1.406466353f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = tmp12
            + z4 * (2.457431844f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - tmp15;
        tmp16 = tmp12
            - z1 * (1.112434820f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + tmp13;
        tmp12 = z2
            * (1.224744871f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z3;
        z2 = (z1 + z4)
            * (0.575212477f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 += z2
            + z1 * (0.475753014f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            - z3;
        tmp15 += z2
            - z4 * (0.869244010f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG
            + z3;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp20 + tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(14) = *range_limit.offset(
            ((tmp20 - tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp21 + tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(13) = *range_limit.offset(
            ((tmp21 - tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp22 + tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(12) = *range_limit.offset(
            ((tmp22 - tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp23 + tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(11) = *range_limit.offset(
            ((tmp23 - tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp24 + tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(10) = *range_limit.offset(
            ((tmp24 - tmp14 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp25 + tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(9) = *range_limit.offset(
            ((tmp25 - tmp15 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp26 + tmp16 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp26 - tmp16 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp27 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK) as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c12 */
/* c6 */
/* c0 = (c6-c12)*2 */
/* (c2+c4)/2 */
/* (c2-c4)/2 */
/* c4+c14 */
/* (c8+c14)/2 */
/* (c8-c14)/2 */
/* (c6+c12)/2 */
/* (c6-c12)/2 */
/* c10 = c6-c12 */
/* c0 = (c6-c12)*2 */
/* Odd part */
/* c5 */
/* c9 */
/* c3-c9 */
/* c3+c9 */
/* -c9 */
/* -c3 */
/* c1 */
/* c1+c7 */
/* c1-c13 */
/* c5 */
/* c11 */
/* c7-c11 */
/* c11+c13 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 16x16 output block.
 *
 * Optimized algorithm with 28 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/32).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_16x16(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jpegint_h::JLONG = 0; /* buffers data between passes */
    let mut tmp1: crate::jpegint_h::JLONG = 0;
    let mut tmp2: crate::jpegint_h::JLONG = 0;
    let mut tmp3: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut tmp20: crate::jpegint_h::JLONG = 0;
    let mut tmp21: crate::jpegint_h::JLONG = 0;
    let mut tmp22: crate::jpegint_h::JLONG = 0;
    let mut tmp23: crate::jpegint_h::JLONG = 0;
    let mut tmp24: crate::jpegint_h::JLONG = 0;
    let mut tmp25: crate::jpegint_h::JLONG = 0;
    let mut tmp26: crate::jpegint_h::JLONG = 0;
    let mut tmp27: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        ::std::ptr::null_mut::< crate::jdct_h::ISLOW_MULT_TYPE>();
    let mut wsptr: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(crate::jmorecfg_h::CENTERJSAMPLE as isize);
    let mut ctr: libc::c_int = 0;
    let mut workspace: [libc::c_int; 128] = [0; 128];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 8i32 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 0i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        /* Add fudge factor here for final descale. */
        tmp0 += (1i32 << CONST_BITS - PASS1_BITS - 1i32) as libc::c_long; /* c4[16] = c2[8] */
        z1 = (*inptr.offset((8i32 * 4i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c12[16] = c6[8] */
        tmp1 = z1
            * (1.306562965f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c14[16] = c7[8] */
        tmp2 = z1 * 4433i64; /* c2[16] = c1[8] */
        tmp10 = tmp0 + tmp1; /* (c6+c2)[16] = (c3+c1)[8] */
        tmp11 = tmp0 - tmp1; /* (c6-c14)[16] = (c3-c7)[8] */
        tmp12 = tmp0 + tmp2; /* (c2-c10)[16] = (c1-c5)[8] */
        tmp13 = tmp0 - tmp2; /* (c10-c14)[16] = (c5-c7)[8] */
        z1 = (*inptr.offset((8i32 * 2i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 2i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z2 = (*inptr.offset((8i32 * 6i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        z3 = z1 - z2;
        z4 = z3
            * (0.275899379f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z3 = z3
            * (1.387039845f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = z3 + z2 * 20995i64;
        tmp1 = z4 + z1 * 7373i64;
        tmp2 = z3
            - z1 * (0.601344887f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp3 = z4
            - z2 * (0.509795579f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 1i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c3 */
        z2 = (*inptr.offset((8i32 * 3i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 3i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c5 */
        z3 = (*inptr.offset((8i32 * 5i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c7 */
        z4 = (*inptr.offset((8i32 * 7i32) as isize) as libc::c_int
            * *quantptr.offset((8i32 * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG; /* c9 */
        tmp11 = z1 + z3; /* c11 */
        tmp1 = (z1 + z2)
            * (1.353318001f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c13 */
        tmp2 = tmp11
            * (1.247225013f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c7+c5+c3-c1 */
        tmp3 = (z1 + z4)
            * (1.093201867f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c9+c11+c13-c15 */
        tmp10 = (z1 - z4)
            * (0.897167586f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c15 */
        tmp11 = tmp11
            * (0.666655658f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c9+c11-c3-c15 */
        tmp12 = (z1 - z2)
            * (0.410524528f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c5+c7+c15-c3 */
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1 */
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c11-c9-c13 */
        z1 = (z2 + z3)
            * (0.138617169f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c5+c13-c7 */
        tmp1 += z1
            + z2 * (0.071888074f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* -c11 */
        tmp2 += z1
            - z3 * (1.125726048f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c3+c11+c15-c7 */
        z1 = (z3 - z2)
            * (1.407403738f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* -c5 */
        tmp11 += z1
            - z3 * (0.766367282f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* c1+c5+c9-c13 */
        tmp12 += z1
            + z2 * (1.971951411f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG; /* -c3 */
        z2 += z4; /* c13 */
        z1 = z2
            * -((0.666655658f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp1 += z1;
        tmp3 += z1
            + z4 * (1.065388962f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = z2
            * -((1.247225013f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp10 += z2
            + z4 * (3.141271809f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp2 += z2;
        tmp3 += z2;
        z2 = (z4 - z3)
            * (0.410524528f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 += z2;
        tmp11 += z2;
        /* Final output stage */
        *wsptr.offset((8i32 * 0i32) as isize) = (tmp20 + tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 15i32) as isize) = (tmp20 - tmp0 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 1i32) as isize) = (tmp21 + tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 14i32) as isize) = (tmp21 - tmp1 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 2i32) as isize) = (tmp22 + tmp2 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 13i32) as isize) = (tmp22 - tmp2 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 3i32) as isize) = (tmp23 + tmp3 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 12i32) as isize) = (tmp23 - tmp3 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 4i32) as isize) = (tmp24 + tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 11i32) as isize) = (tmp24 - tmp10 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 5i32) as isize) = (tmp25 + tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 10i32) as isize) = (tmp25 - tmp11 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 6i32) as isize) = (tmp26 + tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 9i32) as isize) = (tmp26 - tmp12 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 7i32) as isize) = (tmp27 + tmp13 >> 13i32 - 2i32) as libc::c_int;
        *wsptr.offset((8i32 * 8i32) as isize) = (tmp27 - tmp13 >> 13i32 - 2i32) as libc::c_int;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 16 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < 16i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jpegint_h::JLONG
            + ((crate::jdct_h::ONE as crate::jpegint_h::JLONG) << PASS1_BITS + 2i32);
        tmp0 = ((tmp0 as libc::c_ulong) << 13i32) as crate::jpegint_h::JLONG;
        z1 = *wsptr.offset(4) as crate::jpegint_h::JLONG;
        tmp1 = z1
            * (1.306562965f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 = z1 * 4433i64;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;
        z1 = *wsptr.offset(2) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(6) as crate::jpegint_h::JLONG;
        z3 = z1 - z2;
        z4 = z3
            * (0.275899379f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z3 = z3
            * (1.387039845f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = z3 + z2 * 20995i64;
        tmp1 = z4 + z1 * 7373i64;
        tmp2 = z3
            - z1 * (0.601344887f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp3 = z4
            - z2 * (0.509795579f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        z1 = *wsptr.offset(1) as crate::jpegint_h::JLONG;
        z2 = *wsptr.offset(3) as crate::jpegint_h::JLONG;
        z3 = *wsptr.offset(5) as crate::jpegint_h::JLONG;
        z4 = *wsptr.offset(7) as crate::jpegint_h::JLONG;
        tmp11 = z1 + z3;
        tmp1 = (z1 + z2)
            * (1.353318001f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 = tmp11
            * (1.247225013f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp3 = (z1 + z4)
            * (1.093201867f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 = (z1 - z4)
            * (0.897167586f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 = tmp11
            * (0.666655658f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 = (z1 - z2)
            * (0.410524528f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = (z2 + z3)
            * (0.138617169f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp1 += z1
            + z2 * (0.071888074f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp2 += z1
            - z3 * (1.125726048f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z1 = (z3 - z2)
            * (1.407403738f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp11 += z1
            - z3 * (0.766367282f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += z1
            + z2 * (1.971951411f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 += z4;
        z1 = z2
            * -((0.666655658f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp1 += z1;
        tmp3 += z1
            + z4 * (1.065388962f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        z2 = z2
            * -((1.247225013f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp10 += z2
            + z4 * (3.141271809f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG);
        tmp2 += z2;
        tmp3 += z2;
        z2 = (z4 - z3)
            * (0.410524528f64 * ((1i64) << 13i32) as libc::c_double
                + 0.5f64) as crate::jpegint_h::JLONG;
        tmp10 += z2;
        tmp11 += z2;
        *outptr.offset(0) = *range_limit.offset(
            ((tmp20 + tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(15) = *range_limit.offset(
            ((tmp20 - tmp0 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(1) = *range_limit.offset(
            ((tmp21 + tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(14) = *range_limit.offset(
            ((tmp21 - tmp1 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(2) = *range_limit.offset(
            ((tmp22 + tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(13) = *range_limit.offset(
            ((tmp22 - tmp2 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(3) = *range_limit.offset(
            ((tmp23 + tmp3 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(12) = *range_limit.offset(
            ((tmp23 - tmp3 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(4) = *range_limit.offset(
            ((tmp24 + tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(11) = *range_limit.offset(
            ((tmp24 - tmp10 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(5) = *range_limit.offset(
            ((tmp25 + tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(10) = *range_limit.offset(
            ((tmp25 - tmp11 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(6) = *range_limit.offset(
            ((tmp26 + tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(9) = *range_limit.offset(
            ((tmp26 - tmp12 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(7) = *range_limit.offset(
            ((tmp27 + tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        *outptr.offset(8) = *range_limit.offset(
            ((tmp27 - tmp13 >> 13i32 + 2i32 + 3i32) as libc::c_int & crate::jdct_h::RANGE_MASK)
                as isize,
        );
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4[16] = c2[8] */
/* c12[16] = c6[8] */
/* c14[16] = c7[8] */
/* c2[16] = c1[8] */
/* (c6+c2)[16] = (c3+c1)[8] */
/* (c6-c14)[16] = (c3-c7)[8] */
/* (c2-c10)[16] = (c1-c5)[8] */
/* (c10-c14)[16] = (c5-c7)[8] */
/* Odd part */
/* c3 */
/* c5 */
/* c7 */
/* c9 */
/* c11 */
/* c13 */
/* c7+c5+c3-c1 */
/* c9+c11+c13-c15 */
/* c15 */
/* c9+c11-c3-c15 */
/* c5+c7+c15-c3 */
/* c1 */
/* c1+c11-c9-c13 */
/* c1+c5+c13-c7 */
/* -c11 */
/* c3+c11+c15-c7 */
/* -c5 */
/* c1+c5+c9-c13 */
/* -c3 */
/* c13 */
/* Final output stage */
/* DCT_ISLOW_SUPPORTED */
/* IDCT_SCALING_SUPPORTED */
