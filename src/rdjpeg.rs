











































































































use libc::{c_uint, c_ulong, c_int, self};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::stddef_h::size_t;pub use crate::jpeglib_h::{j_common_ptr, j_compress_ptr, j_decompress_ptr,
                           jpeg_CreateDecompress, jpeg_c_coef_controller,
                           jpeg_c_main_controller, jpeg_c_prep_controller,
                           jpeg_color_converter, jpeg_color_deconverter,
                           jpeg_color_quantizer, jpeg_common_struct,
                           jpeg_comp_master, jpeg_component_info,
                           jpeg_compress_struct, jpeg_d_coef_controller,
                           jpeg_d_main_controller, jpeg_d_post_controller,
                           jpeg_decomp_master, jpeg_decompress_struct,
                           jpeg_destination_mgr, jpeg_destroy_decompress,
                           jpeg_downsampler, jpeg_entropy_decoder,
                           jpeg_entropy_encoder, jpeg_error_mgr,
                           jpeg_finish_decompress, jpeg_forward_dct,
                           jpeg_input_controller, jpeg_inverse_dct,
                           jpeg_marker_reader, jpeg_marker_struct,
                           jpeg_marker_writer, jpeg_memory_mgr,
                           jpeg_progress_mgr, jpeg_read_header,
                           jpeg_read_scanlines, jpeg_save_markers,
                           jpeg_saved_marker_ptr, jpeg_scan_info,
                           jpeg_source_mgr, jpeg_start_decompress,
                           jpeg_std_error, jpeg_stdio_src, jpeg_upsampler,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
                           JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB,
                           JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
                           JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
                           JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
                           JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
                           JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPEG_APP0, JPEG_COM,
                           JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPROW,
                           J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE};pub use crate::jconfig_h::JPEG_LIB_VERSION;pub use super::cdjpeg::{cjpeg_source_ptr, cjpeg_source_struct};
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
    pub pub_0: super::cdjpeg::cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub dinfo: jpeg_decompress_struct,
    pub jerr: jpeg_error_mgr,
}

pub type jpeg_source_struct = _jpeg_source_struct;

unsafe extern "C" fn get_rows(
    mut _cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
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

unsafe extern "C" fn start_input_jpeg(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
     
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    (*source).dinfo.err = jpeg_std_error(&mut (*source).jerr);
    jpeg_CreateDecompress(
        &mut (*source).dinfo,
        JPEG_LIB_VERSION,
        ::std::mem::size_of::<jpeg_decompress_struct>() as c_ulong,
    );
    jpeg_stdio_src(&mut (*source).dinfo, (*source).pub_0.input_file);
    jpeg_save_markers(
        &mut (*source).dinfo,
        JPEG_COM,
        0xffffu32,
    );
     let mut m:   c_int =  0i32;
    while m < 16i32 {
        jpeg_save_markers(
            &mut (*source).dinfo,
            JPEG_APP0 + m,
            0xffffu32,
        );
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
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        
        (*cinfo)
            .image_width * (*cinfo).input_components as c_uint,
        1u32,
    );
    (*source).pub_0.buffer_height = 1u32;
    (*source).pub_0.get_pixel_rows = Some(
        get_rows
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> JDIMENSION,
    );
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_input_jpeg(
    mut _cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    jpeg_finish_decompress(&mut (*source).dinfo);
    jpeg_destroy_decompress(&mut (*source).dinfo);
}
/*
 * cdjpeg.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2017, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg file.
 *
 * This file contains common declarations for the sample applications
 * cjpeg and djpeg.  It is NOT used by the core JPEG library.
 */
/* define proper options in jconfig.h */
/* cjpeg.c,djpeg.c need to see xxx_SUPPORTED */
/*
 * Object interface for cjpeg's source file decoding modules
 */
/*
 * Object interface for djpeg's output file encoding modules
 */
/* start_output is called after jpeg_start_decompress finishes.
 * The color map will be ready at this time, if one is needed.
 */
/* Emit the specified number of pixel rows from the buffer. */
/* Finish up at the end of the image. */
/* Re-calculate buffer dimensions based on output dimensions (for use with
partial image decompression.)  If this is NULL, then the output format
does not support partial image decompression (BMP and RLE, in particular,
cannot support partial decompression because they use an inversion buffer
to write the image in bottom-up order.) */
/* Target file spec; filled in by djpeg.c after object is created. */
/* Output pixel-row buffer.  Created by module init or start_output.
 * Width is cinfo->output_width * cinfo->output_components;
 * height is buffer_height.
 */
/*
 * cjpeg/djpeg may need to perform extra passes to convert to or from
 * the source/destination file format.  The JPEG library does not know
 * about these passes, but we'd like them to be counted by the progress
 * monitor.  We use an expanded progress monitor object to hold the
 * additional pass count.
 */
/* fields known to JPEG library */
/* extra passes completed */
/* total extra */
/* last printed percentage stored here to avoid multiple printouts */
/* Module selection routines for I/O modules. */
/*
 * The module selection routine for JPEG format input.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_read_jpeg(
    mut cinfo: j_compress_ptr,
) -> super::cdjpeg::cjpeg_source_ptr {
     
     let mut source:   jpeg_source_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<jpeg_source_struct>() as c_ulong,
    ) as jpeg_source_ptr; /* make back link for subroutines */
    (*source).cinfo = cinfo;
    /* Fill in method ptrs, except get_pixel_rows which start_input sets */
    (*source).pub_0.start_input = Some(
        start_input_jpeg
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_jpeg
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    return source as super::cdjpeg::cjpeg_source_ptr;
}
