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
pub use crate::jpegint_h::jpeg_natural_order;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_alloc_huff_table;
pub use crate::jpeglib_h::jpeg_alloc_quant_table;
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
pub use crate::jpeglib_h::DCTSIZE2;
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
pub use crate::jpeglib_h::JPEG_REACHED_EOI;
pub use crate::jpeglib_h::JPEG_REACHED_SOS;
pub use crate::jpeglib_h::JPEG_SUSPENDED;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::jpeglib_h::MAX_COMPS_IN_SCAN;
pub use crate::jpeglib_h::NUM_ARITH_TBLS;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
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
use crate::stdlib::memcpy;
use crate::stdlib::memset;

pub const M_APP0: C2RustUnnamed_76 = 224;

pub type my_marker_ptr = *mut my_marker_reader;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_marker_reader {
    pub pub_0: crate::jpeglib_h::jpeg_marker_reader,
    pub process_COM: crate::jpeglib_h::jpeg_marker_parser_method,
    pub process_APPn: [crate::jpeglib_h::jpeg_marker_parser_method; 16],
    pub length_limit_COM: libc::c_uint,
    pub length_limit_APPn: [libc::c_uint; 16],
    pub cur_marker: crate::jpeglib_h::jpeg_saved_marker_ptr,
    pub bytes_read: libc::c_uint,
}

pub const M_APP15: C2RustUnnamed_76 = 239;

pub const M_COM: C2RustUnnamed_76 = 254;

pub const M_APP14: C2RustUnnamed_76 = 238;

pub const M_RST0: C2RustUnnamed_76 = 208;

pub const M_RST7: C2RustUnnamed_76 = 215;

pub const M_SOF0: C2RustUnnamed_76 = 192;

pub const M_DNL: C2RustUnnamed_76 = 220;

pub const M_TEM: C2RustUnnamed_76 = 1;

pub const M_RST6: C2RustUnnamed_76 = 214;

pub const M_RST5: C2RustUnnamed_76 = 213;

pub const M_RST4: C2RustUnnamed_76 = 212;

pub const M_RST3: C2RustUnnamed_76 = 211;

pub const M_RST2: C2RustUnnamed_76 = 210;

pub const M_RST1: C2RustUnnamed_76 = 209;

pub const M_APP13: C2RustUnnamed_76 = 237;

pub const M_APP12: C2RustUnnamed_76 = 236;

pub const M_APP11: C2RustUnnamed_76 = 235;

pub const M_APP10: C2RustUnnamed_76 = 234;

pub const M_APP9: C2RustUnnamed_76 = 233;

pub const M_APP8: C2RustUnnamed_76 = 232;

pub const M_APP7: C2RustUnnamed_76 = 231;

pub const M_APP6: C2RustUnnamed_76 = 230;

pub const M_APP5: C2RustUnnamed_76 = 229;

pub const M_APP4: C2RustUnnamed_76 = 228;

pub const M_APP3: C2RustUnnamed_76 = 227;

pub const M_APP2: C2RustUnnamed_76 = 226;

pub const M_APP1: C2RustUnnamed_76 = 225;

pub const M_DRI: C2RustUnnamed_76 = 221;

pub const M_DQT: C2RustUnnamed_76 = 219;

pub const M_DHT: C2RustUnnamed_76 = 196;

pub const M_DAC: C2RustUnnamed_76 = 204;

pub const M_EOI: C2RustUnnamed_76 = 217;

pub const M_SOS: C2RustUnnamed_76 = 218;

pub const M_SOF15: C2RustUnnamed_76 = 207;

pub const M_SOF14: C2RustUnnamed_76 = 206;

pub const M_SOF13: C2RustUnnamed_76 = 205;

pub const M_SOF11: C2RustUnnamed_76 = 203;

pub const M_JPG: C2RustUnnamed_76 = 200;

pub const M_SOF7: C2RustUnnamed_76 = 199;

pub const M_SOF6: C2RustUnnamed_76 = 198;

pub const M_SOF5: C2RustUnnamed_76 = 197;

pub const M_SOF3: C2RustUnnamed_76 = 195;

pub const M_SOF10: C2RustUnnamed_76 = 202;

pub const M_SOF9: C2RustUnnamed_76 = 201;

pub const M_SOF2: C2RustUnnamed_76 = 194;

pub const M_SOF1: C2RustUnnamed_76 = 193;

pub const M_SOI: C2RustUnnamed_76 = 216;

pub type C2RustUnnamed_76 = libc::c_uint;

pub const M_ERROR: C2RustUnnamed_76 = 256;

pub const M_JPG13: C2RustUnnamed_76 = 253;

pub const M_JPG0: C2RustUnnamed_76 = 240;

pub const M_EXP: C2RustUnnamed_76 = 223;

pub const M_DHP: C2RustUnnamed_76 = 222;
/*
 * Macros for fetching data from the data source module.
 *
 * At all times, cinfo->src->next_input_byte and ->bytes_in_buffer reflect
 * the current restart point; we update them only when we have reached a
 * suitable place to restart if a suspension occurs.
 */
/* Declare and initialize local copies of input pointer/count */
/* Unload the local copies --- do this only at a restart boundary */
/* Reload the local copies --- used only in MAKE_BYTE_AVAIL */
/* Internal macro for INPUT_BYTE and INPUT_2BYTES: make a byte available.
 * Note we do *not* do INPUT_SYNC before calling fill_input_buffer,
 * but we must reload the local copies after a successful fill.
 */
/* Read a byte into variable V.
 * If must suspend, take the specified action (typically "return FALSE").
 */
/* As above, but read two bytes interpreted as an unsigned 16-bit integer.
 * V should be declared unsigned int or perhaps JLONG.
 */
/*
 * Routines to process JPEG markers.
 *
 * Entry condition: JPEG marker itself has been read and its code saved
 *   in cinfo->unread_marker; input restart point is just after the marker.
 *
 * Exit: if return TRUE, have read and processed any parameters, and have
 *   updated the restart point to point after the parameters.
 *   If return FALSE, was forced to suspend before reaching end of
 *   marker parameters; restart point has not been moved.  Same routine
 *   will be called again after application supplies more input data.
 *
 * This approach to suspension assumes that all of a marker's parameters
 * can fit into a single input bufferload.  This should hold for "normal"
 * markers.  Some COM/APPn markers might have large parameter segments
 * that might not fit.  If we are simply dropping such a marker, we use
 * skip_input_data to get past it, and thereby put the problem on the
 * source manager's shoulders.  If we are saving the marker's contents
 * into memory, we use a slightly different convention: when forced to
 * suspend, the marker processor updates the restart point to the end of
 * what it's consumed (ie, the end of the buffer) before returning FALSE.
 * On resumption, cinfo->unread_marker still contains the marker code,
 * but the data source will point to the next chunk of marker data.
 * The marker processor must retain internal state to deal with this.
 *
 * Note that we don't bother to avoid duplicate trace messages if a
 * suspension occurs within marker parameters.  Other side effects
 * require more care.
 */

