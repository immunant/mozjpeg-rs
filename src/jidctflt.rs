pub use crate::jdct_h::FLOAT_MULT_TYPE;
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
use libc::c_float;
use libc::c_int;
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
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut FLOAT_MULT_TYPE = 0 as *mut FLOAT_MULT_TYPE;
    let mut wsptr: *mut c_float = 0 as *mut c_float;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_float; 64] = [0.; 64];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut FLOAT_MULT_TYPE;
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
            let mut dcval: c_float = *inptr.offset((8i32 * 0i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 0i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
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
            tmp0 = *inptr.offset((8i32 * 0i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 0i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            tmp1 = *inptr.offset((8i32 * 2i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 2i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            tmp2 = *inptr.offset((8i32 * 4i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 4i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            tmp3 = *inptr.offset((8i32 * 6i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 6i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            tmp10 = tmp0 + tmp2;
            tmp11 = tmp0 - tmp2;
            tmp13 = tmp1 + tmp3;
            tmp12 = (tmp1 - tmp3) * 1.414213562f64 as c_float - tmp13;
            tmp0 = tmp10 + tmp13;
            tmp3 = tmp10 - tmp13;
            tmp1 = tmp11 + tmp12;
            tmp2 = tmp11 - tmp12;
            tmp4 = *inptr.offset((8i32 * 1i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 1i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            tmp5 = *inptr.offset((8i32 * 3i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 3i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            tmp6 = *inptr.offset((8i32 * 5i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 5i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            tmp7 = *inptr.offset((8i32 * 7i32) as isize) as c_float
                * (*quantptr.offset((8i32 * 7i32) as isize) * 0.125f64 as FLOAT_MULT_TYPE);
            z13 = tmp6 + tmp5;
            z10 = tmp6 - tmp5;
            z11 = tmp4 + tmp7;
            z12 = tmp4 - tmp7;
            tmp7 = z11 + z13;
            tmp11 = (z11 - z13) * 1.414213562f64 as c_float;
            z5 = (z10 + z12) * 1.847759065f64 as c_float;
            tmp10 = z5 - z12 * 1.082392200f64 as c_float;
            tmp12 = z5 - z10 * 2.613125930f64 as c_float;
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
        z5 = *wsptr.offset(0isize) + (CENTERJSAMPLE as c_float + 0.5f64 as c_float);
        tmp10 = z5 + *wsptr.offset(4isize);
        tmp11 = z5 - *wsptr.offset(4isize);
        tmp13 = *wsptr.offset(2isize) + *wsptr.offset(6isize);
        tmp12 = (*wsptr.offset(2isize) - *wsptr.offset(6isize)) * 1.414213562f64 as c_float - tmp13;
        tmp0 = tmp10 + tmp13;
        tmp3 = tmp10 - tmp13;
        tmp1 = tmp11 + tmp12;
        tmp2 = tmp11 - tmp12;
        z13 = *wsptr.offset(5isize) + *wsptr.offset(3isize);
        z10 = *wsptr.offset(5isize) - *wsptr.offset(3isize);
        z11 = *wsptr.offset(1isize) + *wsptr.offset(7isize);
        z12 = *wsptr.offset(1isize) - *wsptr.offset(7isize);
        tmp7 = z11 + z13;
        tmp11 = (z11 - z13) * 1.414213562f64 as c_float;
        z5 = (z10 + z12) * 1.847759065f64 as c_float;
        tmp10 = z5 - z12 * 1.082392200f64 as c_float;
        tmp12 = z5 - z10 * 2.613125930f64 as c_float;
        tmp6 = tmp12 - tmp7;
        tmp5 = tmp11 - tmp6;
        tmp4 = tmp10 - tmp5;
        *outptr.offset(0isize) =
            *range_limit.offset(((tmp0 + tmp7) as c_int & RANGE_MASK) as isize);
        *outptr.offset(7isize) =
            *range_limit.offset(((tmp0 - tmp7) as c_int & RANGE_MASK) as isize);
        *outptr.offset(1isize) =
            *range_limit.offset(((tmp1 + tmp6) as c_int & RANGE_MASK) as isize);
        *outptr.offset(6isize) =
            *range_limit.offset(((tmp1 - tmp6) as c_int & RANGE_MASK) as isize);
        *outptr.offset(2isize) =
            *range_limit.offset(((tmp2 + tmp5) as c_int & RANGE_MASK) as isize);
        *outptr.offset(5isize) =
            *range_limit.offset(((tmp2 - tmp5) as c_int & RANGE_MASK) as isize);
        *outptr.offset(3isize) =
            *range_limit.offset(((tmp3 + tmp4) as c_int & RANGE_MASK) as isize);
        *outptr.offset(4isize) =
            *range_limit.offset(((tmp3 - tmp4) as c_int & RANGE_MASK) as isize);
        wsptr = wsptr.offset(DCTSIZE as isize);
        ctr += 1
    }
}
