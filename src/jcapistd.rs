pub use crate::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpegint_h::{
    jinit_compress_master, jpeg_c_coef_controller, jpeg_c_main_controller, jpeg_c_prep_controller,
    jpeg_color_converter, jpeg_comp_master, jpeg_downsampler, jpeg_entropy_encoder,
    jpeg_forward_dct, jpeg_marker_writer, CSTATE_RAW_OK, CSTATE_SCANNING, CSTATE_START,
    JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE,
    J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_common_struct, jpeg_component_info, jpeg_compress_struct,
    jpeg_destination_mgr, jpeg_error_mgr, jpeg_memory_mgr, jpeg_progress_mgr, jpeg_scan_info,
    jpeg_suppress_tables, jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control,
    jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, JBLOCK, JBLOCKARRAY, JBLOCKROW,
    JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
    JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
    JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JQUANT_TBL, JSAMPARRAY,
    JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_long, c_uint};
/*
 * jcapistd.c
 *
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains application interface code for the compression half
 * of the JPEG library.  These are the "standard" API routines that are
 * used in the normal full-compression case.  They are not used by a
 * transcoding-only application.  Note that if an application links in
 * jpeg_start_compress, it will end up linking in the entire compressor.
 * We thus must separate this file from jcapimin.c to avoid linking the
 * whole compression library into a transcoder.
 */
/* Main entry points for compression */
/*
 * Compression initialization.
 * Before calling this, all parameters and a data destination must be set up.
 *
 * We require a write_all_tables parameter as a failsafe check when writing
 * multiple datastreams from the same compression object.  Since prior runs
 * will have left all the tables marked sent_table=TRUE, a subsequent run
 * would emit an abbreviated stream (no tables) by default.  This may be what
 * is wanted, but for safety's sake it should not be the default behavior:
 * programmers should have to make a deliberate choice to emit abbreviated
 * images.  Therefore the documentation and examples should encourage people
 * to pass write_all_tables=TRUE; then it will take active thought to do the
 * wrong thing.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_start_compress(
    mut cinfo: j_compress_ptr,
    mut write_all_tables: boolean,
) {
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if 0 != write_all_tables {
        jpeg_suppress_tables(cinfo, FALSE);
    }
    if (*(*cinfo).master).num_scans_luma == 0i32
        || (*cinfo).scan_info.is_null()
        || (*cinfo).num_scans == 0i32
    {
        (*(*cinfo).master).optimize_scans = FALSE
    }
    (*(*cinfo).err)
        .reset_error_mgr
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    (*(*cinfo).dest)
        .init_destination
        .expect("non-null function pointer")(cinfo);
    jinit_compress_master(cinfo);
    (*(*cinfo).master)
        .prepare_for_pass
        .expect("non-null function pointer")(cinfo);
    (*cinfo).next_scanline = 0i32 as JDIMENSION;
    (*cinfo).global_state = if 0 != (*cinfo).raw_data_in {
        CSTATE_RAW_OK
    } else {
        CSTATE_SCANNING
    };
}
/*
 * Write some scanlines of data to the JPEG compressor.
 *
 * The return value will be the number of lines actually written.
 * This should be less than the supplied num_lines only in case that
 * the data destination module has requested suspension of the compressor,
 * or if more than image_height scanlines are passed in.
 *
 * Note: we warn about excess calls to jpeg_write_scanlines() since
 * this likely signals an application programmer error.  However,
 * excess scanlines passed in the last valid call are *silently* ignored,
 * so that the application need not adjust num_lines for end-of-image
 * when using a multiple-scanline buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_write_scanlines(
    mut cinfo: j_compress_ptr,
    mut scanlines: JSAMPARRAY,
    mut num_lines: JDIMENSION,
) -> JDIMENSION {
    let mut row_ctr: JDIMENSION = 0;
    let mut rows_left: JDIMENSION = 0;
    if (*cinfo).global_state != CSTATE_SCANNING {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).next_scanline >= (*cinfo).image_height {
        (*(*cinfo).err).msg_code = JWRN_TOO_MUCH_DATA as c_int;
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
    }
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).next_scanline as c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).image_height as c_long;
        (*(*cinfo).progress)
            .progress_monitor
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if 0 != (*(*cinfo).master).call_pass_startup {
        (*(*cinfo).master)
            .pass_startup
            .expect("non-null function pointer")(cinfo);
    }
    rows_left = (*cinfo).image_height.wrapping_sub((*cinfo).next_scanline);
    if num_lines > rows_left {
        num_lines = rows_left
    }
    row_ctr = 0i32 as JDIMENSION;
    (*(*cinfo).main)
        .process_data
        .expect("non-null function pointer")(cinfo, scanlines, &mut row_ctr, num_lines);
    (*cinfo).next_scanline =
        ((*cinfo).next_scanline as c_uint).wrapping_add(row_ctr) as JDIMENSION as JDIMENSION;
    return row_ctr;
}
/* Replaces jpeg_write_scanlines when writing raw downsampled data. */
/*
 * Alternate entry point to write raw data.
 * Processes exactly one iMCU row per call, unless suspended.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_write_raw_data(
    mut cinfo: j_compress_ptr,
    mut data: JSAMPIMAGE,
    mut num_lines: JDIMENSION,
) -> JDIMENSION {
    let mut lines_per_iMCU_row: JDIMENSION = 0;
    if (*cinfo).global_state != CSTATE_RAW_OK {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).next_scanline >= (*cinfo).image_height {
        (*(*cinfo).err).msg_code = JWRN_TOO_MUCH_DATA as c_int;
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
        return 0i32 as JDIMENSION;
    }
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).next_scanline as c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).image_height as c_long;
        (*(*cinfo).progress)
            .progress_monitor
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if 0 != (*(*cinfo).master).call_pass_startup {
        (*(*cinfo).master)
            .pass_startup
            .expect("non-null function pointer")(cinfo);
    }
    lines_per_iMCU_row = ((*cinfo).max_v_samp_factor * DCTSIZE) as JDIMENSION;
    if num_lines < lines_per_iMCU_row {
        (*(*cinfo).err).msg_code = JERR_BUFFER_SIZE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if 0 == (*(*cinfo).coef)
        .compress_data
        .expect("non-null function pointer")(cinfo, data)
    {
        return 0i32 as JDIMENSION;
    }
    (*cinfo).next_scanline = ((*cinfo).next_scanline as c_uint).wrapping_add(lines_per_iMCU_row)
        as JDIMENSION as JDIMENSION;
    return lines_per_iMCU_row;
}
