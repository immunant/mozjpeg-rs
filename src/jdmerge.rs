













































































































































use super::simd::x86_64::jsimd::{jsimd_can_h2v1_merged_upsample,
                                 jsimd_can_h2v2_merged_upsample,
                                 jsimd_h2v1_merged_upsample,
                                 jsimd_h2v2_merged_upsample};use libc::{c_uint, c_double, c_char, c_ulong, c_int, self};pub use crate::jpegint_h::{inverse_DCT_method_ptr, jcopy_sample_rows,
                           JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
                           JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, JLONG,
                           J_BUF_MODE};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_parser_method,
                           jpeg_marker_reader, jpeg_marker_struct,
                           jpeg_memory_mgr, jpeg_progress_mgr,
                           jpeg_saved_marker_ptr, jpeg_source_mgr,
                           jpeg_upsampler, jvirt_barray_control,
                           jvirt_barray_ptr, jvirt_sarray_control,
                           jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr,
                           JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
                           JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
                           JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
                           J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE};pub use crate::jmorecfg_h::{boolean, CENTERJSAMPLE, EXT_BGRX_BLUE,
                            EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE, EXT_BGRX_RED,
                            EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE,
                            EXT_BGR_RED, EXT_RGBX_BLUE, EXT_RGBX_GREEN,
                            EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_RGB_BLUE,
                            EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED,
                            EXT_XBGR_BLUE, EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE,
                            EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN,
                            EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, FALSE, INT16,
                            JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAXJSAMPLE,
                            RGB_BLUE_5, RGB_GREEN_5, RGB_PIXELSIZE_5,
                            RGB_RED_5, TRUE, UINT16, UINT8};pub use crate::stddef_h::{size_t, NULL};pub use crate::jdmrg565_c::{h2v1_merged_upsample_565D_be,
                            h2v1_merged_upsample_565D_le,
                            h2v1_merged_upsample_565_be,
                            h2v1_merged_upsample_565_le,
                            h2v2_merged_upsample_565D_be,
                            h2v2_merged_upsample_565D_le,
                            h2v2_merged_upsample_565_be,
                            h2v2_merged_upsample_565_le};pub use crate::jdmrgext_c::{extbgr_h2v1_merged_upsample_internal,
                            extbgr_h2v2_merged_upsample_internal,
                            extbgrx_h2v1_merged_upsample_internal,
                            extbgrx_h2v2_merged_upsample_internal,
                            extrgb_h2v1_merged_upsample_internal,
                            extrgb_h2v2_merged_upsample_internal,
                            extrgbx_h2v1_merged_upsample_internal,
                            extrgbx_h2v2_merged_upsample_internal,
                            extxbgr_h2v1_merged_upsample_internal,
                            extxbgr_h2v2_merged_upsample_internal,
                            extxrgb_h2v1_merged_upsample_internal,
                            extxrgb_h2v2_merged_upsample_internal,
                            h2v1_merged_upsample_internal,
                            h2v2_merged_upsample_internal};

pub type my_upsample_ptr = *mut my_upsampler;

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

pub const SCALEBITS: c_int = 16i32;
/* speediest right-shift on some machines */

