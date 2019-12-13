use ::libc;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:19"]
pub mod jmorecfg_h {

    pub static mut rgb_green: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_GREEN_5,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_GREEN,
        crate::jmorecfg_h::EXT_RGBX_GREEN,
        crate::jmorecfg_h::EXT_BGR_GREEN,
        crate::jmorecfg_h::EXT_BGRX_GREEN,
        crate::jmorecfg_h::EXT_XBGR_GREEN,
        crate::jmorecfg_h::EXT_XRGB_GREEN,
        crate::jmorecfg_h::EXT_RGBX_GREEN,
        crate::jmorecfg_h::EXT_BGRX_GREEN,
        crate::jmorecfg_h::EXT_XBGR_GREEN,
        crate::jmorecfg_h::EXT_XRGB_GREEN,
        -(1 as libc::c_int),
    ];

    pub static mut rgb_blue: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_BLUE_5,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_BLUE,
        crate::jmorecfg_h::EXT_RGBX_BLUE,
        crate::jmorecfg_h::EXT_BGR_BLUE,
        crate::jmorecfg_h::EXT_BGRX_BLUE,
        crate::jmorecfg_h::EXT_XBGR_BLUE,
        crate::jmorecfg_h::EXT_XRGB_BLUE,
        crate::jmorecfg_h::EXT_RGBX_BLUE,
        crate::jmorecfg_h::EXT_BGRX_BLUE,
        crate::jmorecfg_h::EXT_XBGR_BLUE,
        crate::jmorecfg_h::EXT_XRGB_BLUE,
        -(1 as libc::c_int),
    ];

    pub static mut rgb_pixelsize: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_PIXELSIZE_5,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_PIXELSIZE,
        crate::jmorecfg_h::EXT_RGBX_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGRX_PIXELSIZE,
        crate::jmorecfg_h::EXT_XBGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_XRGB_PIXELSIZE,
        crate::jmorecfg_h::EXT_RGBX_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGRX_PIXELSIZE,
        crate::jmorecfg_h::EXT_XBGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_XRGB_PIXELSIZE,
        -(1 as libc::c_int),
    ];

    pub static mut rgb_red: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_RED_5,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_RED,
        crate::jmorecfg_h::EXT_RGBX_RED,
        crate::jmorecfg_h::EXT_BGR_RED,
        crate::jmorecfg_h::EXT_BGRX_RED,
        crate::jmorecfg_h::EXT_XBGR_RED,
        crate::jmorecfg_h::EXT_XRGB_RED,
        crate::jmorecfg_h::EXT_RGBX_RED,
        crate::jmorecfg_h::EXT_BGRX_RED,
        crate::jmorecfg_h::EXT_XBGR_RED,
        crate::jmorecfg_h::EXT_XRGB_RED,
        -(1 as libc::c_int),
    ];
    /* JPEG_INTERNAL_OPTIONS */
    /* FAST_FLOAT should be either float or double, whichever is done faster
     * by your compiler.  (Note that this type is only used in the floating point
     * DCT routines, so it only matters if you've defined DCT_FLOAT_SUPPORTED.)
     */
    /* prefer 16-bit with SIMD for parellelism */
    /* On some machines (notably 68000 series) "int" is 32 bits, but multiplying
     * two 16-bit shorts is faster than multiplying two ints.  Define MULTIPLIER
     * as short on such a machine.  MULTIPLIER must be at least 16 bits wide.
     */
    /* Definitions for speed-related optimizations. */
}

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jdcolext.c:95"]
pub mod jdcolext_c {
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh0 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh0;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
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
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
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
                *outptr.offset(RGB_ALPHA_0 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
                col = col.wrapping_add(1)
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh1 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh1;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                y = *inptr0.offset(col as isize) as libc::c_int;
                cb = *inptr1.offset(col as isize) as libc::c_int;
                cr = *inptr2.offset(col as isize) as libc::c_int;
                *outptr.offset(RGB_RED_2 as isize) =
                    *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
                *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                );
                *outptr.offset(RGB_BLUE_2 as isize) =
                    *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
                *outptr.offset(RGB_ALPHA_2 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
                col = col.wrapping_add(1)
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh2 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh2;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                y = *inptr0.offset(col as isize) as libc::c_int;
                cb = *inptr1.offset(col as isize) as libc::c_int;
                cr = *inptr2.offset(col as isize) as libc::c_int;
                *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                    *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
                *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                );
                *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                    *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
                outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
                col = col.wrapping_add(1)
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh3 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh3;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                y = *inptr0.offset(col as isize) as libc::c_int;
                cb = *inptr1.offset(col as isize) as libc::c_int;
                cr = *inptr2.offset(col as isize) as libc::c_int;
                *outptr.offset(RGB_RED as isize) =
                    *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
                *outptr.offset(RGB_GREEN as isize) = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                );
                *outptr.offset(RGB_BLUE as isize) =
                    *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
                *outptr.offset(RGB_ALPHA as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE as isize);
                col = col.wrapping_add(1)
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh4 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh4;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                y = *inptr0.offset(col as isize) as libc::c_int;
                cb = *inptr1.offset(col as isize) as libc::c_int;
                cr = *inptr2.offset(col as isize) as libc::c_int;
                *outptr.offset(RGB_RED_3 as isize) =
                    *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
                *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh5 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh5;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                y = *inptr0.offset(col as isize) as libc::c_int;
                cb = *inptr1.offset(col as isize) as libc::c_int;
                cr = *inptr2.offset(col as isize) as libc::c_int;
                *outptr.offset(RGB_RED_1 as isize) =
                    *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
                *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                );
                *outptr.offset(RGB_BLUE_1 as isize) =
                    *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
                *outptr.offset(RGB_ALPHA_1 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
                col = col.wrapping_add(1)
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh6 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh6;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                y = *inptr0.offset(col as isize) as libc::c_int;
                cb = *inptr1.offset(col as isize) as libc::c_int;
                cr = *inptr2.offset(col as isize) as libc::c_int;
                *outptr.offset(RGB_RED_4 as isize) =
                    *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
                *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
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
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh7 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh7 as isize);
            let fresh8 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh8;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
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
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh11 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh11 as isize);
            let fresh12 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh12;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let ref mut fresh13 = *outptr.offset(RGB_BLUE_2 as isize);
                *fresh13 = *inptr.offset(col as isize);
                let ref mut fresh14 = *outptr.offset(RGB_GREEN_2 as isize);
                *fresh14 = *fresh13;
                *outptr.offset(RGB_RED_2 as isize) = *fresh14;
                *outptr.offset(RGB_ALPHA_2 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
                col = col.wrapping_add(1)
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
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh15 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh15 as isize);
            let fresh16 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh16;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
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
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh19 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh19 as isize);
            let fresh20 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh20;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let ref mut fresh21 = *outptr.offset(RGB_BLUE_1 as isize);
                *fresh21 = *inptr.offset(col as isize);
                let ref mut fresh22 = *outptr.offset(RGB_GREEN_1 as isize);
                *fresh22 = *fresh21;
                *outptr.offset(RGB_RED_1 as isize) = *fresh22;
                *outptr.offset(RGB_ALPHA_1 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
                col = col.wrapping_add(1)
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
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh23 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh23 as isize);
            let fresh24 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh24;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let ref mut fresh25 = *outptr.offset(RGB_BLUE_0 as isize);
                *fresh25 = *inptr.offset(col as isize);
                let ref mut fresh26 = *outptr.offset(RGB_GREEN_0 as isize);
                *fresh26 = *fresh25;
                *outptr.offset(RGB_RED_0 as isize) = *fresh26;
                *outptr.offset(RGB_ALPHA_0 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
                col = col.wrapping_add(1)
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
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh27 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh27 as isize);
            let fresh28 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh28;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let ref mut fresh29 = *outptr.offset(RGB_BLUE as isize);
                *fresh29 = *inptr.offset(col as isize);
                let ref mut fresh30 = *outptr.offset(RGB_GREEN as isize);
                *fresh30 = *fresh29;
                *outptr.offset(RGB_RED as isize) = *fresh30;
                *outptr.offset(RGB_ALPHA as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE as isize);
                col = col.wrapping_add(1)
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
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh31 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh31 as isize);
            let fresh32 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh32;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let ref mut fresh33 = *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize);
                *fresh33 = *inptr.offset(col as isize);
                let ref mut fresh34 = *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize);
                *fresh34 = *fresh33;
                *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) = *fresh34;
                outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
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
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh35 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh35;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
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
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh36 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh36;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
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
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh37 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh37;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr.offset(RGB_RED_2 as isize) = *inptr0.offset(col as isize);
                *outptr.offset(RGB_GREEN_2 as isize) = *inptr1.offset(col as isize);
                *outptr.offset(RGB_BLUE_2 as isize) = *inptr2.offset(col as isize);
                *outptr.offset(RGB_ALPHA_2 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
                col = col.wrapping_add(1)
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
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh38 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh38;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr.offset(RGB_RED_1 as isize) = *inptr0.offset(col as isize);
                *outptr.offset(RGB_GREEN_1 as isize) = *inptr1.offset(col as isize);
                *outptr.offset(RGB_BLUE_1 as isize) = *inptr2.offset(col as isize);
                *outptr.offset(RGB_ALPHA_1 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
                col = col.wrapping_add(1)
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
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh39 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh39;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr.offset(RGB_RED_0 as isize) = *inptr0.offset(col as isize);
                *outptr.offset(RGB_GREEN_0 as isize) = *inptr1.offset(col as isize);
                *outptr.offset(RGB_BLUE_0 as isize) = *inptr2.offset(col as isize);
                *outptr.offset(RGB_ALPHA_0 as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
                col = col.wrapping_add(1)
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
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh40 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh40;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr.offset(RGB_RED as isize) = *inptr0.offset(col as isize);
                *outptr.offset(RGB_GREEN as isize) = *inptr1.offset(col as isize);
                *outptr.offset(RGB_BLUE as isize) = *inptr2.offset(col as isize);
                *outptr.offset(RGB_ALPHA as isize) =
                    0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(RGB_PIXELSIZE as isize);
                col = col.wrapping_add(1)
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
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh41 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh41;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                    *inptr0.offset(col as isize);
                *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                    *inptr1.offset(col as isize);
                *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                    *inptr2.offset(col as isize);
                outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
                col = col.wrapping_add(1)
            }
        }
    }

    use crate::jmorecfg_h::JDIMENSION;
    use crate::jmorecfg_h::JSAMPLE;
    use crate::jmorecfg_h::RGB_BLUE_5;
    use crate::jmorecfg_h::RGB_GREEN_5;
    use crate::jmorecfg_h::RGB_PIXELSIZE_5;
    use crate::jmorecfg_h::RGB_RED_5;
    use crate::jpegint_h::JLONG;
    use crate::jpeglib_h::JSAMPROW;
    use crate::src::jdcolor::my_cconvert_ptr;
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
}
#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jdcol565.c:627"]
pub mod jdcol565_c {
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
    /* This file is included by jdcolor.c */
    #[inline(always)]

