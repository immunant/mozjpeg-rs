pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jpeg_color_deconverter, jpeg_color_quantizer, jpeg_d_coef_controller,
    jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master, jpeg_entropy_decoder,
    jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader, jpeg_upsampler, CSTATE_START,
    DSTATE_START, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_common_struct, jpeg_component_info,
    jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_parser_method, jpeg_marker_struct,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_source_mgr,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL,
    JPOOL_NUMPOOLS, JPOOL_PERMANENT, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE,
    J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_ulong};
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
/* Generic versions of jpeg_abort and jpeg_destroy that work on either
 * flavor of JPEG object.  These may be more convenient in some places.
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
pub unsafe extern "C" fn jpeg_abort(mut cinfo: j_common_ptr) {
    let mut pool: c_int = 0;
    if (*cinfo).mem.is_null() {
        return;
    }
    pool = JPOOL_NUMPOOLS - 1i32;
    while pool > JPOOL_PERMANENT {
        (*(*cinfo).mem)
            .free_pool
            .expect("non-null function pointer")(cinfo, pool);
        pool -= 1
    }
    if 0 != (*cinfo).is_decompressor {
        (*cinfo).global_state = DSTATE_START;
        let ref mut fresh0 = (*(cinfo as j_decompress_ptr)).marker_list;
        *fresh0 = NULL as jpeg_saved_marker_ptr
    } else {
        (*cinfo).global_state = CSTATE_START
    };
}
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
pub unsafe extern "C" fn jpeg_destroy(mut cinfo: j_common_ptr) {
    if !(*cinfo).mem.is_null() {
        (*(*cinfo).mem)
            .self_destruct
            .expect("non-null function pointer")(cinfo);
    }
    (*cinfo).mem = NULL as *mut jpeg_memory_mgr;
    (*cinfo).global_state = 0i32;
}
/*
 * Convenience routines for allocating quantization and Huffman tables.
 * (Would jutils.c be a more reasonable place to put these?)
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_alloc_quant_table(mut cinfo: j_common_ptr) -> *mut JQUANT_TBL {
    let mut tbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    tbl = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo,
        JPOOL_PERMANENT,
        ::std::mem::size_of::<JQUANT_TBL>() as c_ulong,
    ) as *mut JQUANT_TBL;
    (*tbl).sent_table = FALSE;
    return tbl;
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_alloc_huff_table(mut cinfo: j_common_ptr) -> *mut JHUFF_TBL {
    let mut tbl: *mut JHUFF_TBL = 0 as *mut JHUFF_TBL;
    tbl = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo,
        JPOOL_PERMANENT,
        ::std::mem::size_of::<JHUFF_TBL>() as c_ulong,
    ) as *mut JHUFF_TBL;
    (*tbl).sent_table = FALSE;
    return tbl;
}
