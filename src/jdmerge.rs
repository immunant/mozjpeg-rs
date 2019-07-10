use libc::c_double;use libc::c_char;use libc::c_int;use libc::c_uint;use libc::c_ulong;pub use crate::jdmrg565_c::h2v1_merged_upsample_565D_be;
pub use crate::jdmrg565_c::h2v1_merged_upsample_565D_le;
pub use crate::jdmrg565_c::h2v1_merged_upsample_565_be;
pub use crate::jdmrg565_c::h2v1_merged_upsample_565_le;
pub use crate::jdmrg565_c::h2v2_merged_upsample_565D_be;
pub use crate::jdmrg565_c::h2v2_merged_upsample_565D_le;
pub use crate::jdmrg565_c::h2v2_merged_upsample_565_be;
pub use crate::jdmrg565_c::h2v2_merged_upsample_565_le;
pub use crate::jdmrgext_c::extbgr_h2v1_merged_upsample_internal;
pub use crate::jdmrgext_c::extbgr_h2v2_merged_upsample_internal;
pub use crate::jdmrgext_c::extbgrx_h2v1_merged_upsample_internal;
pub use crate::jdmrgext_c::extbgrx_h2v2_merged_upsample_internal;
pub use crate::jdmrgext_c::extrgb_h2v1_merged_upsample_internal;
pub use crate::jdmrgext_c::extrgb_h2v2_merged_upsample_internal;
pub use crate::jdmrgext_c::extrgbx_h2v1_merged_upsample_internal;
pub use crate::jdmrgext_c::extrgbx_h2v2_merged_upsample_internal;
pub use crate::jdmrgext_c::extxbgr_h2v1_merged_upsample_internal;
pub use crate::jdmrgext_c::extxbgr_h2v2_merged_upsample_internal;
pub use crate::jdmrgext_c::extxrgb_h2v1_merged_upsample_internal;
pub use crate::jdmrgext_c::extxrgb_h2v2_merged_upsample_internal;
pub use crate::jdmrgext_c::h2v1_merged_upsample_internal;
pub use crate::jdmrgext_c::h2v2_merged_upsample_internal;
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
pub use crate::jpegint_h::jcopy_sample_rows;
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
use crate::jsimd::jsimd_can_h2v1_merged_upsample;
use crate::jsimd::jsimd_can_h2v2_merged_upsample;
use crate::jsimd::jsimd_h2v1_merged_upsample;
use crate::jsimd::jsimd_h2v2_merged_upsample;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use libc;
pub type my_upsample_ptr = *mut my_upsampler;
/*
 * jdmerge.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2009, 2011, 2014-2015, D. R. Commander.
 * Copyright (C) 2013, Linaro Limited.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains code for merged upsampling/color conversion.
 *
 * This file combines functions from jdsample.c and jdcolor.c;
 * read those files first to understand what's going on.
 *
 * When the chroma components are to be upsampled by simple replication
 * (ie, box filtering), we can save some work in color conversion by
 * calculating all the output pixels corresponding to a pair of chroma
 * samples at one time.  In the conversion equations
 *      R = Y           + K1 * Cr
 *      G = Y + K2 * Cb + K3 * Cr
 *      B = Y + K4 * Cb
 * only the Y term varies among the group of pixels corresponding to a pair
 * of chroma samples, so the rest of the terms can be calculated just once.
 * At typical sampling ratios, this eliminates half or three-quarters of the
 * multiplications needed for color conversion.
 *
 * This file currently provides implementations for the following cases:
 *      YCbCr => RGB color conversion only.
 *      Sampling ratios of 2h1v or 2h2v.
 *      No scaling needed at upsample time.
 *      Corner-aligned (non-CCIR601) sampling alignment.
 * Other special cases could be added, but in most applications these are
 * the only common cases.  (For uncommon cases we fall back on the more
 * general code in jdsample.c and jdcolor.c.)
 */
