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
pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpegint_h::{
    CSTATE_SCANNING, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_writer, jpeg_memory_mgr,
    jpeg_progress_mgr, jpeg_scan_info, jpeg_write_m_byte, jpeg_write_m_header,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPEG_APP0, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE,
    JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_uint};
/*
 * jcicc.c
 *
 * Copyright (C) 1997-1998, Thomas G. Lane, Todd Newman.
 * Copyright (C) 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file provides code to write International Color Consortium (ICC) device
 * profiles embedded in JFIF JPEG image files.  The ICC has defined a standard
 * for including such data in JPEG "APP2" markers.  The code given here does
 * not know anything about the internal structure of the ICC profile data; it
 * just knows how to embed the profile data in a JPEG file while writing it.
 */
/*
 * Since an ICC profile can be larger than the maximum size of a JPEG marker
 * (64K), we need provisions to split it into multiple markers.  The format
 * defined by the ICC specifies one or more APP2 markers containing the
 * following data:
 *      Identifying string      ASCII "ICC_PROFILE\0"  (12 bytes)
 *      Marker sequence number  1 for first APP2, 2 for next, etc (1 byte)
 *      Number of markers       Total number of APP2's used (1 byte)
 *      Profile data            (remainder of APP2 data)
 * Decoders should use the marker sequence numbers to reassemble the profile,
 * rather than assuming that the APP2 markers appear in the correct sequence.
 */

pub const ICC_MARKER: c_int = JPEG_APP0 + 2i32;
/* JPEG marker code for ICC */

pub const ICC_OVERHEAD_LEN: c_int = 14i32;
/* size of non-profile data in APP2 */

pub const MAX_BYTES_IN_MARKER: c_int = 65533i32;
/* maximum data len of a JPEG marker */

pub const MAX_DATA_BYTES_IN_MARKER: c_int = MAX_BYTES_IN_MARKER - ICC_OVERHEAD_LEN;
/* Write ICC profile.  See libjpeg.txt for usage information. */
/*
 * This routine writes the given ICC profile data into a JPEG file.  It *must*
 * be called AFTER calling jpeg_start_compress() and BEFORE the first call to
 * jpeg_write_scanlines().  (This ordering ensures that the APP2 marker(s) will
 * appear after the SOI and JFIF or Adobe markers, but before all else.)
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_icc_profile(
    mut cinfo: j_compress_ptr,
    mut icc_data_ptr: *const JOCTET,
    mut icc_data_len: c_uint,
) {
    /* number of bytes to write in this marker */
    if icc_data_ptr.is_null() || icc_data_len == 0u32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).global_state < CSTATE_SCANNING {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    let mut num_markers: c_uint = icc_data_len / MAX_DATA_BYTES_IN_MARKER as c_uint;
    if num_markers * MAX_DATA_BYTES_IN_MARKER as c_uint != icc_data_len {
        num_markers += 1
    }
    while icc_data_len > 0u32 {
        /* length of profile to put in this marker */
        let mut cur_marker: c_int = 1i32;
        let mut length: c_uint = icc_data_len;
        if length > MAX_DATA_BYTES_IN_MARKER as c_uint {
            length = MAX_DATA_BYTES_IN_MARKER as c_uint
        }
        icc_data_len -= length;
        /* Write the JPEG marker header (APP2 code and marker length) */
        jpeg_write_m_header(cinfo, ICC_MARKER, length + ICC_OVERHEAD_LEN as c_uint);
        /* Write the marker identifying string "ICC_PROFILE" (null-terminated).  We
         * code it in this less-than-transparent way so that the code works even if
         * the local character set is not ASCII.
         */
        jpeg_write_m_byte(cinfo, 0x49i32);
        jpeg_write_m_byte(cinfo, 0x43i32);
        jpeg_write_m_byte(cinfo, 0x43i32);
        jpeg_write_m_byte(cinfo, 0x5fi32);
        jpeg_write_m_byte(cinfo, 0x50i32);
        jpeg_write_m_byte(cinfo, 0x52i32);
        jpeg_write_m_byte(cinfo, 0x4fi32);
        jpeg_write_m_byte(cinfo, 0x46i32);
        jpeg_write_m_byte(cinfo, 0x49i32);
        jpeg_write_m_byte(cinfo, 0x4ci32);
        jpeg_write_m_byte(cinfo, 0x45i32);
        jpeg_write_m_byte(cinfo, 0i32);
        /* Add the sequencing info */
        jpeg_write_m_byte(cinfo, cur_marker);
        jpeg_write_m_byte(cinfo, num_markers as c_int);
        loop
        /* Add the profile data */
        {
            let fresh0 = length;
            length -= 1;
            if !(fresh0 != 0) {
                break;
            }
            jpeg_write_m_byte(cinfo, *icc_data_ptr as c_int);
            icc_data_ptr = icc_data_ptr.offset(1)
        }
        cur_marker += 1
    }
}
