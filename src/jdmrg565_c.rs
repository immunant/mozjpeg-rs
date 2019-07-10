use crate::jdmerge::my_upsample_ptr;
use crate::jdmerge::my_upsampler;
use crate::jmorecfg_h::INT16;
use crate::jmorecfg_h::JDIMENSION;
use crate::jmorecfg_h::JSAMPLE;
use crate::jpegint_h::JLONG;
use crate::jpeglib_h::j_decompress_ptr;
use crate::jpeglib_h::jpeg_decompress_struct;
use crate::jpeglib_h::JSAMPARRAY;
use crate::jpeglib_h::JSAMPIMAGE;
use crate::jpeglib_h::JSAMPROW;
use ::libc;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
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
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 = *(*input_buf.offset(0isize)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh70 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh70 as c_int;
        let fresh71 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh71 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh72 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh72 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh73 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh73 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
        *(outptr as *mut INT16).offset(1isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(0isize) = (rgb >> 16i32) as INT16;
        outptr = outptr.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16
    };
}
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
pub unsafe extern "C" fn h2v1_merged_upsample_565_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 = *(*input_buf.offset(0isize)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh74 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh74 as c_int;
        let fresh75 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh75 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh76 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh76 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh77 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh77 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
            << 16i32) as c_long
            | rgb;
        *(outptr as *mut INT16).offset(0isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(1isize) = (rgb >> 16i32) as INT16;
        outptr = outptr.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v1_merged_upsample_565D_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 = *(*input_buf.offset(0isize)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh78 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh78 as c_int;
        let fresh79 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh79 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh80 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh80 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh81 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh81 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
        *(outptr as *mut INT16).offset(1isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(0isize) = (rgb >> 16i32) as INT16;
        outptr = outptr.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v1_merged_upsample_565D_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 = *(*input_buf.offset(0isize)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh82 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh82 as c_int;
        let fresh83 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh83 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh84 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh84 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh85 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh85 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
            << 16i32) as c_long
            | rgb;
        *(outptr as *mut INT16).offset(0isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(1isize) = (rgb >> 16i32) as INT16;
        outptr = outptr.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr00: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr01: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 =
        *(*input_buf.offset(0isize)).offset(in_row_group_ctr.wrapping_mul(2i32 as c_uint) as isize);
    inptr01 = *(*input_buf.offset(0isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2i32 as c_uint)
            .wrapping_add(1i32 as c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0isize);
    outptr1 = *output_buf.offset(1isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh86 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh86 as c_int;
        let fresh87 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh87 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh88 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh88 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh89 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh89 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
            << 16i32) as c_long
            | rgb;
        *(outptr0 as *mut INT16).offset(0isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(1isize) = (rgb >> 16i32) as INT16;
        outptr0 = outptr0.offset(4isize);
        let fresh90 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh90 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh91 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh91 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
            << 16i32) as c_long
            | rgb;
        *(outptr1 as *mut INT16).offset(0isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(1isize) = (rgb >> 16i32) as INT16;
        outptr1 = outptr1.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr00: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr01: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 =
        *(*input_buf.offset(0isize)).offset(in_row_group_ctr.wrapping_mul(2i32 as c_uint) as isize);
    inptr01 = *(*input_buf.offset(0isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2i32 as c_uint)
            .wrapping_add(1i32 as c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0isize);
    outptr1 = *output_buf.offset(1isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh92 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh92 as c_int;
        let fresh93 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh93 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh94 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh94 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh95 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh95 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
        *(outptr0 as *mut INT16).offset(1isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(0isize) = (rgb >> 16i32) as INT16;
        outptr0 = outptr0.offset(4isize);
        let fresh96 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh96 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh97 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh97 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
        *(outptr1 as *mut INT16).offset(1isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(0isize) = (rgb >> 16i32) as INT16;
        outptr1 = outptr1.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565D_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr00: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr01: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    let mut d1: JLONG = dither_matrix
        [((*cinfo).output_scanline.wrapping_add(1i32 as c_uint) & DITHER_MASK as c_uint) as usize];
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 =
        *(*input_buf.offset(0isize)).offset(in_row_group_ctr.wrapping_mul(2i32 as c_uint) as isize);
    inptr01 = *(*input_buf.offset(0isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2i32 as c_uint)
            .wrapping_add(1i32 as c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0isize);
    outptr1 = *output_buf.offset(1isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh98 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh98 as c_int;
        let fresh99 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh99 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh100 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh100 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh101 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh101 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
            << 16i32) as c_long
            | rgb;
        *(outptr0 as *mut INT16).offset(0isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(1isize) = (rgb >> 16i32) as INT16;
        outptr0 = outptr0.offset(4isize);
        let fresh102 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh102 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d1 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d1 = (d1 & 0xffi32 as c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as c_long;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh103 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh103 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d1 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d1 = (d1 & 0xffi32 as c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as c_long;
        rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
            << 16i32) as c_long
            | rgb;
        *(outptr1 as *mut INT16).offset(0isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(1isize) = (rgb >> 16i32) as INT16;
        outptr1 = outptr1.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d1 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_565D_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: c_int = 0;
    let mut cred: c_int = 0;
    let mut cgreen: c_int = 0;
    let mut cblue: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr00: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr01: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    let mut d1: JLONG = dither_matrix
        [((*cinfo).output_scanline.wrapping_add(1i32 as c_uint) & DITHER_MASK as c_uint) as usize];
    let mut r: c_uint = 0;
    let mut g: c_uint = 0;
    let mut b: c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 =
        *(*input_buf.offset(0isize)).offset(in_row_group_ctr.wrapping_mul(2i32 as c_uint) as isize);
    inptr01 = *(*input_buf.offset(0isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2i32 as c_uint)
            .wrapping_add(1i32 as c_uint) as isize,
    );
    inptr1 = *(*input_buf.offset(1isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0isize);
    outptr1 = *output_buf.offset(1isize);
    col = (*cinfo).output_width >> 1i32;
    while col > 0i32 as c_uint {
        let fresh104 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh104 as c_int;
        let fresh105 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh105 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh106 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh106 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh107 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh107 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
        *(outptr0 as *mut INT16).offset(1isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(0isize) = (rgb >> 16i32) as INT16;
        outptr0 = outptr0.offset(4isize);
        let fresh108 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh108 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d1 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d1 = (d1 & 0xffi32 as c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as c_long;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh109 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh109 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d1 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        d1 = (d1 & 0xffi32 as c_long) << 24i32 | d1 >> 8i32 & 0xffffffi32 as c_long;
        rgb = rgb << 16i32
            | (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
        *(outptr1 as *mut INT16).offset(1isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(0isize) = (rgb >> 16i32) as INT16;
        outptr1 = outptr1.offset(4isize);
        col = col.wrapping_sub(1)
    }
    if 0 != (*cinfo).output_width & 1i32 as c_uint {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d0 & 0xffi32 as c_long)) as isize)
            as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as c_int;
        r = *range_limit.offset(((y + cred) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        g = *range_limit
            .offset(((y + cgreen) as c_long + ((d1 & 0xffi32 as c_long) >> 1i32)) as isize)
            as c_uint;
        b = *range_limit.offset(((y + cblue) as c_long + (d1 & 0xffi32 as c_long)) as isize)
            as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16
    };
}
use crate::jdmerge::dither_matrix;
use crate::jdmerge::DITHER_MASK;
