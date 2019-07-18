pub use crate::jdhuff::bit_buf_type;
pub use crate::jdhuff::bitread_perm_state;
pub use crate::jdhuff::bitread_working_state;
pub use crate::jdhuff::d_derived_tbl;
pub use crate::jdhuff::jpeg_fill_bit_buffer;
pub use crate::jdhuff::jpeg_huff_decode;
pub use crate::jdhuff::jpeg_make_d_derived_tbl;
pub use crate::jdhuff::HUFF_LOOKAHEAD;
pub use crate::jerror::C2RustUnnamed_3;
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
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_natural_order;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
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
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
pub use crate::limits_h::INT_MAX;
pub use crate::limits_h::INT_MIN;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::__INT_MAX__;
use libc;
use libc::c_int;
use libc::c_uint;
use libc::c_ulong;
pub type phuff_entropy_ptr = *mut phuff_entropy_decoder;
/* This macro is to work around compilers with missing or broken
 * structure assignment.  You'll need to fix this code if you have
 * such a compiler and you change MAX_COMPS_IN_SCAN.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phuff_entropy_decoder {
    pub pub_0: jpeg_entropy_decoder,
    pub bitstate: bitread_perm_state,
    pub saved: savable_state,
    pub restarts_to_go: c_uint,
    pub derived_tbls: [*mut d_derived_tbl; 4],
    pub ac_derived_tbl: *mut d_derived_tbl,
}
/*
 * jdphuff.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1995-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015-2016, 2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains Huffman entropy decoding routines for progressive JPEG.
 *
 * Much of the complexity here has to do with supporting input suspension.
 * If the data source module demands suspension, we want to be able to back
 * up to the start of the current MCU.  To do this, we copy state variables
 * into local working storage, and update them back to the permanent
 * storage only upon successful completion of an MCU.
 *
 * NOTE: All referenced figures are from
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
 */
/*
 * Expanded entropy decoder object for progressive Huffman decoding.
 *
 * The savable_state subrecord contains fields that change within an MCU,
 * but must not be updated permanently until we complete the MCU.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct savable_state {
    pub EOBRUN: c_uint,
    pub last_dc_val: [c_int; 4],
}
/*
 * Initialize for a Huffman-compressed scan.
 */
