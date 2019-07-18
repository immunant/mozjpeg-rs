use crate::jdcolor::my_cconvert_ptr;

use crate::jmorecfg_h::INT16;
use crate::jmorecfg_h::JDIMENSION;
use crate::jmorecfg_h::JSAMPLE;
use crate::jpegint_h::JLONG;
use crate::jpeglib_h::j_decompress_ptr;

use crate::jpeglib_h::JSAMPARRAY;
use crate::jpeglib_h::JSAMPIMAGE;
use crate::jpeglib_h::JSAMPROW;
use crate::stddef_h::size_t;
use ::libc;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
/*
 * jdcol565.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modifications:
 * Copyright (C) 2013, Linaro Limited.
 * Copyright (C) 2014-2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/* This file is included by jdcolor.c */
#[inline(always)]
pub unsafe extern "C" fn ycc_rgb565_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh53 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh53;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh54 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh54 as c_int;
            let fresh55 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh55 as c_int;
            let fresh56 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh56 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh57 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh57 as c_int;
            let fresh58 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh58 as c_int;
            let fresh59 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh59 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            let fresh60 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh60 as c_int;
            let fresh61 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh61 as c_int;
            let fresh62 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh62 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = rgb << 16i32
                | (r & 0xf8i32 as c_uint
                    | g >> 5i32
                    | g << 11i32 & 0xe000i32 as c_uint
                    | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            y = *inptr0 as c_int;
            cb = *inptr1 as c_int;
            cr = *inptr2 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
/*
 * jdcol565.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modifications:
 * Copyright (C) 2013, Linaro Limited.
 * Copyright (C) 2014-2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/* This file is included by jdcolor.c */
#[inline(always)]
pub unsafe extern "C" fn ycc_rgb565_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh63 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh63;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh64 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh64 as c_int;
            let fresh65 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh65 as c_int;
            let fresh66 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh66 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh67 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh67 as c_int;
            let fresh68 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh68 as c_int;
            let fresh69 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh69 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            let fresh70 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh70 as c_int;
            let fresh71 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh71 as c_int;
            let fresh72 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh72 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                << 16i32) as c_long
                | rgb;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            y = *inptr0 as c_int;
            cb = *inptr1 as c_int;
            cr = *inptr2 as c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            ) as c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_rgb565D_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh73 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh73;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh74 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh74 as c_int;
            let fresh75 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh75 as c_int;
            let fresh76 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh76 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh77 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh77 as c_int;
            let fresh78 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh78 as c_int;
            let fresh79 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh79 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            let fresh80 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh80 as c_int;
            let fresh81 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh81 as c_int;
            let fresh82 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh82 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                << 16i32) as c_long
                | rgb;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            y = *inptr0 as c_int;
            cb = *inptr1 as c_int;
            cr = *inptr2 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_rgb565D_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh83 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh83;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh84 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh84 as c_int;
            let fresh85 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh85 as c_int;
            let fresh86 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh86 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh87 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh87 as c_int;
            let fresh88 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh88 as c_int;
            let fresh89 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh89 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            let fresh90 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh90 as c_int;
            let fresh91 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh91 as c_int;
            let fresh92 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh92 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = rgb << 16i32
                | (r & 0xf8i32 as c_uint
                    | g >> 5i32
                    | g << 11i32 & 0xe000i32 as c_uint
                    | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            y = *inptr0 as c_int;
            cb = *inptr1 as c_int;
            cr = *inptr2 as c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as c_long
                    + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as c_long + (d0 & 0xffi32 as c_long)) as isize,
            ) as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb565_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh93 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh93;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh94 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh94 as c_int as c_uint;
            let fresh95 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh95 as c_int as c_uint;
            let fresh96 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh96 as c_int as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh97 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh97 as c_int as c_uint;
            let fresh98 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh98 as c_int as c_uint;
            let fresh99 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh99 as c_int as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            let fresh100 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh100 as c_int as c_uint;
            let fresh101 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh101 as c_int as c_uint;
            let fresh102 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh102 as c_int as c_uint;
            rgb = rgb << 16i32
                | (r & 0xf8i32 as c_uint
                    | g >> 5i32
                    | g << 11i32 & 0xe000i32 as c_uint
                    | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            r = *inptr0 as c_int as c_uint;
            g = *inptr1 as c_int as c_uint;
            b = *inptr2 as c_int as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb565_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh103 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh103;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh104 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh104 as c_int as c_uint;
            let fresh105 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh105 as c_int as c_uint;
            let fresh106 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh106 as c_int as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh107 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh107 as c_int as c_uint;
            let fresh108 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh108 as c_int as c_uint;
            let fresh109 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh109 as c_int as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            let fresh110 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh110 as c_int as c_uint;
            let fresh111 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh111 as c_int as c_uint;
            let fresh112 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh112 as c_int as c_uint;
            rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                << 16i32) as c_long
                | rgb;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            r = *inptr0 as c_int as c_uint;
            g = *inptr1 as c_int as c_uint;
            b = *inptr2 as c_int as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb565D_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh113 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh113;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh114 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit
                .offset((*fresh114 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            let fresh115 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh115 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            let fresh116 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit
                .offset((*fresh116 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh117 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit
                .offset((*fresh117 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            let fresh118 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh118 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            let fresh119 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit
                .offset((*fresh119 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            let fresh120 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit
                .offset((*fresh120 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            let fresh121 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh121 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            let fresh122 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit
                .offset((*fresh122 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = rgb << 16i32
                | (r & 0xf8i32 as c_uint
                    | g >> 5i32
                    | g << 11i32 & 0xe000i32 as c_uint
                    | b << 5i32 & 0x1f00i32 as c_uint) as c_long;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            r = *range_limit
                .offset((*inptr0 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            g = *range_limit
                .offset((*inptr1 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
                as c_uint;
            b = *range_limit
                .offset((*inptr2 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            rgb = (r & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | b << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb565D_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: c_uint = 0;
        let mut g: c_uint = 0;
        let mut b: c_uint = 0;
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh123 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh123;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh124 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit
                .offset((*fresh124 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            let fresh125 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh125 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            let fresh126 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit
                .offset((*fresh126 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh127 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit
                .offset((*fresh127 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            let fresh128 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh128 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            let fresh129 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit
                .offset((*fresh129 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            let fresh130 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit
                .offset((*fresh130 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            let fresh131 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh131 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize,
            ) as c_uint;
            let fresh132 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit
                .offset((*fresh132 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            rgb = ((r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                << 16i32) as c_long
                | rgb;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            r = *range_limit
                .offset((*inptr0 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            g = *range_limit
                .offset((*inptr1 as c_int as c_long + ((d0 & 0xffi32 as c_long) >> 1i32)) as isize)
                as c_uint;
            b = *range_limit
                .offset((*inptr2 as c_int as c_long + (d0 & 0xffi32 as c_long)) as isize)
                as c_uint;
            rgb = (r << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | b >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_rgb565_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: c_uint = 0;
        let fresh133 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0isize)).offset(fresh133 as isize);
        let fresh134 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh134;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh135 = inptr;
            inptr = inptr.offset(1);
            g = *fresh135 as c_uint;
            rgb = (g & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | g << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh136 = inptr;
            inptr = inptr.offset(1);
            g = *fresh136 as c_uint;
            rgb = (g & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | g << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            let fresh137 = inptr;
            inptr = inptr.offset(1);
            g = *fresh137 as c_uint;
            rgb = rgb << 16i32
                | (g & 0xf8i32 as c_uint
                    | g >> 5i32
                    | g << 11i32 & 0xe000i32 as c_uint
                    | g << 5i32 & 0x1f00i32 as c_uint) as c_long;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            g = *inptr as c_uint;
            rgb = (g & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | g << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_rgb565_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: c_uint = 0;
        let fresh138 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0isize)).offset(fresh138 as isize);
        let fresh139 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh139;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh140 = inptr;
            inptr = inptr.offset(1);
            g = *fresh140 as c_uint;
            rgb = (g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh141 = inptr;
            inptr = inptr.offset(1);
            g = *fresh141 as c_uint;
            rgb = (g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                as JLONG;
            let fresh142 = inptr;
            inptr = inptr.offset(1);
            g = *fresh142 as c_uint;
            rgb = ((g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                << 16i32) as c_long
                | rgb;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            g = *inptr as c_uint;
            rgb = (g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_rgb565D_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: c_uint = 0;
        let fresh143 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0isize)).offset(fresh143 as isize);
        let fresh144 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh144;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh145 = inptr;
            inptr = inptr.offset(1);
            g = *fresh145 as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = (g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh146 = inptr;
            inptr = inptr.offset(1);
            g = *fresh146 as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = (g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                as JLONG;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            let fresh147 = inptr;
            inptr = inptr.offset(1);
            g = *fresh147 as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = ((g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                << 16i32) as c_long
                | rgb;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            g = *inptr as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = (g << 8i32 & 0xf800i32 as c_uint | g << 3i32 & 0x7e0i32 as c_uint | g >> 3i32)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_rgb565D_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG = dither_matrix[((*cinfo).output_scanline & DITHER_MASK as c_uint) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: c_uint = 0;
        let fresh148 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0isize)).offset(fresh148 as isize);
        let fresh149 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh149;
        if 0 != outptr as size_t & 3i32 as c_ulong {
            let fresh150 = inptr;
            inptr = inptr.offset(1);
            g = *fresh150 as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = (g & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | g << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2isize);
            num_cols = num_cols.wrapping_sub(1)
        }
        col = 0i32 as JDIMENSION;
        while col < num_cols >> 1i32 {
            let fresh151 = inptr;
            inptr = inptr.offset(1);
            g = *fresh151 as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = (g & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | g << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            let fresh152 = inptr;
            inptr = inptr.offset(1);
            g = *fresh152 as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = rgb << 16i32
                | (g & 0xf8i32 as c_uint
                    | g >> 5i32
                    | g << 11i32 & 0xe000i32 as c_uint
                    | g << 5i32 & 0x1f00i32 as c_uint) as c_long;
            d0 = (d0 & 0xffi32 as c_long) << 24i32 | d0 >> 8i32 & 0xffffffi32 as c_long;
            *(outptr as *mut c_int) = rgb as c_int;
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
        if 0 != num_cols & 1i32 as c_uint {
            g = *inptr as c_uint;
            g = *range_limit.offset((g as c_long + (d0 & 0xffi32 as c_long)) as isize) as c_uint;
            rgb = (g & 0xf8i32 as c_uint
                | g >> 5i32
                | g << 11i32 & 0xe000i32 as c_uint
                | g << 5i32 & 0x1f00i32 as c_uint) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16
        }
    }
}
use crate::jdcolor::dither_matrix;
use crate::jdcolor::DITHER_MASK;
