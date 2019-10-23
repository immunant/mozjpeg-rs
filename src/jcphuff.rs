


























































































































































































































use crate::stdlib::memset;use libc::{c_uint, c_ulong, c_char, c_long, c_void, c_int, self};pub use crate::jpeg_nbits_table_h::jpeg_nbits_table;pub use crate::stddef_h::{size_t, NULL};pub use crate::jpeglib_h::{j_common_ptr, j_compress_ptr,
                           jpeg_alloc_huff_table, jpeg_c_coef_controller,
                           jpeg_c_main_controller, jpeg_c_prep_controller,
                           jpeg_color_converter, jpeg_common_struct,
                           jpeg_comp_master, jpeg_component_info,
                           jpeg_compress_struct, jpeg_destination_mgr,
                           jpeg_downsampler, jpeg_entropy_encoder,
                           jpeg_error_mgr, jpeg_forward_dct,
                           jpeg_marker_writer, jpeg_memory_mgr,
                           jpeg_progress_mgr, jpeg_scan_info,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, DCTSIZE2, JBLOCK,
                           JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR,
                           JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
                           JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
                           JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
                           JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
                           JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
                           JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY,
                           JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
                           NUM_HUFF_TBLS};pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE,
                            UINT16, UINT8};pub use crate::internal::__CHAR_BIT__;pub use crate::limits_h::CHAR_BIT;pub use crate::jpegint_h::{jpeg_natural_order, JBUF_CRANK_DEST,
                           JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
                           JBUF_SAVE_SOURCE, J_BUF_MODE};pub use super::jchuff::{c_derived_tbl, jpeg_gen_optimal_table,
                        jpeg_make_c_derived_tbl, MAX_COEF_BITS};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
                        JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
                        JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID,
                        JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
                        JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE,
                        JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
                        JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION,
                        JERR_BAD_MCU_SIZE, JERR_BAD_PARAM,
                        JERR_BAD_PARAM_VALUE, JERR_BAD_POOL_ID,
                        JERR_BAD_PRECISION, JERR_BAD_PROGRESSION,
                        JERR_BAD_PROG_SCRIPT, JERR_BAD_SAMPLING,
                        JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE,
                        JERR_BAD_STRUCT_SIZE, JERR_BAD_VIRTUAL_ACCESS,
                        JERR_BUFFER_SIZE, JERR_CANT_SUSPEND,
                        JERR_CCIR601_NOTIMPL, JERR_COMPONENT_COUNT,
                        JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX,
                        JERR_DAC_VALUE, JERR_DHT_INDEX, JERR_DQT_INDEX,
                        JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE,
                        JERR_EOI_EXPECTED, JERR_FILE_READ, JERR_FILE_WRITE,
                        JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
                        JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG,
                        JERR_INPUT_EMPTY, JERR_INPUT_EOF,
                        JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA,
                        JERR_MODE_CHANGE, JERR_NOTIMPL, JERR_NOT_COMPILED,
                        JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                        JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                        JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                        JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                        JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                        JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                        JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
                        JERR_TFILE_SEEK, JERR_TFILE_WRITE,
                        JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
                        JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG,
                        JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
                        JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE,
                        JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
                        JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                        JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN, JTRC_EOI,
                        JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE,
                        JTRC_JFIF_EXTENSION, JTRC_JFIF_THUMBNAIL,
                        JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER,
                        JTRC_QUANTVALS, JTRC_QUANT_3_NCOLORS,
                        JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED,
                        JTRC_RECOVERY_ACTION, JTRC_RST, JTRC_SMOOTH_NOTIMPL,
                        JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                        JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE,
                        JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                        JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                        JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                        JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                        JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR,
                        JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
                        JWRN_TOO_MUCH_DATA};

pub type phuff_entropy_ptr = *mut phuff_entropy_encoder;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phuff_entropy_encoder {
    pub pub_0: jpeg_entropy_encoder,
    pub AC_first_prepare: Option<
        unsafe extern "C" fn(
            _: *const JCOEF,
            _: *const c_int,
            _: c_int,
            _: c_int,
            _: *mut JCOEF,
            _: *mut size_t,
        ) -> (),
    >,
    pub AC_refine_prepare: Option<
        unsafe extern "C" fn(
            _: *const JCOEF,
            _: *const c_int,
            _: c_int,
            _: c_int,
            _: *mut JCOEF,
            _: *mut size_t,
        ) -> c_int,
    >,
    pub gather_statistics: boolean,
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub put_buffer: size_t,
    pub put_bits: c_int,
    pub cinfo: j_compress_ptr,
    pub last_dc_val: [c_int; 4],
    pub ac_tbl_no: c_int,
    pub EOBRUN: c_uint,
    pub BE: c_uint,
    pub bit_buffer: *mut c_char,
    pub restarts_to_go: c_uint,
    pub next_restart_num: c_int,
    pub derived_tbls: [*mut super::jchuff::c_derived_tbl; 4],
    pub count_ptrs: [*mut c_long; 4],
}
/* MAX_CORR_BITS is the number of bits the AC refinement correction-bit
 * buffer can hold.  Larger sizes may slightly improve compression, but
 * 1000 is already well into the realm of overkill.
 * The minimum safe size is 64 bits.
 */

