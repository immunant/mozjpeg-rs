use libc::c_uint;use libc::c_ulong;use libc::c_int;use libc;

pub use crate::internal::__INT_MAX__;
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
pub use super::jdhuff::bit_buf_type;
pub use super::jdhuff::bitread_perm_state;
pub use super::jdhuff::bitread_working_state;
pub use super::jdhuff::d_derived_tbl;
pub use super::jdhuff::jpeg_fill_bit_buffer;
pub use super::jdhuff::jpeg_huff_decode;
pub use super::jdhuff::jpeg_make_d_derived_tbl;
pub use super::jdhuff::HUFF_LOOKAHEAD;
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
    
    
    
    
    
    
    
    
    
     let mut is_DC_band:   boolean =
     ((*cinfo).Ss == 0i32) as c_int; let mut bad:   boolean =  FALSE;
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
     let mut ci:   c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
          let mut cindex: c_int = (*(*cinfo).cur_comp_info[ci as usize]).component_index;
         let mut coef_bit_ptr:   *mut c_int =
     &mut *(*(*cinfo).coef_bits.offset(cindex as isize))
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
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, -1i32
            );
        }
         let mut coefi:   c_int =  (*cinfo).Ss;
        while coefi <= (*cinfo).Se {
            let mut expected: c_int = if *coef_bit_ptr.offset(coefi as isize) < 0i32 {
                0i32
            } else {
                *coef_bit_ptr.offset(coefi as isize)
            };
            if (*cinfo).Ah != expected {
                (*(*cinfo).err).msg_code =
                    super::jerror::JWRN_BOGUS_PROGRESSION as c_int;
                (*(*cinfo).err).msg_parm.i[0] = cindex;
                (*(*cinfo).err).msg_parm.i[1] = coefi;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    -1i32,
                );
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
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut JBLOCKROW,
                    ) -> boolean,
            )
        } else {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_AC_first
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut JBLOCKROW,
                    ) -> boolean,
            )
        }
    } else if is_DC_band != 0 {
        (*entropy).pub_0.decode_mcu = Some(
            decode_mcu_DC_refine
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: *mut JBLOCKROW,
                ) -> boolean,
        )
    } else {
        (*entropy).pub_0.decode_mcu = Some(
            decode_mcu_AC_refine
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: *mut JBLOCKROW,
                ) -> boolean,
        )
    }
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
         let mut tbl:  c_int =  0; let mut pdtbl:  *mut *mut super::jdhuff::d_derived_tbl =
    
        ::std::ptr::null_mut::< *mut super::jdhuff::d_derived_tbl>();  let mut compptr:   *mut jpeg_component_info =
     (*cinfo).cur_comp_info[ci as usize];
        /* Make sure requested tables are present, and compute derived tables.
         * We may build same derived table more than once, but it's not expensive.
         */
        if is_DC_band != 0 {
            if (*cinfo).Ah == 0i32 {
                /* DC refinement needs no table */
                tbl = (*compptr).dc_tbl_no;
                pdtbl = (*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize);
                super::jdhuff::jpeg_make_d_derived_tbl(
                    cinfo,
                    TRUE,
                    tbl,
                    pdtbl,
                );
            }
        } else {
            tbl = (*compptr).ac_tbl_no;
            pdtbl = (*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize);
            super::jdhuff::jpeg_make_d_derived_tbl(
                cinfo,
                FALSE,
                tbl,
                pdtbl,
            );
            /* remember the single active table */
            (*entropy).ac_derived_tbl = (*entropy).derived_tbls[tbl as usize]
        }
        /* Initialize DC predictions to 0 */
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    /* Initialize bitread state variables */
    (*entropy).bitstate.bits_left = 0i32; /* unnecessary, but keeps Purify quiet */
    (*entropy).bitstate.get_buffer = 0u64;
    (*entropy).pub_0.insufficient_data = FALSE;
    /* Initialize private state variables */
    (*entropy).saved.EOBRUN = 0u32;
    /* Initialize restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
/* AVOID_TABLES */
/*
 * Check for a restart marker & resynchronize decoder.
 * Returns FALSE if must suspend.
 */

