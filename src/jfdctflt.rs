pub use crate::jpeglib_h::DCTSIZE;
use libc::{self, c_float, c_int};
/*
 * jfdctflt.c
 *
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a floating-point implementation of the
 * forward DCT (Discrete Cosine Transform).
 *
 * This implementation should be more accurate than either of the integer
 * DCT implementations.  However, it may not give the same results on all
 * machines because of differences in roundoff behavior.  Speed will depend
 * on the hardware's floating point capacity.
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
 * The primary disadvantage of this method is that with a fixed-point
 * implementation, accuracy is lost due to imprecise representation of the
 * scaled quantization values.  However, that problem does not arise if
 * we use floating point arithmetic.
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/*
 * Perform the forward DCT on one block of samples.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_fdct_float(mut data: *mut c_float) {
    let mut tmp0: c_float = 0.;
    let mut tmp1: c_float = 0.;
    let mut tmp2: c_float = 0.;
    let mut tmp3: c_float = 0.;
    let mut tmp4: c_float = 0.;
    let mut tmp5: c_float = 0.;
    let mut tmp6: c_float = 0.;
    let mut tmp7: c_float = 0.;
    let mut tmp10: c_float = 0.;
    let mut tmp11: c_float = 0.;
    let mut tmp12: c_float = 0.;
    let mut tmp13: c_float = 0.;
    let mut z1: c_float = 0.;
    let mut z2: c_float = 0.;
    let mut z3: c_float = 0.;
    let mut z4: c_float = 0.;
    let mut z5: c_float = 0.;
    let mut z11: c_float = 0.;
    let mut z13: c_float = 0.;
    let mut dataptr: *mut c_float = 0 as *mut c_float;
    let mut ctr: c_int = 0;
    dataptr = data;
    ctr = DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 = *dataptr.offset(0isize) + *dataptr.offset(7isize);
        tmp7 = *dataptr.offset(0isize) - *dataptr.offset(7isize);
        tmp1 = *dataptr.offset(1isize) + *dataptr.offset(6isize);
        tmp6 = *dataptr.offset(1isize) - *dataptr.offset(6isize);
        tmp2 = *dataptr.offset(2isize) + *dataptr.offset(5isize);
        tmp5 = *dataptr.offset(2isize) - *dataptr.offset(5isize);
        tmp3 = *dataptr.offset(3isize) + *dataptr.offset(4isize);
        tmp4 = *dataptr.offset(3isize) - *dataptr.offset(4isize);
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset(0isize) = tmp10 + tmp11;
        *dataptr.offset(4isize) = tmp10 - tmp11;
        z1 = (tmp12 + tmp13) * 0.707106781f64 as c_float;
        *dataptr.offset(2isize) = tmp13 + z1;
        *dataptr.offset(6isize) = tmp13 - z1;
        tmp10 = tmp4 + tmp5;
        tmp11 = tmp5 + tmp6;
        tmp12 = tmp6 + tmp7;
        z5 = (tmp10 - tmp12) * 0.382683433f64 as c_float;
        z2 = 0.541196100f64 as c_float * tmp10 + z5;
        z4 = 1.306562965f64 as c_float * tmp12 + z5;
        z3 = tmp11 * 0.707106781f64 as c_float;
        z11 = tmp7 + z3;
        z13 = tmp7 - z3;
        *dataptr.offset(5isize) = z13 + z2;
        *dataptr.offset(3isize) = z13 - z2;
        *dataptr.offset(1isize) = z11 + z4;
        *dataptr.offset(7isize) = z11 - z4;
        dataptr = dataptr.offset(DCTSIZE as isize);
        ctr -= 1
    }
    dataptr = data;
    ctr = DCTSIZE - 1i32;
    while ctr >= 0i32 {
        tmp0 =
            *dataptr.offset((DCTSIZE * 0i32) as isize) + *dataptr.offset((DCTSIZE * 7i32) as isize);
        tmp7 =
            *dataptr.offset((DCTSIZE * 0i32) as isize) - *dataptr.offset((DCTSIZE * 7i32) as isize);
        tmp1 =
            *dataptr.offset((DCTSIZE * 1i32) as isize) + *dataptr.offset((DCTSIZE * 6i32) as isize);
        tmp6 =
            *dataptr.offset((DCTSIZE * 1i32) as isize) - *dataptr.offset((DCTSIZE * 6i32) as isize);
        tmp2 =
            *dataptr.offset((DCTSIZE * 2i32) as isize) + *dataptr.offset((DCTSIZE * 5i32) as isize);
        tmp5 =
            *dataptr.offset((DCTSIZE * 2i32) as isize) - *dataptr.offset((DCTSIZE * 5i32) as isize);
        tmp3 =
            *dataptr.offset((DCTSIZE * 3i32) as isize) + *dataptr.offset((DCTSIZE * 4i32) as isize);
        tmp4 =
            *dataptr.offset((DCTSIZE * 3i32) as isize) - *dataptr.offset((DCTSIZE * 4i32) as isize);
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset((DCTSIZE * 0i32) as isize) = tmp10 + tmp11;
        *dataptr.offset((DCTSIZE * 4i32) as isize) = tmp10 - tmp11;
        z1 = (tmp12 + tmp13) * 0.707106781f64 as c_float;
        *dataptr.offset((DCTSIZE * 2i32) as isize) = tmp13 + z1;
        *dataptr.offset((DCTSIZE * 6i32) as isize) = tmp13 - z1;
        tmp10 = tmp4 + tmp5;
        tmp11 = tmp5 + tmp6;
        tmp12 = tmp6 + tmp7;
        z5 = (tmp10 - tmp12) * 0.382683433f64 as c_float;
        z2 = 0.541196100f64 as c_float * tmp10 + z5;
        z4 = 1.306562965f64 as c_float * tmp12 + z5;
        z3 = tmp11 * 0.707106781f64 as c_float;
        z11 = tmp7 + z3;
        z13 = tmp7 - z3;
        *dataptr.offset((DCTSIZE * 5i32) as isize) = z13 + z2;
        *dataptr.offset((DCTSIZE * 3i32) as isize) = z13 - z2;
        *dataptr.offset((DCTSIZE * 1i32) as isize) = z11 + z4;
        *dataptr.offset((DCTSIZE * 7i32) as isize) = z11 - z4;
        dataptr = dataptr.offset(1isize);
        ctr -= 1
    }
}
