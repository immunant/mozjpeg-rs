use crate::jmorecfg_h::{JDIMENSION, JSAMPLE};
use crate::jpegint_h::JLONG;
use crate::jpeglib_h::{j_compress_ptr, JSAMPARRAY, JSAMPIMAGE, JSAMPROW};
use crate::src::jccolor::my_cconvert_ptr;
use libc::c_int;
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
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh0 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh0;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(2) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(0) as c_int;
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
                >> SCALEBITS) as JSAMPLE;
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
                >> SCALEBITS) as JSAMPLE;
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
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh1 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh1;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(0) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(2) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_5 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh2 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh2;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(0) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(2) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxbgr_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh3 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh3;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(3) as c_int;
            let mut g: c_int = *inptr.offset(2) as c_int;
            let mut b: c_int = *inptr.offset(1) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh4 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh4;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(2) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(0) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh5 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh5;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(0) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(2) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxrgb_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh6 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh6;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(1) as c_int;
            let mut g: c_int = *inptr.offset(2) as c_int;
            let mut b: c_int = *inptr.offset(3) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
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
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh7 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh7;
        let mut outptr: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(0) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(2) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_5 as isize);
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
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxrgb_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh8 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh8;
        let mut outptr: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(1) as c_int;
            let mut g: c_int = *inptr.offset(2) as c_int;
            let mut b: c_int = *inptr.offset(3) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxbgr_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh9 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh9;
        let mut outptr: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(3) as c_int;
            let mut g: c_int = *inptr.offset(2) as c_int;
            let mut b: c_int = *inptr.offset(1) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh10 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh10;
        let mut outptr: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(2) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(0) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgr_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh11 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh11;
        let mut outptr: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(2) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(0) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh12 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh12;
        let mut outptr: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(0) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(2) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;

    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;

    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh13 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh13;
        let mut outptr: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            let mut r: c_int = *inptr.offset(0) as c_int;
            let mut g: c_int = *inptr.offset(1) as c_int;
            let mut b: c_int = *inptr.offset(2) as c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col += 1
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
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh14 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh14;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(1);
            *outptr1.offset(col as isize) = *inptr.offset(2);
            *outptr2.offset(col as isize) = *inptr.offset(3);
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extxbgr_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh15 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh15;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(3);
            *outptr1.offset(col as isize) = *inptr.offset(2);
            *outptr2.offset(col as isize) = *inptr.offset(1);
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh16 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh16;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(2);
            *outptr1.offset(col as isize) = *inptr.offset(1);
            *outptr2.offset(col as isize) = *inptr.offset(0);
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh17 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh17;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(0);
            *outptr1.offset(col as isize) = *inptr.offset(1);
            *outptr2.offset(col as isize) = *inptr.offset(2);
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh18 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh18;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(0);
            *outptr1.offset(col as isize) = *inptr.offset(1);
            *outptr2.offset(col as isize) = *inptr.offset(2);
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn extbgr_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh19 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh19;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(2);
            *outptr1.offset(col as isize) = *inptr.offset(1);
            *outptr2.offset(col as isize) = *inptr.offset(0);
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            col += 1
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        let fresh20 = input_buf;
        input_buf = input_buf.offset(1);

        let mut inptr: JSAMPROW = *fresh20;
        let mut outptr0: JSAMPROW = *(*output_buf.offset(0)).offset(output_row as isize);
        let mut outptr1: JSAMPROW = *(*output_buf.offset(1)).offset(output_row as isize);
        let mut outptr2: JSAMPROW = *(*output_buf.offset(2)).offset(output_row as isize);
        output_row += 1;
        let mut col: JDIMENSION = 0u32;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(0);
            *outptr1.offset(col as isize) = *inptr.offset(1);
            *outptr2.offset(col as isize) = *inptr.offset(2);
            inptr = inptr.offset(RGB_PIXELSIZE_5 as isize);
            col += 1
        }
    }
}

use crate::jmorecfg_h::RGB_PIXELSIZE_5;
use crate::src::jccolor::{
    B_CB_OFF, B_CR_OFF, B_Y_OFF, G_CB_OFF, G_CR_OFF, G_Y_OFF, RGB_PIXELSIZE, RGB_PIXELSIZE_0,
    RGB_PIXELSIZE_1, RGB_PIXELSIZE_2, RGB_PIXELSIZE_3, RGB_PIXELSIZE_4, R_CB_OFF, R_CR_OFF,
    R_Y_OFF, SCALEBITS,
};