pub const MAX_CORR_BITS: c_int = 1000i32;
/* Count bit loop zeroes */
#[inline(always)]

unsafe extern "C" fn count_zeroes(mut x: *mut size_t) -> c_int {
     
     let mut result:   c_int =  (*x).trailing_zeros() as i32;
    *x >>= result;
    return result;
}
/*
 * Initialize for a Huffman-compressed scan using progressive JPEG.
 */

unsafe extern "C" fn start_pass_phuff(
    mut cinfo: j_compress_ptr,
    mut gather_statistics: boolean,
) {
      let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    
    
    
    
    (*entropy).cinfo = cinfo;
    (*entropy).gather_statistics = gather_statistics;
     let mut is_DC_band:   boolean =
     ((*cinfo).Ss == 0i32) as c_int;
    /* We assume jcmaster.c already validated the scan parameters. */
    /* Select execution routines */
    if (*cinfo).Ah == 0i32 {
        if is_DC_band != 0 {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_DC_first
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: *mut JBLOCKROW,
                    ) -> boolean,
            )
        } else {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_AC_first
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: *mut JBLOCKROW,
                    ) -> boolean,
            )
        }
        if super::simd::x86_64::jsimd::jsimd_can_encode_mcu_AC_first_prepare() != 0 {
            (*entropy).AC_first_prepare = Some(
                super::simd::x86_64::jsimd::jsimd_encode_mcu_AC_first_prepare
                    as unsafe extern "C" fn(
                        _: *const JCOEF,
                        _: *const c_int,
                        _: c_int,
                        _: c_int,
                        _: *mut JCOEF,
                        _: *mut size_t,
                    ) -> (),
            )
        } else {
            (*entropy).AC_first_prepare = Some(
                encode_mcu_AC_first_prepare
                    as unsafe extern "C" fn(
                        _: *const JCOEF,
                        _: *const c_int,
                        _: c_int,
                        _: c_int,
                        _: *mut JCOEF,
                        _: *mut size_t,
                    ) -> (),
            )
        }
    } else if is_DC_band != 0 {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_DC_refine
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: *mut JBLOCKROW,
                ) -> boolean,
        )
    } else {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_AC_refine
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: *mut JBLOCKROW,
                ) -> boolean,
        );
        if super::simd::x86_64::jsimd::jsimd_can_encode_mcu_AC_refine_prepare() != 0 {
            (*entropy).AC_refine_prepare = Some(
                super::simd::x86_64::jsimd::jsimd_encode_mcu_AC_refine_prepare
                    as unsafe extern "C" fn(
                        _: *const JCOEF,
                        _: *const c_int,
                        _: c_int,
                        _: c_int,
                        _: *mut JCOEF,
                        _: *mut size_t,
                    ) -> c_int,
            )
        } else {
            (*entropy).AC_refine_prepare = Some(
                encode_mcu_AC_refine_prepare
                    as unsafe extern "C" fn(
                        _: *const JCOEF,
                        _: *const c_int,
                        _: c_int,
                        _: c_int,
                        _: *mut JCOEF,
                        _: *mut size_t,
                    ) -> c_int,
            )
        }
        /* AC refinement needs a correction bit buffer */
        if (*entropy).bit_buffer.is_null() {
            (*entropy).bit_buffer = Some(
                (*(*cinfo).mem)
                    .alloc_small
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                MAX_CORR_BITS as c_ulong *
    ::std::mem::size_of::<c_char>() as c_ulong,
            ) as *mut c_char
        }
    }
    if gather_statistics != 0 {
        (*entropy).pub_0.finish_pass = Some(
            finish_pass_gather_phuff
                as unsafe extern "C" fn(_: j_compress_ptr) -> (),
        )
    } else {
        (*entropy).pub_0.finish_pass = Some(
            finish_pass_phuff as unsafe extern "C" fn(_: j_compress_ptr) -> (),
        )
    }
    
    /* Only DC coefficients may be interleaved, so cinfo->comps_in_scan = 1
     * for AC coefficients.
     */
     let mut ci:   c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
         let mut tbl:  c_int =  0;  let mut current_block_45:  u64; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).cur_comp_info[ci as usize];
        /* Initialize DC predictions to 0 */
        (*entropy).last_dc_val[ci as usize] = 0i32;
        /* Get table index */
        if is_DC_band != 0 {
            if (*cinfo).Ah != 0i32 {
                current_block_45 = 13472856163611868459;
            } else {
                tbl = (*compptr).dc_tbl_no;
                current_block_45 = 3275366147856559585;
            }
        } else {
            tbl = (*compptr).ac_tbl_no;
            (*entropy).ac_tbl_no = tbl;
            current_block_45 = 3275366147856559585;
        }
        match current_block_45 {
            3275366147856559585 => {
                if gather_statistics != 0 {
                    /* Check for invalid table index */
                    /* (make_c_derived_tbl does this in the other path) */
                    if tbl < 0i32 || tbl >= NUM_HUFF_TBLS {
                        (*(*cinfo).err).msg_code =
                            super::jerror::JERR_NO_HUFF_TABLE as c_int;
                        (*(*cinfo).err).msg_parm.i[0] = tbl;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                        );
                    }
                    /* Allocate and zero the statistics tables */
                    /* Note that jpeg_gen_optimal_table expects 257 entries in each table! */
                    if (*entropy).count_ptrs[tbl as usize].is_null() {
                        (*entropy).count_ptrs[tbl as usize] = Some(
                            (*(*cinfo).mem)
                                .alloc_small
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                            JPOOL_IMAGE,
                            257u64 *
    
                                ::std::mem::size_of::<c_long>() as c_ulong,
                        )
                            as *mut c_long
                    }
                    memset(
                        (*entropy).count_ptrs[tbl as usize] as *mut c_void,
                        0i32,
                        257u64 *
    ::std::mem::size_of::<c_long>() as c_ulong,
                    );
                    if (*(*cinfo).master).trellis_passes != 0 {
                        
                         
                         let mut i:   c_int =  0i32;
                        while i < 16i32 {
                              let mut j:   c_int =  0i32;
                            while j < 12i32 {
                                *(*entropy).count_ptrs[tbl as usize]
                                    .offset((16i32 * i + j) as isize) = 1i64;
                                j += 1
                            }
                            i += 1
                        }
                    }
                } else {
                    /* Compute derived values for Huffman table */
                    /* We may do this more than once for a table, but it's not expensive */
                    super::jchuff::jpeg_make_c_derived_tbl(
                        cinfo,
                        is_DC_band,
                        tbl,
                        &mut *(*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize),
                    );
                }
            }
            _ => {}
        }
        /* DC refinement needs no table */
        ci += 1
    }
    /* Initialize AC stuff */
    (*entropy).EOBRUN = 0u32;
    (*entropy).BE = 0u32;
    /* Initialize bit buffer to empty */
    (*entropy).put_buffer = 0u64;
    (*entropy).put_bits = 0i32;
    /* Initialize restart stuff */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    (*entropy).next_restart_num = 0i32;
}
/* Outputting bytes to the file.
 * NB: these must be called only when actually outputting,
 * that is, entropy->gather_statistics == FALSE.
 */
