






















































































use libc::{c_int, c_long, self};pub use crate::jpegint_h::{inverse_DCT_method_ptr, JBUF_CRANK_DEST,
                           JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
                           JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE};pub use crate::jdct_h::{DCTELEM, IFAST_MULT_TYPE, RANGE_MASK};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
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
                           DCTSIZE, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR,
                           JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JQUANT_TBL, JSAMPARRAY,
                           JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
                           J_DITHER_MODE};pub use crate::jmorecfg_h::{boolean, CENTERJSAMPLE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, MAXJSAMPLE, UINT16, UINT8};pub use crate::stddef_h::size_t;
/*
 * jidctfst.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a fast, not so accurate integer implementation of the
 * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
 * must also perform dequantization of the input coefficients.
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
 * see jidctint.c for more details.  However, we choose to descale
 * (right shift) multiplication products as soon as they are formed,
 * rather than carrying additional fractional bits into subsequent additions.
 * This compromises accuracy slightly, but it lets us save a few shifts.
 * More importantly, 16-bit arithmetic is then adequate (for 8-bit samples)
 * everywhere except in the multiplications proper; this saves a good deal
 * of work on 16-bit-int machines.
 *
 * The dequantized coefficients are not integers because the AA&N scaling
 * factors have been incorporated.  We represent them scaled up by PASS1_BITS,
 * so that the first and second IDCT rounds have the same input scaling.
 * For 8-bit JSAMPLEs, we choose IFAST_SCALE_BITS = PASS1_BITS so as to
 * avoid a descaling shift; this compromises accuracy rather drastically
 * for small quantization table entries, but it saves a lot of shifts.
 * For 12-bit JSAMPLEs, there's no hope of using 16x16 multiplies anyway,
 * so we use a much larger scaling factor to preserve accuracy.
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
/* FIX(1.082392200) */
/* FIX(1.414213562) */
/* FIX(1.847759065) */
/* FIX(2.613125930) */
/* We can gain a little more speed, with a further compromise in accuracy,
 * by omitting the addition in a descaling shift.  This yields an incorrectly
 * rounded result half the time...
 */
/* Multiply a DCTELEM variable by an JLONG constant, and immediately
 * descale to yield a DCTELEM result.
 */
/* Dequantize a coefficient by multiplying it by the multiplier-table
 * entry; produce a DCTELEM result.  For 8-bit data a 16x16->16
 * multiplication will do.  For 12-bit data, the multiplier table is
 * declared JLONG, so a 32-bit multiply will be used.
 */
