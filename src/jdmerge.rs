use ::libc;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jdmrgext.c:85"]
pub mod jdmrgext_c {
    /*
     * jdmrgext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1994-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2011, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains code for merged upsampling/color conversion.
     */
    /*
     * jdmrgext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1994-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2011, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains code for merged upsampling/color conversion.
     */
    /*
     * jdmrgext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1994-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2011, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains code for merged upsampling/color conversion.
     */
    /*
     * jdmrgext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1994-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2011, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains code for merged upsampling/color conversion.
     */
    /*
     * jdmrgext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1994-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2011, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains code for merged upsampling/color conversion.
     */
    /*
     * jdmrgext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1994-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2011, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains code for merged upsampling/color conversion.
     */
    /*
     * jdmrgext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1994-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2011, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains code for merged upsampling/color conversion.
     */
    /* This file is included by jdmerge.c */
    /* This file is included by jdmerge.c */
    /* This file is included by jdmerge.c */
    /* This file is included by jdmerge.c */
    /* This file is included by jdmerge.c */
    /* This file is included by jdmerge.c */
    /* This file is included by jdmerge.c */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
     */
    #[inline(always)]

    pub unsafe extern "C" fn extxbgr_h2v1_merged_upsample_internal(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            let fresh0 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh0 as libc::c_int;
            let fresh1 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh1 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            let fresh2 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh2 as libc::c_int;
            *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            let fresh3 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh3 as libc::c_int;
            *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_sub(1)
        }
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_0 as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn h2v1_merged_upsample_internal(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh4 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh4 as libc::c_int;
            let fresh5 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh5 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh6 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh6 as libc::c_int;
            *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize);
            outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            let fresh7 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh7 as libc::c_int;
            *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize);
            outptr = outptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            *outptr.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize)
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgb_h2v1_merged_upsample_internal(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh8 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh8 as libc::c_int;
            let fresh9 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh9 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh10 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh10 as libc::c_int;
            *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            let fresh11 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh11 as libc::c_int;
            *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize)
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgbx_h2v1_merged_upsample_internal(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh12 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh12 as libc::c_int;
            let fresh13 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh13 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh14 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh14 as libc::c_int;
            *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            let fresh15 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh15 as libc::c_int;
            *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgr_h2v1_merged_upsample_internal(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh16 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh16 as libc::c_int;
            let fresh17 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh17 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh18 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh18 as libc::c_int;
            *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            let fresh19 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh19 as libc::c_int;
            *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize)
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgrx_h2v1_merged_upsample_internal(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh20 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh20 as libc::c_int;
            let fresh21 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh21 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh22 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh22 as libc::c_int;
            *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            let fresh23 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh23 as libc::c_int;
            *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxrgb_h2v1_merged_upsample_internal(
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh24 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh24 as libc::c_int;
            let fresh25 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh25 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh26 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh26 as libc::c_int;
            *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            let fresh27 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh27 as libc::c_int;
            *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
     */
    /*
     * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
     */
    #[inline(always)]

    pub unsafe extern "C" fn extbgr_h2v2_merged_upsample_internal(
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        /* copy these pointers into registers if possible */
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            let fresh28 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh28 as libc::c_int;
            let fresh29 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh29 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            let fresh30 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh30 as libc::c_int;
            *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr0 = outptr0.offset(RGB_PIXELSIZE_3 as isize);
            let fresh31 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh31 as libc::c_int;
            *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr0 = outptr0.offset(RGB_PIXELSIZE_3 as isize);
            let fresh32 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh32 as libc::c_int;
            *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr1 = outptr1.offset(RGB_PIXELSIZE_3 as isize);
            let fresh33 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh33 as libc::c_int;
            *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr1 = outptr1.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_sub(1)
        }
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
            y = *inptr01 as libc::c_int;
            *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize)
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgbx_h2v2_merged_upsample_internal(
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh34 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh34 as libc::c_int;
            let fresh35 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh35 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh36 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh36 as libc::c_int;
            *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE_2 as isize);
            let fresh37 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh37 as libc::c_int;
            *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE_2 as isize);
            let fresh38 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh38 as libc::c_int;
            *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE_2 as isize);
            let fresh39 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh39 as libc::c_int;
            *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            y = *inptr01 as libc::c_int;
            *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_2 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgb_h2v2_merged_upsample_internal(
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh40 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh40 as libc::c_int;
            let fresh41 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh41 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh42 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh42 as libc::c_int;
            *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr0 = outptr0.offset(RGB_PIXELSIZE_4 as isize);
            let fresh43 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh43 as libc::c_int;
            *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr0 = outptr0.offset(RGB_PIXELSIZE_4 as isize);
            let fresh44 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh44 as libc::c_int;
            *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr1 = outptr1.offset(RGB_PIXELSIZE_4 as isize);
            let fresh45 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh45 as libc::c_int;
            *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
            outptr1 = outptr1.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
            y = *inptr01 as libc::c_int;
            *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize)
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgrx_h2v2_merged_upsample_internal(
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh46 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh46 as libc::c_int;
            let fresh47 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh47 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh48 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh48 as libc::c_int;
            *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE_1 as isize);
            let fresh49 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh49 as libc::c_int;
            *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE_1 as isize);
            let fresh50 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh50 as libc::c_int;
            *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE_1 as isize);
            let fresh51 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh51 as libc::c_int;
            *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            y = *inptr01 as libc::c_int;
            *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_1 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn h2v2_merged_upsample_internal(
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh52 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh52 as libc::c_int;
            let fresh53 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh53 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh54 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh54 as libc::c_int;
            *outptr0.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr0.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize);
            outptr0 = outptr0.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            let fresh55 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh55 as libc::c_int;
            *outptr0.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr0.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize);
            outptr0 = outptr0.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            let fresh56 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh56 as libc::c_int;
            *outptr1.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr1.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize);
            outptr1 = outptr1.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            let fresh57 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh57 as libc::c_int;
            *outptr1.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr1.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize);
            outptr1 = outptr1.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            *outptr0.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr0.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize);
            y = *inptr01 as libc::c_int;
            *outptr1.offset(crate::jmorecfg_h::RGB_RED_5 as isize) =
                *range_limit.offset((y + cred) as isize);
            *outptr1.offset(crate::jmorecfg_h::RGB_GREEN_5 as isize) =
                *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(crate::jmorecfg_h::RGB_BLUE_5 as isize) =
                *range_limit.offset((y + cblue) as isize)
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxrgb_h2v2_merged_upsample_internal(
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh58 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh58 as libc::c_int;
            let fresh59 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh59 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh60 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh60 as libc::c_int;
            *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE as isize);
            let fresh61 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh61 as libc::c_int;
            *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE as isize);
            let fresh62 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh62 as libc::c_int;
            *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE as isize);
            let fresh63 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh63 as libc::c_int;
            *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            y = *inptr01 as libc::c_int;
            *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxbgr_h2v2_merged_upsample_internal(
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh64 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh64 as libc::c_int;
            let fresh65 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh65 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh66 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh66 as libc::c_int;
            *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE_0 as isize);
            let fresh67 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh67 as libc::c_int;
            *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr0 = outptr0.offset(RGB_PIXELSIZE_0 as isize);
            let fresh68 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh68 as libc::c_int;
            *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE_0 as isize);
            let fresh69 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh69 as libc::c_int;
            *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            outptr1 = outptr1.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr0.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
            y = *inptr01 as libc::c_int;
            *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
            *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
            *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
            *outptr1.offset(RGB_ALPHA_0 as isize) =
                0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE
        };
    }

    use crate::jmorecfg_h::JDIMENSION;
    use crate::jmorecfg_h::JSAMPLE;
    use crate::jmorecfg_h::RGB_BLUE_5;
    use crate::jmorecfg_h::RGB_GREEN_5;
    use crate::jmorecfg_h::RGB_PIXELSIZE_5;
    use crate::jmorecfg_h::RGB_RED_5;
    use crate::jpegint_h::JLONG;
    use crate::jpeglib_h::JSAMPROW;
    use crate::src::jdmerge::my_upsample_ptr;
    use crate::src::jdmerge::RGB_ALPHA;
    use crate::src::jdmerge::RGB_ALPHA_0;
    use crate::src::jdmerge::RGB_ALPHA_1;
    use crate::src::jdmerge::RGB_ALPHA_2;
    use crate::src::jdmerge::RGB_BLUE;
    use crate::src::jdmerge::RGB_BLUE_0;
    use crate::src::jdmerge::RGB_BLUE_1;
    use crate::src::jdmerge::RGB_BLUE_2;
    use crate::src::jdmerge::RGB_BLUE_3;
    use crate::src::jdmerge::RGB_BLUE_4;
    use crate::src::jdmerge::RGB_GREEN;
    use crate::src::jdmerge::RGB_GREEN_0;
    use crate::src::jdmerge::RGB_GREEN_1;
    use crate::src::jdmerge::RGB_GREEN_2;
    use crate::src::jdmerge::RGB_GREEN_3;
    use crate::src::jdmerge::RGB_GREEN_4;
    use crate::src::jdmerge::RGB_PIXELSIZE;
    use crate::src::jdmerge::RGB_PIXELSIZE_0;
    use crate::src::jdmerge::RGB_PIXELSIZE_1;
    use crate::src::jdmerge::RGB_PIXELSIZE_2;
    use crate::src::jdmerge::RGB_PIXELSIZE_3;
    use crate::src::jdmerge::RGB_PIXELSIZE_4;
    use crate::src::jdmerge::RGB_RED;
    use crate::src::jdmerge::RGB_RED_0;
    use crate::src::jdmerge::RGB_RED_1;
    use crate::src::jdmerge::RGB_RED_2;
    use crate::src::jdmerge::RGB_RED_3;
    use crate::src::jdmerge::RGB_RED_4;
}
#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jdmrg565.c:473"]
pub mod jdmrg565_c {
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            let fresh70 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh70 as libc::c_int;
            let fresh71 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh71 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            let fresh72 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh72 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            let fresh73 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh73 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = rgb << 16 as libc::c_int
                | (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as libc::c_long;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh74 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh74 as libc::c_int;
            let fresh75 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh75 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh76 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh76 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            let fresh77 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh77 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int)
                << 16 as libc::c_int) as libc::c_long
                | rgb;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        /* Loop for each pair of output pixels */
        /* Loop for each pair of output pixels */
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            let fresh78 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh78 as libc::c_int;
            let fresh79 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh79 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            /* Fetch 2 Y values and emit 2 pixels */
            /* Fetch 2 Y values and emit 2 pixels */
            let fresh80 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh80 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            let fresh81 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh81 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int)
                << 16 as libc::c_int) as libc::c_long
                | rgb;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
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
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
        inptr0 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr = *output_buf.offset(0 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh82 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh82 as libc::c_int;
            let fresh83 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh83 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh84 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh84 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            let fresh85 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh85 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = rgb << 16 as libc::c_int
                | (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as libc::c_long;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr0 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            let fresh86 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh86 as libc::c_int;
            let fresh87 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh87 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            let fresh88 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh88 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            let fresh89 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh89 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int)
                << 16 as libc::c_int) as libc::c_long
                | rgb;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr0 = outptr0.offset(4 as libc::c_int as isize);
            let fresh90 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh90 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            let fresh91 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh91 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int)
                << 16 as libc::c_int) as libc::c_long
                | rgb;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr1 = outptr1.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            y = *inptr01 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh92 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh92 as libc::c_int;
            let fresh93 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh93 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh94 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh94 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            let fresh95 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh95 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = rgb << 16 as libc::c_int
                | (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as libc::c_long;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr0 = outptr0.offset(4 as libc::c_int as isize);
            let fresh96 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh96 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            let fresh97 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh97 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = rgb << 16 as libc::c_int
                | (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as libc::c_long;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr1 = outptr1.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            y = *inptr01 as libc::c_int;
            r = *range_limit.offset((y + cred) as isize) as libc::c_uint;
            g = *range_limit.offset((y + cgreen) as isize) as libc::c_uint;
            b = *range_limit.offset((y + cblue) as isize) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
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
        let mut d1: crate::jpegint_h::JLONG = dither_matrix[((*cinfo)
            .output_scanline
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            & DITHER_MASK as libc::c_uint)
            as usize];
        let mut r: libc::c_uint = 0;
        let mut g: libc::c_uint = 0;
        let mut b: libc::c_uint = 0;
        let mut rgb: crate::jpegint_h::JLONG = 0;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        /* Loop for each group of output pixels */
        /* Loop for each group of output pixels */
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            /* Do the chroma part of the calculation */
            /* Do the chroma part of the calculation */
            let fresh98 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh98 as libc::c_int;
            let fresh99 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh99 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            /* Fetch 4 Y values and emit 4 pixels */
            /* Fetch 4 Y values and emit 4 pixels */
            let fresh100 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh100 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            let fresh101 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh101 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int)
                << 16 as libc::c_int) as libc::c_long
                | rgb;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr0 = outptr0.offset(4 as libc::c_int as isize);
            let fresh102 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh102 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d1 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d1 = (d1 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d1 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            let fresh103 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh103 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d1 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d1 = (d1 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d1 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = ((r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int)
                << 16 as libc::c_int) as libc::c_long
                | rgb;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr1 = outptr1.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        /* If image width is odd, do the last output column separately */
        /* If image width is odd, do the last output column separately */
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
            *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            y = *inptr01 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d1 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            rgb = (r << 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint
                | g << 3 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
                | b >> 3 as libc::c_int) as crate::jpegint_h::JLONG;
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
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
        let mut Crrtab: *mut libc::c_int = (*upsample).Cr_r_tab;
        let mut Cbbtab: *mut libc::c_int = (*upsample).Cb_b_tab;
        let mut Crgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cr_g_tab;
        let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*upsample).Cb_g_tab;
        let mut d0: crate::jpegint_h::JLONG =
            dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
        let mut d1: crate::jpegint_h::JLONG = dither_matrix[((*cinfo)
            .output_scanline
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            & DITHER_MASK as libc::c_uint)
            as usize];
        let mut r: libc::c_uint = 0;
        let mut g: libc::c_uint = 0;
        let mut b: libc::c_uint = 0;
        let mut rgb: crate::jpegint_h::JLONG = 0;
        inptr00 = *(*input_buf.offset(0 as libc::c_int as isize))
            .offset(in_row_group_ctr.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
        inptr01 = *(*input_buf.offset(0 as libc::c_int as isize)).offset(
            in_row_group_ctr
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
        inptr1 = *(*input_buf.offset(1 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        inptr2 = *(*input_buf.offset(2 as libc::c_int as isize)).offset(in_row_group_ctr as isize);
        outptr0 = *output_buf.offset(0 as libc::c_int as isize);
        outptr1 = *output_buf.offset(1 as libc::c_int as isize);
        col = (*cinfo).output_width >> 1 as libc::c_int;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh104 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh104 as libc::c_int;
            let fresh105 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh105 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            let fresh106 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh106 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            let fresh107 = inptr00;
            inptr00 = inptr00.offset(1);
            y = *fresh107 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d0 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = rgb << 16 as libc::c_int
                | (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as libc::c_long;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr0 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr0 = outptr0.offset(4 as libc::c_int as isize);
            let fresh108 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh108 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d1 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d1 = (d1 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d1 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            let fresh109 = inptr01;
            inptr01 = inptr01.offset(1);
            y = *fresh109 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d1 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            d1 = (d1 & 0xff as libc::c_int as libc::c_long) << 24 as libc::c_int
                | d1 >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_long;
            rgb = rgb << 16 as libc::c_int
                | (r & 0xf8 as libc::c_int as libc::c_uint
                    | g >> 5 as libc::c_int
                    | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                    | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                    as libc::c_long;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(1 as libc::c_int as isize) =
                rgb as crate::jmorecfg_h::INT16;
            *(outptr1 as *mut crate::jmorecfg_h::INT16).offset(0 as libc::c_int as isize) =
                (rgb >> 16 as libc::c_int) as crate::jmorecfg_h::INT16;
            outptr1 = outptr1.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
        if (*cinfo).output_width & 1 as libc::c_int as libc::c_uint != 0 {
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            cred = *Crrtab.offset(cr as isize);
            cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                >> 16 as libc::c_int) as libc::c_int;
            cblue = *Cbbtab.offset(cb as isize);
            y = *inptr00 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d0 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d0 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            *(outptr0 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            y = *inptr01 as libc::c_int;
            r = *range_limit.offset(
                ((y + cred) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + cgreen) as libc::c_long
                    + ((d1 & 0xff as libc::c_int as libc::c_long) >> 1 as libc::c_int))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + cblue) as libc::c_long + (d1 & 0xff as libc::c_int as libc::c_long)) as isize,
            ) as libc::c_uint;
            rgb = (r & 0xf8 as libc::c_int as libc::c_uint
                | g >> 5 as libc::c_int
                | g << 11 as libc::c_int & 0xe000 as libc::c_int as libc::c_uint
                | b << 5 as libc::c_int & 0x1f00 as libc::c_int as libc::c_uint)
                as crate::jpegint_h::JLONG;
            *(outptr1 as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
        };
    }

    use crate::jmorecfg_h::INT16;
    use crate::jmorecfg_h::JDIMENSION;
    use crate::jmorecfg_h::JSAMPLE;
    use crate::jpegint_h::JLONG;
    use crate::jpeglib_h::JSAMPROW;
    use crate::src::jdmerge::dither_matrix;
    use crate::src::jdmerge::my_upsample_ptr;
    use crate::src::jdmerge::DITHER_MASK;
}

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
pub use crate::src::jdmerge::jdmrg565_c::h2v1_merged_upsample_565D_be;
pub use crate::src::jdmerge::jdmrg565_c::h2v1_merged_upsample_565D_le;
pub use crate::src::jdmerge::jdmrg565_c::h2v1_merged_upsample_565_be;
pub use crate::src::jdmerge::jdmrg565_c::h2v1_merged_upsample_565_le;
pub use crate::src::jdmerge::jdmrg565_c::h2v2_merged_upsample_565D_be;
pub use crate::src::jdmerge::jdmrg565_c::h2v2_merged_upsample_565D_le;
pub use crate::src::jdmerge::jdmrg565_c::h2v2_merged_upsample_565_be;
pub use crate::src::jdmerge::jdmrg565_c::h2v2_merged_upsample_565_le;
pub use crate::src::jdmerge::jdmrgext_c::extbgr_h2v1_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extbgr_h2v2_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extbgrx_h2v1_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extbgrx_h2v2_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extrgb_h2v1_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extrgb_h2v2_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extrgbx_h2v1_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extrgbx_h2v2_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extxbgr_h2v1_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extxbgr_h2v2_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extxrgb_h2v1_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::extxrgb_h2v2_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::h2v1_merged_upsample_internal;
pub use crate::src::jdmerge::jdmrgext_c::h2v2_merged_upsample_internal;
pub use crate::src::jutils::jcopy_sample_rows;
use crate::src::simd::x86_64::jsimd::jsimd_can_h2v1_merged_upsample;
use crate::src::simd::x86_64::jsimd::jsimd_can_h2v2_merged_upsample;
use crate::src::simd::x86_64::jsimd::jsimd_h2v1_merged_upsample;
use crate::src::simd::x86_64::jsimd::jsimd_h2v2_merged_upsample;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;

pub type my_upsample_ptr = *mut my_upsampler;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_upsampler {
    pub pub_0: crate::jpegint_h::jpeg_upsampler,
    pub upmethod: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
        ) -> (),
    >,
    pub Cr_r_tab: *mut libc::c_int,
    pub Cb_b_tab: *mut libc::c_int,
    pub Cr_g_tab: *mut crate::jpegint_h::JLONG,
    pub Cb_g_tab: *mut crate::jpegint_h::JLONG,
    pub spare_row: crate::jpeglib_h::JSAMPROW,
    pub spare_full: crate::jmorecfg_h::boolean,
    pub out_row_width: crate::jmorecfg_h::JDIMENSION,
    pub rows_to_go: crate::jmorecfg_h::JDIMENSION,
}

pub const SCALEBITS: libc::c_int = 16 as libc::c_int;
/* speediest right-shift on some machines */

pub const ONE_HALF: crate::jpegint_h::JLONG =
    (1 as libc::c_int as crate::jpegint_h::JLONG) << SCALEBITS - 1 as libc::c_int;
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
 * This is taken directly from jdcolor.c; see that file for more info.
 */

unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut i: libc::c_int = 0;
    let mut x: crate::jpegint_h::JLONG = 0;
    (*upsample).Cr_r_tab = Some(
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
    (*upsample).Cb_b_tab = Some(
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
    (*upsample).Cr_g_tab = Some(
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
    (*upsample).Cb_g_tab = Some(
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
        *(*upsample).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * ((1 as libc::c_long) << 16 as libc::c_int) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * x
                + ((1 as libc::c_int as crate::jpegint_h::JLONG)
                    << 16 as libc::c_int - 1 as libc::c_int)
                >> 16 as libc::c_int) as libc::c_int;
        /* Cb=>B value is nearest int to 1.77200 * x */
        *(*upsample).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * ((1 as libc::c_long) << 16 as libc::c_int) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * x
                + ((1 as libc::c_int as crate::jpegint_h::JLONG)
                    << 16 as libc::c_int - 1 as libc::c_int)
                >> 16 as libc::c_int) as libc::c_int;
        /* Cr=>G value is scaled-up -0.71414 * x */
        *(*upsample).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        *(*upsample).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * x
                + ONE_HALF;
        i += 1;
        x += 1
    }
}
/*
 * Initialize for an upsampling pass.
 */

unsafe extern "C" fn start_pass_merged_upsample(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    /* Mark the spare buffer empty */
    (*upsample).spare_full = crate::jmorecfg_h::FALSE;
    /* Initialize total-height counter for detecting bottom of image */
    (*upsample).rows_to_go = (*cinfo).output_height;
}
/*
 * Control routine to do upsampling (and color conversion).
 *
 * The control routine just handles the row buffering considerations.
 */

unsafe extern "C" fn merged_2v_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut in_row_groups_avail: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
)
/* 2:1 vertical sampling case: may need a spare row. */
{
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr; /* number of rows returned to caller */
    let mut work_ptrs: [crate::jpeglib_h::JSAMPROW; 2] = [0 as *mut crate::jmorecfg_h::JSAMPLE; 2];
    let mut num_rows: crate::jmorecfg_h::JDIMENSION = 0;
    if (*upsample).spare_full != 0 {
        /* If we have a spare row saved from a previous cycle, just return it. */
        let mut size: crate::jmorecfg_h::JDIMENSION = (*upsample).out_row_width;
        if (*cinfo).out_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_RGB565 as libc::c_int as libc::c_uint
        {
            size = (*cinfo)
                .output_width
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
        }
        crate::src::jutils::jcopy_sample_rows(
            &mut (*upsample).spare_row,
            0 as libc::c_int,
            output_buf.offset(*out_row_ctr as isize),
            0 as libc::c_int,
            1 as libc::c_int,
            size,
        );
        num_rows = 1 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
        (*upsample).spare_full = crate::jmorecfg_h::FALSE
    } else {
        /* Figure number of rows to return to caller. */
        num_rows = 2 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
        /* Not more than the distance to the end of the image. */
        if num_rows > (*upsample).rows_to_go {
            num_rows = (*upsample).rows_to_go
        }
        /* And not more than what the client can accept: */
        out_rows_avail = (out_rows_avail as libc::c_uint).wrapping_sub(*out_row_ctr)
            as crate::jmorecfg_h::JDIMENSION
            as crate::jmorecfg_h::JDIMENSION;
        if num_rows > out_rows_avail {
            num_rows = out_rows_avail
        }
        /* Create output pointer array for upsampler. */
        work_ptrs[0 as libc::c_int as usize] = *output_buf.offset(*out_row_ctr as isize);
        if num_rows > 1 as libc::c_int as libc::c_uint {
            work_ptrs[1 as libc::c_int as usize] = *output_buf
                .offset((*out_row_ctr).wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
        } else {
            work_ptrs[1 as libc::c_int as usize] = (*upsample).spare_row;
            (*upsample).spare_full = crate::jmorecfg_h::TRUE
        }
        /* Now do the upsampling. */
        Some((*upsample).upmethod.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            cinfo,
            input_buf,
            *in_row_group_ctr,
            work_ptrs.as_mut_ptr(),
        );
    }
    /* Adjust counts */
    *out_row_ctr = (*out_row_ctr as libc::c_uint).wrapping_add(num_rows)
        as crate::jmorecfg_h::JDIMENSION as crate::jmorecfg_h::JDIMENSION;
    (*upsample).rows_to_go = ((*upsample).rows_to_go as libc::c_uint).wrapping_sub(num_rows)
        as crate::jmorecfg_h::JDIMENSION
        as crate::jmorecfg_h::JDIMENSION;
    /* When the buffer is emptied, declare this input row group consumed */
    if (*upsample).spare_full == 0 {
        *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1)
    };
}

unsafe extern "C" fn merged_1v_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut in_row_groups_avail: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
)
/* 1:1 vertical sampling case: much easier, never need a spare row. */
{
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    /* Just do the upsampling. */
    Some((*upsample).upmethod.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        cinfo,
        input_buf,
        *in_row_group_ctr,
        output_buf.offset(*out_row_ctr as isize),
    );
    /* Adjust counts */
    *out_row_ctr = (*out_row_ctr).wrapping_add(1);
    *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1);
}
/*
 * These are the routines invoked by the control routines to do
 * the actual upsampling/conversion.  One row group is processed per call.
 *
 * Note: since we may be writing directly into application-supplied buffers,
 * we have to be honest about the output width; we can't assume the buffer
 * has been rounded up to an even width.
 */