unsafe extern "C" fn get_soi(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process an SOI marker */ {
    let mut i: libc::c_int = 0;
    (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_SOI as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    if (*(*cinfo).marker).saw_SOI != 0 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_SOI_DUPLICATE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Reset all parameters that are defined to be reset by SOI */
    i = 0i32;
    while i < crate::jpeglib_h::NUM_ARITH_TBLS {
        (*cinfo).arith_dc_L[i as usize] = 0i32 as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_dc_U[i as usize] = 1i32 as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_ac_K[i as usize] = 5i32 as crate::jmorecfg_h::UINT8;
        i += 1
    }
    (*cinfo).restart_interval = 0i32 as libc::c_uint;
    /* Set initial assumptions for colorspace etc */
    (*cinfo).jpeg_color_space = crate::jpeglib_h::JCS_UNKNOWN; /* Assume non-CCIR sampling??? */
    (*cinfo).CCIR601_sampling = crate::jmorecfg_h::FALSE; /* set default JFIF APP0 values */
    (*cinfo).saw_JFIF_marker = crate::jmorecfg_h::FALSE;
    (*cinfo).JFIF_major_version = 1i32 as crate::jmorecfg_h::UINT8;
    (*cinfo).JFIF_minor_version = 1i32 as crate::jmorecfg_h::UINT8;
    (*cinfo).density_unit = 0i32 as crate::jmorecfg_h::UINT8;
    (*cinfo).X_density = 1i32 as crate::jmorecfg_h::UINT16;
    (*cinfo).Y_density = 1i32 as crate::jmorecfg_h::UINT16;
    (*cinfo).saw_Adobe_marker = crate::jmorecfg_h::FALSE;
    (*cinfo).Adobe_transform = 0i32 as crate::jmorecfg_h::UINT8;
    (*(*cinfo).marker).saw_SOI = crate::jmorecfg_h::TRUE;
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn get_sof(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut is_prog: crate::jmorecfg_h::boolean,
    mut is_arith: crate::jmorecfg_h::boolean,
) -> crate::jmorecfg_h::boolean
/* Process a SOFn marker */ {
    let mut length: crate::jpegint_h::JLONG = 0;
    let mut c: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    (*cinfo).progressive_mode = is_prog;
    (*cinfo).arith_code = is_arith;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh0 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh0 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh1 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh1 as libc::c_long;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh2 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).data_precision = *fresh2 as libc::c_int;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh3 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_height = (*fresh3 as libc::c_uint) << 8i32;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh4 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_height =
        ((*cinfo).image_height as libc::c_uint).wrapping_add(*fresh4 as libc::c_uint)
            as crate::jmorecfg_h::JDIMENSION as crate::jmorecfg_h::JDIMENSION;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh5 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_width = (*fresh5 as libc::c_uint) << 8i32;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh6 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_width =
        ((*cinfo).image_width as libc::c_uint).wrapping_add(*fresh6 as libc::c_uint)
            as crate::jmorecfg_h::JDIMENSION as crate::jmorecfg_h::JDIMENSION;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh7 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).num_components = *fresh7 as libc::c_int;
    length -= 8i32 as libc::c_long;
    let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
    *_mp.offset(0) = (*cinfo).unread_marker;
    *_mp.offset(1) = (*cinfo).image_width as libc::c_int;
    *_mp.offset(2) = (*cinfo).image_height as libc::c_int;
    *_mp.offset(3) = (*cinfo).num_components;
    (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_SOF as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    if (*(*cinfo).marker).saw_SOF != 0 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_SOF_DUPLICATE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* We don't support files in which the image height is initially specified */
    /* as 0 and is later redefined by DNL.  As long as we have to check that,  */
    /* might as well have a general sanity check. */
    if (*cinfo).image_height <= 0i32 as libc::c_uint
        || (*cinfo).image_width <= 0i32 as libc::c_uint
        || (*cinfo).num_components <= 0i32
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_EMPTY_IMAGE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if length != ((*cinfo).num_components * 3i32) as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if (*cinfo).comp_info.is_null() {
        /* do only once, even if suspend */
        (*cinfo).comp_info = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            ((*cinfo).num_components as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::jpeglib_h::jpeg_component_info>() as libc::c_ulong,
                ),
        ) as *mut crate::jpeglib_h::jpeg_component_info
    }
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).component_index = ci;
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh8 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        (*compptr).component_id = *fresh8 as libc::c_int;
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh9 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh9 as libc::c_int;
        (*compptr).h_samp_factor = c >> 4i32 & 15i32;
        (*compptr).v_samp_factor = c & 15i32;
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh10 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        (*compptr).quant_tbl_no = *fresh10 as libc::c_int;
        let mut _mp_0: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp_0.offset(0) = (*compptr).component_id;
        *_mp_0.offset(1) = (*compptr).h_samp_factor;
        *_mp_0.offset(2) = (*compptr).v_samp_factor;
        *_mp_0.offset(3) = (*compptr).quant_tbl_no;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_SOF_COMPONENT as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
        ci += 1;
        compptr = compptr.offset(1)
    }
    (*(*cinfo).marker).saw_SOF = crate::jmorecfg_h::TRUE;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn get_sos(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a SOS marker */ {
    let mut length: crate::jpegint_h::JLONG = 0; /* Number of components */
    let mut i: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut pi: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if (*(*cinfo).marker).saw_SOF == 0 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_SOS_NO_SOF as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh11 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh11 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh12 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh12 as libc::c_long;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh13 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    n = *fresh13 as libc::c_int;
    (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_SOS as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0] = n;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    if length != (n * 2i32 + 6i32) as libc::c_long
        || n < 1i32
        || n > crate::jpeglib_h::MAX_COMPS_IN_SCAN
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*cinfo).comps_in_scan = n;
    /* Collect the component-spec parameters */
    i = 0i32;
    while i < crate::jpeglib_h::MAX_COMPS_IN_SCAN {
        (*cinfo).cur_comp_info[i as usize] =
            crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_component_info;
        i += 1
    }
    i = 0i32;
    while i < n {
        let mut current_block_80: u64;
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh14 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        cc = *fresh14 as libc::c_int;
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh15 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh15 as libc::c_int;
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        loop {
            if !(ci < (*cinfo).num_components && ci < crate::jpeglib_h::MAX_COMPS_IN_SCAN) {
                current_block_80 = 5181772461570869434;
                break;
            }
            if cc == (*compptr).component_id && (*cinfo).cur_comp_info[ci as usize].is_null() {
                current_block_80 = 4265548930007008436;
                break;
            }
            ci += 1;
            compptr = compptr.offset(1)
        }
        match current_block_80 {
            5181772461570869434 => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_COMPONENT_ID as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = cc;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            _ => {}
        }
        (*cinfo).cur_comp_info[i as usize] = compptr;
        (*compptr).dc_tbl_no = c >> 4i32 & 15i32;
        (*compptr).ac_tbl_no = c & 15i32;
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0) = cc;
        *_mp.offset(1) = (*compptr).dc_tbl_no;
        *_mp.offset(2) = (*compptr).ac_tbl_no;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_SOS_COMPONENT as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
        /* This CSi (cc) should differ from the previous CSi */
        pi = 0i32;
        while pi < i {
            if (*cinfo).cur_comp_info[pi as usize] == compptr {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_COMPONENT_ID as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = cc;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            pi += 1
        }
        i += 1
    }
    /* Collect the additional scan parameters Ss, Se, Ah/Al. */
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh16 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh16 as libc::c_int;
    (*cinfo).Ss = c;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh17 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh17 as libc::c_int;
    (*cinfo).Se = c;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh18 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh18 as libc::c_int;
    (*cinfo).Ah = c >> 4i32 & 15i32;
    (*cinfo).Al = c & 15i32;
    let mut _mp_0: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
    *_mp_0.offset(0) = (*cinfo).Ss;
    *_mp_0.offset(1) = (*cinfo).Se;
    *_mp_0.offset(2) = (*cinfo).Ah;
    *_mp_0.offset(3) = (*cinfo).Al;
    (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_SOS_PARAMS as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    /* Prepare to scan data & restart markers */
    (*(*cinfo).marker).next_restart_num = 0i32;
    /* Count another SOS marker */
    (*cinfo).input_scan_number += 1;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return crate::jmorecfg_h::TRUE;
}
/* !D_ARITH_CODING_SUPPORTED */
/* D_ARITH_CODING_SUPPORTED */

unsafe extern "C" fn get_dht(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a DHT marker */ {
    let mut length: crate::jpegint_h::JLONG = 0;
    let mut bits: [crate::jmorecfg_h::UINT8; 17] = [0; 17];
    let mut huffval: [crate::jmorecfg_h::UINT8; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut htblptr: *mut *mut crate::jpeglib_h::JHUFF_TBL =
        ::std::ptr::null_mut::< *mut crate::jpeglib_h::JHUFF_TBL>();
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh19 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh19 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh20 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh20 as libc::c_long;
    length -= 2i32 as libc::c_long;
    while length > 16i32 as libc::c_long {
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh21 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        index = *fresh21 as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_DHT as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = index;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
        bits[0] = 0i32 as crate::jmorecfg_h::UINT8;
        count = 0i32;
        i = 1i32;
        while i <= 16i32 {
            if bytes_in_buffer == 0i32 as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0i32;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh22 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            bits[i as usize] = *fresh22;
            count += bits[i as usize] as libc::c_int;
            i += 1
        }
        length -= (1i32 + 16i32) as libc::c_long;
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0) = bits[1] as libc::c_int;
        *_mp.offset(1) = bits[2] as libc::c_int;
        *_mp.offset(2) = bits[3] as libc::c_int;
        *_mp.offset(3) = bits[4] as libc::c_int;
        *_mp.offset(4) = bits[5] as libc::c_int;
        *_mp.offset(5) = bits[6] as libc::c_int;
        *_mp.offset(6) = bits[7] as libc::c_int;
        *_mp.offset(7) = bits[8] as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_HUFFBITS as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 2i32);
        let mut _mp_0: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp_0.offset(0) = bits[9] as libc::c_int;
        *_mp_0.offset(1) = bits[10] as libc::c_int;
        *_mp_0.offset(2) = bits[11] as libc::c_int;
        *_mp_0.offset(3) = bits[12] as libc::c_int;
        *_mp_0.offset(4) = bits[13] as libc::c_int;
        *_mp_0.offset(5) = bits[14] as libc::c_int;
        *_mp_0.offset(6) = bits[15] as libc::c_int;
        *_mp_0.offset(7) = bits[16] as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_HUFFBITS as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 2i32);
        /* Here we just do minimal validation of the counts to avoid walking
         * off the end of our table space.  jdhuff.c will check more carefully.
         */
        if count > 256i32 || count as crate::jpegint_h::JLONG > length {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_HUFF_TABLE as libc::c_int; /* DC table definition */
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        i = 0i32;
        while i < count {
            if bytes_in_buffer == 0i32 as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0i32;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh23 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            huffval[i as usize] = *fresh23;
            i += 1
        }
        crate::stdlib::memset(
            &mut *huffval.as_mut_ptr().offset(count as isize) as *mut crate::jmorecfg_h::UINT8
                as *mut libc::c_void,
            0i32,
            ((256i32 - count) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::UINT8>() as libc::c_ulong),
        );
        length -= count as libc::c_long;
        if index & 0x10i32 != 0 {
            /* AC table definition */
            index -= 0x10i32;
            if index < 0i32 || index >= crate::jpeglib_h::NUM_HUFF_TBLS {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_DHT_INDEX as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = index;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            htblptr = &mut *(*cinfo)
                .ac_huff_tbl_ptrs
                .as_mut_ptr()
                .offset(index as isize)
                as *mut *mut crate::jpeglib_h::JHUFF_TBL
        } else {
            if index < 0i32 || index >= crate::jpeglib_h::NUM_HUFF_TBLS {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_DHT_INDEX as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = index;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            htblptr = &mut *(*cinfo)
                .dc_huff_tbl_ptrs
                .as_mut_ptr()
                .offset(index as isize)
                as *mut *mut crate::jpeglib_h::JHUFF_TBL
        }
        if (*htblptr).is_null() {
            *htblptr =
                crate::jpeglib_h::jpeg_alloc_huff_table(cinfo as crate::jpeglib_h::j_common_ptr)
        }
        crate::stdlib::memcpy(
            (**htblptr).bits.as_mut_ptr() as *mut libc::c_void,
            bits.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 17]>() as libc::c_ulong,
        );
        crate::stdlib::memcpy(
            (**htblptr).huffval.as_mut_ptr() as *mut libc::c_void,
            huffval.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 256]>() as libc::c_ulong,
        );
    }
    if length != 0i32 as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn get_dqt(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a DQT marker */ {
    let mut length: crate::jpegint_h::JLONG = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prec: libc::c_int = 0;
    let mut tmp: libc::c_uint = 0;
    let mut quant_ptr: *mut crate::jpeglib_h::JQUANT_TBL = ::std::ptr::null_mut::< crate::jpeglib_h::JQUANT_TBL>();
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh24 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh24 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh25 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh25 as libc::c_long;
    length -= 2i32 as libc::c_long;
    while length > 0i32 as libc::c_long {
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh26 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        n = *fresh26 as libc::c_int;
        prec = n >> 4i32;
        n &= 0xfi32;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_DQT as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = n;
        (*(*cinfo).err).msg_parm.i[1] = prec;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
        if n >= crate::jpeglib_h::NUM_QUANT_TBLS {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_DQT_INDEX as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = n;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if (*cinfo).quant_tbl_ptrs[n as usize].is_null() {
            (*cinfo).quant_tbl_ptrs[n as usize] =
                crate::jpeglib_h::jpeg_alloc_quant_table(cinfo as crate::jpeglib_h::j_common_ptr)
        }
        quant_ptr = (*cinfo).quant_tbl_ptrs[n as usize];
        i = 0i32;
        while i < crate::jpeglib_h::DCTSIZE2 {
            if prec != 0 {
                if bytes_in_buffer == 0i32 as libc::c_ulong {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0i32;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh27 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = (*fresh27 as libc::c_uint) << 8i32;
                if bytes_in_buffer == 0i32 as libc::c_ulong {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0i32;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh28 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = tmp.wrapping_add(*fresh28 as libc::c_uint)
            } else {
                if bytes_in_buffer == 0i32 as libc::c_ulong {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0i32;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh29 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = *fresh29 as libc::c_uint
            }
            /* We convert the zigzag-order table to natural array order. */
            (*quant_ptr).quantval[*crate::jpegint_h::jpeg_natural_order
                .as_ptr()
                .offset(i as isize) as usize] = tmp as crate::jmorecfg_h::UINT16;
            i += 1
        }
        if (*(*cinfo).err).trace_level >= 2i32 {
            i = 0i32;
            while i < crate::jpeglib_h::DCTSIZE2 {
                let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
                *_mp.offset(0) = (*quant_ptr).quantval[i as usize] as libc::c_int;
                *_mp.offset(1) = (*quant_ptr).quantval[(i + 1i32) as usize] as libc::c_int;
                *_mp.offset(2) = (*quant_ptr).quantval[(i + 2i32) as usize] as libc::c_int;
                *_mp.offset(3) = (*quant_ptr).quantval[(i + 3i32) as usize] as libc::c_int;
                *_mp.offset(4) = (*quant_ptr).quantval[(i + 4i32) as usize] as libc::c_int;
                *_mp.offset(5) = (*quant_ptr).quantval[(i + 5i32) as usize] as libc::c_int;
                *_mp.offset(6) = (*quant_ptr).quantval[(i + 6i32) as usize] as libc::c_int;
                *_mp.offset(7) = (*quant_ptr).quantval[(i + 7i32) as usize] as libc::c_int;
                (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_QUANTVALS as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr, 2i32
                );
                i += 8i32
            }
        }
        length -= (crate::jpeglib_h::DCTSIZE2 + 1i32) as libc::c_long;
        if prec != 0 {
            length -= crate::jpeglib_h::DCTSIZE2 as libc::c_long
        }
    }
    if length != 0i32 as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn get_dri(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a DRI marker */ {
    let mut length: crate::jpegint_h::JLONG = 0;
    let mut tmp: libc::c_uint = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh30 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh30 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh31 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh31 as libc::c_long;
    if length != 4i32 as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh32 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    tmp = (*fresh32 as libc::c_uint) << 8i32;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh33 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    tmp = tmp.wrapping_add(*fresh33 as libc::c_uint);
    (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_DRI as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0] = tmp as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    (*cinfo).restart_interval = tmp;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return crate::jmorecfg_h::TRUE;
}
/*
 * Routines for processing APPn and COM markers.
 * These are either saved in memory or discarded, per application request.
 * APP0 and APP14 are specially checked to see if they are
 * JFIF and Adobe markers, respectively.
 */

pub const APP0_DATA_LEN: libc::c_int = 14i32;
/* Length of interesting data in APP0 */

pub const APP14_DATA_LEN: libc::c_int = 12i32;
/* Length of interesting data in APP14 */

pub const APPN_DATA_LEN: libc::c_int = 14i32;
/* Must be the largest of the above!! */

unsafe extern "C" fn examine_app0(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut data: *mut crate::jmorecfg_h::JOCTET,
    mut datalen: libc::c_uint,
    mut remaining: crate::jpegint_h::JLONG,
)
/* Examine first few bytes from an APP0.
 * Take appropriate action if it is a JFIF marker.
 * datalen is # of bytes at data[], remaining is length of rest of marker data.
 */
{
    let mut totallen: crate::jpegint_h::JLONG = datalen as crate::jpegint_h::JLONG + remaining;
    if datalen >= APP0_DATA_LEN as libc::c_uint
        && *data.offset(0) as libc::c_int == 0x4ai32
        && *data.offset(1) as libc::c_int == 0x46i32
        && *data.offset(2) as libc::c_int == 0x49i32
        && *data.offset(3) as libc::c_int == 0x46i32
        && *data.offset(4) as libc::c_int == 0i32
    {
        /* Found JFIF APP0 marker: save info */
        (*cinfo).saw_JFIF_marker = crate::jmorecfg_h::TRUE;
        (*cinfo).JFIF_major_version = *data.offset(5);
        (*cinfo).JFIF_minor_version = *data.offset(6);
        (*cinfo).density_unit = *data.offset(7);
        (*cinfo).X_density = (((*data.offset(8) as libc::c_int) << 8i32)
            + *data.offset(9) as libc::c_int)
            as crate::jmorecfg_h::UINT16;
        (*cinfo).Y_density = (((*data.offset(10) as libc::c_int) << 8i32)
            + *data.offset(11) as libc::c_int)
            as crate::jmorecfg_h::UINT16;
        /* Check version.
         * Major version must be 1, anything else signals an incompatible change.
         * (We used to treat this as an error, but now it's a nonfatal warning,
         * because some bozo at Hijaak couldn't read the spec.)
         * Minor version should be 0..2, but process anyway if newer.
         */
        if (*cinfo).JFIF_major_version as libc::c_int != 1i32 {
            (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_JFIF_MAJOR as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = (*cinfo).JFIF_major_version as libc::c_int;
            (*(*cinfo).err).msg_parm.i[1] = (*cinfo).JFIF_minor_version as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr, -1i32
            );
        }
        /* Generate trace messages */
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0) = (*cinfo).JFIF_major_version as libc::c_int;
        *_mp.offset(1) = (*cinfo).JFIF_minor_version as libc::c_int;
        *_mp.offset(2) = (*cinfo).X_density as libc::c_int;
        *_mp.offset(3) = (*cinfo).Y_density as libc::c_int;
        *_mp.offset(4) = (*cinfo).density_unit as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_JFIF as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
        /* Validate thumbnail dimensions and issue appropriate messages */
        if *data.offset(12) as libc::c_int | *data.offset(13) as libc::c_int != 0 {
            (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_JFIF_THUMBNAIL as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = *data.offset(12) as libc::c_int;
            (*(*cinfo).err).msg_parm.i[1] = *data.offset(13) as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr, 1i32
            );
        }
        totallen -= APP0_DATA_LEN as libc::c_long;
        if totallen
            != *data.offset(12) as crate::jpegint_h::JLONG
                * *data.offset(13) as crate::jpegint_h::JLONG
                * 3i32 as crate::jpegint_h::JLONG
        {
            (*(*cinfo).err).msg_code =
                crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = totallen as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr, 1i32
            );
        }
    } else if datalen >= 6i32 as libc::c_uint
        && *data.offset(0) as libc::c_int == 0x4ai32
        && *data.offset(1) as libc::c_int == 0x46i32
        && *data.offset(2) as libc::c_int == 0x58i32
        && *data.offset(3) as libc::c_int == 0x58i32
        && *data.offset(4) as libc::c_int == 0i32
    {
        /* Found JFIF "JFXX" extension APP0 marker */
        /* The library doesn't actually do anything with these,
         * but we try to produce a helpful trace message.
         */
        match *data.offset(5) as libc::c_int {
            16 => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_THUMB_JPEG as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr, 1i32
                );
            }
            17 => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_THUMB_PALETTE as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr, 1i32
                );
            }
            19 => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_THUMB_RGB as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr, 1i32
                );
            }
            _ => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_JFIF_EXTENSION as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = *data.offset(5) as libc::c_int;
                (*(*cinfo).err).msg_parm.i[1] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr, 1i32
                );
            }
        }
    } else {
        /* Start of APP0 does not match "JFIF" or "JFXX", or too short */
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_APP0 as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = totallen as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    };
}

