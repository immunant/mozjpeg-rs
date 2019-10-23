use crate::jmorecfg_h::JDIMENSION;
use crate::jmorecfg_h::JSAMPLE;
use crate::jpegint_h::JLONG;
use crate::jpeglib_h::j_compress_ptr;
use crate::jpeglib_h::jpeg_compress_struct;
use crate::jpeglib_h::JSAMPARRAY;
use crate::jpeglib_h::JSAMPIMAGE;
use crate::jpeglib_h::JSAMPROW;
use crate::src::jccolor::my_cconvert_ptr;
use crate::src::jccolor::my_color_converter;
/*
 * jccolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2012, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains input colorspace conversion routines.
 */

/*
 * jccolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2012, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains input colorspace conversion routines.
 */

/*
 * jccolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2012, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains input colorspace conversion routines.
 */

/*
 * jccolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2012, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains input colorspace conversion routines.
 */

/*
 * jccolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2012, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains input colorspace conversion routines.
 */

/*
 * jccolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2012, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains input colorspace conversion routines.
 */

/*
 * jccolext.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2012, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains input colorspace conversion routines.
 */

/* This file is included by jccolor.c */

/* This file is included by jccolor.c */

/* This file is included by jccolor.c */

/* This file is included by jccolor.c */

/* This file is included by jccolor.c */

/* This file is included by jccolor.c */

/* This file is included by jccolor.c */

/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */
#[inline(always)]
pub unsafe extern "C" fn extbgr_ycc_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh0 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh0;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(0) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            /* Cb */
            /* Cb */
            /* Cb */
            /* Cb */
            /* Cb */
            /* Cb */
            /* Cb */
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            /* Cr */
            /* Cr */
            /* Cr */
            /* Cr */
            /* Cr */
            /* Cr */
            /* Cr */
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_ycc_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh1 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh1;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(2) as libc::c_int;
            inptr = inptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_ycc_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh2 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh2;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(2) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxbgr_ycc_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh3 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh3;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(3) as libc::c_int;
            g = *inptr.offset(2) as libc::c_int;
            b = *inptr.offset(1) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_ycc_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh4 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh4;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(0) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_ycc_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh5 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh5;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(2) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxrgb_ycc_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh6 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh6;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(1) as libc::c_int;
            g = *inptr.offset(2) as libc::c_int;
            b = *inptr.offset(3) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
/* *************** Cases other than RGB -> YCbCr **************/

/* *************** Cases other than RGB -> YCbCr **************/

/* *************** Cases other than RGB -> YCbCr **************/

/* *************** Cases other than RGB -> YCbCr **************/

/* *************** Cases other than RGB -> YCbCr **************/

/* *************** Cases other than RGB -> YCbCr **************/

/* *************** Cases other than RGB -> YCbCr **************/

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */
#[inline(always)]
pub unsafe extern "C" fn rgb_gray_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh7 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh7;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(2) as libc::c_int;
            inptr = inptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            /* Y */
            /* Y */
            /* Y */
            /* Y */
            /* Y */
            /* Y */
            /* Y */
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxrgb_gray_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh8 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh8;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(1) as libc::c_int;
            g = *inptr.offset(2) as libc::c_int;
            b = *inptr.offset(3) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxbgr_gray_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh9 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh9;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(3) as libc::c_int;
            g = *inptr.offset(2) as libc::c_int;
            b = *inptr.offset(1) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_gray_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh10 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh10;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(0) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgr_gray_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh11 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh11;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(0) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_gray_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh12 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh12;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(2) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_gray_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh13 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh13;
        outptr = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0) as libc::c_int;
            g = *inptr.offset(1) as libc::c_int;
            b = *inptr.offset(2) as libc::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col =  col + 1
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles extended RGB->plain RGB conversion
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles extended RGB->plain RGB conversion
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles extended RGB->plain RGB conversion
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles extended RGB->plain RGB conversion
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles extended RGB->plain RGB conversion
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles extended RGB->plain RGB conversion
 */

/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles extended RGB->plain RGB conversion
 */
#[inline(always)]
pub unsafe extern "C" fn extxrgb_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh14 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh14;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) =
                *inptr.offset(1) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) =
                *inptr.offset(2) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) =
                *inptr.offset(3) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxbgr_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh15 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh15;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) =
                *inptr.offset(3) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) =
                *inptr.offset(2) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) =
                *inptr.offset(1) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh16 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh16;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) =
                *inptr.offset(2) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) =
                *inptr.offset(1) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) =
                *inptr.offset(0) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh17 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh17;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) =
                *inptr.offset(0) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) =
                *inptr.offset(1) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) =
                *inptr.offset(2) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh18 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh18;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) =
                *inptr.offset(0) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) =
                *inptr.offset(1) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) =
                *inptr.offset(2) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgr_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh19 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh19;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) =
                *inptr.offset(2) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) =
                *inptr.offset(1) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) =
                *inptr.offset(0) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            col =  col + 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb_convert_internal(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr2: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh20 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh20;
        outptr0 = *(*output_buf.offset(0)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row =  output_row + 1;
        col = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) =
                *inptr.offset(0) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr1.offset(col as isize) =
                *inptr.offset(1) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            *outptr2.offset(col as isize) =
                *inptr.offset(2) as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            inptr = inptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            col =  col + 1
        }
    }
}
use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
use crate::jmorecfg_h::MAXJSAMPLE;
use crate::jmorecfg_h::RGB_PIXELSIZE_5;
use crate::src::jccolor::B_CB_OFF;
use crate::src::jccolor::B_CR_OFF;
use crate::src::jccolor::B_Y_OFF;
use crate::src::jccolor::G_CB_OFF;
use crate::src::jccolor::G_CR_OFF;
use crate::src::jccolor::G_Y_OFF;
use crate::src::jccolor::RGB_PIXELSIZE;
use crate::src::jccolor::RGB_PIXELSIZE_0;
use crate::src::jccolor::RGB_PIXELSIZE_1;
use crate::src::jccolor::RGB_PIXELSIZE_2;
use crate::src::jccolor::RGB_PIXELSIZE_3;
use crate::src::jccolor::RGB_PIXELSIZE_4;
use crate::src::jccolor::R_CB_OFF;
use crate::src::jccolor::R_CR_OFF;
use crate::src::jccolor::R_Y_OFF;
use crate::src::jccolor::SCALEBITS;