/*
 * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
 */

unsafe extern "C" fn h2v1_merged_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            extrgb_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        7 | 12 => {
            extrgbx_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        8 => {
            extbgr_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        9 | 13 => {
            extbgrx_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        10 | 14 => {
            extxbgr_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        11 | 15 => {
            extxrgb_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        _ => {
            h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
    };
}
/*
 * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
 */

unsafe extern "C" fn h2v2_merged_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            extrgb_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        7 | 12 => {
            extrgbx_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        8 => {
            extbgr_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        9 | 13 => {
            extbgrx_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        10 | 14 => {
            extxbgr_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        11 | 15 => {
            extxrgb_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        _ => {
            h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
    };
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
/* Include inline routines for RGB565 conversion */
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

unsafe extern "C" fn h2v1_merged_upsample_565(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v1_merged_upsample_565_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v1_merged_upsample_565_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}

unsafe extern "C" fn h2v1_merged_upsample_565D(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v1_merged_upsample_565D_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v1_merged_upsample_565D_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}

unsafe extern "C" fn h2v2_merged_upsample_565(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v2_merged_upsample_565_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v2_merged_upsample_565_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}

unsafe extern "C" fn h2v2_merged_upsample_565D(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v2_merged_upsample_565D_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v2_merged_upsample_565D_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}
/*
 * Module initialization routine for merged upsampling/color conversion.
 *
 * NB: this is called under the conditions determined by use_merged_upsample()
 * in jdmaster.c.  That routine MUST correspond to the actual capabilities
 * of this module; no safety checks are made here.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_merged_upsampler(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = 0 as *mut my_upsampler;
    upsample = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_upsampler>() as libc::c_ulong,
    ) as my_upsample_ptr;
    (*cinfo).upsample = upsample as *mut crate::jpegint_h::jpeg_upsampler;
    (*upsample).pub_0.start_pass = Some(
        start_pass_merged_upsample
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*upsample).pub_0.need_context_rows = crate::jmorecfg_h::FALSE;
    (*upsample).out_row_width = (*cinfo)
        .output_width
        .wrapping_mul((*cinfo).out_color_components as libc::c_uint);
    if (*cinfo).max_v_samp_factor == 2 as libc::c_int {
        (*upsample).pub_0.upsample = Some(
            merged_2v_upsample
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPIMAGE,
                    _: *mut crate::jmorecfg_h::JDIMENSION,
                    _: crate::jmorecfg_h::JDIMENSION,
                    _: crate::jpeglib_h::JSAMPARRAY,
                    _: *mut crate::jmorecfg_h::JDIMENSION,
                    _: crate::jmorecfg_h::JDIMENSION,
                ) -> (),
        );
        if crate::src::simd::x86_64::jsimd::jsimd_can_h2v2_merged_upsample() != 0 {
            (*upsample).upmethod = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_merged_upsample
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        } else {
            (*upsample).upmethod = Some(
                h2v2_merged_upsample
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        if (*cinfo).out_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_RGB565 as libc::c_int as libc::c_uint
        {
            if (*cinfo).dither_mode as libc::c_uint
                != crate::jpeglib_h::JDITHER_NONE as libc::c_int as libc::c_uint
            {
                (*upsample).upmethod = Some(
                    h2v2_merged_upsample_565D
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*upsample).upmethod = Some(
                    h2v2_merged_upsample_565
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            }
        }
        /* Allocate a spare row buffer */
        (*upsample).spare_row = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            ((*upsample).out_row_width as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong),
        ) as crate::jpeglib_h::JSAMPROW
    } else {
        (*upsample).pub_0.upsample = Some(
            merged_1v_upsample
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPIMAGE,
                    _: *mut crate::jmorecfg_h::JDIMENSION,
                    _: crate::jmorecfg_h::JDIMENSION,
                    _: crate::jpeglib_h::JSAMPARRAY,
                    _: *mut crate::jmorecfg_h::JDIMENSION,
                    _: crate::jmorecfg_h::JDIMENSION,
                ) -> (),
        );
        if crate::src::simd::x86_64::jsimd::jsimd_can_h2v1_merged_upsample() != 0 {
            (*upsample).upmethod = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_merged_upsample
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        } else {
            (*upsample).upmethod = Some(
                h2v1_merged_upsample
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        if (*cinfo).out_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_RGB565 as libc::c_int as libc::c_uint
        {
            if (*cinfo).dither_mode as libc::c_uint
                != crate::jpeglib_h::JDITHER_NONE as libc::c_int as libc::c_uint
            {
                (*upsample).upmethod = Some(
                    h2v1_merged_upsample_565D
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*upsample).upmethod = Some(
                    h2v1_merged_upsample_565
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            }
        }
        /* No spare row needed */
        (*upsample).spare_row = crate::stddef_h::NULL as crate::jpeglib_h::JSAMPROW
    }
    build_ycc_rgb_table(cinfo);
}
/* UPSAMPLE_MERGING_SUPPORTED */
