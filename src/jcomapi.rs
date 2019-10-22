use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::CSTATE_START;
pub use crate::jpegint_h::DSTATE_START;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
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
pub use crate::jpeglib_h::JPOOL_NUMPOOLS;
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
/*
 * jcomapi.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains application interface routines that are used for both
 * compression and decompression.
 */
/*
 * Abort processing of a JPEG compression or decompression operation,
 * but don't destroy the object itself.
 *
 * For this, we merely clean up all the nonpermanent memory pools.
 * Note that temp files (virtual arrays) are not allowed to belong to
 * the permanent pool, so we will be able to close all temp files here.
 * Closing a data source or destination, if necessary, is the application's
 * responsibility.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_abort(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut pool: libc::c_int = 0;
    /* Do nothing if called on a not-initialized or destroyed JPEG object. */
    if (*cinfo).mem.is_null() {
        return;
    }
    /* Releasing pools in reverse order might help avoid fragmentation
     * with some (brain-damaged) malloc libraries.
     */
    pool = crate::jpeglib_h::JPOOL_NUMPOOLS - 1i32;
    while pool > crate::jpeglib_h::JPOOL_PERMANENT {
        Some(
            (*(*cinfo).mem)
                .free_pool
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, pool);
        pool -= 1
    }
    /* Reset overall state for possible reuse of object */
    if (*cinfo).is_decompressor != 0 {
        (*cinfo).global_state = crate::jpegint_h::DSTATE_START;
        /* Try to keep application from accessing now-deleted marker list.
         * A bit kludgy to do it here, but this is the most central place.
         */
        let ref mut fresh0 = (*(cinfo as crate::jpeglib_h::j_decompress_ptr)).marker_list;
        *fresh0 = crate::stddef_h::NULL as crate::jpeglib_h::jpeg_saved_marker_ptr
    } else {
        (*cinfo).global_state = crate::jpegint_h::CSTATE_START
    };
}
/* Main entry points for compression */
/* Replaces jpeg_write_scanlines when writing raw downsampled data. */
/* Write a special marker.  See libjpeg.txt concerning safe usage. */
/* Same, but piecemeal. */
/* Alternate compression function: just write an abbreviated table file */
/* Write ICC profile.  See libjpeg.txt for usage information. */
/* Decompression startup: read start of JPEG datastream to see what's there */
/* Return value is one of: */
/* Suspended due to lack of input data */
/* Found valid image datastream */
/* Found valid table-specs-only datastream */
/* If you pass require_image = TRUE (normal case), you need not check for
 * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
 * JPEG_SUSPENDED is only possible if you use a data source module that can
 * give a suspension return (the stdio source module doesn't).
 */
/* Main entry points for decompression */
/* Replaces jpeg_read_scanlines when reading raw downsampled data. */
/* Additional entry points for buffered-image mode. */
/* Return value is one of: */
/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */
/* Reached start of new scan */
/* Reached end of image */
/* Completed one iMCU row */
/* Completed last iMCU row of a scan */
/* Precalculate output dimensions for current decompression parameters. */
/* Control saving of COM and APPn markers into marker_list. */
/* Install a special processing method for COM or APPn markers. */
/* Read or write raw DCT coefficients --- useful for lossless transcoding. */
/* If you choose to abort compression or decompression before completing
 * jpeg_finish_(de)compress, then you need to clean up to release memory,
 * temporary files, etc.  You can just call jpeg_destroy_(de)compress
 * if you're done with the JPEG object, but if you want to clean it up and
 * reuse it, call this:
 */
/* Generic versions of jpeg_abort and jpeg_destroy that work on either
 * flavor of JPEG object.  These may be more convenient in some places.
 */
/*
 * Destruction of a JPEG object.
 *
 * Everything gets deallocated except the master jpeg_compress_struct itself
 * and the error manager struct.  Both of these are supplied by the application
 * and must be freed, if necessary, by the application.  (Often they are on
 * the stack and so don't need to be freed anyway.)
 * Closing a data source or destination, if necessary, is the application's
 * responsibility.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_destroy(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    /* We need only tell the memory manager to release everything. */
    /* NB: mem pointer is NULL if memory mgr failed to initialize. */
    if !(*cinfo).mem.is_null() {
        Some(
            (*(*cinfo).mem)
                .self_destruct
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo); /* be safe if jpeg_destroy is called twice */
    }
    (*cinfo).mem = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_memory_mgr;
    (*cinfo).global_state = 0i32;
    /* mark it destroyed */
}
/*
 * Convenience routines for allocating quantization and Huffman tables.
 * (Would jutils.c be a more reasonable place to put these?)
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_alloc_quant_table(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
) -> *mut crate::jpeglib_h::JQUANT_TBL {
    let mut tbl: *mut crate::jpeglib_h::JQUANT_TBL = ::std::ptr::null_mut::< crate::jpeglib_h::JQUANT_TBL>(); /* make sure this is false in any new table */
    tbl = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        crate::jpeglib_h::JPOOL_PERMANENT,
        ::std::mem::size_of::<crate::jpeglib_h::JQUANT_TBL>() as libc::c_ulong,
    ) as *mut crate::jpeglib_h::JQUANT_TBL; /* make sure this is false in any new table */
    (*tbl).sent_table = crate::jmorecfg_h::FALSE;
    return tbl;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_alloc_huff_table(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
) -> *mut crate::jpeglib_h::JHUFF_TBL {
    let mut tbl: *mut crate::jpeglib_h::JHUFF_TBL = ::std::ptr::null_mut::< crate::jpeglib_h::JHUFF_TBL>();
    tbl = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        crate::jpeglib_h::JPOOL_PERMANENT,
        ::std::mem::size_of::<crate::jpeglib_h::JHUFF_TBL>() as libc::c_ulong,
    ) as *mut crate::jpeglib_h::JHUFF_TBL;
    (*tbl).sent_table = crate::jmorecfg_h::FALSE;
    return tbl;
}