    pub unsafe extern "C" fn ycc_rgb565_convert_le(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh53 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh53;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh54 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh54 as libc::c_int;
                let fresh55 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh55 as libc::c_int;
                let fresh56 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh56 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh57 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh57 as libc::c_int;
                let fresh58 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh58 as libc::c_int;
                let fresh59 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh59 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                let fresh60 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh60 as libc::c_int;
                let fresh61 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh61 as libc::c_int;
                let fresh62 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh62 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_long
                    | rgb;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                y = *inptr0 as libc::c_int;
                cb = *inptr1 as libc::c_int;
                cr = *inptr2 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn ycc_rgb565_convert_be(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh63 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh63;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh64 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh64 as libc::c_int;
                let fresh65 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh65 as libc::c_int;
                let fresh66 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh66 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh67 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh67 as libc::c_int;
                let fresh68 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh68 as libc::c_int;
                let fresh69 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh69 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                let fresh70 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh70 as libc::c_int;
                let fresh71 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh71 as libc::c_int;
                let fresh72 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh72 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = rgb << 16 as libc::c_int
                    | (r & 0xf8 as libc::c_int as libc::c_uint
                        | g >> 5 as libc::c_int
                        | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                        | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                        as libc::c_long;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                y = *inptr0 as libc::c_int;
                cb = *inptr1 as libc::c_int;
                cr = *inptr2 as libc::c_int;
                r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
                g = *range_limit.offset(
                    (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as isize,
                ) as libc::c_uint;
                b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn ycc_rgb565D_convert_le(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        let mut d0: crate::jpegint_h::JLONG =
            dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh73 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh73;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh74 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh74 as libc::c_int;
                let fresh75 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh75 as libc::c_int;
                let fresh76 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh76 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh77 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh77 as libc::c_int;
                let fresh78 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh78 as libc::c_int;
                let fresh79 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh79 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                let fresh80 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh80 as libc::c_int;
                let fresh81 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh81 as libc::c_int;
                let fresh82 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh82 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_long
                    | rgb;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                y = *inptr0 as libc::c_int;
                cb = *inptr1 as libc::c_int;
                cr = *inptr2 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn ycc_rgb565D_convert_be(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
        let mut d0: crate::jpegint_h::JLONG =
            dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh83 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh83;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh84 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh84 as libc::c_int;
                let fresh85 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh85 as libc::c_int;
                let fresh86 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh86 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh87 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh87 as libc::c_int;
                let fresh88 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh88 as libc::c_int;
                let fresh89 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh89 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                let fresh90 = inptr0;
                inptr0 = inptr0.offset(1);
                y = *fresh90 as libc::c_int;
                let fresh91 = inptr1;
                inptr1 = inptr1.offset(1);
                cb = *fresh91 as libc::c_int;
                let fresh92 = inptr2;
                inptr2 = inptr2.offset(1);
                cr = *fresh92 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = rgb << 16 as libc::c_int
                    | (r & 0xf8 as libc::c_int as libc::c_uint
                        | g >> 5 as libc::c_int
                        | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                        | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                        as libc::c_long;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                y = *inptr0 as libc::c_int;
                cb = *inptr1 as libc::c_int;
                cr = *inptr2 as libc::c_int;
                r = *range_limit.offset(
                    ((y + *Crrtab.offset(cr as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int) as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    ((y + *Cbbtab.offset(cb as isize)) as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn rgb_rgb565_convert_le(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh93 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh93;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh94 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *fresh94 as libc::c_int as libc::c_uint;
                let fresh95 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *fresh95 as libc::c_int as libc::c_uint;
                let fresh96 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *fresh96 as libc::c_int as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh97 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *fresh97 as libc::c_int as libc::c_uint;
                let fresh98 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *fresh98 as libc::c_int as libc::c_uint;
                let fresh99 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *fresh99 as libc::c_int as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                let fresh100 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *fresh100 as libc::c_int as libc::c_uint;
                let fresh101 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *fresh101 as libc::c_int as libc::c_uint;
                let fresh102 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *fresh102 as libc::c_int as libc::c_uint;
                rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_long
                    | rgb;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                r = *inptr0 as libc::c_int as libc::c_uint;
                g = *inptr1 as libc::c_int as libc::c_uint;
                b = *inptr2 as libc::c_int as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn rgb_rgb565_convert_be(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh103 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh103;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh104 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *fresh104 as libc::c_int as libc::c_uint;
                let fresh105 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *fresh105 as libc::c_int as libc::c_uint;
                let fresh106 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *fresh106 as libc::c_int as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh107 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *fresh107 as libc::c_int as libc::c_uint;
                let fresh108 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *fresh108 as libc::c_int as libc::c_uint;
                let fresh109 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *fresh109 as libc::c_int as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                let fresh110 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *fresh110 as libc::c_int as libc::c_uint;
                let fresh111 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *fresh111 as libc::c_int as libc::c_uint;
                let fresh112 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *fresh112 as libc::c_int as libc::c_uint;
                rgb = rgb << 16 as libc::c_int
                    | (r & 0xf8 as libc::c_int as libc::c_uint
                        | g >> 5 as libc::c_int
                        | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                        | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                        as libc::c_long;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                r = *inptr0 as libc::c_int as libc::c_uint;
                g = *inptr1 as libc::c_int as libc::c_uint;
                b = *inptr2 as libc::c_int as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn rgb_rgb565D_convert_be(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut d0: crate::jpegint_h::JLONG =
            dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh113 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh113;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh114 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *range_limit.offset(
                    (*fresh114 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                let fresh115 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *range_limit.offset(
                    (*fresh115 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                let fresh116 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *range_limit.offset(
                    (*fresh116 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh117 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *range_limit.offset(
                    (*fresh117 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                let fresh118 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *range_limit.offset(
                    (*fresh118 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                let fresh119 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *range_limit.offset(
                    (*fresh119 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                let fresh120 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *range_limit.offset(
                    (*fresh120 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                let fresh121 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *range_limit.offset(
                    (*fresh121 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                let fresh122 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *range_limit.offset(
                    (*fresh122 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = rgb << 16 as libc::c_int
                    | (r & 0xf8 as libc::c_int as libc::c_uint
                        | g >> 5 as libc::c_int
                        | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                        | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                        as libc::c_long;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                r = *range_limit.offset(
                    (*inptr0 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    (*inptr1 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    (*inptr2 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn rgb_rgb565D_convert_le(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut d0: crate::jpegint_h::JLONG =
            dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh123 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh123;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh124 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *range_limit.offset(
                    (*fresh124 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                let fresh125 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *range_limit.offset(
                    (*fresh125 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                let fresh126 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *range_limit.offset(
                    (*fresh126 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh127 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *range_limit.offset(
                    (*fresh127 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                let fresh128 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *range_limit.offset(
                    (*fresh128 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                let fresh129 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *range_limit.offset(
                    (*fresh129 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                let fresh130 = inptr0;
                inptr0 = inptr0.offset(1);
                r = *range_limit.offset(
                    (*fresh130 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                let fresh131 = inptr1;
                inptr1 = inptr1.offset(1);
                g = *range_limit.offset(
                    (*fresh131 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                let fresh132 = inptr2;
                inptr2 = inptr2.offset(1);
                b = *range_limit.offset(
                    (*fresh132 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_long
                    | rgb;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                r = *range_limit.offset(
                    (*inptr0 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                g = *range_limit.offset(
                    (*inptr1 as libc::c_int as libc::c_long
                        + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                        as isize,
                ) as libc::c_uint;
                b = *range_limit.offset(
                    (*inptr2 as libc::c_int as libc::c_long
                        + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn gray_rgb565_convert_be(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut g: libc::c_uint = 0;
            let fresh133 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh133 as isize);
            let fresh134 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh134;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh135 = inptr;
                inptr = inptr.offset(1);
                g = *fresh135 as libc::c_uint;
                rgb = (g & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh136 = inptr;
                inptr = inptr.offset(1);
                g = *fresh136 as libc::c_uint;
                rgb = (g & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                let fresh137 = inptr;
                inptr = inptr.offset(1);
                g = *fresh137 as libc::c_uint;
                rgb = rgb << 16 as libc::c_int
                    | (g & 0xf8 as libc::c_int as libc::c_uint
                        | g >> 5 as libc::c_int
                        | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                        | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                        as libc::c_long;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                g = *inptr as libc::c_uint;
                rgb = (g & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn gray_rgb565_convert_le(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut g: libc::c_uint = 0;
            let fresh138 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh138 as isize);
            let fresh139 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh139;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh140 = inptr;
                inptr = inptr.offset(1);
                g = *fresh140 as libc::c_uint;
                rgb = (g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh141 = inptr;
                inptr = inptr.offset(1);
                g = *fresh141 as libc::c_uint;
                rgb = (g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                let fresh142 = inptr;
                inptr = inptr.offset(1);
                g = *fresh142 as libc::c_uint;
                rgb = ((g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_long
                    | rgb;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                g = *inptr as libc::c_uint;
                rgb = (g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn gray_rgb565D_convert_le(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut d0: crate::jpegint_h::JLONG =
            dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut g: libc::c_uint = 0;
            let fresh143 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh143 as isize);
            let fresh144 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh144;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh145 = inptr;
                inptr = inptr.offset(1);
                g = *fresh145 as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh146 = inptr;
                inptr = inptr.offset(1);
                g = *fresh146 as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                let fresh147 = inptr;
                inptr = inptr.offset(1);
                g = *fresh147 as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = ((g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_long
                    | rgb;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                g = *inptr as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (g << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                    | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                    | g >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn gray_rgb565D_convert_be(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut input_row: crate::jmorecfg_h::JDIMENSION,
        mut output_buf: crate::jpeglib_h::JSAMPARRAY,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
        let mut d0: crate::jpegint_h::JLONG =
            dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let mut rgb: crate::jpegint_h::JLONG = 0;
            let mut g: libc::c_uint = 0;
            let fresh148 = input_row;
            input_row = input_row.wrapping_add(1);
            inptr = *(*input_buf.offset(0 as libc::c_int as isize)).offset(fresh148 as isize);
            let fresh149 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh149;
            if outptr as crate::stddef_h::size_t & 3 as libc::c_int as libc::c_ulong != 0 {
                let fresh150 = inptr;
                inptr = inptr.offset(1);
                g = *fresh150 as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (g & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
                outptr = outptr.offset(2 as libc::c_int as isize);
                num_cols = num_cols.wrapping_sub(1)
            }
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols >> 1 as libc::c_int {
                let fresh151 = inptr;
                inptr = inptr.offset(1);
                g = *fresh151 as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (g & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                let fresh152 = inptr;
                inptr = inptr.offset(1);
                g = *fresh152 as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = rgb << 16 as libc::c_int
                    | (g & 0xf8 as libc::c_int as libc::c_uint
                        | g >> 5 as libc::c_int
                        | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                        | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                        as libc::c_long;
                d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                    | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
                *(outptr as *mut libc::c_int) = rgb as libc::c_int;
                outptr = outptr.offset(4 as libc::c_int as isize);
                col = col.wrapping_add(1)
            }
            if num_cols & 1 as libc::c_int as libc::c_uint != 0 {
                g = *inptr as libc::c_uint;
                g = *range_limit.offset(
                    (g as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
                ) as libc::c_uint;
                rgb = (g & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | g << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as crate::jpegint_h::JLONG;
                *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
            }
        }
    }

    use crate::jmorecfg_h::INT16;
    use crate::jmorecfg_h::JDIMENSION;
    use crate::jmorecfg_h::JSAMPLE;
    use crate::jpegint_h::JLONG;
    use crate::jpeglib_h::JSAMPROW;
    use crate::src::jdcolor::dither_matrix;
    use crate::src::jdcolor::my_cconvert_ptr;
    use crate::src::jdcolor::DITHER_MASK;
    use crate::stddef_h::size_t;
}
pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::CENTERJSAMPLE;
pub use crate::jmorecfg_h::EXT_BGRX_BLUE;
pub use crate::jmorecfg_h::EXT_BGRX_GREEN;
pub use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGRX_RED;
pub use crate::jmorecfg_h::EXT_BGR_BLUE;
pub use crate::jmorecfg_h::EXT_BGR_GREEN;
pub use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGR_RED;
pub use crate::jmorecfg_h::EXT_RGBX_BLUE;
pub use crate::jmorecfg_h::EXT_RGBX_GREEN;
pub use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGBX_RED;
pub use crate::jmorecfg_h::EXT_RGB_BLUE;
pub use crate::jmorecfg_h::EXT_RGB_GREEN;
pub use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGB_RED;
pub use crate::jmorecfg_h::EXT_XBGR_BLUE;
pub use crate::jmorecfg_h::EXT_XBGR_GREEN;
pub use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XBGR_RED;
pub use crate::jmorecfg_h::EXT_XRGB_BLUE;
pub use crate::jmorecfg_h::EXT_XRGB_GREEN;
pub use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XRGB_RED;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE_5;
pub use crate::jmorecfg_h::RGB_GREEN_5;
pub use crate::jmorecfg_h::RGB_PIXELSIZE_5;
pub use crate::jmorecfg_h::RGB_RED_5;
pub use crate::jmorecfg_h::TRUE;
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
pub use crate::jpegint_h::JLONG;
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
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::jdcolor::jdcol565_c::gray_rgb565D_convert_be;
pub use crate::src::jdcolor::jdcol565_c::gray_rgb565D_convert_le;
pub use crate::src::jdcolor::jdcol565_c::gray_rgb565_convert_be;
pub use crate::src::jdcolor::jdcol565_c::gray_rgb565_convert_le;
pub use crate::src::jdcolor::jdcol565_c::rgb_rgb565D_convert_be;
pub use crate::src::jdcolor::jdcol565_c::rgb_rgb565D_convert_le;
pub use crate::src::jdcolor::jdcol565_c::rgb_rgb565_convert_be;
pub use crate::src::jdcolor::jdcol565_c::rgb_rgb565_convert_le;
pub use crate::src::jdcolor::jdcol565_c::ycc_rgb565D_convert_be;
pub use crate::src::jdcolor::jdcol565_c::ycc_rgb565D_convert_le;
pub use crate::src::jdcolor::jdcol565_c::ycc_rgb565_convert_be;
pub use crate::src::jdcolor::jdcol565_c::ycc_rgb565_convert_le;
pub use crate::src::jdcolor::jdcolext_c::gray_extbgr_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::gray_extbgrx_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::gray_extrgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::gray_extrgbx_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::gray_extxbgr_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::gray_extxrgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::gray_rgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::rgb_extbgr_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::rgb_extbgrx_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::rgb_extrgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::rgb_extrgbx_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::rgb_extxbgr_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::rgb_extxrgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::rgb_rgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::ycc_extbgr_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::ycc_extbgrx_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::ycc_extrgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::ycc_extrgbx_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::ycc_extxbgr_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::ycc_extxrgb_convert_internal;
pub use crate::src::jdcolor::jdcolext_c::ycc_rgb_convert_internal;
pub use crate::src::jdcolor::jmorecfg_h::rgb_blue;
pub use crate::src::jdcolor::jmorecfg_h::rgb_green;
pub use crate::src::jdcolor::jmorecfg_h::rgb_pixelsize;
pub use crate::src::jdcolor::jmorecfg_h::rgb_red;
pub use crate::src::jerror::JERR_ARITH_NOTIMPL;
pub use crate::src::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_LENGTH;
pub use crate::src::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jerror::JERR_BAD_PARAM;
pub use crate::src::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::src::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jerror::JERR_BAD_PRECISION;
pub use crate::src::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jerror::JERR_BAD_STATE;
pub use crate::src::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jerror::JERR_DAC_INDEX;
pub use crate::src::jerror::JERR_DAC_VALUE;
pub use crate::src::jerror::JERR_DHT_INDEX;
pub use crate::src::jerror::JERR_DQT_INDEX;
pub use crate::src::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jerror::JERR_EMS_READ;
pub use crate::src::jerror::JERR_EMS_WRITE;
pub use crate::src::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jerror::JERR_FILE_READ;
pub use crate::src::jerror::JERR_FILE_WRITE;
pub use crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jerror::JERR_INPUT_EOF;
pub use crate::src::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jerror::JERR_MISSING_DATA;
pub use crate::src::jerror::JERR_MODE_CHANGE;
pub use crate::src::jerror::JERR_NOTIMPL;
pub use crate::src::jerror::JERR_NOT_COMPILED;
pub use crate::src::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jerror::JERR_NO_IMAGE;
pub use crate::src::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jerror::JERR_NO_SOI;
pub use crate::src::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jerror::JERR_TFILE_CREATE;
pub use crate::src::jerror::JERR_TFILE_READ;
pub use crate::src::jerror::JERR_TFILE_SEEK;
pub use crate::src::jerror::JERR_TFILE_WRITE;
pub use crate::src::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::src::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jerror::JERR_XMS_READ;
pub use crate::src::jerror::JERR_XMS_WRITE;
pub use crate::src::jerror::JMSG_COPYRIGHT;
pub use crate::src::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jerror::JMSG_NOMESSAGE;
pub use crate::src::jerror::JMSG_VERSION;
pub use crate::src::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jerror::JTRC_ADOBE;
pub use crate::src::jerror::JTRC_APP0;
pub use crate::src::jerror::JTRC_APP14;
pub use crate::src::jerror::JTRC_DAC;
pub use crate::src::jerror::JTRC_DHT;
pub use crate::src::jerror::JTRC_DQT;
pub use crate::src::jerror::JTRC_DRI;
pub use crate::src::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jerror::JTRC_EMS_OPEN;
pub use crate::src::jerror::JTRC_EOI;
pub use crate::src::jerror::JTRC_HUFFBITS;
pub use crate::src::jerror::JTRC_JFIF;
pub use crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jerror::JTRC_MISC_MARKER;
pub use crate::src::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jerror::JTRC_QUANTVALS;
pub use crate::src::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jerror::JTRC_RST;
pub use crate::src::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jerror::JTRC_SOF;
pub use crate::src::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jerror::JTRC_SOI;
pub use crate::src::jerror::JTRC_SOS;
pub use crate::src::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jerror::JTRC_THUMB_RGB;
pub use crate::src::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jerror::JTRC_XMS_OPEN;
pub use crate::src::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jerror::JWRN_BOGUS_ICC;
pub use crate::src::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jerror::JWRN_HIT_MARKER;
pub use crate::src::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jerror::JWRN_JPEG_EOF;
pub use crate::src::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::src::jutils::jcopy_sample_rows;
use crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb;
use crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb565;
use crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb565_convert;
use crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb_convert;
pub use crate::stdlib::C2RustUnnamed_0;

pub type my_cconvert_ptr = *mut my_color_deconverter;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_deconverter {
    pub pub_0: crate::jpegint_h::jpeg_color_deconverter,
    pub Cr_r_tab: *mut libc::c_int,
    pub Cb_b_tab: *mut libc::c_int,
    pub Cr_g_tab: *mut crate::jpegint_h::JLONG,
    pub Cb_g_tab: *mut crate::jpegint_h::JLONG,
    pub rgb_y_tab: *mut crate::jpegint_h::JLONG,
}
/* *************** YCbCr -> RGB conversion: most common case **************/
/* ***************   RGB -> Y   conversion: less common case **************/
/*
 * YCbCr is defined per CCIR 601-1, except that Cb and Cr are
 * normalized to the range 0..MAXJSAMPLE rather than -0.5 .. 0.5.
 * The conversion equations to be implemented are therefore
 *
 *      R = Y                + 1.40200 * Cr
 *      G = Y - 0.34414 * Cb - 0.71414 * Cr
 *      B = Y + 1.77200 * Cb
 *
 *      Y = 0.29900 * R + 0.58700 * G + 0.11400 * B
 *
 * where Cb and Cr represent the incoming values less CENTERJSAMPLE.
 * (These numbers are derived from TIFF 6.0 section 21, dated 3-June-92.)
 *
 * To avoid floating-point arithmetic, we represent the fractional constants
 * as integers scaled up by 2^16 (about 4 digits precision); we have to divide
 * the products by 2^16, with appropriate rounding, to get the correct answer.
 * Notice that Y, being an integral input, does not contribute any fraction
 * so it need not participate in the rounding.
 *
 * For even more speed, we avoid doing any multiplications in the inner loop
 * by precalculating the constants times Cb and Cr for all possible values.
 * For 8-bit JSAMPLEs this is very reasonable (only 256 entries per table);
 * for 12-bit samples it is still acceptable.  It's not very reasonable for
 * 16-bit samples, but if you want lossless storage you shouldn't be changing
 * colorspace anyway.
 * The Cr=>R and Cb=>B values can be rounded to integers in advance; the
 * values for the G calculation are left scaled up, since we must add them
 * together before rounding.
 */

pub const SCALEBITS: libc::c_int = 16 as libc::c_int;
/* speediest right-shift on some machines */

pub const ONE_HALF: crate::jpegint_h::JLONG =
    (1 as libc::c_int as crate::jpegint_h::JLONG) << SCALEBITS - 1 as libc::c_int;
/* We allocate one big table for RGB->Y conversion and divide it up into
 * three parts, instead of doing three alloc_small requests.  This lets us
 * use a single table base address, which can be held in a register in the
 * inner loops on many machines (more than can hold all three addresses,
 * anyway).
 */

pub const R_Y_OFF: libc::c_int = 0 as libc::c_int;
/* offset to R => Y section */

pub const G_Y_OFF: libc::c_int =
    1 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);
/* offset to G => Y section */

pub const B_Y_OFF: libc::c_int =
    2 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);
/* etc. */

pub const TABLE_SIZE: libc::c_int =
    3 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);
/* Include inline routines for colorspace extensions */

pub const RGB_RED_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_RED;

pub const RGB_GREEN_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_GREEN;

pub const RGB_BLUE_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_BLUE;

pub const RGB_PIXELSIZE_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_PIXELSIZE;

pub const RGB_RED_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_RED;

pub const RGB_GREEN_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_GREEN;

pub const RGB_BLUE_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_BLUE;

pub const RGB_ALPHA_2: libc::c_int = 3 as libc::c_int;

pub const RGB_PIXELSIZE_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;

pub const RGB_RED_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_RED;

pub const RGB_GREEN_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_GREEN;

pub const RGB_BLUE_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_BLUE;

pub const RGB_PIXELSIZE_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_PIXELSIZE;

pub const RGB_RED_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_RED;

pub const RGB_GREEN_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_GREEN;

pub const RGB_BLUE_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_BLUE;

pub const RGB_ALPHA_1: libc::c_int = 3 as libc::c_int;

pub const RGB_PIXELSIZE_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;

pub const RGB_RED_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_RED;

pub const RGB_GREEN_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_GREEN;

pub const RGB_BLUE_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_BLUE;

pub const RGB_ALPHA_0: libc::c_int = 0 as libc::c_int;

pub const RGB_PIXELSIZE_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;

pub const RGB_RED: libc::c_int = crate::jmorecfg_h::EXT_XRGB_RED;

pub const RGB_GREEN: libc::c_int = crate::jmorecfg_h::EXT_XRGB_GREEN;

pub const RGB_BLUE: libc::c_int = crate::jmorecfg_h::EXT_XRGB_BLUE;

pub const RGB_ALPHA: libc::c_int = 0 as libc::c_int;

pub const RGB_PIXELSIZE: libc::c_int = crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
/*
 * Initialize tables for YCC->RGB colorspace conversion.
 */

unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut i: libc::c_int = 0;
    let mut x: crate::jpegint_h::JLONG = 0;
    (*cconvert).Cr_r_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ((crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*cconvert).Cb_b_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ((crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*cconvert).Cr_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ((crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong),
    ) as *mut crate::jpegint_h::JLONG;
    (*cconvert).Cb_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ((crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong),
    ) as *mut crate::jpegint_h::JLONG;
    i = 0 as libc::c_int;
    x = -crate::jmorecfg_h::CENTERJSAMPLE as crate::jpegint_h::JLONG;
    while i <= crate::jmorecfg_h::MAXJSAMPLE {
        /* i is the actual input pixel value, in the range 0..MAXJSAMPLE */
        /* The Cb or Cr value we are thinking of is x = i - CENTERJSAMPLE */
        /* Cr=>R value is nearest int to 1.40200 * x */
        *(*cconvert).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * ((1 as libc::c_long) << 16 as libc::c_int) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * x
                + ((1 as libc::c_int as crate::jpegint_h::JLONG)
                    << 16 as libc::c_int - 1 as libc::c_int)
                >> 16 as libc::c_int) as libc::c_int;
        /* Cb=>B value is nearest int to 1.77200 * x */
        *(*cconvert).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * ((1 as libc::c_long) << 16 as libc::c_int) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * x
                + ((1 as libc::c_int as crate::jpegint_h::JLONG)
                    << 16 as libc::c_int - 1 as libc::c_int)
                >> 16 as libc::c_int) as libc::c_int;
        /* Cr=>G value is scaled-up -0.71414 * x */
        *(*cconvert).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        *(*cconvert).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * x
                + ONE_HALF;
        i += 1;
        x += 1
    }
}
/*
 * Convert some rows of samples to the output colorspace.
 */

unsafe extern "C" fn ycc_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            ycc_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            ycc_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            ycc_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            ycc_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            ycc_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            ycc_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            ycc_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/* *************** Cases other than YCbCr -> RGB **************/
/*
 * Initialize for RGB->grayscale colorspace conversion.
 */

unsafe extern "C" fn build_rgb_y_table(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_y_tab: *mut crate::jpegint_h::JLONG = 0 as *mut crate::jpegint_h::JLONG;
    let mut i: crate::jpegint_h::JLONG = 0;
    /* Allocate and fill in the conversion tables. */
    rgb_y_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (TABLE_SIZE as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong),
    ) as *mut crate::jpegint_h::JLONG;
    (*cconvert).rgb_y_tab = rgb_y_tab;
    i = 0 as libc::c_int as crate::jpegint_h::JLONG;
    while i <= crate::jmorecfg_h::MAXJSAMPLE as libc::c_long {
        *rgb_y_tab.offset((i + R_Y_OFF as libc::c_long) as isize) =
            (0.29900f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i;
        *rgb_y_tab.offset((i + G_Y_OFF as libc::c_long) as isize) =
            (0.58700f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i;
        *rgb_y_tab.offset((i + B_Y_OFF as libc::c_long) as isize) =
            (0.11400f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i
                + ONE_HALF;
        i += 1
    }
}
/*
 * Convert RGB to grayscale.
 */

unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_y_tab;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as libc::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh42 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh42;
        col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = *inptr0.offset(col as isize) as libc::c_int;
            g = *inptr1.offset(col as isize) as libc::c_int;
            b = *inptr2.offset(col as isize) as libc::c_int;
            /* Y */
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Color conversion for no colorspace change: just copy the data,
 * converting from separate-planes to interleaved representation.
 */

unsafe extern "C" fn null_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr3: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_components: libc::c_int = (*cinfo).num_components;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut ci: libc::c_int = 0;
    if num_components == 3 as libc::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh43 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh43;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let fresh44 = outptr;
                outptr = outptr.offset(1);
                *fresh44 = *inptr0.offset(col as isize);
                let fresh45 = outptr;
                outptr = outptr.offset(1);
                *fresh45 = *inptr1.offset(col as isize);
                let fresh46 = outptr;
                outptr = outptr.offset(1);
                *fresh46 = *inptr2.offset(col as isize);
                col = col.wrapping_add(1)
            }
        }
    } else if num_components == 4 as libc::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
            inptr3 = *(*input_buf.offset(3 as libc::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh47 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh47;
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let fresh48 = outptr;
                outptr = outptr.offset(1);
                *fresh48 = *inptr0.offset(col as isize);
                let fresh49 = outptr;
                outptr = outptr.offset(1);
                *fresh49 = *inptr1.offset(col as isize);
                let fresh50 = outptr;
                outptr = outptr.offset(1);
                *fresh50 = *inptr2.offset(col as isize);
                let fresh51 = outptr;
                outptr = outptr.offset(1);
                *fresh51 = *inptr3.offset(col as isize);
                col = col.wrapping_add(1)
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            ci = 0 as libc::c_int;
            while ci < num_components {
                inptr = *(*input_buf.offset(ci as isize)).offset(input_row as isize);
                outptr = *output_buf;
                col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
                while col < num_cols {
                    *outptr.offset(ci as isize) = *inptr.offset(col as isize);
                    outptr = outptr.offset(num_components as isize);
                    col = col.wrapping_add(1)
                }
                ci += 1
            }
            output_buf = output_buf.offset(1);
            input_row = input_row.wrapping_add(1)
        }
    };
}
/*
 * Color conversion for grayscale: just copy the data.
 * This also works for YCbCr -> grayscale conversion, in which
 * we just copy the Y (luminance) component and ignore chrominance.
 */

unsafe extern "C" fn grayscale_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    crate::src::jutils::jcopy_sample_rows(
        *input_buf.offset(0 as libc::c_int as isize),
        input_row as libc::c_int,
        output_buf,
        0 as libc::c_int,
        num_rows,
        (*cinfo).output_width,
    );
}
/*
 * Convert grayscale to RGB
 */

unsafe extern "C" fn gray_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            gray_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            gray_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            gray_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            gray_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            gray_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            gray_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            gray_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/*
 * Convert plain RGB to extended RGB
 */

unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            rgb_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            rgb_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            rgb_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            rgb_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            rgb_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            rgb_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            rgb_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/*
 * Adobe-style YCCK->CMYK conversion.
 * We convert YCbCr to R=1-C, G=1-M, and B=1-Y using the same
 * conversion as above, while passing K (black) unchanged.
 * We assume build_ycc_rgb_table has been called.
 */

unsafe extern "C" fn ycck_cmyk_convert(
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
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr3: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as libc::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(input_row as isize);
        inptr3 = *(*input_buf.offset(3 as libc::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh52 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh52;
        col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as libc::c_int;
            cb = *inptr1.offset(col as isize) as libc::c_int;
            cr = *inptr2.offset(col as isize) as libc::c_int;
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            *outptr.offset(0 as libc::c_int as isize) = *range_limit.offset(
                (crate::jmorecfg_h::MAXJSAMPLE - (y + *Crrtab.offset(cr as isize))) as isize,
            ); /* red */
            *outptr.offset(1 as libc::c_int as isize) = *range_limit.offset(
                (crate::jmorecfg_h::MAXJSAMPLE
                    - (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as libc::c_int) as libc::c_int)) as isize,
            ); /* blue */
            *outptr.offset(2 as libc::c_int as isize) = *range_limit.offset(
                (crate::jmorecfg_h::MAXJSAMPLE - (y + *Cbbtab.offset(cb as isize))) as isize,
            );
            /* K passes through unchanged */
            *outptr.offset(3 as libc::c_int as isize) = *inptr3.offset(col as isize); /* don't need GETJSAMPLE here */
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * RGB565 conversion
 */
/* Declarations for ordered dithering
 *
 * We use a 4x4 ordered dither array packed into 32 bits.  This array is
 * sufficent for dithering RGB888 to RGB565.
 */

pub const DITHER_MASK: libc::c_int = 0x3 as libc::c_int;

static mut dither_matrix: [crate::jpegint_h::JLONG; 4] = [
    0x8020a as libc::c_int as crate::jpegint_h::JLONG,
    0xc040e06 as libc::c_int as crate::jpegint_h::JLONG,
    0x30b0109 as libc::c_int as crate::jpegint_h::JLONG,
    0xf070d05 as libc::c_int as crate::jpegint_h::JLONG,
];
#[inline(always)]

unsafe extern "C" fn is_big_endian() -> crate::jmorecfg_h::boolean {
    let mut test_value: libc::c_int = 1 as libc::c_int;
    if *(&mut test_value as *mut libc::c_int as *mut libc::c_char) as libc::c_int
        != 1 as libc::c_int
    {
        return crate::jmorecfg_h::TRUE;
    }
    return crate::jmorecfg_h::FALSE;
}
/* Include inline routines for RGB565 conversion */

unsafe extern "C" fn ycc_rgb565_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        ycc_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}

unsafe extern "C" fn ycc_rgb565D_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        ycc_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}

unsafe extern "C" fn rgb_rgb565_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        rgb_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}

unsafe extern "C" fn rgb_rgb565D_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        rgb_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}

unsafe extern "C" fn gray_rgb565_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        gray_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}

unsafe extern "C" fn gray_rgb565D_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    if is_big_endian() != 0 {
        gray_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
/*
 * Empty method for start_pass.
 */

unsafe extern "C" fn start_pass_dcolor(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    /* no work needed */
}
/*
 * jpegint.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 1997-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015-2016, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file provides common declarations for the various JPEG modules.
 * These declarations are considered internal to the JPEG library; most
 * applications using the library shouldn't need to include this file.
 */
/* Declarations for both compression & decompression */
/* Operating modes for buffer controllers */
/* Plain stripwise operation */
/* Remaining modes require a full-image buffer to have been created */
/* Run source subobject only, save output */
/* Run dest subobject only, using saved data */
/* Run both subobjects, save output */
/* Requantize */
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */
/* after create_compress */
/* start_compress done, write_scanlines OK */
/* start_compress done, write_raw_data OK */
/* jpeg_write_coefficients done */
/* after create_decompress */
/* reading header markers, no SOS yet */
/* found SOS, ready for start_decompress */
/* reading multiscan file in start_decompress*/
/* performing dummy pass for 2-pass quant */
/* start_decompress done, read_scanlines OK */
/* start_decompress done, read_raw_data OK */
/* expecting jpeg_start_output */
/* looking for SOS/EOI in jpeg_finish_output */
/* reading file in jpeg_read_coefficients */
/* looking for EOI in jpeg_finish_decompress */
/* JLONG must hold at least signed 32-bit values. */
/*
 * Left shift macro that handles a negative operand without causing any
 * sanitizer warnings
 */
/* Declarations for compression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True if pass_startup must be called */
/* True during last pass */
/* Extension parameters */
/* TRUE=optimize progressive coding scans */
/* TRUE=use trellis quantization */
/* TRUE=use trellis quant for DC coefficient */
/* TRUE=optimize for sequences of EOB */
/* TRUE=use lambda weighting table */
/* TRUE=use scans in trellis optimization */
/* TRUE=currently doing trellis-related passes [not exposed] */
/* TRUE=optimize quant table in trellis loop */
/* TRUE=preprocess input to reduce ringing of edges on white background */
/* compression profile */
/* DC scan optimization mode */
/* Quantization table master index */
/* splitting point for frequency in trellis quantization */
/* number of trellis loops */
/* # of entries in scan_info array pertaining to luma (used when optimize_scans is TRUE */
/* maximum value of Al tested when optimizing scans (luma) */
/* maximum value of Al tested when optimizing scans (chroma) */
/* Main buffer control (downsampled-data buffer) */
/* Compression preprocessing (downsampling input buffer control) */
/* Coefficient buffer control */
/* Colorspace conversion */
/* Downsampling */
/* TRUE if need rows above & below */
/* Forward DCT (also controls coefficient quantization) */
/* perhaps this should be an array??? */
/* Entropy encoding */
/* Marker writing */
/* These routines are exported to allow insertion of extra markers */
/* Probably only COM and APPn markers should be written this way */
/* Declarations for decompression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True during 1st pass for 2-pass quant */
/* Partial decompression variables */
/* Input control module */
/* State variables made visible to other modules */
/* True if file has multiple scans */
/* True when EOI has been consumed */
/* Main buffer control (downsampled-data buffer) */
/* Coefficient buffer control */
/* Pointer to array of coefficient virtual arrays, or NULL if none */
/* Decompression postprocessing (color quantization buffer control) */
/* Marker reading & parsing */
/* Read markers until SOS or EOI.
 * Returns same codes as are defined for jpeg_consume_input:
 * JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 */
/* Read a restart marker --- exported for use by entropy decoder only */
/* State of marker reader --- nominally internal, but applications
 * supplying COM or APPn handlers might like to know the state.
 */
/* found SOI? */
/* found SOF? */
/* next restart number expected (0-7) */
/* # of bytes skipped looking for a marker */
/* Entropy decoding */
/* This is here to share code between baseline and progressive decoders; */
/* other modules probably should not use it */
/* set TRUE after emitting warning */
/* Inverse DCT (also performs dequantization) */
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Compression module initialization routines */
/* Decompression module initialization routines */
/*
 * Module initialization routine for output colorspace conversion.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_color_deconverter(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = 0 as *mut my_color_deconverter;
    let mut ci: libc::c_int = 0;
    cconvert = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_color_deconverter>() as libc::c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut crate::jpegint_h::jpeg_color_deconverter;
    (*cconvert).pub_0.start_pass = Some(
        start_pass_dcolor as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    /* Make sure num_components agrees with jpeg_color_space */
    match (*cinfo).jpeg_color_space as libc::c_uint {
        1 => {
            if (*cinfo).num_components != 1 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        2 | 3 => {
            if (*cinfo).num_components != 3 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        4 | 5 => {
            if (*cinfo).num_components != 4 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        _ => {
            /* JCS_UNKNOWN can be anything */
            if (*cinfo).num_components < 1 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
    }
    /* Set out_color_components and conversion method based on requested space.
     * Also clear the component_needed flags for any unused components,
     * so that earlier pipeline stages can avoid useless computation.
     */
    match (*cinfo).out_color_space as libc::c_uint {
        1 => {
            (*cinfo).out_color_components = 1 as libc::c_int;
            if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
                || (*cinfo).jpeg_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                /* For color->grayscale conversion, only the Y (0) component is needed */
                ci = 1 as libc::c_int;
                while ci < (*cinfo).num_components {
                    (*(*cinfo).comp_info.offset(ci as isize)).component_needed =
                        crate::jmorecfg_h::FALSE;
                    ci += 1
                }
            } else if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_gray_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                build_rgb_y_table(cinfo);
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            (*cinfo).out_color_components = rgb_pixelsize[(*cinfo).out_color_space as usize];
            if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
            {
                if crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    );
                    build_ycc_rgb_table(cinfo);
                }
            } else if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
            {
                if rgb_red[(*cinfo).out_color_space as usize] == 0 as libc::c_int
                    && rgb_green[(*cinfo).out_color_space as usize] == 1 as libc::c_int
                    && rgb_blue[(*cinfo).out_color_space as usize] == 2 as libc::c_int
                    && rgb_pixelsize[(*cinfo).out_color_space as usize] == 3 as libc::c_int
                {
                    (*cconvert).pub_0.color_convert = Some(
                        null_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                }
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        16 => {
            (*cinfo).out_color_components = 3 as libc::c_int;
            if (*cinfo).dither_mode as libc::c_uint
                == crate::jpeglib_h::JDITHER_NONE as libc::c_int as libc::c_uint
            {
                if (*cinfo).jpeg_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
                {
                    if crate::src::simd::x86_64::jsimd::jsimd_can_ycc_rgb565() != 0 {
                        (*cconvert).pub_0.color_convert = Some(
                            crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: crate::jpeglib_h::j_decompress_ptr,
                                    _: crate::jpeglib_h::JSAMPIMAGE,
                                    _: crate::jmorecfg_h::JDIMENSION,
                                    _: crate::jpeglib_h::JSAMPARRAY,
                                    _: libc::c_int,
                                ) -> (),
                        )
                    } else {
                        (*cconvert).pub_0.color_convert = Some(
                            ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: crate::jpeglib_h::j_decompress_ptr,
                                    _: crate::jpeglib_h::JSAMPIMAGE,
                                    _: crate::jmorecfg_h::JDIMENSION,
                                    _: crate::jpeglib_h::JSAMPARRAY,
                                    _: libc::c_int,
                                ) -> (),
                        );
                        build_ycc_rgb_table(cinfo);
                    }
                } else if (*cinfo).jpeg_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
                {
                    (*cconvert).pub_0.color_convert = Some(
                        gray_rgb565_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else if (*cinfo).jpeg_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb565_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*(*cinfo).err).msg_code =
                        crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
            } else if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    ycc_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        4 => {
            (*cinfo).out_color_components = 4 as libc::c_int;
            if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_YCCK as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    ycck_cmyk_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        _ => {
            /* only ordered dithering is supported */
            /* Permit null conversion to same output space */
            if (*cinfo).out_color_space as libc::c_uint == (*cinfo).jpeg_color_space as libc::c_uint
            {
                (*cinfo).out_color_components = (*cinfo).num_components; /* unsupported non-null conversion */
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int; /* single colormapped output component */
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
    }
    if (*cinfo).quantize_colors != 0 {
        (*cinfo).output_components = 1 as libc::c_int
    } else {
        (*cinfo).output_components = (*cinfo).out_color_components
    };
}
