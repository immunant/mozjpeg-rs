use libc::c_long;use libc::c_int;use libc::c_uint;pub use crate::jerror::C2RustUnnamed_3;
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
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jinit_compress_master;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::CSTATE_RAW_OK;
pub use crate::jpegint_h::CSTATE_SCANNING;
pub use crate::jpegint_h::CSTATE_START;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_suppress_tables;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use libc;
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
            .expect("non-null function pointer")(
            cinfo as j_common_ptr, -1i32
        );
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
    (*cinfo).next_scanline = ((*cinfo).next_scanline as c_uint).wrapping_add(row_ctr)
        as JDIMENSION
        as JDIMENSION;
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
            .expect("non-null function pointer")(
            cinfo as j_common_ptr, -1i32
        );
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
    lines_per_iMCU_row =
        ((*cinfo).max_v_samp_factor * DCTSIZE) as JDIMENSION;
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
    (*cinfo).next_scanline =
        ((*cinfo).next_scanline as c_uint).wrapping_add(lines_per_iMCU_row)
            as JDIMENSION as JDIMENSION;
    return lines_per_iMCU_row;
}
