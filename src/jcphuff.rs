use libc;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;

pub use crate::jchuff::c_derived_tbl;
pub use crate::jchuff::jpeg_gen_optimal_table;
pub use crate::jchuff::jpeg_make_c_derived_tbl;
pub use crate::jchuff::MAX_COEF_BITS;
pub use crate::jerror::C2RustUnnamed_4;
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
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpeg_nbits_table_h::jpeg_nbits_table;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
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
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_3;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
use crate::jsimd::jsimd_can_encode_mcu_AC_first_prepare;
use crate::jsimd::jsimd_can_encode_mcu_AC_refine_prepare;
use crate::jsimd::jsimd_encode_mcu_AC_first_prepare;
use crate::jsimd::jsimd_encode_mcu_AC_refine_prepare;
pub use crate::limits_h::CHAR_BIT;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memset;
pub type phuff_entropy_ptr = *mut phuff_entropy_encoder;
/*
 * jcphuff.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1995-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2011, 2015, 2018, D. R. Commander.
 * Copyright (C) 2016, 2018, Matthieu Darbois.
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains Huffman entropy encoding routines for progressive JPEG.
 *
 * We do not support output suspension in this module, since the library
 * currently does not allow multiple-scan files to be written with output
 * suspension.
 */
/*
 * NOTE: If USE_CLZ_INTRINSIC is defined, then clz/bsr instructions will be
 * used for bit counting rather than the lookup table.  This will reduce the
 * memory footprint by 64k, which is important for some mobile applications
 * that create many isolated instances of libjpeg-turbo (web browsers, for
 * instance.)  This may improve performance on some mobile platforms as well.
 * This feature is enabled by default only on ARM processors, because some x86
 * chips have a slow implementation of bsr, and the use of clz/bsr cannot be
 * shown to have a significant performance impact even on the x86 chips that
 * have a fast implementation of it.  When building for ARMv6, you can
 * explicitly disable the use of clz/bsr by adding -mthumb to the compiler
 * flags (this defines __thumb__).
 */
/* NOTE: Both GCC and Clang define __GNUC__ */
/* Expanded entropy encoder object for progressive Huffman encoding. */

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
    pub derived_tbls: [*mut c_derived_tbl; 4],
    pub count_ptrs: [*mut c_long; 4],
}
/* MAX_CORR_BITS is the number of bits the AC refinement correction-bit
 * buffer can hold.  Larger sizes may slightly improve compression, but
 * 1000 is already well into the realm of overkill.
 * The minimum safe size is 64 bits.
 */
/* Max # of correction bits I can buffer */
pub const MAX_CORR_BITS: c_int = 1000i32;
/* Count bit loop zeroes */
#[inline(always)]
unsafe extern "C" fn count_zeroes(mut x: *mut size_t) -> c_int {
    let mut result: c_int = 0;
    result = (*x).trailing_zeros() as i32;
    *x >>= result;
    return result;
}
/*
 * Initialize for a Huffman-compressed scan using progressive JPEG.
 */
