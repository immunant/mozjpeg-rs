use libc::c_uint;use libc::c_ulong;use libc::c_int;use libc::c_char;use libc;

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
pub use super::jerror::C2RustUnnamed_3;
pub use super::jerror::JERR_ARITH_NOTIMPL;
pub use super::jerror::JERR_BAD_ALIGN_TYPE;
pub use super::jerror::JERR_BAD_ALLOC_CHUNK;
pub use super::jerror::JERR_BAD_BUFFER_MODE;
pub use super::jerror::JERR_BAD_COMPONENT_ID;
pub use super::jerror::JERR_BAD_CROP_SPEC;
pub use super::jerror::JERR_BAD_DCTSIZE;
pub use super::jerror::JERR_BAD_DCT_COEF;
pub use super::jerror::JERR_BAD_HUFF_TABLE;
pub use super::jerror::JERR_BAD_IN_COLORSPACE;
pub use super::jerror::JERR_BAD_J_COLORSPACE;
pub use super::jerror::JERR_BAD_LENGTH;
pub use super::jerror::JERR_BAD_LIB_VERSION;
pub use super::jerror::JERR_BAD_MCU_SIZE;
pub use super::jerror::JERR_BAD_PARAM;
pub use super::jerror::JERR_BAD_PARAM_VALUE;
pub use super::jerror::JERR_BAD_POOL_ID;
pub use super::jerror::JERR_BAD_PRECISION;
pub use super::jerror::JERR_BAD_PROGRESSION;
pub use super::jerror::JERR_BAD_PROG_SCRIPT;
pub use super::jerror::JERR_BAD_SAMPLING;
pub use super::jerror::JERR_BAD_SCAN_SCRIPT;
pub use super::jerror::JERR_BAD_STATE;
pub use super::jerror::JERR_BAD_STRUCT_SIZE;
pub use super::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use super::jerror::JERR_BUFFER_SIZE;
pub use super::jerror::JERR_CANT_SUSPEND;
pub use super::jerror::JERR_CCIR601_NOTIMPL;
pub use super::jerror::JERR_COMPONENT_COUNT;
pub use super::jerror::JERR_CONVERSION_NOTIMPL;
pub use super::jerror::JERR_DAC_INDEX;
pub use super::jerror::JERR_DAC_VALUE;
pub use super::jerror::JERR_DHT_INDEX;
pub use super::jerror::JERR_DQT_INDEX;
pub use super::jerror::JERR_EMPTY_IMAGE;
pub use super::jerror::JERR_EMS_READ;
pub use super::jerror::JERR_EMS_WRITE;
pub use super::jerror::JERR_EOI_EXPECTED;
pub use super::jerror::JERR_FILE_READ;
pub use super::jerror::JERR_FILE_WRITE;
pub use super::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use super::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use super::jerror::JERR_HUFF_MISSING_CODE;
pub use super::jerror::JERR_IMAGE_TOO_BIG;
pub use super::jerror::JERR_INPUT_EMPTY;
pub use super::jerror::JERR_INPUT_EOF;
pub use super::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use super::jerror::JERR_MISSING_DATA;
pub use super::jerror::JERR_MODE_CHANGE;
pub use super::jerror::JERR_NOTIMPL;
pub use super::jerror::JERR_NOT_COMPILED;
pub use super::jerror::JERR_NO_BACKING_STORE;
pub use super::jerror::JERR_NO_HUFF_TABLE;
pub use super::jerror::JERR_NO_IMAGE;
pub use super::jerror::JERR_NO_QUANT_TABLE;
pub use super::jerror::JERR_NO_SOI;
pub use super::jerror::JERR_OUT_OF_MEMORY;
pub use super::jerror::JERR_QUANT_COMPONENTS;
pub use super::jerror::JERR_QUANT_FEW_COLORS;
pub use super::jerror::JERR_QUANT_MANY_COLORS;
pub use super::jerror::JERR_SOF_DUPLICATE;
pub use super::jerror::JERR_SOF_NO_SOS;
pub use super::jerror::JERR_SOF_UNSUPPORTED;
pub use super::jerror::JERR_SOI_DUPLICATE;
pub use super::jerror::JERR_SOS_NO_SOF;
pub use super::jerror::JERR_TFILE_CREATE;
pub use super::jerror::JERR_TFILE_READ;
pub use super::jerror::JERR_TFILE_SEEK;
pub use super::jerror::JERR_TFILE_WRITE;
pub use super::jerror::JERR_TOO_LITTLE_DATA;
pub use super::jerror::JERR_UNKNOWN_MARKER;
pub use super::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use super::jerror::JERR_VIRTUAL_BUG;
pub use super::jerror::JERR_WIDTH_OVERFLOW;
pub use super::jerror::JERR_XMS_READ;
pub use super::jerror::JERR_XMS_WRITE;
pub use super::jerror::JMSG_COPYRIGHT;
pub use super::jerror::JMSG_LASTMSGCODE;
pub use super::jerror::JMSG_NOMESSAGE;
pub use super::jerror::JMSG_VERSION;
pub use super::jerror::JTRC_16BIT_TABLES;
pub use super::jerror::JTRC_ADOBE;
pub use super::jerror::JTRC_APP0;
pub use super::jerror::JTRC_APP14;
pub use super::jerror::JTRC_DAC;
pub use super::jerror::JTRC_DHT;
pub use super::jerror::JTRC_DQT;
pub use super::jerror::JTRC_DRI;
pub use super::jerror::JTRC_EMS_CLOSE;
pub use super::jerror::JTRC_EMS_OPEN;
pub use super::jerror::JTRC_EOI;
pub use super::jerror::JTRC_HUFFBITS;
pub use super::jerror::JTRC_JFIF;
pub use super::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use super::jerror::JTRC_JFIF_EXTENSION;
pub use super::jerror::JTRC_JFIF_THUMBNAIL;
pub use super::jerror::JTRC_MISC_MARKER;
pub use super::jerror::JTRC_PARMLESS_MARKER;
pub use super::jerror::JTRC_QUANTVALS;
pub use super::jerror::JTRC_QUANT_3_NCOLORS;
pub use super::jerror::JTRC_QUANT_NCOLORS;
pub use super::jerror::JTRC_QUANT_SELECTED;
pub use super::jerror::JTRC_RECOVERY_ACTION;
pub use super::jerror::JTRC_RST;
pub use super::jerror::JTRC_SMOOTH_NOTIMPL;
pub use super::jerror::JTRC_SOF;
pub use super::jerror::JTRC_SOF_COMPONENT;
pub use super::jerror::JTRC_SOI;
pub use super::jerror::JTRC_SOS;
pub use super::jerror::JTRC_SOS_COMPONENT;
pub use super::jerror::JTRC_SOS_PARAMS;
pub use super::jerror::JTRC_TFILE_CLOSE;
pub use super::jerror::JTRC_TFILE_OPEN;
pub use super::jerror::JTRC_THUMB_JPEG;
pub use super::jerror::JTRC_THUMB_PALETTE;
pub use super::jerror::JTRC_THUMB_RGB;
pub use super::jerror::JTRC_UNKNOWN_IDS;
pub use super::jerror::JTRC_XMS_CLOSE;
pub use super::jerror::JTRC_XMS_OPEN;
pub use super::jerror::JWRN_ADOBE_XFORM;
pub use super::jerror::JWRN_BOGUS_ICC;
pub use super::jerror::JWRN_BOGUS_PROGRESSION;
pub use super::jerror::JWRN_EXTRANEOUS_DATA;
pub use super::jerror::JWRN_HIT_MARKER;
pub use super::jerror::JWRN_HUFF_BAD_CODE;
pub use super::jerror::JWRN_JFIF_MAJOR;
pub use super::jerror::JWRN_JPEG_EOF;
pub use super::jerror::JWRN_MUST_RESYNC;
pub use super::jerror::JWRN_NOT_SEQUENTIAL;
pub use super::jerror::JWRN_TOO_MUCH_DATA;
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