unsafe extern "C" fn start_pass_phuff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut is_DC_band: boolean = 0;
    let mut bad: boolean = 0;
    let mut ci: c_int = 0;
    let mut coefi: c_int = 0;
    let mut tbl: c_int = 0;
    let mut pdtbl: *mut *mut d_derived_tbl = 0 as *mut *mut d_derived_tbl;
    let mut coef_bit_ptr: *mut c_int = 0 as *mut c_int;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    is_DC_band = ((*cinfo).Ss == 0i32) as c_int;
    bad = FALSE;
    if 0 != is_DC_band {
        if (*cinfo).Se != 0i32 {
            bad = TRUE
        }
    } else {
        if (*cinfo).Ss > (*cinfo).Se || (*cinfo).Se >= DCTSIZE2 {
            bad = TRUE
        }
        if (*cinfo).comps_in_scan != 1i32 {
            bad = TRUE
        }
    }
    if (*cinfo).Ah != 0i32 {
        if (*cinfo).Al != (*cinfo).Ah - 1i32 {
            bad = TRUE
        }
    }
    if (*cinfo).Al > 13i32 {
        bad = TRUE
    }
    if 0 != bad {
        (*(*cinfo).err).msg_code = JERR_BAD_PROGRESSION as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).Ss;
        (*(*cinfo).err).msg_parm.i[1usize] = (*cinfo).Se;
        (*(*cinfo).err).msg_parm.i[2usize] = (*cinfo).Ah;
        (*(*cinfo).err).msg_parm.i[3usize] = (*cinfo).Al;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        let mut cindex: c_int = (*(*cinfo).cur_comp_info[ci as usize]).component_index;
        coef_bit_ptr = &mut *(*(*cinfo).coef_bits.offset(cindex as isize))
            .as_mut_ptr()
            .offset(0isize) as *mut c_int;
        if 0 == is_DC_band && *coef_bit_ptr.offset(0isize) < 0i32 {
            (*(*cinfo).err).msg_code = JWRN_BOGUS_PROGRESSION as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = cindex;
            (*(*cinfo).err).msg_parm.i[1usize] = 0i32;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
        }
        coefi = (*cinfo).Ss;
        while coefi <= (*cinfo).Se {
            let mut expected: c_int = if *coef_bit_ptr.offset(coefi as isize) < 0i32 {
                0i32
            } else {
                *coef_bit_ptr.offset(coefi as isize)
            };
            if (*cinfo).Ah != expected {
                (*(*cinfo).err).msg_code = JWRN_BOGUS_PROGRESSION as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = cindex;
                (*(*cinfo).err).msg_parm.i[1usize] = coefi;
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr, -1i32
                );
            }
            *coef_bit_ptr.offset(coefi as isize) = (*cinfo).Al;
            coefi += 1
        }
        ci += 1
    }
    if (*cinfo).Ah == 0i32 {
        if 0 != is_DC_band {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_DC_first
                    as unsafe extern "C" fn(_: j_decompress_ptr, _: *mut JBLOCKROW) -> boolean,
            )
        } else {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_AC_first
                    as unsafe extern "C" fn(_: j_decompress_ptr, _: *mut JBLOCKROW) -> boolean,
            )
        }
    } else if 0 != is_DC_band {
        (*entropy).pub_0.decode_mcu = Some(
            decode_mcu_DC_refine
                as unsafe extern "C" fn(_: j_decompress_ptr, _: *mut JBLOCKROW) -> boolean,
        )
    } else {
        (*entropy).pub_0.decode_mcu = Some(
            decode_mcu_AC_refine
                as unsafe extern "C" fn(_: j_decompress_ptr, _: *mut JBLOCKROW) -> boolean,
        )
    }
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if 0 != is_DC_band {
            if (*cinfo).Ah == 0i32 {
                tbl = (*compptr).dc_tbl_no;
                pdtbl = (*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize);
                jpeg_make_d_derived_tbl(cinfo, TRUE, tbl, pdtbl);
            }
        } else {
            tbl = (*compptr).ac_tbl_no;
            pdtbl = (*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize);
            jpeg_make_d_derived_tbl(cinfo, FALSE, tbl, pdtbl);
            (*entropy).ac_derived_tbl = (*entropy).derived_tbls[tbl as usize]
        }
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    (*entropy).bitstate.bits_left = 0i32;
    (*entropy).bitstate.get_buffer = 0i32 as bit_buf_type;
    (*entropy).pub_0.insufficient_data = FALSE;
    (*entropy).saved.EOBRUN = 0i32 as c_uint;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
/* AVOID_TABLES */
/*
 * Check for a restart marker & resynchronize decoder.
 * Returns FALSE if must suspend.
 */
unsafe extern "C" fn process_restart(mut cinfo: j_decompress_ptr) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut ci: c_int = 0;
    (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
        .discarded_bytes
        .wrapping_add(((*entropy).bitstate.bits_left / 8i32) as c_uint);
    (*entropy).bitstate.bits_left = 0i32;
    if 0 == (*(*cinfo).marker)
        .read_restart_marker
        .expect("non-null function pointer")(cinfo)
    {
        return FALSE;
    }
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    (*entropy).saved.EOBRUN = 0i32 as c_uint;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    if (*cinfo).unread_marker == 0i32 {
        (*entropy).pub_0.insufficient_data = FALSE
    }
    return TRUE;
}
/* Forward declarations */
/*
 * Huffman MCU decoding.
 * Each of these routines decodes and returns one MCU's worth of
 * Huffman-compressed coefficients.
 * The coefficients are reordered from zigzag order into natural array order,
 * but are not dequantized.
 *
 * The i'th block of the MCU is stored into the block pointed to by
 * MCU_data[i].  WE ASSUME THIS AREA IS INITIALLY ZEROED BY THE CALLER.
 *
 * We return FALSE if data source requested suspension.  In that case no
 * changes have been made to permanent state.  (Exception: some output
 * coefficients may already have been assigned.  This is harmless for
 * spectral selection, since we'll just re-assign them on the next call.
 * Successive approximation AC refinement has to be more careful, however.)
 */
