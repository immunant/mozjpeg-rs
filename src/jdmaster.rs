pub use crate::jerror::C2RustUnnamed_4;
pub use crate::jpeglib_h::C2RustUnnamed_3;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
/*
 * jdmaster.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1995, Thomas G. Lane.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains the master control structure for the JPEG decompressor.
 */
/* Private state */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_decomp_master {
    pub pub_0: jpeg_decomp_master,
    pub pass_number: c_int,
    pub using_merged_upsample: boolean,
    pub quantizer_1pass: *mut jpeg_color_quantizer,
    pub quantizer_2pass: *mut jpeg_color_quantizer,
    pub custom_idct_selector: jpeg_idct_method_selector,
}
pub use crate::jmorecfg_h::boolean;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_idct_method_selector;
use libc;
pub type my_master_ptr = *mut my_decomp_master;
#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg-rs/mozjpeg-c/jmorecfg.h"]
pub mod jmorecfg_h {

    use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
    use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
    use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
    use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
    use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
    use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
    use crate::jmorecfg_h::RGB_PIXELSIZE;
    use libc::c_int;
    pub static mut rgb_pixelsize: [c_int; 17] = [
        -1i32,
        -1i32,
        RGB_PIXELSIZE,
        -1i32,
        -1i32,
        -1i32,
        EXT_RGB_PIXELSIZE,
        EXT_RGBX_PIXELSIZE,
        EXT_BGR_PIXELSIZE,
        EXT_BGRX_PIXELSIZE,
        EXT_XBGR_PIXELSIZE,
        EXT_XRGB_PIXELSIZE,
        EXT_RGBX_PIXELSIZE,
        EXT_BGRX_PIXELSIZE,
        EXT_XBGR_PIXELSIZE,
        EXT_XRGB_PIXELSIZE,
        -1i32,
    ];

}

pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::jmorecfg_h::CENTERJSAMPLE;
pub use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_PIXELSIZE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jdiv_round_up;
pub use crate::jpegint_h::jinit_1pass_quantizer;
pub use crate::jpegint_h::jinit_2pass_quantizer;
pub use crate::jpegint_h::jinit_color_deconverter;
pub use crate::jpegint_h::jinit_d_coef_controller;
pub use crate::jpegint_h::jinit_d_main_controller;
pub use crate::jpegint_h::jinit_d_post_controller;
pub use crate::jpegint_h::jinit_huff_decoder;
pub use crate::jpegint_h::jinit_inverse_dct;
pub use crate::jpegint_h::jinit_merged_upsampler;
pub use crate::jpegint_h::jinit_phuff_decoder;
pub use crate::jpegint_h::jinit_upsampler;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::DSTATE_BUFIMAGE;
pub use crate::jpegint_h::DSTATE_READY;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_idct_method;
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
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
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
use crate::jsimd::jsimd_can_ycc_rgb;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use jmorecfg_h::rgb_pixelsize;
/*
 * jdmaster.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2002-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2011, 2016, D. R. Commander.
 * Copyright (C) 2013, Linaro Limited.
 * Copyright (C) 2015, Google, Inc.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains master control logic for the JPEG decompressor.
 * These routines are concerned with selecting the modules to be executed
 * and with determining the number of passes and the work to be done in each
 * pass.
 */
/*
 * Determine whether merged upsample/color conversion should be used.
 * CRUCIAL: this must match the actual capabilities of jdmerge.c!
 */
