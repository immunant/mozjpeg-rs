








use crate::jmorecfg_h::{JDIMENSION, JSAMPLE};use crate::jpegint_h::JLONG;use crate::src::jdmerge::{my_upsample_ptr, my_upsampler};use crate::jpeglib_h::{j_decompress_ptr, jpeg_decompress_struct, JSAMPARRAY,
                       JSAMPIMAGE, JSAMPROW};use libc::c_int;
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
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;     let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
     let mut inptr0:   JSAMPROW =
     *(*input_buf.offset(0)).offset(in_row_group_ctr as isize); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        let fresh0 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh0 as c_int;
        let fresh1 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh1 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
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
        y = *fresh2 as c_int;
        *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_0 as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
        let fresh3 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh3 as c_int;
        *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_0 as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
        col -=  1
    }
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_0 as isize) = 0xffu8
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;     let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
     let mut inptr0:   JSAMPROW =
     *(*input_buf.offset(0)).offset(in_row_group_ctr as isize); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh4 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh4 as c_int;
        let fresh5 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh5 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh6 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh6 as c_int;
        *outptr.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
        let fresh7 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh7 as c_int;
        *outptr.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        *outptr.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize)
    };
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;     let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
     let mut inptr0:   JSAMPROW =
     *(*input_buf.offset(0)).offset(in_row_group_ctr as isize); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh8 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh8 as c_int;
        let fresh9 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh9 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh10 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh10 as c_int;
        *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
        let fresh11 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh11 as c_int;
        *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize)
    };
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;     let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
     let mut inptr0:   JSAMPROW =
     *(*input_buf.offset(0)).offset(in_row_group_ctr as isize); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh12 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh12 as c_int;
        let fresh13 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh13 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh14 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh14 as c_int;
        *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_2 as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
        let fresh15 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh15 as c_int;
        *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_2 as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_2 as isize) = 0xffu8
    };
}
#[inline(always)]
pub unsafe extern "C" fn extbgr_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;     let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
     let mut inptr0:   JSAMPROW =
     *(*input_buf.offset(0)).offset(in_row_group_ctr as isize); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh16 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh16 as c_int;
        let fresh17 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh17 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh18 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh18 as c_int;
        *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
        let fresh19 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh19 as c_int;
        *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize)
    };
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;     let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
     let mut inptr0:   JSAMPROW =
     *(*input_buf.offset(0)).offset(in_row_group_ctr as isize); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh20 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh20 as c_int;
        let fresh21 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh21 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh22 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh22 as c_int;
        *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_1 as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
        let fresh23 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh23 as c_int;
        *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_1 as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_1 as isize) = 0xffu8
    };
}
#[inline(always)]
pub unsafe extern "C" fn extxrgb_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;     let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
     let mut inptr0:   JSAMPROW =
     *(*input_buf.offset(0)).offset(in_row_group_ctr as isize); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr:   JSAMPROW =  *output_buf.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh24 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh24 as c_int;
        let fresh25 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh25 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh26 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh26 as c_int;
        *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE as isize);
        let fresh27 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh27 as c_int;
        *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA as isize) = 0xffu8;
        outptr = outptr.offset(RGB_PIXELSIZE as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as c_int;
        *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA as isize) = 0xffu8
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
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;       let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
    
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
    
    
     let mut inptr00:   JSAMPROW =
     *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2u32) as isize); let mut inptr01:   JSAMPROW =
     *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2u32 + 1u32) as isize,
    ); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr0:   JSAMPROW =  *output_buf.offset(0); let mut outptr1:   JSAMPROW =  *output_buf.offset(1); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        /* Do the chroma part of the calculation */
        let fresh28 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh28 as c_int;
        let fresh29 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh29 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
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
        y = *fresh30 as c_int;
        *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_3 as isize);
        let fresh31 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh31 as c_int;
        *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_3 as isize);
        let fresh32 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh32 as c_int;
        *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_3 as isize);
        let fresh33 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh33 as c_int;
        *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_3 as isize);
        col -=  1
    }
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        y = *inptr01 as c_int;
        *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize)
    };
}
#[inline(always)]
pub unsafe extern "C" fn extrgbx_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;       let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
    
    
     let mut inptr00:   JSAMPROW =
     *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2u32) as isize); let mut inptr01:   JSAMPROW =
     *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2u32 + 1u32) as isize,
    ); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr0:   JSAMPROW =  *output_buf.offset(0); let mut outptr1:   JSAMPROW =  *output_buf.offset(1); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh34 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh34 as c_int;
        let fresh35 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh35 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh36 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh36 as c_int;
        *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_2 as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_2 as isize);
        let fresh37 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh37 as c_int;
        *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_2 as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_2 as isize);
        let fresh38 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh38 as c_int;
        *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_2 as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_2 as isize);
        let fresh39 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh39 as c_int;
        *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_2 as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_2 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_2 as isize) = 0xffu8;
        y = *inptr01 as c_int;
        *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_2 as isize) = 0xffu8
    };
}
#[inline(always)]
pub unsafe extern "C" fn extrgb_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;       let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
    
    
     let mut inptr00:   JSAMPROW =
     *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2u32) as isize); let mut inptr01:   JSAMPROW =
     *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2u32 + 1u32) as isize,
    ); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr0:   JSAMPROW =  *output_buf.offset(0); let mut outptr1:   JSAMPROW =  *output_buf.offset(1); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh40 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh40 as c_int;
        let fresh41 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh41 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh42 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh42 as c_int;
        *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_4 as isize);
        let fresh43 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh43 as c_int;
        *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_4 as isize);
        let fresh44 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh44 as c_int;
        *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_4 as isize);
        let fresh45 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh45 as c_int;
        *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_4 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        y = *inptr01 as c_int;
        *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize)
    };
}
#[inline(always)]
pub unsafe extern "C" fn extbgrx_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;       let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
    
    
     let mut inptr00:   JSAMPROW =
     *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2u32) as isize); let mut inptr01:   JSAMPROW =
     *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2u32 + 1u32) as isize,
    ); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr0:   JSAMPROW =  *output_buf.offset(0); let mut outptr1:   JSAMPROW =  *output_buf.offset(1); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh46 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh46 as c_int;
        let fresh47 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh47 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh48 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh48 as c_int;
        *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_1 as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_1 as isize);
        let fresh49 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh49 as c_int;
        *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_1 as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_1 as isize);
        let fresh50 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh50 as c_int;
        *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_1 as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_1 as isize);
        let fresh51 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh51 as c_int;
        *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_1 as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_1 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_1 as isize) = 0xffu8;
        y = *inptr01 as c_int;
        *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_1 as isize) = 0xffu8
    };
}
#[inline(always)]
pub unsafe extern "C" fn h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;       let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
    
    
     let mut inptr00:   JSAMPROW =
     *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2u32) as isize); let mut inptr01:   JSAMPROW =
     *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2u32 + 1u32) as isize,
    ); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr0:   JSAMPROW =  *output_buf.offset(0); let mut outptr1:   JSAMPROW =  *output_buf.offset(1); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh52 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh52 as c_int;
        let fresh53 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh53 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh54 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh54 as c_int;
        *outptr0.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_5 as isize);
        let fresh55 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh55 as c_int;
        *outptr0.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_5 as isize);
        let fresh56 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh56 as c_int;
        *outptr1.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_5 as isize);
        let fresh57 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh57 as c_int;
        *outptr1.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_5 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        *outptr0.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize);
        y = *inptr01 as c_int;
        *outptr1.offset(RGB_RED_5 as isize) =
            *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_5 as isize) =
            *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_5 as isize) =
            *range_limit.offset((y + cblue) as isize)
    };
}
#[inline(always)]
pub unsafe extern "C" fn extxrgb_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;       let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
    
    
     let mut inptr00:   JSAMPROW =
     *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2u32) as isize); let mut inptr01:   JSAMPROW =
     *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2u32 + 1u32) as isize,
    ); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr0:   JSAMPROW =  *output_buf.offset(0); let mut outptr1:   JSAMPROW =  *output_buf.offset(1); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh58 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh58 as c_int;
        let fresh59 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh59 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh60 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh60 as c_int;
        *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE as isize);
        let fresh61 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh61 as c_int;
        *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE as isize);
        let fresh62 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh62 as c_int;
        *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE as isize);
        let fresh63 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh63 as c_int;
        *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA as isize) = 0xffu8;
        y = *inptr01 as c_int;
        *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA as isize) = 0xffu8
    };
}
#[inline(always)]
pub unsafe extern "C" fn extxbgr_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
     let mut y:  c_int =  0; let mut cred:  c_int =  0; let mut cgreen:  c_int =  0; let mut cblue:  c_int =  0; let mut cb:  c_int =  0; let mut cr:  c_int =  0;       let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
    
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    
    
    
    
    
    
     let mut inptr00:   JSAMPROW =
     *(*input_buf.offset(0))
        .offset((in_row_group_ctr * 2u32) as isize); let mut inptr01:   JSAMPROW =
     *(*input_buf.offset(0)).offset(
        (
        in_row_group_ctr * 2u32 + 1u32) as isize,
    ); let mut inptr1:   JSAMPROW =
     *(*input_buf.offset(1)).offset(in_row_group_ctr as isize); let mut inptr2:   JSAMPROW =
     *(*input_buf.offset(2)).offset(in_row_group_ctr as isize); let mut outptr0:   JSAMPROW =  *output_buf.offset(0); let mut outptr1:   JSAMPROW =  *output_buf.offset(1); let mut col:   JDIMENSION =  (*cinfo).output_width >> 1i32;
    while col > 0u32 {
        let fresh64 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh64 as c_int;
        let fresh65 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh65 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh66 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh66 as c_int;
        *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_0 as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_0 as isize);
        let fresh67 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh67 as c_int;
        *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_0 as isize) = 0xffu8;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_0 as isize);
        let fresh68 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh68 as c_int;
        *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_0 as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_0 as isize);
        let fresh69 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh69 as c_int;
        *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_0 as isize) = 0xffu8;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_0 as isize);
        col -=  1
    }
    if (*cinfo).output_width & 1u32 != 0 {
        cb = *inptr1 as c_int;
        cr = *inptr2 as c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen =
            (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32) as c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as c_int;
        *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_0 as isize) = 0xffu8;
        y = *inptr01 as c_int;
        *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_0 as isize) = 0xffu8
    };
}























































use crate::jmorecfg_h::{EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE,
                        EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN,
                        EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_RGBX_BLUE,
                        EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED,
                        EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE,
                        EXT_RGB_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN,
                        EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE,
                        EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE, EXT_XRGB_RED,
                        RGB_BLUE_5, RGB_GREEN_5, RGB_PIXELSIZE_5, RGB_RED_5};use crate::src::jdmerge::{RGB_ALPHA, RGB_ALPHA_0, RGB_ALPHA_1, RGB_ALPHA_2,
                          RGB_BLUE, RGB_BLUE_0, RGB_BLUE_1, RGB_BLUE_2,
                          RGB_BLUE_3, RGB_BLUE_4, RGB_GREEN, RGB_GREEN_0,
                          RGB_GREEN_1, RGB_GREEN_2, RGB_GREEN_3, RGB_GREEN_4,
                          RGB_PIXELSIZE, RGB_PIXELSIZE_0, RGB_PIXELSIZE_1,
                          RGB_PIXELSIZE_2, RGB_PIXELSIZE_3, RGB_PIXELSIZE_4,
                          RGB_RED, RGB_RED_0, RGB_RED_1, RGB_RED_2, RGB_RED_3,
                          RGB_RED_4};
