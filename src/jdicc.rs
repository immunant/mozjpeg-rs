use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::DSTATE_READY;
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
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::jerror::C2RustUnnamed_3;
pub use crate::src::jerror::JERR_ARITH_NOTIMPL;
pub use crate::src::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_LENGTH;
pub use crate::src::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jerror::JERR_BAD_PARAM;
pub use crate::src::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::src::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jerror::JERR_BAD_PRECISION;
pub use crate::src::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jerror::JERR_BAD_STATE;
pub use crate::src::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jerror::JERR_DAC_INDEX;
pub use crate::src::jerror::JERR_DAC_VALUE;
pub use crate::src::jerror::JERR_DHT_INDEX;
pub use crate::src::jerror::JERR_DQT_INDEX;
pub use crate::src::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jerror::JERR_EMS_READ;
pub use crate::src::jerror::JERR_EMS_WRITE;
pub use crate::src::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jerror::JERR_FILE_READ;
pub use crate::src::jerror::JERR_FILE_WRITE;
pub use crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jerror::JERR_INPUT_EOF;
pub use crate::src::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jerror::JERR_MISSING_DATA;
pub use crate::src::jerror::JERR_MODE_CHANGE;
pub use crate::src::jerror::JERR_NOTIMPL;
pub use crate::src::jerror::JERR_NOT_COMPILED;
pub use crate::src::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jerror::JERR_NO_IMAGE;
pub use crate::src::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jerror::JERR_NO_SOI;
pub use crate::src::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jerror::JERR_TFILE_CREATE;
pub use crate::src::jerror::JERR_TFILE_READ;
pub use crate::src::jerror::JERR_TFILE_SEEK;
pub use crate::src::jerror::JERR_TFILE_WRITE;
pub use crate::src::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::src::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jerror::JERR_XMS_READ;
pub use crate::src::jerror::JERR_XMS_WRITE;
pub use crate::src::jerror::JMSG_COPYRIGHT;
pub use crate::src::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jerror::JMSG_NOMESSAGE;
pub use crate::src::jerror::JMSG_VERSION;
pub use crate::src::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jerror::JTRC_ADOBE;
pub use crate::src::jerror::JTRC_APP0;
pub use crate::src::jerror::JTRC_APP14;
pub use crate::src::jerror::JTRC_DAC;
pub use crate::src::jerror::JTRC_DHT;
pub use crate::src::jerror::JTRC_DQT;
pub use crate::src::jerror::JTRC_DRI;
pub use crate::src::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jerror::JTRC_EMS_OPEN;
pub use crate::src::jerror::JTRC_EOI;
pub use crate::src::jerror::JTRC_HUFFBITS;
pub use crate::src::jerror::JTRC_JFIF;
pub use crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jerror::JTRC_MISC_MARKER;
pub use crate::src::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jerror::JTRC_QUANTVALS;
pub use crate::src::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jerror::JTRC_RST;
pub use crate::src::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jerror::JTRC_SOF;
pub use crate::src::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jerror::JTRC_SOI;
pub use crate::src::jerror::JTRC_SOS;
pub use crate::src::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jerror::JTRC_THUMB_RGB;
pub use crate::src::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jerror::JTRC_XMS_OPEN;
pub use crate::src::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jerror::JWRN_BOGUS_ICC;
pub use crate::src::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jerror::JWRN_HIT_MARKER;
pub use crate::src::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jerror::JWRN_JPEG_EOF;
pub use crate::src::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::malloc;
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

pub const ICC_MARKER: libc::c_int = crate::jpeglib_h::JPEG_APP0 + 2i32;
/* JPEG marker code for ICC */

pub const ICC_OVERHEAD_LEN: libc::c_int = 14i32;
/* size of non-profile data in APP2 */
/*
 * Handy subroutine to test whether a saved marker is an ICC profile marker.
 */