unsafe extern "C" fn start_pass_phuff(mut cinfo: j_compress_ptr, mut gather_statistics: boolean) {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut is_DC_band: boolean = 0;
    let mut ci: c_int = 0;
    let mut tbl: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    (*entropy).cinfo = cinfo;
    (*entropy).gather_statistics = gather_statistics;
    is_DC_band = ((*cinfo).Ss == 0i32) as c_int;
    if (*cinfo).Ah == 0i32 {
        if 0 != is_DC_band {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_DC_first
                    as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
            )
        } else {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_AC_first
                    as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
            )
        }
        if 0 != jsimd_can_encode_mcu_AC_first_prepare() {
            (*entropy).AC_first_prepare = Some(
                jsimd_encode_mcu_AC_first_prepare
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
    } else if 0 != is_DC_band {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_DC_refine
                as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
        )
    } else {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_AC_refine
                as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
        );
        if 0 != jsimd_can_encode_mcu_AC_refine_prepare() {
            (*entropy).AC_refine_prepare = Some(
                jsimd_encode_mcu_AC_refine_prepare
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
        if (*entropy).bit_buffer.is_null() {
            (*entropy).bit_buffer = (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                (MAX_CORR_BITS as c_ulong).wrapping_mul(::std::mem::size_of::<c_char>() as c_ulong),
            ) as *mut c_char
        }
    }
    if 0 != gather_statistics {
        (*entropy).pub_0.finish_pass =
            Some(finish_pass_gather_phuff as unsafe extern "C" fn(_: j_compress_ptr) -> ())
    } else {
        (*entropy).pub_0.finish_pass =
            Some(finish_pass_phuff as unsafe extern "C" fn(_: j_compress_ptr) -> ())
    }
    let mut current_block_45: u64;
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        (*entropy).last_dc_val[ci as usize] = 0i32;
        /* Get table index */
        if 0 != is_DC_band {
            /* DC refinement needs no table */
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
                if 0 != gather_statistics {
                    if tbl < 0i32 || tbl >= NUM_HUFF_TBLS {
                        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as c_int;
                        (*(*cinfo).err).msg_parm.i[0usize] = tbl;
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                    }
                    if (*entropy).count_ptrs[tbl as usize].is_null() {
                        (*entropy).count_ptrs[tbl as usize] = (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                            JPOOL_IMAGE,
                            (257i32 as c_ulong)
                                .wrapping_mul(::std::mem::size_of::<c_long>() as c_ulong),
                        )
                            as *mut c_long
                    }
                    memset(
                        (*entropy).count_ptrs[tbl as usize] as *mut c_void,
                        0i32,
                        (257i32 as c_ulong)
                            .wrapping_mul(::std::mem::size_of::<c_long>() as c_ulong),
                    );
                    if 0 != (*(*cinfo).master).trellis_passes {
                        let mut i: c_int = 0;
                        let mut j: c_int = 0;
                        i = 0i32;
                        while i < 16i32 {
                            j = 0i32;
                            while j < 12i32 {
                                *(*entropy).count_ptrs[tbl as usize]
                                    .offset((16i32 * i + j) as isize) = 1i32 as c_long;
                                j += 1
                            }
                            i += 1
                        }
                    }
                } else {
                    jpeg_make_c_derived_tbl(
                        cinfo,
                        is_DC_band,
                        tbl,
                        &mut *(*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize),
                    );
                }
            }
            _ => {}
        }
        ci += 1
    }
    (*entropy).EOBRUN = 0i32 as c_uint;
    (*entropy).BE = 0i32 as c_uint;
    (*entropy).put_buffer = 0i32 as size_t;
    (*entropy).put_bits = 0i32;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    (*entropy).next_restart_num = 0i32;
}
/* Outputting bytes to the file.
 * NB: these must be called only when actually outputting,
 * that is, entropy->gather_statistics == FALSE.
 */
/* Emit a byte */
unsafe extern "C" fn dump_buffer(mut entropy: phuff_entropy_ptr) {
    let mut dest: *mut jpeg_destination_mgr = (*(*entropy).cinfo).dest;
    if 0 == (*dest)
        .empty_output_buffer
        .expect("non-null function pointer")((*entropy).cinfo)
    {
        (*(*(*entropy).cinfo).err).msg_code = JERR_CANT_SUSPEND as c_int;
        (*(*(*entropy).cinfo).err)
            .error_exit
            .expect("non-null function pointer")((*entropy).cinfo as j_common_ptr);
    }
    (*entropy).next_output_byte = (*dest).next_output_byte;
    (*entropy).free_in_buffer = (*dest).free_in_buffer;
}
/* Outputting bits to the file */
/* Only the right 24 bits of put_buffer are used; the valid bits are
 * left-justified in this part.  At most 16 bits can be passed to emit_bits
 * in one call, and we never retain more than 7 bits in put_buffer
 * between calls, so 24 bits are sufficient.
 */
