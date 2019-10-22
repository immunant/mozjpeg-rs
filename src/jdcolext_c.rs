use crate::jmorecfg_h::{JDIMENSION, JSAMPLE};
use crate::jpegint_h::JLONG;
use crate::jpeglib_h::{
    j_decompress_ptr, jpeg_decompress_struct, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
};
use crate::src::jdcolor::{my_cconvert_ptr, my_color_deconverter};
use libc::{self, c_int};
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
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
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
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh0 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh0;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
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
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
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
            *outptr.offset(RGB_ALPHA_0 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extrgbx_convert_internal(
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
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh1 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh1;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(RGB_RED_2 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            );
            *outptr.offset(RGB_BLUE_2 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_rgb_convert_internal(
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
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh2 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh2;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(RGB_RED_5 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_5 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            );
            *outptr.offset(RGB_BLUE_5 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extxrgb_convert_internal(
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
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh3 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh3;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(RGB_RED as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            );
            *outptr.offset(RGB_BLUE as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extbgr_convert_internal(
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
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh4 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh4;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(RGB_RED_3 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            );
            *outptr.offset(RGB_BLUE_3 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extbgrx_convert_internal(
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
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh5 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh5;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(RGB_RED_1 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            );
            *outptr.offset(RGB_BLUE_1 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn ycc_extrgb_convert_internal(
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
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh6 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh6;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(RGB_RED_4 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int)
                    as isize,
            );
            *outptr.offset(RGB_BLUE_4 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_add(1)
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
        let fresh7 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0)).offset(fresh7 as isize);
        let fresh8 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh8;
        col = 0i32 as JDIMENSION;
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
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extrgbx_convert_internal(
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
        let fresh11 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0)).offset(fresh11 as isize);
        let fresh12 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh12;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh13 = *outptr.offset(RGB_BLUE_2 as isize);
            *fresh13 = *inptr.offset(col as isize);
            let ref mut fresh14 = *outptr.offset(RGB_GREEN_2 as isize);
            *fresh14 = *fresh13;
            *outptr.offset(RGB_RED_2 as isize) = *fresh14;
            *outptr.offset(RGB_ALPHA_2 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extbgr_convert_internal(
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
        let fresh15 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0)).offset(fresh15 as isize);
        let fresh16 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh16;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh17 = *outptr.offset(RGB_BLUE_3 as isize);
            *fresh17 = *inptr.offset(col as isize);
            let ref mut fresh18 = *outptr.offset(RGB_GREEN_3 as isize);
            *fresh18 = *fresh17;
            *outptr.offset(RGB_RED_3 as isize) = *fresh18;
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extbgrx_convert_internal(
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
        let fresh19 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0)).offset(fresh19 as isize);
        let fresh20 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh20;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh21 = *outptr.offset(RGB_BLUE_1 as isize);
            *fresh21 = *inptr.offset(col as isize);
            let ref mut fresh22 = *outptr.offset(RGB_GREEN_1 as isize);
            *fresh22 = *fresh21;
            *outptr.offset(RGB_RED_1 as isize) = *fresh22;
            *outptr.offset(RGB_ALPHA_1 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extxbgr_convert_internal(
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
        let fresh23 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0)).offset(fresh23 as isize);
        let fresh24 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh24;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh25 = *outptr.offset(RGB_BLUE_0 as isize);
            *fresh25 = *inptr.offset(col as isize);
            let ref mut fresh26 = *outptr.offset(RGB_GREEN_0 as isize);
            *fresh26 = *fresh25;
            *outptr.offset(RGB_RED_0 as isize) = *fresh26;
            *outptr.offset(RGB_ALPHA_0 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_extxrgb_convert_internal(
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
        let fresh27 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0)).offset(fresh27 as isize);
        let fresh28 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh28;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh29 = *outptr.offset(RGB_BLUE as isize);
            *fresh29 = *inptr.offset(col as isize);
            let ref mut fresh30 = *outptr.offset(RGB_GREEN as isize);
            *fresh30 = *fresh29;
            *outptr.offset(RGB_RED as isize) = *fresh30;
            *outptr.offset(RGB_ALPHA as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn gray_rgb_convert_internal(
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
        let fresh31 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0)).offset(fresh31 as isize);
        let fresh32 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh32;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh33 = *outptr.offset(RGB_BLUE_5 as isize);
            *fresh33 = *inptr.offset(col as isize);
            let ref mut fresh34 = *outptr.offset(RGB_GREEN_5 as isize);
            *fresh34 = *fresh33;
            *outptr.offset(RGB_RED_5 as isize) = *fresh34;
            outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_add(1)
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
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh35 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh35;
        col = 0i32 as JDIMENSION;
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
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh36 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh36;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_4 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_4 as isize) = *inptr2.offset(col as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extrgbx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh37 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh37;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_2 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_2 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extbgrx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh38 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh38;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_1 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_1 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extxbgr_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh39 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh39;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_0 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_0 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_0 as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_extxrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh40 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh40;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA as isize) = 0xffi32 as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_add(1)
        }
    }
}
#[inline(always)]
pub unsafe extern "C" fn rgb_rgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh41 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh41;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_5 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_5 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_5 as isize) = *inptr2.offset(col as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_add(1)
        }
    }
}

use crate::jmorecfg_h::{
    EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN,
    EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE,
    EXT_RGBX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_XBGR_BLUE,
    EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN,
    EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, RGB_BLUE_5, RGB_GREEN_5, RGB_PIXELSIZE_5, RGB_RED_5,
};
use crate::src::jdcolor::{
    RGB_ALPHA, RGB_ALPHA_0, RGB_ALPHA_1, RGB_ALPHA_2, RGB_BLUE, RGB_BLUE_0, RGB_BLUE_1, RGB_BLUE_2,
    RGB_BLUE_3, RGB_BLUE_4, RGB_GREEN, RGB_GREEN_0, RGB_GREEN_1, RGB_GREEN_2, RGB_GREEN_3,
    RGB_GREEN_4, RGB_PIXELSIZE, RGB_PIXELSIZE_0, RGB_PIXELSIZE_1, RGB_PIXELSIZE_2, RGB_PIXELSIZE_3,
    RGB_PIXELSIZE_4, RGB_RED, RGB_RED_0, RGB_RED_1, RGB_RED_2, RGB_RED_3, RGB_RED_4,
};
