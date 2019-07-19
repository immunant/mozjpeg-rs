pub use crate::cdjpeg::{cjpeg_source_ptr, cjpeg_source_struct};
pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_struct, jpeg_marker_writer,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_scan_info,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JQUANT_TBL, JSAMPARRAY, JSAMPROW, J_COLOR_SPACE,
    J_DCT_METHOD,
};
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, exit, EXIT_FAILURE,
    FILE, _IO_FILE,
};
use crate::stdlib::{fprintf, stderr};
use libc::{self, c_char};
/*
 * rdgif.c
 *
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to read input images in GIF format.
 *
 *****************************************************************************
 * NOTE: to avoid entanglements with Unisys' patent on LZW compression,      *
 * the ability to read GIF files has been removed from the IJG distribution. *
 * Sorry about that.                                                         *
 *****************************************************************************
 *
 * We are required to state that
 *    "The Graphics Interchange Format(c) is the Copyright property of
 *    CompuServe Incorporated. GIF(sm) is a Service Mark property of
 *    CompuServe Incorporated."
 */
/*
 * The module selection routine for GIF format input.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_read_gif(mut _cinfo: j_compress_ptr) -> cjpeg_source_ptr {
    fprintf(
        stderr,
        b"GIF input is unsupported for legal reasons.  Sorry.\n\x00" as *const u8 as *const c_char,
    );
    exit(EXIT_FAILURE);
}