pub const ONE_HALF: JLONG = (1i64) << SCALEBITS - 1i32;
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
    
    
    (*upsample).Cr_r_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1i32) as c_ulong *
    ::std::mem::size_of::<c_int>() as c_ulong,
    ) as *mut c_int;
    (*upsample).Cb_b_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1i32) as c_ulong *
    ::std::mem::size_of::<c_int>() as c_ulong,
    ) as *mut c_int;
    (*upsample).Cr_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1i32) as c_ulong *
    ::std::mem::size_of::<JLONG>() as c_ulong,
    ) as *mut JLONG;
    (*upsample).Cb_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1i32) as c_ulong *
    ::std::mem::size_of::<JLONG>() as c_ulong,
    ) as *mut JLONG;
    
     let mut i:   c_int =  0i32; let mut x:   JLONG =
     -CENTERJSAMPLE as JLONG;
    while i <= MAXJSAMPLE {
        /* i is the actual input pixel value, in the range 0..MAXJSAMPLE */
        /* The Cb or Cr value we are thinking of is x = i - CENTERJSAMPLE */
        /* Cr=>R value is nearest int to 1.40200 * x */
        *(*upsample).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG
                * x
                + ((1i64) << 16i32 - 1i32)
                >> 16i32) as c_int;
        /* Cb=>B value is nearest int to 1.77200 * x */
        *(*upsample).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG
                * x
                + ((1i64) << 16i32 - 1i32)
                >> 16i32) as c_int;
        /* Cr=>G value is scaled-up -0.71414 * x */
        *(*upsample).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * (1i64 << SCALEBITS) as c_double + 0.5f64)
                as JLONG)
                * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
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
    /* Mark the spare buffer empty */
    (*upsample).spare_full = FALSE;
    /* Initialize total-height counter for detecting bottom of image */
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
)
/* 2:1 vertical sampling case: may need a spare row. */
{
     let mut num_rows:  JDIMENSION =  0;let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr; /* number of rows returned to caller */
    
    
    if (*upsample).spare_full != 0 {
        /* If we have a spare row saved from a previous cycle, just return it. */
        let mut size: JDIMENSION = (*upsample).out_row_width;
        if  (*cinfo).out_color_space
            ==  JCS_RGB565
        {
            size =  (*cinfo).output_width * 2u32
        }
        jcopy_sample_rows(
            &mut (*upsample).spare_row,
            0i32,
            output_buf.offset(*out_row_ctr as isize),
            0i32,
            1i32,
            size,
        );
        num_rows = 1u32;
        (*upsample).spare_full = FALSE
    } else {
        /* Figure number of rows to return to caller. */
         let mut work_ptrs:  [JSAMPROW; 2] =
     [::std::ptr::null_mut::< JSAMPLE>(); 2];num_rows = 2u32;
        /* Not more than the distance to the end of the image. */
        if num_rows > (*upsample).rows_to_go {
            num_rows = (*upsample).rows_to_go
        }
        /* And not more than what the client can accept: */
        out_rows_avail -=  *out_row_ctr;
        if num_rows > out_rows_avail {
            num_rows = out_rows_avail
        }
        /* Create output pointer array for upsampler. */
        work_ptrs[0] = *output_buf.offset(*out_row_ctr as isize);
        if num_rows > 1u32 {
            work_ptrs[1] =
                *output_buf.offset((*out_row_ctr + 1u32) as isize)
        } else {
            work_ptrs[1] = (*upsample).spare_row;
            (*upsample).spare_full = TRUE
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
    *out_row_ctr = *out_row_ctr + num_rows;
    (*upsample).rows_to_go = (*upsample).rows_to_go - num_rows;
    /* When the buffer is emptied, declare this input row group consumed */
    if (*upsample).spare_full == 0 {
        *in_row_group_ctr = *in_row_group_ctr + 1
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
    *out_row_ctr = *out_row_ctr + 1;
    *in_row_group_ctr = *in_row_group_ctr + 1;
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
    match  (*cinfo).out_color_space {
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
    match  (*cinfo).out_color_space {
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
    0x8020ai64,
    0xc040e06i64,
    0x30b0109i64,
    0xf070d05i64,
];
/* Include inline routines for RGB565 conversion */
#[inline(always)]

unsafe extern "C" fn is_big_endian() -> boolean {
     let mut test_value:  c_int =  1i32;
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
    if is_big_endian() != 0 {
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
    if is_big_endian() != 0 {
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
    if is_big_endian() != 0 {
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
    if is_big_endian() != 0 {
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
     
     let mut upsample:   my_upsample_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
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
    (*upsample).out_row_width =  (*cinfo)
        .output_width * (*cinfo).out_color_components as c_uint;
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
        if super::simd::x86_64::jsimd::jsimd_can_h2v2_merged_upsample() != 0 {
            (*upsample).upmethod = Some(
                super::simd::x86_64::jsimd::jsimd_h2v2_merged_upsample
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
        if  (*cinfo).out_color_space
            ==  JCS_RGB565
        {
            if  (*cinfo).dither_mode
                !=  JDITHER_NONE
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
        /* Allocate a spare row buffer */
        (*upsample).spare_row = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*upsample).out_row_width as c_ulong *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
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
        if super::simd::x86_64::jsimd::jsimd_can_h2v1_merged_upsample() != 0 {
            (*upsample).upmethod = Some(
                super::simd::x86_64::jsimd::jsimd_h2v1_merged_upsample
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
        if  (*cinfo).out_color_space
            ==  JCS_RGB565
        {
            if  (*cinfo).dither_mode
                !=  JDITHER_NONE
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
        /* No spare row needed */
        (*upsample).spare_row = NULL as JSAMPROW
    }
    build_ycc_rgb_table(cinfo);
}
/* UPSAMPLE_MERGING_SUPPORTED */