/* Emit a byte */

unsafe extern "C" fn dump_buffer(mut entropy: phuff_entropy_ptr)
/* Empty the output buffer; we do not support suspension in this module. */
{
    let mut dest: *mut jpeg_destination_mgr = (*(*entropy).cinfo).dest;
    if Some(
        (*dest)
            .empty_output_buffer
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")((*entropy).cinfo)
        == 0
    {
        (*(*(*entropy).cinfo).err).msg_code = super::jerror::JERR_CANT_SUSPEND as c_int;
        Some(
            (*(*(*entropy).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*entropy).cinfo as j_common_ptr
        );
    }
    /* After a successful buffer dump, must reset buffer pointers */
    (*entropy).next_output_byte = (*dest).next_output_byte;
    (*entropy).free_in_buffer = (*dest).free_in_buffer;
}
/* Outputting bits to the file */
/* Only the right 24 bits of put_buffer are used; the valid bits are
 * left-justified in this part.  At most 16 bits can be passed to emit_bits
 * in one call, and we never retain more than 7 bits in put_buffer
 * between calls, so 24 bits are sufficient.
 */

unsafe extern "C" fn emit_bits(
    mut entropy: phuff_entropy_ptr,
    mut code: c_uint,
    mut size: c_int,
)
/* Emit some bits, unless we are in gather mode */
{
    /* This routine is heavily used, so it's worth coding tightly. */
    let mut put_buffer: size_t = code as size_t;
    let mut put_bits: c_int = (*entropy).put_bits;
    /* if size is 0, caller used an invalid Huffman table entry */
    if size == 0i32 {
        (*(*(*entropy).cinfo).err).msg_code =
            super::jerror::JERR_HUFF_MISSING_CODE as c_int; /* do nothing if we're only getting stats */
        Some(
            (*(*(*entropy).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*entropy).cinfo as j_common_ptr
        ); /* mask off any extra bits in code */
    } /* new number of bits in buffer */
    if (*entropy).gather_statistics != 0 {
        return;
    } /* align incoming bits */
    put_buffer &= ((((1u64) << size))) - 1u64; /* and merge with old buffer contents */
    put_bits += size;
    put_buffer <<= 24i32 - put_bits;
    put_buffer |= (*entropy).put_buffer;
    while put_bits >= 8i32 {
        let mut c: c_int = (put_buffer >> 16i32 & 0xffu64) as c_int;
        let fresh0 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh0 = c as JOCTET;
        (*entropy).free_in_buffer =  (*entropy).free_in_buffer - 1;
        if (*entropy).free_in_buffer == 0u64 {
            dump_buffer(entropy);
        }
        if c == 0xffi32 {
            /* need to stuff a zero byte? */
            let fresh1 = (*entropy).next_output_byte; /* update variables */
            (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1); /* fill any partial byte with ones */
            *fresh1 = 0u8; /* and reset bit-buffer to empty */
            (*entropy).free_in_buffer =  (*entropy).free_in_buffer - 1;
            if (*entropy).free_in_buffer == 0u64 {
                dump_buffer(entropy);
            }
        }
        put_buffer <<= 8i32;
        put_bits -= 8i32
    }
    (*entropy).put_buffer = put_buffer;
    (*entropy).put_bits = put_bits;
}

unsafe extern "C" fn flush_bits(mut entropy: phuff_entropy_ptr) {
    emit_bits(entropy, 0x7fu32, 7i32);
    (*entropy).put_buffer = 0u64;
    (*entropy).put_bits = 0i32;
}
/*
 * Emit (or just count) a Huffman symbol.
 */

unsafe extern "C" fn emit_symbol(
    mut entropy: phuff_entropy_ptr,
    mut tbl_no: c_int,
    mut symbol: c_int,
) {
    if (*entropy).gather_statistics != 0 {
        let ref mut fresh2 = *(*entropy).count_ptrs[tbl_no as usize].offset(symbol as isize);
        *fresh2 += 1
    } else {
        let mut tbl: *mut super::jchuff::c_derived_tbl =
            (*entropy).derived_tbls[tbl_no as usize];
        emit_bits(
            entropy,
            (*tbl).ehufco[symbol as usize],
            (*tbl).ehufsi[symbol as usize] as c_int,
        );
    };
}
/*
 * Emit bits from a correction bit buffer.
 */

unsafe extern "C" fn emit_buffered_bits(
    mut entropy: phuff_entropy_ptr,
    mut bufstart: *mut c_char,
    mut nbits: c_uint,
) {
    if (*entropy).gather_statistics != 0 {
        return;
    } /* no real work */
    while nbits > 0u32 {
        emit_bits(entropy, *bufstart as c_uint, 1i32);
        bufstart = bufstart.offset(1);
        nbits -=  1
    }
}
/*
 * Emit any pending EOBRUN symbol.
 */

unsafe extern "C" fn emit_eobrun(mut entropy: phuff_entropy_ptr) {
    
    
    if (*entropy).EOBRUN > 0u32 {
         
         let mut temp:   c_int =  (*entropy).EOBRUN as c_int; let mut nbits:   c_int =
     jpeg_nbits_table[temp as usize] as c_int - 1i32;
        /* safety check: shouldn't happen given limited correction-bit buffer */
        if nbits > 14i32 {
            (*(*(*entropy).cinfo).err).msg_code =
                super::jerror::JERR_HUFF_MISSING_CODE as c_int;
            Some(
                (*(*(*entropy).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*entropy).cinfo as j_common_ptr,
            );
        }
        emit_symbol(entropy, (*entropy).ac_tbl_no, nbits << 4i32);
        if nbits != 0 {
            emit_bits(entropy, (*entropy).EOBRUN, nbits);
        }
        (*entropy).EOBRUN = 0u32;
        /* Emit any buffered correction bits */
        emit_buffered_bits(entropy, (*entropy).bit_buffer, (*entropy).BE);
        (*entropy).BE = 0u32
    };
}
/*
 * Emit a restart marker & resynchronize predictions.
 */

unsafe extern "C" fn emit_restart(mut entropy: phuff_entropy_ptr, mut restart_num: c_int) {
    
    emit_eobrun(entropy);
    if (*entropy).gather_statistics == 0 {
        flush_bits(entropy);
        let fresh3 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh3 = 0xffu8;
        (*entropy).free_in_buffer =  (*entropy).free_in_buffer - 1;
        if (*entropy).free_in_buffer == 0u64 {
            dump_buffer(entropy);
        }
        let fresh4 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh4 = (0xd0i32 + restart_num) as JOCTET;
        (*entropy).free_in_buffer =  (*entropy).free_in_buffer - 1;
        if (*entropy).free_in_buffer == 0u64 {
            dump_buffer(entropy);
        }
    }
    if (*(*entropy).cinfo).Ss == 0i32 {
         let mut ci:   c_int =  0i32;
        while ci < (*(*entropy).cinfo).comps_in_scan {
            (*entropy).last_dc_val[ci as usize] = 0i32;
            ci += 1
        }
    } else {
        /* Re-initialize all AC-related fields to 0 */
        (*entropy).EOBRUN = 0u32;
        (*entropy).BE = 0u32
    };
}
/* Max # of correction bits I can buffer */
/* IRIGHT_SHIFT is like RIGHT_SHIFT, but works on int rather than JLONG.
 * We assume that int right shift is unsigned if JLONG right shift is,
 * which should be safe.
 */
/* Forward declarations */
/*
 * MCU encoding for DC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn encode_mcu_DC_first(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
     let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    
    
    
    
    
    
    let mut Al: c_int = (*cinfo).Al;
    
    
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
     let mut blkn:   c_int =  0i32;
    while blkn < (*cinfo).blocks_in_MCU {
               
        
        
        /* Compute the DC value after the required point transform by Al.
         * This is simply an arithmetic right shift.
         */
        
         let mut block:   JBLOCKROW =  *MCU_data.offset(blkn as isize); let mut ci:   c_int =  (*cinfo).MCU_membership[blkn as usize]; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).cur_comp_info[ci as usize]; let mut temp2:   c_int =  (*block)[0] as c_int >> Al; let mut temp:   c_int =  temp2 - (*entropy).last_dc_val[ci as usize];
        (*entropy).last_dc_val[ci as usize] = temp2;
        /* Encode the DC coefficient difference per section G.1.2.1 */
        /* This is a well-known technique for obtaining the absolute value without
         * a branch.  It is derived from an assembly language technique presented
         * in "How to Optimize for the Pentium Processors", Copyright (c) 1996,
         * 1997 by Agner Fog.
         */
         let mut temp3:   c_int =
     temp
            >> CHAR_BIT as c_ulong *
    ::std::mem::size_of::<c_int>() as c_ulong -
    1u64; /* temp is abs value of input */
        temp ^= temp3;
        temp -= temp3;
        /* For a negative input, want temp2 = bitwise complement of abs(input) */
        temp2 = temp ^ temp3;
         let mut nbits:   c_int =
     jpeg_nbits_table[temp as usize] as c_int;
        /* Check for out-of-range coefficient values.
         * Since we're encoding a difference, the range limit is twice as much.
         */
        if nbits > super::jchuff::MAX_COEF_BITS + 1i32 {
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
        /* Count/emit the Huffman-coded symbol for the number of bits */
        emit_symbol(entropy, (*compptr).dc_tbl_no, nbits);
        /* Emit that number of bits of the value, if positive, */
        /* or the complement of its magnitude, if negative. */
        if nbits != 0 {
            /* emit_bits rejects calls with size 0 */
            emit_bits(entropy, temp2 as c_uint, nbits);
        }
        blkn += 1
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1
    }
    return TRUE;
}
/*
 * Data preparation for encode_mcu_AC_first().
 */
/* We must apply the point transform by Al.  For AC coefficients this \
 * is an integer division with rounding towards 0.  To do this portably \
 * in C, we shift after obtaining the absolute value; so the code is \
 * interwoven with finding the abs value (temp) and output bits (temp2). \
 */
/* temp is abs value of input */
/* apply the point transform */
/* Watch out for case that nonzero coef is zero after point transform */
/* For a negative coef, want temp2 = bitwise complement of abs(coef) */

unsafe extern "C" fn encode_mcu_AC_first_prepare(
    mut block: *const JCOEF,
    mut jpeg_natural_order_start: *const c_int,
    mut Sl: c_int,
    mut Al: c_int,
    mut values: *mut JCOEF,
    mut bits: *mut size_t,
) {
    
    
    
      let mut zerobits:  size_t =  0u64;
    let mut Sl0: c_int = Sl;
     let mut k:   c_int =  0i32;
    while k < Sl0 {
          let mut temp:   c_int =
     *block.offset(*jpeg_natural_order_start.offset(k as isize) as isize) as c_int;
        if !(temp == 0i32) {
              let mut temp2:   c_int =
     temp
                >> CHAR_BIT as c_ulong *
    ::std::mem::size_of::<c_int>() as c_ulong -
    1u64;
            temp ^= temp2;
            temp -= temp2;
            temp >>= Al;
            if !(temp == 0i32) {
                temp2 ^= temp;
                *values.offset(k as isize) = temp as JCOEF;
                *values.offset((k + DCTSIZE2) as isize) =
                    temp2 as JCOEF;
                zerobits |= (1u64) << k
            }
        }
        k += 1
    }
    *bits.offset(0) = zerobits;
}
/*
 * MCU encoding for AC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */
/* if run length > 15, must emit special run-length-16 codes (0xF0) */
/* Find the number of bits needed for the magnitude of the coefficient */
/* there must be at least one 1 bit */
/* Check for out-of-range coefficient values */
/* Count/emit Huffman symbol for run length / number of bits */
/* Emit that number of bits of the value, if positive, */
/* or the complement of its magnitude, if negative. */

unsafe extern "C" fn encode_mcu_AC_first(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
     let mut values_unaligned:  [JCOEF; 143] =  [0; 143];    let mut bits:  [size_t; 1] =  [0; 1];let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    
    
    
    
    let mut Sl: c_int = (*cinfo).Se - (*cinfo).Ss + 1i32;
    let mut Al: c_int = (*cinfo).Al;
    
    
    
    
    
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
    
     let mut values:   *mut JCOEF =
     (values_unaligned.as_mut_ptr() as size_t +
    16u64 - 1u64
        & !(16i32 - 1i32) as c_ulong) as *mut JCOEF; let mut cvalue:   *const JCOEF =  values;
    /* Prepare data */
    (*entropy)
        .AC_first_prepare
        .expect("non-null function pointer")(
        (*(*MCU_data.offset(0)).offset(0)).as_mut_ptr(),
        jpeg_natural_order
            .as_ptr()
            .offset((*cinfo).Ss as isize),
        Sl,
        Al,
        values,
        bits.as_mut_ptr(),
    );
     let mut zerobits:   size_t =  bits[0];
    /* Emit any pending EOBRUN */
    if zerobits != 0 && (*entropy).EOBRUN > 0u32 {
        emit_eobrun(entropy);
    }
    /* Encode the AC coefficients per section G.1.2.2, fig. G.3 */
    while zerobits != 0 {
             let mut r:   c_int =  count_zeroes(&mut zerobits);
        cvalue = cvalue.offset(r as isize);
        
         let mut temp:   c_int =  *cvalue.offset(0) as c_int; let mut temp2:   c_int =
     *cvalue.offset(DCTSIZE2 as isize) as c_int;
        while r > 15i32 {
            emit_symbol(entropy, (*entropy).ac_tbl_no, 0xf0i32);
            r -= 16i32
        }
         let mut nbits:   c_int =
     jpeg_nbits_table[temp as usize] as c_int;
        if nbits > super::jchuff::MAX_COEF_BITS {
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
        emit_symbol(entropy, (*entropy).ac_tbl_no, (r << 4i32) + nbits);
        emit_bits(entropy, temp2 as c_uint, nbits);
        cvalue = cvalue.offset(1);
        zerobits >>= 1i32
    }
    if cvalue < values.offset(Sl as isize) as *const JCOEF {
        /* If there are trailing zeroes, */
        (*entropy).EOBRUN =  (*entropy).EOBRUN + 1;
        if (*entropy).EOBRUN == 0x7fffu32 {
            emit_eobrun(entropy); /* count an EOB */
        }
        /* force it out to avoid overflow */
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1
    }
    return TRUE;
}
/*
 * MCU encoding for DC successive approximation refinement scan.
 * Note: we assume such scans can be multi-component, although the spec
 * is not very clear on the point.
 */

unsafe extern "C" fn encode_mcu_DC_refine(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
     let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    
    
    let mut Al: c_int = (*cinfo).Al;
    
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
     let mut blkn:   c_int =  0i32;
    while blkn < (*cinfo).blocks_in_MCU {
          
         let mut block:   JBLOCKROW =  *MCU_data.offset(blkn as isize); let mut temp:   c_int =  (*block)[0] as c_int;
        emit_bits(entropy, (temp >> Al) as c_uint, 1i32);
        blkn += 1
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1
    }
    return TRUE;
}
/*
 * Data preparation for encode_mcu_AC_refine().
 */
/* It is convenient to make a pre-pass to determine the transformed \
 * coefficients' absolute values and the EOB position. \
 */
/* We must apply the point transform by Al.  For AC coefficients this \
 * is an integer division with rounding towards 0.  To do this portably \
 * in C, we shift after obtaining the absolute value. \
 */
/* temp is abs value of input */
/* apply the point transform */
/* save abs value for main pass */
/* EOB = index of last newly-nonzero coef */

unsafe extern "C" fn encode_mcu_AC_refine_prepare(
    mut block: *const JCOEF,
    mut jpeg_natural_order_start: *const c_int,
    mut Sl: c_int,
    mut Al: c_int,
    mut absvalues: *mut JCOEF,
    mut bits: *mut size_t,
) -> c_int {
    
    
    
    
    
      let mut EOB:  c_int =  0i32; let mut zerobits:  size_t =  0u64; let mut signbits:  size_t =  0u64;
    let mut Sl0: c_int = Sl;
     let mut k:   c_int =  0i32;
    while k < Sl0 {
          
         let mut temp:   c_int =
     *block.offset(*jpeg_natural_order_start.offset(k as isize) as isize) as c_int; let mut temp2:   c_int =
     temp
            >> CHAR_BIT as c_ulong *
    ::std::mem::size_of::<c_int>() as c_ulong -
    1u64;
        temp ^= temp2;
        temp -= temp2;
        temp >>= Al;
        if temp != 0i32 {
            zerobits |= (1u64) << k;
            signbits |= ((temp2 + 1i32) as size_t) << k
        }
        *absvalues.offset(k as isize) = temp as JCOEF;
        if temp == 1i32 {
            EOB = k + 0i32
        }
        k += 1
    }
    *bits.offset(0) = zerobits;
    *bits.offset(1) = signbits;
    return EOB;
}
/*
 * MCU encoding for AC successive approximation refinement scan.
 */
/* Emit any required ZRLs, but not if they can be folded into EOB */
/* emit any pending EOBRUN and the BE correction bits */
/* Emit ZRL */
/* Emit buffered correction bits that must be associated with ZRL */
/* BE bits are gone now */
/* If the coef was previously nonzero, it only needs a correction bit. \
 * NOTE: a straight translation of the spec's figure G.7 would suggest \
 * that we also need to test r > 15.  But if r > 15, we can only get here \
 * if k > EOB, which implies that this coefficient is not 1. \
 */
/* The correction bit is the next bit of the absolute value. */
/* Emit any pending EOBRUN and the BE correction bits */
/* Count/emit Huffman symbol for run length / number of bits */
/* Emit output bit for newly-nonzero coef */
/* ((*block)[jpeg_natural_order_start[k]] < 0) ? 0 : 1 */
/* Emit buffered correction bits that must be associated with this code */
/* BE bits are gone now */
/* reset zero run length */

unsafe extern "C" fn encode_mcu_AC_refine(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
        let mut absvalues_unaligned:  [JCOEF; 79] =  [0; 79];      let mut bits:  [size_t; 2] =  [0; 2];let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    
    
    
    
    let mut Sl: c_int = (*cinfo).Se - (*cinfo).Ss + 1i32;
    let mut Al: c_int = (*cinfo).Al;
    
    
    
    
    
    
    
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
    
    
    
     /* r = run length of zeros */
     /* BR = count of buffered bits added now */
     /* Append bits to buffer */
    
     let mut absvalues:   *mut JCOEF =
     (absvalues_unaligned.as_mut_ptr() as size_t +
    16u64 - 1u64
        & !(16i32 - 1i32) as c_ulong) as *mut JCOEF; let mut cabsvalue:   *const JCOEF =  absvalues; let mut EOBPTR:   *const JCOEF =
     absvalues.offset((*entropy)
        .AC_refine_prepare
        .expect("non-null function pointer")(
        (*(*MCU_data.offset(0)).offset(0)).as_mut_ptr(),
        jpeg_natural_order
            .as_ptr()
            .offset((*cinfo).Ss as isize),
        Sl,
        Al,
        absvalues,
        bits.as_mut_ptr(),
    ) as isize); let mut r:   c_int =  0i32; let mut BR:   c_uint =  0u32; let mut BR_buffer:   *mut c_char =
     (*entropy).bit_buffer.offset((*entropy).BE as isize); let mut zerobits:   size_t =  bits[0]; let mut signbits:   size_t =  bits[1];
    while zerobits != 0 {
         let mut idx: c_int = count_zeroes(&mut zerobits);
        r += idx;
        cabsvalue = cabsvalue.offset(idx as isize);
        signbits >>= idx;
        while r > 15i32 && cabsvalue <= EOBPTR {
            emit_eobrun(entropy);
            emit_symbol(entropy, (*entropy).ac_tbl_no, 0xf0i32);
            r -= 16i32;
            emit_buffered_bits(entropy, BR_buffer, BR);
            BR_buffer = (*entropy).bit_buffer;
            BR = 0u32
        }
        let fresh5 = cabsvalue;
        cabsvalue = cabsvalue.offset(1);
         let mut temp:   c_int =  *fresh5 as c_int;
        if temp > 1i32 {
            let fresh6 = BR;
            BR +=  1;
            *BR_buffer.offset(fresh6 as isize) = (temp & 1i32) as c_char;
            signbits >>= 1i32;
            zerobits >>= 1i32
        } else {
            emit_eobrun(entropy);
            emit_symbol(entropy, (*entropy).ac_tbl_no, (r << 4i32) + 1i32);
            temp = (signbits & 1u64) as c_int;
            emit_bits(entropy, temp as c_uint, 1i32);
            emit_buffered_bits(entropy, BR_buffer, BR);
            BR_buffer = (*entropy).bit_buffer;
            BR = 0u32;
            r = 0i32;
            signbits >>= 1i32;
            zerobits >>= 1i32
        }
    }
    r |=  absvalues
        .offset(Sl as isize)
        .wrapping_offset_from(cabsvalue) as c_int;
    if r > 0i32 || BR > 0u32 {
        /* If there are trailing zeroes, */
        (*entropy).EOBRUN =  (*entropy).EOBRUN + 1; /* count an EOB */
        (*entropy).BE =  (*entropy).BE + BR; /* concat my correction bits to older ones */
        /* We force out the EOB if we risk either:
         * 1. overflow of the EOB counter;
         * 2. overflow of the correction bit buffer during the next MCU.
         */
        if (*entropy).EOBRUN == 0x7fffu32
            || (*entropy).BE > (MAX_CORR_BITS - DCTSIZE2 + 1i32) as c_uint
        {
            emit_eobrun(entropy);
        }
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go =  (*entropy).restarts_to_go - 1
    }
    return TRUE;
}
/*
 * Finish up at the end of a Huffman-compressed progressive scan.
 */

unsafe extern "C" fn finish_pass_phuff(mut cinfo: j_compress_ptr) {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Flush out any buffered data */
    emit_eobrun(entropy);
    flush_bits(entropy);
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
}
/*
 * Finish up a statistics-gathering pass and create the new Huffman tables.
 */

unsafe extern "C" fn finish_pass_gather_phuff(mut cinfo: j_compress_ptr) {
       let mut did:  [boolean; 4] =  [0; 4];let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    
    
    
    
    
    
    /* Flush out buffered data (all we care about is counting the EOB symbol) */
    emit_eobrun(entropy);
     let mut is_DC_band:   boolean =
     ((*cinfo).Ss == 0i32) as c_int;
    /* It's important not to apply jpeg_gen_optimal_table more than once
     * per table, because it clobbers the input frequency counts!
     */
    memset(
        did.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[boolean; 4]>() as c_ulong,
    );
    
     let mut ci:   c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
         let mut tbl:  c_int =  0;  let mut current_block_16:  u64; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).cur_comp_info[ci as usize];
        if is_DC_band != 0 {
            if (*cinfo).Ah != 0i32 {
                current_block_16 = 4906268039856690917;
            } else {
                tbl = (*compptr).dc_tbl_no;
                current_block_16 = 3512920355445576850;
            }
        } else {
            tbl = (*compptr).ac_tbl_no;
            current_block_16 = 3512920355445576850;
        }
        match current_block_16 {
            3512920355445576850 => {
                if did[tbl as usize] == 0 {
                     let mut htblptr:  *mut *mut JHUFF_TBL =
    
        ::std::ptr::null_mut::< *mut JHUFF_TBL>();if is_DC_band != 0 {
                        htblptr = &mut *(*cinfo).dc_huff_tbl_ptrs.as_mut_ptr().offset(tbl as isize)
                            as *mut *mut JHUFF_TBL
                    } else {
                        htblptr = &mut *(*cinfo).ac_huff_tbl_ptrs.as_mut_ptr().offset(tbl as isize)
                            as *mut *mut JHUFF_TBL
                    }
                    if (*htblptr).is_null() {
                        *htblptr = jpeg_alloc_huff_table(
                            cinfo as j_common_ptr,
                        )
                    }
                    super::jchuff::jpeg_gen_optimal_table(
                        cinfo,
                        *htblptr,
                        (*entropy).count_ptrs[tbl as usize],
                    );
                    did[tbl as usize] = TRUE
                }
            }
            _ => {}
        }
        /* DC refinement needs no table */
        ci += 1
    }
}
/*
 * Module initialization routine for progressive Huffman entropy encoding.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_phuff_encoder(mut cinfo: j_compress_ptr) {
    
      
     let mut entropy:   phuff_entropy_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<phuff_entropy_encoder>() as c_ulong,
    ) as phuff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_encoder;
    (*entropy).pub_0.start_pass = Some(
        start_pass_phuff
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: boolean,
            ) -> (),
    );
     let mut i:   c_int =  0i32;
    while i < NUM_HUFF_TBLS {
        (*entropy).derived_tbls[i as usize] =
            NULL as *mut super::jchuff::c_derived_tbl;
        (*entropy).count_ptrs[i as usize] = NULL as *mut c_long;
        i += 1
    }
    (*entropy).bit_buffer = NULL as *mut c_char;
    /* needed only in AC refinement scan */
}
/* C_PROGRESSIVE_SUPPORTED */
