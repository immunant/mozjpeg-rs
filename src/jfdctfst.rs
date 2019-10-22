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
/*
 * jfdctfst.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a fast, not so accurate integer implementation of the
 * forward DCT (Discrete Cosine Transform).
 *
 * A 2-D DCT can be done by 1-D DCT on each row followed by 1-D DCT
 * on each column.  Direct algorithms are also available, but they are
 * much more complex and seem not to be any faster when reduced to code.
 *
 * This implementation is based on Arai, Agui, and Nakajima's algorithm for
 * scaled DCT.  Their original paper (Trans. IEICE E-71(11):1095) is in
 * Japanese, but the algorithm is described in the Pennebaker & Mitchell
 * JPEG textbook (see REFERENCES section in file README.ijg).  The following
 * code is based directly on figure 4-8 in P&M.
 * While an 8-point DCT cannot be done in less than 11 multiplies, it is
 * possible to arrange the computation so that many of the multiplies are
 * simple scalings of the final outputs.  These multiplies can then be
 * folded into the multiplications or divisions by the JPEG quantization
 * table entries.  The AA&N method leaves only 5 multiplies and 29 adds
 * to be done in the DCT itself.
 * The primary disadvantage of this method is that with fixed-point math,
 * accuracy is lost due to imprecise representation of the scaled
 * quantization values.  The smaller the quantization table entry, the less
 * precise the scaled value, so this implementation does worse with high-
 * quality-setting files than with low-quality ones.
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/* Scaling decisions are generally the same as in the LL&M algorithm;
 * see jfdctint.c for more details.  However, we choose to descale
 * (right shift) multiplication products as soon as they are formed,
 * rather than carrying additional fractional bits into subsequent additions.
 * This compromises accuracy slightly, but it lets us save a few shifts.
 * More importantly, 16-bit arithmetic is then adequate (for 8-bit samples)
 * everywhere except in the multiplications proper; this saves a good deal
 * of work on 16-bit-int machines.
 *
 * Again to save a few shifts, the intermediate results between pass 1 and
 * pass 2 are not upscaled, but are represented only to integral precision.
 *
 * A final compromise is to represent the multiplicative constants to only
 * 8 fractional bits, rather than 13.  This saves some shifting work on some
 * machines, and may also reduce the cost of multiplication (since there
 * are fewer one-bits in the constants).
 */
/* Some C compilers fail to reduce "FIX(constant)" at compile time, thus
 * causing a lot of useless floating-point operations at run time.
 * To get around this we use the following pre-calculated constants.
 * If you change CONST_BITS you may want to add appropriate values.
 * (With a reasonable C compiler, you can just rely on the FIX() macro...)
 */
/* FIX(0.382683433) */
/* FIX(0.541196100) */
/* FIX(0.707106781) */
/* FIX(1.306562965) */
/* We can gain a little more speed, with a further compromise in accuracy,
 * by omitting the addition in a descaling shift.  This yields an incorrectly
 * rounded result half the time...
 */
/* Multiply a DCTELEM variable by an JLONG constant, and immediately
 * descale to yield a DCTELEM result.
 */
