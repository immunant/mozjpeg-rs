use libc;

/* JPEGLIB_H */

/*
 * The JPEG library modules define JPEG_INTERNALS before including this file.
 * The internal structure declarations are read only when that is true.
 * Applications using the library should not include jpegint.h, but may wish
 * to include jerror.h.
 */

/* INCOMPLETE_TYPES_BROKEN */

/* If we have a brain-damaged compiler that emits warnings (or worse, errors)
 * for structure definitions that are never filled in, keep it quiet by
 * supplying dummy definitions for the various substructures.
 */

/* COM marker code */

/* APP0 marker code */

/* EOI marker code */

/* RST0 marker code */

/* These marker codes are exported since applications and data source modules
 * are likely to want to use them.
 */
pub use crate::jdct_h::DCTELEM;
pub use crate::jpegint_h::JLONG;
pub use crate::jpeglib_h::DCTSIZE;
/* preferred floating type */
/*
 * Each IDCT routine is responsible for range-limiting its results and
 * converting them to unsigned form (0..MAXJSAMPLE).  The raw outputs could
 * be quite far out of range if the input data is corrupt, so a bulletproof
 * range-limiting step is required.  We use a mask-and-table-lookup method
 * to do the combined operations quickly.  See the comments with
 * prepare_range_limit_table (in jdmaster.c) for more info.
 */