unsafe extern "C" fn examine_app14(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut data: *mut crate::jmorecfg_h::JOCTET,
    mut datalen: libc::c_uint,
    mut remaining: crate::jpegint_h::JLONG,
)
/* Examine first few bytes from an APP14.
 * Take appropriate action if it is an Adobe marker.
 * datalen is # of bytes at data[], remaining is length of rest of marker data.
 */
{
    let mut version: libc::c_uint = 0;
    let mut flags0: libc::c_uint = 0;
    let mut flags1: libc::c_uint = 0;
    let mut transform: libc::c_uint = 0;
    if datalen >= APP14_DATA_LEN as libc::c_uint
        && *data.offset(0) as libc::c_int == 0x41i32
        && *data.offset(1) as libc::c_int == 0x64i32
        && *data.offset(2) as libc::c_int == 0x6fi32
        && *data.offset(3) as libc::c_int == 0x62i32
        && *data.offset(4) as libc::c_int == 0x65i32
    {
        /* Found Adobe APP14 marker */
        version = (((*data.offset(5) as libc::c_int) << 8i32) + *data.offset(6) as libc::c_int)
            as libc::c_uint;
        flags0 = (((*data.offset(7) as libc::c_int) << 8i32) + *data.offset(8) as libc::c_int)
            as libc::c_uint;
        flags1 = (((*data.offset(9) as libc::c_int) << 8i32) + *data.offset(10) as libc::c_int)
            as libc::c_uint;
        transform = *data.offset(11) as libc::c_uint;
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0) = version as libc::c_int;
        *_mp.offset(1) = flags0 as libc::c_int;
        *_mp.offset(2) = flags1 as libc::c_int;
        *_mp.offset(3) = transform as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_ADOBE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
        (*cinfo).saw_Adobe_marker = crate::jmorecfg_h::TRUE;
        (*cinfo).Adobe_transform = transform as crate::jmorecfg_h::UINT8
    } else {
        /* Start of APP14 does not match "Adobe", or too short */
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_APP14 as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (datalen as libc::c_long + remaining) as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    };
}