unsafe extern "C" fn marker_is_icc(
    mut marker: crate::jpeglib_h::jpeg_saved_marker_ptr,
) -> crate::jmorecfg_h::boolean {
    return ((*marker).marker as libc::c_int == ICC_MARKER
        && (*marker).data_length >= ICC_OVERHEAD_LEN as libc::c_uint
        && *(*marker).data.offset(0) as libc::c_int == 0x49i32
        && *(*marker).data.offset(1) as libc::c_int == 0x43i32
        && *(*marker).data.offset(2) as libc::c_int == 0x43i32
        && *(*marker).data.offset(3) as libc::c_int == 0x5fi32
        && *(*marker).data.offset(4) as libc::c_int == 0x50i32
        && *(*marker).data.offset(5) as libc::c_int == 0x52i32
        && *(*marker).data.offset(6) as libc::c_int == 0x4fi32
        && *(*marker).data.offset(7) as libc::c_int == 0x46i32
        && *(*marker).data.offset(8) as libc::c_int == 0x49i32
        && *(*marker).data.offset(9) as libc::c_int == 0x4ci32
        && *(*marker).data.offset(10) as libc::c_int == 0x45i32
        && *(*marker).data.offset(11) as libc::c_int == 0i32) as libc::c_int;
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut icc_data_ptr: *mut *mut crate::jmorecfg_h::JOCTET,
    mut icc_data_len: *mut libc::c_uint,
) -> crate::jmorecfg_h::boolean {
    let mut marker: crate::jpeglib_h::jpeg_saved_marker_ptr =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_marker_struct>();
    let mut num_markers: libc::c_int = 0i32;
    let mut seq_no: libc::c_int = 0;
    let mut icc_data: *mut crate::jmorecfg_h::JOCTET = ::std::ptr::null_mut::< crate::jmorecfg_h::JOCTET>();
    let mut total_length: libc::c_uint = 0;
    /* sufficient since marker numbers are bytes */
    let mut marker_present: [libc::c_char; 256] = [0; 256]; /* 1 if marker found */
    let mut data_length: [libc::c_uint; 256] = [0; 256]; /* size of profile data in marker */
    let mut data_offset: [libc::c_uint; 256] = [0; 256]; /* offset for data in marker */
    if icc_data_ptr.is_null() || icc_data_len.is_null() {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BUFFER_SIZE as libc::c_int; /* avoid confusion if FALSE return */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if (*cinfo).global_state < crate::jpegint_h::DSTATE_READY {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    *icc_data_ptr = crate::stddef_h::NULL as *mut crate::jmorecfg_h::JOCTET;
    *icc_data_len = 0i32 as libc::c_uint;
    /* This first pass over the saved markers discovers whether there are
     * any ICC markers and verifies the consistency of the marker numbering.
     */
    seq_no = 1i32; /* inconsistent num_markers fields */
    while seq_no <= MAX_SEQ_NO {
        marker_present[seq_no as usize] = 0i32 as libc::c_char; /* bogus sequence number */
        seq_no += 1
    } /* duplicate sequence numbers */
    marker = (*cinfo).marker_list;
    while !marker.is_null() {
        if marker_is_icc(marker) != 0 {
            if num_markers == 0i32 {
                num_markers = *(*marker).data.offset(13) as libc::c_int
            } else if num_markers != *(*marker).data.offset(13) as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_BOGUS_ICC as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    -1i32,
                );
                return crate::jmorecfg_h::FALSE;
            }
            seq_no = *(*marker).data.offset(12) as libc::c_int;
            if seq_no <= 0i32 || seq_no > num_markers {
                (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_BOGUS_ICC as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    -1i32,
                );
                return crate::jmorecfg_h::FALSE;
            }
            if marker_present[seq_no as usize] != 0 {
                (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_BOGUS_ICC as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    -1i32,
                );
                return crate::jmorecfg_h::FALSE;
            }
            marker_present[seq_no as usize] = 1i32 as libc::c_char;
            data_length[seq_no as usize] = (*marker)
                .data_length
                .wrapping_sub(ICC_OVERHEAD_LEN as libc::c_uint)
        }
        marker = (*marker).next
    }
    if num_markers == 0i32 {
        return crate::jmorecfg_h::FALSE;
    }
    /* Check for missing markers, count total space needed,
     * compute offset of each marker's part of the data.
     */
    total_length = 0i32 as libc::c_uint; /* missing sequence number */
    seq_no = 1i32; /* found only empty markers? */
    while seq_no <= num_markers {
        if marker_present[seq_no as usize] as libc::c_int == 0i32 {
            (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_BOGUS_ICC as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr, -1i32
            );
            return crate::jmorecfg_h::FALSE;
        }
        data_offset[seq_no as usize] = total_length;
        total_length = total_length.wrapping_add(data_length[seq_no as usize]);
        seq_no += 1
    }
    if total_length == 0i32 as libc::c_uint {
        (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_BOGUS_ICC as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, -1i32);
        return crate::jmorecfg_h::FALSE;
    }
    /* Allocate space for assembled data */
    icc_data = crate::stdlib::malloc(
        (total_length as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JOCTET>() as libc::c_ulong),
    ) as *mut crate::jmorecfg_h::JOCTET; /* oops, out of memory */
    if icc_data.is_null() {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_OUT_OF_MEMORY as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = 11i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* and fill it in */
    marker = (*cinfo).marker_list;
    while !marker.is_null() {
        if marker_is_icc(marker) != 0 {
            let mut src_ptr: *mut crate::jmorecfg_h::JOCTET = ::std::ptr::null_mut::< crate::jmorecfg_h::JOCTET>();
            let mut dst_ptr: *mut crate::jmorecfg_h::JOCTET = ::std::ptr::null_mut::< crate::jmorecfg_h::JOCTET>();
            let mut length: libc::c_uint = 0;
            seq_no = *(*marker).data.offset(12) as libc::c_int;
            dst_ptr = icc_data.offset(data_offset[seq_no as usize] as isize);
            src_ptr = (*marker).data.offset(ICC_OVERHEAD_LEN as isize);
            length = data_length[seq_no as usize];
            loop {
                let fresh0 = length;
                length = length.wrapping_sub(1);
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
    return crate::jmorecfg_h::TRUE;
}

pub const MAX_SEQ_NO: libc::c_int = 255i32;