/*
 * MCU decoding for DC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */
unsafe extern "C" fn decode_mcu_DC_first(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut Al: c_int = (*cinfo).Al;
    let mut s: c_int = 0;
    let mut r: c_int = 0;
    let mut blkn: c_int = 0;
    let mut ci: c_int = 0;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    let mut state: savable_state = savable_state {
        EOBRUN: 0,
        last_dc_val: [0; 4],
    };
    let mut tbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if 0 == process_restart(cinfo) {
                return FALSE;
            }
        }
    }
    if 0 == (*entropy).pub_0.insufficient_data {
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        state = (*entropy).saved;
        blkn = 0i32;
        while blkn < (*cinfo).blocks_in_MCU {
            block = *MCU_data.offset(blkn as isize);
            ci = (*cinfo).MCU_membership[blkn as usize];
            compptr = (*cinfo).cur_comp_info[ci as usize];
            tbl = (*entropy).derived_tbls[(*compptr).dc_tbl_no as usize];
            let mut current_block_31: u64;
            let mut nb: c_int = 0;
            let mut look: c_int = 0;
            if bits_left < HUFF_LOOKAHEAD {
                if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) {
                    return 0i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left;
                if bits_left < HUFF_LOOKAHEAD {
                    nb = 1i32;
                    current_block_31 = 18387791726498337928;
                } else {
                    current_block_31 = 5494826135382683477;
                }
            } else {
                current_block_31 = 5494826135382683477;
            }
            match current_block_31 {
                5494826135382683477 => {
                    look = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                    nb = (*tbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                    if nb <= HUFF_LOOKAHEAD {
                        bits_left -= nb;
                        s = (*tbl).lookup[look as usize] & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                        current_block_31 = 17784502470059252271;
                    } else {
                        current_block_31 = 18387791726498337928;
                    }
                }
                _ => {}
            }
            match current_block_31 {
                18387791726498337928 => {
                    s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, tbl, nb);
                    if s < 0i32 {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left
                }
                _ => {}
            }
            if 0 != s {
                if bits_left < s {
                    if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left
                }
                bits_left -= s;
                r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
                s = (if r < 1i32 << s - 1i32 {
                    (r as c_uint)
                        .wrapping_add(((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint))
                } else {
                    r as c_uint
                }) as c_int
            }
            if state.last_dc_val[ci as usize] >= 0i32
                && s > INT_MAX - state.last_dc_val[ci as usize]
                || state.last_dc_val[ci as usize] < 0i32
                    && s < INT_MIN - state.last_dc_val[ci as usize]
            {
                (*(*cinfo).err).msg_code = JERR_BAD_DCT_COEF as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            (*block)[0usize] = ((s as c_ulong) << Al) as JLONG as JCOEF;
            blkn += 1
        }
        (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
        (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
        (*entropy).bitstate.get_buffer = get_buffer;
        (*entropy).bitstate.bits_left = bits_left;
        (*entropy).saved = state
    }
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return TRUE;
}
/*
 * MCU decoding for AC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */
unsafe extern "C" fn decode_mcu_AC_first(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut Se: c_int = (*cinfo).Se;
    let mut Al: c_int = (*cinfo).Al;
    let mut s: c_int = 0;
    let mut k: c_int = 0;
    let mut r: c_int = 0;
    let mut EOBRUN: c_uint = 0;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    let mut tbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if 0 == process_restart(cinfo) {
                return FALSE;
            }
        }
    }
    if 0 == (*entropy).pub_0.insufficient_data {
        EOBRUN = (*entropy).saved.EOBRUN;
        if EOBRUN > 0i32 as c_uint {
            EOBRUN = EOBRUN.wrapping_sub(1)
        } else {
            br_state.cinfo = cinfo;
            br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
            br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
            get_buffer = (*entropy).bitstate.get_buffer;
            bits_left = (*entropy).bitstate.bits_left;
            block = *MCU_data.offset(0isize);
            tbl = (*entropy).ac_derived_tbl;
            k = (*cinfo).Ss;
            while k <= Se {
                let mut current_block_30: u64;
                let mut nb: c_int = 0;
                let mut look: c_int = 0;
                if bits_left < HUFF_LOOKAHEAD {
                    if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < HUFF_LOOKAHEAD {
                        nb = 1i32;
                        current_block_30 = 3118944067848530783;
                    } else {
                        current_block_30 = 2569451025026770673;
                    }
                } else {
                    current_block_30 = 2569451025026770673;
                }
                match current_block_30 {
                    2569451025026770673 => {
                        look = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                        nb = (*tbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                        if nb <= HUFF_LOOKAHEAD {
                            bits_left -= nb;
                            s = (*tbl).lookup[look as usize] & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                            current_block_30 = 7427571413727699167;
                        } else {
                            current_block_30 = 3118944067848530783;
                        }
                    }
                    _ => {}
                }
                match current_block_30 {
                    3118944067848530783 => {
                        s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, tbl, nb);
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
                if 0 != s {
                    k += r;
                    if bits_left < s {
                        if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
                    s = (if r < 1i32 << s - 1i32 {
                        (r as c_uint)
                            .wrapping_add(((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint))
                    } else {
                        r as c_uint
                    }) as c_int;
                    (*block)[*jpeg_natural_order.as_ptr().offset(k as isize) as usize] =
                        ((s as c_ulong) << Al) as JLONG as JCOEF
                } else if r == 15i32 {
                    k += 15i32
                } else {
                    /* EOBr, run length is 2^r + appended bits */
                    EOBRUN = (1i32 << r) as c_uint;
                    if 0 != r {
                        if bits_left < r {
                            if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, r) {
                                return 0i32;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= r;
                        r = (get_buffer >> bits_left) as c_int & (1i32 << r) - 1i32;
                        EOBRUN = EOBRUN.wrapping_add(r as c_uint)
                    }
                    EOBRUN = EOBRUN.wrapping_sub(1);
                    /* force end-of-band */
                    break;
                }
                k += 1
            }
            (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
            (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
            (*entropy).bitstate.get_buffer = get_buffer;
            (*entropy).bitstate.bits_left = bits_left
        }
        (*entropy).saved.EOBRUN = EOBRUN
    }
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return TRUE;
}
/*
 * MCU decoding for DC successive approximation refinement scan.
 * Note: we assume such scans can be multi-component, although the spec
 * is not very clear on the point.
 */
unsafe extern "C" fn decode_mcu_DC_refine(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    /* 1 in the bit position being coded */
    let mut p1: c_int = 1i32 << (*cinfo).Al;
    let mut blkn: c_int = 0;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if 0 == process_restart(cinfo) {
                return FALSE;
            }
        }
    }
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        if bits_left < 1i32 {
            if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 1i32) {
                return 0i32;
            }
            get_buffer = br_state.get_buffer;
            bits_left = br_state.bits_left
        }
        bits_left -= 1i32;
        if 0 != (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 {
            (*block)[0usize] = ((*block)[0usize] as c_int | p1) as JCOEF
        }
        blkn += 1
    }
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return TRUE;
}
/*
 * MCU decoding for AC successive approximation refinement scan.
 */
unsafe extern "C" fn decode_mcu_AC_refine(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut current_block: u64;
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut Se: c_int = (*cinfo).Se;
    /* 1 in the bit position being coded */
    let mut p1: c_int = 1i32 << (*cinfo).Al;
    /* -1 in the bit position being coded */
    let mut m1: c_int = ((-1i32 as c_uint) << (*cinfo).Al) as c_int;
    let mut s: c_int = 0;
    let mut k: c_int = 0;
    let mut r: c_int = 0;
    let mut EOBRUN: c_uint = 0;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut thiscoef: JCOEFPTR = 0 as *mut JCOEF;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    let mut tbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    let mut num_newnz: c_int = 0;
    let mut newnz_pos: [c_int; 64] = [0; 64];
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if 0 == process_restart(cinfo) {
                return FALSE;
            }
        }
    }
    /* If we've run out of data, don't modify the MCU.
     */
    if 0 == (*entropy).pub_0.insufficient_data {
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        EOBRUN = (*entropy).saved.EOBRUN;
        block = *MCU_data.offset(0isize);
        tbl = (*entropy).ac_derived_tbl;
        num_newnz = 0i32;
        k = (*cinfo).Ss;
        if EOBRUN == 0i32 as c_uint {
            current_block = 10652014663920648156;
        } else {
            current_block = 17958840340921835115;
        }
        's_120: loop {
            match current_block {
                17958840340921835115 => {
                    if !(EOBRUN > 0i32 as c_uint) {
                        current_block = 10041771570435381152;
                        break;
                    }
                    /* Scan any remaining coefficient positions after the end-of-band
                     * (the last newly nonzero coefficient, if any).  Append a correction
                     * bit to each already-nonzero coefficient.  A correction bit is 1
                     * if the absolute value of the coefficient must be increased.
                     */
                    current_block = 12369290732426379360;
                    break;
                }
                _ => {
                    if !(k <= Se) {
                        current_block = 17958840340921835115;
                        continue;
                    }
                    let mut nb: c_int = 0;
                    let mut look: c_int = 0;
                    if bits_left < HUFF_LOOKAHEAD {
                        if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) {
                            current_block = 6153397765503504804;
                            break;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                        if bits_left < HUFF_LOOKAHEAD {
                            nb = 1i32;
                            current_block = 15705034766489281697;
                        } else {
                            current_block = 17500079516916021833;
                        }
                    } else {
                        current_block = 17500079516916021833;
                    }
                    match current_block {
                        17500079516916021833 => {
                            look =
                                (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                            nb = (*tbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                            if nb <= HUFF_LOOKAHEAD {
                                bits_left -= nb;
                                s = (*tbl).lookup[look as usize] & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                                current_block = 2543120759711851213;
                            } else {
                                current_block = 15705034766489281697;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        15705034766489281697 => {
                            s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, tbl, nb);
                            if s < 0i32 {
                                current_block = 6153397765503504804;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        _ => {}
                    }
                    r = s >> 4i32;
                    s &= 15i32;
                    if 0 != s {
                        if s != 1i32 {
                            (*(*cinfo).err).msg_code = JWRN_HUFF_BAD_CODE as c_int;
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer")(
                                cinfo as j_common_ptr, -1i32
                            );
                        }
                        if bits_left < 1i32 {
                            if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 1i32)
                            {
                                current_block = 6153397765503504804;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= 1i32;
                        if 0 != (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 {
                            s = p1
                        } else {
                            s = m1
                        }
                    } else if r != 15i32 {
                        EOBRUN = (1i32 << r) as c_uint;
                        if 0 != r {
                            if bits_left < r {
                                if 0 == jpeg_fill_bit_buffer(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    r,
                                ) {
                                    current_block = 6153397765503504804;
                                    break;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= r;
                            r = (get_buffer >> bits_left) as c_int & (1i32 << r) - 1i32;
                            EOBRUN = EOBRUN.wrapping_add(r as c_uint)
                        }
                        /* rest of block is handled by EOB logic */
                        current_block = 17958840340921835115;
                        continue;
                    }
                    /* note s = 0 for processing ZRL */
                    /* Advance over already-nonzero coefs and r still-zero coefs,
                     * appending correction bits to the nonzeroes.  A correction bit is 1
                     * if the absolute value of the coefficient must be increased.
                     */
                    loop {
                        thiscoef = (*block)
                            .as_mut_ptr()
                            .offset(*jpeg_natural_order.as_ptr().offset(k as isize) as isize);
                        if *thiscoef as c_int != 0i32 {
                            if bits_left < 1i32 {
                                if 0 == jpeg_fill_bit_buffer(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1i32,
                                ) {
                                    current_block = 6153397765503504804;
                                    break 's_120;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= 1i32;
                            if 0 != (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 {
                                if *thiscoef as c_int & p1 == 0i32 {
                                    if *thiscoef as c_int >= 0i32 {
                                        *thiscoef = (*thiscoef as c_int + p1) as JCOEF
                                    } else {
                                        *thiscoef = (*thiscoef as c_int + m1) as JCOEF
                                    }
                                }
                            }
                        } else {
                            r -= 1;
                            if r < 0i32 {
                                /* reached target zero coefficient */
                                break;
                            }
                        }
                        k += 1;
                        if !(k <= Se) {
                            break;
                        }
                    }
                    if 0 != s {
                        let mut pos: c_int = *jpeg_natural_order.as_ptr().offset(k as isize);
                        (*block)[pos as usize] = s as JCOEF;
                        let fresh0 = num_newnz;
                        num_newnz = num_newnz + 1;
                        newnz_pos[fresh0 as usize] = pos
                    }
                    k += 1;
                    current_block = 10652014663920648156;
                }
            }
        }
        loop {
            match current_block {
                6153397765503504804 => {
                    while num_newnz > 0i32 {
                        num_newnz -= 1;
                        (*block)[newnz_pos[num_newnz as usize] as usize] = 0i32 as JCOEF
                    }
                    return FALSE;
                }
                12369290732426379360 => {
                    if k <= Se {
                        thiscoef = (*block)
                            .as_mut_ptr()
                            .offset(*jpeg_natural_order.as_ptr().offset(k as isize) as isize);
                        if *thiscoef as c_int != 0i32 {
                            if bits_left < 1i32 {
                                if 0 == jpeg_fill_bit_buffer(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1i32,
                                ) {
                                    current_block = 6153397765503504804;
                                    continue;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= 1i32;
                            if 0 != (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 {
                                if *thiscoef as c_int & p1 == 0i32 {
                                    if *thiscoef as c_int >= 0i32 {
                                        *thiscoef = (*thiscoef as c_int + p1) as JCOEF
                                    } else {
                                        *thiscoef = (*thiscoef as c_int + m1) as JCOEF
                                    }
                                }
                            }
                        }
                        k += 1;
                        current_block = 12369290732426379360;
                    } else {
                        EOBRUN = EOBRUN.wrapping_sub(1);
                        current_block = 10041771570435381152;
                    }
                }
                _ => {
                    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
                    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
                    (*entropy).bitstate.get_buffer = get_buffer;
                    (*entropy).bitstate.bits_left = bits_left;
                    (*entropy).saved.EOBRUN = EOBRUN;
                    break;
                }
            }
        }
    }
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return TRUE;
}
/*
 * Module initialization routine for progressive Huffman entropy decoding.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_phuff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: phuff_entropy_ptr = 0 as *mut phuff_entropy_decoder;
    let mut coef_bit_ptr: *mut c_int = 0 as *mut c_int;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    entropy = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<phuff_entropy_decoder>() as c_ulong,
    ) as phuff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass_phuff_decoder as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        (*entropy).derived_tbls[i as usize] = NULL as *mut d_derived_tbl;
        i += 1
    }
    (*cinfo).coef_bits = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (((*cinfo).num_components * DCTSIZE2) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut [c_int; 64];
    coef_bit_ptr = &mut *(*(*cinfo).coef_bits.offset(0isize))
        .as_mut_ptr()
        .offset(0isize) as *mut c_int;
    ci = 0i32;
    while ci < (*cinfo).num_components {
        i = 0i32;
        while i < DCTSIZE2 {
            let fresh1 = coef_bit_ptr;
            coef_bit_ptr = coef_bit_ptr.offset(1);
            *fresh1 = -1i32;
            i += 1
        }
        ci += 1
    }
}
