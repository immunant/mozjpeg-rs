use libc::c_uint;use libc::c_ulong;use libc::c_void;use libc::c_long;use libc::c_int;pub use crate::jmorecfg_h::boolean;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_idct_method_selector;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use super::jerror::C2RustUnnamed_3;
use libc;

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:22"]
pub mod jmorecfg_h {

    use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;use libc::c_int;use crate::jmorecfg_h::RGB_PIXELSIZE;pub static mut rgb_pixelsize: [c_int; 17] = [
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
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_idct_method;
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
pub use jmorecfg_h::rgb_pixelsize;
pub use super::jerror::JERR_ARITH_NOTIMPL;
pub use super::jerror::JERR_BAD_ALIGN_TYPE;
pub use super::jerror::JERR_BAD_ALLOC_CHUNK;
pub use super::jerror::JERR_BAD_BUFFER_MODE;
pub use super::jerror::JERR_BAD_COMPONENT_ID;
pub use super::jerror::JERR_BAD_CROP_SPEC;
pub use super::jerror::JERR_BAD_DCTSIZE;
pub use super::jerror::JERR_BAD_DCT_COEF;
pub use super::jerror::JERR_BAD_HUFF_TABLE;
pub use super::jerror::JERR_BAD_IN_COLORSPACE;
pub use super::jerror::JERR_BAD_J_COLORSPACE;
pub use super::jerror::JERR_BAD_LENGTH;
pub use super::jerror::JERR_BAD_LIB_VERSION;
pub use super::jerror::JERR_BAD_MCU_SIZE;
pub use super::jerror::JERR_BAD_PARAM;
pub use super::jerror::JERR_BAD_PARAM_VALUE;
pub use super::jerror::JERR_BAD_POOL_ID;
pub use super::jerror::JERR_BAD_PRECISION;
pub use super::jerror::JERR_BAD_PROGRESSION;
pub use super::jerror::JERR_BAD_PROG_SCRIPT;
pub use super::jerror::JERR_BAD_SAMPLING;
pub use super::jerror::JERR_BAD_SCAN_SCRIPT;
pub use super::jerror::JERR_BAD_STATE;
pub use super::jerror::JERR_BAD_STRUCT_SIZE;
pub use super::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use super::jerror::JERR_BUFFER_SIZE;
pub use super::jerror::JERR_CANT_SUSPEND;
pub use super::jerror::JERR_CCIR601_NOTIMPL;
pub use super::jerror::JERR_COMPONENT_COUNT;
pub use super::jerror::JERR_CONVERSION_NOTIMPL;
pub use super::jerror::JERR_DAC_INDEX;
pub use super::jerror::JERR_DAC_VALUE;
pub use super::jerror::JERR_DHT_INDEX;
pub use super::jerror::JERR_DQT_INDEX;
pub use super::jerror::JERR_EMPTY_IMAGE;
pub use super::jerror::JERR_EMS_READ;
pub use super::jerror::JERR_EMS_WRITE;
pub use super::jerror::JERR_EOI_EXPECTED;
pub use super::jerror::JERR_FILE_READ;
pub use super::jerror::JERR_FILE_WRITE;
pub use super::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use super::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use super::jerror::JERR_HUFF_MISSING_CODE;
pub use super::jerror::JERR_IMAGE_TOO_BIG;
pub use super::jerror::JERR_INPUT_EMPTY;
pub use super::jerror::JERR_INPUT_EOF;
pub use super::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use super::jerror::JERR_MISSING_DATA;
pub use super::jerror::JERR_MODE_CHANGE;
pub use super::jerror::JERR_NOTIMPL;
pub use super::jerror::JERR_NOT_COMPILED;
pub use super::jerror::JERR_NO_BACKING_STORE;
pub use super::jerror::JERR_NO_HUFF_TABLE;
pub use super::jerror::JERR_NO_IMAGE;
pub use super::jerror::JERR_NO_QUANT_TABLE;
pub use super::jerror::JERR_NO_SOI;
pub use super::jerror::JERR_OUT_OF_MEMORY;
pub use super::jerror::JERR_QUANT_COMPONENTS;
pub use super::jerror::JERR_QUANT_FEW_COLORS;
pub use super::jerror::JERR_QUANT_MANY_COLORS;
pub use super::jerror::JERR_SOF_DUPLICATE;
pub use super::jerror::JERR_SOF_NO_SOS;
pub use super::jerror::JERR_SOF_UNSUPPORTED;
pub use super::jerror::JERR_SOI_DUPLICATE;
pub use super::jerror::JERR_SOS_NO_SOF;
pub use super::jerror::JERR_TFILE_CREATE;
pub use super::jerror::JERR_TFILE_READ;
pub use super::jerror::JERR_TFILE_SEEK;
pub use super::jerror::JERR_TFILE_WRITE;
pub use super::jerror::JERR_TOO_LITTLE_DATA;
pub use super::jerror::JERR_UNKNOWN_MARKER;
pub use super::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use super::jerror::JERR_VIRTUAL_BUG;
pub use super::jerror::JERR_WIDTH_OVERFLOW;
pub use super::jerror::JERR_XMS_READ;
pub use super::jerror::JERR_XMS_WRITE;
pub use super::jerror::JMSG_COPYRIGHT;
pub use super::jerror::JMSG_LASTMSGCODE;
pub use super::jerror::JMSG_NOMESSAGE;
pub use super::jerror::JMSG_VERSION;
pub use super::jerror::JTRC_16BIT_TABLES;
pub use super::jerror::JTRC_ADOBE;
pub use super::jerror::JTRC_APP0;
pub use super::jerror::JTRC_APP14;
pub use super::jerror::JTRC_DAC;
pub use super::jerror::JTRC_DHT;
pub use super::jerror::JTRC_DQT;
pub use super::jerror::JTRC_DRI;
pub use super::jerror::JTRC_EMS_CLOSE;
pub use super::jerror::JTRC_EMS_OPEN;
pub use super::jerror::JTRC_EOI;
pub use super::jerror::JTRC_HUFFBITS;
pub use super::jerror::JTRC_JFIF;
pub use super::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use super::jerror::JTRC_JFIF_EXTENSION;
pub use super::jerror::JTRC_JFIF_THUMBNAIL;
pub use super::jerror::JTRC_MISC_MARKER;
pub use super::jerror::JTRC_PARMLESS_MARKER;
pub use super::jerror::JTRC_QUANTVALS;
pub use super::jerror::JTRC_QUANT_3_NCOLORS;
pub use super::jerror::JTRC_QUANT_NCOLORS;
pub use super::jerror::JTRC_QUANT_SELECTED;
pub use super::jerror::JTRC_RECOVERY_ACTION;
pub use super::jerror::JTRC_RST;
pub use super::jerror::JTRC_SMOOTH_NOTIMPL;
pub use super::jerror::JTRC_SOF;
pub use super::jerror::JTRC_SOF_COMPONENT;
pub use super::jerror::JTRC_SOI;
pub use super::jerror::JTRC_SOS;
pub use super::jerror::JTRC_SOS_COMPONENT;
pub use super::jerror::JTRC_SOS_PARAMS;
pub use super::jerror::JTRC_TFILE_CLOSE;
pub use super::jerror::JTRC_TFILE_OPEN;
pub use super::jerror::JTRC_THUMB_JPEG;
pub use super::jerror::JTRC_THUMB_PALETTE;
pub use super::jerror::JTRC_THUMB_RGB;
pub use super::jerror::JTRC_UNKNOWN_IDS;
pub use super::jerror::JTRC_XMS_CLOSE;
pub use super::jerror::JTRC_XMS_OPEN;
pub use super::jerror::JWRN_ADOBE_XFORM;
pub use super::jerror::JWRN_BOGUS_ICC;
pub use super::jerror::JWRN_BOGUS_PROGRESSION;
pub use super::jerror::JWRN_EXTRANEOUS_DATA;
pub use super::jerror::JWRN_HIT_MARKER;
pub use super::jerror::JWRN_HUFF_BAD_CODE;
pub use super::jerror::JWRN_JFIF_MAJOR;
pub use super::jerror::JWRN_JPEG_EOF;
pub use super::jerror::JWRN_MUST_RESYNC;
pub use super::jerror::JWRN_NOT_SEQUENTIAL;
pub use super::jerror::JWRN_TOO_MUCH_DATA;
use super::simd::x86_64::jsimd::jsimd_can_h2v1_merged_upsample;
use super::simd::x86_64::jsimd::jsimd_can_h2v2_merged_upsample;
use super::simd::x86_64::jsimd::jsimd_can_ycc_rgb;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
// =============== BEGIN jdmaster_h ================
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

pub type my_master_ptr = *mut my_decomp_master;
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

unsafe extern "C" fn use_merged_upsample(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    /* Merging is the equivalent of plain box-filter upsampling */
    if (*cinfo).do_fancy_upsampling != 0 || (*cinfo).CCIR601_sampling != 0 {
        return FALSE;
    }
    /* jdmerge.c only supports YCC=>RGB and YCC=>RGB565 color conversion */
    if  (*cinfo).jpeg_color_space
        !=  JCS_YCbCr
        || (*cinfo).num_components != 3i32
        ||  (*cinfo).out_color_space
            !=  JCS_RGB
            &&  (*cinfo).out_color_space
                !=  JCS_RGB565
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_RGBX
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_BGR
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_BGRX
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_XBGR
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_XRGB
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_RGBA
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_BGRA
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_ABGR
            &&  (*cinfo).out_color_space
                !=  JCS_EXT_ARGB
    {
        return FALSE;
    }
    if  (*cinfo).out_color_space
        ==  JCS_RGB565
        && (*cinfo).out_color_components != 3i32
        ||  (*cinfo).out_color_space
            !=  JCS_RGB565
            && (*cinfo).out_color_components != rgb_pixelsize[(*cinfo).out_color_space as usize]
    {
        return FALSE;
    }
    /* and it only handles 2h1v or 2h2v sampling ratios */
    if (*(*cinfo).comp_info.offset(0)).h_samp_factor != 2i32
        || (*(*cinfo).comp_info.offset(1)).h_samp_factor != 1i32
        || (*(*cinfo).comp_info.offset(2)).h_samp_factor != 1i32
        || (*(*cinfo).comp_info.offset(0)).v_samp_factor > 2i32
        || (*(*cinfo).comp_info.offset(1)).v_samp_factor != 1i32
        || (*(*cinfo).comp_info.offset(2)).v_samp_factor != 1i32
    {
        return FALSE;
    }
    /* furthermore, it doesn't work if we've scaled the IDCTs differently */
    if (*(*cinfo).comp_info.offset(0)).DCT_scaled_size != (*cinfo).min_DCT_scaled_size
        || (*(*cinfo).comp_info.offset(1)).DCT_scaled_size != (*cinfo).min_DCT_scaled_size
        || (*(*cinfo).comp_info.offset(2)).DCT_scaled_size != (*cinfo).min_DCT_scaled_size
    {
        return FALSE;
    }
    /* If YCbCr-to-RGB color conversion is SIMD-accelerated but merged upsampling
    isn't, then disabling merged upsampling is likely to be faster when
    decompressing YCbCr JPEG images. */
    if super::simd::x86_64::jsimd::jsimd_can_h2v2_merged_upsample() == 0
        && super::simd::x86_64::jsimd::jsimd_can_h2v1_merged_upsample() == 0
        && super::simd::x86_64::jsimd::jsimd_can_ycc_rgb() != 0
        &&  (*cinfo).jpeg_color_space
            ==  JCS_YCbCr
        && ((*cinfo).out_color_space
            ==  JCS_RGB
            ||  (*cinfo).out_color_space
                >=  JCS_EXT_RGB
                &&  (*cinfo).out_color_space
                    <=  JCS_EXT_ARGB)
    {
        return FALSE;
    }
    /* ??? also need to test for upsample-time rescaling, when & if supported */
    return TRUE;
    /* by golly, it'll work... */
}
/*
 * Compute output image dimensions and related values.
 * NOTE: this is exported for possible use by application.
 * Hence it mustn't do anything that can't be done twice.
 */

unsafe extern "C" fn jpeg_core_output_dimensions(mut cinfo: j_decompress_ptr)
/* Do computations that are needed before master selection phase.
 * This function is used for transcoding and full decompression.
 */
{
    
      
    /* Compute actual output image dimensions and DCT scaling choices. */
    if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <= (*cinfo).scale_denom
    {
        /* Provide 1/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 1i32;
        (*cinfo).min_DCT_scaled_size = 1i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 2u32
    {
        /* Provide 2/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 2i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 2i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 2i32;
        (*cinfo).min_DCT_scaled_size = 2i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 3u32
    {
        /* Provide 3/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 3i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 3i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 3i32;
        (*cinfo).min_DCT_scaled_size = 3i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 4u32
    {
        /* Provide 4/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 4i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 4i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 4i32;
        (*cinfo).min_DCT_scaled_size = 4i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 5u32
    {
        /* Provide 5/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 5i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 5i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 5i32;
        (*cinfo).min_DCT_scaled_size = 5i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 6u32
    {
        /* Provide 6/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 6i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 6i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 6i32;
        (*cinfo).min_DCT_scaled_size = 6i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 7u32
    {
        /* Provide 7/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 7i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 7i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 7i32;
        (*cinfo).min_DCT_scaled_size = 7i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 8u32
    {
        /* Provide 8/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 8i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 8i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 8i32;
        (*cinfo).min_DCT_scaled_size = 8i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 9u32
    {
        /* Provide 9/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 9i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 9i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 9i32;
        (*cinfo).min_DCT_scaled_size = 9i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 10u32
    {
        /* Provide 10/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 10i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 10i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 10i32;
        (*cinfo).min_DCT_scaled_size = 10i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 11u32
    {
        /* Provide 11/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 11i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 11i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 11i32;
        (*cinfo).min_DCT_scaled_size = 11i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 12u32
    {
        /* Provide 12/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 12i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 12i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 12i32;
        (*cinfo).min_DCT_scaled_size = 12i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 13u32
    {
        /* Provide 13/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 13i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 13i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 13i32;
        (*cinfo).min_DCT_scaled_size = 13i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 14u32
    {
        /* Provide 14/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 14i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 14i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 14i32;
        (*cinfo).min_DCT_scaled_size = 14i32
    } else if  (*cinfo)
        .scale_num * DCTSIZE as c_uint
        <=  (*cinfo).scale_denom * 15u32
    {
        /* Provide 15/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 15i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 15i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 15i32;
        (*cinfo).min_DCT_scaled_size = 15i32
    } else {
        /* Provide 16/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as c_long * 16i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as c_long * 16i64,
            DCTSIZE as c_long,
        ) as JDIMENSION;
        (*cinfo).min_DCT_scaled_size = 16i32;
        (*cinfo).min_DCT_scaled_size = 16i32
    }
    
     let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).DCT_scaled_size = (*cinfo).min_DCT_scaled_size;
        (*compptr).DCT_scaled_size = (*cinfo).min_DCT_scaled_size;
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* !IDCT_SCALING_SUPPORTED */
    /* IDCT_SCALING_SUPPORTED */
}
/* Return value is one of: */
/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */
/* Reached start of new scan */
/* Reached end of image */
/* Completed one iMCU row */
/* Completed last iMCU row of a scan */
/* Precalculate output dimensions for current decompression parameters. */
/*
 * Compute output image dimensions and related values.
 * NOTE: this is exported for possible use by application.
 * Hence it mustn't do anything that can't be done twice.
 * Also note that it may be called before the master module is initialized!
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_calc_output_dimensions(
    mut cinfo: j_decompress_ptr,
)
/* Do computations that are needed before master selection phase */
{
    
      
    /* Prevent application from calling me at wrong times */
    if (*cinfo).global_state != DSTATE_READY {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Compute core output image dimensions and DCT scaling choices. */
    jpeg_core_output_dimensions(cinfo);
    /* In selecting the actual DCT scaling for each component, we try to
     * scale up the chroma components via IDCT scaling rather than upsampling.
     * This saves time if the upsampler gets to use 1:1 scaling.
     * Note this code adapts subsampling ratios which are powers of 2.
     */
    
     let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
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
        compptr = compptr.offset(1)
    }
    /* Recompute downsampled dimensions of components;
     * application needs to know these if using raw downsampled data.
     */
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Size in samples, after IDCT scaling */
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
        compptr = compptr.offset(1)
    }
    /* !IDCT_SCALING_SUPPORTED */
    /* IDCT_SCALING_SUPPORTED */
    /* Report number of components in selected colorspace. */
    /* Probably this should be in the color conversion module... */
    match  (*cinfo).out_color_space {
        1 => (*cinfo).out_color_components = 1i32,
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            (*cinfo).out_color_components = rgb_pixelsize[(*cinfo).out_color_space as usize]
        }
        3 | 16 => (*cinfo).out_color_components = 3i32,
        4 | 5 => (*cinfo).out_color_components = 4i32,
        _ => {
            /* else must be same colorspace as in file */
            (*cinfo).out_color_components = (*cinfo).num_components
        }
    }
    (*cinfo).output_components = if (*cinfo).quantize_colors != 0 {
        1i32
    } else {
        (*cinfo).out_color_components
    };
    /* See if upsampler will want to emit more than one row at a time */
    if use_merged_upsample(cinfo) != 0 {
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

unsafe extern "C" fn prepare_range_limit_table(mut cinfo: j_decompress_ptr)
/* Allocate and fill in the sample_range_limit table */
{
      
     let mut table:   *mut JSAMPLE =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (5i32 * (MAXJSAMPLE + 1i32) + CENTERJSAMPLE)
            as c_ulong *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
    ) as *mut JSAMPLE;
    table = table.offset((MAXJSAMPLE + 1i32) as isize);
    (*cinfo).sample_range_limit = table;
    /* First segment of "simple" table: limit[x] = 0 for x < 0 */
    memset(
        table.offset(-((255i32 + 1i32) as isize)) as *mut c_void,
        0i32,
        (255i32 + 1i32) as c_ulong *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
    );
     let mut i:   c_int =  0i32; /* Point to where post-IDCT table starts */
    while i <= MAXJSAMPLE {
        *table.offset(i as isize) = i as JSAMPLE;
        i += 1
    }
    table = table.offset(CENTERJSAMPLE as isize);
    /* End of simple table, rest of first half of post-IDCT table */
    i = CENTERJSAMPLE;
    while i < 2i32 * (MAXJSAMPLE + 1i32) {
        *table.offset(i as isize) = MAXJSAMPLE as JSAMPLE;
        i += 1
    }
    /* Second half of post-IDCT table */
    memset(
        table.offset((2i32 * (255i32 + 1i32)) as isize) as *mut c_void,
        0i32,
        (2i32 * (255i32 + 1i32) - 128i32) as c_ulong *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
    );
    memcpy(
        table.offset((4i32 * (255i32 + 1i32) - 128i32) as isize) as *mut c_void,
        (*cinfo).sample_range_limit as *const c_void,
        128u64 *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
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
       let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    
    
    
    /* Initialize dimensions and other stuff */
    jpeg_calc_output_dimensions(cinfo);
    prepare_range_limit_table(cinfo);
    
     let mut samplesperrow:   c_long =
    
        (*cinfo).output_width as c_long * (*cinfo).out_color_components as c_long; let mut jd_samplesperrow:   JDIMENSION =
     samplesperrow as JDIMENSION;
    if jd_samplesperrow as c_long != samplesperrow {
        (*(*cinfo).err).msg_code = super::jerror::JERR_WIDTH_OVERFLOW as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Initialize my private state */
    (*master).pass_number = 0i32;
    (*master).using_merged_upsample = use_merged_upsample(cinfo);
    /* Color quantizer selection */
    (*master).quantizer_1pass =
        NULL as *mut jpeg_color_quantizer;
    (*master).quantizer_2pass =
        NULL as *mut jpeg_color_quantizer;
    /* No mode changes if not using buffered-image mode. */
    if (*cinfo).quantize_colors == 0 || (*cinfo).buffered_image == 0 {
        (*cinfo).enable_1pass_quant = FALSE;
        (*cinfo).enable_external_quant = FALSE;
        (*cinfo).enable_2pass_quant = FALSE
    }
    if (*cinfo).quantize_colors != 0 {
        if (*cinfo).raw_data_out != 0 {
            (*(*cinfo).err).msg_code = super::jerror::JERR_NOTIMPL as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        /* If both quantizers are initialized, the 2-pass one is left active;
         * this is necessary for starting with quantization to an external map.
         */
        if (*cinfo).out_color_components != 3i32 {
            (*cinfo).enable_1pass_quant = TRUE;
            (*cinfo).enable_external_quant = FALSE;
            (*cinfo).enable_2pass_quant = FALSE;
            (*cinfo).colormap = NULL as JSAMPARRAY
        } else if !(*cinfo).colormap.is_null() {
            (*cinfo).enable_external_quant = TRUE
        } else if (*cinfo).two_pass_quantize != 0 {
            (*cinfo).enable_2pass_quant = TRUE
        } else {
            (*cinfo).enable_1pass_quant = TRUE
        }
        if (*cinfo).enable_1pass_quant != 0 {
            jinit_1pass_quantizer(cinfo);
            (*master).quantizer_1pass = (*cinfo).cquantize
        }
        if (*cinfo).enable_2pass_quant != 0 || (*cinfo).enable_external_quant != 0 {
            jinit_2pass_quantizer(cinfo);
            (*master).quantizer_2pass = (*cinfo).cquantize
        }
    }
    /* 2-pass quantizer only works in 3-component color space. */
    /* We use the 2-pass code to map to external colormaps. */
    /* Post-processing: in particular, color conversion first */
    if (*cinfo).raw_data_out == 0 {
        if (*master).using_merged_upsample != 0 {
            jinit_merged_upsampler(cinfo);
        /* does color conversion too */
        } else {
            jinit_color_deconverter(cinfo);
            jinit_upsampler(cinfo);
        }
        jinit_d_post_controller(cinfo, (*cinfo).enable_2pass_quant);
    }
    /* Inverse DCT */
    jinit_inverse_dct(cinfo);
    /* Entropy decoding: either Huffman or arithmetic coding. */
    if (*cinfo).arith_code != 0 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_ARITH_NOTIMPL as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    } else if (*cinfo).progressive_mode != 0 {
        jinit_phuff_decoder(cinfo);
    } else {
        jinit_huff_decoder(cinfo);
    }
     let mut use_c_buffer:   boolean =
     ((*(*cinfo).inputctl).has_multiple_scans != 0 || (*cinfo).buffered_image != 0)
        as c_int;
    jinit_d_coef_controller(cinfo, use_c_buffer);
    if (*cinfo).raw_data_out == 0 {
        jinit_d_main_controller(cinfo, FALSE);
    }
    /* We can now tell the memory manager to allocate virtual arrays. */
    Some(
        (*(*cinfo).mem)
            .realize_virt_arrays
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr);
    /* Initialize input side of decompressor to consume first scan. */
    Some(
        (*(*cinfo).inputctl)
            .start_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Set the first and last iMCU columns to decompress from single-scan images.
     * By default, decompress all of the iMCU columns.
     */
    (*(*cinfo).master).first_iMCU_col = 0u32;
    (*(*cinfo).master).last_iMCU_col =  (*cinfo).MCUs_per_row - 1u32;
    /* If jpeg_start_decompress will read the whole file, initialize
     * progress monitoring appropriately.  The input step is counted
     * as one pass.
     */
    if !(*cinfo).progress.is_null()
        && (*cinfo).buffered_image == 0
        && (*(*cinfo).inputctl).has_multiple_scans != 0
    {
         let mut nscans:  c_int =  0;
        /* Estimate number of scans to set pass_limit. */
        if (*cinfo).progressive_mode != 0 {
            /* Arbitrarily estimate 2 interleaved DC scans + 3 AC scans/component. */
            nscans = 2i32 + 3i32 * (*cinfo).num_components
        } else {
            /* For a nonprogressive multiscan file, estimate 1 scan per component. */
            nscans = (*cinfo).num_components
        }
        (*(*cinfo).progress).pass_counter = 0i64;
        (*(*cinfo).progress).pass_limit =
            (*cinfo).total_iMCU_rows as c_long * nscans as c_long;
        (*(*cinfo).progress).completed_passes = 0i32;
        (*(*cinfo).progress).total_passes = if (*cinfo).enable_2pass_quant != 0 {
            3i32
        } else {
            2i32
        };
        /* Count the input pass as done */
        (*master).pass_number += 1
    };
    /* D_MULTISCAN_FILES_SUPPORTED */
}
/*
 * Per-pass setup.
 * This is called at the beginning of each output pass.  We determine which
 * modules will be active during this pass and give them appropriate
 * start_pass calls.  We also set is_dummy_pass to indicate whether this
 * is a "real" output pass or a dummy pass for color quantization.
 * (In the latter case, jdapistd.c will crank the pass to completion.)
 */

unsafe extern "C" fn prepare_for_output_pass(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    if (*master).pub_0.is_dummy_pass != 0 {
        /* Final pass of 2-pass quantization */
        (*master).pub_0.is_dummy_pass = FALSE;
        Some(
            (*(*cinfo).cquantize)
                .start_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, FALSE);
        Some(
            (*(*cinfo).post)
                .start_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, JBUF_CRANK_DEST);
        Some(
            (*(*cinfo).main)
                .start_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, JBUF_CRANK_DEST);
    /* QUANT_2PASS_SUPPORTED */
    } else {
        if (*cinfo).quantize_colors != 0 && (*cinfo).colormap.is_null() {
            /* Select new quantization method */
            if (*cinfo).two_pass_quantize != 0 && (*cinfo).enable_2pass_quant != 0 {
                (*cinfo).cquantize = (*master).quantizer_2pass;
                (*master).pub_0.is_dummy_pass = TRUE
            } else if (*cinfo).enable_1pass_quant != 0 {
                (*cinfo).cquantize = (*master).quantizer_1pass
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_MODE_CHANGE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
        }
        Some(
            (*(*cinfo).idct)
                .start_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        Some(
            (*(*cinfo).coef)
                .start_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        if (*cinfo).raw_data_out == 0 {
            if (*master).using_merged_upsample == 0 {
                Some(
                    (*(*cinfo).cconvert)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            Some(
                (*(*cinfo).upsample)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            if (*cinfo).quantize_colors != 0 {
                Some(
                    (*(*cinfo).cquantize)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo, (*master).pub_0.is_dummy_pass
                );
            }
            Some(
                (*(*cinfo).post)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                if (*master).pub_0.is_dummy_pass != 0 {
                    JBUF_SAVE_AND_PASS as c_int
                } else {
                    JBUF_PASS_THRU as c_int
                } as J_BUF_MODE,
            );
            Some(
                (*(*cinfo).main)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, JBUF_PASS_THRU
            );
        }
    }
    /* Set up progress monitor's pass info if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).completed_passes = (*master).pass_number;
        (*(*cinfo).progress).total_passes = (*master).pass_number
            + (if (*master).pub_0.is_dummy_pass != 0 {
                2i32
            } else {
                1i32
            });
        /* In buffered-image mode, we assume one more output pass if EOI not
         * yet reached, but no more passes if EOI has been reached.
         */
        if (*cinfo).buffered_image != 0 && (*(*cinfo).inputctl).eoi_reached == 0 {
            (*(*cinfo).progress).total_passes += if (*cinfo).enable_2pass_quant != 0 {
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
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    if (*cinfo).quantize_colors != 0 {
        Some(
            (*(*cinfo).cquantize)
                .finish_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    (*master).pass_number += 1;
}
/*
 * Switch to a new external colormap between output passes.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_new_colormap(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    /* Prevent application from calling me at wrong times */
    if (*cinfo).global_state != DSTATE_BUFIMAGE {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).quantize_colors != 0
        && (*cinfo).enable_external_quant != 0
        && !(*cinfo).colormap.is_null()
    {
        /* Select 2-pass quantizer for external colormap use */
        (*cinfo).cquantize = (*master).quantizer_2pass;
        /* just in case */
        Some(
            (*(*cinfo).cquantize)
                .new_color_map
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*master).pub_0.is_dummy_pass = FALSE
    } else {
        (*(*cinfo).err).msg_code = super::jerror::JERR_MODE_CHANGE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/* Notify quantizer of colormap change */
/* D_MULTISCAN_FILES_SUPPORTED */
/*
 * Initialize master decompression control and select active modules.
 * This is performed at the start of jpeg_start_decompress.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_master_decompress(mut cinfo: j_decompress_ptr) {
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    (*master).pub_0.prepare_for_output_pass = Some(
        prepare_for_output_pass
            as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
    (*master).pub_0.finish_output_pass = Some(
        finish_output_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
    (*master).pub_0.is_dummy_pass = FALSE;
    (*master).pub_0.jinit_upsampler_no_alloc = FALSE;
    master_selection(cinfo);
}