/*
 * Perform the forward DCT on one block of samples.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_ifast(mut data: *mut crate::jdct_h::DCTELEM) {
    let mut tmp0: crate::jdct_h::DCTELEM = 0;
    let mut tmp1: crate::jdct_h::DCTELEM = 0;
    let mut tmp2: crate::jdct_h::DCTELEM = 0;
    let mut tmp3: crate::jdct_h::DCTELEM = 0;
    let mut tmp4: crate::jdct_h::DCTELEM = 0;
    let mut tmp5: crate::jdct_h::DCTELEM = 0;
    let mut tmp6: crate::jdct_h::DCTELEM = 0;
    let mut tmp7: crate::jdct_h::DCTELEM = 0;
    let mut tmp10: crate::jdct_h::DCTELEM = 0;
    let mut tmp11: crate::jdct_h::DCTELEM = 0;
    let mut tmp12: crate::jdct_h::DCTELEM = 0;
    let mut tmp13: crate::jdct_h::DCTELEM = 0;
    let mut z1: crate::jdct_h::DCTELEM = 0;
    let mut z2: crate::jdct_h::DCTELEM = 0;
    let mut z3: crate::jdct_h::DCTELEM = 0;
    let mut z4: crate::jdct_h::DCTELEM = 0;
    let mut z5: crate::jdct_h::DCTELEM = 0;
    let mut z11: crate::jdct_h::DCTELEM = 0;
    let mut z13: crate::jdct_h::DCTELEM = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut ctr: libc::c_int = 0;
    /* Pass 1: process rows. */
    dataptr = data;
    ctr = crate::jpeglib_h::DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 = (*dataptr.offset(0) as libc::c_int + *dataptr.offset(7) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp7 = (*dataptr.offset(0) as libc::c_int - *dataptr.offset(7) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp1 = (*dataptr.offset(1) as libc::c_int + *dataptr.offset(6) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp6 = (*dataptr.offset(1) as libc::c_int - *dataptr.offset(6) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp2 = (*dataptr.offset(2) as libc::c_int + *dataptr.offset(5) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp5 = (*dataptr.offset(2) as libc::c_int - *dataptr.offset(5) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp3 = (*dataptr.offset(3) as libc::c_int + *dataptr.offset(4) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp4 = (*dataptr.offset(3) as libc::c_int - *dataptr.offset(4) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next row */
        tmp10 = (tmp0 as libc::c_int + tmp3 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp13 = (tmp0 as libc::c_int - tmp3 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp11 = (tmp1 as libc::c_int + tmp2 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp12 = (tmp1 as libc::c_int - tmp2 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset(0) =
            (tmp10 as libc::c_int + tmp11 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset(4) =
            (tmp10 as libc::c_int - tmp11 as libc::c_int) as crate::jdct_h::DCTELEM;
        z1 = ((tmp12 as libc::c_int + tmp13 as libc::c_int) as libc::c_long
            * 181i32 as crate::jpegint_h::JLONG
            >> 8i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(2) = (tmp13 as libc::c_int + z1 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset(6) = (tmp13 as libc::c_int - z1 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp10 = (tmp4 as libc::c_int + tmp5 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp11 = (tmp5 as libc::c_int + tmp6 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp12 = (tmp6 as libc::c_int + tmp7 as libc::c_int) as crate::jdct_h::DCTELEM;
        z5 = ((tmp10 as libc::c_int - tmp12 as libc::c_int) as libc::c_long
            * 98i32 as crate::jpegint_h::JLONG
            >> 8i32) as crate::jdct_h::DCTELEM;
        z2 = ((tmp10 as libc::c_long * 139i32 as crate::jpegint_h::JLONG >> 8i32)
            as crate::jdct_h::DCTELEM as libc::c_int
            + z5 as libc::c_int) as crate::jdct_h::DCTELEM;
        z4 = ((tmp12 as libc::c_long * 334i32 as crate::jpegint_h::JLONG >> 8i32)
            as crate::jdct_h::DCTELEM as libc::c_int
            + z5 as libc::c_int) as crate::jdct_h::DCTELEM;
        z3 = (tmp11 as libc::c_long * 181i32 as crate::jpegint_h::JLONG >> 8i32)
            as crate::jdct_h::DCTELEM;
        z11 = (tmp7 as libc::c_int + z3 as libc::c_int) as crate::jdct_h::DCTELEM;
        z13 = (tmp7 as libc::c_int - z3 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5) = (z13 as libc::c_int + z2 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3) = (z13 as libc::c_int - z2 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset(1) = (z11 as libc::c_int + z4 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7) = (z11 as libc::c_int - z4 as libc::c_int) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(crate::jpeglib_h::DCTSIZE as isize);
        ctr -= 1
    }
    /* Even part */
    /* phase 2 */
    /* phase 3 */
    /* c4 */
    /* phase 5 */
    /* Odd part */
    /* phase 2 */
    /* The rotator is modified from fig 4-8 to avoid extra negations. */
    /* c6 */
    /* c2-c6 */
    /* c2+c6 */
    /* c4 */
    /* phase 5 */
    /* phase 6 */
    /* Pass 2: process columns. */
    dataptr = data;
    ctr = crate::jpeglib_h::DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp7 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp1 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp6 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp2 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp5 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp3 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) as libc::c_int
            + *dataptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        tmp4 = (*dataptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) as libc::c_int
            - *dataptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) as libc::c_int)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp0 as libc::c_int + tmp3 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp13 = (tmp0 as libc::c_int - tmp3 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp11 = (tmp1 as libc::c_int + tmp2 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp12 = (tmp1 as libc::c_int - tmp2 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 0i32) as isize) =
            (tmp10 as libc::c_int + tmp11 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 4i32) as isize) =
            (tmp10 as libc::c_int - tmp11 as libc::c_int) as crate::jdct_h::DCTELEM;
        z1 = ((tmp12 as libc::c_int + tmp13 as libc::c_int) as libc::c_long
            * 181i32 as crate::jpegint_h::JLONG
            >> 8i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 2i32) as isize) =
            (tmp13 as libc::c_int + z1 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 6i32) as isize) =
            (tmp13 as libc::c_int - z1 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp10 = (tmp4 as libc::c_int + tmp5 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp11 = (tmp5 as libc::c_int + tmp6 as libc::c_int) as crate::jdct_h::DCTELEM;
        tmp12 = (tmp6 as libc::c_int + tmp7 as libc::c_int) as crate::jdct_h::DCTELEM;
        z5 = ((tmp10 as libc::c_int - tmp12 as libc::c_int) as libc::c_long
            * 98i32 as crate::jpegint_h::JLONG
            >> 8i32) as crate::jdct_h::DCTELEM;
        z2 = ((tmp10 as libc::c_long * 139i32 as crate::jpegint_h::JLONG >> 8i32)
            as crate::jdct_h::DCTELEM as libc::c_int
            + z5 as libc::c_int) as crate::jdct_h::DCTELEM;
        z4 = ((tmp12 as libc::c_long * 334i32 as crate::jpegint_h::JLONG >> 8i32)
            as crate::jdct_h::DCTELEM as libc::c_int
            + z5 as libc::c_int) as crate::jdct_h::DCTELEM;
        z3 = (tmp11 as libc::c_long * 181i32 as crate::jpegint_h::JLONG >> 8i32)
            as crate::jdct_h::DCTELEM;
        z11 = (tmp7 as libc::c_int + z3 as libc::c_int) as crate::jdct_h::DCTELEM;
        z13 = (tmp7 as libc::c_int - z3 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 5i32) as isize) =
            (z13 as libc::c_int + z2 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 3i32) as isize) =
            (z13 as libc::c_int - z2 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 1i32) as isize) =
            (z11 as libc::c_int + z4 as libc::c_int) as crate::jdct_h::DCTELEM;
        *dataptr.offset((crate::jpeglib_h::DCTSIZE * 7i32) as isize) =
            (z11 as libc::c_int - z4 as libc::c_int) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Even part */
/* phase 2 */
/* phase 3 */
/* c4 */
/* phase 5 */
/* Odd part */
/* phase 2 */
/* The rotator is modified from fig 4-8 to avoid extra negations. */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* c4 */
/* phase 5 */
/* phase 6 */
/* DCT_IFAST_SUPPORTED */
