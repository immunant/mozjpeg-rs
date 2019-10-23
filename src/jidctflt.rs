pub use crate::jdct_h::{FLOAT_MULT_TYPE, RANGE_MASK};
pub use crate::jmorecfg_h::{
    boolean, CENTERJSAMPLE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAXJSAMPLE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, J_BUF_MODE,
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
use libc::{self, c_float, c_int};
/*
 * jidctflt.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * Modified 2010 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2014, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a floating-point implementation of the
 * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
 * must also perform dequantization of the input coefficients.
 *
 * This implementation should be more accurate than either of the integer
 * IDCT implementations.  However, it may not give the same results on all
 * machines because of differences in roundoff behavior.  Speed will depend
 * on the hardware's floating point capacity.
 *
 * A 2-D IDCT can be done by 1-D IDCT on each column followed by 1-D IDCT
 * on each row (or vice versa, but it's more convenient to emit a row at
 * a time).  Direct algorithms are also available, but they are much more
 * complex and seem not to be any faster when reduced to code.
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
/* Dequantize a coefficient by multiplying it by the multiplier-table
 * entry; produce a float result.
 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_float(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    /* buffers data between passes */

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
    let mut z5: c_float = 0.;
    let mut z10: c_float = 0.;
    let mut z11: c_float = 0.;
    let mut z12: c_float = 0.;
    let mut z13: c_float = 0.;
    let mut workspace: [c_float; 64] = [0.; 64];
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;

    let mut inptr: JCOEFPTR = coef_block;
    let mut quantptr: *mut FLOAT_MULT_TYPE = (*compptr).dct_table as *mut FLOAT_MULT_TYPE;
    let mut wsptr: *mut c_float = workspace.as_mut_ptr();
    let mut ctr: c_int = DCTSIZE;
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
            let mut dcval: c_float = *inptr.offset((8i32 * 0i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 0i32) as isize) * 0.125f32); /* advance pointers to next column */
            *wsptr.offset((DCTSIZE * 0i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 1i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 2i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 3i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 4i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 5i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 6i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 7i32) as isize) = dcval;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        } else {
            /* Even part */
            tmp0 = *inptr.offset((8i32 * 0i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 0i32) as isize) * 0.125f32); /* phase 3 */
            tmp1 = *inptr.offset((8i32 * 2i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 2i32) as isize) * 0.125f32); /* phases 5-3 */
            tmp2 = *inptr.offset((8i32 * 4i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 4i32) as isize) * 0.125f32); /* 2*c4 */
            tmp3 = *inptr.offset((8i32 * 6i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 6i32) as isize) * 0.125f32); /* phase 2 */
            tmp10 = tmp0 + tmp2;
            tmp11 = tmp0 - tmp2;
            tmp13 = tmp1 + tmp3;
            tmp12 = (tmp1 - tmp3) * 1.414213562f32 - tmp13;
            tmp0 = tmp10 + tmp13;
            tmp3 = tmp10 - tmp13;
            tmp1 = tmp11 + tmp12;
            tmp2 = tmp11 - tmp12;
            /* Odd part */
            tmp4 = *inptr.offset((8i32 * 1i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 1i32) as isize) * 0.125f32); /* phase 6 */
            tmp5 = *inptr.offset((8i32 * 3i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 3i32) as isize) * 0.125f32); /* phase 5 */
            tmp6 = *inptr.offset((8i32 * 5i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 5i32) as isize) * 0.125f32); /* 2*c4 */
            tmp7 = *inptr.offset((8i32 * 7i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 7i32) as isize) * 0.125f32); /* 2*c2 */
            z13 = tmp6 + tmp5; /* 2*(c2-c6) */
            z10 = tmp6 - tmp5; /* 2*(c2+c6) */
            z11 = tmp4 + tmp7; /* phase 2 */
            z12 = tmp4 - tmp7; /* advance pointers to next column */
            tmp7 = z11 + z13;
            tmp11 = (z11 - z13) * 1.414213562f32;
            z5 = (z10 + z12) * 1.847759065f32;
            tmp10 = z5 - z12 * 1.0823922f32;
            tmp12 = z5 - z10 * 2.61312593f32;
            tmp6 = tmp12 - tmp7;
            tmp5 = tmp11 - tmp6;
            tmp4 = tmp10 - tmp5;
            *wsptr.offset((DCTSIZE * 0i32) as isize) = tmp0 + tmp7;
            *wsptr.offset((DCTSIZE * 7i32) as isize) = tmp0 - tmp7;
            *wsptr.offset((DCTSIZE * 1i32) as isize) = tmp1 + tmp6;
            *wsptr.offset((DCTSIZE * 6i32) as isize) = tmp1 - tmp6;
            *wsptr.offset((DCTSIZE * 2i32) as isize) = tmp2 + tmp5;
            *wsptr.offset((DCTSIZE * 5i32) as isize) = tmp2 - tmp5;
            *wsptr.offset((DCTSIZE * 3i32) as isize) = tmp3 + tmp4;
            *wsptr.offset((DCTSIZE * 4i32) as isize) = tmp3 - tmp4;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        }
        ctr -= 1
    }
    /* Pass 2: process rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < DCTSIZE {
        let mut outptr: JSAMPROW = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z5 = *wsptr.offset(0) + (CENTERJSAMPLE as c_float + 0.5f32);
        tmp10 = z5 + *wsptr.offset(4);
        tmp11 = z5 - *wsptr.offset(4);
        tmp13 = *wsptr.offset(2) + *wsptr.offset(6);
        tmp12 = (*wsptr.offset(2) - *wsptr.offset(6)) * 1.414213562f32 - tmp13;
        tmp0 = tmp10 + tmp13;
        tmp3 = tmp10 - tmp13;
        tmp1 = tmp11 + tmp12;
        tmp2 = tmp11 - tmp12;
        z13 = *wsptr.offset(5) + *wsptr.offset(3);
        z10 = *wsptr.offset(5) - *wsptr.offset(3);
        z11 = *wsptr.offset(1) + *wsptr.offset(7);
        z12 = *wsptr.offset(1) - *wsptr.offset(7);
        tmp7 = z11 + z13;
        tmp11 = (z11 - z13) * 1.414213562f32;
        z5 = (z10 + z12) * 1.847759065f32;
        tmp10 = z5 - z12 * 1.0823922f32;
        tmp12 = z5 - z10 * 2.61312593f32;
        tmp6 = tmp12 - tmp7;
        tmp5 = tmp11 - tmp6;
        tmp4 = tmp10 - tmp5;
        *outptr.offset(0) = *range_limit.offset(((tmp0 + tmp7) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7) = *range_limit.offset(((tmp0 - tmp7) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1) = *range_limit.offset(((tmp1 + tmp6) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6) = *range_limit.offset(((tmp1 - tmp6) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2) = *range_limit.offset(((tmp2 + tmp5) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5) = *range_limit.offset(((tmp2 - tmp5) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3) = *range_limit.offset(((tmp3 + tmp4) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4) = *range_limit.offset(((tmp3 - tmp4) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(DCTSIZE as isize);
        ctr += 1
    }
}
/* Rows of zeroes can be exploited in the same way as we did with columns.
 * However, the column calculation has created many nonzero AC terms, so
 * the simplification applies less often (typically 5% to 10% of the time).
 * And testing floats for zero is relatively expensive, so we don't bother.
 */
/* Even part */
/* Apply signed->unsigned and prepare float->int conversion */
/* Odd part */
/* 2*c2 */
/* 2*(c2-c6) */
/* 2*(c2+c6) */
/* Final output stage: float->int conversion and range-limit */
/* DCT_FLOAT_SUPPORTED */
