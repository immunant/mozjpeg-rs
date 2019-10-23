use crate::jmorecfg_h::INT16;
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
use crate::stddef_h::size_t;
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
    
    
    
    
    
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
         let mut y:  libc::c_int =  0; let mut cb:  libc::c_int =  0; let mut cr:  libc::c_int =  0;      let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh53 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh53;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
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
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
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
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
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
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = ((r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32)
                << 16i32) as libc::c_long
                | rgb;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            y = *inptr0 as libc::c_int;
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
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
    
    
    
    
    
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    loop {
         let mut y:  libc::c_int =  0; let mut cb:  libc::c_int =  0; let mut cr:  libc::c_int =  0;      let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh63 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh63;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
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
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
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
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
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
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = rgb << 16i32
                | (r & 0xf8u32
                    | g >> 5i32
                    | g << 11i32 & 0xe000u32
                    | b << 5i32 & 0x1f00u32) as libc::c_long;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            y = *inptr0 as libc::c_int;
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize) as libc::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize) as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
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
         let mut y:  libc::c_int =  0; let mut cb:  libc::c_int =  0; let mut cr:  libc::c_int =  0;      let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh73 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh73;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
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
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
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
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
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
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = ((r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32)
                << 16i32) as libc::c_long
                | rgb;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            y = *inptr0 as libc::c_int;
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
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
    
    
    
    
    
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut libc::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut libc::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jpegint_h::JLONG = (*cconvert).Cb_g_tab;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    loop {
         let mut y:  libc::c_int =  0; let mut cb:  libc::c_int =  0; let mut cr:  libc::c_int =  0;      let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh83 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh83;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
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
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
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
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
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
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = rgb << 16i32
                | (r & 0xf8u32
                    | g >> 5i32
                    | g << 11i32 & 0xe000u32
                    | b << 5i32 & 0x1f00u32) as libc::c_long;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            y = *inptr0 as libc::c_int;
            cb = *inptr1 as libc::c_int;
            cr = *inptr2 as libc::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                    as libc::c_int) as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
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
    
    
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
              let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh93 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh93;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh94 = inptr0;
            inptr0 = inptr0.offset(1);
            r =  *fresh94 as libc::c_uint;
            let fresh95 = inptr1;
            inptr1 = inptr1.offset(1);
            g =  *fresh95 as libc::c_uint;
            let fresh96 = inptr2;
            inptr2 = inptr2.offset(1);
            b =  *fresh96 as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh97 = inptr0;
            inptr0 = inptr0.offset(1);
            r =  *fresh97 as libc::c_uint;
            let fresh98 = inptr1;
            inptr1 = inptr1.offset(1);
            g =  *fresh98 as libc::c_uint;
            let fresh99 = inptr2;
            inptr2 = inptr2.offset(1);
            b =  *fresh99 as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
            let fresh100 = inptr0;
            inptr0 = inptr0.offset(1);
            r =  *fresh100 as libc::c_uint;
            let fresh101 = inptr1;
            inptr1 = inptr1.offset(1);
            g =  *fresh101 as libc::c_uint;
            let fresh102 = inptr2;
            inptr2 = inptr2.offset(1);
            b =  *fresh102 as libc::c_uint;
            rgb = ((r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32)
                << 16i32) as libc::c_long
                | rgb;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            r =  *inptr0 as libc::c_uint;
            g =  *inptr1 as libc::c_uint;
            b =  *inptr2 as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
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
    
    
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
              let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh103 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh103;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh104 = inptr0;
            inptr0 = inptr0.offset(1);
            r =  *fresh104 as libc::c_uint;
            let fresh105 = inptr1;
            inptr1 = inptr1.offset(1);
            g =  *fresh105 as libc::c_uint;
            let fresh106 = inptr2;
            inptr2 = inptr2.offset(1);
            b =  *fresh106 as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh107 = inptr0;
            inptr0 = inptr0.offset(1);
            r =  *fresh107 as libc::c_uint;
            let fresh108 = inptr1;
            inptr1 = inptr1.offset(1);
            g =  *fresh108 as libc::c_uint;
            let fresh109 = inptr2;
            inptr2 = inptr2.offset(1);
            b =  *fresh109 as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            let fresh110 = inptr0;
            inptr0 = inptr0.offset(1);
            r =  *fresh110 as libc::c_uint;
            let fresh111 = inptr1;
            inptr1 = inptr1.offset(1);
            g =  *fresh111 as libc::c_uint;
            let fresh112 = inptr2;
            inptr2 = inptr2.offset(1);
            b =  *fresh112 as libc::c_uint;
            rgb = rgb << 16i32
                | (r & 0xf8u32
                    | g >> 5i32
                    | g << 11i32 & 0xe000u32
                    | b << 5i32 & 0x1f00u32) as libc::c_long;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            r =  *inptr0 as libc::c_uint;
            g =  *inptr1 as libc::c_uint;
            b =  *inptr2 as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
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
    
    
    
    
    
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    loop {
              let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh113 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh113;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh114 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset(
                (*fresh114 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            let fresh115 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh115 as libc::c_int as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            let fresh116 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset(
                (*fresh116 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh117 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset(
                (*fresh117 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            let fresh118 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh118 as libc::c_int as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            let fresh119 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset(
                (*fresh119 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            let fresh120 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset(
                (*fresh120 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            let fresh121 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh121 as libc::c_int as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            let fresh122 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset(
                (*fresh122 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = rgb << 16i32
                | (r & 0xf8u32
                    | g >> 5i32
                    | g << 11i32 & 0xe000u32
                    | b << 5i32 & 0x1f00u32) as libc::c_long;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            r = *range_limit.offset(
                (*inptr0 as libc::c_int as libc::c_long + (d0 & 0xffi64)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                (*inptr1 as libc::c_int as libc::c_long + ((d0 & 0xffi64) >> 1i32))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                (*inptr2 as libc::c_int as libc::c_long + (d0 & 0xffi64)) as isize,
            ) as libc::c_uint;
            rgb = (r & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | b << 5i32 & 0x1f00u32)
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
    
    
    
    
    
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    loop {
              let mut rgb:  crate::jpegint_h::JLONG =  0; let mut r:  libc::c_uint =  0; let mut g:  libc::c_uint =  0; let mut b:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        
        
        
        
         let mut inptr0:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(input_row as isize); let mut inptr1:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(1)).offset(input_row as isize); let mut inptr2:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(2)).offset(input_row as isize);
        input_row +=  1;
        let fresh123 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh123;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh124 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset(
                (*fresh124 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            let fresh125 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh125 as libc::c_int as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            let fresh126 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset(
                (*fresh126 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh127 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset(
                (*fresh127 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            let fresh128 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh128 as libc::c_int as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            let fresh129 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset(
                (*fresh129 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
            let fresh130 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset(
                (*fresh130 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            let fresh131 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh131 as libc::c_int as libc::c_long
                    + ((d0 & 0xffi64) >> 1i32)) as isize,
            ) as libc::c_uint;
            let fresh132 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset(
                (*fresh132 as libc::c_int as libc::c_long + (d0 & 0xffi64))
                    as isize,
            ) as libc::c_uint;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            rgb = ((r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32)
                << 16i32) as libc::c_long
                | rgb;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            r = *range_limit.offset(
                (*inptr0 as libc::c_int as libc::c_long + (d0 & 0xffi64)) as isize,
            ) as libc::c_uint;
            g = *range_limit.offset(
                (*inptr1 as libc::c_int as libc::c_long + ((d0 & 0xffi64) >> 1i32))
                    as isize,
            ) as libc::c_uint;
            b = *range_limit.offset(
                (*inptr2 as libc::c_int as libc::c_long + (d0 & 0xffi64)) as isize,
            ) as libc::c_uint;
            rgb = (r << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | b >> 3i32) as crate::jpegint_h::JLONG;
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
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
            let mut rgb:  crate::jpegint_h::JLONG =  0; let mut g:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        let fresh133 = input_row;
        input_row +=  1;
         let mut inptr:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(fresh133 as isize);
        let fresh134 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh134;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh135 = inptr;
            inptr = inptr.offset(1);
            g = *fresh135 as libc::c_uint;
            rgb = (g & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | g << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh136 = inptr;
            inptr = inptr.offset(1);
            g = *fresh136 as libc::c_uint;
            rgb = (g & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | g << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            let fresh137 = inptr;
            inptr = inptr.offset(1);
            g = *fresh137 as libc::c_uint;
            rgb = rgb << 16i32
                | (g & 0xf8u32
                    | g >> 5i32
                    | g << 11i32 & 0xe000u32
                    | g << 5i32 & 0x1f00u32) as libc::c_long;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            g = *inptr as libc::c_uint;
            rgb = (g & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | g << 5i32 & 0x1f00u32)
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
    
    
    
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
            let mut rgb:  crate::jpegint_h::JLONG =  0; let mut g:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        let fresh138 = input_row;
        input_row +=  1;
         let mut inptr:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(fresh138 as isize);
        let fresh139 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh139;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh140 = inptr;
            inptr = inptr.offset(1);
            g = *fresh140 as libc::c_uint;
            rgb = (g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32) as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh141 = inptr;
            inptr = inptr.offset(1);
            g = *fresh141 as libc::c_uint;
            rgb = (g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32) as crate::jpegint_h::JLONG;
            let fresh142 = inptr;
            inptr = inptr.offset(1);
            g = *fresh142 as libc::c_uint;
            rgb = ((g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32)
                << 16i32) as libc::c_long
                | rgb;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            g = *inptr as libc::c_uint;
            rgb = (g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32) as crate::jpegint_h::JLONG;
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
    
    
    
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    loop {
            let mut rgb:  crate::jpegint_h::JLONG =  0; let mut g:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        let fresh143 = input_row;
        input_row +=  1;
         let mut inptr:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(fresh143 as isize);
        let fresh144 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh144;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh145 = inptr;
            inptr = inptr.offset(1);
            g = *fresh145 as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = (g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32) as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh146 = inptr;
            inptr = inptr.offset(1);
            g = *fresh146 as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = (g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32) as crate::jpegint_h::JLONG;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            let fresh147 = inptr;
            inptr = inptr.offset(1);
            g = *fresh147 as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = ((g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32)
                << 16i32) as libc::c_long
                | rgb;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            g = *inptr as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = (g << 8i32 & 0xf800u32
                | g << 3i32 & 0x7e0u32
                | g >> 3i32) as crate::jpegint_h::JLONG;
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
    
    
    
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut d0: crate::jpegint_h::JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as libc::c_uint) as usize];
    loop {
            let mut rgb:  crate::jpegint_h::JLONG =  0; let mut g:  libc::c_uint =  0;num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        
        
        let fresh148 = input_row;
        input_row +=  1;
         let mut inptr:   crate::jpeglib_h::JSAMPROW =
     *(*input_buf.offset(0)).offset(fresh148 as isize);
        let fresh149 = output_buf;
        output_buf = output_buf.offset(1);
         let mut outptr:   crate::jpeglib_h::JSAMPROW =  *fresh149;
        if outptr as crate::stddef_h::size_t & 3u64 != 0 {
            let fresh150 = inptr;
            inptr = inptr.offset(1);
            g = *fresh150 as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = (g & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | g << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16;
            outptr = outptr.offset(2);
            num_cols -=  1
        }
         let mut col:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while col < num_cols >> 1i32 {
            let fresh151 = inptr;
            inptr = inptr.offset(1);
            g = *fresh151 as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = (g & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | g << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            let fresh152 = inptr;
            inptr = inptr.offset(1);
            g = *fresh152 as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = rgb << 16i32
                | (g & 0xf8u32
                    | g >> 5i32
                    | g << 11i32 & 0xe000u32
                    | g << 5i32 & 0x1f00u32) as libc::c_long;
            d0 = (d0 & 0xffi64) << 24i32 | d0 >> 8i32 & 0xffffffi64;
            *(outptr as *mut libc::c_int) = rgb as libc::c_int;
            outptr = outptr.offset(4);
            col +=  1
        }
        if num_cols & 1u32 != 0 {
            g = *inptr as libc::c_uint;
            g = *range_limit.offset((g as libc::c_long + (d0 & 0xffi64)) as isize)
                as libc::c_uint;
            rgb = (g & 0xf8u32
                | g >> 5i32
                | g << 11i32 & 0xe000u32
                | g << 5i32 & 0x1f00u32)
                as crate::jpegint_h::JLONG;
            *(outptr as *mut crate::jmorecfg_h::INT16) = rgb as crate::jmorecfg_h::INT16
        }
    }
}
use crate::src::jdcolor::dither_matrix;
use crate::src::jdcolor::DITHER_MASK;
