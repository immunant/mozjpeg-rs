use crate::jmorecfg_h::JDIMENSION;
use crate::jmorecfg_h::JSAMPLE;
use crate::jpegint_h::JLONG;
use crate::jpeglib_h::j_decompress_ptr;
use crate::jpeglib_h::jpeg_decompress_struct;
use crate::jpeglib_h::JSAMPARRAY;
use crate::jpeglib_h::JSAMPIMAGE;
use crate::jpeglib_h::JSAMPROW;
use crate::src::jdcolor::my_cconvert_ptr;
use crate::src::jdcolor::my_color_deconverter;
/*
 * jdcolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2011, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/*
 * jdcolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2011, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/*
 * jdcolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2011, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/*
 * jdcolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2011, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/*
 * jdcolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2011, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/*
 * jdcolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2011, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/*
 * jdcolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2011, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */

/* This file is included by jdcolor.c */

/* This file is included by jdcolor.c */

/* This file is included by jdcolor.c */

/* This file is included by jdcolor.c */

/* This file is included by jdcolor.c */

/* This file is included by jdcolor.c */

/* This file is included by jdcolor.c */

/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */
#[inline(always)]
pub unsafe extern "C" fn ycc_extxbgr_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh0 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh0;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            *outptr.offset(RGB_RED_0 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_0 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            *outptr.offset(RGB_ALPHA_0 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extrgbx_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh1 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            *outptr.offset(RGB_RED_2 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_2 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh2 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh2;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            );
            *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extxrgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh3 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh3;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            *outptr.offset(RGB_RED as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extbgr_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh4 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh4;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            *outptr.offset(RGB_RED_3 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_3 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extbgrx_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh5 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh5;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            *outptr.offset(RGB_RED_1 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_1 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extrgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh6 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh6;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            *outptr.offset(RGB_RED_4 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_4 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col =  col + 1
        }
    }
}
/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */

/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */

/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */

/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */

/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */

/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */

/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */
#[inline(always)]
pub unsafe extern "C" fn gray_extrgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh7 = input_row;
        input_row =  input_row + 1;
        inptr = *(*input_buf.offset(0)).offset(fresh7 as isize);
        let fresh8 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh8;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            let ref mut fresh9 = *outptr.offset(RGB_BLUE_4 as isize);
            *fresh9 = *inptr.offset(col as isize);
            let ref mut fresh10 = *outptr.offset(RGB_GREEN_4 as isize);
            *fresh10 = *fresh9;
            *outptr.offset(RGB_RED_4 as isize) = *fresh10;
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extrgbx_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh11 = input_row;
        input_row =  input_row + 1;
        inptr = *(*input_buf.offset(0)).offset(fresh11 as isize);
        let fresh12 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh12;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            let ref mut fresh13 = *outptr.offset(RGB_BLUE_2 as isize);
            *fresh13 = *inptr.offset(col as isize);
            let ref mut fresh14 = *outptr.offset(RGB_GREEN_2 as isize);
            *fresh14 = *fresh13;
            *outptr.offset(RGB_RED_2 as isize) = *fresh14;
            *outptr.offset(RGB_ALPHA_2 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extbgr_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh15 = input_row;
        input_row =  input_row + 1;
        inptr = *(*input_buf.offset(0)).offset(fresh15 as isize);
        let fresh16 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh16;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            let ref mut fresh17 = *outptr.offset(RGB_BLUE_3 as isize);
            *fresh17 = *inptr.offset(col as isize);
            let ref mut fresh18 = *outptr.offset(RGB_GREEN_3 as isize);
            *fresh18 = *fresh17;
            *outptr.offset(RGB_RED_3 as isize) = *fresh18;
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extbgrx_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh19 = input_row;
        input_row =  input_row + 1;
        inptr = *(*input_buf.offset(0)).offset(fresh19 as isize);
        let fresh20 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh20;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            let ref mut fresh21 = *outptr.offset(RGB_BLUE_1 as isize);
            *fresh21 = *inptr.offset(col as isize);
            let ref mut fresh22 = *outptr.offset(RGB_GREEN_1 as isize);
            *fresh22 = *fresh21;
            *outptr.offset(RGB_RED_1 as isize) = *fresh22;
            *outptr.offset(RGB_ALPHA_1 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extxbgr_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh23 = input_row;
        input_row =  input_row + 1;
        inptr = *(*input_buf.offset(0)).offset(fresh23 as isize);
        let fresh24 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh24;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            let ref mut fresh25 = *outptr.offset(RGB_BLUE_0 as isize);
            *fresh25 = *inptr.offset(col as isize);
            let ref mut fresh26 = *outptr.offset(RGB_GREEN_0 as isize);
            *fresh26 = *fresh25;
            *outptr.offset(RGB_RED_0 as isize) = *fresh26;
            *outptr.offset(RGB_ALPHA_0 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extxrgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh27 = input_row;
        input_row =  input_row + 1;
        inptr = *(*input_buf.offset(0)).offset(fresh27 as isize);
        let fresh28 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh28;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            let ref mut fresh29 = *outptr.offset(RGB_BLUE as isize);
            *fresh29 = *inptr.offset(col as isize);
            let ref mut fresh30 = *outptr.offset(RGB_GREEN as isize);
            *fresh30 = *fresh29;
            *outptr.offset(RGB_RED as isize) = *fresh30;
            *outptr.offset(RGB_ALPHA as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh31 = input_row;
        input_row =  input_row + 1;
        inptr = *(*input_buf.offset(0)).offset(fresh31 as isize);
        let fresh32 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh32;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            let ref mut fresh33 = *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize);
            *fresh33 = *inptr.offset(col as isize);
            let ref mut fresh34 = *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize);
            *fresh34 = *fresh33;
            *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) = *fresh34;
            outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            col =  col + 1
        }
    }
}
/*
 * Convert RGB to extended RGB: just swap the order of source pixels
 */

/*
 * Convert RGB to extended RGB: just swap the order of source pixels
 */

/*
 * Convert RGB to extended RGB: just swap the order of source pixels
 */

/*
 * Convert RGB to extended RGB: just swap the order of source pixels
 */

/*
 * Convert RGB to extended RGB: just swap the order of source pixels
 */

/*
 * Convert RGB to extended RGB: just swap the order of source pixels
 */

/*
 * Convert RGB to extended RGB: just swap the order of source pixels
 */
#[inline(always)]
pub unsafe extern "C" fn rgb_extbgr_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh35 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh35;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            /* We can dispense with GETJSAMPLE() here */
            *outptr.offset(RGB_RED_3 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_3 as isize) = *inptr2.offset(col as isize);
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            /* Set unused byte to 0xFF so it can be interpreted as an opaque */
            /* alpha channel value */
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extrgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh36 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh36;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_4 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_4 as isize) = *inptr2.offset(col as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extrgbx_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh37 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh37;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_2 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_2 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extbgrx_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh38 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh38;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_1 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_1 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extxbgr_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh39 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh39;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_0 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_0 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_0 as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extxrgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh40 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh40;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA as isize) = 0xffi32 as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row =  input_row + 1;
        let fresh41 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh41;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) = *inptr2.offset(col as isize);
            outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            col =  col + 1
        }
    }
}
use crate::jmorecfg_h::EXT_BGRX_BLUE;
use crate::jmorecfg_h::EXT_BGRX_GREEN;
use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
use crate::jmorecfg_h::EXT_BGRX_RED;
use crate::jmorecfg_h::EXT_BGR_BLUE;
use crate::jmorecfg_h::EXT_BGR_GREEN;
use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
use crate::jmorecfg_h::EXT_BGR_RED;
use crate::jmorecfg_h::EXT_RGBX_BLUE;
use crate::jmorecfg_h::EXT_RGBX_GREEN;
use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
use crate::jmorecfg_h::EXT_RGBX_RED;
use crate::jmorecfg_h::EXT_RGB_BLUE;
use crate::jmorecfg_h::EXT_RGB_GREEN;
use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
use crate::jmorecfg_h::EXT_RGB_RED;
use crate::jmorecfg_h::EXT_XBGR_BLUE;
use crate::jmorecfg_h::EXT_XBGR_GREEN;
use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
use crate::jmorecfg_h::EXT_XBGR_RED;
use crate::jmorecfg_h::EXT_XRGB_BLUE;
use crate::jmorecfg_h::EXT_XRGB_GREEN;
use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
use crate::jmorecfg_h::EXT_XRGB_RED;
use crate::jmorecfg_h::RGB_BLUE_5;
use crate::jmorecfg_h::RGB_GREEN_5;
use crate::jmorecfg_h::RGB_PIXELSIZE_5;
use crate::jmorecfg_h::RGB_RED_5;
use crate::src::jdcolor::RGB_ALPHA;
use crate::src::jdcolor::RGB_ALPHA_0;
use crate::src::jdcolor::RGB_ALPHA_1;
use crate::src::jdcolor::RGB_ALPHA_2;
use crate::src::jdcolor::RGB_BLUE;
use crate::src::jdcolor::RGB_BLUE_0;
use crate::src::jdcolor::RGB_BLUE_1;
use crate::src::jdcolor::RGB_BLUE_2;
use crate::src::jdcolor::RGB_BLUE_3;
use crate::src::jdcolor::RGB_BLUE_4;
use crate::src::jdcolor::RGB_GREEN;
use crate::src::jdcolor::RGB_GREEN_0;
use crate::src::jdcolor::RGB_GREEN_1;
use crate::src::jdcolor::RGB_GREEN_2;
use crate::src::jdcolor::RGB_GREEN_3;
use crate::src::jdcolor::RGB_GREEN_4;
use crate::src::jdcolor::RGB_PIXELSIZE;
use crate::src::jdcolor::RGB_PIXELSIZE_0;
use crate::src::jdcolor::RGB_PIXELSIZE_1;
use crate::src::jdcolor::RGB_PIXELSIZE_2;
use crate::src::jdcolor::RGB_PIXELSIZE_3;
use crate::src::jdcolor::RGB_PIXELSIZE_4;
use crate::src::jdcolor::RGB_RED;
use crate::src::jdcolor::RGB_RED_0;
use crate::src::jdcolor::RGB_RED_1;
use crate::src::jdcolor::RGB_RED_2;
use crate::src::jdcolor::RGB_RED_3;
use crate::src::jdcolor::RGB_RED_4;
