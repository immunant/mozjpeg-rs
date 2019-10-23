pub use super::jerror::{
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
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, DSTATE_READY, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
    JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPEG_APP0, JQUANT_TBL, JSAMPARRAY,
    JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::malloc;
use libc::{self, c_char, c_int, c_uint, c_ulong};
/*
 * jdicc.c
 *
 * Copyright (C) 1997-1998, Thomas G. Lane, Todd Newman.
 * Copyright (C) 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file provides code to read International Color Consortium (ICC) device
 * profiles embedded in JFIF JPEG image files.  The ICC has defined a standard
 * for including such data in JPEG "APP2" markers.  The code given here does
 * not know anything about the internal structure of the ICC profile data; it
 * just knows how to get the profile data from a JPEG file while reading it.
 */
/* <stdlib.h> should declare malloc() */

pub const ICC_MARKER: c_int = JPEG_APP0 + 2i32;
/* JPEG marker code for ICC */

pub const ICC_OVERHEAD_LEN: c_int = 14i32;
/* size of non-profile data in APP2 */
/*
 * Handy subroutine to test whether a saved marker is an ICC profile marker.
 */

unsafe extern "C" fn marker_is_icc(mut marker: jpeg_saved_marker_ptr) -> boolean {
    return ((*marker).marker as c_int == ICC_MARKER
        && (*marker).data_length >= ICC_OVERHEAD_LEN as c_uint
        && *(*marker).data.offset(0) as c_int == 0x49i32
        && *(*marker).data.offset(1) as c_int == 0x43i32
        && *(*marker).data.offset(2) as c_int == 0x43i32
        && *(*marker).data.offset(3) as c_int == 0x5fi32
        && *(*marker).data.offset(4) as c_int == 0x50i32
        && *(*marker).data.offset(5) as c_int == 0x52i32
        && *(*marker).data.offset(6) as c_int == 0x4fi32
        && *(*marker).data.offset(7) as c_int == 0x46i32
        && *(*marker).data.offset(8) as c_int == 0x49i32
        && *(*marker).data.offset(9) as c_int == 0x4ci32
        && *(*marker).data.offset(10) as c_int == 0x45i32
        && *(*marker).data.offset(11) as c_int == 0i32) as c_int;
}
/* Accessor functions for extension parameters */
/* Read ICC profile.  See libjpeg.txt for usage information. */
/*
 * See if there was an ICC profile in the JPEG file being read; if so,
 * reassemble and return the profile data.
 *
 * TRUE is returned if an ICC profile was found, FALSE if not.  If TRUE is
 * returned, *icc_data_ptr is set to point to the returned data, and
 * *icc_data_len is set to its length.
 *
 * IMPORTANT: the data at *icc_data_ptr is allocated with malloc() and must be
 * freed by the caller with free() when the caller no longer needs it.
 * (Alternatively, we could write this routine to use the IJG library's memory
 * allocator, so that the data would be freed implicitly when
 * jpeg_finish_decompress() is called.  But it seems likely that many
 * applications will prefer to have the data stick around after decompression
 * finishes.)
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_read_icc_profile(
    mut cinfo: j_decompress_ptr,
    mut icc_data_ptr: *mut *mut JOCTET,
    mut icc_data_len: *mut c_uint,
) -> boolean {
    let mut num_markers: c_int = 0i32;
    let mut marker_present: [c_char; 256] = [0; 256];
    let mut data_length: [c_uint; 256] = [0; 256];
    let mut data_offset: [c_uint; 256] = [0; 256]; /* offset for data in marker */
    if icc_data_ptr.is_null() || icc_data_len.is_null() {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int; /* avoid confusion if FALSE return */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).global_state < DSTATE_READY {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    *icc_data_ptr = NULL as *mut JOCTET;
    *icc_data_len = 0u32;
    /* This first pass over the saved markers discovers whether there are
     * any ICC markers and verifies the consistency of the marker numbering.
     */
    let mut seq_no: c_int = 1i32; /* inconsistent num_markers fields */
    while seq_no <= MAX_SEQ_NO {
        marker_present[seq_no as usize] = 0i8; /* bogus sequence number */
        seq_no += 1
    } /* duplicate sequence numbers */
    let mut marker: jpeg_saved_marker_ptr = (*cinfo).marker_list;
    while !marker.is_null() {
        if marker_is_icc(marker) != 0 {
            if num_markers == 0i32 {
                num_markers = *(*marker).data.offset(13) as c_int
            } else if num_markers != *(*marker).data.offset(13) as c_int {
                (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_ICC as c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
                return FALSE;
            }
            seq_no = *(*marker).data.offset(12) as c_int;
            if seq_no <= 0i32 || seq_no > num_markers {
                (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_ICC as c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
                return FALSE;
            }
            if marker_present[seq_no as usize] != 0 {
                (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_ICC as c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
                return FALSE;
            }
            marker_present[seq_no as usize] = 1i8;
            data_length[seq_no as usize] = (*marker).data_length - ICC_OVERHEAD_LEN as c_uint
        }
        marker = (*marker).next
    }
    if num_markers == 0i32 {
        return FALSE;
    }
    /* Check for missing markers, count total space needed,
     * compute offset of each marker's part of the data.
     */
    let mut total_length: c_uint = 0u32; /* missing sequence number */
    seq_no = 1i32; /* found only empty markers? */
    while seq_no <= num_markers {
        if marker_present[seq_no as usize] as c_int == 0i32 {
            (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_ICC as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
            return FALSE;
        }
        data_offset[seq_no as usize] = total_length;
        total_length += data_length[seq_no as usize];
        seq_no += 1
    }
    if total_length == 0u32 {
        (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_ICC as c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
        return FALSE;
    }
    let mut icc_data: *mut JOCTET =
        malloc(total_length as c_ulong * ::std::mem::size_of::<JOCTET>() as c_ulong) as *mut JOCTET; /* oops, out of memory */
    if icc_data.is_null() {
        (*(*cinfo).err).msg_code = super::jerror::JERR_OUT_OF_MEMORY as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 11i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* and fill it in */
    marker = (*cinfo).marker_list;
    while !marker.is_null() {
        if marker_is_icc(marker) != 0 {
            seq_no = *(*marker).data.offset(12) as c_int;

            let mut dst_ptr: *mut JOCTET = icc_data.offset(data_offset[seq_no as usize] as isize);
            let mut src_ptr: *mut JOCTET = (*marker).data.offset(ICC_OVERHEAD_LEN as isize);
            let mut length: c_uint = data_length[seq_no as usize];
            loop {
                let fresh0 = length;
                length -= 1;
                if !(fresh0 != 0) {
                    break;
                }
                let fresh1 = src_ptr;
                src_ptr = src_ptr.offset(1);
                let fresh2 = dst_ptr;
                dst_ptr = dst_ptr.offset(1);
                *fresh2 = *fresh1
            }
        }
        marker = (*marker).next
    }
    *icc_data_ptr = icc_data;
    *icc_data_len = total_length;
    return TRUE;
}

pub const MAX_SEQ_NO: c_int = 255i32;