/* Private subobject */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_upsampler {
    pub pub_0: jpeg_upsampler,
    pub upmethod: Option<
        unsafe extern "C" fn(
            _: j_decompress_ptr,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: JSAMPARRAY,
        ) -> (),
    >,
    pub Cr_r_tab: *mut c_int,
    pub Cb_b_tab: *mut c_int,
    pub Cr_g_tab: *mut JLONG,
    pub Cb_g_tab: *mut JLONG,
    pub spare_row: JSAMPROW,
    pub spare_full: boolean,
    pub out_row_width: JDIMENSION,
    pub rows_to_go: JDIMENSION,
}
/* speediest right-shift on some machines */
pub const SCALEBITS: c_int = 16i32;
pub const ONE_HALF: JLONG = (1i32 as JLONG) << SCALEBITS - 1i32;
/* Include inline routines for colorspace extensions */
pub const RGB_RED_4: c_int = EXT_RGB_RED;
pub const RGB_GREEN_4: c_int = EXT_RGB_GREEN;
pub const RGB_BLUE_4: c_int = EXT_RGB_BLUE;
pub const RGB_PIXELSIZE_4: c_int = EXT_RGB_PIXELSIZE;
pub const RGB_RED_2: c_int = EXT_RGBX_RED;
pub const RGB_GREEN_2: c_int = EXT_RGBX_GREEN;
pub const RGB_BLUE_2: c_int = EXT_RGBX_BLUE;
pub const RGB_ALPHA_2: c_int = 3i32;
pub const RGB_PIXELSIZE_2: c_int = EXT_RGBX_PIXELSIZE;
pub const RGB_RED_3: c_int = EXT_BGR_RED;
pub const RGB_GREEN_3: c_int = EXT_BGR_GREEN;
pub const RGB_BLUE_3: c_int = EXT_BGR_BLUE;
pub const RGB_PIXELSIZE_3: c_int = EXT_BGR_PIXELSIZE;
pub const RGB_RED_1: c_int = EXT_BGRX_RED;
pub const RGB_GREEN_1: c_int = EXT_BGRX_GREEN;
pub const RGB_BLUE_1: c_int = EXT_BGRX_BLUE;
pub const RGB_ALPHA_1: c_int = 3i32;
pub const RGB_PIXELSIZE_1: c_int = EXT_BGRX_PIXELSIZE;
pub const RGB_RED_0: c_int = EXT_XBGR_RED;
pub const RGB_GREEN_0: c_int = EXT_XBGR_GREEN;
pub const RGB_BLUE_0: c_int = EXT_XBGR_BLUE;
pub const RGB_ALPHA_0: c_int = 0i32;
pub const RGB_PIXELSIZE_0: c_int = EXT_XBGR_PIXELSIZE;
pub const RGB_RED: c_int = EXT_XRGB_RED;
pub const RGB_GREEN: c_int = EXT_XRGB_GREEN;
pub const RGB_BLUE: c_int = EXT_XRGB_BLUE;
pub const RGB_ALPHA: c_int = 0i32;
pub const RGB_PIXELSIZE: c_int = EXT_XRGB_PIXELSIZE;
/*
 * Initialize tables for YCC->RGB colorspace conversion.
 * This is taken directly from jdcolor.c; see that file for more info.
 */
unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut i: c_int = 0;
    let mut x: JLONG = 0;
    (*upsample).Cr_r_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut c_int;
    (*upsample).Cb_b_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut c_int;
    (*upsample).Cr_g_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    (*upsample).Cb_g_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    i = 0i32;
    x = -CENTERJSAMPLE as JLONG;
    while i <= MAXJSAMPLE {
        *(*upsample).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG
                * x
                + ((1i32 as JLONG) << 16i32 - 1i32)
                >> 16i32) as c_int;
        *(*upsample).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG
                * x
                + ((1i32 as JLONG) << 16i32 - 1i32)
                >> 16i32) as c_int;
        *(*upsample).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG)
                * x;
        *(*upsample).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG)
                * x
                + ONE_HALF;
        i += 1;
        x += 1
    }
}
/*
 * Initialize for an upsampling pass.
 */
unsafe extern "C" fn start_pass_merged_upsample(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    (*upsample).spare_full = FALSE;
    (*upsample).rows_to_go = (*cinfo).output_height;
}
/*
 * Control routine to do upsampling (and color conversion).
 *
 * The control routine just handles the row buffering considerations.
 */
