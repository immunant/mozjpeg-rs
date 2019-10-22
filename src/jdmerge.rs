use libc;

pub use crate::jdmrg565_c::h2v1_merged_upsample_565D_be;
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
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
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
    pub pub_0: crate::jpeglib_h::jpeg_upsampler,
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

pub const SCALEBITS: libc::c_int = 16i32;
/* speediest right-shift on some machines */

pub const ONE_HALF: crate::jpegint_h::JLONG = (1i32 as crate::jpegint_h::JLONG) << SCALEBITS - 1i32;
/* Include inline routines for colorspace extensions */

pub const RGB_RED_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_RED;

pub const RGB_GREEN_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_GREEN;

pub const RGB_BLUE_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_BLUE;

pub const RGB_PIXELSIZE_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_PIXELSIZE;

pub const RGB_RED_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_RED;

pub const RGB_GREEN_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_GREEN;

pub const RGB_BLUE_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_BLUE;

pub const RGB_ALPHA_2: libc::c_int = 3i32;

pub const RGB_PIXELSIZE_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;

pub const RGB_RED_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_RED;

pub const RGB_GREEN_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_GREEN;

pub const RGB_BLUE_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_BLUE;

pub const RGB_PIXELSIZE_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_PIXELSIZE;

pub const RGB_RED_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_RED;

pub const RGB_GREEN_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_GREEN;

pub const RGB_BLUE_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_BLUE;

pub const RGB_ALPHA_1: libc::c_int = 3i32;

pub const RGB_PIXELSIZE_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;

pub const RGB_RED_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_RED;

pub const RGB_GREEN_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_GREEN;

pub const RGB_BLUE_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_BLUE;

pub const RGB_ALPHA_0: libc::c_int = 0i32;

pub const RGB_PIXELSIZE_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;

pub const RGB_RED: libc::c_int = crate::jmorecfg_h::EXT_XRGB_RED;

pub const RGB_GREEN: libc::c_int = crate::jmorecfg_h::EXT_XRGB_GREEN;

pub const RGB_BLUE: libc::c_int = crate::jmorecfg_h::EXT_XRGB_BLUE;