unsafe extern "C" fn use_merged_upsample(mut cinfo: j_decompress_ptr) -> boolean {
    if 0 != (*cinfo).do_fancy_upsampling || 0 != (*cinfo).CCIR601_sampling {
        return FALSE;
    }
    if (*cinfo).jpeg_color_space as c_uint != JCS_YCbCr as c_int as c_uint
        || (*cinfo).num_components != 3i32
        || (*cinfo).out_color_space as c_uint != JCS_RGB as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_RGB565 as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_RGB as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_RGBX as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_BGR as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_BGRX as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_XBGR as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_XRGB as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_RGBA as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_BGRA as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_ABGR as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_EXT_ARGB as c_int as c_uint
    {
        return FALSE;
    }
    if (*cinfo).out_color_space as c_uint == JCS_RGB565 as c_int as c_uint
        && (*cinfo).out_color_components != 3i32
        || (*cinfo).out_color_space as c_uint != JCS_RGB565 as c_int as c_uint
            && (*cinfo).out_color_components != rgb_pixelsize[(*cinfo).out_color_space as usize]
    {
        return FALSE;
    }
    if (*(*cinfo).comp_info.offset(0isize)).h_samp_factor != 2i32
        || (*(*cinfo).comp_info.offset(1isize)).h_samp_factor != 1i32
        || (*(*cinfo).comp_info.offset(2isize)).h_samp_factor != 1i32
        || (*(*cinfo).comp_info.offset(0isize)).v_samp_factor > 2i32
        || (*(*cinfo).comp_info.offset(1isize)).v_samp_factor != 1i32
        || (*(*cinfo).comp_info.offset(2isize)).v_samp_factor != 1i32
    {
        return FALSE;
    }
    if (*(*cinfo).comp_info.offset(0isize)).DCT_scaled_size != (*cinfo).min_DCT_scaled_size
        || (*(*cinfo).comp_info.offset(1isize)).DCT_scaled_size != (*cinfo).min_DCT_scaled_size
        || (*(*cinfo).comp_info.offset(2isize)).DCT_scaled_size != (*cinfo).min_DCT_scaled_size
    {
        return FALSE;
    }
    if 0 == jsimd_can_h2v2_merged_upsample()
        && 0 == jsimd_can_h2v1_merged_upsample()
        && 0 != jsimd_can_ycc_rgb()
        && (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint
        && ((*cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint
            || (*cinfo).out_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                && (*cinfo).out_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint)
    {
        return FALSE;
    }
    return TRUE;
}
/*
 * Compute output image dimensions and related values.
 * NOTE: this is exported for possible use by application.
 * Hence it mustn't do anything that can't be done twice.
 */
unsafe extern "C" fn jpeg_core_output_dimensions(mut cinfo: j_decompress_ptr) {
    let mut ci: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint) <= (*cinfo).scale_denom {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 1i32;
        (*cinfo).min_DCT_scaled_size = 1i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(2i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 2i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 2i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 2i32;
        (*cinfo).min_DCT_scaled_size = 2i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(3i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 3i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 3i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 3i32;
        (*cinfo).min_DCT_scaled_size = 3i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(4i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 4i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 4i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 4i32;
        (*cinfo).min_DCT_scaled_size = 4i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(5i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 5i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 5i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 5i32;
        (*cinfo).min_DCT_scaled_size = 5i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(6i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 6i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 6i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 6i32;
        (*cinfo).min_DCT_scaled_size = 6i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(7i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 7i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 7i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 7i32;
        (*cinfo).min_DCT_scaled_size = 7i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(8i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 8i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 8i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 8i32;
        (*cinfo).min_DCT_scaled_size = 8i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(9i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 9i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 9i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 9i32;
        (*cinfo).min_DCT_scaled_size = 9i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(10i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 10i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 10i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 10i32;
        (*cinfo).min_DCT_scaled_size = 10i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(11i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 11i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 11i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 11i32;
        (*cinfo).min_DCT_scaled_size = 11i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(12i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 12i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 12i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 12i32;
        (*cinfo).min_DCT_scaled_size = 12i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(13i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 13i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 13i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 13i32;
        (*cinfo).min_DCT_scaled_size = 13i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(14i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 14i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 14i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 14i32;
        (*cinfo).min_DCT_scaled_size = 14i32
    } else if (*cinfo).scale_num.wrapping_mul(DCTSIZE as c_uint)
        <= (*cinfo).scale_denom.wrapping_mul(15i32 as c_uint)
    {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 15i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 15i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 15i32;
        (*cinfo).min_DCT_scaled_size = 15i32
    } else {
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as c_long * 16i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as c_long * 16i64, DCTSIZE as c_long) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 16i32;
        (*cinfo).min_DCT_scaled_size = 16i32
    }
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).DCT_scaled_size = (*cinfo).min_DCT_scaled_size;
        (*compptr).DCT_scaled_size = (*cinfo).min_DCT_scaled_size;
        ci += 1;
        compptr = compptr.offset(1isize)
    }
}
/* !IDCT_SCALING_SUPPORTED */
/* IDCT_SCALING_SUPPORTED */
/*
 * Compute output image dimensions and related values.
 * NOTE: this is exported for possible use by application.
 * Hence it mustn't do anything that can't be done twice.
 * Also note that it may be called before the master module is initialized!
 */
/* Return value is one of: */
/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */
/* Reached start of new scan */
/* Reached end of image */
/* Completed one iMCU row */
/* Completed last iMCU row of a scan */
/* Precalculate output dimensions for current decompression parameters. */
#[no_mangle]
pub unsafe extern "C" fn jpeg_calc_output_dimensions(mut cinfo: j_decompress_ptr) {
    let mut ci: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if (*cinfo).global_state != DSTATE_READY {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    jpeg_core_output_dimensions(cinfo);
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        let mut ssize: c_int = (*cinfo).min_DCT_scaled_size;
        while ssize < DCTSIZE
            && (*cinfo).max_h_samp_factor * (*cinfo).min_DCT_scaled_size
                % ((*compptr).h_samp_factor * ssize * 2i32)
                == 0i32
            && (*cinfo).max_v_samp_factor * (*cinfo).min_DCT_scaled_size
                % ((*compptr).v_samp_factor * ssize * 2i32)
                == 0i32
        {
            ssize = ssize * 2i32
        }
        (*compptr).DCT_scaled_size = ssize;
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).downsampled_width = jdiv_round_up(
            (*cinfo).image_width as c_long
                * ((*compptr).h_samp_factor * (*compptr).DCT_scaled_size) as c_long,
            ((*cinfo).max_h_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*compptr).downsampled_height = jdiv_round_up(
            (*cinfo).image_height as c_long
                * ((*compptr).v_samp_factor * (*compptr).DCT_scaled_size) as c_long,
            ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    match (*cinfo).out_color_space as c_uint {
        1 => (*cinfo).out_color_components = 1i32,
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            (*cinfo).out_color_components = rgb_pixelsize[(*cinfo).out_color_space as usize]
        }
        3 | 16 => (*cinfo).out_color_components = 3i32,
        4 | 5 => (*cinfo).out_color_components = 4i32,
        _ => (*cinfo).out_color_components = (*cinfo).num_components,
    }
    (*cinfo).output_components = if 0 != (*cinfo).quantize_colors {
        1i32
    } else {
        (*cinfo).out_color_components
    };
    if 0 != use_merged_upsample(cinfo) {
        (*cinfo).rec_outbuf_height = (*cinfo).max_v_samp_factor
    } else {
        (*cinfo).rec_outbuf_height = 1i32
    };
}
/*
 * Several decompression processes need to range-limit values to the range
 * 0..MAXJSAMPLE; the input value may fall somewhat outside this range
 * due to noise introduced by quantization, roundoff error, etc.  These
 * processes are inner loops and need to be as fast as possible.  On most
 * machines, particularly CPUs with pipelines or instruction prefetch,
 * a (subscript-check-less) C table lookup
 *              x = sample_range_limit[x];
 * is faster than explicit tests
 *              if (x < 0)  x = 0;
 *              else if (x > MAXJSAMPLE)  x = MAXJSAMPLE;
 * These processes all use a common table prepared by the routine below.
 *
 * For most steps we can mathematically guarantee that the initial value
 * of x is within MAXJSAMPLE+1 of the legal range, so a table running from
 * -(MAXJSAMPLE+1) to 2*MAXJSAMPLE+1 is sufficient.  But for the initial
 * limiting step (just after the IDCT), a wildly out-of-range value is
 * possible if the input data is corrupt.  To avoid any chance of indexing
 * off the end of memory and getting a bad-pointer trap, we perform the
 * post-IDCT limiting thus:
 *              x = range_limit[x & MASK];
 * where MASK is 2 bits wider than legal sample data, ie 10 bits for 8-bit
 * samples.  Under normal circumstances this is more than enough range and
 * a correct output will be generated; with bogus input data the mask will
 * cause wraparound, and we will safely generate a bogus-but-in-range output.
 * For the post-IDCT step, we want to convert the data from signed to unsigned
 * representation by adding CENTERJSAMPLE at the same time that we limit it.
 * So the post-IDCT limiting table ends up looking like this:
 *   CENTERJSAMPLE,CENTERJSAMPLE+1,...,MAXJSAMPLE,
 *   MAXJSAMPLE (repeat 2*(MAXJSAMPLE+1)-CENTERJSAMPLE times),
 *   0          (repeat 2*(MAXJSAMPLE+1)-CENTERJSAMPLE times),
 *   0,1,...,CENTERJSAMPLE-1
 * Negative inputs select values from the upper half of the table after
 * masking.
 *
 * We can save some space by overlapping the start of the post-IDCT table
 * with the simpler range limiting table.  The post-IDCT table begins at
 * sample_range_limit + CENTERJSAMPLE.
 */
unsafe extern "C" fn prepare_range_limit_table(mut cinfo: j_decompress_ptr) {
    let mut table: *mut JSAMPLE = 0 as *mut JSAMPLE;
    let mut i: c_int = 0;
    table = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((5i32 * (MAXJSAMPLE + 1i32) + CENTERJSAMPLE) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
    ) as *mut JSAMPLE;
    table = table.offset((MAXJSAMPLE + 1i32) as isize);
    (*cinfo).sample_range_limit = table;
    memset(
        table.offset(-((255i32 + 1i32) as isize)) as *mut c_void,
        0i32,
        ((255i32 + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
    );
    i = 0i32;
    while i <= MAXJSAMPLE {
        *table.offset(i as isize) = i as JSAMPLE;
        i += 1
    }
    table = table.offset(CENTERJSAMPLE as isize);
    i = CENTERJSAMPLE;
    while i < 2i32 * (MAXJSAMPLE + 1i32) {
        *table.offset(i as isize) = MAXJSAMPLE as JSAMPLE;
        i += 1
    }
    memset(
        table.offset((2i32 * (255i32 + 1i32)) as isize) as *mut c_void,
        0i32,
        ((2i32 * (255i32 + 1i32) - 128i32) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
    );
    memcpy(
        table.offset((4i32 * (255i32 + 1i32) - 128i32) as isize) as *mut c_void,
        (*cinfo).sample_range_limit as *const c_void,
        (128i32 as c_ulong).wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
    );
}
/*
 * Master selection of decompression modules.
 * This is done once at jpeg_start_decompress time.  We determine
 * which modules will be used and give them appropriate initialization calls.
 * We also initialize the decompressor input side to begin consuming data.
 *
 * Since jpeg_read_header has finished, we know what is in the SOF
 * and (first) SOS markers.  We also have all the application parameter
 * settings.
 */
unsafe extern "C" fn master_selection(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    let mut use_c_buffer: boolean = 0;
    let mut samplesperrow: c_long = 0;
    let mut jd_samplesperrow: JDIMENSION = 0;
    jpeg_calc_output_dimensions(cinfo);
    prepare_range_limit_table(cinfo);
    samplesperrow = (*cinfo).output_width as c_long * (*cinfo).out_color_components as c_long;
    jd_samplesperrow = samplesperrow as JDIMENSION;
    if jd_samplesperrow as c_long != samplesperrow {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*master).pass_number = 0i32;
    (*master).using_merged_upsample = use_merged_upsample(cinfo);
    (*master).quantizer_1pass = NULL as *mut jpeg_color_quantizer;
    (*master).quantizer_2pass = NULL as *mut jpeg_color_quantizer;
    if 0 == (*cinfo).quantize_colors || 0 == (*cinfo).buffered_image {
        (*cinfo).enable_1pass_quant = FALSE;
        (*cinfo).enable_external_quant = FALSE;
        (*cinfo).enable_2pass_quant = FALSE
    }
    if 0 != (*cinfo).quantize_colors {
        if 0 != (*cinfo).raw_data_out {
            (*(*cinfo).err).msg_code = JERR_NOTIMPL as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if (*cinfo).out_color_components != 3i32 {
            (*cinfo).enable_1pass_quant = TRUE;
            (*cinfo).enable_external_quant = FALSE;
            (*cinfo).enable_2pass_quant = FALSE;
            (*cinfo).colormap = NULL as JSAMPARRAY
        } else if !(*cinfo).colormap.is_null() {
            (*cinfo).enable_external_quant = TRUE
        } else if 0 != (*cinfo).two_pass_quantize {
            (*cinfo).enable_2pass_quant = TRUE
        } else {
            (*cinfo).enable_1pass_quant = TRUE
        }
        if 0 != (*cinfo).enable_1pass_quant {
            jinit_1pass_quantizer(cinfo);
            (*master).quantizer_1pass = (*cinfo).cquantize
        }
        if 0 != (*cinfo).enable_2pass_quant || 0 != (*cinfo).enable_external_quant {
            jinit_2pass_quantizer(cinfo);
            (*master).quantizer_2pass = (*cinfo).cquantize
        }
    }
    if 0 == (*cinfo).raw_data_out {
        if 0 != (*master).using_merged_upsample {
            jinit_merged_upsampler(cinfo);
        } else {
            jinit_color_deconverter(cinfo);
            jinit_upsampler(cinfo);
        }
        jinit_d_post_controller(cinfo, (*cinfo).enable_2pass_quant);
    }
    jinit_inverse_dct(cinfo);
    if 0 != (*cinfo).arith_code {
        (*(*cinfo).err).msg_code = JERR_ARITH_NOTIMPL as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    } else if 0 != (*cinfo).progressive_mode {
        jinit_phuff_decoder(cinfo);
    } else {
        jinit_huff_decoder(cinfo);
    }
    use_c_buffer =
        (0 != (*(*cinfo).inputctl).has_multiple_scans || 0 != (*cinfo).buffered_image) as c_int;
    jinit_d_coef_controller(cinfo, use_c_buffer);
    if 0 == (*cinfo).raw_data_out {
        jinit_d_main_controller(cinfo, FALSE);
    }
    (*(*cinfo).mem)
        .realize_virt_arrays
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    (*(*cinfo).inputctl)
        .start_input_pass
        .expect("non-null function pointer")(cinfo);
    (*(*cinfo).master).first_iMCU_col = 0i32 as JDIMENSION;
    (*(*cinfo).master).last_iMCU_col = (*cinfo).MCUs_per_row.wrapping_sub(1i32 as c_uint);
    if !(*cinfo).progress.is_null()
        && 0 == (*cinfo).buffered_image
        && 0 != (*(*cinfo).inputctl).has_multiple_scans
    {
        let mut nscans: c_int = 0;
        if 0 != (*cinfo).progressive_mode {
            nscans = 2i32 + 3i32 * (*cinfo).num_components
        } else {
            nscans = (*cinfo).num_components
        }
        (*(*cinfo).progress).pass_counter = 0i64;
        (*(*cinfo).progress).pass_limit = (*cinfo).total_iMCU_rows as c_long * nscans as c_long;
        (*(*cinfo).progress).completed_passes = 0i32;
        (*(*cinfo).progress).total_passes = if 0 != (*cinfo).enable_2pass_quant {
            3i32
        } else {
            2i32
        };
        (*master).pass_number += 1
    };
}
/* D_MULTISCAN_FILES_SUPPORTED */
/*
 * Per-pass setup.
 * This is called at the beginning of each output pass.  We determine which
 * modules will be active during this pass and give them appropriate
 * start_pass calls.  We also set is_dummy_pass to indicate whether this
 * is a "real" output pass or a dummy pass for color quantization.
 * (In the latter case, jdapistd.c will crank the pass to completion.)
 */
unsafe extern "C" fn prepare_for_output_pass(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    if 0 != (*master).pub_0.is_dummy_pass {
        (*master).pub_0.is_dummy_pass = FALSE;
        (*(*cinfo).cquantize)
            .start_pass
            .expect("non-null function pointer")(cinfo, FALSE);
        (*(*cinfo).post)
            .start_pass
            .expect("non-null function pointer")(cinfo, JBUF_CRANK_DEST);
        (*(*cinfo).main)
            .start_pass
            .expect("non-null function pointer")(cinfo, JBUF_CRANK_DEST);
    } else {
        if 0 != (*cinfo).quantize_colors && (*cinfo).colormap.is_null() {
            if 0 != (*cinfo).two_pass_quantize && 0 != (*cinfo).enable_2pass_quant {
                (*cinfo).cquantize = (*master).quantizer_2pass;
                (*master).pub_0.is_dummy_pass = TRUE
            } else if 0 != (*cinfo).enable_1pass_quant {
                (*cinfo).cquantize = (*master).quantizer_1pass
            } else {
                (*(*cinfo).err).msg_code = JERR_MODE_CHANGE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        (*(*cinfo).idct)
            .start_pass
            .expect("non-null function pointer")(cinfo);
        (*(*cinfo).coef)
            .start_output_pass
            .expect("non-null function pointer")(cinfo);
        if 0 == (*cinfo).raw_data_out {
            if 0 == (*master).using_merged_upsample {
                (*(*cinfo).cconvert)
                    .start_pass
                    .expect("non-null function pointer")(cinfo);
            }
            (*(*cinfo).upsample)
                .start_pass
                .expect("non-null function pointer")(cinfo);
            if 0 != (*cinfo).quantize_colors {
                (*(*cinfo).cquantize)
                    .start_pass
                    .expect("non-null function pointer")(
                    cinfo, (*master).pub_0.is_dummy_pass
                );
            }
            (*(*cinfo).post)
                .start_pass
                .expect("non-null function pointer")(
                cinfo,
                (if 0 != (*master).pub_0.is_dummy_pass {
                    JBUF_SAVE_AND_PASS as c_int
                } else {
                    JBUF_PASS_THRU as c_int
                }) as J_BUF_MODE,
            );
            (*(*cinfo).main)
                .start_pass
                .expect("non-null function pointer")(cinfo, JBUF_PASS_THRU);
        }
    }
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).completed_passes = (*master).pass_number;
        (*(*cinfo).progress).total_passes = (*master).pass_number
            + (if 0 != (*master).pub_0.is_dummy_pass {
                2i32
            } else {
                1i32
            });
        if 0 != (*cinfo).buffered_image && 0 == (*(*cinfo).inputctl).eoi_reached {
            (*(*cinfo).progress).total_passes += if 0 != (*cinfo).enable_2pass_quant {
                2i32
            } else {
                1i32
            }
        }
    };
}
/*
 * Finish up at end of an output pass.
 */
unsafe extern "C" fn finish_output_pass(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    if 0 != (*cinfo).quantize_colors {
        (*(*cinfo).cquantize)
            .finish_pass
            .expect("non-null function pointer")(cinfo);
    }
    (*master).pass_number += 1;
}
/*
 * Switch to a new external colormap between output passes.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_new_colormap(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    if (*cinfo).global_state != DSTATE_BUFIMAGE {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if 0 != (*cinfo).quantize_colors
        && 0 != (*cinfo).enable_external_quant
        && !(*cinfo).colormap.is_null()
    {
        (*cinfo).cquantize = (*master).quantizer_2pass;
        (*(*cinfo).cquantize)
            .new_color_map
            .expect("non-null function pointer")(cinfo);
        (*master).pub_0.is_dummy_pass = FALSE
    } else {
        (*(*cinfo).err).msg_code = JERR_MODE_CHANGE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/* Decompression module initialization routines */
/* D_MULTISCAN_FILES_SUPPORTED */
/*
 * Initialize master decompression control and select active modules.
 * This is performed at the start of jpeg_start_decompress.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_master_decompress(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    (*master).pub_0.prepare_for_output_pass =
        Some(prepare_for_output_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*master).pub_0.finish_output_pass =
        Some(finish_output_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*master).pub_0.is_dummy_pass = FALSE;
    (*master).pub_0.jinit_upsampler_no_alloc = FALSE;
    master_selection(cinfo);
}
