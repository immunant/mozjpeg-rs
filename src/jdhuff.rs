pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jpegint_h::JLONG;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::src::jerror::C2RustUnnamed_3;
pub use crate::stddef_h::size_t;
use libc;

pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
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
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_alloc_huff_table;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
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
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
pub use crate::jstdhuff_c::add_huff_table;
pub use crate::jstdhuff_c::std_huff_tables;
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
pub use crate::stddef_h::NULL;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
// =============== BEGIN jdhuff_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_derived_tbl {
    pub maxcode: [crate::jpegint_h::JLONG; 18],
    pub valoffset: [crate::jpegint_h::JLONG; 18],
    pub pub_0: *mut crate::jpeglib_h::JHUFF_TBL,
    pub lookup: [libc::c_int; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitread_perm_state {
    pub get_buffer: crate::src::jdhuff::bit_buf_type,
    pub bits_left: libc::c_int,
}
/*
 * Fetching the next N bits from the input stream is a time-critical operation
 * for the Huffman decoders.  We implement it with a combination of inline
 * macros and out-of-line subroutines.  Note that N (the number of bits
 * demanded at one time) never exceeds 15 for JPEG use.
 *
 * We read source bytes into get_buffer and dole out bits as needed.
 * If get_buffer already contains enough bits, they are fetched in-line
 * by the macros CHECK_BIT_BUFFER and GET_BITS.  When there aren't enough
 * bits, jpeg_fill_bit_buffer is called; it will attempt to fill get_buffer
 * as full as possible (not just to the number of bits needed; this
 * prefetching reduces the overhead cost of calling jpeg_fill_bit_buffer).
 * Note that jpeg_fill_bit_buffer may return FALSE to indicate suspension.
 * On TRUE return, jpeg_fill_bit_buffer guarantees that get_buffer contains
 * at least the requested number of bits --- dummy zeroes are inserted if
 * necessary.
 */

pub type bit_buf_type = crate::stddef_h::size_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitread_working_state {
    pub next_input_byte: *const crate::jmorecfg_h::JOCTET,
    pub bytes_in_buffer: crate::stddef_h::size_t,
    pub get_buffer: crate::src::jdhuff::bit_buf_type,
    pub bits_left: libc::c_int,
    pub cinfo: crate::jpeglib_h::j_decompress_ptr,
}
/*
 * jdhuff.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010-2011, 2015-2016, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains declarations for Huffman entropy decoding routines
 * that are shared between the sequential decoder (jdhuff.c) and the
 * progressive decoder (jdphuff.c).  No other modules need to see these.
 */
/* Derived data constructed for each Huffman table */

pub const HUFF_LOOKAHEAD: libc::c_int = 8i32;
/* type of bit-extraction buffer */

pub const BIT_BUF_SIZE: libc::c_int = 64i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savable_state {
    pub last_dc_val: [libc::c_int; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct huff_entropy_decoder {
    pub pub_0: crate::jpeglib_h::jpeg_entropy_decoder,
    pub bitstate: crate::src::jdhuff::bitread_perm_state,
    pub saved: savable_state,
    pub restarts_to_go: libc::c_uint,
    pub dc_derived_tbls: [*mut crate::src::jdhuff::d_derived_tbl; 4],
    pub ac_derived_tbls: [*mut crate::src::jdhuff::d_derived_tbl; 4],
    pub dc_cur_tbls: [*mut crate::src::jdhuff::d_derived_tbl; 10],
    pub ac_cur_tbls: [*mut crate::src::jdhuff::d_derived_tbl; 10],
    pub dc_needed: [crate::jmorecfg_h::boolean; 10],
    pub ac_needed: [crate::jmorecfg_h::boolean; 10],
}

pub type huff_entropy_ptr = *mut huff_entropy_decoder;
/*
 * Initialize for a Huffman-compressed scan.
 */

unsafe extern "C" fn start_pass_huff_decoder(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
       let mut compptr:  *mut crate::jpeglib_h::jpeg_component_info =
    
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    
    
    
    
    
    
    /* Check that the scan parameters Ss, Se, Ah/Al are OK for sequential JPEG.
     * This ought to be an error condition, but we make it a warning because
     * there are some baseline files out there with all zeroes in these bytes.
     */
    if (*cinfo).Ss != 0i32
        || (*cinfo).Se != crate::jpeglib_h::DCTSIZE2 - 1i32
        || (*cinfo).Ah != 0i32
        || (*cinfo).Al != 0i32
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_NOT_SEQUENTIAL as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, -1i32);
    }
     let mut ci:   libc::c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
           compptr = (*cinfo).cur_comp_info[ci as usize];
        
        
         let mut dctbl:   libc::c_int =  (*compptr).dc_tbl_no; let mut actbl:   libc::c_int =  (*compptr).ac_tbl_no; let mut pdtbl:   *mut *mut crate::src::jdhuff::d_derived_tbl =
     (*entropy)
            .dc_derived_tbls
            .as_mut_ptr()
            .offset(dctbl as isize);
        jpeg_make_d_derived_tbl(cinfo, crate::jmorecfg_h::TRUE, dctbl, pdtbl);
        pdtbl = (*entropy)
            .ac_derived_tbls
            .as_mut_ptr()
            .offset(actbl as isize);
        jpeg_make_d_derived_tbl(cinfo, crate::jmorecfg_h::FALSE, actbl, pdtbl);
        /* Initialize DC predictions to 0 */
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
     let mut blkn:   libc::c_int =  0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* Precalculate which table to use for each block */
        (*entropy).dc_cur_tbls[blkn as usize] =
            (*entropy).dc_derived_tbls[(*compptr).dc_tbl_no as usize];
        (*entropy).ac_cur_tbls[blkn as usize] =
            (*entropy).ac_derived_tbls[(*compptr).ac_tbl_no as usize];
        /* Decide whether we really care about the coefficient values */
        if (*compptr).component_needed != 0 {
            (*entropy).dc_needed[blkn as usize] = crate::jmorecfg_h::TRUE;
            /* we don't need the ACs if producing a 1/8th-size image */
            (*entropy).ac_needed[blkn as usize] = ((*compptr).DCT_scaled_size > 1i32) as libc::c_int
        } else {
            (*entropy).ac_needed[blkn as usize] = crate::jmorecfg_h::FALSE;
            (*entropy).dc_needed[blkn as usize] = (*entropy).ac_needed[blkn as usize]
        }
        blkn += 1
    }
    /* Initialize bitread state variables */
    (*entropy).bitstate.bits_left = 0i32; /* unnecessary, but keeps Purify quiet */
    (*entropy).bitstate.get_buffer = 0u64;
    (*entropy).pub_0.insufficient_data = crate::jmorecfg_h::FALSE;
    /* Initialize restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
/* Expand a Huffman table definition into the derived format */
/*
 * Compute the derived values for a Huffman table.
 * This routine also performs some validation checks on the table.
 *
 * Note this is also used by jdphuff.c.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_make_d_derived_tbl(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut isDC: crate::jmorecfg_h::boolean,
    mut tblno: libc::c_int,
    mut pdtbl: *mut *mut crate::src::jdhuff::d_derived_tbl,
) {
    
    
    
    
    
    
    
    
    
    
    
        let mut i:  libc::c_int =  0;    let mut huffsize:  [libc::c_char; 257] =  [0; 257]; let mut huffcode:  [libc::c_uint; 257] =  [0; 257]; 
    /* Note that huffsize[] and huffcode[] are filled in code-length order,
     * paralleling the order of the symbols themselves in htbl->huffval[].
     */
    /* Find the input Huffman table */
    if tblno < 0i32 || tblno >= crate::jpeglib_h::NUM_HUFF_TBLS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_NO_HUFF_TABLE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = tblno;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
     let mut htbl:   *mut crate::jpeglib_h::JHUFF_TBL =
     if isDC != 0 {
        (*cinfo).dc_huff_tbl_ptrs[tblno as usize]
    } else {
        (*cinfo).ac_huff_tbl_ptrs[tblno as usize]
    };
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_NO_HUFF_TABLE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = tblno;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Allocate a workspace if we haven't already done so. */
    if (*pdtbl).is_null() {
        *pdtbl = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            ::std::mem::size_of::<crate::src::jdhuff::d_derived_tbl>() as libc::c_ulong,
        ) as *mut crate::src::jdhuff::d_derived_tbl
    } /* fill in back link */
     let mut dtbl:   *mut crate::src::jdhuff::d_derived_tbl =  *pdtbl;
    (*dtbl).pub_0 = htbl;
    
     let mut p:   libc::c_int =  0i32; let mut l:   libc::c_int =  1i32;
    while l <= 16i32 {
        i = (*htbl).bits[l as usize] as libc::c_int;
        if i < 0i32 || p + i > 256i32 {
            /* protect against table overrun */
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_HUFF_TABLE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        loop {
            let fresh0 = i;
            i -=  1;
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = p;
            p +=  1;
            huffsize[fresh1 as usize] = l as libc::c_char
        }
        l += 1
    }
    huffsize[p as usize] = 0i8;
    
    
     let mut numsymbols:   libc::c_int =  p; let mut code:   libc::c_uint =  0u32; let mut si:   libc::c_int =  huffsize[0] as libc::c_int;
    p = 0i32;
    while huffsize[p as usize] != 0 {
        while huffsize[p as usize] as libc::c_int == si {
            let fresh2 = p;
            p +=  1;
            huffcode[fresh2 as usize] = code;
            code +=  1
        }
        /* code is now 1 more than the last code used for codelength si; but
         * it must still fit in si bits, since no code is allowed to be all ones.
         */
        if code as crate::jpegint_h::JLONG >= (1i64) << si {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_HUFF_TABLE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        code <<= 1i32;
        si += 1
    }
    /* Figure F.15: generate decoding tables for bit-sequential decoding */
    p = 0i32;
    l = 1i32;
    while l <= 16i32 {
        if (*htbl).bits[l as usize] != 0 {
            /* valoffset[l] = huffval[] index of 1st symbol of code length l,
             * minus the minimum code of length l
             */
            (*dtbl).valoffset[l as usize] =
                p as crate::jpegint_h::JLONG - huffcode[p as usize] as crate::jpegint_h::JLONG;
            p += (*htbl).bits[l as usize] as libc::c_int;
            (*dtbl).maxcode[l as usize] = huffcode[(p - 1i32) as usize] as crate::jpegint_h::JLONG
        /* maximum code of length l */
        } else {
            (*dtbl).maxcode[l as usize] = -1i64
            /* -1 if no codes of this length */
        } /* ensures jpeg_huff_decode terminates */
        l += 1
    }
    (*dtbl).valoffset[17] = 0i64;
    (*dtbl).maxcode[17] = 0xfffffi64;
    /* Compute lookahead tables to speed up decoding.
     * First we set all the table entries to 0, indicating "too long";
     * then we iterate through the Huffman codes that are short enough and
     * fill in all the entries that correspond to bit sequences starting
     * with that code.
     */
    i = 0i32;
    while i < 1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD {
        (*dtbl).lookup[i as usize] =
            crate::src::jdhuff::HUFF_LOOKAHEAD + 1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD;
        i += 1
    }
    p = 0i32;
    l = 1i32;
    while l <= crate::src::jdhuff::HUFF_LOOKAHEAD {
        i = 1i32;
        while i <= (*htbl).bits[l as usize] as libc::c_int {
            /* l = current code's length, p = its index in huffcode[] & huffval[]. */
            /* Generate left-justified code followed by all possible bit sequences */
              
             let mut lookbits:   libc::c_int =
    
                (huffcode[p as usize] << crate::src::jdhuff::HUFF_LOOKAHEAD - l) as libc::c_int; let mut ctr:   libc::c_int =  1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD - l;
            while ctr > 0i32 {
                (*dtbl).lookup[lookbits as usize] = l << crate::src::jdhuff::HUFF_LOOKAHEAD
                    | (*htbl).huffval[p as usize] as libc::c_int;
                lookbits += 1;
                ctr -= 1
            }
            i += 1;
            p += 1
        }
        l += 1
    }
    /* Validate symbols as being reasonable.
     * For AC tables, we make no check, but accept all byte values 0..255.
     * For DC tables, we require the symbols to be in range 0..15.
     * (Tighter bounds could be applied depending on the data depth and mode,
     * but this is sufficient to ensure safe decoding.)
     */
    if isDC != 0 {
        i = 0i32;
        while i < numsymbols {
            let mut sym: libc::c_int = (*htbl).huffval[i as usize] as libc::c_int;
            if sym < 0i32 || sym > 15i32 {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_HUFF_TABLE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            i += 1
        }
    };
}
/*
 * Out-of-line code for bit fetching (shared with jdphuff.c).
 * See jdhuff.h for info about usage.
 * Note: current values of get_buffer and bits_left are passed as parameters,
 * but are returned in the corresponding fields of the state struct.
 *
 * On most machines MIN_GET_BITS should be 25 to allow the full 32-bit width
 * of get_buffer to be used.  (On machines with wider words, an even larger
 * buffer could be used.)  However, on some machines 32-bit shifts are
 * quite slow and take time proportional to the number of places shifted.
 * (This is true with most PC compilers, for instance.)  In this case it may
 * be a win to set MIN_GET_BITS to the minimum value of 15.  This reduces the
 * average shift distance at the cost of more calls to jpeg_fill_bit_buffer.
 */

pub const MIN_GET_BITS: libc::c_int = crate::src::jdhuff::BIT_BUF_SIZE - 7i32;
/* Macros to declare and load/save bitread local variables. */
/*
 * These macros provide the in-line portion of bit fetching.
 * Use CHECK_BIT_BUFFER to ensure there are N bits in get_buffer
 * before using GET_BITS, PEEK_BITS, or DROP_BITS.
 * The variables get_buffer and bits_left are assumed to be locals,
 * but the state struct might not be (jpeg_huff_decode needs this).
 *      CHECK_BIT_BUFFER(state, n, action);
 *              Ensure there are N bits in get_buffer; if suspend, take action.
 *      val = GET_BITS(n);
 *              Fetch next N bits.
 *      val = PEEK_BITS(n);
 *              Fetch next N bits without removing them from the buffer.
 *      DROP_BITS(n);
 *              Discard next N bits.
 * The value N should be a simple variable, not an expression, because it
 * is evaluated multiple times.
 */
/* Load up the bit buffer to a depth of at least nbits */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fill_bit_buffer(
    mut state: *mut crate::src::jdhuff::bitread_working_state,
    mut get_buffer: crate::src::jdhuff::bit_buf_type,
    mut bits_left: libc::c_int,
    mut nbits: libc::c_int,
) -> crate::jmorecfg_h::boolean
/* Load up the bit buffer to a depth of at least nbits */ {
    /* Copy heavily used state fields into locals (hopefully registers) */
     let mut current_block_30:  u64;let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*state).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*state).bytes_in_buffer;
    let mut cinfo: crate::jpeglib_h::j_decompress_ptr = (*state).cinfo;
    
    /* Attempt to load at least MIN_GET_BITS bits into get_buffer. */
    /* (It is assumed that no request will be for more than that many bits.) */
    /* We fail to do so only if we hit a marker or are forced to suspend. */
    if (*cinfo).unread_marker == 0i32 {
        loop {
             if !(bits_left < MIN_GET_BITS) {
                current_block_30 = 6417057564578538666;
                break;
                /* end while */
            }
            
            /* Attempt to read a byte */
            if bytes_in_buffer == 0u64 {
                if Some(
                    (*(*cinfo).src)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return crate::jmorecfg_h::FALSE;
                }
                next_input_byte = (*(*cinfo).src).next_input_byte;
                bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer
            }
            bytes_in_buffer -=  1;
            let fresh3 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
             let mut c:   libc::c_int =  *fresh3 as libc::c_int;
            /* If it's 0xFF, check and discard stuffed zero byte */
            if c == 0xffi32 {
                loop
                /* Loop here to discard any padding FF's on terminating marker,
                 * so that we can save a valid unread_marker value.  NOTE: we will
                 * accept multiple FF's followed by a 0 as meaning a single FF data
                 * byte.  This data pattern is not valid according to the standard.
                 */
                {
                    if bytes_in_buffer == 0u64 {
                        if Some(
                            (*(*cinfo).src)
                                .fill_input_buffer
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(cinfo)
                            == 0
                        {
                            return crate::jmorecfg_h::FALSE;
                        }
                        next_input_byte = (*(*cinfo).src).next_input_byte;
                        bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer
                    }
                    bytes_in_buffer -=  1;
                    let fresh4 = next_input_byte;
                    next_input_byte = next_input_byte.offset(1);
                    c = *fresh4 as libc::c_int;
                    if !(c == 0xffi32) {
                        break;
                    }
                }
                if c == 0i32 {
                    /* Found FF/00, which represents an FF data byte */
                    c = 0xffi32
                } else {
                    /* Oops, it's actually a marker indicating end of compressed data.
                     * Save the marker code for later use.
                     * Fine point: it might appear that we should save the marker into
                     * bitread working state, not straight into permanent state.  But
                     * once we have hit a marker, we cannot need to suspend within the
                     * current MCU, because we will read no more bytes from the data
                     * source.  So it is OK to update permanent state right away.
                     */
                    (*cinfo).unread_marker = c;
                    current_block_30 = 7022714159392939963;
                    break;
                }
            }
            /* OK, load c into get_buffer */
            get_buffer = get_buffer << 8i32 | c as libc::c_ulong;
            bits_left += 8i32
        }
    } else {
        current_block_30 = 7022714159392939963;
    }
    match current_block_30 {
        7022714159392939963 =>
        /* See if we need to insert some fake zero bits. */
        /* We get here if we've read the marker that terminates the compressed
         * data segment.  There should be enough bits in the buffer register
         * to satisfy the request; if so, no problem.
         */
        {
            if nbits > bits_left {
                /* Uh-oh.  Report corrupted data to user and stuff zeroes into
                 * the data stream, so that we can produce some kind of image.
                 * We use a nonvolatile flag to ensure that only one warning message
                 * appears per data segment.
                 */
                if (*(*cinfo).entropy).insufficient_data == 0 {
                    (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_HIT_MARKER as libc::c_int;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        -1i32,
                    );
                    (*(*cinfo).entropy).insufficient_data = crate::jmorecfg_h::TRUE
                }
                /* Fill the buffer with zero bits */
                get_buffer <<= MIN_GET_BITS - bits_left;
                bits_left = MIN_GET_BITS
            }
        }
        _ => {}
    }
    /* Unload the local registers */
    (*state).next_input_byte = next_input_byte;
    (*state).bytes_in_buffer = bytes_in_buffer;
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    return crate::jmorecfg_h::TRUE;
}
/*
 * Code for extracting next Huffman-coded symbol from input bit stream.
 * Again, this is time-critical and we make the main paths be macros.
 *
 * We use a lookahead table to process codes of up to HUFF_LOOKAHEAD bits
 * without looping.  Usually, more than 95% of the Huffman codes will be 8
 * or fewer bits long.  The few overlength codes are handled with a loop,
 * which need not be inline code.
 *
 * Notes about the HUFF_DECODE macro:
 * 1. Near the end of the data segment, we may fail to get enough bits
 *    for a lookahead.  In that case, we do it the hard way.
 * 2. If the lookahead table contains no entry, the next code must be
 *    more than HUFF_LOOKAHEAD bits long.
 * 3. jpeg_huff_decode returns -1 if forced to suspend.
 */
/* Pre-execute the common case of nb <= HUFF_LOOKAHEAD */
/* Equivalent of jpeg_huff_decode() */
/* Don't use GET_BITS() here because we don't want to modify bits_left */
/* Out-of-line case for Huffman code fetching */
/* Macro version of the above, which performs much better but does not
handle markers.  We have to hand off any blocks with markers to the
slower routines. */
/* Pre-execute most common case */
/* Pre-execute case of FF/00, which represents an FF data byte */
/* Oops, it's actually a marker indicating end of compressed data. */
/* Back out pre-execution and fill the buffer with zero bits */
/* Pre-fetch 48 bytes, because the holding register is 64-bit */
/*
 * Out-of-line code for Huffman code decoding.
 * See jdhuff.h for info about usage.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_huff_decode(
    mut state: *mut crate::src::jdhuff::bitread_working_state,
    mut get_buffer: crate::src::jdhuff::bit_buf_type,
    mut bits_left: libc::c_int,
    mut htbl: *mut crate::src::jdhuff::d_derived_tbl,
    mut min_bits: libc::c_int,
) -> libc::c_int {
     let mut l: libc::c_int = min_bits;
    
    /* HUFF_DECODE has determined that the code is at least min_bits */
    /* bits long, so fetch that many bits in one swoop. */
    if bits_left < l {
        if jpeg_fill_bit_buffer(state, get_buffer, bits_left, l) == 0 {
            return -1i32;
        }
        get_buffer = (*state).get_buffer;
        bits_left = (*state).bits_left
    }
    bits_left -= l;
     let mut code:   crate::jpegint_h::JLONG =
    
        ((get_buffer >> bits_left) as libc::c_int & (1i32 << l) - 1i32) as crate::jpegint_h::JLONG;
    /* Collect the rest of the Huffman code one bit at a time. */
    /* This is per Figure F.16. */
    while code > (*htbl).maxcode[l as usize] {
        code <<= 1i32;
        if bits_left < 1i32 {
            if jpeg_fill_bit_buffer(state, get_buffer, bits_left, 1i32) == 0 {
                return -1i32;
            }
            get_buffer = (*state).get_buffer;
            bits_left = (*state).bits_left
        }
        bits_left -= 1i32;
        code |= ((get_buffer >> bits_left) as libc::c_int & (1i32 << 1i32) - 1i32) as libc::c_long;
        l += 1
    }
    /* Unload the local registers */
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    /* With garbage input we may reach the sentinel value l = 17. */
    if l > 16i32 {
        (*(*(*state).cinfo).err).msg_code = crate::src::jerror::JWRN_HUFF_BAD_CODE as libc::c_int;
        Some(
            (*(*(*state).cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*state).cinfo as crate::jpeglib_h::j_common_ptr,
            -1i32,
        );
        return 0i32;
        /* fake a zero as the safest result */
    }
    return (*(*htbl).pub_0).huffval[(code + (*htbl).valoffset[l as usize]) as libc::c_int as usize]
        as libc::c_int;
}
/* AVOID_TABLES */
/*
 * Check for a restart marker & resynchronize decoder.
 * Returns FALSE if must suspend.
 */

unsafe extern "C" fn process_restart(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
     let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    
    /* Throw away any unused bits remaining in bit buffer; */
    /* include any full bytes in next_marker's count of discarded bytes */
    (*(*cinfo).marker).discarded_bytes =  (*(*cinfo).marker)
        .discarded_bytes +
    ((*entropy).bitstate.bits_left / 8i32) as libc::c_uint;
    (*entropy).bitstate.bits_left = 0i32;
    /* Advance past the RSTn marker */
    if Some(
        (*(*cinfo).marker)
            .read_restart_marker
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo)
        == 0
    {
        return crate::jmorecfg_h::FALSE;
    }
    /* Re-initialize DC predictions to 0 */
     let mut ci:   libc::c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    /* Reset restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    /* Reset out-of-data flag, unless read_restart_marker left us smack up
     * against a marker.  In that case we will end up treating the next data
     * segment as empty, and we can avoid producing bogus output pixels by
     * leaving the flag set.
     */
    if (*cinfo).unread_marker == 0i32 {
        (*entropy).pub_0.insufficient_data = crate::jmorecfg_h::FALSE
    }
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn decode_mcu_slow(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
       let mut br_state:  crate::src::jdhuff::bitread_working_state =
    
        crate::src::jdhuff::bitread_working_state {
            next_input_byte: ::std::ptr::null::< crate::jmorecfg_h::JOCTET>(),
            bytes_in_buffer: 0,
            get_buffer: 0,
            bits_left: 0,
            cinfo: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_decompress_struct>(),
        };  let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    
    
    
    
    
    /* Outer loop handles each block in the MCU */
    /* Load up working state */
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    
    
    
     let mut get_buffer:   crate::src::jdhuff::bit_buf_type =
     (*entropy).bitstate.get_buffer; let mut bits_left:   libc::c_int =  (*entropy).bitstate.bits_left; let mut state:   savable_state =  (*entropy).saved; let mut blkn:   libc::c_int =  0i32;
    while blkn < (*cinfo).blocks_in_MCU {
         let mut s:  libc::c_int =  0; let mut k:  libc::c_int =  0; let mut r:  libc::c_int =  0; let mut current_block_22:  u64; let mut nb:  libc::c_int =  0;let mut block: crate::jpeglib_h::JBLOCKROW = if !MCU_data.is_null() {
            *MCU_data.offset(blkn as isize)
        } else {
            crate::stddef_h::NULL as crate::jpeglib_h::JBLOCKROW
        };
        let mut dctbl: *mut crate::src::jdhuff::d_derived_tbl =
            (*entropy).dc_cur_tbls[blkn as usize];
        let mut actbl: *mut crate::src::jdhuff::d_derived_tbl =
            (*entropy).ac_cur_tbls[blkn as usize];
        
        
        
        
        /* Decode a single block's worth of coefficients */
        /* Section F.2.2.1: decode the DC coefficient difference */
        
        
        if bits_left < crate::src::jdhuff::HUFF_LOOKAHEAD {
            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) == 0 {
                return 0i32;
            }
            get_buffer = br_state.get_buffer;
            bits_left = br_state.bits_left;
            if bits_left < crate::src::jdhuff::HUFF_LOOKAHEAD {
                nb = 1i32;
                current_block_22 = 6603671518751921130;
            } else {
                current_block_22 = 14576567515993809846;
            }
        } else {
            current_block_22 = 14576567515993809846;
        }
        match current_block_22 {
            14576567515993809846 => {
                  let mut look:   libc::c_int =
     (get_buffer >> bits_left - 8i32) as libc::c_int & (1i32 << 8i32) - 1i32;
                nb = (*dctbl).lookup[look as usize] >> crate::src::jdhuff::HUFF_LOOKAHEAD;
                if nb <= crate::src::jdhuff::HUFF_LOOKAHEAD {
                    bits_left -= nb;
                    s = (*dctbl).lookup[look as usize]
                        & (1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                    current_block_22 = 652864300344834934;
                } else {
                    current_block_22 = 6603671518751921130;
                }
            }
            _ => {}
        }
        match current_block_22 {
            6603671518751921130 => {
                s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, dctbl, nb);
                if s < 0i32 {
                    return 0i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left
            }
            _ => {}
        }
        if s != 0 {
            if bits_left < s {
                if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                    return 0i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left
            }
            bits_left -= s;
            r = (get_buffer >> bits_left) as libc::c_int & (1i32 << s) - 1i32;
            s = (r as libc::c_uint +
    (((
                (r - (1i32 << s - 1i32) >> 31i32) as libc::c_uint
                    & (((-1i32 as libc::c_uint) << s)) + 1u32)))) as libc::c_int
        }
        if (*entropy).dc_needed[blkn as usize] != 0 {
            /* Convert DC difference to actual value, update last_dc_val */
            let mut ci: libc::c_int = (*cinfo).MCU_membership[blkn as usize];
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            if !block.is_null() {
                /* Output the DC coefficient (assumes jpeg_natural_order[0] = 0) */
                (*block)[0] = s as crate::jmorecfg_h::JCOEF
            }
        }
        if (*entropy).ac_needed[blkn as usize] != 0 && !block.is_null() {
            /* Section F.2.2.2: decode the AC coefficients */
            /* Since zeroes are skipped, output area must be cleared beforehand */
            k = 1i32;
            while k < crate::jpeglib_h::DCTSIZE2 {
                
                
                 let mut current_block_60:  u64; let mut nb_0:  libc::c_int =  0;
                if bits_left < crate::src::jdhuff::HUFF_LOOKAHEAD {
                    if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) == 0 {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < crate::src::jdhuff::HUFF_LOOKAHEAD {
                        nb_0 = 1i32;
                        current_block_60 = 276222993270550982;
                    } else {
                        current_block_60 = 3580086814630675314;
                    }
                } else {
                    current_block_60 = 3580086814630675314;
                }
                match current_block_60 {
                    3580086814630675314 => {
                          let mut look_0:   libc::c_int =
    
                            (get_buffer >> bits_left - 8i32) as libc::c_int & (1i32 << 8i32) - 1i32;
                        nb_0 =
                            (*actbl).lookup[look_0 as usize] >> crate::src::jdhuff::HUFF_LOOKAHEAD;
                        if nb_0 <= crate::src::jdhuff::HUFF_LOOKAHEAD {
                            bits_left -= nb_0;
                            s = (*actbl).lookup[look_0 as usize]
                                & (1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                            current_block_60 = 7385833325316299293;
                        } else {
                            current_block_60 = 276222993270550982;
                        }
                    }
                    _ => {}
                }
                match current_block_60 {
                    276222993270550982 => {
                        s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, actbl, nb_0);
                        if s < 0i32 {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    _ => {}
                }
                r = s >> 4i32;
                s &= 15i32;
                if s != 0 {
                    k += r;
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as libc::c_int & (1i32 << s) - 1i32;
                    s = (r as libc::c_uint +
    (((
                        (r - (1i32 << s - 1i32) >> 31i32) as libc::c_uint
                            & (((-1i32 as libc::c_uint) << s)) + 1u32)))) as libc::c_int;
                    /* Output coefficient in natural (dezigzagged) order.
                     * Note: the extra entries in jpeg_natural_order[] will save us
                     * if k >= DCTSIZE2, which could happen if the data is corrupted.
                     */
                    (*block)[*crate::jpegint_h::jpeg_natural_order
                        .as_ptr()
                        .offset(k as isize) as usize] = s as crate::jmorecfg_h::JCOEF
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        } else {
            /* Section F.2.2.2: decode the AC coefficients */
            /* In this path we just discard the values */
            k = 1i32;
            while k < crate::jpeglib_h::DCTSIZE2 {
                
                
                 let mut current_block_97:  u64; let mut nb_1:  libc::c_int =  0;
                if bits_left < crate::src::jdhuff::HUFF_LOOKAHEAD {
                    if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) == 0 {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < crate::src::jdhuff::HUFF_LOOKAHEAD {
                        nb_1 = 1i32;
                        current_block_97 = 6072411194766323756;
                    } else {
                        current_block_97 = 9521147444787763968;
                    }
                } else {
                    current_block_97 = 9521147444787763968;
                }
                match current_block_97 {
                    9521147444787763968 => {
                          let mut look_1:   libc::c_int =
    
                            (get_buffer >> bits_left - 8i32) as libc::c_int & (1i32 << 8i32) - 1i32;
                        nb_1 =
                            (*actbl).lookup[look_1 as usize] >> crate::src::jdhuff::HUFF_LOOKAHEAD;
                        if nb_1 <= crate::src::jdhuff::HUFF_LOOKAHEAD {
                            bits_left -= nb_1;
                            s = (*actbl).lookup[look_1 as usize]
                                & (1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                            current_block_97 = 16375338222180917333;
                        } else {
                            current_block_97 = 6072411194766323756;
                        }
                    }
                    _ => {}
                }
                match current_block_97 {
                    6072411194766323756 => {
                        s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, actbl, nb_1);
                        if s < 0i32 {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    _ => {}
                }
                r = s >> 4i32;
                s &= 15i32;
                if s != 0 {
                    k += r;
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        }
        blkn += 1
    }
    /* Completed MCU, so update state */
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    (*entropy).saved = state;
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn decode_mcu_fast(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
       let mut br_state:  crate::src::jdhuff::bitread_working_state =
    
        crate::src::jdhuff::bitread_working_state {
            next_input_byte: ::std::ptr::null::< crate::jmorecfg_h::JOCTET>(),
            bytes_in_buffer: 0,
            get_buffer: 0,
            bits_left: 0,
            cinfo: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_decompress_struct>(),
        };   let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    
    
    
    
    
    
    /* Outer loop handles each block in the MCU */
    /* Load up working state */
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    
    
    
    
     let mut get_buffer:   crate::src::jdhuff::bit_buf_type =
     (*entropy).bitstate.get_buffer; let mut bits_left:   libc::c_int =  (*entropy).bitstate.bits_left; let mut buffer:   *mut crate::jmorecfg_h::JOCTET =
     br_state.next_input_byte as *mut crate::jmorecfg_h::JOCTET; let mut state:   savable_state =  (*entropy).saved; let mut blkn:   libc::c_int =  0i32;
    while blkn < (*cinfo).blocks_in_MCU {
          let mut k:  libc::c_int =  0; let mut r:  libc::c_int =  0; let mut block: crate::jpeglib_h::JBLOCKROW = if !MCU_data.is_null() {
            *MCU_data.offset(blkn as isize)
        } else {
            crate::stddef_h::NULL as crate::jpeglib_h::JBLOCKROW
        };
        let mut dctbl: *mut crate::src::jdhuff::d_derived_tbl =
            (*entropy).dc_cur_tbls[blkn as usize];
        let mut actbl: *mut crate::src::jdhuff::d_derived_tbl =
            (*entropy).ac_cur_tbls[blkn as usize];
        
        
        
        
        if bits_left <= 16i32 {
            
                        
            let fresh5 = buffer;
            buffer = buffer.offset(1);
            
             let mut c0:   libc::c_int =  *fresh5 as libc::c_int; let mut c1:   libc::c_int =  *buffer as libc::c_int;
            get_buffer = get_buffer << 8i32 | c0 as libc::c_ulong;
            bits_left += 8i32;
            if c0 == 0xffi32 {
                buffer = buffer.offset(1);
                if c1 != 0i32 {
                    (*cinfo).unread_marker = c1;
                    buffer = buffer.offset(-2);
                    get_buffer &= !0xffi32 as libc::c_ulong
                }
            }
            
            
            let fresh6 = buffer;
            buffer = buffer.offset(1);
            
             let mut c0_0:   libc::c_int =  *fresh6 as libc::c_int; let mut c1_0:   libc::c_int =  *buffer as libc::c_int;
            get_buffer = get_buffer << 8i32 | c0_0 as libc::c_ulong;
            bits_left += 8i32;
            if c0_0 == 0xffi32 {
                buffer = buffer.offset(1);
                if c1_0 != 0i32 {
                    (*cinfo).unread_marker = c1_0;
                    buffer = buffer.offset(-2);
                    get_buffer &= !0xffi32 as libc::c_ulong
                }
            }
            
            
            let fresh7 = buffer;
            buffer = buffer.offset(1);
            
             let mut c0_1:   libc::c_int =  *fresh7 as libc::c_int; let mut c1_1:   libc::c_int =  *buffer as libc::c_int;
            get_buffer = get_buffer << 8i32 | c0_1 as libc::c_ulong;
            bits_left += 8i32;
            if c0_1 == 0xffi32 {
                buffer = buffer.offset(1);
                if c1_1 != 0i32 {
                    (*cinfo).unread_marker = c1_1;
                    buffer = buffer.offset(-2);
                    get_buffer &= !0xffi32 as libc::c_ulong
                }
            }
            
            
            let fresh8 = buffer;
            buffer = buffer.offset(1);
            
             let mut c0_2:   libc::c_int =  *fresh8 as libc::c_int; let mut c1_2:   libc::c_int =  *buffer as libc::c_int;
            get_buffer = get_buffer << 8i32 | c0_2 as libc::c_ulong;
            bits_left += 8i32;
            if c0_2 == 0xffi32 {
                buffer = buffer.offset(1);
                if c1_2 != 0i32 {
                    (*cinfo).unread_marker = c1_2;
                    buffer = buffer.offset(-2);
                    get_buffer &= !0xffi32 as libc::c_ulong
                }
            }
            
            
            let fresh9 = buffer;
            buffer = buffer.offset(1);
            
             let mut c0_3:   libc::c_int =  *fresh9 as libc::c_int; let mut c1_3:   libc::c_int =  *buffer as libc::c_int;
            get_buffer = get_buffer << 8i32 | c0_3 as libc::c_ulong;
            bits_left += 8i32;
            if c0_3 == 0xffi32 {
                buffer = buffer.offset(1);
                if c1_3 != 0i32 {
                    (*cinfo).unread_marker = c1_3;
                    buffer = buffer.offset(-2);
                    get_buffer &= !0xffi32 as libc::c_ulong
                }
            }
            
            
            let fresh10 = buffer;
            buffer = buffer.offset(1);
            
             let mut c0_4:   libc::c_int =  *fresh10 as libc::c_int; let mut c1_4:   libc::c_int =  *buffer as libc::c_int;
            get_buffer = get_buffer << 8i32 | c0_4 as libc::c_ulong;
            bits_left += 8i32;
            if c0_4 == 0xffi32 {
                buffer = buffer.offset(1);
                if c1_4 != 0i32 {
                    (*cinfo).unread_marker = c1_4;
                    buffer = buffer.offset(-2);
                    get_buffer &= !0xffi32 as libc::c_ulong
                }
            }
        }
         let mut s:   libc::c_int =
     (get_buffer >> bits_left - 8i32) as libc::c_int & (1i32 << 8i32) - 1i32;
        s = (*dctbl).lookup[s as usize];
         let mut l:   libc::c_int =  s >> crate::src::jdhuff::HUFF_LOOKAHEAD;
        bits_left -= l;
        s = s & (1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD) - 1i32;
        if l > crate::src::jdhuff::HUFF_LOOKAHEAD {
            s = (get_buffer >> bits_left & ((1i32 << l) - 1i32) as libc::c_ulong) as libc::c_int;
            while s as libc::c_long > (*dctbl).maxcode[l as usize] {
                s <<= 1i32;
                bits_left -= 1i32;
                s |= (get_buffer >> bits_left) as libc::c_int & (1i32 << 1i32) - 1i32;
                l += 1
            }
            s = (*(*dctbl).pub_0).huffval[((s as libc::c_long + (*dctbl).valoffset[l as usize])
                as libc::c_int
                & 0xffi32) as usize] as libc::c_int
        }
        if s != 0 {
            if bits_left <= 16i32 {
                
                            
                let fresh11 = buffer;
                buffer = buffer.offset(1);
                
                 let mut c0_5:   libc::c_int =  *fresh11 as libc::c_int; let mut c1_5:   libc::c_int =  *buffer as libc::c_int;
                get_buffer = get_buffer << 8i32 | c0_5 as libc::c_ulong;
                bits_left += 8i32;
                if c0_5 == 0xffi32 {
                    buffer = buffer.offset(1);
                    if c1_5 != 0i32 {
                        (*cinfo).unread_marker = c1_5;
                        buffer = buffer.offset(-2);
                        get_buffer &= !0xffi32 as libc::c_ulong
                    }
                }
                
                
                let fresh12 = buffer;
                buffer = buffer.offset(1);
                
                 let mut c0_6:   libc::c_int =  *fresh12 as libc::c_int; let mut c1_6:   libc::c_int =  *buffer as libc::c_int;
                get_buffer = get_buffer << 8i32 | c0_6 as libc::c_ulong;
                bits_left += 8i32;
                if c0_6 == 0xffi32 {
                    buffer = buffer.offset(1);
                    if c1_6 != 0i32 {
                        (*cinfo).unread_marker = c1_6;
                        buffer = buffer.offset(-2);
                        get_buffer &= !0xffi32 as libc::c_ulong
                    }
                }
                
                
                let fresh13 = buffer;
                buffer = buffer.offset(1);
                
                 let mut c0_7:   libc::c_int =  *fresh13 as libc::c_int; let mut c1_7:   libc::c_int =  *buffer as libc::c_int;
                get_buffer = get_buffer << 8i32 | c0_7 as libc::c_ulong;
                bits_left += 8i32;
                if c0_7 == 0xffi32 {
                    buffer = buffer.offset(1);
                    if c1_7 != 0i32 {
                        (*cinfo).unread_marker = c1_7;
                        buffer = buffer.offset(-2);
                        get_buffer &= !0xffi32 as libc::c_ulong
                    }
                }
                
                
                let fresh14 = buffer;
                buffer = buffer.offset(1);
                
                 let mut c0_8:   libc::c_int =  *fresh14 as libc::c_int; let mut c1_8:   libc::c_int =  *buffer as libc::c_int;
                get_buffer = get_buffer << 8i32 | c0_8 as libc::c_ulong;
                bits_left += 8i32;
                if c0_8 == 0xffi32 {
                    buffer = buffer.offset(1);
                    if c1_8 != 0i32 {
                        (*cinfo).unread_marker = c1_8;
                        buffer = buffer.offset(-2);
                        get_buffer &= !0xffi32 as libc::c_ulong
                    }
                }
                
                
                let fresh15 = buffer;
                buffer = buffer.offset(1);
                
                 let mut c0_9:   libc::c_int =  *fresh15 as libc::c_int; let mut c1_9:   libc::c_int =  *buffer as libc::c_int;
                get_buffer = get_buffer << 8i32 | c0_9 as libc::c_ulong;
                bits_left += 8i32;
                if c0_9 == 0xffi32 {
                    buffer = buffer.offset(1);
                    if c1_9 != 0i32 {
                        (*cinfo).unread_marker = c1_9;
                        buffer = buffer.offset(-2);
                        get_buffer &= !0xffi32 as libc::c_ulong
                    }
                }
                
                
                let fresh16 = buffer;
                buffer = buffer.offset(1);
                
                 let mut c0_10:   libc::c_int =  *fresh16 as libc::c_int; let mut c1_10:   libc::c_int =  *buffer as libc::c_int;
                get_buffer = get_buffer << 8i32 | c0_10 as libc::c_ulong;
                bits_left += 8i32;
                if c0_10 == 0xffi32 {
                    buffer = buffer.offset(1);
                    if c1_10 != 0i32 {
                        (*cinfo).unread_marker = c1_10;
                        buffer = buffer.offset(-2);
                        get_buffer &= !0xffi32 as libc::c_ulong
                    }
                }
            }
            bits_left -= s;
            r = (get_buffer >> bits_left) as libc::c_int & (1i32 << s) - 1i32;
            s = (r as libc::c_uint +
    (((
                (r - (1i32 << s - 1i32) >> 31i32) as libc::c_uint
                    & (((-1i32 as libc::c_uint) << s)) + 1u32)))) as libc::c_int
        }
        if (*entropy).dc_needed[blkn as usize] != 0 {
            let mut ci: libc::c_int = (*cinfo).MCU_membership[blkn as usize];
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            if !block.is_null() {
                (*block)[0] = s as crate::jmorecfg_h::JCOEF
            }
        }
        if (*entropy).ac_needed[blkn as usize] != 0 && !block.is_null() {
            k = 1i32;
            while k < crate::jpeglib_h::DCTSIZE2 {
                if bits_left <= 16i32 {
                    
                                
                    let fresh17 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_11:   libc::c_int =  *fresh17 as libc::c_int; let mut c1_11:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_11 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_11 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_11 != 0i32 {
                            (*cinfo).unread_marker = c1_11;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh18 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_12:   libc::c_int =  *fresh18 as libc::c_int; let mut c1_12:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_12 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_12 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_12 != 0i32 {
                            (*cinfo).unread_marker = c1_12;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh19 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_13:   libc::c_int =  *fresh19 as libc::c_int; let mut c1_13:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_13 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_13 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_13 != 0i32 {
                            (*cinfo).unread_marker = c1_13;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh20 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_14:   libc::c_int =  *fresh20 as libc::c_int; let mut c1_14:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_14 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_14 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_14 != 0i32 {
                            (*cinfo).unread_marker = c1_14;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh21 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_15:   libc::c_int =  *fresh21 as libc::c_int; let mut c1_15:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_15 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_15 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_15 != 0i32 {
                            (*cinfo).unread_marker = c1_15;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh22 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_16:   libc::c_int =  *fresh22 as libc::c_int; let mut c1_16:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_16 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_16 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_16 != 0i32 {
                            (*cinfo).unread_marker = c1_16;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                }
                s = (get_buffer >> bits_left - 8i32) as libc::c_int & (1i32 << 8i32) - 1i32;
                s = (*actbl).lookup[s as usize];
                l = s >> crate::src::jdhuff::HUFF_LOOKAHEAD;
                bits_left -= l;
                s = s & (1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                if l > crate::src::jdhuff::HUFF_LOOKAHEAD {
                    s = (get_buffer >> bits_left & ((1i32 << l) - 1i32) as libc::c_ulong)
                        as libc::c_int;
                    while s as libc::c_long > (*actbl).maxcode[l as usize] {
                        s <<= 1i32;
                        bits_left -= 1i32;
                        s |= (get_buffer >> bits_left) as libc::c_int & (1i32 << 1i32) - 1i32;
                        l += 1
                    }
                    s = (*(*actbl).pub_0).huffval[((s as libc::c_long
                        + (*actbl).valoffset[l as usize])
                        as libc::c_int
                        & 0xffi32) as usize] as libc::c_int
                }
                r = s >> 4i32;
                s &= 15i32;
                if s != 0 {
                    k += r;
                    if bits_left <= 16i32 {
                        
                                    
                        let fresh23 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_17:   libc::c_int =  *fresh23 as libc::c_int; let mut c1_17:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_17 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_17 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_17 != 0i32 {
                                (*cinfo).unread_marker = c1_17;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh24 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_18:   libc::c_int =  *fresh24 as libc::c_int; let mut c1_18:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_18 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_18 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_18 != 0i32 {
                                (*cinfo).unread_marker = c1_18;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh25 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_19:   libc::c_int =  *fresh25 as libc::c_int; let mut c1_19:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_19 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_19 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_19 != 0i32 {
                                (*cinfo).unread_marker = c1_19;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh26 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_20:   libc::c_int =  *fresh26 as libc::c_int; let mut c1_20:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_20 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_20 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_20 != 0i32 {
                                (*cinfo).unread_marker = c1_20;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh27 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_21:   libc::c_int =  *fresh27 as libc::c_int; let mut c1_21:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_21 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_21 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_21 != 0i32 {
                                (*cinfo).unread_marker = c1_21;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh28 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_22:   libc::c_int =  *fresh28 as libc::c_int; let mut c1_22:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_22 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_22 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_22 != 0i32 {
                                (*cinfo).unread_marker = c1_22;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as libc::c_int & (1i32 << s) - 1i32;
                    s = (r as libc::c_uint +
    (((
                        (r - (1i32 << s - 1i32) >> 31i32) as libc::c_uint
                            & (((-1i32 as libc::c_uint) << s)) + 1u32)))) as libc::c_int;
                    (*block)[*crate::jpegint_h::jpeg_natural_order
                        .as_ptr()
                        .offset(k as isize) as usize] = s as crate::jmorecfg_h::JCOEF
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        } else {
            k = 1i32;
            while k < crate::jpeglib_h::DCTSIZE2 {
                if bits_left <= 16i32 {
                    
                                
                    let fresh29 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_23:   libc::c_int =  *fresh29 as libc::c_int; let mut c1_23:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_23 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_23 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_23 != 0i32 {
                            (*cinfo).unread_marker = c1_23;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh30 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_24:   libc::c_int =  *fresh30 as libc::c_int; let mut c1_24:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_24 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_24 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_24 != 0i32 {
                            (*cinfo).unread_marker = c1_24;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh31 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_25:   libc::c_int =  *fresh31 as libc::c_int; let mut c1_25:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_25 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_25 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_25 != 0i32 {
                            (*cinfo).unread_marker = c1_25;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh32 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_26:   libc::c_int =  *fresh32 as libc::c_int; let mut c1_26:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_26 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_26 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_26 != 0i32 {
                            (*cinfo).unread_marker = c1_26;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh33 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_27:   libc::c_int =  *fresh33 as libc::c_int; let mut c1_27:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_27 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_27 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_27 != 0i32 {
                            (*cinfo).unread_marker = c1_27;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                    
                    
                    let fresh34 = buffer;
                    buffer = buffer.offset(1);
                    
                     let mut c0_28:   libc::c_int =  *fresh34 as libc::c_int; let mut c1_28:   libc::c_int =  *buffer as libc::c_int;
                    get_buffer = get_buffer << 8i32 | c0_28 as libc::c_ulong;
                    bits_left += 8i32;
                    if c0_28 == 0xffi32 {
                        buffer = buffer.offset(1);
                        if c1_28 != 0i32 {
                            (*cinfo).unread_marker = c1_28;
                            buffer = buffer.offset(-2);
                            get_buffer &= !0xffi32 as libc::c_ulong
                        }
                    }
                }
                s = (get_buffer >> bits_left - 8i32) as libc::c_int & (1i32 << 8i32) - 1i32;
                s = (*actbl).lookup[s as usize];
                l = s >> crate::src::jdhuff::HUFF_LOOKAHEAD;
                bits_left -= l;
                s = s & (1i32 << crate::src::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                if l > crate::src::jdhuff::HUFF_LOOKAHEAD {
                    s = (get_buffer >> bits_left & ((1i32 << l) - 1i32) as libc::c_ulong)
                        as libc::c_int;
                    while s as libc::c_long > (*actbl).maxcode[l as usize] {
                        s <<= 1i32;
                        bits_left -= 1i32;
                        s |= (get_buffer >> bits_left) as libc::c_int & (1i32 << 1i32) - 1i32;
                        l += 1
                    }
                    s = (*(*actbl).pub_0).huffval[((s as libc::c_long
                        + (*actbl).valoffset[l as usize])
                        as libc::c_int
                        & 0xffi32) as usize] as libc::c_int
                }
                r = s >> 4i32;
                s &= 15i32;
                if s != 0 {
                    k += r;
                    if bits_left <= 16i32 {
                        
                                    
                        let fresh35 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_29:   libc::c_int =  *fresh35 as libc::c_int; let mut c1_29:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_29 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_29 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_29 != 0i32 {
                                (*cinfo).unread_marker = c1_29;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh36 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_30:   libc::c_int =  *fresh36 as libc::c_int; let mut c1_30:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_30 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_30 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_30 != 0i32 {
                                (*cinfo).unread_marker = c1_30;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh37 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_31:   libc::c_int =  *fresh37 as libc::c_int; let mut c1_31:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_31 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_31 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_31 != 0i32 {
                                (*cinfo).unread_marker = c1_31;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh38 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_32:   libc::c_int =  *fresh38 as libc::c_int; let mut c1_32:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_32 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_32 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_32 != 0i32 {
                                (*cinfo).unread_marker = c1_32;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh39 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_33:   libc::c_int =  *fresh39 as libc::c_int; let mut c1_33:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_33 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_33 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_33 != 0i32 {
                                (*cinfo).unread_marker = c1_33;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                        
                        
                        let fresh40 = buffer;
                        buffer = buffer.offset(1);
                        
                         let mut c0_34:   libc::c_int =  *fresh40 as libc::c_int; let mut c1_34:   libc::c_int =  *buffer as libc::c_int;
                        get_buffer = get_buffer << 8i32 | c0_34 as libc::c_ulong;
                        bits_left += 8i32;
                        if c0_34 == 0xffi32 {
                            buffer = buffer.offset(1);
                            if c1_34 != 0i32 {
                                (*cinfo).unread_marker = c1_34;
                                buffer = buffer.offset(-2);
                                get_buffer &= !0xffi32 as libc::c_ulong
                            }
                        }
                    }
                    bits_left -= s
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        }
        blkn += 1
    }
    if (*cinfo).unread_marker != 0i32 {
        (*cinfo).unread_marker = 0i32;
        return crate::jmorecfg_h::FALSE;
    }
    br_state.bytes_in_buffer = br_state.bytes_in_buffer -
    
        
    
        buffer.wrapping_offset_from(br_state.next_input_byte) as libc::c_ulong;
    br_state.next_input_byte = buffer;
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    (*entropy).saved = state;
    return crate::jmorecfg_h::TRUE;
}
/*
 * Decode and return one MCU's worth of Huffman-compressed coefficients.
 * The coefficients are reordered from zigzag order into natural array order,
 * but are not dequantized.
 *
 * The i'th block of the MCU is stored into the block pointed to by
 * MCU_data[i].  WE ASSUME THIS AREA HAS BEEN ZEROED BY THE CALLER.
 * (Wholesale zeroing is usually a little faster than retail...)
 *
 * Returns FALSE if data source requested suspension.  In that case no
 * changes have been made to permanent state.  (Exception: some output
 * coefficients may already have been assigned.  This is harmless for
 * this module, since we'll just re-assign them on the next call.)
 */

pub const BUFSIZE: libc::c_int = crate::jpeglib_h::DCTSIZE2 * 8i32;

unsafe extern "C" fn decode_mcu(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
     let mut usefast:  libc::c_int =  1i32;let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            if process_restart(cinfo) == 0 {
                return crate::jmorecfg_h::FALSE;
            }
        }
        usefast = 0i32
    }
    if (*(*cinfo).src).bytes_in_buffer
        < BUFSIZE as libc::c_ulong * (*cinfo).blocks_in_MCU as crate::stddef_h::size_t
        || (*cinfo).unread_marker != 0i32
    {
        usefast = 0i32
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).pub_0.insufficient_data == 0 {
         let mut current_block_9:  u64;
        if usefast != 0 {
            if decode_mcu_fast(cinfo, MCU_data) == 0 {
                current_block_9 = 4519966451224754431;
            } else {
                current_block_9 = 12349973810996921269;
            }
        } else {
            current_block_9 = 4519966451224754431;
        }
        match current_block_9 {
            4519966451224754431 => {
                if decode_mcu_slow(cinfo, MCU_data) == 0 {
                    return crate::jmorecfg_h::FALSE;
                }
            }
            _ => {}
        }
    }
    /* Account for restart interval (no-op if not using restarts) */
    (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1;
    return crate::jmorecfg_h::TRUE;
}
/*
 * Module initialization routine for Huffman entropy decoding.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_huff_decoder(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    
      
    /* Motion JPEG frames typically do not include the Huffman tables if they
    are the default tables.  Thus, if the tables are not set by the time
    the Huffman decoder is initialized (usually within the body of
    jpeg_start_decompress()), we set them to default values. */
    crate::jstdhuff_c::std_huff_tables(cinfo as crate::jpeglib_h::j_common_ptr);
     let mut entropy:   huff_entropy_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<huff_entropy_decoder>() as libc::c_ulong,
    ) as huff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut crate::jpeglib_h::jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass = Some(
        start_pass_huff_decoder
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*entropy).pub_0.decode_mcu = Some(
        decode_mcu
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: *mut crate::jpeglib_h::JBLOCKROW,
            ) -> crate::jmorecfg_h::boolean,
    );
    /* Mark tables unallocated */
     let mut i:   libc::c_int =  0i32;
    while i < crate::jpeglib_h::NUM_HUFF_TBLS {
        (*entropy).ac_derived_tbls[i as usize] =
            crate::stddef_h::NULL as *mut crate::src::jdhuff::d_derived_tbl;
        (*entropy).dc_derived_tbls[i as usize] = (*entropy).ac_derived_tbls[i as usize];
        i += 1
    }
}
