use crate::jmorecfg_h::INT16;
use crate::jmorecfg_h::JDIMENSION;
use crate::jmorecfg_h::JSAMPLE;
use crate::jpegint_h::JLONG;
use crate::jpeglib_h::j_decompress_ptr;
use crate::jpeglib_h::jpeg_decompress_struct;
use crate::jpeglib_h::JSAMPARRAY;
use crate::jpeglib_h::JSAMPIMAGE;
use crate::jpeglib_h::JSAMPROW;
use crate::src::jdmerge::my_upsample_ptr;
use crate::src::jdmerge::my_upsampler;
/*
 * jdmrg565.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2013, Linaro Limited.
 * Copyright (C) 2014-2015, 2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains code for merged upsampling/color conversion.
 */

/*
 * jdmrg565.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2013, Linaro Limited.
 * Copyright (C) 2014-2015, 2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains code for merged upsampling/color conversion.
 */
#[inline(always)]
pub unsafe extern "C" fn h2v1_merged_upsample_565_be(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr0 = *(*input_buf.offset(0)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0);
    /* Loop for each pair of output pixels */
    /* Loop for each pair of output pixels */
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        let fresh70 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh70 as libc::c_int;
        let fresh71 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh71 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        /* Fetch 2 Y values and emit 2 pixels */
        /* Fetch 2 Y values and emit 2 pixels */
        let fresh72 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh72 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        let fresh73 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh73 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as libc::c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as libc::c_uint
                | b << 5i32 & 0x1f00i32 as libc::c_uint) as libc::c_long;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(1) = rgb as crate::jmorecfg_h::INT16;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(0) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr = outptr.offset(4);
        col =  col - 1
    }
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v1_merged_upsample_565_le(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr0 = *(*input_buf.offset(0)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        let fresh74 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh74 as libc::c_int;
        let fresh75 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh75 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh76 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh76 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        let fresh77 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh77 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = ((r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32)
            << 16i32) as libc::c_long
            | rgb;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(0) = rgb as crate::jmorecfg_h::INT16;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(1) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr = outptr.offset(4);
        col =  col - 1
    }
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v1_merged_upsample_565D_le(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr0 = *(*input_buf.offset(0)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0);
    /* Loop for each pair of output pixels */
    /* Loop for each pair of output pixels */
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        let fresh78 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh78 as libc::c_int;
        let fresh79 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh79 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        /* Fetch 2 Y values and emit 2 pixels */
        /* Fetch 2 Y values and emit 2 pixels */
        let fresh80 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh80 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        let fresh81 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh81 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = ((r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32)
            << 16i32) as libc::c_long
            | rgb;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(0) = rgb as crate::jmorecfg_h::INT16;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(1) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr = outptr.offset(4);
        col =  col - 1
    }
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v1_merged_upsample_565D_be(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr0 = *(*input_buf.offset(0)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        let fresh82 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh82 as libc::c_int;
        let fresh83 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh83 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh84 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh84 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        let fresh85 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh85 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as libc::c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as libc::c_uint
                | b << 5i32 & 0x1f00i32 as libc::c_uint) as libc::c_long;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(1) = rgb as crate::jmorecfg_h::INT16;
        *(outptr as *mut crate::jmorecfg_h::INT16).offset(0) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr = outptr.offset(4);
        col =  col - 1
    }
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565_le(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr00: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr01: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr00 = *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2i32 as libc::c_uint) as isize);
    inptr01 = *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2i32 as libc::c_uint + 1i32 as libc::c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0);
    outptr1 = *output_buf.offset(1);
    /* Loop for each group of output pixels */
    /* Loop for each group of output pixels */
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        let fresh86 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh86 as libc::c_int;
        let fresh87 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh87 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        /* Fetch 4 Y values and emit 4 pixels */
        /* Fetch 4 Y values and emit 4 pixels */
        let fresh88 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh88 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        let fresh89 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh89 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = ((r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32)
            << 16i32) as libc::c_long
            | rgb;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0) = rgb as crate::jmorecfg_h::INT16;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr0 = outptr0.offset(4);
        let fresh90 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh90 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        let fresh91 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh91 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = ((r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32)
            << 16i32) as libc::c_long
            | rgb;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0) = rgb as crate::jmorecfg_h::INT16;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr1 = outptr1.offset(4);
        col =  col - 1
    }
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
        y = *inptr01 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        *(outptr1 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565_be(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr00: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr01: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr00 = *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2i32 as libc::c_uint) as isize);
    inptr01 = *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2i32 as libc::c_uint + 1i32 as libc::c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0);
    outptr1 = *output_buf.offset(1);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        let fresh92 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh92 as libc::c_int;
        let fresh93 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh93 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh94 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh94 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        let fresh95 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh95 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as libc::c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as libc::c_uint
                | b << 5i32 & 0x1f00i32 as libc::c_uint) as libc::c_long;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1) = rgb as crate::jmorecfg_h::INT16;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr0 = outptr0.offset(4);
        let fresh96 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh96 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        let fresh97 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh97 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as libc::c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as libc::c_uint
                | b << 5i32 & 0x1f00i32 as libc::c_uint) as libc::c_long;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1) = rgb as crate::jmorecfg_h::INT16;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr1 = outptr1.offset(4);
        col =  col - 1
    }
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
        y = *inptr01 as libc::c_int;
        r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        *(outptr1 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565D_le(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr00: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr01: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    let mut d1: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline + 1i32 as libc::c_uint
            & DITHER_MASK as libc::c_uint) as usize];
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr00 = *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2i32 as libc::c_uint) as isize);
    inptr01 = *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2i32 as libc::c_uint + 1i32 as libc::c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0);
    outptr1 = *output_buf.offset(1);
    /* Loop for each group of output pixels */
    /* Loop for each group of output pixels */
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        let fresh98 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh98 as libc::c_int;
        let fresh99 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh99 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        /* Fetch 4 Y values and emit 4 pixels */
        /* Fetch 4 Y values and emit 4 pixels */
        let fresh100 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh100 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        let fresh101 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh101 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = ((r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32)
            << 16i32) as libc::c_long
            | rgb;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0) = rgb as crate::jmorecfg_h::INT16;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr0 = outptr0.offset(4);
        let fresh102 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh102 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d1 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d1 = (d1 & 0xffi32 as libc::c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        let fresh103 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh103 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d1 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d1 = (d1 & 0xffi32 as libc::c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = ((r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32)
            << 16i32) as libc::c_long
            | rgb;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0) = rgb as crate::jmorecfg_h::INT16;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr1 = outptr1.offset(4);
        col =  col - 1
    }
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
        y = *inptr01 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d1 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        rgb = (r << 8i32 & 0xf800i32 as libc::c_uint
            | g << 3i32 & 0x7e0i32 as libc::c_uint
            | b >> 3i32) as crate::jpegint_h::JLONG;
        *(outptr1 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565D_be(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: libc::c_int = 0;
    let mut cred: libc::c_int = 0;
    let mut cgreen: libc::c_int = 0;
    let mut cblue: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr00: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr01: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    let mut d1: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline + 1i32 as libc::c_uint
            & DITHER_MASK as libc::c_uint) as usize];
    let mut r: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut rgb: crate::jpegint_h::JLONG = 0;
    inptr00 = *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2i32 as libc::c_uint) as isize);
    inptr01 = *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2i32 as libc::c_uint + 1i32 as libc::c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0);
    outptr1 = *output_buf.offset(1);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as libc::c_uint {
        let fresh104 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh104 as libc::c_int;
        let fresh105 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh105 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh106 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh106 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        let fresh107 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh107 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d0 = (d0 & 0xffi32 as libc::c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as libc::c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as libc::c_uint
                | b << 5i32 & 0x1f00i32 as libc::c_uint) as libc::c_long;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1) = rgb as crate::jmorecfg_h::INT16;
        *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr0 = outptr0.offset(4);
        let fresh108 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh108 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d1 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d1 = (d1 & 0xffi32 as libc::c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        let fresh109 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh109 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d1 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        d1 = (d1 & 0xffi32 as libc::c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as libc::c_long;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as libc::c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as libc::c_uint
                | b << 5i32 & 0x1f00i32 as libc::c_uint) as libc::c_long;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1) = rgb as crate::jmorecfg_h::INT16;
        *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0) =
            (rgb >> 16i32) as crate::jmorecfg_h::INT16;
        outptr1 = outptr1.offset(4);
        col =  col - 1
    }
    if (*cinfo).output_width & 1i32 as libc::c_uint != 0 {
        cb = *inptr1 as libc::c_int;
        cr = *inptr2 as libc::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as libc::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d0 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d0 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
        y = *inptr01 as libc::c_int;
        r = *range_limit
            .offset(((y + cred) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as libc::c_long + ((d1 & 0xffi32 as libc::c_long) >> 1i32)) as isize,
        ) as libc::c_uint;
        b = *range_limit
            .offset(((y + cblue) as libc::c_long + (d1 & 0xffi32 as libc::c_long)) as isize)
            as libc::c_uint;
        rgb = (r & 0xf8i32 as libc::c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as libc::c_uint
            | b << 5i32 & 0x1f00i32 as libc::c_uint) as crate::jpegint_h::JLONG;
        *(outptr1 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
    };
}
use crate::src::jdmerge::dither_matrix;
use crate::src::jdmerge::DITHER_MASK;