/* Like DESCALE, but applies to a DCTELEM and produces an int.
 * We assume that int right shift is unsigned if JLONG right shift is.
 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_ifast(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
     /* buffers data between passes */
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
     let mut tmp0:  DCTELEM =  0; let mut tmp1:  DCTELEM =  0; let mut tmp2:  DCTELEM =  0; let mut tmp3:  DCTELEM =  0; let mut tmp4:  DCTELEM =  0; let mut tmp5:  DCTELEM =  0; let mut tmp6:  DCTELEM =  0; let mut tmp7:  DCTELEM =  0; let mut tmp10:  DCTELEM =  0; let mut tmp11:  DCTELEM =  0; let mut tmp12:  DCTELEM =  0; let mut tmp13:  DCTELEM =  0; let mut z5:  DCTELEM =  0; let mut z10:  DCTELEM =  0; let mut z11:  DCTELEM =  0; let mut z12:  DCTELEM =  0; let mut z13:  DCTELEM =  0;     let mut workspace:  [c_int; 64] =  [0; 64];
    let mut range_limit: *mut JSAMPLE = (*cinfo)
        .sample_range_limit
        .offset(CENTERJSAMPLE as isize);
    
    
    
    
    
     let mut inptr:   JCOEFPTR =  coef_block; let mut quantptr:   *mut IFAST_MULT_TYPE =
     (*compptr).dct_table as *mut IFAST_MULT_TYPE; let mut wsptr:   *mut c_int =  workspace.as_mut_ptr(); let mut ctr:   c_int =  DCTSIZE;
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
            let mut dcval: c_int = *inptr.offset((8i32 * 0i32) as isize) as c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as c_int; /* advance pointers to next column */
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
            tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as c_int)
                as DCTELEM; /* phase 3 */
            tmp1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
                * *quantptr.offset((8i32 * 2i32) as isize) as c_int)
                as DCTELEM; /* phases 5-3 */
            tmp2 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
                * *quantptr.offset((8i32 * 4i32) as isize) as c_int)
                as DCTELEM; /* 2*c4 */
            tmp3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
                * *quantptr.offset((8i32 * 6i32) as isize) as c_int)
                as DCTELEM; /* phase 2 */
            tmp10 = (tmp0 as c_int + tmp2 as c_int) as DCTELEM;
            tmp11 = (tmp0 as c_int - tmp2 as c_int) as DCTELEM;
            tmp13 = (tmp1 as c_int + tmp3 as c_int) as DCTELEM;
            tmp12 = (((tmp1 as c_int - tmp3 as c_int) as c_long
                * 362i64
                >> 8i32) as DCTELEM as c_int
                - tmp13 as c_int) as DCTELEM;
            tmp0 = (tmp10 as c_int + tmp13 as c_int) as DCTELEM;
            tmp3 = (tmp10 as c_int - tmp13 as c_int) as DCTELEM;
            tmp1 = (tmp11 as c_int + tmp12 as c_int) as DCTELEM;
            tmp2 = (tmp11 as c_int - tmp12 as c_int) as DCTELEM;
            /* Odd part */
            tmp4 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
                * *quantptr.offset((8i32 * 1i32) as isize) as c_int)
                as DCTELEM; /* phase 6 */
            tmp5 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
                * *quantptr.offset((8i32 * 3i32) as isize) as c_int)
                as DCTELEM; /* phase 5 */
            tmp6 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
                * *quantptr.offset((8i32 * 5i32) as isize) as c_int)
                as DCTELEM; /* 2*c4 */
            tmp7 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
                * *quantptr.offset((8i32 * 7i32) as isize) as c_int)
                as DCTELEM; /* 2*c2 */
            z13 = (tmp6 as c_int + tmp5 as c_int) as DCTELEM; /* 2*(c2-c6) */
            z10 = (tmp6 as c_int - tmp5 as c_int) as DCTELEM; /* -2*(c2+c6) */
            z11 = (tmp4 as c_int + tmp7 as c_int) as DCTELEM; /* phase 2 */
            z12 = (tmp4 as c_int - tmp7 as c_int) as DCTELEM; /* advance pointers to next column */
            tmp7 = (z11 as c_int + z13 as c_int) as DCTELEM;
            tmp11 = ((z11 as c_int - z13 as c_int) as c_long
                * 362i64
                >> 8i32) as DCTELEM;
            z5 = ((z10 as c_int + z12 as c_int) as c_long
                * 473i64
                >> 8i32) as DCTELEM;
            tmp10 = ((z12 as c_long * 277i64 >> 8i32)
                as DCTELEM as c_int
                - z5 as c_int) as DCTELEM;
            tmp12 = ((z10 as c_long * -(669i64) >> 8i32)
                as DCTELEM as c_int
                + z5 as c_int) as DCTELEM;
            tmp6 = (tmp12 as c_int - tmp7 as c_int) as DCTELEM;
            tmp5 = (tmp11 as c_int - tmp6 as c_int) as DCTELEM;
            tmp4 = (tmp10 as c_int + tmp5 as c_int) as DCTELEM;
            *wsptr.offset((DCTSIZE * 0i32) as isize) =
                tmp0 as c_int + tmp7 as c_int;
            *wsptr.offset((DCTSIZE * 7i32) as isize) =
                tmp0 as c_int - tmp7 as c_int;
            *wsptr.offset((DCTSIZE * 1i32) as isize) =
                tmp1 as c_int + tmp6 as c_int;
            *wsptr.offset((DCTSIZE * 6i32) as isize) =
                tmp1 as c_int - tmp6 as c_int;
            *wsptr.offset((DCTSIZE * 2i32) as isize) =
                tmp2 as c_int + tmp5 as c_int;
            *wsptr.offset((DCTSIZE * 5i32) as isize) =
                tmp2 as c_int - tmp5 as c_int;
            *wsptr.offset((DCTSIZE * 4i32) as isize) =
                tmp3 as c_int + tmp4 as c_int;
            *wsptr.offset((DCTSIZE * 3i32) as isize) =
                tmp3 as c_int - tmp4 as c_int;
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
    while ctr < DCTSIZE {
          let mut outptr:   JSAMPROW =
     (*output_buf.offset(ctr as isize)).offset(output_col as isize);
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
            let mut dcval_0: JSAMPLE = *range_limit
                .offset((*wsptr.offset(0) >> 2i32 + 3i32 & RANGE_MASK) as isize); /* advance pointer to next row */
            *outptr.offset(0) = dcval_0;
            *outptr.offset(1) = dcval_0;
            *outptr.offset(2) = dcval_0;
            *outptr.offset(3) = dcval_0;
            *outptr.offset(4) = dcval_0;
            *outptr.offset(5) = dcval_0;
            *outptr.offset(6) = dcval_0;
            *outptr.offset(7) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize)
        } else {
            /* Even part */
            tmp10 = (*wsptr.offset(0) as DCTELEM as c_int
                + *wsptr.offset(4) as DCTELEM as c_int)
                as DCTELEM;
            tmp11 = (*wsptr.offset(0) as DCTELEM as c_int
                - *wsptr.offset(4) as DCTELEM as c_int)
                as DCTELEM;
            tmp13 = (*wsptr.offset(2) as DCTELEM as c_int
                + *wsptr.offset(6) as DCTELEM as c_int)
                as DCTELEM;
            tmp12 = (((*wsptr.offset(2) as DCTELEM as c_int
                - *wsptr.offset(6) as DCTELEM as c_int)
                as c_long
                * 362i64
                >> 8i32) as DCTELEM as c_int
                - tmp13 as c_int) as DCTELEM;
            tmp0 = (tmp10 as c_int + tmp13 as c_int) as DCTELEM;
            tmp3 = (tmp10 as c_int - tmp13 as c_int) as DCTELEM;
            tmp1 = (tmp11 as c_int + tmp12 as c_int) as DCTELEM;
            tmp2 = (tmp11 as c_int - tmp12 as c_int) as DCTELEM;
            /* Odd part */
            z13 = (*wsptr.offset(5) as DCTELEM as c_int
                + *wsptr.offset(3) as DCTELEM as c_int)
                as DCTELEM; /* phase 5 */
            z10 = (*wsptr.offset(5) as DCTELEM as c_int
                - *wsptr.offset(3) as DCTELEM as c_int)
                as DCTELEM; /* 2*c4 */
            z11 = (*wsptr.offset(1) as DCTELEM as c_int
                + *wsptr.offset(7) as DCTELEM as c_int)
                as DCTELEM; /* 2*c2 */
            z12 = (*wsptr.offset(1) as DCTELEM as c_int
                - *wsptr.offset(7) as DCTELEM as c_int)
                as DCTELEM; /* 2*(c2-c6) */
            tmp7 = (z11 as c_int + z13 as c_int) as DCTELEM; /* -2*(c2+c6) */
            tmp11 = ((z11 as c_int - z13 as c_int) as c_long
                * 362i64
                >> 8i32) as DCTELEM; /* phase 2 */
            z5 = ((z10 as c_int + z12 as c_int) as c_long
                * 473i64
                >> 8i32) as DCTELEM;
            tmp10 = ((z12 as c_long * 277i64 >> 8i32)
                as DCTELEM as c_int
                - z5 as c_int) as DCTELEM;
            tmp12 = ((z10 as c_long * -(669i64) >> 8i32)
                as DCTELEM as c_int
                + z5 as c_int) as DCTELEM;
            tmp6 = (tmp12 as c_int - tmp7 as c_int) as DCTELEM;
            tmp5 = (tmp11 as c_int - tmp6 as c_int) as DCTELEM;
            tmp4 = (tmp10 as c_int + tmp5 as c_int) as DCTELEM;
            /* Final output stage: scale down by a factor of 8 and range-limit */
            *outptr.offset(0) = *range_limit.offset(
                (tmp0 as c_int + tmp7 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(7) = *range_limit.offset(
                (tmp0 as c_int - tmp7 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1) = *range_limit.offset(
                (tmp1 as c_int + tmp6 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(6) = *range_limit.offset(
                (tmp1 as c_int - tmp6 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(2) = *range_limit.offset(
                (tmp2 as c_int + tmp5 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(5) = *range_limit.offset(
                (tmp2 as c_int - tmp5 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(4) = *range_limit.offset(
                (tmp3 as c_int + tmp4 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(3) = *range_limit.offset(
                (tmp3 as c_int - tmp4 as c_int >> 2i32 + 3i32
                    & RANGE_MASK) as isize,
            );
            wsptr = wsptr.offset(DCTSIZE as isize)
        }
        ctr += 1
    }
}
/* DCT_IFAST_SUPPORTED */