pub const ICC_MARKER: c_int = JPEG_APP0 + 2i32;
/* JPEG marker code for ICC */

pub const ICC_OVERHEAD_LEN: c_int = 14i32;
/* size of non-profile data in APP2 */
/*
 * Handy subroutine to test whether a saved marker is an ICC profile marker.
 */

unsafe extern "C" fn marker_is_icc(
    mut marker: jpeg_saved_marker_ptr,
) -> boolean {
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
    
    
    
    
    
      let mut num_markers:  c_int =  0i32;    let mut marker_present:  [c_char; 256] =  [0; 256]; let mut data_length:  [c_uint; 256] =  [0; 256]; let mut data_offset:  [c_uint; 256] =  [0; 256]; /* offset for data in marker */
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
     let mut seq_no:   c_int =  1i32; /* inconsistent num_markers fields */
    while seq_no <= MAX_SEQ_NO {
        marker_present[seq_no as usize] = 0i8; /* bogus sequence number */
        seq_no += 1
    } /* duplicate sequence numbers */
     let mut marker:   jpeg_saved_marker_ptr =
     (*cinfo).marker_list;
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
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    -1i32,
                );
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
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    -1i32,
                );
                return FALSE;
            }
            if marker_present[seq_no as usize] != 0 {
                (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_ICC as c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    -1i32,
                );
                return FALSE;
            }
            marker_present[seq_no as usize] = 1i8;
            data_length[seq_no as usize] =  (*marker)
                .data_length - ICC_OVERHEAD_LEN as c_uint
        }
        marker = (*marker).next
    }
    if num_markers == 0i32 {
        return FALSE;
    }
    /* Check for missing markers, count total space needed,
     * compute offset of each marker's part of the data.
     */
     let mut total_length:   c_uint =  0u32; /* missing sequence number */
    seq_no = 1i32; /* found only empty markers? */
    while seq_no <= num_markers {
        if marker_present[seq_no as usize] as c_int == 0i32 {
            (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_ICC as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, -1i32
            );
            return FALSE;
        }
        data_offset[seq_no as usize] = total_length;
        total_length +=  data_length[seq_no as usize];
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
     let mut icc_data:   *mut JOCTET =
     malloc(
        total_length as c_ulong *
    ::std::mem::size_of::<JOCTET>() as c_ulong,
    ) as *mut JOCTET; /* oops, out of memory */
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
            
            
             let mut dst_ptr:   *mut JOCTET =
     icc_data.offset(data_offset[seq_no as usize] as isize); let mut src_ptr:   *mut JOCTET =
     (*marker).data.offset(ICC_OVERHEAD_LEN as isize); let mut length:   c_uint =  data_length[seq_no as usize];
            loop {
                let fresh0 = length;
                length -=  1;
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