pub const RGB_ALPHA: libc::c_int = 0i32;

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
        ((crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong)
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
        ((crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong)
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
        ((crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong)
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
        ((crate::jmorecfg_h::MAXJSAMPLE + 1i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong),
    ) as *mut crate::jpegint_h::JLONG;
    i = 0i32;
    x = -crate::jmorecfg_h::CENTERJSAMPLE as crate::jpegint_h::JLONG;
    while i <= crate::jmorecfg_h::MAXJSAMPLE {
        /* i is the actual input pixel value, in the range 0..MAXJSAMPLE */
        /* The Cb or Cr value we are thinking of is x = i - CENTERJSAMPLE */
        /* Cr=>R value is nearest int to 1.40200 * x */
        *(*upsample).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * (1i64 << 16i32) as libc::c_double + 0.5f64) as crate::jpegint_h::JLONG
                * x
                + ((1i32 as crate::jpegint_h::JLONG) << 16i32 - 1i32)
                >> 16i32) as libc::c_int;
        /* Cb=>B value is nearest int to 1.77200 * x */
        *(*upsample).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * (1i64 << 16i32) as libc::c_double + 0.5f64) as crate::jpegint_h::JLONG
                * x
                + ((1i32 as crate::jpegint_h::JLONG) << 16i32 - 1i32)
                >> 16i32) as libc::c_int;
        /* Cr=>G value is scaled-up -0.71414 * x */
        *(*upsample).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * (1i64 << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        *(*upsample).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * (1i64 << SCALEBITS) as libc::c_double + 0.5f64)
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
            size = (*cinfo).output_width.wrapping_mul(2i32 as libc::c_uint)
        }
        crate::jpegint_h::jcopy_sample_rows(
            &mut (*upsample).spare_row,
            0i32,
            output_buf.offset(*out_row_ctr as isize),
            0i32,
            1i32,
            size,
        );
        num_rows = 1i32 as crate::jmorecfg_h::JDIMENSION;
        (*upsample).spare_full = crate::jmorecfg_h::FALSE
    } else {
        /* Figure number of rows to return to caller. */
        num_rows = 2i32 as crate::jmorecfg_h::JDIMENSION;
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
        work_ptrs[0] = *output_buf.offset(*out_row_ctr as isize);
        if num_rows > 1i32 as libc::c_uint {
            work_ptrs[1] =
                *output_buf.offset((*out_row_ctr).wrapping_add(1i32 as libc::c_uint) as isize)
        } else {
            work_ptrs[1] = (*upsample).spare_row;
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
            crate::jdmrgext_c::extrgb_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        7 | 12 => {
            crate::jdmrgext_c::extrgbx_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        8 => {
            crate::jdmrgext_c::extbgr_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        9 | 13 => {
            crate::jdmrgext_c::extbgrx_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        10 | 14 => {
            crate::jdmrgext_c::extxbgr_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        11 | 15 => {
            crate::jdmrgext_c::extxrgb_h2v1_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        _ => {
            crate::jdmrgext_c::h2v1_merged_upsample_internal(
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            crate::jdmrgext_c::extrgb_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        7 | 12 => {
            crate::jdmrgext_c::extrgbx_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        8 => {
            crate::jdmrgext_c::extbgr_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        9 | 13 => {
            crate::jdmrgext_c::extbgrx_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        10 | 14 => {
            crate::jdmrgext_c::extxbgr_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        11 | 15 => {
            crate::jdmrgext_c::extxrgb_h2v2_merged_upsample_internal(
                cinfo,
                input_buf,
                in_row_group_ctr,
                output_buf,
            );
        }
        _ => {
            crate::jdmrgext_c::h2v2_merged_upsample_internal(
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

pub const DITHER_MASK: libc::c_int = 0x3i32;

pub(crate) static mut dither_matrix: [crate::jpegint_h::JLONG; 4] = [
    0x8020ai32 as crate::jpegint_h::JLONG,
    0xc040e06i32 as crate::jpegint_h::JLONG,
    0x30b0109i32 as crate::jpegint_h::JLONG,
    0xf070d05i32 as crate::jpegint_h::JLONG,
];
/* Include inline routines for RGB565 conversion */
#[inline(always)]

unsafe extern "C" fn is_big_endian() -> crate::jmorecfg_h::boolean {
    let mut test_value: libc::c_int = 1i32;
    if *(&mut test_value as *mut libc::c_int as *mut libc::c_char) as libc::c_int != 1i32 {
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
        crate::jdmrg565_c::h2v1_merged_upsample_565_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        crate::jdmrg565_c::h2v1_merged_upsample_565_le(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}

unsafe extern "C" fn h2v1_merged_upsample_565D(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        crate::jdmrg565_c::h2v1_merged_upsample_565D_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        crate::jdmrg565_c::h2v1_merged_upsample_565D_le(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}

unsafe extern "C" fn h2v2_merged_upsample_565(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        crate::jdmrg565_c::h2v2_merged_upsample_565_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        crate::jdmrg565_c::h2v2_merged_upsample_565_le(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}

unsafe extern "C" fn h2v2_merged_upsample_565D(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        crate::jdmrg565_c::h2v2_merged_upsample_565D_be(
            cinfo,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        crate::jdmrg565_c::h2v2_merged_upsample_565D_le(
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
    (*cinfo).upsample = upsample as *mut crate::jpeglib_h::jpeg_upsampler;
    (*upsample).pub_0.start_pass = Some(
        start_pass_merged_upsample
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*upsample).pub_0.need_context_rows = crate::jmorecfg_h::FALSE;
    (*upsample).out_row_width = (*cinfo)
        .output_width
        .wrapping_mul((*cinfo).out_color_components as libc::c_uint);
    if (*cinfo).max_v_samp_factor == 2i32 {
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
