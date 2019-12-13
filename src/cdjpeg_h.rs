#[repr(C)]
#[derive(Copy, Clone)]
pub struct cjpeg_source_struct {
    pub start_input: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::cdjpeg_h::cjpeg_source_ptr,
        ) -> (),
    >,
    pub get_pixel_rows: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::cdjpeg_h::cjpeg_source_ptr,
        ) -> crate::jmorecfg_h::JDIMENSION,
    >,
    pub finish_input: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::cdjpeg_h::cjpeg_source_ptr,
        ) -> (),
    >,
    pub input_file: *mut crate::stdlib::FILE,
    pub buffer: crate::jpeglib_h::JSAMPARRAY,
    pub buffer_height: crate::jmorecfg_h::JDIMENSION,
    pub marker_list: crate::jpeglib_h::jpeg_saved_marker_ptr,
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
pub type cjpeg_source_ptr = *mut crate::cdjpeg_h::cjpeg_source_struct;
/*
 * cjpeg/djpeg may need to perform extra passes to convert to or from
 * the source/destination file format.  The JPEG library does not know
 * about these passes, but we'd like them to be counted by the progress
 * monitor.  We use an expanded progress monitor object to hold the
 * additional pass count.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdjpeg_progress_mgr {
    pub pub_0: crate::jpeglib_h::jpeg_progress_mgr,
    pub completed_extra_passes: libc::c_int,
    pub total_extra_passes: libc::c_int,
    pub percent_done: libc::c_int,
}
pub type cd_progress_ptr = *mut crate::cdjpeg_h::cdjpeg_progress_mgr;
/*
 * Object interface for djpeg's output file encoding modules
 */
pub type djpeg_dest_ptr = *mut crate::cdjpeg_h::djpeg_dest_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct djpeg_dest_struct {
    pub start_output: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::cdjpeg_h::djpeg_dest_ptr,
        ) -> (),
    >,
    pub put_pixel_rows: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::cdjpeg_h::djpeg_dest_ptr,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
    pub finish_output: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::cdjpeg_h::djpeg_dest_ptr,
        ) -> (),
    >,
    pub calc_buffer_dimensions: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::cdjpeg_h::djpeg_dest_ptr,
        ) -> (),
    >,
    pub output_file: *mut crate::stdlib::FILE,
    pub buffer: crate::jpeglib_h::JSAMPARRAY,
    pub buffer_height: crate::jmorecfg_h::JDIMENSION,
}