unsafe extern "C" fn merged_2v_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut work_ptrs: [JSAMPROW; 2] = [0 as *mut JSAMPLE; 2];
    /* number of rows returned to caller */
    let mut num_rows: JDIMENSION = 0;
    if 0 != (*upsample).spare_full {
        let mut size: JDIMENSION = (*upsample).out_row_width;
        if (*cinfo).out_color_space as c_uint
            == JCS_RGB565 as c_int as c_uint
        {
            size = (*cinfo).output_width.wrapping_mul(2i32 as c_uint)
        }
        jcopy_sample_rows(
            &mut (*upsample).spare_row,
            0i32,
            output_buf.offset(*out_row_ctr as isize),
            0i32,
            1i32,
            size,
        );
        num_rows = 1i32 as JDIMENSION;
        (*upsample).spare_full = FALSE
    } else {
        num_rows = 2i32 as JDIMENSION;
        if num_rows > (*upsample).rows_to_go {
            num_rows = (*upsample).rows_to_go
        }
        out_rows_avail = (out_rows_avail as c_uint).wrapping_sub(*out_row_ctr)
            as JDIMENSION
            as JDIMENSION;
        if num_rows > out_rows_avail {
            num_rows = out_rows_avail
        }
        work_ptrs[0usize] = *output_buf.offset(*out_row_ctr as isize);
        if num_rows > 1i32 as c_uint {
            work_ptrs[1usize] =
                *output_buf.offset((*out_row_ctr).wrapping_add(1i32 as c_uint) as isize)
        } else {
            work_ptrs[1usize] = (*upsample).spare_row;
            (*upsample).spare_full = TRUE
        }
        (*upsample).upmethod.expect("non-null function pointer")(
            cinfo,
            input_buf,
            *in_row_group_ctr,
            work_ptrs.as_mut_ptr(),
        );
    }
    *out_row_ctr = (*out_row_ctr as c_uint).wrapping_add(num_rows)
        as JDIMENSION as JDIMENSION;
    (*upsample).rows_to_go = ((*upsample).rows_to_go as c_uint).wrapping_sub(num_rows)
        as JDIMENSION
        as JDIMENSION;
    if 0 == (*upsample).spare_full {
        *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1)
    };
}
unsafe extern "C" fn merged_1v_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    (*upsample).upmethod.expect("non-null function pointer")(
        cinfo,
        input_buf,
        *in_row_group_ctr,
        output_buf.offset(*out_row_ctr as isize),
    );
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
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            extrgb_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        7 | 12 => {
            extrgbx_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        8 => {
            extbgr_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        9 | 13 => {
            extbgrx_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        10 | 14 => {
            extxbgr_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        11 | 15 => {
            extxrgb_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        _ => {
            h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
    };
}
/*
 * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
 */
unsafe extern "C" fn h2v2_merged_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            extrgb_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        7 | 12 => {
            extrgbx_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        8 => {
            extbgr_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        9 | 13 => {
            extbgrx_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        10 | 14 => {
            extxbgr_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        11 | 15 => {
            extxrgb_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        _ => {
            h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
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
pub const DITHER_MASK: c_int = 0x3i32;
pub(crate) static mut dither_matrix: [JLONG; 4] = [
    0x8020ai32 as JLONG,
    0xc040e06i32 as JLONG,
    0x30b0109i32 as JLONG,
    0xf070d05i32 as JLONG,
];
/* Include inline routines for RGB565 conversion */
#[inline(always)]
unsafe extern "C" fn is_big_endian() -> boolean {
    let mut test_value: c_int = 1i32;
    if *(&mut test_value as *mut c_int as *mut c_char) as c_int != 1i32 {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn h2v1_merged_upsample_565(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if 0 != is_big_endian() {
        h2v1_merged_upsample_565_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        h2v1_merged_upsample_565_le(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}
unsafe extern "C" fn h2v1_merged_upsample_565D(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if 0 != is_big_endian() {
        h2v1_merged_upsample_565D_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        h2v1_merged_upsample_565D_le(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}
unsafe extern "C" fn h2v2_merged_upsample_565(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if 0 != is_big_endian() {
        h2v2_merged_upsample_565_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        h2v2_merged_upsample_565_le(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}
unsafe extern "C" fn h2v2_merged_upsample_565D(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if 0 != is_big_endian() {
        h2v2_merged_upsample_565D_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        h2v2_merged_upsample_565D_le(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
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
pub unsafe extern "C" fn jinit_merged_upsampler(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = 0 as *mut my_upsampler;
    upsample = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_upsampler>() as c_ulong,
    ) as my_upsample_ptr;
    (*cinfo).upsample = upsample as *mut jpeg_upsampler;
    (*upsample).pub_0.start_pass = Some(
        start_pass_merged_upsample
            as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
    (*upsample).pub_0.need_context_rows = FALSE;
    (*upsample).out_row_width = (*cinfo)
        .output_width
        .wrapping_mul((*cinfo).out_color_components as c_uint);
    if (*cinfo).max_v_samp_factor == 2i32 {
        (*upsample).pub_0.upsample = Some(
            merged_2v_upsample
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPIMAGE,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                    _: JSAMPARRAY,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                ) -> (),
        );
        if 0 != jsimd_can_h2v2_merged_upsample() {
            (*upsample).upmethod = Some(
                jsimd_h2v2_merged_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        } else {
            (*upsample).upmethod = Some(
                h2v2_merged_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        if (*cinfo).out_color_space as c_uint
            == JCS_RGB565 as c_int as c_uint
        {
            if (*cinfo).dither_mode as c_uint
                != JDITHER_NONE as c_int as c_uint
            {
                (*upsample).upmethod = Some(
                    h2v2_merged_upsample_565D
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*upsample).upmethod = Some(
                    h2v2_merged_upsample_565
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            }
        }
        (*upsample).spare_row = (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*upsample).out_row_width as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
        ) as JSAMPROW
    } else {
        (*upsample).pub_0.upsample = Some(
            merged_1v_upsample
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPIMAGE,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                    _: JSAMPARRAY,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                ) -> (),
        );
        if 0 != jsimd_can_h2v1_merged_upsample() {
            (*upsample).upmethod = Some(
                jsimd_h2v1_merged_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        } else {
            (*upsample).upmethod = Some(
                h2v1_merged_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        if (*cinfo).out_color_space as c_uint
            == JCS_RGB565 as c_int as c_uint
        {
            if (*cinfo).dither_mode as c_uint
                != JDITHER_NONE as c_int as c_uint
            {
                (*upsample).upmethod = Some(
                    h2v1_merged_upsample_565D
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*upsample).upmethod = Some(
                    h2v1_merged_upsample_565
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            }
        }
        (*upsample).spare_row = NULL as JSAMPROW
    }
    build_ycc_rgb_table(cinfo);
}