unsafe extern "C" fn emit_bits(mut entropy: phuff_entropy_ptr, mut code: c_uint, mut size: c_int) {
    /* This routine is heavily used, so it's worth coding tightly. */
    let mut put_buffer: size_t = code as size_t;
    let mut put_bits: c_int = (*entropy).put_bits;
    if size == 0i32 {
        (*(*(*entropy).cinfo).err).msg_code = JERR_HUFF_MISSING_CODE as c_int;
        (*(*(*entropy).cinfo).err)
            .error_exit
            .expect("non-null function pointer")((*entropy).cinfo as j_common_ptr);
    }
    if 0 != (*entropy).gather_statistics {
        return;
    }
    put_buffer &= ((1i32 as size_t) << size).wrapping_sub(1i32 as c_ulong);
    put_bits += size;
    put_buffer <<= 24i32 - put_bits;
    put_buffer |= (*entropy).put_buffer;
    while put_bits >= 8i32 {
        let mut c: c_int = (put_buffer >> 16i32 & 0xffi32 as c_ulong) as c_int;
        let fresh0 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh0 = c as JOCTET;
        (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
        if (*entropy).free_in_buffer == 0i32 as c_ulong {
            dump_buffer(entropy);
        }
        if c == 0xffi32 {
            let fresh1 = (*entropy).next_output_byte;
            (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
            *fresh1 = 0i32 as JOCTET;
            (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
            if (*entropy).free_in_buffer == 0i32 as c_ulong {
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
    emit_bits(entropy, 0x7fi32 as c_uint, 7i32);
    (*entropy).put_buffer = 0i32 as size_t;
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
    if 0 != (*entropy).gather_statistics {
        let ref mut fresh2 = *(*entropy).count_ptrs[tbl_no as usize].offset(symbol as isize);
        *fresh2 += 1
    } else {
        let mut tbl: *mut c_derived_tbl = (*entropy).derived_tbls[tbl_no as usize];
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
    if 0 != (*entropy).gather_statistics {
        return;
    }
    while nbits > 0i32 as c_uint {
        emit_bits(entropy, *bufstart as c_uint, 1i32);
        bufstart = bufstart.offset(1isize);
        nbits = nbits.wrapping_sub(1)
    }
}
/*
 * Emit any pending EOBRUN symbol.
 */
unsafe extern "C" fn emit_eobrun(mut entropy: phuff_entropy_ptr) {
    let mut temp: c_int = 0;
    let mut nbits: c_int = 0;
    if (*entropy).EOBRUN > 0i32 as c_uint {
        temp = (*entropy).EOBRUN as c_int;
        nbits = jpeg_nbits_table[temp as usize] as c_int - 1i32;
        if nbits > 14i32 {
            (*(*(*entropy).cinfo).err).msg_code = JERR_HUFF_MISSING_CODE as c_int;
            (*(*(*entropy).cinfo).err)
                .error_exit
                .expect("non-null function pointer")((*entropy).cinfo as j_common_ptr);
        }
        emit_symbol(entropy, (*entropy).ac_tbl_no, nbits << 4i32);
        if 0 != nbits {
            emit_bits(entropy, (*entropy).EOBRUN, nbits);
        }
        (*entropy).EOBRUN = 0i32 as c_uint;
        emit_buffered_bits(entropy, (*entropy).bit_buffer, (*entropy).BE);
        (*entropy).BE = 0i32 as c_uint
    };
}
/*
 * Emit a restart marker & resynchronize predictions.
 */
unsafe extern "C" fn emit_restart(mut entropy: phuff_entropy_ptr, mut restart_num: c_int) {
    let mut ci: c_int = 0;
    emit_eobrun(entropy);
    if 0 == (*entropy).gather_statistics {
        flush_bits(entropy);
        let fresh3 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh3 = 0xffi32 as JOCTET;
        (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
        if (*entropy).free_in_buffer == 0i32 as c_ulong {
            dump_buffer(entropy);
        }
        let fresh4 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh4 = (0xd0i32 + restart_num) as JOCTET;
        (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
        if (*entropy).free_in_buffer == 0i32 as c_ulong {
            dump_buffer(entropy);
        }
    }
    if (*(*entropy).cinfo).Ss == 0i32 {
        ci = 0i32;
        while ci < (*(*entropy).cinfo).comps_in_scan {
            (*entropy).last_dc_val[ci as usize] = 0i32;
            ci += 1
        }
    } else {
        (*entropy).EOBRUN = 0i32 as c_uint;
        (*entropy).BE = 0i32 as c_uint
    };
}
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
    let mut temp: c_int = 0;
    let mut temp2: c_int = 0;
    let mut temp3: c_int = 0;
    let mut nbits: c_int = 0;
    let mut blkn: c_int = 0;
    let mut ci: c_int = 0;
    let mut Al: c_int = (*cinfo).Al;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        temp2 = (*block)[0usize] as c_int >> Al;
        temp = temp2 - (*entropy).last_dc_val[ci as usize];
        (*entropy).last_dc_val[ci as usize] = temp2;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 = temp ^ temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        if nbits > MAX_COEF_BITS + 1i32 {
            (*(*cinfo).err).msg_code = JERR_BAD_DCT_COEF as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        emit_symbol(entropy, (*compptr).dc_tbl_no, nbits);
        if 0 != nbits {
            emit_bits(entropy, temp2 as c_uint, nbits);
        }
        blkn += 1
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
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
    let mut k: c_int = 0;
    let mut temp: c_int = 0;
    let mut temp2: c_int = 0;
    let mut zerobits: size_t = 0u32 as size_t;
    let mut Sl0: c_int = Sl;
    k = 0i32;
    while k < Sl0 {
        temp = *block.offset(*jpeg_natural_order_start.offset(k as isize) as isize) as c_int;
        if !(temp == 0i32) {
            temp2 = temp
                >> (CHAR_BIT as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                    .wrapping_sub(1i32 as c_ulong);
            temp ^= temp2;
            temp -= temp2;
            temp >>= Al;
            if !(temp == 0i32) {
                temp2 ^= temp;
                *values.offset(k as isize) = temp as JCOEF;
                *values.offset((k + DCTSIZE2) as isize) = temp2 as JCOEF;
                zerobits |= (1u32 as size_t) << k
            }
        }
        k += 1
    }
    *bits.offset(0isize) = zerobits;
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
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut temp: c_int = 0;
    let mut temp2: c_int = 0;
    let mut nbits: c_int = 0;
    let mut r: c_int = 0;
    let mut Sl: c_int = (*cinfo).Se - (*cinfo).Ss + 1i32;
    let mut Al: c_int = (*cinfo).Al;
    let mut values_unaligned: [JCOEF; 143] = [0; 143];
    let mut values: *mut JCOEF = 0 as *mut JCOEF;
    let mut cvalue: *const JCOEF = 0 as *const JCOEF;
    let mut zerobits: size_t = 0;
    let mut bits: [size_t; 1] = [0; 1];
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
    values = ((values_unaligned.as_mut_ptr() as size_t)
        .wrapping_add(16i32 as c_ulong)
        .wrapping_sub(1i32 as c_ulong)
        & !(16i32 - 1i32) as c_ulong) as *mut JCOEF;
    cvalue = values;
    (*entropy)
        .AC_first_prepare
        .expect("non-null function pointer")(
        (*(*MCU_data.offset(0isize)).offset(0isize)).as_mut_ptr(),
        jpeg_natural_order.as_ptr().offset((*cinfo).Ss as isize),
        Sl,
        Al,
        values,
        bits.as_mut_ptr(),
    );
    zerobits = bits[0usize];
    if 0 != zerobits && (*entropy).EOBRUN > 0i32 as c_uint {
        emit_eobrun(entropy);
    }
    while 0 != zerobits {
        r = count_zeroes(&mut zerobits);
        cvalue = cvalue.offset(r as isize);
        temp = *cvalue.offset(0isize) as c_int;
        temp2 = *cvalue.offset(DCTSIZE2 as isize) as c_int;
        while r > 15i32 {
            emit_symbol(entropy, (*entropy).ac_tbl_no, 0xf0i32);
            r -= 16i32
        }
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        if nbits > MAX_COEF_BITS {
            (*(*cinfo).err).msg_code = JERR_BAD_DCT_COEF as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        emit_symbol(entropy, (*entropy).ac_tbl_no, (r << 4i32) + nbits);
        emit_bits(entropy, temp2 as c_uint, nbits);
        cvalue = cvalue.offset(1isize);
        zerobits >>= 1i32
    }
    if cvalue < values.offset(Sl as isize) as *const JCOEF {
        (*entropy).EOBRUN = (*entropy).EOBRUN.wrapping_add(1);
        if (*entropy).EOBRUN == 0x7fffi32 as c_uint {
            emit_eobrun(entropy);
        }
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
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
    let mut temp: c_int = 0;
    let mut blkn: c_int = 0;
    let mut Al: c_int = (*cinfo).Al;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        temp = (*block)[0usize] as c_int;
        emit_bits(entropy, (temp >> Al) as c_uint, 1i32);
        blkn += 1
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
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
    let mut k: c_int = 0;
    let mut temp: c_int = 0;
    let mut temp2: c_int = 0;
    let mut EOB: c_int = 0i32;
    let mut zerobits: size_t = 0u32 as size_t;
    let mut signbits: size_t = 0u32 as size_t;
    let mut Sl0: c_int = Sl;
    k = 0i32;
    while k < Sl0 {
        temp = *block.offset(*jpeg_natural_order_start.offset(k as isize) as isize) as c_int;
        temp2 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp2;
        temp -= temp2;
        temp >>= Al;
        if temp != 0i32 {
            zerobits |= (1u32 as size_t) << k;
            signbits |= ((temp2 + 1i32) as size_t) << k
        }
        *absvalues.offset(k as isize) = temp as JCOEF;
        if temp == 1i32 {
            EOB = k + 0i32
        }
        k += 1
    }
    *bits.offset(0isize) = zerobits;
    *bits.offset(1isize) = signbits;
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
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut temp: c_int = 0;
    let mut r: c_int = 0;
    let mut BR_buffer: *mut c_char = 0 as *mut c_char;
    let mut BR: c_uint = 0;
    let mut Sl: c_int = (*cinfo).Se - (*cinfo).Ss + 1i32;
    let mut Al: c_int = (*cinfo).Al;
    let mut absvalues_unaligned: [JCOEF; 79] = [0; 79];
    let mut absvalues: *mut JCOEF = 0 as *mut JCOEF;
    let mut cabsvalue: *const JCOEF = 0 as *const JCOEF;
    let mut EOBPTR: *const JCOEF = 0 as *const JCOEF;
    let mut zerobits: size_t = 0;
    let mut signbits: size_t = 0;
    let mut bits: [size_t; 2] = [0; 2];
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            emit_restart(entropy, (*entropy).next_restart_num);
        }
    }
    absvalues = ((absvalues_unaligned.as_mut_ptr() as size_t)
        .wrapping_add(16i32 as c_ulong)
        .wrapping_sub(1i32 as c_ulong)
        & !(16i32 - 1i32) as c_ulong) as *mut JCOEF;
    cabsvalue = absvalues;
    EOBPTR = absvalues.offset((*entropy)
        .AC_refine_prepare
        .expect("non-null function pointer")(
        (*(*MCU_data.offset(0isize)).offset(0isize)).as_mut_ptr(),
        jpeg_natural_order.as_ptr().offset((*cinfo).Ss as isize),
        Sl,
        Al,
        absvalues,
        bits.as_mut_ptr(),
    ) as isize);
    r = 0i32;
    BR = 0i32 as c_uint;
    BR_buffer = (*entropy).bit_buffer.offset((*entropy).BE as isize);
    zerobits = bits[0usize];
    signbits = bits[1usize];
    while 0 != zerobits {
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
            BR = 0i32 as c_uint
        }
        let fresh5 = cabsvalue;
        cabsvalue = cabsvalue.offset(1);
        temp = *fresh5 as c_int;
        if temp > 1i32 {
            let fresh6 = BR;
            BR = BR.wrapping_add(1);
            *BR_buffer.offset(fresh6 as isize) = (temp & 1i32) as c_char;
            signbits >>= 1i32;
            zerobits >>= 1i32
        } else {
            emit_eobrun(entropy);
            emit_symbol(entropy, (*entropy).ac_tbl_no, (r << 4i32) + 1i32);
            temp = (signbits & 1i32 as c_ulong) as c_int;
            emit_bits(entropy, temp as c_uint, 1i32);
            emit_buffered_bits(entropy, BR_buffer, BR);
            BR_buffer = (*entropy).bit_buffer;
            BR = 0i32 as c_uint;
            r = 0i32;
            signbits >>= 1i32;
            zerobits >>= 1i32
        }
    }
    r |= absvalues
        .offset(Sl as isize)
        .wrapping_offset_from(cabsvalue) as c_long as c_int;
    if r > 0i32 || BR > 0i32 as c_uint {
        (*entropy).EOBRUN = (*entropy).EOBRUN.wrapping_add(1);
        (*entropy).BE = (*entropy).BE.wrapping_add(BR);
        if (*entropy).EOBRUN == 0x7fffi32 as c_uint
            || (*entropy).BE > (MAX_CORR_BITS - DCTSIZE2 + 1i32) as c_uint
        {
            emit_eobrun(entropy);
        }
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
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
    emit_eobrun(entropy);
    flush_bits(entropy);
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
}
/*
 * Finish up a statistics-gathering pass and create the new Huffman tables.
 */
unsafe extern "C" fn finish_pass_gather_phuff(mut cinfo: j_compress_ptr) {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut is_DC_band: boolean = 0;
    let mut ci: c_int = 0;
    let mut tbl: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut htblptr: *mut *mut JHUFF_TBL = 0 as *mut *mut JHUFF_TBL;
    let mut did: [boolean; 4] = [0; 4];
    emit_eobrun(entropy);
    is_DC_band = ((*cinfo).Ss == 0i32) as c_int;
    memset(
        did.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[boolean; 4]>() as c_ulong,
    );
    let mut current_block_16: u64;
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if 0 != is_DC_band {
            /* DC refinement needs no table */
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
                if 0 == did[tbl as usize] {
                    if 0 != is_DC_band {
                        htblptr = &mut *(*cinfo).dc_huff_tbl_ptrs.as_mut_ptr().offset(tbl as isize)
                            as *mut *mut JHUFF_TBL
                    } else {
                        htblptr = &mut *(*cinfo).ac_huff_tbl_ptrs.as_mut_ptr().offset(tbl as isize)
                            as *mut *mut JHUFF_TBL
                    }
                    if (*htblptr).is_null() {
                        *htblptr = jpeg_alloc_huff_table(cinfo as j_common_ptr)
                    }
                    jpeg_gen_optimal_table(cinfo, *htblptr, (*entropy).count_ptrs[tbl as usize]);
                    did[tbl as usize] = TRUE
                }
            }
            _ => {}
        }
        ci += 1
    }
}
/*
 * Module initialization routine for progressive Huffman entropy encoding.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_phuff_encoder(mut cinfo: j_compress_ptr) {
    let mut entropy: phuff_entropy_ptr = 0 as *mut phuff_entropy_encoder;
    let mut i: c_int = 0;
    entropy = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<phuff_entropy_encoder>() as c_ulong,
    ) as phuff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_encoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass_phuff as unsafe extern "C" fn(_: j_compress_ptr, _: boolean) -> ());
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        (*entropy).derived_tbls[i as usize] = NULL as *mut c_derived_tbl;
        (*entropy).count_ptrs[i as usize] = NULL as *mut c_long;
        i += 1
    }
    (*entropy).bit_buffer = NULL as *mut c_char;
}
pub const __CHAR_BIT__: c_int = 8i32;