unsafe extern "C" fn process_restart(
    mut cinfo: j_decompress_ptr,
) -> boolean {
     let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    
    /* Throw away any unused bits remaining in bit buffer; */
    /* include any full bytes in next_marker's count of discarded bytes */
    (*(*cinfo).marker).discarded_bytes =  (*(*cinfo).marker)
        .discarded_bytes +
    ((*entropy).bitstate.bits_left / 8i32) as c_uint;
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
     let mut ci:   c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    /* Re-init EOB run count, too */
    (*entropy).saved.EOBRUN = 0u32;
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
    
    
    
    
    
    
    
    
    
    
    
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).pub_0.insufficient_data == 0 {
           let mut br_state:  super::jdhuff::bitread_working_state =
    
        super::jdhuff::bitread_working_state{next_input_byte:
                                          ::std::ptr::null::< JOCTET>(),
                                     bytes_in_buffer:  0,
                                     get_buffer:  0,
                                     bits_left:  0,
                                     cinfo:
                                          ::std::ptr::null_mut::< jpeg_decompress_struct>(),}; br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        
        
        
         let mut get_buffer:   super::jdhuff::bit_buf_type =
     (*entropy).bitstate.get_buffer; let mut bits_left:   c_int =  (*entropy).bitstate.bits_left; let mut state:   savable_state =  (*entropy).saved; let mut blkn:   c_int =  0i32;
        while blkn < (*cinfo).blocks_in_MCU {
             let mut s:  c_int =  0;     let mut current_block_31:  u64; let mut nb:  c_int =  0;
            
            
             let mut block:   JBLOCKROW =  *MCU_data.offset(blkn as isize); let mut ci:   c_int =  (*cinfo).MCU_membership[blkn as usize]; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).cur_comp_info[ci as usize]; let mut tbl:   *mut super::jdhuff::d_derived_tbl =
     (*entropy).derived_tbls[(*compptr).dc_tbl_no as usize];
            
            
            
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
                    current_block_31 = 18387791726498337928;
                } else {
                    current_block_31 = 5494826135382683477;
                }
            } else {
                current_block_31 = 5494826135382683477;
            }
            match current_block_31 {
                5494826135382683477 => {
                      let mut look:   c_int =
     (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
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
                 let mut r:   c_int =
     (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
                s = if r < 1i32 << s - 1i32 {
                    r as c_uint + (((((-1i32 as c_uint) << s))) + 1u32)
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
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            /* Scale and output the coefficient (assumes jpeg_natural_order[0]=0) */
            (*block)[0] =
                ((((s as c_ulong) << Al))) as JCOEF;
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
    (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1;
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
    
    
    
    
    
    
    
    
    
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
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
          let mut EOBRUN:   c_uint =  (*entropy).saved.EOBRUN; /* only part of saved state we need */
        /* only part of saved state we need */
        if EOBRUN > 0u32 {
            /* There is always only one block per MCU */
            /* if it's a band of zeroes... */
            EOBRUN -=  1
        } else {
                 let mut br_state:  super::jdhuff::bitread_working_state =
    
        super::jdhuff::bitread_working_state{next_input_byte:
                                          ::std::ptr::null::< JOCTET>(),
                                     bytes_in_buffer:  0,
                                     get_buffer:  0,
                                     bits_left:  0,
                                     cinfo:
                                          ::std::ptr::null_mut::< jpeg_decompress_struct>(),}; br_state.cinfo = cinfo; /* ...process it now (we do nothing) */
            br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
            br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
            
            
            
            
             let mut get_buffer:   super::jdhuff::bit_buf_type =
     (*entropy).bitstate.get_buffer; let mut bits_left:   c_int =  (*entropy).bitstate.bits_left; let mut block:   JBLOCKROW =  *MCU_data.offset(0); let mut tbl:   *mut super::jdhuff::d_derived_tbl =
     (*entropy).ac_derived_tbl; let mut k:   c_int =  (*cinfo).Ss;
            while k <= Se {
                
                
                 let mut s:  c_int =  0;  let mut current_block_30:  u64; let mut nb:  c_int =  0;
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
                          let mut look:   c_int =
    
                            (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
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
                 let mut r:   c_int =  s >> 4i32;
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
                        r as c_uint + (((((-1i32 as c_uint) << s))) + 1u32)
                    } else {
                        r as c_uint
                    } as c_int;
                    /* Scale and output coefficient in natural (dezigzagged) order */
                    (*block)[*jpeg_natural_order
                        .as_ptr()
                        .offset(k as isize) as usize] = ((((s as c_ulong) << Al)))
                        as JCOEF
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
                        EOBRUN +=  r as c_uint
                    }
                    EOBRUN -=  1;
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
    (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1;
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
        let mut br_state:  super::jdhuff::bitread_working_state =
    
        super::jdhuff::bitread_working_state{next_input_byte:
                                          ::std::ptr::null::< JOCTET>(),
                                     bytes_in_buffer:  0,
                                     get_buffer:  0,
                                     bits_left:  0,
                                     cinfo:
                                          ::std::ptr::null_mut::< jpeg_decompress_struct>(),};let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr; /* 1 in the bit position being coded */
    let mut p1: c_int = 1i32 << (*cinfo).Al;
    
    
    
    
    
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
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
    
    
     let mut get_buffer:   super::jdhuff::bit_buf_type =
     (*entropy).bitstate.get_buffer; let mut bits_left:   c_int =  (*entropy).bitstate.bits_left; let mut blkn:   c_int =  0i32;
    while blkn < (*cinfo).blocks_in_MCU {
          let mut block:   JBLOCKROW =  *MCU_data.offset(blkn as isize);
        /* Note: since we use |=, repeating the assignment later is safe */
        if bits_left < 1i32 {
            if super::jdhuff::jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 1i32)
                == 0
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
    (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1;
    return TRUE;
}
/*
 * MCU decoding for AC successive approximation refinement scan.
 */

unsafe extern "C" fn decode_mcu_AC_refine(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
     /* 1 in the bit position being coded */
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr; /* -1 in the bit position being coded */
    let mut Se: c_int = (*cinfo).Se;
    let mut p1: c_int = 1i32 << (*cinfo).Al;
    let mut m1: c_int = ((-1i32 as c_uint) << (*cinfo).Al) as c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    /* If we've run out of data, don't modify the MCU.
     */
    if (*entropy).pub_0.insufficient_data == 0 {
        /* Load up working state */
         let mut current_block:  u64;    let mut thiscoef:  JCOEFPTR =
     ::std::ptr::null_mut::< JCOEF>();   let mut br_state:  super::jdhuff::bitread_working_state =
    
        super::jdhuff::bitread_working_state{next_input_byte:
                                          ::std::ptr::null::< JOCTET>(),
                                     bytes_in_buffer:  0,
                                     get_buffer:  0,
                                     bits_left:  0,
                                     cinfo:
                                          ::std::ptr::null_mut::< jpeg_decompress_struct>(),};   let mut newnz_pos:  [c_int; 64] =  [0; 64];br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        
        
         /* only part of saved state we need */
        
        
        /* If we are forced to suspend, we must undo the assignments to any newly
         * nonzero coefficients in the block, because otherwise we'd get confused
         * next time about which coefficients were already nonzero.
         * But we need not undo addition of bits to already-nonzero coefficients;
         * instead, we can test the current bit to see if we already did it.
         */
        
         let mut get_buffer:   super::jdhuff::bit_buf_type =
     (*entropy).bitstate.get_buffer; let mut bits_left:   c_int =  (*entropy).bitstate.bits_left; let mut EOBRUN:   c_uint =  (*entropy).saved.EOBRUN; let mut block:   JBLOCKROW =  *MCU_data.offset(0); let mut tbl:   *mut super::jdhuff::d_derived_tbl =
     (*entropy).ac_derived_tbl; let mut num_newnz:   c_int =  0i32; let mut k:   c_int =  (*cinfo).Ss;
        if EOBRUN == 0u32 {
            current_block = 10652014663920648156;
        } else {
            current_block = 17958840340921835115;
        }
        's_120: loop {
            match current_block {
                17958840340921835115 => {
                    if EOBRUN > 0u32 {
                        current_block = 12369290732426379360;
                        break;
                    } else {
                        current_block = 10041771570435381152;
                        break;
                    }
                }
                _ => {
                     let mut s:  c_int =  0;  let mut nb:  c_int =  0;if !(k <= Se) {
                        current_block = 17958840340921835115;
                        continue;
                    }
                    
                    
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
                              let mut look:   c_int =
     (get_buffer >> bits_left - 8i32) as c_int
                                & (1i32 << 8i32) - 1i32;
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
                     let mut r:   c_int =  s >> 4i32;
                    s &= 15i32;
                    if s != 0 {
                        if s != 1i32 {
                            /* size of new coef should always be 1 */
                            (*(*cinfo).err).msg_code =
                                super::jerror::JWRN_HUFF_BAD_CODE as c_int;
                            Some(
                                (*(*cinfo).err)
                                    .emit_message
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                cinfo as j_common_ptr,
                                -1i32,
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
                        EOBRUN +=  r as c_uint;
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
                        thiscoef = (*block).as_mut_ptr().offset(
                            *jpeg_natural_order
                                .as_ptr()
                                .offset(k as isize) as isize,
                        );
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
                            if (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 != 0
                            {
                                if *thiscoef as c_int & p1 == 0i32 {
                                    /* do nothing if already set it */
                                    if *thiscoef as c_int >= 0i32 {
                                        *thiscoef = (*thiscoef as c_int + p1)
                                            as JCOEF
                                    } else {
                                        *thiscoef = (*thiscoef as c_int + m1)
                                            as JCOEF
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
                        let mut pos: c_int = *jpeg_natural_order
                            .as_ptr()
                            .offset(k as isize);
                        /* Output newly nonzero coefficient */
                        (*block)[pos as usize] = s as JCOEF;
                        /* Remember its position in case we have to suspend */
                        let fresh0 = num_newnz;
                        num_newnz +=  1;
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
                        (*block)[newnz_pos[num_newnz as usize] as usize] =
                            0i16
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
                        thiscoef = (*block).as_mut_ptr().offset(
                            *jpeg_natural_order
                                .as_ptr()
                                .offset(k as isize) as isize,
                        );
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
                            if (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32 != 0
                            {
                                if *thiscoef as c_int & p1 == 0i32 {
                                    /* do nothing if already changed it */
                                    if *thiscoef as c_int >= 0i32 {
                                        *thiscoef = (*thiscoef as c_int + p1)
                                            as JCOEF
                                    } else {
                                        *thiscoef = (*thiscoef as c_int + m1)
                                            as JCOEF
                                    }
                                }
                            }
                        }
                        k += 1;
                        current_block = 12369290732426379360;
                    } else {
                        /* Count one block completed in EOB run */
                        EOBRUN -=  1;
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
    (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1;
    return TRUE;
}
/*
 * Module initialization routine for progressive Huffman entropy decoding.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_phuff_decoder(mut cinfo: j_decompress_ptr) {
    
    
    
        
     let mut entropy:   phuff_entropy_ptr =
     Some(
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
    (*entropy).pub_0.start_pass = Some(
        start_pass_phuff_decoder
            as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
     let mut i:   c_int =  0i32;
    while i < NUM_HUFF_TBLS {
        (*entropy).derived_tbls[i as usize] =
            NULL as *mut super::jdhuff::d_derived_tbl;
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
        ((*cinfo).num_components * DCTSIZE2) as c_ulong *
    ::std::mem::size_of::<c_int>() as c_ulong,
    ) as *mut [c_int; 64];
    
     let mut coef_bit_ptr:   *mut c_int =
     &mut *(*(*cinfo).coef_bits.offset(0)).as_mut_ptr().offset(0) as *mut c_int; let mut ci:   c_int =  0i32;
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
