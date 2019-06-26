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
        let fresh0 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh0 as c_int;
        let fresh1 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh1 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh2 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh2 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh3 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh3 as c_int;
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
        let fresh4 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh4 as c_int;
        let fresh5 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh5 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh6 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh6 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh7 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh7 as c_int;
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
        let fresh8 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh8 as c_int;
        let fresh9 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh9 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh10 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh10 as c_int;
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
        let fresh11 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh11 as c_int;
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
        let fresh12 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh12 as c_int;
        let fresh13 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh13 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh14 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh14 as c_int;
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
        let fresh15 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh15 as c_int;
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
        let fresh16 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh16 as c_int;
        let fresh17 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh17 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh18 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh18 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh19 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh19 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
            << 16i32) as c_long
            | rgb;
        *(outptr0 as *mut INT16).offset(0isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(1isize) = (rgb >> 16i32) as INT16;
        outptr0 = outptr0.offset(4isize);
        let fresh20 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh20 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb =
            (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32) as JLONG;
        let fresh21 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh21 as c_int;
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
        let fresh22 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh22 as c_int;
        let fresh23 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh23 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh24 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh24 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh25 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh25 as c_int;
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
        let fresh26 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh26 as c_int;
        r = *range_limit.offset((y + cred) as isize) as c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as c_uint;
        b = *range_limit.offset((y + cblue) as isize) as c_uint;
        rgb = (r & 0xf8i32 as c_uint
            | g >> 5i32
            | g << 11i32 & 0xe000i32 as c_uint
            | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
        let fresh27 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh27 as c_int;
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
        let fresh28 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh28 as c_int;
        let fresh29 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh29 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh30 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh30 as c_int;
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
        let fresh31 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh31 as c_int;
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
        let fresh32 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh32 as c_int;
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
        let fresh33 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh33 as c_int;
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
        let fresh34 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh34 as c_int;
        let fresh35 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh35 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh36 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh36 as c_int;
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
        let fresh37 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh37 as c_int;
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
        let fresh38 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh38 as c_int;
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
        let fresh39 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh39 as c_int;
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
