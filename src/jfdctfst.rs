pub use crate::jdct_h::DCTELEM;
pub use crate::jpegint_h::JLONG;
pub use crate::jpeglib_h::DCTSIZE;
use libc;
use libc::c_int;
use libc::c_long;
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
pub unsafe extern "C" fn jpeg_fdct_ifast(mut data: *mut DCTELEM) {
    let mut tmp0: DCTELEM = 0;
    let mut tmp1: DCTELEM = 0;
    let mut tmp2: DCTELEM = 0;
    let mut tmp3: DCTELEM = 0;
    let mut tmp4: DCTELEM = 0;
    let mut tmp5: DCTELEM = 0;
    let mut tmp6: DCTELEM = 0;
    let mut tmp7: DCTELEM = 0;
    let mut tmp10: DCTELEM = 0;
    let mut tmp11: DCTELEM = 0;
    let mut tmp12: DCTELEM = 0;
    let mut tmp13: DCTELEM = 0;
    let mut z1: DCTELEM = 0;
    let mut z2: DCTELEM = 0;
    let mut z3: DCTELEM = 0;
    let mut z4: DCTELEM = 0;
    let mut z5: DCTELEM = 0;
    let mut z11: DCTELEM = 0;
    let mut z13: DCTELEM = 0;
    let mut dataptr: *mut DCTELEM = 0 as *mut DCTELEM;
    let mut ctr: c_int = 0;
    dataptr = data;
    ctr = DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 = (*dataptr.offset(0isize) as c_int + *dataptr.offset(7isize) as c_int) as DCTELEM;
        tmp7 = (*dataptr.offset(0isize) as c_int - *dataptr.offset(7isize) as c_int) as DCTELEM;
        tmp1 = (*dataptr.offset(1isize) as c_int + *dataptr.offset(6isize) as c_int) as DCTELEM;
        tmp6 = (*dataptr.offset(1isize) as c_int - *dataptr.offset(6isize) as c_int) as DCTELEM;
        tmp2 = (*dataptr.offset(2isize) as c_int + *dataptr.offset(5isize) as c_int) as DCTELEM;
        tmp5 = (*dataptr.offset(2isize) as c_int - *dataptr.offset(5isize) as c_int) as DCTELEM;
        tmp3 = (*dataptr.offset(3isize) as c_int + *dataptr.offset(4isize) as c_int) as DCTELEM;
        tmp4 = (*dataptr.offset(3isize) as c_int - *dataptr.offset(4isize) as c_int) as DCTELEM;
        tmp10 = (tmp0 as c_int + tmp3 as c_int) as DCTELEM;
        tmp13 = (tmp0 as c_int - tmp3 as c_int) as DCTELEM;
        tmp11 = (tmp1 as c_int + tmp2 as c_int) as DCTELEM;
        tmp12 = (tmp1 as c_int - tmp2 as c_int) as DCTELEM;
        *dataptr.offset(0isize) = (tmp10 as c_int + tmp11 as c_int) as DCTELEM;
        *dataptr.offset(4isize) = (tmp10 as c_int - tmp11 as c_int) as DCTELEM;
        z1 = ((tmp12 as c_int + tmp13 as c_int) as c_long * 181i32 as JLONG >> 8i32) as DCTELEM;
        *dataptr.offset(2isize) = (tmp13 as c_int + z1 as c_int) as DCTELEM;
        *dataptr.offset(6isize) = (tmp13 as c_int - z1 as c_int) as DCTELEM;
        tmp10 = (tmp4 as c_int + tmp5 as c_int) as DCTELEM;
        tmp11 = (tmp5 as c_int + tmp6 as c_int) as DCTELEM;
        tmp12 = (tmp6 as c_int + tmp7 as c_int) as DCTELEM;
        z5 = ((tmp10 as c_int - tmp12 as c_int) as c_long * 98i32 as JLONG >> 8i32) as DCTELEM;
        z2 = ((tmp10 as c_long * 139i32 as JLONG >> 8i32) as DCTELEM as c_int + z5 as c_int)
            as DCTELEM;
        z4 = ((tmp12 as c_long * 334i32 as JLONG >> 8i32) as DCTELEM as c_int + z5 as c_int)
            as DCTELEM;
        z3 = (tmp11 as c_long * 181i32 as JLONG >> 8i32) as DCTELEM;
        z11 = (tmp7 as c_int + z3 as c_int) as DCTELEM;
        z13 = (tmp7 as c_int - z3 as c_int) as DCTELEM;
        *dataptr.offset(5isize) = (z13 as c_int + z2 as c_int) as DCTELEM;
        *dataptr.offset(3isize) = (z13 as c_int - z2 as c_int) as DCTELEM;
        *dataptr.offset(1isize) = (z11 as c_int + z4 as c_int) as DCTELEM;
        *dataptr.offset(7isize) = (z11 as c_int - z4 as c_int) as DCTELEM;
        dataptr = dataptr.offset(DCTSIZE as isize);
        ctr -= 1
    }
    dataptr = data;
    ctr = DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 = (*dataptr.offset((DCTSIZE * 0i32) as isize) as c_int
            + *dataptr.offset((DCTSIZE * 7i32) as isize) as c_int) as DCTELEM;
        tmp7 = (*dataptr.offset((DCTSIZE * 0i32) as isize) as c_int
            - *dataptr.offset((DCTSIZE * 7i32) as isize) as c_int) as DCTELEM;
        tmp1 = (*dataptr.offset((DCTSIZE * 1i32) as isize) as c_int
            + *dataptr.offset((DCTSIZE * 6i32) as isize) as c_int) as DCTELEM;
        tmp6 = (*dataptr.offset((DCTSIZE * 1i32) as isize) as c_int
            - *dataptr.offset((DCTSIZE * 6i32) as isize) as c_int) as DCTELEM;
        tmp2 = (*dataptr.offset((DCTSIZE * 2i32) as isize) as c_int
            + *dataptr.offset((DCTSIZE * 5i32) as isize) as c_int) as DCTELEM;
        tmp5 = (*dataptr.offset((DCTSIZE * 2i32) as isize) as c_int
            - *dataptr.offset((DCTSIZE * 5i32) as isize) as c_int) as DCTELEM;
        tmp3 = (*dataptr.offset((DCTSIZE * 3i32) as isize) as c_int
            + *dataptr.offset((DCTSIZE * 4i32) as isize) as c_int) as DCTELEM;
        tmp4 = (*dataptr.offset((DCTSIZE * 3i32) as isize) as c_int
            - *dataptr.offset((DCTSIZE * 4i32) as isize) as c_int) as DCTELEM;
        tmp10 = (tmp0 as c_int + tmp3 as c_int) as DCTELEM;
        tmp13 = (tmp0 as c_int - tmp3 as c_int) as DCTELEM;
        tmp11 = (tmp1 as c_int + tmp2 as c_int) as DCTELEM;
        tmp12 = (tmp1 as c_int - tmp2 as c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 0i32) as isize) = (tmp10 as c_int + tmp11 as c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 4i32) as isize) = (tmp10 as c_int - tmp11 as c_int) as DCTELEM;
        z1 = ((tmp12 as c_int + tmp13 as c_int) as c_long * 181i32 as JLONG >> 8i32) as DCTELEM;
        *dataptr.offset((DCTSIZE * 2i32) as isize) = (tmp13 as c_int + z1 as c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 6i32) as isize) = (tmp13 as c_int - z1 as c_int) as DCTELEM;
        tmp10 = (tmp4 as c_int + tmp5 as c_int) as DCTELEM;
        tmp11 = (tmp5 as c_int + tmp6 as c_int) as DCTELEM;
        tmp12 = (tmp6 as c_int + tmp7 as c_int) as DCTELEM;
        z5 = ((tmp10 as c_int - tmp12 as c_int) as c_long * 98i32 as JLONG >> 8i32) as DCTELEM;
        z2 = ((tmp10 as c_long * 139i32 as JLONG >> 8i32) as DCTELEM as c_int + z5 as c_int)
            as DCTELEM;
        z4 = ((tmp12 as c_long * 334i32 as JLONG >> 8i32) as DCTELEM as c_int + z5 as c_int)
            as DCTELEM;
        z3 = (tmp11 as c_long * 181i32 as JLONG >> 8i32) as DCTELEM;
        z11 = (tmp7 as c_int + z3 as c_int) as DCTELEM;
        z13 = (tmp7 as c_int - z3 as c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 5i32) as isize) = (z13 as c_int + z2 as c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 3i32) as isize) = (z13 as c_int - z2 as c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 1i32) as isize) = (z11 as c_int + z4 as c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 7i32) as isize) = (z11 as c_int - z4 as c_int) as DCTELEM;
        dataptr = dataptr.offset(1isize);
        ctr -= 1
    }
}