unsafe extern "C" fn get_interesting_appn(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process an APP0 or APP14 marker without saving it */ {
    let mut length: crate::jpegint_h::JLONG = 0;
    let mut b: [crate::jmorecfg_h::JOCTET; 14] = [0; 14];
    let mut i: libc::c_uint = 0;
    let mut numtoread: libc::c_uint = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh34 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh34 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh35 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh35 as libc::c_long;
    length -= 2i32 as libc::c_long;
    /* get the interesting part of the marker data */
    if length >= APPN_DATA_LEN as libc::c_long {
        numtoread = APPN_DATA_LEN as libc::c_uint
    } else if length > 0i32 as libc::c_long {
        numtoread = length as libc::c_uint
    } else {
        numtoread = 0i32 as libc::c_uint
    }
    i = 0i32 as libc::c_uint;
    while i < numtoread {
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh36 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        b[i as usize] = *fresh36;
        i = i.wrapping_add(1)
    }
    length -= numtoread as libc::c_long;
    /* process it */
    match (*cinfo).unread_marker {
        224 => {
            examine_app0(cinfo, b.as_mut_ptr(), numtoread, length);
        }
        238 => {
            examine_app14(cinfo, b.as_mut_ptr(), numtoread, length);
        }
        _ => {
            /* can't get here unless jpeg_save_markers chooses wrong processor */
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = (*cinfo).unread_marker;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    /* skip any remaining data -- could be lots */
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0i32 as libc::c_long {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn save_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Save an APPn or COM marker into the marker list */ {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut cur_marker: crate::jpeglib_h::jpeg_saved_marker_ptr = (*marker).cur_marker;
    let mut bytes_read: libc::c_uint = 0;
    let mut data_length: libc::c_uint = 0;
    let mut data: *mut crate::jmorecfg_h::JOCTET = ::std::ptr::null_mut::< crate::jmorecfg_h::JOCTET>();
    let mut length: crate::jpegint_h::JLONG = 0i32 as crate::jpegint_h::JLONG;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if cur_marker.is_null() {
        /* begin reading a marker */
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh37 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        length = ((*fresh37 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh38 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        length += *fresh38 as libc::c_long;
        length -= 2i32 as libc::c_long;
        if length >= 0i32 as libc::c_long {
            /* watch out for bogus length word */
            /* figure out how much we want to save */
            let mut limit: libc::c_uint = 0;
            if (*cinfo).unread_marker == M_COM as libc::c_int {
                limit = (*marker).length_limit_COM
            } else {
                limit = (*marker).length_limit_APPn
                    [((*cinfo).unread_marker - M_APP0 as libc::c_int) as usize]
            }
            if (length as libc::c_uint) < limit {
                limit = length as libc::c_uint
            }
            /* allocate and initialize the marker item */
            cur_marker = Some(
                (*(*cinfo).mem)
                    .alloc_large
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                crate::jpeglib_h::JPOOL_IMAGE,
                (::std::mem::size_of::<crate::jpeglib_h::jpeg_marker_struct>() as libc::c_ulong)
                    .wrapping_add(limit as libc::c_ulong),
            ) as crate::jpeglib_h::jpeg_saved_marker_ptr;
            (*cur_marker).next = crate::stddef_h::NULL as crate::jpeglib_h::jpeg_saved_marker_ptr;
            (*cur_marker).marker = (*cinfo).unread_marker as crate::jmorecfg_h::UINT8;
            (*cur_marker).original_length = length as libc::c_uint;
            (*cur_marker).data_length = limit;
            /* data area is just beyond the jpeg_marker_struct */
            (*cur_marker).data = cur_marker.offset(1) as *mut crate::jmorecfg_h::JOCTET;
            data = (*cur_marker).data;
            (*marker).cur_marker = cur_marker;
            (*marker).bytes_read = 0i32 as libc::c_uint;
            bytes_read = 0i32 as libc::c_uint;
            data_length = limit
        } else {
            /* deal with bogus length word */
            data_length = 0i32 as libc::c_uint;
            bytes_read = data_length;
            data = crate::stddef_h::NULL as *mut crate::jmorecfg_h::JOCTET
        }
    } else {
        /* resume reading a marker */
        bytes_read = (*marker).bytes_read; /* move the restart point to here */
        data_length = (*cur_marker).data_length;
        data = (*cur_marker).data.offset(bytes_read as isize)
    }
    while bytes_read < data_length {
        (*datasrc).next_input_byte = next_input_byte;
        (*datasrc).bytes_in_buffer = bytes_in_buffer;
        (*marker).bytes_read = bytes_read;
        /* If there's not at least one byte in buffer, suspend */
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        /* Copy bytes with reasonable rapidity */
        while bytes_read < data_length && bytes_in_buffer > 0i32 as libc::c_ulong {
            let fresh39 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            let fresh40 = data;
            data = data.offset(1);
            *fresh40 = *fresh39;
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            bytes_read = bytes_read.wrapping_add(1)
        }
    }
    /* Done reading what we want to read */
    if !cur_marker.is_null() {
        /* will be NULL if bogus length word */
        /* Add new marker to end of list */
        if (*cinfo).marker_list.is_null() {
            (*cinfo).marker_list = cur_marker
        } else {
            let mut prev: crate::jpeglib_h::jpeg_saved_marker_ptr = (*cinfo).marker_list;
            while !(*prev).next.is_null() {
                prev = (*prev).next
            }
            (*prev).next = cur_marker
        }
        /* Reset pointer & calc remaining data length */
        data = (*cur_marker).data;
        length = (*cur_marker).original_length.wrapping_sub(data_length) as crate::jpegint_h::JLONG
    }
    /* Reset to initial state for next marker */
    (*marker).cur_marker = crate::stddef_h::NULL as crate::jpeglib_h::jpeg_saved_marker_ptr;
    /* Process the marker if interesting; else just make a generic trace msg */
    match (*cinfo).unread_marker {
        224 => {
            examine_app0(cinfo, data, data_length, length);
        }
        238 => {
            examine_app14(cinfo, data, data_length, length);
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_MISC_MARKER as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0] = (*cinfo).unread_marker;
            (*(*cinfo).err).msg_parm.i[1] = (data_length as libc::c_long + length) as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr, 1i32
            );
        }
    }
    /* skip any remaining data -- could be lots */
    (*datasrc).next_input_byte = next_input_byte; /* do before skip_input_data */
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0i32 as libc::c_long {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return crate::jmorecfg_h::TRUE;
}
/* SAVE_MARKERS_SUPPORTED */

unsafe extern "C" fn skip_variable(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Skip over an unknown or uninteresting variable-length marker */ {
    let mut length: crate::jpegint_h::JLONG = 0; /* do before skip_input_data */
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh41 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh41 as libc::c_uint) << 8i32) as crate::jpegint_h::JLONG;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh42 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh42 as libc::c_long;
    length -= 2i32 as libc::c_long;
    (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_MISC_MARKER as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0] = (*cinfo).unread_marker;
    (*(*cinfo).err).msg_parm.i[1] = length as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0i32 as libc::c_long {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return crate::jmorecfg_h::TRUE;
}
/*
 * Find the next JPEG marker, save it in cinfo->unread_marker.
 * Returns FALSE if had to suspend before reaching a marker;
 * in that case cinfo->unread_marker is unchanged.
 *
 * Note that the result might not be a valid marker code,
 * but it will never be 0 or FF.
 */

unsafe extern "C" fn next_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut c: libc::c_int = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    loop {
        if bytes_in_buffer == 0i32 as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0i32;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh43 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh43 as libc::c_int;
        /* Skip any non-FF bytes.
         * This may look a bit inefficient, but it will not occur in a valid file.
         * We sync after each discarded byte so that a suspending data source
         * can discard the byte from its buffer.
         */
        while c != 0xffi32 {
            (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker).discarded_bytes.wrapping_add(1);
            (*datasrc).next_input_byte = next_input_byte;
            (*datasrc).bytes_in_buffer = bytes_in_buffer;
            if bytes_in_buffer == 0i32 as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0i32;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh44 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh44 as libc::c_int
        }
        loop
        /* This loop swallows any duplicate FF bytes.  Extra FFs are legal as
         * pad bytes, so don't count them in discarded_bytes.  We assume there
         * will not be so many consecutive FF bytes as to overflow a suspending
         * data source's input buffer.
         */
        {
            if bytes_in_buffer == 0i32 as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0i32;
                } /* found a valid marker, exit loop */
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh45 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh45 as libc::c_int;
            if !(c == 0xffi32) {
                break;
            }
        }
        if c != 0i32 {
            break;
        }
        /* Reach here if we found a stuffed-zero data sequence (FF/00).
         * Discard it and loop back to try again.
         */
        (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
            .discarded_bytes
            .wrapping_add(2i32 as libc::c_uint);
        (*datasrc).next_input_byte = next_input_byte;
        (*datasrc).bytes_in_buffer = bytes_in_buffer
    }
    if (*(*cinfo).marker).discarded_bytes != 0i32 as libc::c_uint {
        (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_EXTRANEOUS_DATA as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*(*cinfo).marker).discarded_bytes as libc::c_int;
        (*(*cinfo).err).msg_parm.i[1] = c;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, -1i32);
        (*(*cinfo).marker).discarded_bytes = 0i32 as libc::c_uint
    }
    (*cinfo).unread_marker = c;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn first_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Like next_marker, but used to obtain the initial SOI marker. */
/* For this marker, we do not allow preceding garbage or fill; otherwise,
 * we might well scan an entire input file before realizing it ain't JPEG.
 * If an application wants to process non-JFIF files, it must seek to the
 * SOI before calling the JPEG library.
 */ {
    let mut c: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh46 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh46 as libc::c_int;
    if bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0i32;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh47 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c2 = *fresh47 as libc::c_int;
    if c != 0xffi32 || c2 != M_SOI as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_NO_SOI as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = c;
        (*(*cinfo).err).msg_parm.i[1] = c2;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*cinfo).unread_marker = c2;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return crate::jmorecfg_h::TRUE;
}
/*
 * Read markers until SOS or EOI.
 *
 * Returns same codes as are defined for jpeg_consume_input:
 * JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 */

unsafe extern "C" fn read_markers(mut cinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int {
    loop
    /* Outer loop repeats once for each marker. */
    /* Collect the marker proper, unless we already did. */
    /* NB: first_marker() enforces the requirement that SOI appear first. */
    {
        if (*cinfo).unread_marker == 0i32 {
            if (*(*cinfo).marker).saw_SOI == 0 {
                if first_marker(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
            } else if next_marker(cinfo) == 0 {
                return crate::jpeglib_h::JPEG_SUSPENDED;
            }
        }
        let mut current_block_42: u64;
        /* At this point cinfo->unread_marker contains the marker code and the
         * input point is just past the marker proper, but before any parameters.
         * A suspension will cause us to return with this state still true.
         */
        match (*cinfo).unread_marker {
            216 => {
                if get_soi(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            192 | 193 => {
                /* Baseline */
                /* Extended sequential, Huffman */
                if get_sof(cinfo, crate::jmorecfg_h::FALSE, crate::jmorecfg_h::FALSE) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            194 => {
                /* Progressive, Huffman */
                if get_sof(cinfo, crate::jmorecfg_h::TRUE, crate::jmorecfg_h::FALSE) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            201 => {
                /* Extended sequential, arithmetic */
                if get_sof(cinfo, crate::jmorecfg_h::FALSE, crate::jmorecfg_h::TRUE) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            202 => {
                /* Progressive, arithmetic */
                if get_sof(cinfo, crate::jmorecfg_h::TRUE, crate::jmorecfg_h::TRUE) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            195 => {
                /* Lossless, Huffman */
                current_block_42 = 16427644017298241796; /* processed the marker */
            }
            197 => {
                current_block_42 = 16427644017298241796; /* processed the marker */
            }
            198 => {
                current_block_42 = 12574308213030772164;
            }
            199 => {
                current_block_42 = 10328241491981538519;
            }
            200 => {
                current_block_42 = 5792855011569847122;
            }
            203 => {
                current_block_42 = 12865972141859106327;
            }
            205 => {
                current_block_42 = 17801610534491083400;
            }
            206 | 207 => {
                current_block_42 = 5221305508586845081;
            }
            218 => {
                if get_sos(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                (*cinfo).unread_marker = 0i32;
                return crate::jpeglib_h::JPEG_REACHED_SOS;
            }
            217 => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_EOI as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr, 1i32
                );
                (*cinfo).unread_marker = 0i32;
                return crate::jpeglib_h::JPEG_REACHED_EOI;
            }
            204 => {
                if skip_variable(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            196 => {
                if get_dht(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            219 => {
                if get_dqt(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            221 => {
                if get_dri(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237
            | 238 | 239 => {
                if Some(
                    (*(*((*cinfo).marker as my_marker_ptr))
                        .process_APPn
                        .as_mut_ptr()
                        .offset(((*cinfo).unread_marker - M_APP0 as libc::c_int) as isize))
                    .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            254 => {
                if Some(
                    (*((*cinfo).marker as my_marker_ptr))
                        .process_COM
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 1 => {
                /* these are all parameterless */
                (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_PARMLESS_MARKER as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr, 1i32
                );
                current_block_42 = 15594603006322722090;
            }
            220 => {
                /* Ignore DNL ... perhaps the wrong thing */
                if skip_variable(cinfo) == 0 {
                    return crate::jpeglib_h::JPEG_SUSPENDED;
                }
                current_block_42 = 15594603006322722090;
            }
            _ => {
                /* must be DHP, EXP, JPGn, or RESn */
                /* For now, we treat the reserved markers as fatal errors since they are
                 * likely to be used to signal incompatible JPEG Part 3 extensions.
                 * Once the JPEG 3 version-number marker is well defined, this code
                 * ought to change!
                 */
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
                current_block_42 = 15594603006322722090;
            }
        }
        match current_block_42 {
            16427644017298241796 =>
            /* Differential sequential, Huffman */
            {
                current_block_42 = 12574308213030772164;
            }
            _ => {}
        }
        match current_block_42 {
            12574308213030772164 =>
            /* Differential progressive, Huffman */
            {
                current_block_42 = 10328241491981538519;
            }
            _ => {}
        }
        match current_block_42 {
            10328241491981538519 =>
            /* Differential lossless, Huffman */
            {
                current_block_42 = 5792855011569847122;
            }
            _ => {}
        }
        match current_block_42 {
            5792855011569847122 =>
            /* Reserved for JPEG extensions */
            {
                current_block_42 = 12865972141859106327;
            }
            _ => {}
        }
        match current_block_42 {
            12865972141859106327 =>
            /* Lossless, arithmetic */
            {
                current_block_42 = 17801610534491083400;
            }
            _ => {}
        }
        match current_block_42 {
            17801610534491083400 =>
            /* Differential sequential, arithmetic */
            {
                current_block_42 = 5221305508586845081;
            }
            _ => {}
        }
        match current_block_42 {
            5221305508586845081 =>
            /* Differential progressive, arithmetic */
            /* Differential lossless, arithmetic */
            {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_SOF_UNSUPPORTED as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            _ => {}
        }
        /* Successfully processed marker, so reset state variable */
        (*cinfo).unread_marker = 0i32
    }
    /* end loop */
}
/*
 * Read a restart marker, which is expected to appear next in the datastream;
 * if the marker is not there, take appropriate recovery action.
 * Returns FALSE if suspension is required.
 *
 * This is called by the entropy decoder after it has read an appropriate
 * number of MCUs.  cinfo->unread_marker may be nonzero if the entropy decoder
 * has already read a marker from the data source.  Under normal conditions
 * cinfo->unread_marker will be reset to 0 before returning; if not reset,
 * it holds a marker which the decoder will be unable to read past.
 */

unsafe extern "C" fn read_restart_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    /* Obtain a marker unless we already did. */
    /* Note that next_marker will complain if it skips any data. */
    if (*cinfo).unread_marker == 0i32 {
        if next_marker(cinfo) == 0 {
            return crate::jmorecfg_h::FALSE;
        }
    }
    if (*cinfo).unread_marker == M_RST0 as libc::c_int + (*(*cinfo).marker).next_restart_num {
        /* Normal case --- swallow the marker and let entropy decoder continue */
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_RST as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*(*cinfo).marker).next_restart_num;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 3i32);
        (*cinfo).unread_marker = 0i32
    } else if Some(
        (*(*cinfo).src)
            .resync_to_restart
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, (*(*cinfo).marker).next_restart_num)
        == 0
    {
        return crate::jmorecfg_h::FALSE;
    }
    /* Uh-oh, the restart markers have been messed up. */
    /* Let the data source manager determine how to resync. */
    /* Update next-restart state */
    (*(*cinfo).marker).next_restart_num = (*(*cinfo).marker).next_restart_num + 1i32 & 7i32;
    return crate::jmorecfg_h::TRUE;
}
/* Default restart-marker-resync procedure for use by data source modules */
/*
 * This is the default resync_to_restart method for data source managers
 * to use if they don't have any better approach.  Some data source managers
 * may be able to back up, or may have additional knowledge about the data
 * which permits a more intelligent recovery strategy; such managers would
 * presumably supply their own resync method.
 *
 * read_restart_marker calls resync_to_restart if it finds a marker other than
 * the restart marker it was expecting.  (This code is *not* used unless
 * a nonzero restart interval has been declared.)  cinfo->unread_marker is
 * the marker code actually found (might be anything, except 0 or FF).
 * The desired restart marker number (0..7) is passed as a parameter.
 * This routine is supposed to apply whatever error recovery strategy seems
 * appropriate in order to position the input stream to the next data segment.
 * Note that cinfo->unread_marker is treated as a marker appearing before
 * the current data-source input point; usually it should be reset to zero
 * before returning.
 * Returns FALSE if suspension is required.
 *
 * This implementation is substantially constrained by wanting to treat the
 * input as a data stream; this means we can't back up.  Therefore, we have
 * only the following actions to work with:
 *   1. Simply discard the marker and let the entropy decoder resume at next
 *      byte of file.
 *   2. Read forward until we find another marker, discarding intervening
 *      data.  (In theory we could look ahead within the current bufferload,
 *      without having to discard data if we don't find the desired marker.
 *      This idea is not implemented here, in part because it makes behavior
 *      dependent on buffer size and chance buffer-boundary positions.)
 *   3. Leave the marker unread (by failing to zero cinfo->unread_marker).
 *      This will cause the entropy decoder to process an empty data segment,
 *      inserting dummy zeroes, and then we will reprocess the marker.
 *
 * #2 is appropriate if we think the desired marker lies ahead, while #3 is
 * appropriate if the found marker is a future restart marker (indicating
 * that we have missed the desired restart marker, probably because it got
 * corrupted).
 * We apply #2 or #3 if the found marker is a restart marker no more than
 * two counts behind or ahead of the expected one.  We also apply #2 if the
 * found marker is not a legal JPEG marker code (it's certainly bogus data).
 * If the found marker is a restart marker more than 2 counts away, we do #1
 * (too much risk that the marker is erroneous; with luck we will be able to
 * resync at some future point).
 * For any valid non-restart JPEG marker, we apply #3.  This keeps us from
 * overrunning the end of a scan.  An implementation limited to single-scan
 * files might find it better to apply #2 for markers other than EOI, since
 * any other marker would have to be bogus data in that case.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_resync_to_restart(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut desired: libc::c_int,
) -> crate::jmorecfg_h::boolean {
    let mut marker: libc::c_int = (*cinfo).unread_marker;
    let mut action: libc::c_int = 1i32;
    /* Always put up a warning. */
    (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_MUST_RESYNC as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0] = marker;
    (*(*cinfo).err).msg_parm.i[1] = desired;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, -1i32);
    loop
    /* Outer loop handles repeated decision after scanning forward. */
    {
        if marker < M_SOF0 as libc::c_int {
            action = 2i32
        } else if marker < M_RST0 as libc::c_int || marker > M_RST7 as libc::c_int {
            /* invalid marker */
            action = 3i32
        } else if marker == M_RST0 as libc::c_int + (desired + 1i32 & 7i32)
            || marker == M_RST0 as libc::c_int + (desired + 2i32 & 7i32)
        {
            /* valid non-restart marker */
            action = 3i32
        } else if marker == M_RST0 as libc::c_int + (desired - 1i32 & 7i32)
            || marker == M_RST0 as libc::c_int + (desired - 2i32 & 7i32)
        {
            /* one of the next two expected restarts */
            action = 2i32
        } else {
            action = 1i32
        } /* a prior restart, so advance */
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_RECOVERY_ACTION as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = marker;
        (*(*cinfo).err).msg_parm.i[1] = action;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 4i32);
        match action {
            1 => {
                /* desired restart or too far away */
                /* Discard marker and let entropy decoder resume processing. */
                (*cinfo).unread_marker = 0i32;
                return crate::jmorecfg_h::TRUE;
            }
            2 => {
                /* Scan to the next marker, and repeat the decision loop. */
                if next_marker(cinfo) == 0 {
                    return crate::jmorecfg_h::FALSE;
                }
                marker = (*cinfo).unread_marker
            }
            3 => {
                /* Return without advancing past this marker. */
                /* Entropy decoder will be forced to process an empty segment. */
                return crate::jmorecfg_h::TRUE;
            }
            _ => {}
        }
    }
    /* end loop */
}
/*
 * Reset marker processing state to begin a fresh datastream.
 */

unsafe extern "C" fn reset_marker_reader(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr; /* until allocated by get_sof */
    (*cinfo).comp_info = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_component_info; /* no SOS seen yet */
    (*cinfo).input_scan_number = 0i32; /* no pending marker */
    (*cinfo).unread_marker = 0i32; /* set internal state too */
    (*marker).pub_0.saw_SOI = crate::jmorecfg_h::FALSE;
    (*marker).pub_0.saw_SOF = crate::jmorecfg_h::FALSE;
    (*marker).pub_0.discarded_bytes = 0i32 as libc::c_uint;
    (*marker).cur_marker = crate::stddef_h::NULL as crate::jpeglib_h::jpeg_saved_marker_ptr;
}
/*
 * Initialize the marker reader module.
 * This is called only once, when the decompression object is created.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_marker_reader(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut marker: my_marker_ptr = ::std::ptr::null_mut::< my_marker_reader>();
    let mut i: libc::c_int = 0;
    /* Create subobject in permanent pool */
    marker = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_PERMANENT,
        ::std::mem::size_of::<my_marker_reader>() as libc::c_ulong,
    ) as my_marker_ptr;
    (*cinfo).marker = marker as *mut crate::jpeglib_h::jpeg_marker_reader;
    /* Initialize public method pointers */
    (*marker).pub_0.reset_marker_reader = Some(
        reset_marker_reader as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*marker).pub_0.read_markers = Some(
        read_markers as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int,
    );
    (*marker).pub_0.read_restart_marker = Some(
        read_restart_marker
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    /* Initialize COM/APPn processing.
     * By default, we examine and then discard APP0 and APP14,
     * but simply discard COM and all other APPn.
     */
    (*marker).process_COM = Some(
        skip_variable
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    (*marker).length_limit_COM = 0i32 as libc::c_uint;
    i = 0i32;
    while i < 16i32 {
        (*marker).process_APPn[i as usize] = Some(
            skip_variable
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        );
        (*marker).length_limit_APPn[i as usize] = 0i32 as libc::c_uint;
        i += 1
    }
    (*marker).process_APPn[0] = Some(
        get_interesting_appn
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    (*marker).process_APPn[14] = Some(
        get_interesting_appn
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    /* Reset marker processing state */
    reset_marker_reader(cinfo);
}
/* Control saving of COM and APPn markers into marker_list. */
/*
 * Control saving of COM and APPn markers into marker_list.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_save_markers(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut marker_code: libc::c_int,
    mut length_limit: libc::c_uint,
) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut maxlength: libc::c_long = 0;
    let mut processor: crate::jpeglib_h::jpeg_marker_parser_method = None;
    /* Length limit mustn't be larger than what we can allocate
     * (should only be a concern in a 16-bit environment).
     */
    maxlength =
        ((*(*cinfo).mem).max_alloc_chunk as libc::c_ulong)
            .wrapping_sub(
                ::std::mem::size_of::<crate::jpeglib_h::jpeg_marker_struct>() as libc::c_ulong,
            ) as libc::c_long;
    if length_limit as libc::c_long > maxlength {
        length_limit = maxlength as libc::c_uint
    }
    /* Choose processor routine to use.
     * APP0/APP14 have special requirements.
     */
    if length_limit != 0 {
        processor = Some(
            save_marker
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        );
        /* If saving APP0/APP14, save at least enough for our internal use. */
        if marker_code == M_APP0 as libc::c_int && length_limit < APP0_DATA_LEN as libc::c_uint {
            length_limit = APP0_DATA_LEN as libc::c_uint
        } else if marker_code == M_APP14 as libc::c_int
            && length_limit < APP14_DATA_LEN as libc::c_uint
        {
            length_limit = APP14_DATA_LEN as libc::c_uint
        }
    } else {
        processor = Some(
            skip_variable
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        );
        /* If discarding APP0/APP14, use our regular on-the-fly processor. */
        if marker_code == M_APP0 as libc::c_int || marker_code == M_APP14 as libc::c_int {
            processor = Some(
                get_interesting_appn
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
    }
    if marker_code == M_COM as libc::c_int {
        (*marker).process_COM = processor;
        (*marker).length_limit_COM = length_limit
    } else if marker_code >= M_APP0 as libc::c_int && marker_code <= M_APP15 as libc::c_int {
        (*marker).process_APPn[(marker_code - M_APP0 as libc::c_int) as usize] = processor;
        (*marker).length_limit_APPn[(marker_code - M_APP0 as libc::c_int) as usize] = length_limit
    } else {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = marker_code;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    };
}
/* Install a special processing method for COM or APPn markers. */
/* SAVE_MARKERS_SUPPORTED */
/*
 * Install a special processing method for COM or APPn markers.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_marker_processor(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut marker_code: libc::c_int,
    mut routine: crate::jpeglib_h::jpeg_marker_parser_method,
) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    if marker_code == M_COM as libc::c_int {
        (*marker).process_COM = routine
    } else if marker_code >= M_APP0 as libc::c_int && marker_code <= M_APP15 as libc::c_int {
        (*marker).process_APPn[(marker_code - M_APP0 as libc::c_int) as usize] = routine
    } else {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = marker_code;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    };
}
