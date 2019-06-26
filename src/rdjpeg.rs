use libc;
use libc::c_int;
use libc::c_uint;
use libc::c_ulong;

pub use crate::cdjpeg::cjpeg_source_ptr;
pub use crate::cdjpeg::cjpeg_source_struct;
pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_CreateDecompress;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_destroy_decompress;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_finish_decompress;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_read_header;
pub use crate::jpeglib_h::jpeg_read_scanlines;
pub use crate::jpeglib_h::jpeg_save_markers;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_start_decompress;
pub use crate::jpeglib_h::jpeg_std_error;
pub use crate::jpeglib_h::jpeg_stdio_src;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_3;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
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
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JPEG_COM;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
/*
 * rdjpeg.c
 *
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 */
/* Private version of data source object */
pub type jpeg_source_ptr = *mut _jpeg_source_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jpeg_source_struct {
    pub pub_0: cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub dinfo: jpeg_decompress_struct,
    pub jerr: jpeg_error_mgr,
}
pub type jpeg_source_struct = _jpeg_source_struct;
unsafe extern "C" fn get_rows(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    return jpeg_read_scanlines(
        &mut (*source).dinfo,
        (*source).pub_0.buffer,
        (*source).pub_0.buffer_height,
    );
}
/*
 * Read the file header; return image size and component count.
 */
unsafe extern "C" fn start_input_jpeg(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut m: c_int = 0;
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    (*source).dinfo.err = jpeg_std_error(&mut (*source).jerr);
    jpeg_CreateDecompress(
        &mut (*source).dinfo,
        JPEG_LIB_VERSION,
        ::std::mem::size_of::<jpeg_decompress_struct>() as c_ulong,
    );
    jpeg_stdio_src(&mut (*source).dinfo, (*source).pub_0.input_file);
    jpeg_save_markers(&mut (*source).dinfo, JPEG_COM, 0xffffi32 as c_uint);
    m = 0i32;
    while m < 16i32 {
        jpeg_save_markers(&mut (*source).dinfo, JPEG_APP0 + m, 0xffffi32 as c_uint);
        m += 1
    }
    jpeg_read_header(&mut (*source).dinfo, TRUE);
    (*source).pub_0.marker_list = (*source).dinfo.marker_list;
    (*source).dinfo.raw_data_out = FALSE;
    jpeg_start_decompress(&mut (*source).dinfo);
    (*cinfo).in_color_space = (*source).dinfo.out_color_space;
    (*cinfo).input_components = (*source).dinfo.output_components;
    (*cinfo).data_precision = (*source).dinfo.data_precision;
    (*cinfo).image_width = (*source).dinfo.image_width;
    (*cinfo).image_height = (*source).dinfo.image_height;
    (*cinfo).raw_data_in = FALSE;
    (*source).pub_0.buffer = (*(*cinfo).mem)
        .alloc_sarray
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (*cinfo)
            .image_width
            .wrapping_mul((*cinfo).input_components as c_uint),
        1i32 as JDIMENSION,
    );
    (*source).pub_0.buffer_height = 1i32 as JDIMENSION;
    (*source).pub_0.get_pixel_rows = Some(
        get_rows as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION,
    );
}
/*
 * Finish up at the end of the file.
 */
unsafe extern "C" fn finish_input_jpeg(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    jpeg_finish_decompress(&mut (*source).dinfo);
    jpeg_destroy_decompress(&mut (*source).dinfo);
}
/*
 * The module selection routine for JPEG format input.
 */
/* Module selection routines for I/O modules. */
#[no_mangle]
pub unsafe extern "C" fn jinit_read_jpeg(mut cinfo: j_compress_ptr) -> cjpeg_source_ptr {
    let mut source: jpeg_source_ptr = 0 as *mut _jpeg_source_struct;
    source = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<jpeg_source_struct>() as c_ulong,
    ) as jpeg_source_ptr;
    (*source).cinfo = cinfo;
    (*source).pub_0.start_input = Some(
        start_input_jpeg as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_jpeg as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> (),
    );
    return source as cjpeg_source_ptr;
}
