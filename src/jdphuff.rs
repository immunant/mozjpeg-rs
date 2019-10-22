pub use super::jdhuff::{
    bit_buf_type, bitread_perm_state, bitread_working_state, d_derived_tbl, jpeg_fill_bit_buffer,
    jpeg_huff_decode, jpeg_make_d_derived_tbl, HUFF_LOOKAHEAD,
};
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
pub use crate::__INT_MAX__;
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jpeg_natural_order, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE2, JBLOCK,
    JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
    JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
    JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
    JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
    JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
    NUM_HUFF_TBLS,
};
pub use crate::limits_h::{INT_MAX, INT_MIN};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_uint, c_ulong};

pub type phuff_entropy_ptr = *mut phuff_entropy_decoder;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phuff_entropy_decoder {
    pub pub_0: jpeg_entropy_decoder,
    pub bitstate: super::jdhuff::bitread_perm_state,
    pub saved: savable_state,
    pub restarts_to_go: c_uint,
    pub derived_tbls: [*mut super::jdhuff::d_derived_tbl; 4],
    pub ac_derived_tbl: *mut super::jdhuff::d_derived_tbl,
}

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
    let mut pdtbl: *mut *mut super::jdhuff::d_derived_tbl =
        0 as *mut *mut super::jdhuff::d_derived_tbl;
    let mut coef_bit_ptr: *mut c_int = 0 as *mut c_int;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    is_DC_band = ((*cinfo).Ss == 0i32) as c_int;
    /* Validate scan parameters */
    bad = FALSE;
    if is_DC_band != 0 {
        if (*cinfo).Se != 0i32 {
            bad = TRUE
        }
    } else {
        /* need not check Ss/Se < 0 since they came from unsigned bytes */
        if (*cinfo).Ss > (*cinfo).Se || (*cinfo).Se >= DCTSIZE2 {
            bad = TRUE
        }
        /* AC scans may have only one component */
        if (*cinfo).comps_in_scan != 1i32 {
            bad = TRUE
        }
    }
    if (*cinfo).Ah != 0i32 {
        /* Successive approximation refinement scan: must have Al = Ah-1. */
        if (*cinfo).Al != (*cinfo).Ah - 1i32 {
            bad = TRUE
        }
    }
    if (*cinfo).Al > 13i32 {
        /* need not check for < 0 */
        bad = TRUE
    }
    /* Arguably the maximum Al value should be less than 13 for 8-bit precision,
     * but the spec doesn't say so, and we try to be liberal about what we
     * accept.  Note: large Al values could result in out-of-range DC
     * coefficients during early scans, leading to bizarre displays due to
     * overflows in the IDCT math.  But we won't crash.
     */
    if bad != 0 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PROGRESSION as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).Ss;
        (*(*cinfo).err).msg_parm.i[1] = (*cinfo).Se;
        (*(*cinfo).err).msg_parm.i[2] = (*cinfo).Ah;
        (*(*cinfo).err).msg_parm.i[3] = (*cinfo).Al;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Update progression status, and verify that scan order is legal.
     * Note that inter-scan inconsistencies are treated as warnings
     * not fatal errors ... not clear if this is right way to behave.
     */
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        let mut cindex: c_int = (*(*cinfo).cur_comp_info[ci as usize]).component_index;
        coef_bit_ptr = &mut *(*(*cinfo).coef_bits.offset(cindex as isize))
            .as_mut_ptr()
            .offset(0) as *mut c_int;
        if is_DC_band == 0 && *coef_bit_ptr.offset(0) < 0i32 {
            /* AC without prior DC scan */
            (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_PROGRESSION as c_int;
            (*(*cinfo).err).msg_parm.i[0] = cindex;
            (*(*cinfo).err).msg_parm.i[1] = 0i32;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
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
                (*(*cinfo).err).msg_code = super::jerror::JWRN_BOGUS_PROGRESSION as c_int;
                (*(*cinfo).err).msg_parm.i[0] = cindex;
                (*(*cinfo).err).msg_parm.i[1] = coefi;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
            }
            *coef_bit_ptr.offset(coefi as isize) = (*cinfo).Al;
            coefi += 1
        }
        ci += 1
    }
    /* Select MCU decoding routine */
    if (*cinfo).Ah == 0i32 {
        if is_DC_band != 0 {
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
    } else if is_DC_band != 0 {
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
        /* Make sure requested tables are present, and compute derived tables.
         * We may build same derived table more than once, but it's not expensive.
         */
        if is_DC_band != 0 {
            if (*cinfo).Ah == 0i32 {
                /* DC refinement needs no table */
                tbl = (*compptr).dc_tbl_no;
                pdtbl = (*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize);
                super::jdhuff::jpeg_make_d_derived_tbl(cinfo, TRUE, tbl, pdtbl);
            }
        } else {
            tbl = (*compptr).ac_tbl_no;
            pdtbl = (*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize);
            super::jdhuff::jpeg_make_d_derived_tbl(cinfo, FALSE, tbl, pdtbl);
            /* remember the single active table */
            (*entropy).ac_derived_tbl = (*entropy).derived_tbls[tbl as usize]
        }
        /* Initialize DC predictions to 0 */
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    /* Initialize bitread state variables */
    (*entropy).bitstate.bits_left = 0i32; /* unnecessary, but keeps Purify quiet */
    (*entropy).bitstate.get_buffer = 0i32 as super::jdhuff::bit_buf_type;
    (*entropy).pub_0.insufficient_data = FALSE;
    /* Initialize private state variables */
    (*entropy).saved.EOBRUN = 0i32 as c_uint;
    /* Initialize restart counter */
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
    /* Throw away any unused bits remaining in bit buffer; */
    /* include any full bytes in next_marker's count of discarded bytes */
    (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
        .discarded_bytes
        .wrapping_add(((*entropy).bitstate.bits_left / 8i32) as c_uint);
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
        return FALSE;
    }
    /* Re-initialize DC predictions to 0 */
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    /* Re-init EOB run count, too */
    (*entropy).saved.EOBRUN = 0i32 as c_uint;
    /* Reset restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    /* Reset out-of-data flag, unless read_restart_marker left us smack up
     * against a marker.  In that case we will end up treating the next data
     * segment as empty, and we can avoid producing bogus output pixels by
     * leaving the flag set.
     */
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
    let mut get_buffer: super::jdhuff::bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: super::jdhuff::bitread_working_state = super::jdhuff::bitread_working_state {
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
    let mut tbl: *mut super::jdhuff::d_derived_tbl = 0 as *mut super::jdhuff::d_derived_tbl;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).pub_0.insufficient_data == 0 {
        /* Load up working state */
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        state = (*entropy).saved;
        /* Outer loop handles each block in the MCU */
        blkn = 0i32;
        while blkn < (*cinfo).blocks_in_MCU {
            block = *MCU_data.offset(blkn as isize);
            ci = (*cinfo).MCU_membership[blkn as usize];
            compptr = (*cinfo).cur_comp_info[ci as usize];
            tbl = (*entropy).derived_tbls[(*compptr).dc_tbl_no as usize];
            let mut current_block_31: u64;
            /* Decode a single block's worth of coefficients */
            /* Section F.2.2.1: decode the DC coefficient difference */
            let mut nb: c_int = 0;
            let mut look: c_int = 0;
            if bits_left < super::jdhuff::HUFF_LOOKAHEAD {
                if super::jdhuff::jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32)
                    == 0
                {
                    return 0i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left;
                if bits_left < super::jdhuff::HUFF_LOOKAHEAD {
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
                    nb = (*tbl).lookup[look as usize] >> super::jdhuff::HUFF_LOOKAHEAD;
                    if nb <= super::jdhuff::HUFF_LOOKAHEAD {
                        bits_left -= nb;
                        s = (*tbl).lookup[look as usize]
                            & (1i32 << super::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                        current_block_31 = 17784502470059252271;
                    } else {
                        current_block_31 = 18387791726498337928;
                    }
                }
                _ => {}
            }
            match current_block_31 {
                18387791726498337928 => {
                    s = super::jdhuff::jpeg_huff_decode(
                        &mut br_state,
                        get_buffer,
                        bits_left,
                        tbl,
                        nb,
                    );
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
                    if super::jdhuff::jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s)
                        == 0
                    {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left
                }
                bits_left -= s;
                r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
                s = if r < 1i32 << s - 1i32 {
                    (r as c_uint)
                        .wrapping_add(((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint))
                } else {
                    r as c_uint
                } as c_int
            }
            /* Convert DC difference to actual value, update last_dc_val */
            if state.last_dc_val[ci as usize] >= 0i32
                && s > INT_MAX - state.last_dc_val[ci as usize]
                || state.last_dc_val[ci as usize] < 0i32
                    && s < INT_MIN - state.last_dc_val[ci as usize]
            {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_DCT_COEF as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            /* Scale and output the coefficient (assumes jpeg_natural_order[0]=0) */
            (*block)[0] = ((s as c_ulong) << Al) as JLONG as JCOEF;
            blkn += 1
        }
        /* Completed MCU, so update state */
        (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
        (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
        (*entropy).bitstate.get_buffer = get_buffer;
        (*entropy).bitstate.bits_left = bits_left;
        (*entropy).saved = state
    }
    /* Account for restart interval (no-op if not using restarts) */
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
    let mut get_buffer: super::jdhuff::bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: super::jdhuff::bitread_working_state = super::jdhuff::bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    let mut tbl: *mut super::jdhuff::d_derived_tbl = 0 as *mut super::jdhuff::d_derived_tbl;
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).pub_0.insufficient_data == 0 {
        /* Load up working state.
         * We can avoid loading/saving bitread state if in an EOB run.
         */
        EOBRUN = (*entropy).saved.EOBRUN; /* only part of saved state we need */
        /* only part of saved state we need */
        if EOBRUN > 0i32 as c_uint {
            /* There is always only one block per MCU */
            /* if it's a band of zeroes... */
            EOBRUN = EOBRUN.wrapping_sub(1)
        } else {
            br_state.cinfo = cinfo; /* ...process it now (we do nothing) */
            br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
            br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
            get_buffer = (*entropy).bitstate.get_buffer;
            bits_left = (*entropy).bitstate.bits_left;
            block = *MCU_data.offset(0);
            tbl = (*entropy).ac_derived_tbl;
            k = (*cinfo).Ss;
            while k <= Se {
                let mut current_block_30: u64;
                let mut nb: c_int = 0;
                let mut look: c_int = 0;
                if bits_left < super::jdhuff::HUFF_LOOKAHEAD {
                    if super::jdhuff::jpeg_fill_bit_buffer(
                        &mut br_state,
                        get_buffer,
                        bits_left,
                        0i32,
                    ) == 0
                    {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < super::jdhuff::HUFF_LOOKAHEAD {
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
                        nb = (*tbl).lookup[look as usize] >> super::jdhuff::HUFF_LOOKAHEAD;
                        if nb <= super::jdhuff::HUFF_LOOKAHEAD {
                            bits_left -= nb;
                            s = (*tbl).lookup[look as usize]
                                & (1i32 << super::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                            current_block_30 = 7427571413727699167;
                        } else {
                            current_block_30 = 3118944067848530783;
                        }
                    }
                    _ => {}
                }
                match current_block_30 {
                    3118944067848530783 => {
                        s = super::jdhuff::jpeg_huff_decode(
                            &mut br_state,
                            get_buffer,
                            bits_left,
                            tbl,
                            nb,
                        );
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
                        if super::jdhuff::jpeg_fill_bit_buffer(
                            &mut br_state,
                            get_buffer,
                            bits_left,
                            s,
                        ) == 0
                        {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
                    s = if r < 1i32 << s - 1i32 {
                        (r as c_uint)
                            .wrapping_add(((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint))
                    } else {
                        r as c_uint
                    } as c_int;
                    /* Scale and output coefficient in natural (dezigzagged) order */
                    (*block)[*jpeg_natural_order.as_ptr().offset(k as isize) as usize] =
                        ((s as c_ulong) << Al) as JLONG as JCOEF
                } else if r == 15i32 {
                    /* ZRL */
                    k += 15i32
                /* skip 15 zeroes in band */
                } else {
                    EOBRUN = (1i32 << r) as c_uint;
                    if r != 0 {
                        /* force end-of-band */
                        /* EOBr, r > 0 */
                        if bits_left < r {
                            if super::jdhuff::jpeg_fill_bit_buffer(
                                &mut br_state,
                                get_buffer,
                                bits_left,
                                r,
                            ) == 0
                            {
                                return 0i32;
                            } /* this band is processed at this moment */
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= r;
                        r = (get_buffer >> bits_left) as c_int & (1i32 << r) - 1i32;
                        EOBRUN = EOBRUN.wrapping_add(r as c_uint)
                    }
                    EOBRUN = EOBRUN.wrapping_sub(1);
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
    /* Completed MCU, so update state */
    /* Account for restart interval (no-op if not using restarts) */
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
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr; /* 1 in the bit position being coded */
    let mut p1: c_int = 1i32 << (*cinfo).Al;
    let mut blkn: c_int = 0;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut get_buffer: super::jdhuff::bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: super::jdhuff::bitread_working_state = super::jdhuff::bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    /* Not worth the cycles to check insufficient_data here,
     * since we will not change the data anyway if we read zeroes.
     */
    /* Load up working state */
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    /* Outer loop handles each block in the MCU */
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        /* Note: since we use |=, repeating the assignment later is safe */
        if bits_left < 1i32 {
            if super::jdhuff::jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 1i32) == 0
            {
                return 0i32;
            }
            get_buffer = br_state.get_buffer;
            bits_left = br_state.bits_left
        }
        bits_left -= 1i32;
        if (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 != 0 {
            (*block)[0] = ((*block)[0] as c_int | p1) as JCOEF
        }
        blkn += 1
    }
    /* Encoded data is simply the next bit of the two's-complement DC value */
    /* Completed MCU, so update state */
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    /* Account for restart interval (no-op if not using restarts) */
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
    let mut current_block: u64; /* 1 in the bit position being coded */
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr; /* -1 in the bit position being coded */
    let mut Se: c_int = (*cinfo).Se;
    let mut p1: c_int = 1i32 << (*cinfo).Al;
    let mut m1: c_int = ((-1i32 as c_uint) << (*cinfo).Al) as c_int;
    let mut s: c_int = 0;
    let mut k: c_int = 0;
    let mut r: c_int = 0;
    let mut EOBRUN: c_uint = 0;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut thiscoef: JCOEFPTR = 0 as *mut JCOEF;
    let mut get_buffer: super::jdhuff::bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: super::jdhuff::bitread_working_state = super::jdhuff::bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    let mut tbl: *mut super::jdhuff::d_derived_tbl = 0 as *mut super::jdhuff::d_derived_tbl;
    let mut num_newnz: c_int = 0;
    let mut newnz_pos: [c_int; 64] = [0; 64];
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    /* If we've run out of data, don't modify the MCU.
     */
    if (*entropy).pub_0.insufficient_data == 0 {
        /* Load up working state */
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        /* only part of saved state we need */
        EOBRUN = (*entropy).saved.EOBRUN; /* only part of saved state we need */
        /* There is always only one block per MCU */
        block = *MCU_data.offset(0);
        tbl = (*entropy).ac_derived_tbl;
        /* If we are forced to suspend, we must undo the assignments to any newly
         * nonzero coefficients in the block, because otherwise we'd get confused
         * next time about which coefficients were already nonzero.
         * But we need not undo addition of bits to already-nonzero coefficients;
         * instead, we can test the current bit to see if we already did it.
         */
        num_newnz = 0i32;
        /* initialize coefficient loop counter to start of band */
        k = (*cinfo).Ss;
        if EOBRUN == 0i32 as c_uint {
            current_block = 10652014663920648156;
        } else {
            current_block = 17958840340921835115;
        }
        's_120: loop {
            match current_block {
                17958840340921835115 => {
                    if EOBRUN > 0i32 as c_uint {
                        current_block = 12369290732426379360;
                        break;
                    } else {
                        current_block = 10041771570435381152;
                        break;
                    }
                }
                _ => {
                    if !(k <= Se) {
                        current_block = 17958840340921835115;
                        continue;
                    }
                    let mut nb: c_int = 0;
                    let mut look: c_int = 0;
                    if bits_left < super::jdhuff::HUFF_LOOKAHEAD {
                        if super::jdhuff::jpeg_fill_bit_buffer(
                            &mut br_state,
                            get_buffer,
                            bits_left,
                            0i32,
                        ) == 0
                        {
                            current_block = 6153397765503504804;
                            break;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                        if bits_left < super::jdhuff::HUFF_LOOKAHEAD {
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
                            nb = (*tbl).lookup[look as usize] >> super::jdhuff::HUFF_LOOKAHEAD;
                            if nb <= super::jdhuff::HUFF_LOOKAHEAD {
                                bits_left -= nb;
                                s = (*tbl).lookup[look as usize]
                                    & (1i32 << super::jdhuff::HUFF_LOOKAHEAD) - 1i32;
                                current_block = 2543120759711851213;
                            } else {
                                current_block = 15705034766489281697;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        15705034766489281697 => {
                            s = super::jdhuff::jpeg_huff_decode(
                                &mut br_state,
                                get_buffer,
                                bits_left,
                                tbl,
                                nb,
                            );
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
                    if s != 0 {
                        if s != 1i32 {
                            /* size of new coef should always be 1 */
                            (*(*cinfo).err).msg_code = super::jerror::JWRN_HUFF_BAD_CODE as c_int;
                            Some(
                                (*(*cinfo).err)
                                    .emit_message
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                cinfo as j_common_ptr, -1i32
                            );
                        }
                        if bits_left < 1i32 {
                            if super::jdhuff::jpeg_fill_bit_buffer(
                                &mut br_state,
                                get_buffer,
                                bits_left,
                                1i32,
                            ) == 0
                            {
                                current_block = 6153397765503504804;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= 1i32;
                        if (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 != 0 {
                            /* newly nonzero coef is negative */
                            s = p1
                        } else {
                            s = m1
                        }
                    } else if r != 15i32 {
                        /* newly nonzero coef is positive */
                        EOBRUN = (1i32 << r) as c_uint;
                        if !(r != 0) {
                            current_block = 17958840340921835115; /* EOBr, run length is 2^r + appended bits */
                            continue;
                        }
                        if bits_left < r {
                            if super::jdhuff::jpeg_fill_bit_buffer(
                                &mut br_state,
                                get_buffer,
                                bits_left,
                                r,
                            ) == 0
                            {
                                current_block = 6153397765503504804;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= r;
                        r = (get_buffer >> bits_left) as c_int & (1i32 << r) - 1i32;
                        EOBRUN = EOBRUN.wrapping_add(r as c_uint);
                        current_block = 17958840340921835115;
                        continue;
                        /* rest of block is handled by EOB logic */
                    }
                    loop
                    /* note s = 0 for processing ZRL */
                    /* Advance over already-nonzero coefs and r still-zero coefs,
                     * appending correction bits to the nonzeroes.  A correction bit is 1
                     * if the absolute value of the coefficient must be increased.
                     */
                    {
                        thiscoef = (*block)
                            .as_mut_ptr()
                            .offset(*jpeg_natural_order.as_ptr().offset(k as isize) as isize);
                        if *thiscoef as c_int != 0i32 {
                            if bits_left < 1i32 {
                                if super::jdhuff::jpeg_fill_bit_buffer(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1i32,
                                ) == 0
                                {
                                    current_block = 6153397765503504804;
                                    break 's_120;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= 1i32;
                            if (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 != 0 {
                                if *thiscoef as c_int & p1 == 0i32 {
                                    /* do nothing if already set it */
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
                                break;
                                /* reached target zero coefficient */
                            }
                        }
                        k += 1;
                        if !(k <= Se) {
                            break;
                        }
                    }
                    if s != 0 {
                        let mut pos: c_int = *jpeg_natural_order.as_ptr().offset(k as isize);
                        /* Output newly nonzero coefficient */
                        (*block)[pos as usize] = s as JCOEF;
                        /* Remember its position in case we have to suspend */
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
                    /* Re-zero any output coefficients that we made newly nonzero */
                    while num_newnz > 0i32 {
                        num_newnz -= 1;
                        (*block)[newnz_pos[num_newnz as usize] as usize] = 0i32 as JCOEF
                    }
                    return FALSE;
                }
                12369290732426379360 =>
                /* Scan any remaining coefficient positions after the end-of-band
                 * (the last newly nonzero coefficient, if any).  Append a correction
                 * bit to each already-nonzero coefficient.  A correction bit is 1
                 * if the absolute value of the coefficient must be increased.
                 */
                {
                    if k <= Se {
                        thiscoef = (*block)
                            .as_mut_ptr()
                            .offset(*jpeg_natural_order.as_ptr().offset(k as isize) as isize);
                        if *thiscoef as c_int != 0i32 {
                            if bits_left < 1i32 {
                                if super::jdhuff::jpeg_fill_bit_buffer(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1i32,
                                ) == 0
                                {
                                    current_block = 6153397765503504804;
                                    continue;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= 1i32;
                            if (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 != 0 {
                                if *thiscoef as c_int & p1 == 0i32 {
                                    /* do nothing if already changed it */
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
                        /* Count one block completed in EOB run */
                        EOBRUN = EOBRUN.wrapping_sub(1);
                        current_block = 10041771570435381152;
                    }
                }
                _ => {
                    /* Completed MCU, so update state */
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
    /* Account for restart interval (no-op if not using restarts) */
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
    entropy = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<phuff_entropy_decoder>() as c_ulong,
    ) as phuff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass_phuff_decoder as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    /* Mark derived tables unallocated */
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        (*entropy).derived_tbls[i as usize] = NULL as *mut super::jdhuff::d_derived_tbl;
        i += 1
    }
    /* Create progression status table */
    (*cinfo).coef_bits = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (((*cinfo).num_components * DCTSIZE2) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut [c_int; 64];
    coef_bit_ptr = &mut *(*(*cinfo).coef_bits.offset(0)).as_mut_ptr().offset(0) as *mut c_int;
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
/* D_PROGRESSIVE_SUPPORTED */
