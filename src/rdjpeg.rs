use libc;

/* Define to `unsigned int' if <sys/types.h> does not define. */

/* #undef size_t */

/* Define to empty if `const' does not conform to ANSI C. */

/* #undef const */

/* #undef __CHAR_UNSIGNED__ */

/* Define to 1 if type `char' is unsigned and you are not using gcc.  */

/* Define if your (broken) compiler shifts signed values as if they were
unsigned. */

/* #undef RIGHT_SHIFT_IS_UNSIGNED */

/* Compiler does not support pointers to undefined structures. */

/* #undef INCOMPLETE_TYPES_BROKEN */

/* Define to 1 if the system has the type `unsigned short'. */

/* Define to 1 if the system has the type `unsigned char'. */

/* Define if you have BSD-like bzero and bcopy in <strings.h> rather than
memset/memcpy in <string.h>. */

/* #undef NEED_BSD_STRINGS */

/* Define if you need to include <sys/types.h> to get size_t. */

/* Define to 1 if you have the <stdlib.h> header file. */

/* Define to 1 if you have the <stddef.h> header file. */

/* Define to 1 if you have the <locale.h> header file. */

/* use 8 or 12 */

/*
 * Define BITS_IN_JSAMPLE as either
 *   8   for 8-bit sample values (the usual setting)
 *   12  for 12-bit sample values
 * Only 8 and 12 are legal data precisions for lossy JPEG according to the
 * JPEG standard, and the IJG code does not support anything else!
 * We do not support run-time selection of data precision, sorry.
 */

/* Use accelerated SIMD routines. */

/* Support in-memory source/destination managers */

/* Support arithmetic decoding */

/* #undef D_ARITH_CODING_SUPPORTED */

/* Support arithmetic encoding */

/* #undef C_ARITH_CODING_SUPPORTED */

/* libjpeg-turbo version in integer form */

/* libjpeg-turbo version */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

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
pub use crate::jpeglib_h::C2RustUnnamed_2;
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
pub use crate::src::cdjpeg::cjpeg_source_ptr;
pub use crate::src::cdjpeg::cjpeg_source_struct;
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
    pub pub_0: crate::src::cdjpeg::cjpeg_source_struct,
    pub cinfo: crate::jpeglib_h::j_compress_ptr,
    pub dinfo: crate::jpeglib_h::jpeg_decompress_struct,
    pub jerr: crate::jpeglib_h::jpeg_error_mgr,
}

pub type jpeg_source_struct = _jpeg_source_struct;

unsafe extern "C" fn get_rows(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::src::cdjpeg::cjpeg_source_ptr,
) -> crate::jmorecfg_h::JDIMENSION {
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    return crate::jpeglib_h::jpeg_read_scanlines(
        &mut (*source).dinfo,
        (*source).pub_0.buffer,
        (*source).pub_0.buffer_height,
    );
}
/*
 * Read the file header; return image size and component count.
 */

unsafe extern "C" fn start_input_jpeg(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::src::cdjpeg::cjpeg_source_ptr,
) {
    let mut m: libc::c_int = 0;
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    (*source).dinfo.err = crate::jpeglib_h::jpeg_std_error(&mut (*source).jerr);
    crate::jpeglib_h::jpeg_CreateDecompress(
        &mut (*source).dinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_decompress_struct>() as libc::c_ulong,
    );
    crate::jpeglib_h::jpeg_stdio_src(&mut (*source).dinfo, (*source).pub_0.input_file);
    crate::jpeglib_h::jpeg_save_markers(
        &mut (*source).dinfo,
        crate::jpeglib_h::JPEG_COM,
        0xffffi32 as libc::c_uint,
    );
    m = 0i32;
    while m < 16i32 {
        crate::jpeglib_h::jpeg_save_markers(
            &mut (*source).dinfo,
            crate::jpeglib_h::JPEG_APP0 + m,
            0xffffi32 as libc::c_uint,
        );
        m += 1
    }
    crate::jpeglib_h::jpeg_read_header(&mut (*source).dinfo, crate::jmorecfg_h::TRUE);
    (*source).pub_0.marker_list = (*source).dinfo.marker_list;
    (*source).dinfo.raw_data_out = crate::jmorecfg_h::FALSE;
    crate::jpeglib_h::jpeg_start_decompress(&mut (*source).dinfo);
    (*cinfo).in_color_space = (*source).dinfo.out_color_space;
    (*cinfo).input_components = (*source).dinfo.output_components;
    (*cinfo).data_precision = (*source).dinfo.data_precision;
    (*cinfo).image_width = (*source).dinfo.image_width;
    (*cinfo).image_height = (*source).dinfo.image_height;
    (*cinfo).raw_data_in = crate::jmorecfg_h::FALSE;
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        
        (*cinfo)
            .image_width * (*cinfo).input_components as libc::c_uint,
        1i32 as crate::jmorecfg_h::JDIMENSION,
    );
    (*source).pub_0.buffer_height = 1i32 as crate::jmorecfg_h::JDIMENSION;
    (*source).pub_0.get_pixel_rows = Some(
        get_rows
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::src::cdjpeg::cjpeg_source_ptr,
            ) -> crate::jmorecfg_h::JDIMENSION,
    );
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_input_jpeg(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::src::cdjpeg::cjpeg_source_ptr,
) {
    let mut source: jpeg_source_ptr = sinfo as jpeg_source_ptr;
    crate::jpeglib_h::jpeg_finish_decompress(&mut (*source).dinfo);
    crate::jpeglib_h::jpeg_destroy_decompress(&mut (*source).dinfo);
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
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
) -> crate::src::cdjpeg::cjpeg_source_ptr {
    let mut source: jpeg_source_ptr = ::std::ptr::null_mut::< _jpeg_source_struct>();
    /* Create module interface object */
    source = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<jpeg_source_struct>() as libc::c_ulong,
    ) as jpeg_source_ptr; /* make back link for subroutines */
    (*source).cinfo = cinfo;
    /* Fill in method ptrs, except get_pixel_rows which start_input sets */
    (*source).pub_0.start_input = Some(
        start_input_jpeg
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::src::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_jpeg
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::src::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    return source as crate::src::cdjpeg::cjpeg_source_ptr;
}