/* 2 bits wider than legal samples */
/* Extern declarations for the forward and inverse DCT routines. */
/*
 * jfdctint.c
 *
 * This file was part of the Independent JPEG Group's software.
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a slow-but-accurate integer implementation of the
 * forward DCT (Discrete Cosine Transform).
 *
 * A 2-D DCT can be done by 1-D DCT on each row followed by 1-D DCT
 * on each column.  Direct algorithms are also available, but they are
 * much more complex and seem not to be any faster when reduced to code.
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
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/*
 * The poop on this scaling stuff is as follows:
 *
 * Each 1-D DCT step produces outputs which are a factor of sqrt(N)
 * larger than the true DCT outputs.  The final outputs are therefore
 * a factor of N larger than desired; since N=8 this can be cured by
 * a simple right shift at the end of the algorithm.  The advantage of
 * this arrangement is that we save two multiplications per 1-D DCT,
 * because the y0 and y4 outputs need not be divided by sqrt(N).
 * In the IJG code, this factor of 8 is removed by the quantization step
 * (in jcdctmgr.c), NOT in this module.
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
 * with the recommended scaling.  (For 12-bit sample data, the intermediate
 * array is JLONG anyway.)
 *
 * To avoid overflow of the 32-bit intermediate results in pass 2, we must
 * have BITS_IN_JSAMPLE + CONST_BITS + PASS1_BITS <= 26.  Error analysis
 * shows that the values given below are the most effective.
 */
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
/*
 * Perform the forward DCT on one block of samples.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_islow(mut data: *mut crate::jdct_h::DCTELEM) {
    let mut tmp0: crate::jpegint_h::JLONG = 0;
    let mut tmp1: crate::jpegint_h::JLONG = 0;
    let mut tmp2: crate::jpegint_h::JLONG = 0;
    let mut tmp3: crate::jpegint_h::JLONG = 0;
    let mut tmp4: crate::jpegint_h::JLONG = 0;
    let mut tmp5: crate::jpegint_h::JLONG = 0;
    let mut tmp6: crate::jpegint_h::JLONG = 0;
    let mut tmp7: crate::jpegint_h::JLONG = 0;
    let mut tmp10: crate::jpegint_h::JLONG = 0;
    let mut tmp11: crate::jpegint_h::JLONG = 0;
    let mut tmp12: crate::jpegint_h::JLONG = 0;
    let mut tmp13: crate::jpegint_h::JLONG = 0;
    let mut z1: crate::jpegint_h::JLONG = 0;
    let mut z2: crate::jpegint_h::JLONG = 0;
    let mut z3: crate::jpegint_h::JLONG = 0;
    let mut z4: crate::jpegint_h::JLONG = 0;
    let mut z5: crate::jpegint_h::JLONG = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut ctr: libc::c_int = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    dataptr = data;
    ctr = crate::jpeglib_h::DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 = (*dataptr.offset(0) as libc::c_int + *dataptr.offset(7) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp7 = (*dataptr.offset(0) as libc::c_int - *dataptr.offset(7) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp1 = (*dataptr.offset(1) as libc::c_int + *dataptr.offset(6) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp6 = (*dataptr.offset(1) as libc::c_int - *dataptr.offset(6) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp2 = (*dataptr.offset(2) as libc::c_int + *dataptr.offset(5) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp5 = (*dataptr.offset(2) as libc::c_int - *dataptr.offset(5) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp3 = (*dataptr.offset(3) as libc::c_int + *dataptr.offset(4) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp4 = (*dataptr.offset(3) as libc::c_int - *dataptr.offset(4) as libc::c_int)
            as crate::jpegint_h::JLONG;
        /* advance pointer to next row */
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset(0) = (((tmp10 + tmp11) as libc::c_ulong) << 2i32) as crate::jpegint_h::JLONG
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4) = (((tmp10 - tmp11) as libc::c_ulong) << 2i32) as crate::jpegint_h::JLONG
            as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433i32 as crate::jpegint_h::JLONG;
        *dataptr.offset(2) = (z1
            + tmp13 * 6270i32 as crate::jpegint_h::JLONG
            + ((1i32 as crate::jpegint_h::JLONG) << 13i32 - 2i32 - 1i32)
            >> 13i32 - 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(6) = (z1
            + tmp12 * -(15137i32 as crate::jpegint_h::JLONG)
            + ((1i32 as crate::jpegint_h::JLONG) << 13i32 - 2i32 - 1i32)
            >> 13i32 - 2i32) as crate::jdct_h::DCTELEM;
        z1 = tmp4 + tmp7;
        z2 = tmp5 + tmp6;
        z3 = tmp4 + tmp6;
        z4 = tmp5 + tmp7;
        z5 = (z3 + z4) * 9633i32 as crate::jpegint_h::JLONG;
        tmp4 = tmp4 * 2446i32 as crate::jpegint_h::JLONG;
        tmp5 = tmp5 * 16819i32 as crate::jpegint_h::JLONG;
        tmp6 = tmp6 * 25172i32 as crate::jpegint_h::JLONG;
        tmp7 = tmp7 * 12299i32 as crate::jpegint_h::JLONG;
        z1 = z1 * -(7373i32 as crate::jpegint_h::JLONG);
        z2 = z2 * -(20995i32 as crate::jpegint_h::JLONG);
        z3 = z3 * -(16069i32 as crate::jpegint_h::JLONG);
        z4 = z4 * -(3196i32 as crate::jpegint_h::JLONG);
        z3 += z5;
        z4 += z5;
        *dataptr.offset(7) =
            (tmp4 + z1 + z3 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 - 2i32 - 1i32)
                >> 13i32 - 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5) =
            (tmp5 + z2 + z4 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 - 2i32 - 1i32)
                >> 13i32 - 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3) =
            (tmp6 + z2 + z3 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 - 2i32 - 1i32)
                >> 13i32 - 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(1) =
            (tmp7 + z1 + z4 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 - 2i32 - 1i32)
                >> 13i32 - 2i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(crate::jpeglib_h::DCTSIZE as isize);
        ctr -= 1
    }
    /* Even part per LL&M figure 1 --- note that published figure is faulty;
     * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
     */
    /* Odd part per figure 8 --- note paper omits factor of sqrt(2).
     * cK represents cos(K*pi/16).
     * i0..i3 in the paper are tmp4..tmp7 here.
     */
    /* sqrt(2) * c3 */
    /* sqrt(2) * (-c1+c3+c5-c7) */
    /* sqrt(2) * ( c1+c3-c5+c7) */
    /* sqrt(2) * ( c1+c3+c5-c7) */
    /* sqrt(2) * ( c1+c3-c5-c7) */
    /* sqrt(2) * ( c7-c3) */
    /* sqrt(2) * (-c1-c3) */
    /* sqrt(2) * (-c3-c5) */
    /* sqrt(2) * ( c5-c3) */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     */
    dataptr = data;
    ctr = crate::jpeglib_h::DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp7 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp1 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp6 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp2 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp5 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp3 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        tmp4 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) as libc::c_int)
            as crate::jpegint_h::JLONG;
        /* advance pointer to next column */
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) =
            (tmp10 + tmp11 + ((1i32 as crate::jpegint_h::JLONG) << 2i32 - 1i32) >> 2i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) =
            (tmp10 - tmp11 + ((1i32 as crate::jpegint_h::JLONG) << 2i32 - 1i32) >> 2i32)
                as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433i32 as crate::jpegint_h::JLONG;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) =
            (z1 + tmp13 * 6270i32 as crate::jpegint_h::JLONG
                + ((1i32 as crate::jpegint_h::JLONG) << 13i32 + 2i32 - 1i32)
                >> 13i32 + 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) =
            (z1 + tmp12 * -(15137i32 as crate::jpegint_h::JLONG)
                + ((1i32 as crate::jpegint_h::JLONG) << 13i32 + 2i32 - 1i32)
                >> 13i32 + 2i32) as crate::jdct_h::DCTELEM;
        z1 = tmp4 + tmp7;
        z2 = tmp5 + tmp6;
        z3 = tmp4 + tmp6;
        z4 = tmp5 + tmp7;
        z5 = (z3 + z4) * 9633i32 as crate::jpegint_h::JLONG;
        tmp4 = tmp4 * 2446i32 as crate::jpegint_h::JLONG;
        tmp5 = tmp5 * 16819i32 as crate::jpegint_h::JLONG;
        tmp6 = tmp6 * 25172i32 as crate::jpegint_h::JLONG;
        tmp7 = tmp7 * 12299i32 as crate::jpegint_h::JLONG;
        z1 = z1 * -(7373i32 as crate::jpegint_h::JLONG);
        z2 = z2 * -(20995i32 as crate::jpegint_h::JLONG);
        z3 = z3 * -(16069i32 as crate::jpegint_h::JLONG);
        z4 = z4 * -(3196i32 as crate::jpegint_h::JLONG);
        z3 += z5;
        z4 += z5;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) =
            (tmp4 + z1 + z3 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 + 2i32 - 1i32)
                >> 13i32 + 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) =
            (tmp5 + z2 + z4 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 + 2i32 - 1i32)
                >> 13i32 + 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) =
            (tmp6 + z2 + z3 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 + 2i32 - 1i32)
                >> 13i32 + 2i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) =
            (tmp7 + z1 + z4 + ((1i32 as crate::jpegint_h::JLONG) << 13i32 + 2i32 - 1i32)
                >> 13i32 + 2i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Even part per LL&M figure 1 --- note that published figure is faulty;
 * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
 */
/* Odd part per figure 8 --- note paper omits factor of sqrt(2).
 * cK represents cos(K*pi/16).
 * i0..i3 in the paper are tmp4..tmp7 here.
 */
/* sqrt(2) * c3 */
/* sqrt(2) * (-c1+c3+c5-c7) */
/* sqrt(2) * ( c1+c3-c5+c7) */
/* sqrt(2) * ( c1+c3+c5-c7) */
/* sqrt(2) * ( c1+c3-c5-c7) */
/* sqrt(2) * ( c7-c3) */
/* sqrt(2) * (-c1-c3) */
/* sqrt(2) * (-c3-c5) */
/* sqrt(2) * ( c5-c3) */
/* DCT_ISLOW_SUPPORTED */
