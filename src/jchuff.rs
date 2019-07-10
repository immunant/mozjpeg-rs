use libc::c_char;
use libc::c_double;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
extern "C" {
    #[no_mangle]
    pub fn quantize_trellis(
        cinfo: j_compress_ptr,
        dctbl: *mut c_derived_tbl,
        actbl: *mut c_derived_tbl,
        coef_blocks: JBLOCKROW,
        src: JBLOCKROW,
        num_blocks: JDIMENSION,
        qtbl: *mut JQUANT_TBL,
        norm_src: *mut c_double,
        norm_coef: *mut c_double,
        last_dc_val: *mut JCOEF,
        coef_blocks_above: JBLOCKROW,
        src_above: JBLOCKROW,
    );
}
pub use crate::internal::__CHAR_BIT__;
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
pub use crate::jpegint_h::JLONG;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
use crate::jsimd::jsimd_can_huff_encode_one_block;
use crate::jsimd::jsimd_huff_encode_one_block;
pub use crate::limits_h::CHAR_BIT;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use libc;
// =============== BEGIN jchuff_h ================

/*
 * jchuff.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains declarations for Huffman entropy encoding routines
 * that are shared between the sequential encoder (jchuff.c) and the
 * progressive encoder (jcphuff.c).  No other modules need to see these.
 */

/* The legal range of a DCT coefficient is
 *  -1024 .. +1023  for 8-bit data;
 * -16384 .. +16383 for 12-bit data.
 * Hence the magnitude should always fit in 10 or 14 bits respectively.
 */

/* Derived data constructed for each Huffman table */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c_derived_tbl {
    pub ehufco: [c_uint; 256],
    pub ehufsi: [c_char; 256],
}
/*
 * jchuff.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains declarations for Huffman entropy encoding routines
 * that are shared between the sequential encoder (jchuff.c) and the
 * progressive encoder (jcphuff.c).  No other modules need to see these.
 */

/* The legal range of a DCT coefficient is
 *  -1024 .. +1023  for 8-bit data;
 * -16384 .. +16383 for 12-bit data.
 * Hence the magnitude should always fit in 10 or 14 bits respectively.
 */
pub const MAX_COEF_BITS: c_int = 10i32;
// ================ END jchuff_h ================

/*
 * jchuff.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2011, 2014-2016, 2018, D. R. Commander.
 * Copyright (C) 2015, Matthieu Darbois.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains Huffman entropy encoding routines.
 *
 * Much of the complexity here has to do with supporting output suspension.
 * If the data destination module demands suspension, we want to be able to
 * back up to the start of the current MCU.  To do this, we copy state
 * variables into local working storage, and update them back to the
 * permanent JPEG objects only upon successful completion of an MCU.
 *
 * NOTE: All referenced figures are from
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
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

/* Expanded entropy encoder object for Huffman encoding.
 *
 * The savable_state subrecord contains fields that change within an MCU,
 * but must not be updated permanently until we complete the MCU.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct savable_state {
    pub put_buffer: size_t,
    pub put_bits: c_int,
    pub last_dc_val: [c_int; 4],
}
/* This macro is to work around compilers with missing or broken
 * structure assignment.  You'll need to fix this code if you have
 * such a compiler and you change MAX_COMPS_IN_SCAN.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct huff_entropy_encoder {
    pub pub_0: jpeg_entropy_encoder,
    pub saved: savable_state,
    pub restarts_to_go: c_uint,
    pub next_restart_num: c_int,
    pub dc_derived_tbls: [*mut c_derived_tbl; 4],
    pub ac_derived_tbls: [*mut c_derived_tbl; 4],
    pub dc_count_ptrs: [*mut c_long; 4],
    pub ac_count_ptrs: [*mut c_long; 4],
    pub simd: c_int,
}
pub type huff_entropy_ptr = *mut huff_entropy_encoder;
/* Working state while writing an MCU.
 * This struct contains all the fields that are needed by subroutines.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct working_state {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub cur: savable_state,
    pub cinfo: j_compress_ptr,
}
/*
 * Initialize for a Huffman-compressed scan.
 * If gather_statistics is TRUE, we do not output anything during the scan,
 * just count the Huffman symbols used and generate Huffman code tables.
 */
unsafe extern "C" fn start_pass_huff(mut cinfo: j_compress_ptr, mut gather_statistics: boolean) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: c_int = 0;
    let mut dctbl: c_int = 0;
    let mut actbl: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if 0 != gather_statistics {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_gather
                as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
        );
        (*entropy).pub_0.finish_pass =
            Some(finish_pass_gather as unsafe extern "C" fn(_: j_compress_ptr) -> ())
    } else {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_huff
                as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
        );
        (*entropy).pub_0.finish_pass =
            Some(finish_pass_huff as unsafe extern "C" fn(_: j_compress_ptr) -> ())
    }
    (*entropy).simd = jsimd_can_huff_encode_one_block();
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        dctbl = (*compptr).dc_tbl_no;
        actbl = (*compptr).ac_tbl_no;
        if 0 != gather_statistics {
            if dctbl < 0i32 || dctbl >= NUM_HUFF_TBLS {
                (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = dctbl;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if actbl < 0i32 || actbl >= NUM_HUFF_TBLS {
                (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = actbl;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*entropy).dc_count_ptrs[dctbl as usize].is_null() {
                (*entropy).dc_count_ptrs[dctbl as usize] = (*(*cinfo).mem)
                    .alloc_small
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    JPOOL_IMAGE,
                    (257i32 as c_ulong).wrapping_mul(::std::mem::size_of::<c_long>() as c_ulong),
                ) as *mut c_long
            }
            memset(
                (*entropy).dc_count_ptrs[dctbl as usize] as *mut c_void,
                0i32,
                (257i32 as c_ulong).wrapping_mul(::std::mem::size_of::<c_long>() as c_ulong),
            );
            if (*entropy).ac_count_ptrs[actbl as usize].is_null() {
                (*entropy).ac_count_ptrs[actbl as usize] = (*(*cinfo).mem)
                    .alloc_small
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    JPOOL_IMAGE,
                    (257i32 as c_ulong).wrapping_mul(::std::mem::size_of::<c_long>() as c_ulong),
                ) as *mut c_long
            }
            memset(
                (*entropy).ac_count_ptrs[actbl as usize] as *mut c_void,
                0i32,
                (257i32 as c_ulong).wrapping_mul(::std::mem::size_of::<c_long>() as c_ulong),
            );
        } else {
            jpeg_make_c_derived_tbl(
                cinfo,
                TRUE,
                dctbl,
                &mut *(*entropy)
                    .dc_derived_tbls
                    .as_mut_ptr()
                    .offset(dctbl as isize),
            );
            jpeg_make_c_derived_tbl(
                cinfo,
                FALSE,
                actbl,
                &mut *(*entropy)
                    .ac_derived_tbls
                    .as_mut_ptr()
                    .offset(actbl as isize),
            );
        }
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    (*entropy).saved.put_buffer = 0i32 as size_t;
    (*entropy).saved.put_bits = 0i32;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    (*entropy).next_restart_num = 0i32;
}
/* Expand a Huffman table definition into the derived format */

/*
 * Compute the derived values for a Huffman table.
 * This routine also performs some validation checks on the table.
 *
 * Note this is also used by jcphuff.c.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_make_c_derived_tbl(
    mut cinfo: j_compress_ptr,
    mut isDC: boolean,
    mut tblno: c_int,
    mut pdtbl: *mut *mut c_derived_tbl,
) {
    let mut htbl: *mut JHUFF_TBL = 0 as *mut JHUFF_TBL;
    let mut dtbl: *mut c_derived_tbl = 0 as *mut c_derived_tbl;
    let mut p: c_int = 0;
    let mut i: c_int = 0;
    let mut l: c_int = 0;
    let mut lastp: c_int = 0;
    let mut si: c_int = 0;
    let mut maxsymbol: c_int = 0;
    let mut huffsize: [c_char; 257] = [0; 257];
    let mut huffcode: [c_uint; 257] = [0; 257];
    let mut code: c_uint = 0;
    if tblno < 0i32 || tblno >= NUM_HUFF_TBLS {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = tblno;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    htbl = if 0 != isDC {
        (*cinfo).dc_huff_tbl_ptrs[tblno as usize]
    } else {
        (*cinfo).ac_huff_tbl_ptrs[tblno as usize]
    };
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = tblno;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*pdtbl).is_null() {
        *pdtbl = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ::std::mem::size_of::<c_derived_tbl>() as c_ulong,
        ) as *mut c_derived_tbl
    }
    dtbl = *pdtbl;
    p = 0i32;
    l = 1i32;
    while l <= 16i32 {
        i = (*htbl).bits[l as usize] as c_int;
        if i < 0i32 || p + i > 256i32 {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(0 != fresh0) {
                break;
            }
            let fresh1 = p;
            p = p + 1;
            huffsize[fresh1 as usize] = l as c_char
        }
        l += 1
    }
    huffsize[p as usize] = 0i32 as c_char;
    lastp = p;
    code = 0i32 as c_uint;
    si = huffsize[0usize] as c_int;
    p = 0i32;
    while 0 != huffsize[p as usize] {
        while huffsize[p as usize] as c_int == si {
            let fresh2 = p;
            p = p + 1;
            huffcode[fresh2 as usize] = code;
            code = code.wrapping_add(1)
        }
        if code as JLONG >= (1i32 as JLONG) << si {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        code <<= 1i32;
        si += 1
    }
    memset(
        (*dtbl).ehufsi.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 256]>() as c_ulong,
    );
    maxsymbol = if 0 != isDC { 15i32 } else { 255i32 };
    p = 0i32;
    while p < lastp {
        i = (*htbl).huffval[p as usize] as c_int;
        if i < 0i32 || i > maxsymbol || 0 != (*dtbl).ehufsi[i as usize] as c_int {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*dtbl).ehufco[i as usize] = huffcode[p as usize];
        (*dtbl).ehufsi[i as usize] = huffsize[p as usize];
        p += 1
    }
}
/* Outputting bytes to the file */

/* Emit a byte, taking 'action' if must suspend. */
unsafe extern "C" fn dump_buffer(mut state: *mut working_state) -> boolean {
    let mut dest: *mut jpeg_destination_mgr = (*(*state).cinfo).dest;
    if 0 == (*dest)
        .empty_output_buffer
        .expect("non-null function pointer")((*state).cinfo)
    {
        return FALSE;
    }
    (*state).next_output_byte = (*dest).next_output_byte;
    (*state).free_in_buffer = (*dest).free_in_buffer;
    return TRUE;
}
/* Outputting bits to the file */

/* These macros perform the same task as the emit_bits() function in the
 * original libjpeg code.  In addition to reducing overhead by explicitly
 * inlining the code, additional performance is achieved by taking into
 * account the size of the bit buffer and waiting until it is almost full
 * before emptying it.  This mostly benefits 64-bit platforms, since 6
 * bytes can be stored in a 64-bit bit buffer before it has to be emptied.
 */

/* need to stuff a zero byte? */

/* Although it is exceedingly rare, it is possible for a Huffman-encoded
 * coefficient block to be larger than the 128-byte unencoded block.  For each
 * of the 64 coefficients, PUT_BITS is invoked twice, and each invocation can
 * theoretically store 16 bits (for a maximum of 2048 bits or 256 bytes per
 * encoded block.)  If, for instance, one artificially sets the AC
 * coefficients to alternating values of 32767 and -32768 (using the JPEG
 * scanning order-- 1, 8, 16, etc.), then this will produce an encoded block
 * larger than 200 bytes.
 */
pub const BUFSIZE: c_int = DCTSIZE2 * 4i32;
unsafe extern "C" fn flush_bits(mut state: *mut working_state) -> boolean {
    let mut _buffer: [JOCTET; 256] = [0; 256];
    let mut buffer: *mut JOCTET = 0 as *mut JOCTET;
    let mut put_buffer: size_t = 0;
    let mut put_bits: c_int = 0;
    let mut bytes: size_t = 0;
    let mut bytestocopy: size_t = 0;
    let mut localbuf: c_int = 0i32;
    put_buffer = (*state).cur.put_buffer;
    put_bits = (*state).cur.put_bits;
    if (*state).free_in_buffer < BUFSIZE as c_ulong {
        localbuf = 1i32;
        buffer = _buffer.as_mut_ptr()
    } else {
        buffer = (*state).next_output_byte
    }
    put_bits += 7i32;
    put_buffer = put_buffer << 7i32 | 0x7fi32 as c_ulong;
    while put_bits >= 8i32 {
        let mut c: JOCTET = 0;
        put_bits -= 8i32;
        c = (put_buffer >> put_bits) as JOCTET;
        let fresh3 = buffer;
        buffer = buffer.offset(1);
        *fresh3 = c;
        if c as c_int == 0xffi32 {
            let fresh4 = buffer;
            buffer = buffer.offset(1);
            *fresh4 = 0i32 as JOCTET
        }
    }
    (*state).cur.put_buffer = 0i32 as size_t;
    (*state).cur.put_bits = 0i32;
    if 0 != localbuf {
        bytes = buffer.wrapping_offset_from(_buffer.as_mut_ptr()) as c_long as size_t;
        buffer = _buffer.as_mut_ptr();
        while bytes > 0i32 as c_ulong {
            bytestocopy = if bytes < (*state).free_in_buffer {
                bytes
            } else {
                (*state).free_in_buffer
            };
            memcpy(
                (*state).next_output_byte as *mut c_void,
                buffer as *const c_void,
                bytestocopy,
            );
            (*state).next_output_byte = (*state).next_output_byte.offset(bytestocopy as isize);
            buffer = buffer.offset(bytestocopy as isize);
            (*state).free_in_buffer =
                ((*state).free_in_buffer as c_ulong).wrapping_sub(bytestocopy) as size_t as size_t;
            if (*state).free_in_buffer == 0i32 as c_ulong {
                if 0 == dump_buffer(state) {
                    return FALSE;
                }
            }
            bytes = (bytes as c_ulong).wrapping_sub(bytestocopy) as size_t as size_t
        }
    } else {
        (*state).free_in_buffer = ((*state).free_in_buffer as c_ulong).wrapping_sub(
            buffer.wrapping_offset_from((*state).next_output_byte) as c_long as c_ulong,
        ) as size_t as size_t;
        (*state).next_output_byte = buffer
    }
    return TRUE;
}
/* Encode a single block's worth of coefficients */
unsafe extern "C" fn encode_one_block_simd(
    mut state: *mut working_state,
    mut block: JCOEFPTR,
    mut last_dc_val: c_int,
    mut dctbl: *mut c_derived_tbl,
    mut actbl: *mut c_derived_tbl,
) -> boolean {
    let mut _buffer: [JOCTET; 256] = [0; 256];
    let mut buffer: *mut JOCTET = 0 as *mut JOCTET;
    let mut bytes: size_t = 0;
    let mut bytestocopy: size_t = 0;
    let mut localbuf: c_int = 0i32;
    if (*state).free_in_buffer < BUFSIZE as c_ulong {
        localbuf = 1i32;
        buffer = _buffer.as_mut_ptr()
    } else {
        buffer = (*state).next_output_byte
    }
    buffer = jsimd_huff_encode_one_block(
        state as *mut c_void,
        buffer,
        block,
        last_dc_val,
        dctbl,
        actbl,
    );
    if 0 != localbuf {
        bytes = buffer.wrapping_offset_from(_buffer.as_mut_ptr()) as c_long as size_t;
        buffer = _buffer.as_mut_ptr();
        while bytes > 0i32 as c_ulong {
            bytestocopy = if bytes < (*state).free_in_buffer {
                bytes
            } else {
                (*state).free_in_buffer
            };
            memcpy(
                (*state).next_output_byte as *mut c_void,
                buffer as *const c_void,
                bytestocopy,
            );
            (*state).next_output_byte = (*state).next_output_byte.offset(bytestocopy as isize);
            buffer = buffer.offset(bytestocopy as isize);
            (*state).free_in_buffer =
                ((*state).free_in_buffer as c_ulong).wrapping_sub(bytestocopy) as size_t as size_t;
            if (*state).free_in_buffer == 0i32 as c_ulong {
                if 0 == dump_buffer(state) {
                    return FALSE;
                }
            }
            bytes = (bytes as c_ulong).wrapping_sub(bytestocopy) as size_t as size_t
        }
    } else {
        (*state).free_in_buffer = ((*state).free_in_buffer as c_ulong).wrapping_sub(
            buffer.wrapping_offset_from((*state).next_output_byte) as c_long as c_ulong,
        ) as size_t as size_t;
        (*state).next_output_byte = buffer
    }
    return TRUE;
}
unsafe extern "C" fn encode_one_block(
    mut state: *mut working_state,
    mut block: JCOEFPTR,
    mut last_dc_val: c_int,
    mut dctbl: *mut c_derived_tbl,
    mut actbl: *mut c_derived_tbl,
) -> boolean {
    let mut temp: c_int = 0;
    let mut temp2: c_int = 0;
    let mut temp3: c_int = 0;
    let mut nbits: c_int = 0;
    let mut r: c_int = 0;
    let mut code: c_int = 0;
    let mut size: c_int = 0;
    let mut _buffer: [JOCTET; 256] = [0; 256];
    let mut buffer: *mut JOCTET = 0 as *mut JOCTET;
    let mut put_buffer: size_t = 0;
    let mut put_bits: c_int = 0;
    let mut code_0xf0: c_int = (*actbl).ehufco[0xf0i32 as usize] as c_int;
    let mut size_0xf0: c_int = (*actbl).ehufsi[0xf0i32 as usize] as c_int;
    let mut bytes: size_t = 0;
    let mut bytestocopy: size_t = 0;
    let mut localbuf: c_int = 0i32;
    put_buffer = (*state).cur.put_buffer;
    put_bits = (*state).cur.put_bits;
    if (*state).free_in_buffer < BUFSIZE as c_ulong {
        localbuf = 1i32;
        buffer = _buffer.as_mut_ptr()
    } else {
        buffer = (*state).next_output_byte
    }
    temp2 = *block.offset(0isize) as c_int - last_dc_val;
    temp = temp2;
    temp3 = temp
        >> (CHAR_BIT as c_ulong)
            .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
            .wrapping_sub(1i32 as c_ulong);
    temp ^= temp3;
    temp -= temp3;
    temp2 += temp3;
    nbits = jpeg_nbits_table[temp as usize] as c_int;
    code = (*dctbl).ehufco[nbits as usize] as c_int;
    size = (*dctbl).ehufsi[nbits as usize] as c_int;
    if put_bits > 47i32 {
        let mut c: JOCTET = 0;
        put_bits -= 8i32;
        c = (put_buffer >> put_bits) as JOCTET;
        let fresh5 = buffer;
        buffer = buffer.offset(1);
        *fresh5 = c;
        if c as c_int == 0xffi32 {
            let fresh6 = buffer;
            buffer = buffer.offset(1);
            *fresh6 = 0i32 as JOCTET
        }
        let mut c_0: JOCTET = 0;
        put_bits -= 8i32;
        c_0 = (put_buffer >> put_bits) as JOCTET;
        let fresh7 = buffer;
        buffer = buffer.offset(1);
        *fresh7 = c_0;
        if c_0 as c_int == 0xffi32 {
            let fresh8 = buffer;
            buffer = buffer.offset(1);
            *fresh8 = 0i32 as JOCTET
        }
        let mut c_1: JOCTET = 0;
        put_bits -= 8i32;
        c_1 = (put_buffer >> put_bits) as JOCTET;
        let fresh9 = buffer;
        buffer = buffer.offset(1);
        *fresh9 = c_1;
        if c_1 as c_int == 0xffi32 {
            let fresh10 = buffer;
            buffer = buffer.offset(1);
            *fresh10 = 0i32 as JOCTET
        }
        let mut c_2: JOCTET = 0;
        put_bits -= 8i32;
        c_2 = (put_buffer >> put_bits) as JOCTET;
        let fresh11 = buffer;
        buffer = buffer.offset(1);
        *fresh11 = c_2;
        if c_2 as c_int == 0xffi32 {
            let fresh12 = buffer;
            buffer = buffer.offset(1);
            *fresh12 = 0i32 as JOCTET
        }
        let mut c_3: JOCTET = 0;
        put_bits -= 8i32;
        c_3 = (put_buffer >> put_bits) as JOCTET;
        let fresh13 = buffer;
        buffer = buffer.offset(1);
        *fresh13 = c_3;
        if c_3 as c_int == 0xffi32 {
            let fresh14 = buffer;
            buffer = buffer.offset(1);
            *fresh14 = 0i32 as JOCTET
        }
        let mut c_4: JOCTET = 0;
        put_bits -= 8i32;
        c_4 = (put_buffer >> put_bits) as JOCTET;
        let fresh15 = buffer;
        buffer = buffer.offset(1);
        *fresh15 = c_4;
        if c_4 as c_int == 0xffi32 {
            let fresh16 = buffer;
            buffer = buffer.offset(1);
            *fresh16 = 0i32 as JOCTET
        }
    }
    put_bits += size;
    put_buffer = put_buffer << size | code as c_ulong;
    temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
    if put_bits > 47i32 {
        let mut c_5: JOCTET = 0;
        put_bits -= 8i32;
        c_5 = (put_buffer >> put_bits) as JOCTET;
        let fresh17 = buffer;
        buffer = buffer.offset(1);
        *fresh17 = c_5;
        if c_5 as c_int == 0xffi32 {
            let fresh18 = buffer;
            buffer = buffer.offset(1);
            *fresh18 = 0i32 as JOCTET
        }
        let mut c_6: JOCTET = 0;
        put_bits -= 8i32;
        c_6 = (put_buffer >> put_bits) as JOCTET;
        let fresh19 = buffer;
        buffer = buffer.offset(1);
        *fresh19 = c_6;
        if c_6 as c_int == 0xffi32 {
            let fresh20 = buffer;
            buffer = buffer.offset(1);
            *fresh20 = 0i32 as JOCTET
        }
        let mut c_7: JOCTET = 0;
        put_bits -= 8i32;
        c_7 = (put_buffer >> put_bits) as JOCTET;
        let fresh21 = buffer;
        buffer = buffer.offset(1);
        *fresh21 = c_7;
        if c_7 as c_int == 0xffi32 {
            let fresh22 = buffer;
            buffer = buffer.offset(1);
            *fresh22 = 0i32 as JOCTET
        }
        let mut c_8: JOCTET = 0;
        put_bits -= 8i32;
        c_8 = (put_buffer >> put_bits) as JOCTET;
        let fresh23 = buffer;
        buffer = buffer.offset(1);
        *fresh23 = c_8;
        if c_8 as c_int == 0xffi32 {
            let fresh24 = buffer;
            buffer = buffer.offset(1);
            *fresh24 = 0i32 as JOCTET
        }
        let mut c_9: JOCTET = 0;
        put_bits -= 8i32;
        c_9 = (put_buffer >> put_bits) as JOCTET;
        let fresh25 = buffer;
        buffer = buffer.offset(1);
        *fresh25 = c_9;
        if c_9 as c_int == 0xffi32 {
            let fresh26 = buffer;
            buffer = buffer.offset(1);
            *fresh26 = 0i32 as JOCTET
        }
        let mut c_10: JOCTET = 0;
        put_bits -= 8i32;
        c_10 = (put_buffer >> put_bits) as JOCTET;
        let fresh27 = buffer;
        buffer = buffer.offset(1);
        *fresh27 = c_10;
        if c_10 as c_int == 0xffi32 {
            let fresh28 = buffer;
            buffer = buffer.offset(1);
            *fresh28 = 0i32 as JOCTET
        }
    }
    put_bits += nbits;
    put_buffer = put_buffer << nbits | temp2 as c_ulong;
    r = 0i32;
    temp = *block.offset(1isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_11: JOCTET = 0;
                put_bits -= 8i32;
                c_11 = (put_buffer >> put_bits) as JOCTET;
                let fresh29 = buffer;
                buffer = buffer.offset(1);
                *fresh29 = c_11;
                if c_11 as c_int == 0xffi32 {
                    let fresh30 = buffer;
                    buffer = buffer.offset(1);
                    *fresh30 = 0i32 as JOCTET
                }
                let mut c_12: JOCTET = 0;
                put_bits -= 8i32;
                c_12 = (put_buffer >> put_bits) as JOCTET;
                let fresh31 = buffer;
                buffer = buffer.offset(1);
                *fresh31 = c_12;
                if c_12 as c_int == 0xffi32 {
                    let fresh32 = buffer;
                    buffer = buffer.offset(1);
                    *fresh32 = 0i32 as JOCTET
                }
                let mut c_13: JOCTET = 0;
                put_bits -= 8i32;
                c_13 = (put_buffer >> put_bits) as JOCTET;
                let fresh33 = buffer;
                buffer = buffer.offset(1);
                *fresh33 = c_13;
                if c_13 as c_int == 0xffi32 {
                    let fresh34 = buffer;
                    buffer = buffer.offset(1);
                    *fresh34 = 0i32 as JOCTET
                }
                let mut c_14: JOCTET = 0;
                put_bits -= 8i32;
                c_14 = (put_buffer >> put_bits) as JOCTET;
                let fresh35 = buffer;
                buffer = buffer.offset(1);
                *fresh35 = c_14;
                if c_14 as c_int == 0xffi32 {
                    let fresh36 = buffer;
                    buffer = buffer.offset(1);
                    *fresh36 = 0i32 as JOCTET
                }
                let mut c_15: JOCTET = 0;
                put_bits -= 8i32;
                c_15 = (put_buffer >> put_bits) as JOCTET;
                let fresh37 = buffer;
                buffer = buffer.offset(1);
                *fresh37 = c_15;
                if c_15 as c_int == 0xffi32 {
                    let fresh38 = buffer;
                    buffer = buffer.offset(1);
                    *fresh38 = 0i32 as JOCTET
                }
                let mut c_16: JOCTET = 0;
                put_bits -= 8i32;
                c_16 = (put_buffer >> put_bits) as JOCTET;
                let fresh39 = buffer;
                buffer = buffer.offset(1);
                *fresh39 = c_16;
                if c_16 as c_int == 0xffi32 {
                    let fresh40 = buffer;
                    buffer = buffer.offset(1);
                    *fresh40 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_17: JOCTET = 0;
            put_bits -= 8i32;
            c_17 = (put_buffer >> put_bits) as JOCTET;
            let fresh41 = buffer;
            buffer = buffer.offset(1);
            *fresh41 = c_17;
            if c_17 as c_int == 0xffi32 {
                let fresh42 = buffer;
                buffer = buffer.offset(1);
                *fresh42 = 0i32 as JOCTET
            }
            let mut c_18: JOCTET = 0;
            put_bits -= 8i32;
            c_18 = (put_buffer >> put_bits) as JOCTET;
            let fresh43 = buffer;
            buffer = buffer.offset(1);
            *fresh43 = c_18;
            if c_18 as c_int == 0xffi32 {
                let fresh44 = buffer;
                buffer = buffer.offset(1);
                *fresh44 = 0i32 as JOCTET
            }
            let mut c_19: JOCTET = 0;
            put_bits -= 8i32;
            c_19 = (put_buffer >> put_bits) as JOCTET;
            let fresh45 = buffer;
            buffer = buffer.offset(1);
            *fresh45 = c_19;
            if c_19 as c_int == 0xffi32 {
                let fresh46 = buffer;
                buffer = buffer.offset(1);
                *fresh46 = 0i32 as JOCTET
            }
            let mut c_20: JOCTET = 0;
            put_bits -= 8i32;
            c_20 = (put_buffer >> put_bits) as JOCTET;
            let fresh47 = buffer;
            buffer = buffer.offset(1);
            *fresh47 = c_20;
            if c_20 as c_int == 0xffi32 {
                let fresh48 = buffer;
                buffer = buffer.offset(1);
                *fresh48 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(8isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_21: JOCTET = 0;
                put_bits -= 8i32;
                c_21 = (put_buffer >> put_bits) as JOCTET;
                let fresh49 = buffer;
                buffer = buffer.offset(1);
                *fresh49 = c_21;
                if c_21 as c_int == 0xffi32 {
                    let fresh50 = buffer;
                    buffer = buffer.offset(1);
                    *fresh50 = 0i32 as JOCTET
                }
                let mut c_22: JOCTET = 0;
                put_bits -= 8i32;
                c_22 = (put_buffer >> put_bits) as JOCTET;
                let fresh51 = buffer;
                buffer = buffer.offset(1);
                *fresh51 = c_22;
                if c_22 as c_int == 0xffi32 {
                    let fresh52 = buffer;
                    buffer = buffer.offset(1);
                    *fresh52 = 0i32 as JOCTET
                }
                let mut c_23: JOCTET = 0;
                put_bits -= 8i32;
                c_23 = (put_buffer >> put_bits) as JOCTET;
                let fresh53 = buffer;
                buffer = buffer.offset(1);
                *fresh53 = c_23;
                if c_23 as c_int == 0xffi32 {
                    let fresh54 = buffer;
                    buffer = buffer.offset(1);
                    *fresh54 = 0i32 as JOCTET
                }
                let mut c_24: JOCTET = 0;
                put_bits -= 8i32;
                c_24 = (put_buffer >> put_bits) as JOCTET;
                let fresh55 = buffer;
                buffer = buffer.offset(1);
                *fresh55 = c_24;
                if c_24 as c_int == 0xffi32 {
                    let fresh56 = buffer;
                    buffer = buffer.offset(1);
                    *fresh56 = 0i32 as JOCTET
                }
                let mut c_25: JOCTET = 0;
                put_bits -= 8i32;
                c_25 = (put_buffer >> put_bits) as JOCTET;
                let fresh57 = buffer;
                buffer = buffer.offset(1);
                *fresh57 = c_25;
                if c_25 as c_int == 0xffi32 {
                    let fresh58 = buffer;
                    buffer = buffer.offset(1);
                    *fresh58 = 0i32 as JOCTET
                }
                let mut c_26: JOCTET = 0;
                put_bits -= 8i32;
                c_26 = (put_buffer >> put_bits) as JOCTET;
                let fresh59 = buffer;
                buffer = buffer.offset(1);
                *fresh59 = c_26;
                if c_26 as c_int == 0xffi32 {
                    let fresh60 = buffer;
                    buffer = buffer.offset(1);
                    *fresh60 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_27: JOCTET = 0;
            put_bits -= 8i32;
            c_27 = (put_buffer >> put_bits) as JOCTET;
            let fresh61 = buffer;
            buffer = buffer.offset(1);
            *fresh61 = c_27;
            if c_27 as c_int == 0xffi32 {
                let fresh62 = buffer;
                buffer = buffer.offset(1);
                *fresh62 = 0i32 as JOCTET
            }
            let mut c_28: JOCTET = 0;
            put_bits -= 8i32;
            c_28 = (put_buffer >> put_bits) as JOCTET;
            let fresh63 = buffer;
            buffer = buffer.offset(1);
            *fresh63 = c_28;
            if c_28 as c_int == 0xffi32 {
                let fresh64 = buffer;
                buffer = buffer.offset(1);
                *fresh64 = 0i32 as JOCTET
            }
            let mut c_29: JOCTET = 0;
            put_bits -= 8i32;
            c_29 = (put_buffer >> put_bits) as JOCTET;
            let fresh65 = buffer;
            buffer = buffer.offset(1);
            *fresh65 = c_29;
            if c_29 as c_int == 0xffi32 {
                let fresh66 = buffer;
                buffer = buffer.offset(1);
                *fresh66 = 0i32 as JOCTET
            }
            let mut c_30: JOCTET = 0;
            put_bits -= 8i32;
            c_30 = (put_buffer >> put_bits) as JOCTET;
            let fresh67 = buffer;
            buffer = buffer.offset(1);
            *fresh67 = c_30;
            if c_30 as c_int == 0xffi32 {
                let fresh68 = buffer;
                buffer = buffer.offset(1);
                *fresh68 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(16isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_31: JOCTET = 0;
                put_bits -= 8i32;
                c_31 = (put_buffer >> put_bits) as JOCTET;
                let fresh69 = buffer;
                buffer = buffer.offset(1);
                *fresh69 = c_31;
                if c_31 as c_int == 0xffi32 {
                    let fresh70 = buffer;
                    buffer = buffer.offset(1);
                    *fresh70 = 0i32 as JOCTET
                }
                let mut c_32: JOCTET = 0;
                put_bits -= 8i32;
                c_32 = (put_buffer >> put_bits) as JOCTET;
                let fresh71 = buffer;
                buffer = buffer.offset(1);
                *fresh71 = c_32;
                if c_32 as c_int == 0xffi32 {
                    let fresh72 = buffer;
                    buffer = buffer.offset(1);
                    *fresh72 = 0i32 as JOCTET
                }
                let mut c_33: JOCTET = 0;
                put_bits -= 8i32;
                c_33 = (put_buffer >> put_bits) as JOCTET;
                let fresh73 = buffer;
                buffer = buffer.offset(1);
                *fresh73 = c_33;
                if c_33 as c_int == 0xffi32 {
                    let fresh74 = buffer;
                    buffer = buffer.offset(1);
                    *fresh74 = 0i32 as JOCTET
                }
                let mut c_34: JOCTET = 0;
                put_bits -= 8i32;
                c_34 = (put_buffer >> put_bits) as JOCTET;
                let fresh75 = buffer;
                buffer = buffer.offset(1);
                *fresh75 = c_34;
                if c_34 as c_int == 0xffi32 {
                    let fresh76 = buffer;
                    buffer = buffer.offset(1);
                    *fresh76 = 0i32 as JOCTET
                }
                let mut c_35: JOCTET = 0;
                put_bits -= 8i32;
                c_35 = (put_buffer >> put_bits) as JOCTET;
                let fresh77 = buffer;
                buffer = buffer.offset(1);
                *fresh77 = c_35;
                if c_35 as c_int == 0xffi32 {
                    let fresh78 = buffer;
                    buffer = buffer.offset(1);
                    *fresh78 = 0i32 as JOCTET
                }
                let mut c_36: JOCTET = 0;
                put_bits -= 8i32;
                c_36 = (put_buffer >> put_bits) as JOCTET;
                let fresh79 = buffer;
                buffer = buffer.offset(1);
                *fresh79 = c_36;
                if c_36 as c_int == 0xffi32 {
                    let fresh80 = buffer;
                    buffer = buffer.offset(1);
                    *fresh80 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_37: JOCTET = 0;
            put_bits -= 8i32;
            c_37 = (put_buffer >> put_bits) as JOCTET;
            let fresh81 = buffer;
            buffer = buffer.offset(1);
            *fresh81 = c_37;
            if c_37 as c_int == 0xffi32 {
                let fresh82 = buffer;
                buffer = buffer.offset(1);
                *fresh82 = 0i32 as JOCTET
            }
            let mut c_38: JOCTET = 0;
            put_bits -= 8i32;
            c_38 = (put_buffer >> put_bits) as JOCTET;
            let fresh83 = buffer;
            buffer = buffer.offset(1);
            *fresh83 = c_38;
            if c_38 as c_int == 0xffi32 {
                let fresh84 = buffer;
                buffer = buffer.offset(1);
                *fresh84 = 0i32 as JOCTET
            }
            let mut c_39: JOCTET = 0;
            put_bits -= 8i32;
            c_39 = (put_buffer >> put_bits) as JOCTET;
            let fresh85 = buffer;
            buffer = buffer.offset(1);
            *fresh85 = c_39;
            if c_39 as c_int == 0xffi32 {
                let fresh86 = buffer;
                buffer = buffer.offset(1);
                *fresh86 = 0i32 as JOCTET
            }
            let mut c_40: JOCTET = 0;
            put_bits -= 8i32;
            c_40 = (put_buffer >> put_bits) as JOCTET;
            let fresh87 = buffer;
            buffer = buffer.offset(1);
            *fresh87 = c_40;
            if c_40 as c_int == 0xffi32 {
                let fresh88 = buffer;
                buffer = buffer.offset(1);
                *fresh88 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(9isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_41: JOCTET = 0;
                put_bits -= 8i32;
                c_41 = (put_buffer >> put_bits) as JOCTET;
                let fresh89 = buffer;
                buffer = buffer.offset(1);
                *fresh89 = c_41;
                if c_41 as c_int == 0xffi32 {
                    let fresh90 = buffer;
                    buffer = buffer.offset(1);
                    *fresh90 = 0i32 as JOCTET
                }
                let mut c_42: JOCTET = 0;
                put_bits -= 8i32;
                c_42 = (put_buffer >> put_bits) as JOCTET;
                let fresh91 = buffer;
                buffer = buffer.offset(1);
                *fresh91 = c_42;
                if c_42 as c_int == 0xffi32 {
                    let fresh92 = buffer;
                    buffer = buffer.offset(1);
                    *fresh92 = 0i32 as JOCTET
                }
                let mut c_43: JOCTET = 0;
                put_bits -= 8i32;
                c_43 = (put_buffer >> put_bits) as JOCTET;
                let fresh93 = buffer;
                buffer = buffer.offset(1);
                *fresh93 = c_43;
                if c_43 as c_int == 0xffi32 {
                    let fresh94 = buffer;
                    buffer = buffer.offset(1);
                    *fresh94 = 0i32 as JOCTET
                }
                let mut c_44: JOCTET = 0;
                put_bits -= 8i32;
                c_44 = (put_buffer >> put_bits) as JOCTET;
                let fresh95 = buffer;
                buffer = buffer.offset(1);
                *fresh95 = c_44;
                if c_44 as c_int == 0xffi32 {
                    let fresh96 = buffer;
                    buffer = buffer.offset(1);
                    *fresh96 = 0i32 as JOCTET
                }
                let mut c_45: JOCTET = 0;
                put_bits -= 8i32;
                c_45 = (put_buffer >> put_bits) as JOCTET;
                let fresh97 = buffer;
                buffer = buffer.offset(1);
                *fresh97 = c_45;
                if c_45 as c_int == 0xffi32 {
                    let fresh98 = buffer;
                    buffer = buffer.offset(1);
                    *fresh98 = 0i32 as JOCTET
                }
                let mut c_46: JOCTET = 0;
                put_bits -= 8i32;
                c_46 = (put_buffer >> put_bits) as JOCTET;
                let fresh99 = buffer;
                buffer = buffer.offset(1);
                *fresh99 = c_46;
                if c_46 as c_int == 0xffi32 {
                    let fresh100 = buffer;
                    buffer = buffer.offset(1);
                    *fresh100 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_47: JOCTET = 0;
            put_bits -= 8i32;
            c_47 = (put_buffer >> put_bits) as JOCTET;
            let fresh101 = buffer;
            buffer = buffer.offset(1);
            *fresh101 = c_47;
            if c_47 as c_int == 0xffi32 {
                let fresh102 = buffer;
                buffer = buffer.offset(1);
                *fresh102 = 0i32 as JOCTET
            }
            let mut c_48: JOCTET = 0;
            put_bits -= 8i32;
            c_48 = (put_buffer >> put_bits) as JOCTET;
            let fresh103 = buffer;
            buffer = buffer.offset(1);
            *fresh103 = c_48;
            if c_48 as c_int == 0xffi32 {
                let fresh104 = buffer;
                buffer = buffer.offset(1);
                *fresh104 = 0i32 as JOCTET
            }
            let mut c_49: JOCTET = 0;
            put_bits -= 8i32;
            c_49 = (put_buffer >> put_bits) as JOCTET;
            let fresh105 = buffer;
            buffer = buffer.offset(1);
            *fresh105 = c_49;
            if c_49 as c_int == 0xffi32 {
                let fresh106 = buffer;
                buffer = buffer.offset(1);
                *fresh106 = 0i32 as JOCTET
            }
            let mut c_50: JOCTET = 0;
            put_bits -= 8i32;
            c_50 = (put_buffer >> put_bits) as JOCTET;
            let fresh107 = buffer;
            buffer = buffer.offset(1);
            *fresh107 = c_50;
            if c_50 as c_int == 0xffi32 {
                let fresh108 = buffer;
                buffer = buffer.offset(1);
                *fresh108 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(2isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_51: JOCTET = 0;
                put_bits -= 8i32;
                c_51 = (put_buffer >> put_bits) as JOCTET;
                let fresh109 = buffer;
                buffer = buffer.offset(1);
                *fresh109 = c_51;
                if c_51 as c_int == 0xffi32 {
                    let fresh110 = buffer;
                    buffer = buffer.offset(1);
                    *fresh110 = 0i32 as JOCTET
                }
                let mut c_52: JOCTET = 0;
                put_bits -= 8i32;
                c_52 = (put_buffer >> put_bits) as JOCTET;
                let fresh111 = buffer;
                buffer = buffer.offset(1);
                *fresh111 = c_52;
                if c_52 as c_int == 0xffi32 {
                    let fresh112 = buffer;
                    buffer = buffer.offset(1);
                    *fresh112 = 0i32 as JOCTET
                }
                let mut c_53: JOCTET = 0;
                put_bits -= 8i32;
                c_53 = (put_buffer >> put_bits) as JOCTET;
                let fresh113 = buffer;
                buffer = buffer.offset(1);
                *fresh113 = c_53;
                if c_53 as c_int == 0xffi32 {
                    let fresh114 = buffer;
                    buffer = buffer.offset(1);
                    *fresh114 = 0i32 as JOCTET
                }
                let mut c_54: JOCTET = 0;
                put_bits -= 8i32;
                c_54 = (put_buffer >> put_bits) as JOCTET;
                let fresh115 = buffer;
                buffer = buffer.offset(1);
                *fresh115 = c_54;
                if c_54 as c_int == 0xffi32 {
                    let fresh116 = buffer;
                    buffer = buffer.offset(1);
                    *fresh116 = 0i32 as JOCTET
                }
                let mut c_55: JOCTET = 0;
                put_bits -= 8i32;
                c_55 = (put_buffer >> put_bits) as JOCTET;
                let fresh117 = buffer;
                buffer = buffer.offset(1);
                *fresh117 = c_55;
                if c_55 as c_int == 0xffi32 {
                    let fresh118 = buffer;
                    buffer = buffer.offset(1);
                    *fresh118 = 0i32 as JOCTET
                }
                let mut c_56: JOCTET = 0;
                put_bits -= 8i32;
                c_56 = (put_buffer >> put_bits) as JOCTET;
                let fresh119 = buffer;
                buffer = buffer.offset(1);
                *fresh119 = c_56;
                if c_56 as c_int == 0xffi32 {
                    let fresh120 = buffer;
                    buffer = buffer.offset(1);
                    *fresh120 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_57: JOCTET = 0;
            put_bits -= 8i32;
            c_57 = (put_buffer >> put_bits) as JOCTET;
            let fresh121 = buffer;
            buffer = buffer.offset(1);
            *fresh121 = c_57;
            if c_57 as c_int == 0xffi32 {
                let fresh122 = buffer;
                buffer = buffer.offset(1);
                *fresh122 = 0i32 as JOCTET
            }
            let mut c_58: JOCTET = 0;
            put_bits -= 8i32;
            c_58 = (put_buffer >> put_bits) as JOCTET;
            let fresh123 = buffer;
            buffer = buffer.offset(1);
            *fresh123 = c_58;
            if c_58 as c_int == 0xffi32 {
                let fresh124 = buffer;
                buffer = buffer.offset(1);
                *fresh124 = 0i32 as JOCTET
            }
            let mut c_59: JOCTET = 0;
            put_bits -= 8i32;
            c_59 = (put_buffer >> put_bits) as JOCTET;
            let fresh125 = buffer;
            buffer = buffer.offset(1);
            *fresh125 = c_59;
            if c_59 as c_int == 0xffi32 {
                let fresh126 = buffer;
                buffer = buffer.offset(1);
                *fresh126 = 0i32 as JOCTET
            }
            let mut c_60: JOCTET = 0;
            put_bits -= 8i32;
            c_60 = (put_buffer >> put_bits) as JOCTET;
            let fresh127 = buffer;
            buffer = buffer.offset(1);
            *fresh127 = c_60;
            if c_60 as c_int == 0xffi32 {
                let fresh128 = buffer;
                buffer = buffer.offset(1);
                *fresh128 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(3isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_61: JOCTET = 0;
                put_bits -= 8i32;
                c_61 = (put_buffer >> put_bits) as JOCTET;
                let fresh129 = buffer;
                buffer = buffer.offset(1);
                *fresh129 = c_61;
                if c_61 as c_int == 0xffi32 {
                    let fresh130 = buffer;
                    buffer = buffer.offset(1);
                    *fresh130 = 0i32 as JOCTET
                }
                let mut c_62: JOCTET = 0;
                put_bits -= 8i32;
                c_62 = (put_buffer >> put_bits) as JOCTET;
                let fresh131 = buffer;
                buffer = buffer.offset(1);
                *fresh131 = c_62;
                if c_62 as c_int == 0xffi32 {
                    let fresh132 = buffer;
                    buffer = buffer.offset(1);
                    *fresh132 = 0i32 as JOCTET
                }
                let mut c_63: JOCTET = 0;
                put_bits -= 8i32;
                c_63 = (put_buffer >> put_bits) as JOCTET;
                let fresh133 = buffer;
                buffer = buffer.offset(1);
                *fresh133 = c_63;
                if c_63 as c_int == 0xffi32 {
                    let fresh134 = buffer;
                    buffer = buffer.offset(1);
                    *fresh134 = 0i32 as JOCTET
                }
                let mut c_64: JOCTET = 0;
                put_bits -= 8i32;
                c_64 = (put_buffer >> put_bits) as JOCTET;
                let fresh135 = buffer;
                buffer = buffer.offset(1);
                *fresh135 = c_64;
                if c_64 as c_int == 0xffi32 {
                    let fresh136 = buffer;
                    buffer = buffer.offset(1);
                    *fresh136 = 0i32 as JOCTET
                }
                let mut c_65: JOCTET = 0;
                put_bits -= 8i32;
                c_65 = (put_buffer >> put_bits) as JOCTET;
                let fresh137 = buffer;
                buffer = buffer.offset(1);
                *fresh137 = c_65;
                if c_65 as c_int == 0xffi32 {
                    let fresh138 = buffer;
                    buffer = buffer.offset(1);
                    *fresh138 = 0i32 as JOCTET
                }
                let mut c_66: JOCTET = 0;
                put_bits -= 8i32;
                c_66 = (put_buffer >> put_bits) as JOCTET;
                let fresh139 = buffer;
                buffer = buffer.offset(1);
                *fresh139 = c_66;
                if c_66 as c_int == 0xffi32 {
                    let fresh140 = buffer;
                    buffer = buffer.offset(1);
                    *fresh140 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_67: JOCTET = 0;
            put_bits -= 8i32;
            c_67 = (put_buffer >> put_bits) as JOCTET;
            let fresh141 = buffer;
            buffer = buffer.offset(1);
            *fresh141 = c_67;
            if c_67 as c_int == 0xffi32 {
                let fresh142 = buffer;
                buffer = buffer.offset(1);
                *fresh142 = 0i32 as JOCTET
            }
            let mut c_68: JOCTET = 0;
            put_bits -= 8i32;
            c_68 = (put_buffer >> put_bits) as JOCTET;
            let fresh143 = buffer;
            buffer = buffer.offset(1);
            *fresh143 = c_68;
            if c_68 as c_int == 0xffi32 {
                let fresh144 = buffer;
                buffer = buffer.offset(1);
                *fresh144 = 0i32 as JOCTET
            }
            let mut c_69: JOCTET = 0;
            put_bits -= 8i32;
            c_69 = (put_buffer >> put_bits) as JOCTET;
            let fresh145 = buffer;
            buffer = buffer.offset(1);
            *fresh145 = c_69;
            if c_69 as c_int == 0xffi32 {
                let fresh146 = buffer;
                buffer = buffer.offset(1);
                *fresh146 = 0i32 as JOCTET
            }
            let mut c_70: JOCTET = 0;
            put_bits -= 8i32;
            c_70 = (put_buffer >> put_bits) as JOCTET;
            let fresh147 = buffer;
            buffer = buffer.offset(1);
            *fresh147 = c_70;
            if c_70 as c_int == 0xffi32 {
                let fresh148 = buffer;
                buffer = buffer.offset(1);
                *fresh148 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(10isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_71: JOCTET = 0;
                put_bits -= 8i32;
                c_71 = (put_buffer >> put_bits) as JOCTET;
                let fresh149 = buffer;
                buffer = buffer.offset(1);
                *fresh149 = c_71;
                if c_71 as c_int == 0xffi32 {
                    let fresh150 = buffer;
                    buffer = buffer.offset(1);
                    *fresh150 = 0i32 as JOCTET
                }
                let mut c_72: JOCTET = 0;
                put_bits -= 8i32;
                c_72 = (put_buffer >> put_bits) as JOCTET;
                let fresh151 = buffer;
                buffer = buffer.offset(1);
                *fresh151 = c_72;
                if c_72 as c_int == 0xffi32 {
                    let fresh152 = buffer;
                    buffer = buffer.offset(1);
                    *fresh152 = 0i32 as JOCTET
                }
                let mut c_73: JOCTET = 0;
                put_bits -= 8i32;
                c_73 = (put_buffer >> put_bits) as JOCTET;
                let fresh153 = buffer;
                buffer = buffer.offset(1);
                *fresh153 = c_73;
                if c_73 as c_int == 0xffi32 {
                    let fresh154 = buffer;
                    buffer = buffer.offset(1);
                    *fresh154 = 0i32 as JOCTET
                }
                let mut c_74: JOCTET = 0;
                put_bits -= 8i32;
                c_74 = (put_buffer >> put_bits) as JOCTET;
                let fresh155 = buffer;
                buffer = buffer.offset(1);
                *fresh155 = c_74;
                if c_74 as c_int == 0xffi32 {
                    let fresh156 = buffer;
                    buffer = buffer.offset(1);
                    *fresh156 = 0i32 as JOCTET
                }
                let mut c_75: JOCTET = 0;
                put_bits -= 8i32;
                c_75 = (put_buffer >> put_bits) as JOCTET;
                let fresh157 = buffer;
                buffer = buffer.offset(1);
                *fresh157 = c_75;
                if c_75 as c_int == 0xffi32 {
                    let fresh158 = buffer;
                    buffer = buffer.offset(1);
                    *fresh158 = 0i32 as JOCTET
                }
                let mut c_76: JOCTET = 0;
                put_bits -= 8i32;
                c_76 = (put_buffer >> put_bits) as JOCTET;
                let fresh159 = buffer;
                buffer = buffer.offset(1);
                *fresh159 = c_76;
                if c_76 as c_int == 0xffi32 {
                    let fresh160 = buffer;
                    buffer = buffer.offset(1);
                    *fresh160 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_77: JOCTET = 0;
            put_bits -= 8i32;
            c_77 = (put_buffer >> put_bits) as JOCTET;
            let fresh161 = buffer;
            buffer = buffer.offset(1);
            *fresh161 = c_77;
            if c_77 as c_int == 0xffi32 {
                let fresh162 = buffer;
                buffer = buffer.offset(1);
                *fresh162 = 0i32 as JOCTET
            }
            let mut c_78: JOCTET = 0;
            put_bits -= 8i32;
            c_78 = (put_buffer >> put_bits) as JOCTET;
            let fresh163 = buffer;
            buffer = buffer.offset(1);
            *fresh163 = c_78;
            if c_78 as c_int == 0xffi32 {
                let fresh164 = buffer;
                buffer = buffer.offset(1);
                *fresh164 = 0i32 as JOCTET
            }
            let mut c_79: JOCTET = 0;
            put_bits -= 8i32;
            c_79 = (put_buffer >> put_bits) as JOCTET;
            let fresh165 = buffer;
            buffer = buffer.offset(1);
            *fresh165 = c_79;
            if c_79 as c_int == 0xffi32 {
                let fresh166 = buffer;
                buffer = buffer.offset(1);
                *fresh166 = 0i32 as JOCTET
            }
            let mut c_80: JOCTET = 0;
            put_bits -= 8i32;
            c_80 = (put_buffer >> put_bits) as JOCTET;
            let fresh167 = buffer;
            buffer = buffer.offset(1);
            *fresh167 = c_80;
            if c_80 as c_int == 0xffi32 {
                let fresh168 = buffer;
                buffer = buffer.offset(1);
                *fresh168 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(17isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_81: JOCTET = 0;
                put_bits -= 8i32;
                c_81 = (put_buffer >> put_bits) as JOCTET;
                let fresh169 = buffer;
                buffer = buffer.offset(1);
                *fresh169 = c_81;
                if c_81 as c_int == 0xffi32 {
                    let fresh170 = buffer;
                    buffer = buffer.offset(1);
                    *fresh170 = 0i32 as JOCTET
                }
                let mut c_82: JOCTET = 0;
                put_bits -= 8i32;
                c_82 = (put_buffer >> put_bits) as JOCTET;
                let fresh171 = buffer;
                buffer = buffer.offset(1);
                *fresh171 = c_82;
                if c_82 as c_int == 0xffi32 {
                    let fresh172 = buffer;
                    buffer = buffer.offset(1);
                    *fresh172 = 0i32 as JOCTET
                }
                let mut c_83: JOCTET = 0;
                put_bits -= 8i32;
                c_83 = (put_buffer >> put_bits) as JOCTET;
                let fresh173 = buffer;
                buffer = buffer.offset(1);
                *fresh173 = c_83;
                if c_83 as c_int == 0xffi32 {
                    let fresh174 = buffer;
                    buffer = buffer.offset(1);
                    *fresh174 = 0i32 as JOCTET
                }
                let mut c_84: JOCTET = 0;
                put_bits -= 8i32;
                c_84 = (put_buffer >> put_bits) as JOCTET;
                let fresh175 = buffer;
                buffer = buffer.offset(1);
                *fresh175 = c_84;
                if c_84 as c_int == 0xffi32 {
                    let fresh176 = buffer;
                    buffer = buffer.offset(1);
                    *fresh176 = 0i32 as JOCTET
                }
                let mut c_85: JOCTET = 0;
                put_bits -= 8i32;
                c_85 = (put_buffer >> put_bits) as JOCTET;
                let fresh177 = buffer;
                buffer = buffer.offset(1);
                *fresh177 = c_85;
                if c_85 as c_int == 0xffi32 {
                    let fresh178 = buffer;
                    buffer = buffer.offset(1);
                    *fresh178 = 0i32 as JOCTET
                }
                let mut c_86: JOCTET = 0;
                put_bits -= 8i32;
                c_86 = (put_buffer >> put_bits) as JOCTET;
                let fresh179 = buffer;
                buffer = buffer.offset(1);
                *fresh179 = c_86;
                if c_86 as c_int == 0xffi32 {
                    let fresh180 = buffer;
                    buffer = buffer.offset(1);
                    *fresh180 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_87: JOCTET = 0;
            put_bits -= 8i32;
            c_87 = (put_buffer >> put_bits) as JOCTET;
            let fresh181 = buffer;
            buffer = buffer.offset(1);
            *fresh181 = c_87;
            if c_87 as c_int == 0xffi32 {
                let fresh182 = buffer;
                buffer = buffer.offset(1);
                *fresh182 = 0i32 as JOCTET
            }
            let mut c_88: JOCTET = 0;
            put_bits -= 8i32;
            c_88 = (put_buffer >> put_bits) as JOCTET;
            let fresh183 = buffer;
            buffer = buffer.offset(1);
            *fresh183 = c_88;
            if c_88 as c_int == 0xffi32 {
                let fresh184 = buffer;
                buffer = buffer.offset(1);
                *fresh184 = 0i32 as JOCTET
            }
            let mut c_89: JOCTET = 0;
            put_bits -= 8i32;
            c_89 = (put_buffer >> put_bits) as JOCTET;
            let fresh185 = buffer;
            buffer = buffer.offset(1);
            *fresh185 = c_89;
            if c_89 as c_int == 0xffi32 {
                let fresh186 = buffer;
                buffer = buffer.offset(1);
                *fresh186 = 0i32 as JOCTET
            }
            let mut c_90: JOCTET = 0;
            put_bits -= 8i32;
            c_90 = (put_buffer >> put_bits) as JOCTET;
            let fresh187 = buffer;
            buffer = buffer.offset(1);
            *fresh187 = c_90;
            if c_90 as c_int == 0xffi32 {
                let fresh188 = buffer;
                buffer = buffer.offset(1);
                *fresh188 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(24isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_91: JOCTET = 0;
                put_bits -= 8i32;
                c_91 = (put_buffer >> put_bits) as JOCTET;
                let fresh189 = buffer;
                buffer = buffer.offset(1);
                *fresh189 = c_91;
                if c_91 as c_int == 0xffi32 {
                    let fresh190 = buffer;
                    buffer = buffer.offset(1);
                    *fresh190 = 0i32 as JOCTET
                }
                let mut c_92: JOCTET = 0;
                put_bits -= 8i32;
                c_92 = (put_buffer >> put_bits) as JOCTET;
                let fresh191 = buffer;
                buffer = buffer.offset(1);
                *fresh191 = c_92;
                if c_92 as c_int == 0xffi32 {
                    let fresh192 = buffer;
                    buffer = buffer.offset(1);
                    *fresh192 = 0i32 as JOCTET
                }
                let mut c_93: JOCTET = 0;
                put_bits -= 8i32;
                c_93 = (put_buffer >> put_bits) as JOCTET;
                let fresh193 = buffer;
                buffer = buffer.offset(1);
                *fresh193 = c_93;
                if c_93 as c_int == 0xffi32 {
                    let fresh194 = buffer;
                    buffer = buffer.offset(1);
                    *fresh194 = 0i32 as JOCTET
                }
                let mut c_94: JOCTET = 0;
                put_bits -= 8i32;
                c_94 = (put_buffer >> put_bits) as JOCTET;
                let fresh195 = buffer;
                buffer = buffer.offset(1);
                *fresh195 = c_94;
                if c_94 as c_int == 0xffi32 {
                    let fresh196 = buffer;
                    buffer = buffer.offset(1);
                    *fresh196 = 0i32 as JOCTET
                }
                let mut c_95: JOCTET = 0;
                put_bits -= 8i32;
                c_95 = (put_buffer >> put_bits) as JOCTET;
                let fresh197 = buffer;
                buffer = buffer.offset(1);
                *fresh197 = c_95;
                if c_95 as c_int == 0xffi32 {
                    let fresh198 = buffer;
                    buffer = buffer.offset(1);
                    *fresh198 = 0i32 as JOCTET
                }
                let mut c_96: JOCTET = 0;
                put_bits -= 8i32;
                c_96 = (put_buffer >> put_bits) as JOCTET;
                let fresh199 = buffer;
                buffer = buffer.offset(1);
                *fresh199 = c_96;
                if c_96 as c_int == 0xffi32 {
                    let fresh200 = buffer;
                    buffer = buffer.offset(1);
                    *fresh200 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_97: JOCTET = 0;
            put_bits -= 8i32;
            c_97 = (put_buffer >> put_bits) as JOCTET;
            let fresh201 = buffer;
            buffer = buffer.offset(1);
            *fresh201 = c_97;
            if c_97 as c_int == 0xffi32 {
                let fresh202 = buffer;
                buffer = buffer.offset(1);
                *fresh202 = 0i32 as JOCTET
            }
            let mut c_98: JOCTET = 0;
            put_bits -= 8i32;
            c_98 = (put_buffer >> put_bits) as JOCTET;
            let fresh203 = buffer;
            buffer = buffer.offset(1);
            *fresh203 = c_98;
            if c_98 as c_int == 0xffi32 {
                let fresh204 = buffer;
                buffer = buffer.offset(1);
                *fresh204 = 0i32 as JOCTET
            }
            let mut c_99: JOCTET = 0;
            put_bits -= 8i32;
            c_99 = (put_buffer >> put_bits) as JOCTET;
            let fresh205 = buffer;
            buffer = buffer.offset(1);
            *fresh205 = c_99;
            if c_99 as c_int == 0xffi32 {
                let fresh206 = buffer;
                buffer = buffer.offset(1);
                *fresh206 = 0i32 as JOCTET
            }
            let mut c_100: JOCTET = 0;
            put_bits -= 8i32;
            c_100 = (put_buffer >> put_bits) as JOCTET;
            let fresh207 = buffer;
            buffer = buffer.offset(1);
            *fresh207 = c_100;
            if c_100 as c_int == 0xffi32 {
                let fresh208 = buffer;
                buffer = buffer.offset(1);
                *fresh208 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(32isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_101: JOCTET = 0;
                put_bits -= 8i32;
                c_101 = (put_buffer >> put_bits) as JOCTET;
                let fresh209 = buffer;
                buffer = buffer.offset(1);
                *fresh209 = c_101;
                if c_101 as c_int == 0xffi32 {
                    let fresh210 = buffer;
                    buffer = buffer.offset(1);
                    *fresh210 = 0i32 as JOCTET
                }
                let mut c_102: JOCTET = 0;
                put_bits -= 8i32;
                c_102 = (put_buffer >> put_bits) as JOCTET;
                let fresh211 = buffer;
                buffer = buffer.offset(1);
                *fresh211 = c_102;
                if c_102 as c_int == 0xffi32 {
                    let fresh212 = buffer;
                    buffer = buffer.offset(1);
                    *fresh212 = 0i32 as JOCTET
                }
                let mut c_103: JOCTET = 0;
                put_bits -= 8i32;
                c_103 = (put_buffer >> put_bits) as JOCTET;
                let fresh213 = buffer;
                buffer = buffer.offset(1);
                *fresh213 = c_103;
                if c_103 as c_int == 0xffi32 {
                    let fresh214 = buffer;
                    buffer = buffer.offset(1);
                    *fresh214 = 0i32 as JOCTET
                }
                let mut c_104: JOCTET = 0;
                put_bits -= 8i32;
                c_104 = (put_buffer >> put_bits) as JOCTET;
                let fresh215 = buffer;
                buffer = buffer.offset(1);
                *fresh215 = c_104;
                if c_104 as c_int == 0xffi32 {
                    let fresh216 = buffer;
                    buffer = buffer.offset(1);
                    *fresh216 = 0i32 as JOCTET
                }
                let mut c_105: JOCTET = 0;
                put_bits -= 8i32;
                c_105 = (put_buffer >> put_bits) as JOCTET;
                let fresh217 = buffer;
                buffer = buffer.offset(1);
                *fresh217 = c_105;
                if c_105 as c_int == 0xffi32 {
                    let fresh218 = buffer;
                    buffer = buffer.offset(1);
                    *fresh218 = 0i32 as JOCTET
                }
                let mut c_106: JOCTET = 0;
                put_bits -= 8i32;
                c_106 = (put_buffer >> put_bits) as JOCTET;
                let fresh219 = buffer;
                buffer = buffer.offset(1);
                *fresh219 = c_106;
                if c_106 as c_int == 0xffi32 {
                    let fresh220 = buffer;
                    buffer = buffer.offset(1);
                    *fresh220 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_107: JOCTET = 0;
            put_bits -= 8i32;
            c_107 = (put_buffer >> put_bits) as JOCTET;
            let fresh221 = buffer;
            buffer = buffer.offset(1);
            *fresh221 = c_107;
            if c_107 as c_int == 0xffi32 {
                let fresh222 = buffer;
                buffer = buffer.offset(1);
                *fresh222 = 0i32 as JOCTET
            }
            let mut c_108: JOCTET = 0;
            put_bits -= 8i32;
            c_108 = (put_buffer >> put_bits) as JOCTET;
            let fresh223 = buffer;
            buffer = buffer.offset(1);
            *fresh223 = c_108;
            if c_108 as c_int == 0xffi32 {
                let fresh224 = buffer;
                buffer = buffer.offset(1);
                *fresh224 = 0i32 as JOCTET
            }
            let mut c_109: JOCTET = 0;
            put_bits -= 8i32;
            c_109 = (put_buffer >> put_bits) as JOCTET;
            let fresh225 = buffer;
            buffer = buffer.offset(1);
            *fresh225 = c_109;
            if c_109 as c_int == 0xffi32 {
                let fresh226 = buffer;
                buffer = buffer.offset(1);
                *fresh226 = 0i32 as JOCTET
            }
            let mut c_110: JOCTET = 0;
            put_bits -= 8i32;
            c_110 = (put_buffer >> put_bits) as JOCTET;
            let fresh227 = buffer;
            buffer = buffer.offset(1);
            *fresh227 = c_110;
            if c_110 as c_int == 0xffi32 {
                let fresh228 = buffer;
                buffer = buffer.offset(1);
                *fresh228 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(25isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_111: JOCTET = 0;
                put_bits -= 8i32;
                c_111 = (put_buffer >> put_bits) as JOCTET;
                let fresh229 = buffer;
                buffer = buffer.offset(1);
                *fresh229 = c_111;
                if c_111 as c_int == 0xffi32 {
                    let fresh230 = buffer;
                    buffer = buffer.offset(1);
                    *fresh230 = 0i32 as JOCTET
                }
                let mut c_112: JOCTET = 0;
                put_bits -= 8i32;
                c_112 = (put_buffer >> put_bits) as JOCTET;
                let fresh231 = buffer;
                buffer = buffer.offset(1);
                *fresh231 = c_112;
                if c_112 as c_int == 0xffi32 {
                    let fresh232 = buffer;
                    buffer = buffer.offset(1);
                    *fresh232 = 0i32 as JOCTET
                }
                let mut c_113: JOCTET = 0;
                put_bits -= 8i32;
                c_113 = (put_buffer >> put_bits) as JOCTET;
                let fresh233 = buffer;
                buffer = buffer.offset(1);
                *fresh233 = c_113;
                if c_113 as c_int == 0xffi32 {
                    let fresh234 = buffer;
                    buffer = buffer.offset(1);
                    *fresh234 = 0i32 as JOCTET
                }
                let mut c_114: JOCTET = 0;
                put_bits -= 8i32;
                c_114 = (put_buffer >> put_bits) as JOCTET;
                let fresh235 = buffer;
                buffer = buffer.offset(1);
                *fresh235 = c_114;
                if c_114 as c_int == 0xffi32 {
                    let fresh236 = buffer;
                    buffer = buffer.offset(1);
                    *fresh236 = 0i32 as JOCTET
                }
                let mut c_115: JOCTET = 0;
                put_bits -= 8i32;
                c_115 = (put_buffer >> put_bits) as JOCTET;
                let fresh237 = buffer;
                buffer = buffer.offset(1);
                *fresh237 = c_115;
                if c_115 as c_int == 0xffi32 {
                    let fresh238 = buffer;
                    buffer = buffer.offset(1);
                    *fresh238 = 0i32 as JOCTET
                }
                let mut c_116: JOCTET = 0;
                put_bits -= 8i32;
                c_116 = (put_buffer >> put_bits) as JOCTET;
                let fresh239 = buffer;
                buffer = buffer.offset(1);
                *fresh239 = c_116;
                if c_116 as c_int == 0xffi32 {
                    let fresh240 = buffer;
                    buffer = buffer.offset(1);
                    *fresh240 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_117: JOCTET = 0;
            put_bits -= 8i32;
            c_117 = (put_buffer >> put_bits) as JOCTET;
            let fresh241 = buffer;
            buffer = buffer.offset(1);
            *fresh241 = c_117;
            if c_117 as c_int == 0xffi32 {
                let fresh242 = buffer;
                buffer = buffer.offset(1);
                *fresh242 = 0i32 as JOCTET
            }
            let mut c_118: JOCTET = 0;
            put_bits -= 8i32;
            c_118 = (put_buffer >> put_bits) as JOCTET;
            let fresh243 = buffer;
            buffer = buffer.offset(1);
            *fresh243 = c_118;
            if c_118 as c_int == 0xffi32 {
                let fresh244 = buffer;
                buffer = buffer.offset(1);
                *fresh244 = 0i32 as JOCTET
            }
            let mut c_119: JOCTET = 0;
            put_bits -= 8i32;
            c_119 = (put_buffer >> put_bits) as JOCTET;
            let fresh245 = buffer;
            buffer = buffer.offset(1);
            *fresh245 = c_119;
            if c_119 as c_int == 0xffi32 {
                let fresh246 = buffer;
                buffer = buffer.offset(1);
                *fresh246 = 0i32 as JOCTET
            }
            let mut c_120: JOCTET = 0;
            put_bits -= 8i32;
            c_120 = (put_buffer >> put_bits) as JOCTET;
            let fresh247 = buffer;
            buffer = buffer.offset(1);
            *fresh247 = c_120;
            if c_120 as c_int == 0xffi32 {
                let fresh248 = buffer;
                buffer = buffer.offset(1);
                *fresh248 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(18isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_121: JOCTET = 0;
                put_bits -= 8i32;
                c_121 = (put_buffer >> put_bits) as JOCTET;
                let fresh249 = buffer;
                buffer = buffer.offset(1);
                *fresh249 = c_121;
                if c_121 as c_int == 0xffi32 {
                    let fresh250 = buffer;
                    buffer = buffer.offset(1);
                    *fresh250 = 0i32 as JOCTET
                }
                let mut c_122: JOCTET = 0;
                put_bits -= 8i32;
                c_122 = (put_buffer >> put_bits) as JOCTET;
                let fresh251 = buffer;
                buffer = buffer.offset(1);
                *fresh251 = c_122;
                if c_122 as c_int == 0xffi32 {
                    let fresh252 = buffer;
                    buffer = buffer.offset(1);
                    *fresh252 = 0i32 as JOCTET
                }
                let mut c_123: JOCTET = 0;
                put_bits -= 8i32;
                c_123 = (put_buffer >> put_bits) as JOCTET;
                let fresh253 = buffer;
                buffer = buffer.offset(1);
                *fresh253 = c_123;
                if c_123 as c_int == 0xffi32 {
                    let fresh254 = buffer;
                    buffer = buffer.offset(1);
                    *fresh254 = 0i32 as JOCTET
                }
                let mut c_124: JOCTET = 0;
                put_bits -= 8i32;
                c_124 = (put_buffer >> put_bits) as JOCTET;
                let fresh255 = buffer;
                buffer = buffer.offset(1);
                *fresh255 = c_124;
                if c_124 as c_int == 0xffi32 {
                    let fresh256 = buffer;
                    buffer = buffer.offset(1);
                    *fresh256 = 0i32 as JOCTET
                }
                let mut c_125: JOCTET = 0;
                put_bits -= 8i32;
                c_125 = (put_buffer >> put_bits) as JOCTET;
                let fresh257 = buffer;
                buffer = buffer.offset(1);
                *fresh257 = c_125;
                if c_125 as c_int == 0xffi32 {
                    let fresh258 = buffer;
                    buffer = buffer.offset(1);
                    *fresh258 = 0i32 as JOCTET
                }
                let mut c_126: JOCTET = 0;
                put_bits -= 8i32;
                c_126 = (put_buffer >> put_bits) as JOCTET;
                let fresh259 = buffer;
                buffer = buffer.offset(1);
                *fresh259 = c_126;
                if c_126 as c_int == 0xffi32 {
                    let fresh260 = buffer;
                    buffer = buffer.offset(1);
                    *fresh260 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_127: JOCTET = 0;
            put_bits -= 8i32;
            c_127 = (put_buffer >> put_bits) as JOCTET;
            let fresh261 = buffer;
            buffer = buffer.offset(1);
            *fresh261 = c_127;
            if c_127 as c_int == 0xffi32 {
                let fresh262 = buffer;
                buffer = buffer.offset(1);
                *fresh262 = 0i32 as JOCTET
            }
            let mut c_128: JOCTET = 0;
            put_bits -= 8i32;
            c_128 = (put_buffer >> put_bits) as JOCTET;
            let fresh263 = buffer;
            buffer = buffer.offset(1);
            *fresh263 = c_128;
            if c_128 as c_int == 0xffi32 {
                let fresh264 = buffer;
                buffer = buffer.offset(1);
                *fresh264 = 0i32 as JOCTET
            }
            let mut c_129: JOCTET = 0;
            put_bits -= 8i32;
            c_129 = (put_buffer >> put_bits) as JOCTET;
            let fresh265 = buffer;
            buffer = buffer.offset(1);
            *fresh265 = c_129;
            if c_129 as c_int == 0xffi32 {
                let fresh266 = buffer;
                buffer = buffer.offset(1);
                *fresh266 = 0i32 as JOCTET
            }
            let mut c_130: JOCTET = 0;
            put_bits -= 8i32;
            c_130 = (put_buffer >> put_bits) as JOCTET;
            let fresh267 = buffer;
            buffer = buffer.offset(1);
            *fresh267 = c_130;
            if c_130 as c_int == 0xffi32 {
                let fresh268 = buffer;
                buffer = buffer.offset(1);
                *fresh268 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(11isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_131: JOCTET = 0;
                put_bits -= 8i32;
                c_131 = (put_buffer >> put_bits) as JOCTET;
                let fresh269 = buffer;
                buffer = buffer.offset(1);
                *fresh269 = c_131;
                if c_131 as c_int == 0xffi32 {
                    let fresh270 = buffer;
                    buffer = buffer.offset(1);
                    *fresh270 = 0i32 as JOCTET
                }
                let mut c_132: JOCTET = 0;
                put_bits -= 8i32;
                c_132 = (put_buffer >> put_bits) as JOCTET;
                let fresh271 = buffer;
                buffer = buffer.offset(1);
                *fresh271 = c_132;
                if c_132 as c_int == 0xffi32 {
                    let fresh272 = buffer;
                    buffer = buffer.offset(1);
                    *fresh272 = 0i32 as JOCTET
                }
                let mut c_133: JOCTET = 0;
                put_bits -= 8i32;
                c_133 = (put_buffer >> put_bits) as JOCTET;
                let fresh273 = buffer;
                buffer = buffer.offset(1);
                *fresh273 = c_133;
                if c_133 as c_int == 0xffi32 {
                    let fresh274 = buffer;
                    buffer = buffer.offset(1);
                    *fresh274 = 0i32 as JOCTET
                }
                let mut c_134: JOCTET = 0;
                put_bits -= 8i32;
                c_134 = (put_buffer >> put_bits) as JOCTET;
                let fresh275 = buffer;
                buffer = buffer.offset(1);
                *fresh275 = c_134;
                if c_134 as c_int == 0xffi32 {
                    let fresh276 = buffer;
                    buffer = buffer.offset(1);
                    *fresh276 = 0i32 as JOCTET
                }
                let mut c_135: JOCTET = 0;
                put_bits -= 8i32;
                c_135 = (put_buffer >> put_bits) as JOCTET;
                let fresh277 = buffer;
                buffer = buffer.offset(1);
                *fresh277 = c_135;
                if c_135 as c_int == 0xffi32 {
                    let fresh278 = buffer;
                    buffer = buffer.offset(1);
                    *fresh278 = 0i32 as JOCTET
                }
                let mut c_136: JOCTET = 0;
                put_bits -= 8i32;
                c_136 = (put_buffer >> put_bits) as JOCTET;
                let fresh279 = buffer;
                buffer = buffer.offset(1);
                *fresh279 = c_136;
                if c_136 as c_int == 0xffi32 {
                    let fresh280 = buffer;
                    buffer = buffer.offset(1);
                    *fresh280 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_137: JOCTET = 0;
            put_bits -= 8i32;
            c_137 = (put_buffer >> put_bits) as JOCTET;
            let fresh281 = buffer;
            buffer = buffer.offset(1);
            *fresh281 = c_137;
            if c_137 as c_int == 0xffi32 {
                let fresh282 = buffer;
                buffer = buffer.offset(1);
                *fresh282 = 0i32 as JOCTET
            }
            let mut c_138: JOCTET = 0;
            put_bits -= 8i32;
            c_138 = (put_buffer >> put_bits) as JOCTET;
            let fresh283 = buffer;
            buffer = buffer.offset(1);
            *fresh283 = c_138;
            if c_138 as c_int == 0xffi32 {
                let fresh284 = buffer;
                buffer = buffer.offset(1);
                *fresh284 = 0i32 as JOCTET
            }
            let mut c_139: JOCTET = 0;
            put_bits -= 8i32;
            c_139 = (put_buffer >> put_bits) as JOCTET;
            let fresh285 = buffer;
            buffer = buffer.offset(1);
            *fresh285 = c_139;
            if c_139 as c_int == 0xffi32 {
                let fresh286 = buffer;
                buffer = buffer.offset(1);
                *fresh286 = 0i32 as JOCTET
            }
            let mut c_140: JOCTET = 0;
            put_bits -= 8i32;
            c_140 = (put_buffer >> put_bits) as JOCTET;
            let fresh287 = buffer;
            buffer = buffer.offset(1);
            *fresh287 = c_140;
            if c_140 as c_int == 0xffi32 {
                let fresh288 = buffer;
                buffer = buffer.offset(1);
                *fresh288 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(4isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_141: JOCTET = 0;
                put_bits -= 8i32;
                c_141 = (put_buffer >> put_bits) as JOCTET;
                let fresh289 = buffer;
                buffer = buffer.offset(1);
                *fresh289 = c_141;
                if c_141 as c_int == 0xffi32 {
                    let fresh290 = buffer;
                    buffer = buffer.offset(1);
                    *fresh290 = 0i32 as JOCTET
                }
                let mut c_142: JOCTET = 0;
                put_bits -= 8i32;
                c_142 = (put_buffer >> put_bits) as JOCTET;
                let fresh291 = buffer;
                buffer = buffer.offset(1);
                *fresh291 = c_142;
                if c_142 as c_int == 0xffi32 {
                    let fresh292 = buffer;
                    buffer = buffer.offset(1);
                    *fresh292 = 0i32 as JOCTET
                }
                let mut c_143: JOCTET = 0;
                put_bits -= 8i32;
                c_143 = (put_buffer >> put_bits) as JOCTET;
                let fresh293 = buffer;
                buffer = buffer.offset(1);
                *fresh293 = c_143;
                if c_143 as c_int == 0xffi32 {
                    let fresh294 = buffer;
                    buffer = buffer.offset(1);
                    *fresh294 = 0i32 as JOCTET
                }
                let mut c_144: JOCTET = 0;
                put_bits -= 8i32;
                c_144 = (put_buffer >> put_bits) as JOCTET;
                let fresh295 = buffer;
                buffer = buffer.offset(1);
                *fresh295 = c_144;
                if c_144 as c_int == 0xffi32 {
                    let fresh296 = buffer;
                    buffer = buffer.offset(1);
                    *fresh296 = 0i32 as JOCTET
                }
                let mut c_145: JOCTET = 0;
                put_bits -= 8i32;
                c_145 = (put_buffer >> put_bits) as JOCTET;
                let fresh297 = buffer;
                buffer = buffer.offset(1);
                *fresh297 = c_145;
                if c_145 as c_int == 0xffi32 {
                    let fresh298 = buffer;
                    buffer = buffer.offset(1);
                    *fresh298 = 0i32 as JOCTET
                }
                let mut c_146: JOCTET = 0;
                put_bits -= 8i32;
                c_146 = (put_buffer >> put_bits) as JOCTET;
                let fresh299 = buffer;
                buffer = buffer.offset(1);
                *fresh299 = c_146;
                if c_146 as c_int == 0xffi32 {
                    let fresh300 = buffer;
                    buffer = buffer.offset(1);
                    *fresh300 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_147: JOCTET = 0;
            put_bits -= 8i32;
            c_147 = (put_buffer >> put_bits) as JOCTET;
            let fresh301 = buffer;
            buffer = buffer.offset(1);
            *fresh301 = c_147;
            if c_147 as c_int == 0xffi32 {
                let fresh302 = buffer;
                buffer = buffer.offset(1);
                *fresh302 = 0i32 as JOCTET
            }
            let mut c_148: JOCTET = 0;
            put_bits -= 8i32;
            c_148 = (put_buffer >> put_bits) as JOCTET;
            let fresh303 = buffer;
            buffer = buffer.offset(1);
            *fresh303 = c_148;
            if c_148 as c_int == 0xffi32 {
                let fresh304 = buffer;
                buffer = buffer.offset(1);
                *fresh304 = 0i32 as JOCTET
            }
            let mut c_149: JOCTET = 0;
            put_bits -= 8i32;
            c_149 = (put_buffer >> put_bits) as JOCTET;
            let fresh305 = buffer;
            buffer = buffer.offset(1);
            *fresh305 = c_149;
            if c_149 as c_int == 0xffi32 {
                let fresh306 = buffer;
                buffer = buffer.offset(1);
                *fresh306 = 0i32 as JOCTET
            }
            let mut c_150: JOCTET = 0;
            put_bits -= 8i32;
            c_150 = (put_buffer >> put_bits) as JOCTET;
            let fresh307 = buffer;
            buffer = buffer.offset(1);
            *fresh307 = c_150;
            if c_150 as c_int == 0xffi32 {
                let fresh308 = buffer;
                buffer = buffer.offset(1);
                *fresh308 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(5isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_151: JOCTET = 0;
                put_bits -= 8i32;
                c_151 = (put_buffer >> put_bits) as JOCTET;
                let fresh309 = buffer;
                buffer = buffer.offset(1);
                *fresh309 = c_151;
                if c_151 as c_int == 0xffi32 {
                    let fresh310 = buffer;
                    buffer = buffer.offset(1);
                    *fresh310 = 0i32 as JOCTET
                }
                let mut c_152: JOCTET = 0;
                put_bits -= 8i32;
                c_152 = (put_buffer >> put_bits) as JOCTET;
                let fresh311 = buffer;
                buffer = buffer.offset(1);
                *fresh311 = c_152;
                if c_152 as c_int == 0xffi32 {
                    let fresh312 = buffer;
                    buffer = buffer.offset(1);
                    *fresh312 = 0i32 as JOCTET
                }
                let mut c_153: JOCTET = 0;
                put_bits -= 8i32;
                c_153 = (put_buffer >> put_bits) as JOCTET;
                let fresh313 = buffer;
                buffer = buffer.offset(1);
                *fresh313 = c_153;
                if c_153 as c_int == 0xffi32 {
                    let fresh314 = buffer;
                    buffer = buffer.offset(1);
                    *fresh314 = 0i32 as JOCTET
                }
                let mut c_154: JOCTET = 0;
                put_bits -= 8i32;
                c_154 = (put_buffer >> put_bits) as JOCTET;
                let fresh315 = buffer;
                buffer = buffer.offset(1);
                *fresh315 = c_154;
                if c_154 as c_int == 0xffi32 {
                    let fresh316 = buffer;
                    buffer = buffer.offset(1);
                    *fresh316 = 0i32 as JOCTET
                }
                let mut c_155: JOCTET = 0;
                put_bits -= 8i32;
                c_155 = (put_buffer >> put_bits) as JOCTET;
                let fresh317 = buffer;
                buffer = buffer.offset(1);
                *fresh317 = c_155;
                if c_155 as c_int == 0xffi32 {
                    let fresh318 = buffer;
                    buffer = buffer.offset(1);
                    *fresh318 = 0i32 as JOCTET
                }
                let mut c_156: JOCTET = 0;
                put_bits -= 8i32;
                c_156 = (put_buffer >> put_bits) as JOCTET;
                let fresh319 = buffer;
                buffer = buffer.offset(1);
                *fresh319 = c_156;
                if c_156 as c_int == 0xffi32 {
                    let fresh320 = buffer;
                    buffer = buffer.offset(1);
                    *fresh320 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_157: JOCTET = 0;
            put_bits -= 8i32;
            c_157 = (put_buffer >> put_bits) as JOCTET;
            let fresh321 = buffer;
            buffer = buffer.offset(1);
            *fresh321 = c_157;
            if c_157 as c_int == 0xffi32 {
                let fresh322 = buffer;
                buffer = buffer.offset(1);
                *fresh322 = 0i32 as JOCTET
            }
            let mut c_158: JOCTET = 0;
            put_bits -= 8i32;
            c_158 = (put_buffer >> put_bits) as JOCTET;
            let fresh323 = buffer;
            buffer = buffer.offset(1);
            *fresh323 = c_158;
            if c_158 as c_int == 0xffi32 {
                let fresh324 = buffer;
                buffer = buffer.offset(1);
                *fresh324 = 0i32 as JOCTET
            }
            let mut c_159: JOCTET = 0;
            put_bits -= 8i32;
            c_159 = (put_buffer >> put_bits) as JOCTET;
            let fresh325 = buffer;
            buffer = buffer.offset(1);
            *fresh325 = c_159;
            if c_159 as c_int == 0xffi32 {
                let fresh326 = buffer;
                buffer = buffer.offset(1);
                *fresh326 = 0i32 as JOCTET
            }
            let mut c_160: JOCTET = 0;
            put_bits -= 8i32;
            c_160 = (put_buffer >> put_bits) as JOCTET;
            let fresh327 = buffer;
            buffer = buffer.offset(1);
            *fresh327 = c_160;
            if c_160 as c_int == 0xffi32 {
                let fresh328 = buffer;
                buffer = buffer.offset(1);
                *fresh328 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(12isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_161: JOCTET = 0;
                put_bits -= 8i32;
                c_161 = (put_buffer >> put_bits) as JOCTET;
                let fresh329 = buffer;
                buffer = buffer.offset(1);
                *fresh329 = c_161;
                if c_161 as c_int == 0xffi32 {
                    let fresh330 = buffer;
                    buffer = buffer.offset(1);
                    *fresh330 = 0i32 as JOCTET
                }
                let mut c_162: JOCTET = 0;
                put_bits -= 8i32;
                c_162 = (put_buffer >> put_bits) as JOCTET;
                let fresh331 = buffer;
                buffer = buffer.offset(1);
                *fresh331 = c_162;
                if c_162 as c_int == 0xffi32 {
                    let fresh332 = buffer;
                    buffer = buffer.offset(1);
                    *fresh332 = 0i32 as JOCTET
                }
                let mut c_163: JOCTET = 0;
                put_bits -= 8i32;
                c_163 = (put_buffer >> put_bits) as JOCTET;
                let fresh333 = buffer;
                buffer = buffer.offset(1);
                *fresh333 = c_163;
                if c_163 as c_int == 0xffi32 {
                    let fresh334 = buffer;
                    buffer = buffer.offset(1);
                    *fresh334 = 0i32 as JOCTET
                }
                let mut c_164: JOCTET = 0;
                put_bits -= 8i32;
                c_164 = (put_buffer >> put_bits) as JOCTET;
                let fresh335 = buffer;
                buffer = buffer.offset(1);
                *fresh335 = c_164;
                if c_164 as c_int == 0xffi32 {
                    let fresh336 = buffer;
                    buffer = buffer.offset(1);
                    *fresh336 = 0i32 as JOCTET
                }
                let mut c_165: JOCTET = 0;
                put_bits -= 8i32;
                c_165 = (put_buffer >> put_bits) as JOCTET;
                let fresh337 = buffer;
                buffer = buffer.offset(1);
                *fresh337 = c_165;
                if c_165 as c_int == 0xffi32 {
                    let fresh338 = buffer;
                    buffer = buffer.offset(1);
                    *fresh338 = 0i32 as JOCTET
                }
                let mut c_166: JOCTET = 0;
                put_bits -= 8i32;
                c_166 = (put_buffer >> put_bits) as JOCTET;
                let fresh339 = buffer;
                buffer = buffer.offset(1);
                *fresh339 = c_166;
                if c_166 as c_int == 0xffi32 {
                    let fresh340 = buffer;
                    buffer = buffer.offset(1);
                    *fresh340 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_167: JOCTET = 0;
            put_bits -= 8i32;
            c_167 = (put_buffer >> put_bits) as JOCTET;
            let fresh341 = buffer;
            buffer = buffer.offset(1);
            *fresh341 = c_167;
            if c_167 as c_int == 0xffi32 {
                let fresh342 = buffer;
                buffer = buffer.offset(1);
                *fresh342 = 0i32 as JOCTET
            }
            let mut c_168: JOCTET = 0;
            put_bits -= 8i32;
            c_168 = (put_buffer >> put_bits) as JOCTET;
            let fresh343 = buffer;
            buffer = buffer.offset(1);
            *fresh343 = c_168;
            if c_168 as c_int == 0xffi32 {
                let fresh344 = buffer;
                buffer = buffer.offset(1);
                *fresh344 = 0i32 as JOCTET
            }
            let mut c_169: JOCTET = 0;
            put_bits -= 8i32;
            c_169 = (put_buffer >> put_bits) as JOCTET;
            let fresh345 = buffer;
            buffer = buffer.offset(1);
            *fresh345 = c_169;
            if c_169 as c_int == 0xffi32 {
                let fresh346 = buffer;
                buffer = buffer.offset(1);
                *fresh346 = 0i32 as JOCTET
            }
            let mut c_170: JOCTET = 0;
            put_bits -= 8i32;
            c_170 = (put_buffer >> put_bits) as JOCTET;
            let fresh347 = buffer;
            buffer = buffer.offset(1);
            *fresh347 = c_170;
            if c_170 as c_int == 0xffi32 {
                let fresh348 = buffer;
                buffer = buffer.offset(1);
                *fresh348 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(19isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_171: JOCTET = 0;
                put_bits -= 8i32;
                c_171 = (put_buffer >> put_bits) as JOCTET;
                let fresh349 = buffer;
                buffer = buffer.offset(1);
                *fresh349 = c_171;
                if c_171 as c_int == 0xffi32 {
                    let fresh350 = buffer;
                    buffer = buffer.offset(1);
                    *fresh350 = 0i32 as JOCTET
                }
                let mut c_172: JOCTET = 0;
                put_bits -= 8i32;
                c_172 = (put_buffer >> put_bits) as JOCTET;
                let fresh351 = buffer;
                buffer = buffer.offset(1);
                *fresh351 = c_172;
                if c_172 as c_int == 0xffi32 {
                    let fresh352 = buffer;
                    buffer = buffer.offset(1);
                    *fresh352 = 0i32 as JOCTET
                }
                let mut c_173: JOCTET = 0;
                put_bits -= 8i32;
                c_173 = (put_buffer >> put_bits) as JOCTET;
                let fresh353 = buffer;
                buffer = buffer.offset(1);
                *fresh353 = c_173;
                if c_173 as c_int == 0xffi32 {
                    let fresh354 = buffer;
                    buffer = buffer.offset(1);
                    *fresh354 = 0i32 as JOCTET
                }
                let mut c_174: JOCTET = 0;
                put_bits -= 8i32;
                c_174 = (put_buffer >> put_bits) as JOCTET;
                let fresh355 = buffer;
                buffer = buffer.offset(1);
                *fresh355 = c_174;
                if c_174 as c_int == 0xffi32 {
                    let fresh356 = buffer;
                    buffer = buffer.offset(1);
                    *fresh356 = 0i32 as JOCTET
                }
                let mut c_175: JOCTET = 0;
                put_bits -= 8i32;
                c_175 = (put_buffer >> put_bits) as JOCTET;
                let fresh357 = buffer;
                buffer = buffer.offset(1);
                *fresh357 = c_175;
                if c_175 as c_int == 0xffi32 {
                    let fresh358 = buffer;
                    buffer = buffer.offset(1);
                    *fresh358 = 0i32 as JOCTET
                }
                let mut c_176: JOCTET = 0;
                put_bits -= 8i32;
                c_176 = (put_buffer >> put_bits) as JOCTET;
                let fresh359 = buffer;
                buffer = buffer.offset(1);
                *fresh359 = c_176;
                if c_176 as c_int == 0xffi32 {
                    let fresh360 = buffer;
                    buffer = buffer.offset(1);
                    *fresh360 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_177: JOCTET = 0;
            put_bits -= 8i32;
            c_177 = (put_buffer >> put_bits) as JOCTET;
            let fresh361 = buffer;
            buffer = buffer.offset(1);
            *fresh361 = c_177;
            if c_177 as c_int == 0xffi32 {
                let fresh362 = buffer;
                buffer = buffer.offset(1);
                *fresh362 = 0i32 as JOCTET
            }
            let mut c_178: JOCTET = 0;
            put_bits -= 8i32;
            c_178 = (put_buffer >> put_bits) as JOCTET;
            let fresh363 = buffer;
            buffer = buffer.offset(1);
            *fresh363 = c_178;
            if c_178 as c_int == 0xffi32 {
                let fresh364 = buffer;
                buffer = buffer.offset(1);
                *fresh364 = 0i32 as JOCTET
            }
            let mut c_179: JOCTET = 0;
            put_bits -= 8i32;
            c_179 = (put_buffer >> put_bits) as JOCTET;
            let fresh365 = buffer;
            buffer = buffer.offset(1);
            *fresh365 = c_179;
            if c_179 as c_int == 0xffi32 {
                let fresh366 = buffer;
                buffer = buffer.offset(1);
                *fresh366 = 0i32 as JOCTET
            }
            let mut c_180: JOCTET = 0;
            put_bits -= 8i32;
            c_180 = (put_buffer >> put_bits) as JOCTET;
            let fresh367 = buffer;
            buffer = buffer.offset(1);
            *fresh367 = c_180;
            if c_180 as c_int == 0xffi32 {
                let fresh368 = buffer;
                buffer = buffer.offset(1);
                *fresh368 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(26isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_181: JOCTET = 0;
                put_bits -= 8i32;
                c_181 = (put_buffer >> put_bits) as JOCTET;
                let fresh369 = buffer;
                buffer = buffer.offset(1);
                *fresh369 = c_181;
                if c_181 as c_int == 0xffi32 {
                    let fresh370 = buffer;
                    buffer = buffer.offset(1);
                    *fresh370 = 0i32 as JOCTET
                }
                let mut c_182: JOCTET = 0;
                put_bits -= 8i32;
                c_182 = (put_buffer >> put_bits) as JOCTET;
                let fresh371 = buffer;
                buffer = buffer.offset(1);
                *fresh371 = c_182;
                if c_182 as c_int == 0xffi32 {
                    let fresh372 = buffer;
                    buffer = buffer.offset(1);
                    *fresh372 = 0i32 as JOCTET
                }
                let mut c_183: JOCTET = 0;
                put_bits -= 8i32;
                c_183 = (put_buffer >> put_bits) as JOCTET;
                let fresh373 = buffer;
                buffer = buffer.offset(1);
                *fresh373 = c_183;
                if c_183 as c_int == 0xffi32 {
                    let fresh374 = buffer;
                    buffer = buffer.offset(1);
                    *fresh374 = 0i32 as JOCTET
                }
                let mut c_184: JOCTET = 0;
                put_bits -= 8i32;
                c_184 = (put_buffer >> put_bits) as JOCTET;
                let fresh375 = buffer;
                buffer = buffer.offset(1);
                *fresh375 = c_184;
                if c_184 as c_int == 0xffi32 {
                    let fresh376 = buffer;
                    buffer = buffer.offset(1);
                    *fresh376 = 0i32 as JOCTET
                }
                let mut c_185: JOCTET = 0;
                put_bits -= 8i32;
                c_185 = (put_buffer >> put_bits) as JOCTET;
                let fresh377 = buffer;
                buffer = buffer.offset(1);
                *fresh377 = c_185;
                if c_185 as c_int == 0xffi32 {
                    let fresh378 = buffer;
                    buffer = buffer.offset(1);
                    *fresh378 = 0i32 as JOCTET
                }
                let mut c_186: JOCTET = 0;
                put_bits -= 8i32;
                c_186 = (put_buffer >> put_bits) as JOCTET;
                let fresh379 = buffer;
                buffer = buffer.offset(1);
                *fresh379 = c_186;
                if c_186 as c_int == 0xffi32 {
                    let fresh380 = buffer;
                    buffer = buffer.offset(1);
                    *fresh380 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_187: JOCTET = 0;
            put_bits -= 8i32;
            c_187 = (put_buffer >> put_bits) as JOCTET;
            let fresh381 = buffer;
            buffer = buffer.offset(1);
            *fresh381 = c_187;
            if c_187 as c_int == 0xffi32 {
                let fresh382 = buffer;
                buffer = buffer.offset(1);
                *fresh382 = 0i32 as JOCTET
            }
            let mut c_188: JOCTET = 0;
            put_bits -= 8i32;
            c_188 = (put_buffer >> put_bits) as JOCTET;
            let fresh383 = buffer;
            buffer = buffer.offset(1);
            *fresh383 = c_188;
            if c_188 as c_int == 0xffi32 {
                let fresh384 = buffer;
                buffer = buffer.offset(1);
                *fresh384 = 0i32 as JOCTET
            }
            let mut c_189: JOCTET = 0;
            put_bits -= 8i32;
            c_189 = (put_buffer >> put_bits) as JOCTET;
            let fresh385 = buffer;
            buffer = buffer.offset(1);
            *fresh385 = c_189;
            if c_189 as c_int == 0xffi32 {
                let fresh386 = buffer;
                buffer = buffer.offset(1);
                *fresh386 = 0i32 as JOCTET
            }
            let mut c_190: JOCTET = 0;
            put_bits -= 8i32;
            c_190 = (put_buffer >> put_bits) as JOCTET;
            let fresh387 = buffer;
            buffer = buffer.offset(1);
            *fresh387 = c_190;
            if c_190 as c_int == 0xffi32 {
                let fresh388 = buffer;
                buffer = buffer.offset(1);
                *fresh388 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(33isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_191: JOCTET = 0;
                put_bits -= 8i32;
                c_191 = (put_buffer >> put_bits) as JOCTET;
                let fresh389 = buffer;
                buffer = buffer.offset(1);
                *fresh389 = c_191;
                if c_191 as c_int == 0xffi32 {
                    let fresh390 = buffer;
                    buffer = buffer.offset(1);
                    *fresh390 = 0i32 as JOCTET
                }
                let mut c_192: JOCTET = 0;
                put_bits -= 8i32;
                c_192 = (put_buffer >> put_bits) as JOCTET;
                let fresh391 = buffer;
                buffer = buffer.offset(1);
                *fresh391 = c_192;
                if c_192 as c_int == 0xffi32 {
                    let fresh392 = buffer;
                    buffer = buffer.offset(1);
                    *fresh392 = 0i32 as JOCTET
                }
                let mut c_193: JOCTET = 0;
                put_bits -= 8i32;
                c_193 = (put_buffer >> put_bits) as JOCTET;
                let fresh393 = buffer;
                buffer = buffer.offset(1);
                *fresh393 = c_193;
                if c_193 as c_int == 0xffi32 {
                    let fresh394 = buffer;
                    buffer = buffer.offset(1);
                    *fresh394 = 0i32 as JOCTET
                }
                let mut c_194: JOCTET = 0;
                put_bits -= 8i32;
                c_194 = (put_buffer >> put_bits) as JOCTET;
                let fresh395 = buffer;
                buffer = buffer.offset(1);
                *fresh395 = c_194;
                if c_194 as c_int == 0xffi32 {
                    let fresh396 = buffer;
                    buffer = buffer.offset(1);
                    *fresh396 = 0i32 as JOCTET
                }
                let mut c_195: JOCTET = 0;
                put_bits -= 8i32;
                c_195 = (put_buffer >> put_bits) as JOCTET;
                let fresh397 = buffer;
                buffer = buffer.offset(1);
                *fresh397 = c_195;
                if c_195 as c_int == 0xffi32 {
                    let fresh398 = buffer;
                    buffer = buffer.offset(1);
                    *fresh398 = 0i32 as JOCTET
                }
                let mut c_196: JOCTET = 0;
                put_bits -= 8i32;
                c_196 = (put_buffer >> put_bits) as JOCTET;
                let fresh399 = buffer;
                buffer = buffer.offset(1);
                *fresh399 = c_196;
                if c_196 as c_int == 0xffi32 {
                    let fresh400 = buffer;
                    buffer = buffer.offset(1);
                    *fresh400 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_197: JOCTET = 0;
            put_bits -= 8i32;
            c_197 = (put_buffer >> put_bits) as JOCTET;
            let fresh401 = buffer;
            buffer = buffer.offset(1);
            *fresh401 = c_197;
            if c_197 as c_int == 0xffi32 {
                let fresh402 = buffer;
                buffer = buffer.offset(1);
                *fresh402 = 0i32 as JOCTET
            }
            let mut c_198: JOCTET = 0;
            put_bits -= 8i32;
            c_198 = (put_buffer >> put_bits) as JOCTET;
            let fresh403 = buffer;
            buffer = buffer.offset(1);
            *fresh403 = c_198;
            if c_198 as c_int == 0xffi32 {
                let fresh404 = buffer;
                buffer = buffer.offset(1);
                *fresh404 = 0i32 as JOCTET
            }
            let mut c_199: JOCTET = 0;
            put_bits -= 8i32;
            c_199 = (put_buffer >> put_bits) as JOCTET;
            let fresh405 = buffer;
            buffer = buffer.offset(1);
            *fresh405 = c_199;
            if c_199 as c_int == 0xffi32 {
                let fresh406 = buffer;
                buffer = buffer.offset(1);
                *fresh406 = 0i32 as JOCTET
            }
            let mut c_200: JOCTET = 0;
            put_bits -= 8i32;
            c_200 = (put_buffer >> put_bits) as JOCTET;
            let fresh407 = buffer;
            buffer = buffer.offset(1);
            *fresh407 = c_200;
            if c_200 as c_int == 0xffi32 {
                let fresh408 = buffer;
                buffer = buffer.offset(1);
                *fresh408 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(40isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_201: JOCTET = 0;
                put_bits -= 8i32;
                c_201 = (put_buffer >> put_bits) as JOCTET;
                let fresh409 = buffer;
                buffer = buffer.offset(1);
                *fresh409 = c_201;
                if c_201 as c_int == 0xffi32 {
                    let fresh410 = buffer;
                    buffer = buffer.offset(1);
                    *fresh410 = 0i32 as JOCTET
                }
                let mut c_202: JOCTET = 0;
                put_bits -= 8i32;
                c_202 = (put_buffer >> put_bits) as JOCTET;
                let fresh411 = buffer;
                buffer = buffer.offset(1);
                *fresh411 = c_202;
                if c_202 as c_int == 0xffi32 {
                    let fresh412 = buffer;
                    buffer = buffer.offset(1);
                    *fresh412 = 0i32 as JOCTET
                }
                let mut c_203: JOCTET = 0;
                put_bits -= 8i32;
                c_203 = (put_buffer >> put_bits) as JOCTET;
                let fresh413 = buffer;
                buffer = buffer.offset(1);
                *fresh413 = c_203;
                if c_203 as c_int == 0xffi32 {
                    let fresh414 = buffer;
                    buffer = buffer.offset(1);
                    *fresh414 = 0i32 as JOCTET
                }
                let mut c_204: JOCTET = 0;
                put_bits -= 8i32;
                c_204 = (put_buffer >> put_bits) as JOCTET;
                let fresh415 = buffer;
                buffer = buffer.offset(1);
                *fresh415 = c_204;
                if c_204 as c_int == 0xffi32 {
                    let fresh416 = buffer;
                    buffer = buffer.offset(1);
                    *fresh416 = 0i32 as JOCTET
                }
                let mut c_205: JOCTET = 0;
                put_bits -= 8i32;
                c_205 = (put_buffer >> put_bits) as JOCTET;
                let fresh417 = buffer;
                buffer = buffer.offset(1);
                *fresh417 = c_205;
                if c_205 as c_int == 0xffi32 {
                    let fresh418 = buffer;
                    buffer = buffer.offset(1);
                    *fresh418 = 0i32 as JOCTET
                }
                let mut c_206: JOCTET = 0;
                put_bits -= 8i32;
                c_206 = (put_buffer >> put_bits) as JOCTET;
                let fresh419 = buffer;
                buffer = buffer.offset(1);
                *fresh419 = c_206;
                if c_206 as c_int == 0xffi32 {
                    let fresh420 = buffer;
                    buffer = buffer.offset(1);
                    *fresh420 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_207: JOCTET = 0;
            put_bits -= 8i32;
            c_207 = (put_buffer >> put_bits) as JOCTET;
            let fresh421 = buffer;
            buffer = buffer.offset(1);
            *fresh421 = c_207;
            if c_207 as c_int == 0xffi32 {
                let fresh422 = buffer;
                buffer = buffer.offset(1);
                *fresh422 = 0i32 as JOCTET
            }
            let mut c_208: JOCTET = 0;
            put_bits -= 8i32;
            c_208 = (put_buffer >> put_bits) as JOCTET;
            let fresh423 = buffer;
            buffer = buffer.offset(1);
            *fresh423 = c_208;
            if c_208 as c_int == 0xffi32 {
                let fresh424 = buffer;
                buffer = buffer.offset(1);
                *fresh424 = 0i32 as JOCTET
            }
            let mut c_209: JOCTET = 0;
            put_bits -= 8i32;
            c_209 = (put_buffer >> put_bits) as JOCTET;
            let fresh425 = buffer;
            buffer = buffer.offset(1);
            *fresh425 = c_209;
            if c_209 as c_int == 0xffi32 {
                let fresh426 = buffer;
                buffer = buffer.offset(1);
                *fresh426 = 0i32 as JOCTET
            }
            let mut c_210: JOCTET = 0;
            put_bits -= 8i32;
            c_210 = (put_buffer >> put_bits) as JOCTET;
            let fresh427 = buffer;
            buffer = buffer.offset(1);
            *fresh427 = c_210;
            if c_210 as c_int == 0xffi32 {
                let fresh428 = buffer;
                buffer = buffer.offset(1);
                *fresh428 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(48isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_211: JOCTET = 0;
                put_bits -= 8i32;
                c_211 = (put_buffer >> put_bits) as JOCTET;
                let fresh429 = buffer;
                buffer = buffer.offset(1);
                *fresh429 = c_211;
                if c_211 as c_int == 0xffi32 {
                    let fresh430 = buffer;
                    buffer = buffer.offset(1);
                    *fresh430 = 0i32 as JOCTET
                }
                let mut c_212: JOCTET = 0;
                put_bits -= 8i32;
                c_212 = (put_buffer >> put_bits) as JOCTET;
                let fresh431 = buffer;
                buffer = buffer.offset(1);
                *fresh431 = c_212;
                if c_212 as c_int == 0xffi32 {
                    let fresh432 = buffer;
                    buffer = buffer.offset(1);
                    *fresh432 = 0i32 as JOCTET
                }
                let mut c_213: JOCTET = 0;
                put_bits -= 8i32;
                c_213 = (put_buffer >> put_bits) as JOCTET;
                let fresh433 = buffer;
                buffer = buffer.offset(1);
                *fresh433 = c_213;
                if c_213 as c_int == 0xffi32 {
                    let fresh434 = buffer;
                    buffer = buffer.offset(1);
                    *fresh434 = 0i32 as JOCTET
                }
                let mut c_214: JOCTET = 0;
                put_bits -= 8i32;
                c_214 = (put_buffer >> put_bits) as JOCTET;
                let fresh435 = buffer;
                buffer = buffer.offset(1);
                *fresh435 = c_214;
                if c_214 as c_int == 0xffi32 {
                    let fresh436 = buffer;
                    buffer = buffer.offset(1);
                    *fresh436 = 0i32 as JOCTET
                }
                let mut c_215: JOCTET = 0;
                put_bits -= 8i32;
                c_215 = (put_buffer >> put_bits) as JOCTET;
                let fresh437 = buffer;
                buffer = buffer.offset(1);
                *fresh437 = c_215;
                if c_215 as c_int == 0xffi32 {
                    let fresh438 = buffer;
                    buffer = buffer.offset(1);
                    *fresh438 = 0i32 as JOCTET
                }
                let mut c_216: JOCTET = 0;
                put_bits -= 8i32;
                c_216 = (put_buffer >> put_bits) as JOCTET;
                let fresh439 = buffer;
                buffer = buffer.offset(1);
                *fresh439 = c_216;
                if c_216 as c_int == 0xffi32 {
                    let fresh440 = buffer;
                    buffer = buffer.offset(1);
                    *fresh440 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_217: JOCTET = 0;
            put_bits -= 8i32;
            c_217 = (put_buffer >> put_bits) as JOCTET;
            let fresh441 = buffer;
            buffer = buffer.offset(1);
            *fresh441 = c_217;
            if c_217 as c_int == 0xffi32 {
                let fresh442 = buffer;
                buffer = buffer.offset(1);
                *fresh442 = 0i32 as JOCTET
            }
            let mut c_218: JOCTET = 0;
            put_bits -= 8i32;
            c_218 = (put_buffer >> put_bits) as JOCTET;
            let fresh443 = buffer;
            buffer = buffer.offset(1);
            *fresh443 = c_218;
            if c_218 as c_int == 0xffi32 {
                let fresh444 = buffer;
                buffer = buffer.offset(1);
                *fresh444 = 0i32 as JOCTET
            }
            let mut c_219: JOCTET = 0;
            put_bits -= 8i32;
            c_219 = (put_buffer >> put_bits) as JOCTET;
            let fresh445 = buffer;
            buffer = buffer.offset(1);
            *fresh445 = c_219;
            if c_219 as c_int == 0xffi32 {
                let fresh446 = buffer;
                buffer = buffer.offset(1);
                *fresh446 = 0i32 as JOCTET
            }
            let mut c_220: JOCTET = 0;
            put_bits -= 8i32;
            c_220 = (put_buffer >> put_bits) as JOCTET;
            let fresh447 = buffer;
            buffer = buffer.offset(1);
            *fresh447 = c_220;
            if c_220 as c_int == 0xffi32 {
                let fresh448 = buffer;
                buffer = buffer.offset(1);
                *fresh448 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(41isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_221: JOCTET = 0;
                put_bits -= 8i32;
                c_221 = (put_buffer >> put_bits) as JOCTET;
                let fresh449 = buffer;
                buffer = buffer.offset(1);
                *fresh449 = c_221;
                if c_221 as c_int == 0xffi32 {
                    let fresh450 = buffer;
                    buffer = buffer.offset(1);
                    *fresh450 = 0i32 as JOCTET
                }
                let mut c_222: JOCTET = 0;
                put_bits -= 8i32;
                c_222 = (put_buffer >> put_bits) as JOCTET;
                let fresh451 = buffer;
                buffer = buffer.offset(1);
                *fresh451 = c_222;
                if c_222 as c_int == 0xffi32 {
                    let fresh452 = buffer;
                    buffer = buffer.offset(1);
                    *fresh452 = 0i32 as JOCTET
                }
                let mut c_223: JOCTET = 0;
                put_bits -= 8i32;
                c_223 = (put_buffer >> put_bits) as JOCTET;
                let fresh453 = buffer;
                buffer = buffer.offset(1);
                *fresh453 = c_223;
                if c_223 as c_int == 0xffi32 {
                    let fresh454 = buffer;
                    buffer = buffer.offset(1);
                    *fresh454 = 0i32 as JOCTET
                }
                let mut c_224: JOCTET = 0;
                put_bits -= 8i32;
                c_224 = (put_buffer >> put_bits) as JOCTET;
                let fresh455 = buffer;
                buffer = buffer.offset(1);
                *fresh455 = c_224;
                if c_224 as c_int == 0xffi32 {
                    let fresh456 = buffer;
                    buffer = buffer.offset(1);
                    *fresh456 = 0i32 as JOCTET
                }
                let mut c_225: JOCTET = 0;
                put_bits -= 8i32;
                c_225 = (put_buffer >> put_bits) as JOCTET;
                let fresh457 = buffer;
                buffer = buffer.offset(1);
                *fresh457 = c_225;
                if c_225 as c_int == 0xffi32 {
                    let fresh458 = buffer;
                    buffer = buffer.offset(1);
                    *fresh458 = 0i32 as JOCTET
                }
                let mut c_226: JOCTET = 0;
                put_bits -= 8i32;
                c_226 = (put_buffer >> put_bits) as JOCTET;
                let fresh459 = buffer;
                buffer = buffer.offset(1);
                *fresh459 = c_226;
                if c_226 as c_int == 0xffi32 {
                    let fresh460 = buffer;
                    buffer = buffer.offset(1);
                    *fresh460 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_227: JOCTET = 0;
            put_bits -= 8i32;
            c_227 = (put_buffer >> put_bits) as JOCTET;
            let fresh461 = buffer;
            buffer = buffer.offset(1);
            *fresh461 = c_227;
            if c_227 as c_int == 0xffi32 {
                let fresh462 = buffer;
                buffer = buffer.offset(1);
                *fresh462 = 0i32 as JOCTET
            }
            let mut c_228: JOCTET = 0;
            put_bits -= 8i32;
            c_228 = (put_buffer >> put_bits) as JOCTET;
            let fresh463 = buffer;
            buffer = buffer.offset(1);
            *fresh463 = c_228;
            if c_228 as c_int == 0xffi32 {
                let fresh464 = buffer;
                buffer = buffer.offset(1);
                *fresh464 = 0i32 as JOCTET
            }
            let mut c_229: JOCTET = 0;
            put_bits -= 8i32;
            c_229 = (put_buffer >> put_bits) as JOCTET;
            let fresh465 = buffer;
            buffer = buffer.offset(1);
            *fresh465 = c_229;
            if c_229 as c_int == 0xffi32 {
                let fresh466 = buffer;
                buffer = buffer.offset(1);
                *fresh466 = 0i32 as JOCTET
            }
            let mut c_230: JOCTET = 0;
            put_bits -= 8i32;
            c_230 = (put_buffer >> put_bits) as JOCTET;
            let fresh467 = buffer;
            buffer = buffer.offset(1);
            *fresh467 = c_230;
            if c_230 as c_int == 0xffi32 {
                let fresh468 = buffer;
                buffer = buffer.offset(1);
                *fresh468 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(34isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_231: JOCTET = 0;
                put_bits -= 8i32;
                c_231 = (put_buffer >> put_bits) as JOCTET;
                let fresh469 = buffer;
                buffer = buffer.offset(1);
                *fresh469 = c_231;
                if c_231 as c_int == 0xffi32 {
                    let fresh470 = buffer;
                    buffer = buffer.offset(1);
                    *fresh470 = 0i32 as JOCTET
                }
                let mut c_232: JOCTET = 0;
                put_bits -= 8i32;
                c_232 = (put_buffer >> put_bits) as JOCTET;
                let fresh471 = buffer;
                buffer = buffer.offset(1);
                *fresh471 = c_232;
                if c_232 as c_int == 0xffi32 {
                    let fresh472 = buffer;
                    buffer = buffer.offset(1);
                    *fresh472 = 0i32 as JOCTET
                }
                let mut c_233: JOCTET = 0;
                put_bits -= 8i32;
                c_233 = (put_buffer >> put_bits) as JOCTET;
                let fresh473 = buffer;
                buffer = buffer.offset(1);
                *fresh473 = c_233;
                if c_233 as c_int == 0xffi32 {
                    let fresh474 = buffer;
                    buffer = buffer.offset(1);
                    *fresh474 = 0i32 as JOCTET
                }
                let mut c_234: JOCTET = 0;
                put_bits -= 8i32;
                c_234 = (put_buffer >> put_bits) as JOCTET;
                let fresh475 = buffer;
                buffer = buffer.offset(1);
                *fresh475 = c_234;
                if c_234 as c_int == 0xffi32 {
                    let fresh476 = buffer;
                    buffer = buffer.offset(1);
                    *fresh476 = 0i32 as JOCTET
                }
                let mut c_235: JOCTET = 0;
                put_bits -= 8i32;
                c_235 = (put_buffer >> put_bits) as JOCTET;
                let fresh477 = buffer;
                buffer = buffer.offset(1);
                *fresh477 = c_235;
                if c_235 as c_int == 0xffi32 {
                    let fresh478 = buffer;
                    buffer = buffer.offset(1);
                    *fresh478 = 0i32 as JOCTET
                }
                let mut c_236: JOCTET = 0;
                put_bits -= 8i32;
                c_236 = (put_buffer >> put_bits) as JOCTET;
                let fresh479 = buffer;
                buffer = buffer.offset(1);
                *fresh479 = c_236;
                if c_236 as c_int == 0xffi32 {
                    let fresh480 = buffer;
                    buffer = buffer.offset(1);
                    *fresh480 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_237: JOCTET = 0;
            put_bits -= 8i32;
            c_237 = (put_buffer >> put_bits) as JOCTET;
            let fresh481 = buffer;
            buffer = buffer.offset(1);
            *fresh481 = c_237;
            if c_237 as c_int == 0xffi32 {
                let fresh482 = buffer;
                buffer = buffer.offset(1);
                *fresh482 = 0i32 as JOCTET
            }
            let mut c_238: JOCTET = 0;
            put_bits -= 8i32;
            c_238 = (put_buffer >> put_bits) as JOCTET;
            let fresh483 = buffer;
            buffer = buffer.offset(1);
            *fresh483 = c_238;
            if c_238 as c_int == 0xffi32 {
                let fresh484 = buffer;
                buffer = buffer.offset(1);
                *fresh484 = 0i32 as JOCTET
            }
            let mut c_239: JOCTET = 0;
            put_bits -= 8i32;
            c_239 = (put_buffer >> put_bits) as JOCTET;
            let fresh485 = buffer;
            buffer = buffer.offset(1);
            *fresh485 = c_239;
            if c_239 as c_int == 0xffi32 {
                let fresh486 = buffer;
                buffer = buffer.offset(1);
                *fresh486 = 0i32 as JOCTET
            }
            let mut c_240: JOCTET = 0;
            put_bits -= 8i32;
            c_240 = (put_buffer >> put_bits) as JOCTET;
            let fresh487 = buffer;
            buffer = buffer.offset(1);
            *fresh487 = c_240;
            if c_240 as c_int == 0xffi32 {
                let fresh488 = buffer;
                buffer = buffer.offset(1);
                *fresh488 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(27isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_241: JOCTET = 0;
                put_bits -= 8i32;
                c_241 = (put_buffer >> put_bits) as JOCTET;
                let fresh489 = buffer;
                buffer = buffer.offset(1);
                *fresh489 = c_241;
                if c_241 as c_int == 0xffi32 {
                    let fresh490 = buffer;
                    buffer = buffer.offset(1);
                    *fresh490 = 0i32 as JOCTET
                }
                let mut c_242: JOCTET = 0;
                put_bits -= 8i32;
                c_242 = (put_buffer >> put_bits) as JOCTET;
                let fresh491 = buffer;
                buffer = buffer.offset(1);
                *fresh491 = c_242;
                if c_242 as c_int == 0xffi32 {
                    let fresh492 = buffer;
                    buffer = buffer.offset(1);
                    *fresh492 = 0i32 as JOCTET
                }
                let mut c_243: JOCTET = 0;
                put_bits -= 8i32;
                c_243 = (put_buffer >> put_bits) as JOCTET;
                let fresh493 = buffer;
                buffer = buffer.offset(1);
                *fresh493 = c_243;
                if c_243 as c_int == 0xffi32 {
                    let fresh494 = buffer;
                    buffer = buffer.offset(1);
                    *fresh494 = 0i32 as JOCTET
                }
                let mut c_244: JOCTET = 0;
                put_bits -= 8i32;
                c_244 = (put_buffer >> put_bits) as JOCTET;
                let fresh495 = buffer;
                buffer = buffer.offset(1);
                *fresh495 = c_244;
                if c_244 as c_int == 0xffi32 {
                    let fresh496 = buffer;
                    buffer = buffer.offset(1);
                    *fresh496 = 0i32 as JOCTET
                }
                let mut c_245: JOCTET = 0;
                put_bits -= 8i32;
                c_245 = (put_buffer >> put_bits) as JOCTET;
                let fresh497 = buffer;
                buffer = buffer.offset(1);
                *fresh497 = c_245;
                if c_245 as c_int == 0xffi32 {
                    let fresh498 = buffer;
                    buffer = buffer.offset(1);
                    *fresh498 = 0i32 as JOCTET
                }
                let mut c_246: JOCTET = 0;
                put_bits -= 8i32;
                c_246 = (put_buffer >> put_bits) as JOCTET;
                let fresh499 = buffer;
                buffer = buffer.offset(1);
                *fresh499 = c_246;
                if c_246 as c_int == 0xffi32 {
                    let fresh500 = buffer;
                    buffer = buffer.offset(1);
                    *fresh500 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_247: JOCTET = 0;
            put_bits -= 8i32;
            c_247 = (put_buffer >> put_bits) as JOCTET;
            let fresh501 = buffer;
            buffer = buffer.offset(1);
            *fresh501 = c_247;
            if c_247 as c_int == 0xffi32 {
                let fresh502 = buffer;
                buffer = buffer.offset(1);
                *fresh502 = 0i32 as JOCTET
            }
            let mut c_248: JOCTET = 0;
            put_bits -= 8i32;
            c_248 = (put_buffer >> put_bits) as JOCTET;
            let fresh503 = buffer;
            buffer = buffer.offset(1);
            *fresh503 = c_248;
            if c_248 as c_int == 0xffi32 {
                let fresh504 = buffer;
                buffer = buffer.offset(1);
                *fresh504 = 0i32 as JOCTET
            }
            let mut c_249: JOCTET = 0;
            put_bits -= 8i32;
            c_249 = (put_buffer >> put_bits) as JOCTET;
            let fresh505 = buffer;
            buffer = buffer.offset(1);
            *fresh505 = c_249;
            if c_249 as c_int == 0xffi32 {
                let fresh506 = buffer;
                buffer = buffer.offset(1);
                *fresh506 = 0i32 as JOCTET
            }
            let mut c_250: JOCTET = 0;
            put_bits -= 8i32;
            c_250 = (put_buffer >> put_bits) as JOCTET;
            let fresh507 = buffer;
            buffer = buffer.offset(1);
            *fresh507 = c_250;
            if c_250 as c_int == 0xffi32 {
                let fresh508 = buffer;
                buffer = buffer.offset(1);
                *fresh508 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(20isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_251: JOCTET = 0;
                put_bits -= 8i32;
                c_251 = (put_buffer >> put_bits) as JOCTET;
                let fresh509 = buffer;
                buffer = buffer.offset(1);
                *fresh509 = c_251;
                if c_251 as c_int == 0xffi32 {
                    let fresh510 = buffer;
                    buffer = buffer.offset(1);
                    *fresh510 = 0i32 as JOCTET
                }
                let mut c_252: JOCTET = 0;
                put_bits -= 8i32;
                c_252 = (put_buffer >> put_bits) as JOCTET;
                let fresh511 = buffer;
                buffer = buffer.offset(1);
                *fresh511 = c_252;
                if c_252 as c_int == 0xffi32 {
                    let fresh512 = buffer;
                    buffer = buffer.offset(1);
                    *fresh512 = 0i32 as JOCTET
                }
                let mut c_253: JOCTET = 0;
                put_bits -= 8i32;
                c_253 = (put_buffer >> put_bits) as JOCTET;
                let fresh513 = buffer;
                buffer = buffer.offset(1);
                *fresh513 = c_253;
                if c_253 as c_int == 0xffi32 {
                    let fresh514 = buffer;
                    buffer = buffer.offset(1);
                    *fresh514 = 0i32 as JOCTET
                }
                let mut c_254: JOCTET = 0;
                put_bits -= 8i32;
                c_254 = (put_buffer >> put_bits) as JOCTET;
                let fresh515 = buffer;
                buffer = buffer.offset(1);
                *fresh515 = c_254;
                if c_254 as c_int == 0xffi32 {
                    let fresh516 = buffer;
                    buffer = buffer.offset(1);
                    *fresh516 = 0i32 as JOCTET
                }
                let mut c_255: JOCTET = 0;
                put_bits -= 8i32;
                c_255 = (put_buffer >> put_bits) as JOCTET;
                let fresh517 = buffer;
                buffer = buffer.offset(1);
                *fresh517 = c_255;
                if c_255 as c_int == 0xffi32 {
                    let fresh518 = buffer;
                    buffer = buffer.offset(1);
                    *fresh518 = 0i32 as JOCTET
                }
                let mut c_256: JOCTET = 0;
                put_bits -= 8i32;
                c_256 = (put_buffer >> put_bits) as JOCTET;
                let fresh519 = buffer;
                buffer = buffer.offset(1);
                *fresh519 = c_256;
                if c_256 as c_int == 0xffi32 {
                    let fresh520 = buffer;
                    buffer = buffer.offset(1);
                    *fresh520 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_257: JOCTET = 0;
            put_bits -= 8i32;
            c_257 = (put_buffer >> put_bits) as JOCTET;
            let fresh521 = buffer;
            buffer = buffer.offset(1);
            *fresh521 = c_257;
            if c_257 as c_int == 0xffi32 {
                let fresh522 = buffer;
                buffer = buffer.offset(1);
                *fresh522 = 0i32 as JOCTET
            }
            let mut c_258: JOCTET = 0;
            put_bits -= 8i32;
            c_258 = (put_buffer >> put_bits) as JOCTET;
            let fresh523 = buffer;
            buffer = buffer.offset(1);
            *fresh523 = c_258;
            if c_258 as c_int == 0xffi32 {
                let fresh524 = buffer;
                buffer = buffer.offset(1);
                *fresh524 = 0i32 as JOCTET
            }
            let mut c_259: JOCTET = 0;
            put_bits -= 8i32;
            c_259 = (put_buffer >> put_bits) as JOCTET;
            let fresh525 = buffer;
            buffer = buffer.offset(1);
            *fresh525 = c_259;
            if c_259 as c_int == 0xffi32 {
                let fresh526 = buffer;
                buffer = buffer.offset(1);
                *fresh526 = 0i32 as JOCTET
            }
            let mut c_260: JOCTET = 0;
            put_bits -= 8i32;
            c_260 = (put_buffer >> put_bits) as JOCTET;
            let fresh527 = buffer;
            buffer = buffer.offset(1);
            *fresh527 = c_260;
            if c_260 as c_int == 0xffi32 {
                let fresh528 = buffer;
                buffer = buffer.offset(1);
                *fresh528 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(13isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_261: JOCTET = 0;
                put_bits -= 8i32;
                c_261 = (put_buffer >> put_bits) as JOCTET;
                let fresh529 = buffer;
                buffer = buffer.offset(1);
                *fresh529 = c_261;
                if c_261 as c_int == 0xffi32 {
                    let fresh530 = buffer;
                    buffer = buffer.offset(1);
                    *fresh530 = 0i32 as JOCTET
                }
                let mut c_262: JOCTET = 0;
                put_bits -= 8i32;
                c_262 = (put_buffer >> put_bits) as JOCTET;
                let fresh531 = buffer;
                buffer = buffer.offset(1);
                *fresh531 = c_262;
                if c_262 as c_int == 0xffi32 {
                    let fresh532 = buffer;
                    buffer = buffer.offset(1);
                    *fresh532 = 0i32 as JOCTET
                }
                let mut c_263: JOCTET = 0;
                put_bits -= 8i32;
                c_263 = (put_buffer >> put_bits) as JOCTET;
                let fresh533 = buffer;
                buffer = buffer.offset(1);
                *fresh533 = c_263;
                if c_263 as c_int == 0xffi32 {
                    let fresh534 = buffer;
                    buffer = buffer.offset(1);
                    *fresh534 = 0i32 as JOCTET
                }
                let mut c_264: JOCTET = 0;
                put_bits -= 8i32;
                c_264 = (put_buffer >> put_bits) as JOCTET;
                let fresh535 = buffer;
                buffer = buffer.offset(1);
                *fresh535 = c_264;
                if c_264 as c_int == 0xffi32 {
                    let fresh536 = buffer;
                    buffer = buffer.offset(1);
                    *fresh536 = 0i32 as JOCTET
                }
                let mut c_265: JOCTET = 0;
                put_bits -= 8i32;
                c_265 = (put_buffer >> put_bits) as JOCTET;
                let fresh537 = buffer;
                buffer = buffer.offset(1);
                *fresh537 = c_265;
                if c_265 as c_int == 0xffi32 {
                    let fresh538 = buffer;
                    buffer = buffer.offset(1);
                    *fresh538 = 0i32 as JOCTET
                }
                let mut c_266: JOCTET = 0;
                put_bits -= 8i32;
                c_266 = (put_buffer >> put_bits) as JOCTET;
                let fresh539 = buffer;
                buffer = buffer.offset(1);
                *fresh539 = c_266;
                if c_266 as c_int == 0xffi32 {
                    let fresh540 = buffer;
                    buffer = buffer.offset(1);
                    *fresh540 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_267: JOCTET = 0;
            put_bits -= 8i32;
            c_267 = (put_buffer >> put_bits) as JOCTET;
            let fresh541 = buffer;
            buffer = buffer.offset(1);
            *fresh541 = c_267;
            if c_267 as c_int == 0xffi32 {
                let fresh542 = buffer;
                buffer = buffer.offset(1);
                *fresh542 = 0i32 as JOCTET
            }
            let mut c_268: JOCTET = 0;
            put_bits -= 8i32;
            c_268 = (put_buffer >> put_bits) as JOCTET;
            let fresh543 = buffer;
            buffer = buffer.offset(1);
            *fresh543 = c_268;
            if c_268 as c_int == 0xffi32 {
                let fresh544 = buffer;
                buffer = buffer.offset(1);
                *fresh544 = 0i32 as JOCTET
            }
            let mut c_269: JOCTET = 0;
            put_bits -= 8i32;
            c_269 = (put_buffer >> put_bits) as JOCTET;
            let fresh545 = buffer;
            buffer = buffer.offset(1);
            *fresh545 = c_269;
            if c_269 as c_int == 0xffi32 {
                let fresh546 = buffer;
                buffer = buffer.offset(1);
                *fresh546 = 0i32 as JOCTET
            }
            let mut c_270: JOCTET = 0;
            put_bits -= 8i32;
            c_270 = (put_buffer >> put_bits) as JOCTET;
            let fresh547 = buffer;
            buffer = buffer.offset(1);
            *fresh547 = c_270;
            if c_270 as c_int == 0xffi32 {
                let fresh548 = buffer;
                buffer = buffer.offset(1);
                *fresh548 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(6isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_271: JOCTET = 0;
                put_bits -= 8i32;
                c_271 = (put_buffer >> put_bits) as JOCTET;
                let fresh549 = buffer;
                buffer = buffer.offset(1);
                *fresh549 = c_271;
                if c_271 as c_int == 0xffi32 {
                    let fresh550 = buffer;
                    buffer = buffer.offset(1);
                    *fresh550 = 0i32 as JOCTET
                }
                let mut c_272: JOCTET = 0;
                put_bits -= 8i32;
                c_272 = (put_buffer >> put_bits) as JOCTET;
                let fresh551 = buffer;
                buffer = buffer.offset(1);
                *fresh551 = c_272;
                if c_272 as c_int == 0xffi32 {
                    let fresh552 = buffer;
                    buffer = buffer.offset(1);
                    *fresh552 = 0i32 as JOCTET
                }
                let mut c_273: JOCTET = 0;
                put_bits -= 8i32;
                c_273 = (put_buffer >> put_bits) as JOCTET;
                let fresh553 = buffer;
                buffer = buffer.offset(1);
                *fresh553 = c_273;
                if c_273 as c_int == 0xffi32 {
                    let fresh554 = buffer;
                    buffer = buffer.offset(1);
                    *fresh554 = 0i32 as JOCTET
                }
                let mut c_274: JOCTET = 0;
                put_bits -= 8i32;
                c_274 = (put_buffer >> put_bits) as JOCTET;
                let fresh555 = buffer;
                buffer = buffer.offset(1);
                *fresh555 = c_274;
                if c_274 as c_int == 0xffi32 {
                    let fresh556 = buffer;
                    buffer = buffer.offset(1);
                    *fresh556 = 0i32 as JOCTET
                }
                let mut c_275: JOCTET = 0;
                put_bits -= 8i32;
                c_275 = (put_buffer >> put_bits) as JOCTET;
                let fresh557 = buffer;
                buffer = buffer.offset(1);
                *fresh557 = c_275;
                if c_275 as c_int == 0xffi32 {
                    let fresh558 = buffer;
                    buffer = buffer.offset(1);
                    *fresh558 = 0i32 as JOCTET
                }
                let mut c_276: JOCTET = 0;
                put_bits -= 8i32;
                c_276 = (put_buffer >> put_bits) as JOCTET;
                let fresh559 = buffer;
                buffer = buffer.offset(1);
                *fresh559 = c_276;
                if c_276 as c_int == 0xffi32 {
                    let fresh560 = buffer;
                    buffer = buffer.offset(1);
                    *fresh560 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_277: JOCTET = 0;
            put_bits -= 8i32;
            c_277 = (put_buffer >> put_bits) as JOCTET;
            let fresh561 = buffer;
            buffer = buffer.offset(1);
            *fresh561 = c_277;
            if c_277 as c_int == 0xffi32 {
                let fresh562 = buffer;
                buffer = buffer.offset(1);
                *fresh562 = 0i32 as JOCTET
            }
            let mut c_278: JOCTET = 0;
            put_bits -= 8i32;
            c_278 = (put_buffer >> put_bits) as JOCTET;
            let fresh563 = buffer;
            buffer = buffer.offset(1);
            *fresh563 = c_278;
            if c_278 as c_int == 0xffi32 {
                let fresh564 = buffer;
                buffer = buffer.offset(1);
                *fresh564 = 0i32 as JOCTET
            }
            let mut c_279: JOCTET = 0;
            put_bits -= 8i32;
            c_279 = (put_buffer >> put_bits) as JOCTET;
            let fresh565 = buffer;
            buffer = buffer.offset(1);
            *fresh565 = c_279;
            if c_279 as c_int == 0xffi32 {
                let fresh566 = buffer;
                buffer = buffer.offset(1);
                *fresh566 = 0i32 as JOCTET
            }
            let mut c_280: JOCTET = 0;
            put_bits -= 8i32;
            c_280 = (put_buffer >> put_bits) as JOCTET;
            let fresh567 = buffer;
            buffer = buffer.offset(1);
            *fresh567 = c_280;
            if c_280 as c_int == 0xffi32 {
                let fresh568 = buffer;
                buffer = buffer.offset(1);
                *fresh568 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(7isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_281: JOCTET = 0;
                put_bits -= 8i32;
                c_281 = (put_buffer >> put_bits) as JOCTET;
                let fresh569 = buffer;
                buffer = buffer.offset(1);
                *fresh569 = c_281;
                if c_281 as c_int == 0xffi32 {
                    let fresh570 = buffer;
                    buffer = buffer.offset(1);
                    *fresh570 = 0i32 as JOCTET
                }
                let mut c_282: JOCTET = 0;
                put_bits -= 8i32;
                c_282 = (put_buffer >> put_bits) as JOCTET;
                let fresh571 = buffer;
                buffer = buffer.offset(1);
                *fresh571 = c_282;
                if c_282 as c_int == 0xffi32 {
                    let fresh572 = buffer;
                    buffer = buffer.offset(1);
                    *fresh572 = 0i32 as JOCTET
                }
                let mut c_283: JOCTET = 0;
                put_bits -= 8i32;
                c_283 = (put_buffer >> put_bits) as JOCTET;
                let fresh573 = buffer;
                buffer = buffer.offset(1);
                *fresh573 = c_283;
                if c_283 as c_int == 0xffi32 {
                    let fresh574 = buffer;
                    buffer = buffer.offset(1);
                    *fresh574 = 0i32 as JOCTET
                }
                let mut c_284: JOCTET = 0;
                put_bits -= 8i32;
                c_284 = (put_buffer >> put_bits) as JOCTET;
                let fresh575 = buffer;
                buffer = buffer.offset(1);
                *fresh575 = c_284;
                if c_284 as c_int == 0xffi32 {
                    let fresh576 = buffer;
                    buffer = buffer.offset(1);
                    *fresh576 = 0i32 as JOCTET
                }
                let mut c_285: JOCTET = 0;
                put_bits -= 8i32;
                c_285 = (put_buffer >> put_bits) as JOCTET;
                let fresh577 = buffer;
                buffer = buffer.offset(1);
                *fresh577 = c_285;
                if c_285 as c_int == 0xffi32 {
                    let fresh578 = buffer;
                    buffer = buffer.offset(1);
                    *fresh578 = 0i32 as JOCTET
                }
                let mut c_286: JOCTET = 0;
                put_bits -= 8i32;
                c_286 = (put_buffer >> put_bits) as JOCTET;
                let fresh579 = buffer;
                buffer = buffer.offset(1);
                *fresh579 = c_286;
                if c_286 as c_int == 0xffi32 {
                    let fresh580 = buffer;
                    buffer = buffer.offset(1);
                    *fresh580 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_287: JOCTET = 0;
            put_bits -= 8i32;
            c_287 = (put_buffer >> put_bits) as JOCTET;
            let fresh581 = buffer;
            buffer = buffer.offset(1);
            *fresh581 = c_287;
            if c_287 as c_int == 0xffi32 {
                let fresh582 = buffer;
                buffer = buffer.offset(1);
                *fresh582 = 0i32 as JOCTET
            }
            let mut c_288: JOCTET = 0;
            put_bits -= 8i32;
            c_288 = (put_buffer >> put_bits) as JOCTET;
            let fresh583 = buffer;
            buffer = buffer.offset(1);
            *fresh583 = c_288;
            if c_288 as c_int == 0xffi32 {
                let fresh584 = buffer;
                buffer = buffer.offset(1);
                *fresh584 = 0i32 as JOCTET
            }
            let mut c_289: JOCTET = 0;
            put_bits -= 8i32;
            c_289 = (put_buffer >> put_bits) as JOCTET;
            let fresh585 = buffer;
            buffer = buffer.offset(1);
            *fresh585 = c_289;
            if c_289 as c_int == 0xffi32 {
                let fresh586 = buffer;
                buffer = buffer.offset(1);
                *fresh586 = 0i32 as JOCTET
            }
            let mut c_290: JOCTET = 0;
            put_bits -= 8i32;
            c_290 = (put_buffer >> put_bits) as JOCTET;
            let fresh587 = buffer;
            buffer = buffer.offset(1);
            *fresh587 = c_290;
            if c_290 as c_int == 0xffi32 {
                let fresh588 = buffer;
                buffer = buffer.offset(1);
                *fresh588 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(14isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_291: JOCTET = 0;
                put_bits -= 8i32;
                c_291 = (put_buffer >> put_bits) as JOCTET;
                let fresh589 = buffer;
                buffer = buffer.offset(1);
                *fresh589 = c_291;
                if c_291 as c_int == 0xffi32 {
                    let fresh590 = buffer;
                    buffer = buffer.offset(1);
                    *fresh590 = 0i32 as JOCTET
                }
                let mut c_292: JOCTET = 0;
                put_bits -= 8i32;
                c_292 = (put_buffer >> put_bits) as JOCTET;
                let fresh591 = buffer;
                buffer = buffer.offset(1);
                *fresh591 = c_292;
                if c_292 as c_int == 0xffi32 {
                    let fresh592 = buffer;
                    buffer = buffer.offset(1);
                    *fresh592 = 0i32 as JOCTET
                }
                let mut c_293: JOCTET = 0;
                put_bits -= 8i32;
                c_293 = (put_buffer >> put_bits) as JOCTET;
                let fresh593 = buffer;
                buffer = buffer.offset(1);
                *fresh593 = c_293;
                if c_293 as c_int == 0xffi32 {
                    let fresh594 = buffer;
                    buffer = buffer.offset(1);
                    *fresh594 = 0i32 as JOCTET
                }
                let mut c_294: JOCTET = 0;
                put_bits -= 8i32;
                c_294 = (put_buffer >> put_bits) as JOCTET;
                let fresh595 = buffer;
                buffer = buffer.offset(1);
                *fresh595 = c_294;
                if c_294 as c_int == 0xffi32 {
                    let fresh596 = buffer;
                    buffer = buffer.offset(1);
                    *fresh596 = 0i32 as JOCTET
                }
                let mut c_295: JOCTET = 0;
                put_bits -= 8i32;
                c_295 = (put_buffer >> put_bits) as JOCTET;
                let fresh597 = buffer;
                buffer = buffer.offset(1);
                *fresh597 = c_295;
                if c_295 as c_int == 0xffi32 {
                    let fresh598 = buffer;
                    buffer = buffer.offset(1);
                    *fresh598 = 0i32 as JOCTET
                }
                let mut c_296: JOCTET = 0;
                put_bits -= 8i32;
                c_296 = (put_buffer >> put_bits) as JOCTET;
                let fresh599 = buffer;
                buffer = buffer.offset(1);
                *fresh599 = c_296;
                if c_296 as c_int == 0xffi32 {
                    let fresh600 = buffer;
                    buffer = buffer.offset(1);
                    *fresh600 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_297: JOCTET = 0;
            put_bits -= 8i32;
            c_297 = (put_buffer >> put_bits) as JOCTET;
            let fresh601 = buffer;
            buffer = buffer.offset(1);
            *fresh601 = c_297;
            if c_297 as c_int == 0xffi32 {
                let fresh602 = buffer;
                buffer = buffer.offset(1);
                *fresh602 = 0i32 as JOCTET
            }
            let mut c_298: JOCTET = 0;
            put_bits -= 8i32;
            c_298 = (put_buffer >> put_bits) as JOCTET;
            let fresh603 = buffer;
            buffer = buffer.offset(1);
            *fresh603 = c_298;
            if c_298 as c_int == 0xffi32 {
                let fresh604 = buffer;
                buffer = buffer.offset(1);
                *fresh604 = 0i32 as JOCTET
            }
            let mut c_299: JOCTET = 0;
            put_bits -= 8i32;
            c_299 = (put_buffer >> put_bits) as JOCTET;
            let fresh605 = buffer;
            buffer = buffer.offset(1);
            *fresh605 = c_299;
            if c_299 as c_int == 0xffi32 {
                let fresh606 = buffer;
                buffer = buffer.offset(1);
                *fresh606 = 0i32 as JOCTET
            }
            let mut c_300: JOCTET = 0;
            put_bits -= 8i32;
            c_300 = (put_buffer >> put_bits) as JOCTET;
            let fresh607 = buffer;
            buffer = buffer.offset(1);
            *fresh607 = c_300;
            if c_300 as c_int == 0xffi32 {
                let fresh608 = buffer;
                buffer = buffer.offset(1);
                *fresh608 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(21isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_301: JOCTET = 0;
                put_bits -= 8i32;
                c_301 = (put_buffer >> put_bits) as JOCTET;
                let fresh609 = buffer;
                buffer = buffer.offset(1);
                *fresh609 = c_301;
                if c_301 as c_int == 0xffi32 {
                    let fresh610 = buffer;
                    buffer = buffer.offset(1);
                    *fresh610 = 0i32 as JOCTET
                }
                let mut c_302: JOCTET = 0;
                put_bits -= 8i32;
                c_302 = (put_buffer >> put_bits) as JOCTET;
                let fresh611 = buffer;
                buffer = buffer.offset(1);
                *fresh611 = c_302;
                if c_302 as c_int == 0xffi32 {
                    let fresh612 = buffer;
                    buffer = buffer.offset(1);
                    *fresh612 = 0i32 as JOCTET
                }
                let mut c_303: JOCTET = 0;
                put_bits -= 8i32;
                c_303 = (put_buffer >> put_bits) as JOCTET;
                let fresh613 = buffer;
                buffer = buffer.offset(1);
                *fresh613 = c_303;
                if c_303 as c_int == 0xffi32 {
                    let fresh614 = buffer;
                    buffer = buffer.offset(1);
                    *fresh614 = 0i32 as JOCTET
                }
                let mut c_304: JOCTET = 0;
                put_bits -= 8i32;
                c_304 = (put_buffer >> put_bits) as JOCTET;
                let fresh615 = buffer;
                buffer = buffer.offset(1);
                *fresh615 = c_304;
                if c_304 as c_int == 0xffi32 {
                    let fresh616 = buffer;
                    buffer = buffer.offset(1);
                    *fresh616 = 0i32 as JOCTET
                }
                let mut c_305: JOCTET = 0;
                put_bits -= 8i32;
                c_305 = (put_buffer >> put_bits) as JOCTET;
                let fresh617 = buffer;
                buffer = buffer.offset(1);
                *fresh617 = c_305;
                if c_305 as c_int == 0xffi32 {
                    let fresh618 = buffer;
                    buffer = buffer.offset(1);
                    *fresh618 = 0i32 as JOCTET
                }
                let mut c_306: JOCTET = 0;
                put_bits -= 8i32;
                c_306 = (put_buffer >> put_bits) as JOCTET;
                let fresh619 = buffer;
                buffer = buffer.offset(1);
                *fresh619 = c_306;
                if c_306 as c_int == 0xffi32 {
                    let fresh620 = buffer;
                    buffer = buffer.offset(1);
                    *fresh620 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_307: JOCTET = 0;
            put_bits -= 8i32;
            c_307 = (put_buffer >> put_bits) as JOCTET;
            let fresh621 = buffer;
            buffer = buffer.offset(1);
            *fresh621 = c_307;
            if c_307 as c_int == 0xffi32 {
                let fresh622 = buffer;
                buffer = buffer.offset(1);
                *fresh622 = 0i32 as JOCTET
            }
            let mut c_308: JOCTET = 0;
            put_bits -= 8i32;
            c_308 = (put_buffer >> put_bits) as JOCTET;
            let fresh623 = buffer;
            buffer = buffer.offset(1);
            *fresh623 = c_308;
            if c_308 as c_int == 0xffi32 {
                let fresh624 = buffer;
                buffer = buffer.offset(1);
                *fresh624 = 0i32 as JOCTET
            }
            let mut c_309: JOCTET = 0;
            put_bits -= 8i32;
            c_309 = (put_buffer >> put_bits) as JOCTET;
            let fresh625 = buffer;
            buffer = buffer.offset(1);
            *fresh625 = c_309;
            if c_309 as c_int == 0xffi32 {
                let fresh626 = buffer;
                buffer = buffer.offset(1);
                *fresh626 = 0i32 as JOCTET
            }
            let mut c_310: JOCTET = 0;
            put_bits -= 8i32;
            c_310 = (put_buffer >> put_bits) as JOCTET;
            let fresh627 = buffer;
            buffer = buffer.offset(1);
            *fresh627 = c_310;
            if c_310 as c_int == 0xffi32 {
                let fresh628 = buffer;
                buffer = buffer.offset(1);
                *fresh628 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(28isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_311: JOCTET = 0;
                put_bits -= 8i32;
                c_311 = (put_buffer >> put_bits) as JOCTET;
                let fresh629 = buffer;
                buffer = buffer.offset(1);
                *fresh629 = c_311;
                if c_311 as c_int == 0xffi32 {
                    let fresh630 = buffer;
                    buffer = buffer.offset(1);
                    *fresh630 = 0i32 as JOCTET
                }
                let mut c_312: JOCTET = 0;
                put_bits -= 8i32;
                c_312 = (put_buffer >> put_bits) as JOCTET;
                let fresh631 = buffer;
                buffer = buffer.offset(1);
                *fresh631 = c_312;
                if c_312 as c_int == 0xffi32 {
                    let fresh632 = buffer;
                    buffer = buffer.offset(1);
                    *fresh632 = 0i32 as JOCTET
                }
                let mut c_313: JOCTET = 0;
                put_bits -= 8i32;
                c_313 = (put_buffer >> put_bits) as JOCTET;
                let fresh633 = buffer;
                buffer = buffer.offset(1);
                *fresh633 = c_313;
                if c_313 as c_int == 0xffi32 {
                    let fresh634 = buffer;
                    buffer = buffer.offset(1);
                    *fresh634 = 0i32 as JOCTET
                }
                let mut c_314: JOCTET = 0;
                put_bits -= 8i32;
                c_314 = (put_buffer >> put_bits) as JOCTET;
                let fresh635 = buffer;
                buffer = buffer.offset(1);
                *fresh635 = c_314;
                if c_314 as c_int == 0xffi32 {
                    let fresh636 = buffer;
                    buffer = buffer.offset(1);
                    *fresh636 = 0i32 as JOCTET
                }
                let mut c_315: JOCTET = 0;
                put_bits -= 8i32;
                c_315 = (put_buffer >> put_bits) as JOCTET;
                let fresh637 = buffer;
                buffer = buffer.offset(1);
                *fresh637 = c_315;
                if c_315 as c_int == 0xffi32 {
                    let fresh638 = buffer;
                    buffer = buffer.offset(1);
                    *fresh638 = 0i32 as JOCTET
                }
                let mut c_316: JOCTET = 0;
                put_bits -= 8i32;
                c_316 = (put_buffer >> put_bits) as JOCTET;
                let fresh639 = buffer;
                buffer = buffer.offset(1);
                *fresh639 = c_316;
                if c_316 as c_int == 0xffi32 {
                    let fresh640 = buffer;
                    buffer = buffer.offset(1);
                    *fresh640 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_317: JOCTET = 0;
            put_bits -= 8i32;
            c_317 = (put_buffer >> put_bits) as JOCTET;
            let fresh641 = buffer;
            buffer = buffer.offset(1);
            *fresh641 = c_317;
            if c_317 as c_int == 0xffi32 {
                let fresh642 = buffer;
                buffer = buffer.offset(1);
                *fresh642 = 0i32 as JOCTET
            }
            let mut c_318: JOCTET = 0;
            put_bits -= 8i32;
            c_318 = (put_buffer >> put_bits) as JOCTET;
            let fresh643 = buffer;
            buffer = buffer.offset(1);
            *fresh643 = c_318;
            if c_318 as c_int == 0xffi32 {
                let fresh644 = buffer;
                buffer = buffer.offset(1);
                *fresh644 = 0i32 as JOCTET
            }
            let mut c_319: JOCTET = 0;
            put_bits -= 8i32;
            c_319 = (put_buffer >> put_bits) as JOCTET;
            let fresh645 = buffer;
            buffer = buffer.offset(1);
            *fresh645 = c_319;
            if c_319 as c_int == 0xffi32 {
                let fresh646 = buffer;
                buffer = buffer.offset(1);
                *fresh646 = 0i32 as JOCTET
            }
            let mut c_320: JOCTET = 0;
            put_bits -= 8i32;
            c_320 = (put_buffer >> put_bits) as JOCTET;
            let fresh647 = buffer;
            buffer = buffer.offset(1);
            *fresh647 = c_320;
            if c_320 as c_int == 0xffi32 {
                let fresh648 = buffer;
                buffer = buffer.offset(1);
                *fresh648 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(35isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_321: JOCTET = 0;
                put_bits -= 8i32;
                c_321 = (put_buffer >> put_bits) as JOCTET;
                let fresh649 = buffer;
                buffer = buffer.offset(1);
                *fresh649 = c_321;
                if c_321 as c_int == 0xffi32 {
                    let fresh650 = buffer;
                    buffer = buffer.offset(1);
                    *fresh650 = 0i32 as JOCTET
                }
                let mut c_322: JOCTET = 0;
                put_bits -= 8i32;
                c_322 = (put_buffer >> put_bits) as JOCTET;
                let fresh651 = buffer;
                buffer = buffer.offset(1);
                *fresh651 = c_322;
                if c_322 as c_int == 0xffi32 {
                    let fresh652 = buffer;
                    buffer = buffer.offset(1);
                    *fresh652 = 0i32 as JOCTET
                }
                let mut c_323: JOCTET = 0;
                put_bits -= 8i32;
                c_323 = (put_buffer >> put_bits) as JOCTET;
                let fresh653 = buffer;
                buffer = buffer.offset(1);
                *fresh653 = c_323;
                if c_323 as c_int == 0xffi32 {
                    let fresh654 = buffer;
                    buffer = buffer.offset(1);
                    *fresh654 = 0i32 as JOCTET
                }
                let mut c_324: JOCTET = 0;
                put_bits -= 8i32;
                c_324 = (put_buffer >> put_bits) as JOCTET;
                let fresh655 = buffer;
                buffer = buffer.offset(1);
                *fresh655 = c_324;
                if c_324 as c_int == 0xffi32 {
                    let fresh656 = buffer;
                    buffer = buffer.offset(1);
                    *fresh656 = 0i32 as JOCTET
                }
                let mut c_325: JOCTET = 0;
                put_bits -= 8i32;
                c_325 = (put_buffer >> put_bits) as JOCTET;
                let fresh657 = buffer;
                buffer = buffer.offset(1);
                *fresh657 = c_325;
                if c_325 as c_int == 0xffi32 {
                    let fresh658 = buffer;
                    buffer = buffer.offset(1);
                    *fresh658 = 0i32 as JOCTET
                }
                let mut c_326: JOCTET = 0;
                put_bits -= 8i32;
                c_326 = (put_buffer >> put_bits) as JOCTET;
                let fresh659 = buffer;
                buffer = buffer.offset(1);
                *fresh659 = c_326;
                if c_326 as c_int == 0xffi32 {
                    let fresh660 = buffer;
                    buffer = buffer.offset(1);
                    *fresh660 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_327: JOCTET = 0;
            put_bits -= 8i32;
            c_327 = (put_buffer >> put_bits) as JOCTET;
            let fresh661 = buffer;
            buffer = buffer.offset(1);
            *fresh661 = c_327;
            if c_327 as c_int == 0xffi32 {
                let fresh662 = buffer;
                buffer = buffer.offset(1);
                *fresh662 = 0i32 as JOCTET
            }
            let mut c_328: JOCTET = 0;
            put_bits -= 8i32;
            c_328 = (put_buffer >> put_bits) as JOCTET;
            let fresh663 = buffer;
            buffer = buffer.offset(1);
            *fresh663 = c_328;
            if c_328 as c_int == 0xffi32 {
                let fresh664 = buffer;
                buffer = buffer.offset(1);
                *fresh664 = 0i32 as JOCTET
            }
            let mut c_329: JOCTET = 0;
            put_bits -= 8i32;
            c_329 = (put_buffer >> put_bits) as JOCTET;
            let fresh665 = buffer;
            buffer = buffer.offset(1);
            *fresh665 = c_329;
            if c_329 as c_int == 0xffi32 {
                let fresh666 = buffer;
                buffer = buffer.offset(1);
                *fresh666 = 0i32 as JOCTET
            }
            let mut c_330: JOCTET = 0;
            put_bits -= 8i32;
            c_330 = (put_buffer >> put_bits) as JOCTET;
            let fresh667 = buffer;
            buffer = buffer.offset(1);
            *fresh667 = c_330;
            if c_330 as c_int == 0xffi32 {
                let fresh668 = buffer;
                buffer = buffer.offset(1);
                *fresh668 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(42isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_331: JOCTET = 0;
                put_bits -= 8i32;
                c_331 = (put_buffer >> put_bits) as JOCTET;
                let fresh669 = buffer;
                buffer = buffer.offset(1);
                *fresh669 = c_331;
                if c_331 as c_int == 0xffi32 {
                    let fresh670 = buffer;
                    buffer = buffer.offset(1);
                    *fresh670 = 0i32 as JOCTET
                }
                let mut c_332: JOCTET = 0;
                put_bits -= 8i32;
                c_332 = (put_buffer >> put_bits) as JOCTET;
                let fresh671 = buffer;
                buffer = buffer.offset(1);
                *fresh671 = c_332;
                if c_332 as c_int == 0xffi32 {
                    let fresh672 = buffer;
                    buffer = buffer.offset(1);
                    *fresh672 = 0i32 as JOCTET
                }
                let mut c_333: JOCTET = 0;
                put_bits -= 8i32;
                c_333 = (put_buffer >> put_bits) as JOCTET;
                let fresh673 = buffer;
                buffer = buffer.offset(1);
                *fresh673 = c_333;
                if c_333 as c_int == 0xffi32 {
                    let fresh674 = buffer;
                    buffer = buffer.offset(1);
                    *fresh674 = 0i32 as JOCTET
                }
                let mut c_334: JOCTET = 0;
                put_bits -= 8i32;
                c_334 = (put_buffer >> put_bits) as JOCTET;
                let fresh675 = buffer;
                buffer = buffer.offset(1);
                *fresh675 = c_334;
                if c_334 as c_int == 0xffi32 {
                    let fresh676 = buffer;
                    buffer = buffer.offset(1);
                    *fresh676 = 0i32 as JOCTET
                }
                let mut c_335: JOCTET = 0;
                put_bits -= 8i32;
                c_335 = (put_buffer >> put_bits) as JOCTET;
                let fresh677 = buffer;
                buffer = buffer.offset(1);
                *fresh677 = c_335;
                if c_335 as c_int == 0xffi32 {
                    let fresh678 = buffer;
                    buffer = buffer.offset(1);
                    *fresh678 = 0i32 as JOCTET
                }
                let mut c_336: JOCTET = 0;
                put_bits -= 8i32;
                c_336 = (put_buffer >> put_bits) as JOCTET;
                let fresh679 = buffer;
                buffer = buffer.offset(1);
                *fresh679 = c_336;
                if c_336 as c_int == 0xffi32 {
                    let fresh680 = buffer;
                    buffer = buffer.offset(1);
                    *fresh680 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_337: JOCTET = 0;
            put_bits -= 8i32;
            c_337 = (put_buffer >> put_bits) as JOCTET;
            let fresh681 = buffer;
            buffer = buffer.offset(1);
            *fresh681 = c_337;
            if c_337 as c_int == 0xffi32 {
                let fresh682 = buffer;
                buffer = buffer.offset(1);
                *fresh682 = 0i32 as JOCTET
            }
            let mut c_338: JOCTET = 0;
            put_bits -= 8i32;
            c_338 = (put_buffer >> put_bits) as JOCTET;
            let fresh683 = buffer;
            buffer = buffer.offset(1);
            *fresh683 = c_338;
            if c_338 as c_int == 0xffi32 {
                let fresh684 = buffer;
                buffer = buffer.offset(1);
                *fresh684 = 0i32 as JOCTET
            }
            let mut c_339: JOCTET = 0;
            put_bits -= 8i32;
            c_339 = (put_buffer >> put_bits) as JOCTET;
            let fresh685 = buffer;
            buffer = buffer.offset(1);
            *fresh685 = c_339;
            if c_339 as c_int == 0xffi32 {
                let fresh686 = buffer;
                buffer = buffer.offset(1);
                *fresh686 = 0i32 as JOCTET
            }
            let mut c_340: JOCTET = 0;
            put_bits -= 8i32;
            c_340 = (put_buffer >> put_bits) as JOCTET;
            let fresh687 = buffer;
            buffer = buffer.offset(1);
            *fresh687 = c_340;
            if c_340 as c_int == 0xffi32 {
                let fresh688 = buffer;
                buffer = buffer.offset(1);
                *fresh688 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(49isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_341: JOCTET = 0;
                put_bits -= 8i32;
                c_341 = (put_buffer >> put_bits) as JOCTET;
                let fresh689 = buffer;
                buffer = buffer.offset(1);
                *fresh689 = c_341;
                if c_341 as c_int == 0xffi32 {
                    let fresh690 = buffer;
                    buffer = buffer.offset(1);
                    *fresh690 = 0i32 as JOCTET
                }
                let mut c_342: JOCTET = 0;
                put_bits -= 8i32;
                c_342 = (put_buffer >> put_bits) as JOCTET;
                let fresh691 = buffer;
                buffer = buffer.offset(1);
                *fresh691 = c_342;
                if c_342 as c_int == 0xffi32 {
                    let fresh692 = buffer;
                    buffer = buffer.offset(1);
                    *fresh692 = 0i32 as JOCTET
                }
                let mut c_343: JOCTET = 0;
                put_bits -= 8i32;
                c_343 = (put_buffer >> put_bits) as JOCTET;
                let fresh693 = buffer;
                buffer = buffer.offset(1);
                *fresh693 = c_343;
                if c_343 as c_int == 0xffi32 {
                    let fresh694 = buffer;
                    buffer = buffer.offset(1);
                    *fresh694 = 0i32 as JOCTET
                }
                let mut c_344: JOCTET = 0;
                put_bits -= 8i32;
                c_344 = (put_buffer >> put_bits) as JOCTET;
                let fresh695 = buffer;
                buffer = buffer.offset(1);
                *fresh695 = c_344;
                if c_344 as c_int == 0xffi32 {
                    let fresh696 = buffer;
                    buffer = buffer.offset(1);
                    *fresh696 = 0i32 as JOCTET
                }
                let mut c_345: JOCTET = 0;
                put_bits -= 8i32;
                c_345 = (put_buffer >> put_bits) as JOCTET;
                let fresh697 = buffer;
                buffer = buffer.offset(1);
                *fresh697 = c_345;
                if c_345 as c_int == 0xffi32 {
                    let fresh698 = buffer;
                    buffer = buffer.offset(1);
                    *fresh698 = 0i32 as JOCTET
                }
                let mut c_346: JOCTET = 0;
                put_bits -= 8i32;
                c_346 = (put_buffer >> put_bits) as JOCTET;
                let fresh699 = buffer;
                buffer = buffer.offset(1);
                *fresh699 = c_346;
                if c_346 as c_int == 0xffi32 {
                    let fresh700 = buffer;
                    buffer = buffer.offset(1);
                    *fresh700 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_347: JOCTET = 0;
            put_bits -= 8i32;
            c_347 = (put_buffer >> put_bits) as JOCTET;
            let fresh701 = buffer;
            buffer = buffer.offset(1);
            *fresh701 = c_347;
            if c_347 as c_int == 0xffi32 {
                let fresh702 = buffer;
                buffer = buffer.offset(1);
                *fresh702 = 0i32 as JOCTET
            }
            let mut c_348: JOCTET = 0;
            put_bits -= 8i32;
            c_348 = (put_buffer >> put_bits) as JOCTET;
            let fresh703 = buffer;
            buffer = buffer.offset(1);
            *fresh703 = c_348;
            if c_348 as c_int == 0xffi32 {
                let fresh704 = buffer;
                buffer = buffer.offset(1);
                *fresh704 = 0i32 as JOCTET
            }
            let mut c_349: JOCTET = 0;
            put_bits -= 8i32;
            c_349 = (put_buffer >> put_bits) as JOCTET;
            let fresh705 = buffer;
            buffer = buffer.offset(1);
            *fresh705 = c_349;
            if c_349 as c_int == 0xffi32 {
                let fresh706 = buffer;
                buffer = buffer.offset(1);
                *fresh706 = 0i32 as JOCTET
            }
            let mut c_350: JOCTET = 0;
            put_bits -= 8i32;
            c_350 = (put_buffer >> put_bits) as JOCTET;
            let fresh707 = buffer;
            buffer = buffer.offset(1);
            *fresh707 = c_350;
            if c_350 as c_int == 0xffi32 {
                let fresh708 = buffer;
                buffer = buffer.offset(1);
                *fresh708 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(56isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_351: JOCTET = 0;
                put_bits -= 8i32;
                c_351 = (put_buffer >> put_bits) as JOCTET;
                let fresh709 = buffer;
                buffer = buffer.offset(1);
                *fresh709 = c_351;
                if c_351 as c_int == 0xffi32 {
                    let fresh710 = buffer;
                    buffer = buffer.offset(1);
                    *fresh710 = 0i32 as JOCTET
                }
                let mut c_352: JOCTET = 0;
                put_bits -= 8i32;
                c_352 = (put_buffer >> put_bits) as JOCTET;
                let fresh711 = buffer;
                buffer = buffer.offset(1);
                *fresh711 = c_352;
                if c_352 as c_int == 0xffi32 {
                    let fresh712 = buffer;
                    buffer = buffer.offset(1);
                    *fresh712 = 0i32 as JOCTET
                }
                let mut c_353: JOCTET = 0;
                put_bits -= 8i32;
                c_353 = (put_buffer >> put_bits) as JOCTET;
                let fresh713 = buffer;
                buffer = buffer.offset(1);
                *fresh713 = c_353;
                if c_353 as c_int == 0xffi32 {
                    let fresh714 = buffer;
                    buffer = buffer.offset(1);
                    *fresh714 = 0i32 as JOCTET
                }
                let mut c_354: JOCTET = 0;
                put_bits -= 8i32;
                c_354 = (put_buffer >> put_bits) as JOCTET;
                let fresh715 = buffer;
                buffer = buffer.offset(1);
                *fresh715 = c_354;
                if c_354 as c_int == 0xffi32 {
                    let fresh716 = buffer;
                    buffer = buffer.offset(1);
                    *fresh716 = 0i32 as JOCTET
                }
                let mut c_355: JOCTET = 0;
                put_bits -= 8i32;
                c_355 = (put_buffer >> put_bits) as JOCTET;
                let fresh717 = buffer;
                buffer = buffer.offset(1);
                *fresh717 = c_355;
                if c_355 as c_int == 0xffi32 {
                    let fresh718 = buffer;
                    buffer = buffer.offset(1);
                    *fresh718 = 0i32 as JOCTET
                }
                let mut c_356: JOCTET = 0;
                put_bits -= 8i32;
                c_356 = (put_buffer >> put_bits) as JOCTET;
                let fresh719 = buffer;
                buffer = buffer.offset(1);
                *fresh719 = c_356;
                if c_356 as c_int == 0xffi32 {
                    let fresh720 = buffer;
                    buffer = buffer.offset(1);
                    *fresh720 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_357: JOCTET = 0;
            put_bits -= 8i32;
            c_357 = (put_buffer >> put_bits) as JOCTET;
            let fresh721 = buffer;
            buffer = buffer.offset(1);
            *fresh721 = c_357;
            if c_357 as c_int == 0xffi32 {
                let fresh722 = buffer;
                buffer = buffer.offset(1);
                *fresh722 = 0i32 as JOCTET
            }
            let mut c_358: JOCTET = 0;
            put_bits -= 8i32;
            c_358 = (put_buffer >> put_bits) as JOCTET;
            let fresh723 = buffer;
            buffer = buffer.offset(1);
            *fresh723 = c_358;
            if c_358 as c_int == 0xffi32 {
                let fresh724 = buffer;
                buffer = buffer.offset(1);
                *fresh724 = 0i32 as JOCTET
            }
            let mut c_359: JOCTET = 0;
            put_bits -= 8i32;
            c_359 = (put_buffer >> put_bits) as JOCTET;
            let fresh725 = buffer;
            buffer = buffer.offset(1);
            *fresh725 = c_359;
            if c_359 as c_int == 0xffi32 {
                let fresh726 = buffer;
                buffer = buffer.offset(1);
                *fresh726 = 0i32 as JOCTET
            }
            let mut c_360: JOCTET = 0;
            put_bits -= 8i32;
            c_360 = (put_buffer >> put_bits) as JOCTET;
            let fresh727 = buffer;
            buffer = buffer.offset(1);
            *fresh727 = c_360;
            if c_360 as c_int == 0xffi32 {
                let fresh728 = buffer;
                buffer = buffer.offset(1);
                *fresh728 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(57isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_361: JOCTET = 0;
                put_bits -= 8i32;
                c_361 = (put_buffer >> put_bits) as JOCTET;
                let fresh729 = buffer;
                buffer = buffer.offset(1);
                *fresh729 = c_361;
                if c_361 as c_int == 0xffi32 {
                    let fresh730 = buffer;
                    buffer = buffer.offset(1);
                    *fresh730 = 0i32 as JOCTET
                }
                let mut c_362: JOCTET = 0;
                put_bits -= 8i32;
                c_362 = (put_buffer >> put_bits) as JOCTET;
                let fresh731 = buffer;
                buffer = buffer.offset(1);
                *fresh731 = c_362;
                if c_362 as c_int == 0xffi32 {
                    let fresh732 = buffer;
                    buffer = buffer.offset(1);
                    *fresh732 = 0i32 as JOCTET
                }
                let mut c_363: JOCTET = 0;
                put_bits -= 8i32;
                c_363 = (put_buffer >> put_bits) as JOCTET;
                let fresh733 = buffer;
                buffer = buffer.offset(1);
                *fresh733 = c_363;
                if c_363 as c_int == 0xffi32 {
                    let fresh734 = buffer;
                    buffer = buffer.offset(1);
                    *fresh734 = 0i32 as JOCTET
                }
                let mut c_364: JOCTET = 0;
                put_bits -= 8i32;
                c_364 = (put_buffer >> put_bits) as JOCTET;
                let fresh735 = buffer;
                buffer = buffer.offset(1);
                *fresh735 = c_364;
                if c_364 as c_int == 0xffi32 {
                    let fresh736 = buffer;
                    buffer = buffer.offset(1);
                    *fresh736 = 0i32 as JOCTET
                }
                let mut c_365: JOCTET = 0;
                put_bits -= 8i32;
                c_365 = (put_buffer >> put_bits) as JOCTET;
                let fresh737 = buffer;
                buffer = buffer.offset(1);
                *fresh737 = c_365;
                if c_365 as c_int == 0xffi32 {
                    let fresh738 = buffer;
                    buffer = buffer.offset(1);
                    *fresh738 = 0i32 as JOCTET
                }
                let mut c_366: JOCTET = 0;
                put_bits -= 8i32;
                c_366 = (put_buffer >> put_bits) as JOCTET;
                let fresh739 = buffer;
                buffer = buffer.offset(1);
                *fresh739 = c_366;
                if c_366 as c_int == 0xffi32 {
                    let fresh740 = buffer;
                    buffer = buffer.offset(1);
                    *fresh740 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_367: JOCTET = 0;
            put_bits -= 8i32;
            c_367 = (put_buffer >> put_bits) as JOCTET;
            let fresh741 = buffer;
            buffer = buffer.offset(1);
            *fresh741 = c_367;
            if c_367 as c_int == 0xffi32 {
                let fresh742 = buffer;
                buffer = buffer.offset(1);
                *fresh742 = 0i32 as JOCTET
            }
            let mut c_368: JOCTET = 0;
            put_bits -= 8i32;
            c_368 = (put_buffer >> put_bits) as JOCTET;
            let fresh743 = buffer;
            buffer = buffer.offset(1);
            *fresh743 = c_368;
            if c_368 as c_int == 0xffi32 {
                let fresh744 = buffer;
                buffer = buffer.offset(1);
                *fresh744 = 0i32 as JOCTET
            }
            let mut c_369: JOCTET = 0;
            put_bits -= 8i32;
            c_369 = (put_buffer >> put_bits) as JOCTET;
            let fresh745 = buffer;
            buffer = buffer.offset(1);
            *fresh745 = c_369;
            if c_369 as c_int == 0xffi32 {
                let fresh746 = buffer;
                buffer = buffer.offset(1);
                *fresh746 = 0i32 as JOCTET
            }
            let mut c_370: JOCTET = 0;
            put_bits -= 8i32;
            c_370 = (put_buffer >> put_bits) as JOCTET;
            let fresh747 = buffer;
            buffer = buffer.offset(1);
            *fresh747 = c_370;
            if c_370 as c_int == 0xffi32 {
                let fresh748 = buffer;
                buffer = buffer.offset(1);
                *fresh748 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(50isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_371: JOCTET = 0;
                put_bits -= 8i32;
                c_371 = (put_buffer >> put_bits) as JOCTET;
                let fresh749 = buffer;
                buffer = buffer.offset(1);
                *fresh749 = c_371;
                if c_371 as c_int == 0xffi32 {
                    let fresh750 = buffer;
                    buffer = buffer.offset(1);
                    *fresh750 = 0i32 as JOCTET
                }
                let mut c_372: JOCTET = 0;
                put_bits -= 8i32;
                c_372 = (put_buffer >> put_bits) as JOCTET;
                let fresh751 = buffer;
                buffer = buffer.offset(1);
                *fresh751 = c_372;
                if c_372 as c_int == 0xffi32 {
                    let fresh752 = buffer;
                    buffer = buffer.offset(1);
                    *fresh752 = 0i32 as JOCTET
                }
                let mut c_373: JOCTET = 0;
                put_bits -= 8i32;
                c_373 = (put_buffer >> put_bits) as JOCTET;
                let fresh753 = buffer;
                buffer = buffer.offset(1);
                *fresh753 = c_373;
                if c_373 as c_int == 0xffi32 {
                    let fresh754 = buffer;
                    buffer = buffer.offset(1);
                    *fresh754 = 0i32 as JOCTET
                }
                let mut c_374: JOCTET = 0;
                put_bits -= 8i32;
                c_374 = (put_buffer >> put_bits) as JOCTET;
                let fresh755 = buffer;
                buffer = buffer.offset(1);
                *fresh755 = c_374;
                if c_374 as c_int == 0xffi32 {
                    let fresh756 = buffer;
                    buffer = buffer.offset(1);
                    *fresh756 = 0i32 as JOCTET
                }
                let mut c_375: JOCTET = 0;
                put_bits -= 8i32;
                c_375 = (put_buffer >> put_bits) as JOCTET;
                let fresh757 = buffer;
                buffer = buffer.offset(1);
                *fresh757 = c_375;
                if c_375 as c_int == 0xffi32 {
                    let fresh758 = buffer;
                    buffer = buffer.offset(1);
                    *fresh758 = 0i32 as JOCTET
                }
                let mut c_376: JOCTET = 0;
                put_bits -= 8i32;
                c_376 = (put_buffer >> put_bits) as JOCTET;
                let fresh759 = buffer;
                buffer = buffer.offset(1);
                *fresh759 = c_376;
                if c_376 as c_int == 0xffi32 {
                    let fresh760 = buffer;
                    buffer = buffer.offset(1);
                    *fresh760 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_377: JOCTET = 0;
            put_bits -= 8i32;
            c_377 = (put_buffer >> put_bits) as JOCTET;
            let fresh761 = buffer;
            buffer = buffer.offset(1);
            *fresh761 = c_377;
            if c_377 as c_int == 0xffi32 {
                let fresh762 = buffer;
                buffer = buffer.offset(1);
                *fresh762 = 0i32 as JOCTET
            }
            let mut c_378: JOCTET = 0;
            put_bits -= 8i32;
            c_378 = (put_buffer >> put_bits) as JOCTET;
            let fresh763 = buffer;
            buffer = buffer.offset(1);
            *fresh763 = c_378;
            if c_378 as c_int == 0xffi32 {
                let fresh764 = buffer;
                buffer = buffer.offset(1);
                *fresh764 = 0i32 as JOCTET
            }
            let mut c_379: JOCTET = 0;
            put_bits -= 8i32;
            c_379 = (put_buffer >> put_bits) as JOCTET;
            let fresh765 = buffer;
            buffer = buffer.offset(1);
            *fresh765 = c_379;
            if c_379 as c_int == 0xffi32 {
                let fresh766 = buffer;
                buffer = buffer.offset(1);
                *fresh766 = 0i32 as JOCTET
            }
            let mut c_380: JOCTET = 0;
            put_bits -= 8i32;
            c_380 = (put_buffer >> put_bits) as JOCTET;
            let fresh767 = buffer;
            buffer = buffer.offset(1);
            *fresh767 = c_380;
            if c_380 as c_int == 0xffi32 {
                let fresh768 = buffer;
                buffer = buffer.offset(1);
                *fresh768 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(43isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_381: JOCTET = 0;
                put_bits -= 8i32;
                c_381 = (put_buffer >> put_bits) as JOCTET;
                let fresh769 = buffer;
                buffer = buffer.offset(1);
                *fresh769 = c_381;
                if c_381 as c_int == 0xffi32 {
                    let fresh770 = buffer;
                    buffer = buffer.offset(1);
                    *fresh770 = 0i32 as JOCTET
                }
                let mut c_382: JOCTET = 0;
                put_bits -= 8i32;
                c_382 = (put_buffer >> put_bits) as JOCTET;
                let fresh771 = buffer;
                buffer = buffer.offset(1);
                *fresh771 = c_382;
                if c_382 as c_int == 0xffi32 {
                    let fresh772 = buffer;
                    buffer = buffer.offset(1);
                    *fresh772 = 0i32 as JOCTET
                }
                let mut c_383: JOCTET = 0;
                put_bits -= 8i32;
                c_383 = (put_buffer >> put_bits) as JOCTET;
                let fresh773 = buffer;
                buffer = buffer.offset(1);
                *fresh773 = c_383;
                if c_383 as c_int == 0xffi32 {
                    let fresh774 = buffer;
                    buffer = buffer.offset(1);
                    *fresh774 = 0i32 as JOCTET
                }
                let mut c_384: JOCTET = 0;
                put_bits -= 8i32;
                c_384 = (put_buffer >> put_bits) as JOCTET;
                let fresh775 = buffer;
                buffer = buffer.offset(1);
                *fresh775 = c_384;
                if c_384 as c_int == 0xffi32 {
                    let fresh776 = buffer;
                    buffer = buffer.offset(1);
                    *fresh776 = 0i32 as JOCTET
                }
                let mut c_385: JOCTET = 0;
                put_bits -= 8i32;
                c_385 = (put_buffer >> put_bits) as JOCTET;
                let fresh777 = buffer;
                buffer = buffer.offset(1);
                *fresh777 = c_385;
                if c_385 as c_int == 0xffi32 {
                    let fresh778 = buffer;
                    buffer = buffer.offset(1);
                    *fresh778 = 0i32 as JOCTET
                }
                let mut c_386: JOCTET = 0;
                put_bits -= 8i32;
                c_386 = (put_buffer >> put_bits) as JOCTET;
                let fresh779 = buffer;
                buffer = buffer.offset(1);
                *fresh779 = c_386;
                if c_386 as c_int == 0xffi32 {
                    let fresh780 = buffer;
                    buffer = buffer.offset(1);
                    *fresh780 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_387: JOCTET = 0;
            put_bits -= 8i32;
            c_387 = (put_buffer >> put_bits) as JOCTET;
            let fresh781 = buffer;
            buffer = buffer.offset(1);
            *fresh781 = c_387;
            if c_387 as c_int == 0xffi32 {
                let fresh782 = buffer;
                buffer = buffer.offset(1);
                *fresh782 = 0i32 as JOCTET
            }
            let mut c_388: JOCTET = 0;
            put_bits -= 8i32;
            c_388 = (put_buffer >> put_bits) as JOCTET;
            let fresh783 = buffer;
            buffer = buffer.offset(1);
            *fresh783 = c_388;
            if c_388 as c_int == 0xffi32 {
                let fresh784 = buffer;
                buffer = buffer.offset(1);
                *fresh784 = 0i32 as JOCTET
            }
            let mut c_389: JOCTET = 0;
            put_bits -= 8i32;
            c_389 = (put_buffer >> put_bits) as JOCTET;
            let fresh785 = buffer;
            buffer = buffer.offset(1);
            *fresh785 = c_389;
            if c_389 as c_int == 0xffi32 {
                let fresh786 = buffer;
                buffer = buffer.offset(1);
                *fresh786 = 0i32 as JOCTET
            }
            let mut c_390: JOCTET = 0;
            put_bits -= 8i32;
            c_390 = (put_buffer >> put_bits) as JOCTET;
            let fresh787 = buffer;
            buffer = buffer.offset(1);
            *fresh787 = c_390;
            if c_390 as c_int == 0xffi32 {
                let fresh788 = buffer;
                buffer = buffer.offset(1);
                *fresh788 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(36isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_391: JOCTET = 0;
                put_bits -= 8i32;
                c_391 = (put_buffer >> put_bits) as JOCTET;
                let fresh789 = buffer;
                buffer = buffer.offset(1);
                *fresh789 = c_391;
                if c_391 as c_int == 0xffi32 {
                    let fresh790 = buffer;
                    buffer = buffer.offset(1);
                    *fresh790 = 0i32 as JOCTET
                }
                let mut c_392: JOCTET = 0;
                put_bits -= 8i32;
                c_392 = (put_buffer >> put_bits) as JOCTET;
                let fresh791 = buffer;
                buffer = buffer.offset(1);
                *fresh791 = c_392;
                if c_392 as c_int == 0xffi32 {
                    let fresh792 = buffer;
                    buffer = buffer.offset(1);
                    *fresh792 = 0i32 as JOCTET
                }
                let mut c_393: JOCTET = 0;
                put_bits -= 8i32;
                c_393 = (put_buffer >> put_bits) as JOCTET;
                let fresh793 = buffer;
                buffer = buffer.offset(1);
                *fresh793 = c_393;
                if c_393 as c_int == 0xffi32 {
                    let fresh794 = buffer;
                    buffer = buffer.offset(1);
                    *fresh794 = 0i32 as JOCTET
                }
                let mut c_394: JOCTET = 0;
                put_bits -= 8i32;
                c_394 = (put_buffer >> put_bits) as JOCTET;
                let fresh795 = buffer;
                buffer = buffer.offset(1);
                *fresh795 = c_394;
                if c_394 as c_int == 0xffi32 {
                    let fresh796 = buffer;
                    buffer = buffer.offset(1);
                    *fresh796 = 0i32 as JOCTET
                }
                let mut c_395: JOCTET = 0;
                put_bits -= 8i32;
                c_395 = (put_buffer >> put_bits) as JOCTET;
                let fresh797 = buffer;
                buffer = buffer.offset(1);
                *fresh797 = c_395;
                if c_395 as c_int == 0xffi32 {
                    let fresh798 = buffer;
                    buffer = buffer.offset(1);
                    *fresh798 = 0i32 as JOCTET
                }
                let mut c_396: JOCTET = 0;
                put_bits -= 8i32;
                c_396 = (put_buffer >> put_bits) as JOCTET;
                let fresh799 = buffer;
                buffer = buffer.offset(1);
                *fresh799 = c_396;
                if c_396 as c_int == 0xffi32 {
                    let fresh800 = buffer;
                    buffer = buffer.offset(1);
                    *fresh800 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_397: JOCTET = 0;
            put_bits -= 8i32;
            c_397 = (put_buffer >> put_bits) as JOCTET;
            let fresh801 = buffer;
            buffer = buffer.offset(1);
            *fresh801 = c_397;
            if c_397 as c_int == 0xffi32 {
                let fresh802 = buffer;
                buffer = buffer.offset(1);
                *fresh802 = 0i32 as JOCTET
            }
            let mut c_398: JOCTET = 0;
            put_bits -= 8i32;
            c_398 = (put_buffer >> put_bits) as JOCTET;
            let fresh803 = buffer;
            buffer = buffer.offset(1);
            *fresh803 = c_398;
            if c_398 as c_int == 0xffi32 {
                let fresh804 = buffer;
                buffer = buffer.offset(1);
                *fresh804 = 0i32 as JOCTET
            }
            let mut c_399: JOCTET = 0;
            put_bits -= 8i32;
            c_399 = (put_buffer >> put_bits) as JOCTET;
            let fresh805 = buffer;
            buffer = buffer.offset(1);
            *fresh805 = c_399;
            if c_399 as c_int == 0xffi32 {
                let fresh806 = buffer;
                buffer = buffer.offset(1);
                *fresh806 = 0i32 as JOCTET
            }
            let mut c_400: JOCTET = 0;
            put_bits -= 8i32;
            c_400 = (put_buffer >> put_bits) as JOCTET;
            let fresh807 = buffer;
            buffer = buffer.offset(1);
            *fresh807 = c_400;
            if c_400 as c_int == 0xffi32 {
                let fresh808 = buffer;
                buffer = buffer.offset(1);
                *fresh808 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(29isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_401: JOCTET = 0;
                put_bits -= 8i32;
                c_401 = (put_buffer >> put_bits) as JOCTET;
                let fresh809 = buffer;
                buffer = buffer.offset(1);
                *fresh809 = c_401;
                if c_401 as c_int == 0xffi32 {
                    let fresh810 = buffer;
                    buffer = buffer.offset(1);
                    *fresh810 = 0i32 as JOCTET
                }
                let mut c_402: JOCTET = 0;
                put_bits -= 8i32;
                c_402 = (put_buffer >> put_bits) as JOCTET;
                let fresh811 = buffer;
                buffer = buffer.offset(1);
                *fresh811 = c_402;
                if c_402 as c_int == 0xffi32 {
                    let fresh812 = buffer;
                    buffer = buffer.offset(1);
                    *fresh812 = 0i32 as JOCTET
                }
                let mut c_403: JOCTET = 0;
                put_bits -= 8i32;
                c_403 = (put_buffer >> put_bits) as JOCTET;
                let fresh813 = buffer;
                buffer = buffer.offset(1);
                *fresh813 = c_403;
                if c_403 as c_int == 0xffi32 {
                    let fresh814 = buffer;
                    buffer = buffer.offset(1);
                    *fresh814 = 0i32 as JOCTET
                }
                let mut c_404: JOCTET = 0;
                put_bits -= 8i32;
                c_404 = (put_buffer >> put_bits) as JOCTET;
                let fresh815 = buffer;
                buffer = buffer.offset(1);
                *fresh815 = c_404;
                if c_404 as c_int == 0xffi32 {
                    let fresh816 = buffer;
                    buffer = buffer.offset(1);
                    *fresh816 = 0i32 as JOCTET
                }
                let mut c_405: JOCTET = 0;
                put_bits -= 8i32;
                c_405 = (put_buffer >> put_bits) as JOCTET;
                let fresh817 = buffer;
                buffer = buffer.offset(1);
                *fresh817 = c_405;
                if c_405 as c_int == 0xffi32 {
                    let fresh818 = buffer;
                    buffer = buffer.offset(1);
                    *fresh818 = 0i32 as JOCTET
                }
                let mut c_406: JOCTET = 0;
                put_bits -= 8i32;
                c_406 = (put_buffer >> put_bits) as JOCTET;
                let fresh819 = buffer;
                buffer = buffer.offset(1);
                *fresh819 = c_406;
                if c_406 as c_int == 0xffi32 {
                    let fresh820 = buffer;
                    buffer = buffer.offset(1);
                    *fresh820 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_407: JOCTET = 0;
            put_bits -= 8i32;
            c_407 = (put_buffer >> put_bits) as JOCTET;
            let fresh821 = buffer;
            buffer = buffer.offset(1);
            *fresh821 = c_407;
            if c_407 as c_int == 0xffi32 {
                let fresh822 = buffer;
                buffer = buffer.offset(1);
                *fresh822 = 0i32 as JOCTET
            }
            let mut c_408: JOCTET = 0;
            put_bits -= 8i32;
            c_408 = (put_buffer >> put_bits) as JOCTET;
            let fresh823 = buffer;
            buffer = buffer.offset(1);
            *fresh823 = c_408;
            if c_408 as c_int == 0xffi32 {
                let fresh824 = buffer;
                buffer = buffer.offset(1);
                *fresh824 = 0i32 as JOCTET
            }
            let mut c_409: JOCTET = 0;
            put_bits -= 8i32;
            c_409 = (put_buffer >> put_bits) as JOCTET;
            let fresh825 = buffer;
            buffer = buffer.offset(1);
            *fresh825 = c_409;
            if c_409 as c_int == 0xffi32 {
                let fresh826 = buffer;
                buffer = buffer.offset(1);
                *fresh826 = 0i32 as JOCTET
            }
            let mut c_410: JOCTET = 0;
            put_bits -= 8i32;
            c_410 = (put_buffer >> put_bits) as JOCTET;
            let fresh827 = buffer;
            buffer = buffer.offset(1);
            *fresh827 = c_410;
            if c_410 as c_int == 0xffi32 {
                let fresh828 = buffer;
                buffer = buffer.offset(1);
                *fresh828 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(22isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_411: JOCTET = 0;
                put_bits -= 8i32;
                c_411 = (put_buffer >> put_bits) as JOCTET;
                let fresh829 = buffer;
                buffer = buffer.offset(1);
                *fresh829 = c_411;
                if c_411 as c_int == 0xffi32 {
                    let fresh830 = buffer;
                    buffer = buffer.offset(1);
                    *fresh830 = 0i32 as JOCTET
                }
                let mut c_412: JOCTET = 0;
                put_bits -= 8i32;
                c_412 = (put_buffer >> put_bits) as JOCTET;
                let fresh831 = buffer;
                buffer = buffer.offset(1);
                *fresh831 = c_412;
                if c_412 as c_int == 0xffi32 {
                    let fresh832 = buffer;
                    buffer = buffer.offset(1);
                    *fresh832 = 0i32 as JOCTET
                }
                let mut c_413: JOCTET = 0;
                put_bits -= 8i32;
                c_413 = (put_buffer >> put_bits) as JOCTET;
                let fresh833 = buffer;
                buffer = buffer.offset(1);
                *fresh833 = c_413;
                if c_413 as c_int == 0xffi32 {
                    let fresh834 = buffer;
                    buffer = buffer.offset(1);
                    *fresh834 = 0i32 as JOCTET
                }
                let mut c_414: JOCTET = 0;
                put_bits -= 8i32;
                c_414 = (put_buffer >> put_bits) as JOCTET;
                let fresh835 = buffer;
                buffer = buffer.offset(1);
                *fresh835 = c_414;
                if c_414 as c_int == 0xffi32 {
                    let fresh836 = buffer;
                    buffer = buffer.offset(1);
                    *fresh836 = 0i32 as JOCTET
                }
                let mut c_415: JOCTET = 0;
                put_bits -= 8i32;
                c_415 = (put_buffer >> put_bits) as JOCTET;
                let fresh837 = buffer;
                buffer = buffer.offset(1);
                *fresh837 = c_415;
                if c_415 as c_int == 0xffi32 {
                    let fresh838 = buffer;
                    buffer = buffer.offset(1);
                    *fresh838 = 0i32 as JOCTET
                }
                let mut c_416: JOCTET = 0;
                put_bits -= 8i32;
                c_416 = (put_buffer >> put_bits) as JOCTET;
                let fresh839 = buffer;
                buffer = buffer.offset(1);
                *fresh839 = c_416;
                if c_416 as c_int == 0xffi32 {
                    let fresh840 = buffer;
                    buffer = buffer.offset(1);
                    *fresh840 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_417: JOCTET = 0;
            put_bits -= 8i32;
            c_417 = (put_buffer >> put_bits) as JOCTET;
            let fresh841 = buffer;
            buffer = buffer.offset(1);
            *fresh841 = c_417;
            if c_417 as c_int == 0xffi32 {
                let fresh842 = buffer;
                buffer = buffer.offset(1);
                *fresh842 = 0i32 as JOCTET
            }
            let mut c_418: JOCTET = 0;
            put_bits -= 8i32;
            c_418 = (put_buffer >> put_bits) as JOCTET;
            let fresh843 = buffer;
            buffer = buffer.offset(1);
            *fresh843 = c_418;
            if c_418 as c_int == 0xffi32 {
                let fresh844 = buffer;
                buffer = buffer.offset(1);
                *fresh844 = 0i32 as JOCTET
            }
            let mut c_419: JOCTET = 0;
            put_bits -= 8i32;
            c_419 = (put_buffer >> put_bits) as JOCTET;
            let fresh845 = buffer;
            buffer = buffer.offset(1);
            *fresh845 = c_419;
            if c_419 as c_int == 0xffi32 {
                let fresh846 = buffer;
                buffer = buffer.offset(1);
                *fresh846 = 0i32 as JOCTET
            }
            let mut c_420: JOCTET = 0;
            put_bits -= 8i32;
            c_420 = (put_buffer >> put_bits) as JOCTET;
            let fresh847 = buffer;
            buffer = buffer.offset(1);
            *fresh847 = c_420;
            if c_420 as c_int == 0xffi32 {
                let fresh848 = buffer;
                buffer = buffer.offset(1);
                *fresh848 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(15isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_421: JOCTET = 0;
                put_bits -= 8i32;
                c_421 = (put_buffer >> put_bits) as JOCTET;
                let fresh849 = buffer;
                buffer = buffer.offset(1);
                *fresh849 = c_421;
                if c_421 as c_int == 0xffi32 {
                    let fresh850 = buffer;
                    buffer = buffer.offset(1);
                    *fresh850 = 0i32 as JOCTET
                }
                let mut c_422: JOCTET = 0;
                put_bits -= 8i32;
                c_422 = (put_buffer >> put_bits) as JOCTET;
                let fresh851 = buffer;
                buffer = buffer.offset(1);
                *fresh851 = c_422;
                if c_422 as c_int == 0xffi32 {
                    let fresh852 = buffer;
                    buffer = buffer.offset(1);
                    *fresh852 = 0i32 as JOCTET
                }
                let mut c_423: JOCTET = 0;
                put_bits -= 8i32;
                c_423 = (put_buffer >> put_bits) as JOCTET;
                let fresh853 = buffer;
                buffer = buffer.offset(1);
                *fresh853 = c_423;
                if c_423 as c_int == 0xffi32 {
                    let fresh854 = buffer;
                    buffer = buffer.offset(1);
                    *fresh854 = 0i32 as JOCTET
                }
                let mut c_424: JOCTET = 0;
                put_bits -= 8i32;
                c_424 = (put_buffer >> put_bits) as JOCTET;
                let fresh855 = buffer;
                buffer = buffer.offset(1);
                *fresh855 = c_424;
                if c_424 as c_int == 0xffi32 {
                    let fresh856 = buffer;
                    buffer = buffer.offset(1);
                    *fresh856 = 0i32 as JOCTET
                }
                let mut c_425: JOCTET = 0;
                put_bits -= 8i32;
                c_425 = (put_buffer >> put_bits) as JOCTET;
                let fresh857 = buffer;
                buffer = buffer.offset(1);
                *fresh857 = c_425;
                if c_425 as c_int == 0xffi32 {
                    let fresh858 = buffer;
                    buffer = buffer.offset(1);
                    *fresh858 = 0i32 as JOCTET
                }
                let mut c_426: JOCTET = 0;
                put_bits -= 8i32;
                c_426 = (put_buffer >> put_bits) as JOCTET;
                let fresh859 = buffer;
                buffer = buffer.offset(1);
                *fresh859 = c_426;
                if c_426 as c_int == 0xffi32 {
                    let fresh860 = buffer;
                    buffer = buffer.offset(1);
                    *fresh860 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_427: JOCTET = 0;
            put_bits -= 8i32;
            c_427 = (put_buffer >> put_bits) as JOCTET;
            let fresh861 = buffer;
            buffer = buffer.offset(1);
            *fresh861 = c_427;
            if c_427 as c_int == 0xffi32 {
                let fresh862 = buffer;
                buffer = buffer.offset(1);
                *fresh862 = 0i32 as JOCTET
            }
            let mut c_428: JOCTET = 0;
            put_bits -= 8i32;
            c_428 = (put_buffer >> put_bits) as JOCTET;
            let fresh863 = buffer;
            buffer = buffer.offset(1);
            *fresh863 = c_428;
            if c_428 as c_int == 0xffi32 {
                let fresh864 = buffer;
                buffer = buffer.offset(1);
                *fresh864 = 0i32 as JOCTET
            }
            let mut c_429: JOCTET = 0;
            put_bits -= 8i32;
            c_429 = (put_buffer >> put_bits) as JOCTET;
            let fresh865 = buffer;
            buffer = buffer.offset(1);
            *fresh865 = c_429;
            if c_429 as c_int == 0xffi32 {
                let fresh866 = buffer;
                buffer = buffer.offset(1);
                *fresh866 = 0i32 as JOCTET
            }
            let mut c_430: JOCTET = 0;
            put_bits -= 8i32;
            c_430 = (put_buffer >> put_bits) as JOCTET;
            let fresh867 = buffer;
            buffer = buffer.offset(1);
            *fresh867 = c_430;
            if c_430 as c_int == 0xffi32 {
                let fresh868 = buffer;
                buffer = buffer.offset(1);
                *fresh868 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(23isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_431: JOCTET = 0;
                put_bits -= 8i32;
                c_431 = (put_buffer >> put_bits) as JOCTET;
                let fresh869 = buffer;
                buffer = buffer.offset(1);
                *fresh869 = c_431;
                if c_431 as c_int == 0xffi32 {
                    let fresh870 = buffer;
                    buffer = buffer.offset(1);
                    *fresh870 = 0i32 as JOCTET
                }
                let mut c_432: JOCTET = 0;
                put_bits -= 8i32;
                c_432 = (put_buffer >> put_bits) as JOCTET;
                let fresh871 = buffer;
                buffer = buffer.offset(1);
                *fresh871 = c_432;
                if c_432 as c_int == 0xffi32 {
                    let fresh872 = buffer;
                    buffer = buffer.offset(1);
                    *fresh872 = 0i32 as JOCTET
                }
                let mut c_433: JOCTET = 0;
                put_bits -= 8i32;
                c_433 = (put_buffer >> put_bits) as JOCTET;
                let fresh873 = buffer;
                buffer = buffer.offset(1);
                *fresh873 = c_433;
                if c_433 as c_int == 0xffi32 {
                    let fresh874 = buffer;
                    buffer = buffer.offset(1);
                    *fresh874 = 0i32 as JOCTET
                }
                let mut c_434: JOCTET = 0;
                put_bits -= 8i32;
                c_434 = (put_buffer >> put_bits) as JOCTET;
                let fresh875 = buffer;
                buffer = buffer.offset(1);
                *fresh875 = c_434;
                if c_434 as c_int == 0xffi32 {
                    let fresh876 = buffer;
                    buffer = buffer.offset(1);
                    *fresh876 = 0i32 as JOCTET
                }
                let mut c_435: JOCTET = 0;
                put_bits -= 8i32;
                c_435 = (put_buffer >> put_bits) as JOCTET;
                let fresh877 = buffer;
                buffer = buffer.offset(1);
                *fresh877 = c_435;
                if c_435 as c_int == 0xffi32 {
                    let fresh878 = buffer;
                    buffer = buffer.offset(1);
                    *fresh878 = 0i32 as JOCTET
                }
                let mut c_436: JOCTET = 0;
                put_bits -= 8i32;
                c_436 = (put_buffer >> put_bits) as JOCTET;
                let fresh879 = buffer;
                buffer = buffer.offset(1);
                *fresh879 = c_436;
                if c_436 as c_int == 0xffi32 {
                    let fresh880 = buffer;
                    buffer = buffer.offset(1);
                    *fresh880 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_437: JOCTET = 0;
            put_bits -= 8i32;
            c_437 = (put_buffer >> put_bits) as JOCTET;
            let fresh881 = buffer;
            buffer = buffer.offset(1);
            *fresh881 = c_437;
            if c_437 as c_int == 0xffi32 {
                let fresh882 = buffer;
                buffer = buffer.offset(1);
                *fresh882 = 0i32 as JOCTET
            }
            let mut c_438: JOCTET = 0;
            put_bits -= 8i32;
            c_438 = (put_buffer >> put_bits) as JOCTET;
            let fresh883 = buffer;
            buffer = buffer.offset(1);
            *fresh883 = c_438;
            if c_438 as c_int == 0xffi32 {
                let fresh884 = buffer;
                buffer = buffer.offset(1);
                *fresh884 = 0i32 as JOCTET
            }
            let mut c_439: JOCTET = 0;
            put_bits -= 8i32;
            c_439 = (put_buffer >> put_bits) as JOCTET;
            let fresh885 = buffer;
            buffer = buffer.offset(1);
            *fresh885 = c_439;
            if c_439 as c_int == 0xffi32 {
                let fresh886 = buffer;
                buffer = buffer.offset(1);
                *fresh886 = 0i32 as JOCTET
            }
            let mut c_440: JOCTET = 0;
            put_bits -= 8i32;
            c_440 = (put_buffer >> put_bits) as JOCTET;
            let fresh887 = buffer;
            buffer = buffer.offset(1);
            *fresh887 = c_440;
            if c_440 as c_int == 0xffi32 {
                let fresh888 = buffer;
                buffer = buffer.offset(1);
                *fresh888 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(30isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_441: JOCTET = 0;
                put_bits -= 8i32;
                c_441 = (put_buffer >> put_bits) as JOCTET;
                let fresh889 = buffer;
                buffer = buffer.offset(1);
                *fresh889 = c_441;
                if c_441 as c_int == 0xffi32 {
                    let fresh890 = buffer;
                    buffer = buffer.offset(1);
                    *fresh890 = 0i32 as JOCTET
                }
                let mut c_442: JOCTET = 0;
                put_bits -= 8i32;
                c_442 = (put_buffer >> put_bits) as JOCTET;
                let fresh891 = buffer;
                buffer = buffer.offset(1);
                *fresh891 = c_442;
                if c_442 as c_int == 0xffi32 {
                    let fresh892 = buffer;
                    buffer = buffer.offset(1);
                    *fresh892 = 0i32 as JOCTET
                }
                let mut c_443: JOCTET = 0;
                put_bits -= 8i32;
                c_443 = (put_buffer >> put_bits) as JOCTET;
                let fresh893 = buffer;
                buffer = buffer.offset(1);
                *fresh893 = c_443;
                if c_443 as c_int == 0xffi32 {
                    let fresh894 = buffer;
                    buffer = buffer.offset(1);
                    *fresh894 = 0i32 as JOCTET
                }
                let mut c_444: JOCTET = 0;
                put_bits -= 8i32;
                c_444 = (put_buffer >> put_bits) as JOCTET;
                let fresh895 = buffer;
                buffer = buffer.offset(1);
                *fresh895 = c_444;
                if c_444 as c_int == 0xffi32 {
                    let fresh896 = buffer;
                    buffer = buffer.offset(1);
                    *fresh896 = 0i32 as JOCTET
                }
                let mut c_445: JOCTET = 0;
                put_bits -= 8i32;
                c_445 = (put_buffer >> put_bits) as JOCTET;
                let fresh897 = buffer;
                buffer = buffer.offset(1);
                *fresh897 = c_445;
                if c_445 as c_int == 0xffi32 {
                    let fresh898 = buffer;
                    buffer = buffer.offset(1);
                    *fresh898 = 0i32 as JOCTET
                }
                let mut c_446: JOCTET = 0;
                put_bits -= 8i32;
                c_446 = (put_buffer >> put_bits) as JOCTET;
                let fresh899 = buffer;
                buffer = buffer.offset(1);
                *fresh899 = c_446;
                if c_446 as c_int == 0xffi32 {
                    let fresh900 = buffer;
                    buffer = buffer.offset(1);
                    *fresh900 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_447: JOCTET = 0;
            put_bits -= 8i32;
            c_447 = (put_buffer >> put_bits) as JOCTET;
            let fresh901 = buffer;
            buffer = buffer.offset(1);
            *fresh901 = c_447;
            if c_447 as c_int == 0xffi32 {
                let fresh902 = buffer;
                buffer = buffer.offset(1);
                *fresh902 = 0i32 as JOCTET
            }
            let mut c_448: JOCTET = 0;
            put_bits -= 8i32;
            c_448 = (put_buffer >> put_bits) as JOCTET;
            let fresh903 = buffer;
            buffer = buffer.offset(1);
            *fresh903 = c_448;
            if c_448 as c_int == 0xffi32 {
                let fresh904 = buffer;
                buffer = buffer.offset(1);
                *fresh904 = 0i32 as JOCTET
            }
            let mut c_449: JOCTET = 0;
            put_bits -= 8i32;
            c_449 = (put_buffer >> put_bits) as JOCTET;
            let fresh905 = buffer;
            buffer = buffer.offset(1);
            *fresh905 = c_449;
            if c_449 as c_int == 0xffi32 {
                let fresh906 = buffer;
                buffer = buffer.offset(1);
                *fresh906 = 0i32 as JOCTET
            }
            let mut c_450: JOCTET = 0;
            put_bits -= 8i32;
            c_450 = (put_buffer >> put_bits) as JOCTET;
            let fresh907 = buffer;
            buffer = buffer.offset(1);
            *fresh907 = c_450;
            if c_450 as c_int == 0xffi32 {
                let fresh908 = buffer;
                buffer = buffer.offset(1);
                *fresh908 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(37isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_451: JOCTET = 0;
                put_bits -= 8i32;
                c_451 = (put_buffer >> put_bits) as JOCTET;
                let fresh909 = buffer;
                buffer = buffer.offset(1);
                *fresh909 = c_451;
                if c_451 as c_int == 0xffi32 {
                    let fresh910 = buffer;
                    buffer = buffer.offset(1);
                    *fresh910 = 0i32 as JOCTET
                }
                let mut c_452: JOCTET = 0;
                put_bits -= 8i32;
                c_452 = (put_buffer >> put_bits) as JOCTET;
                let fresh911 = buffer;
                buffer = buffer.offset(1);
                *fresh911 = c_452;
                if c_452 as c_int == 0xffi32 {
                    let fresh912 = buffer;
                    buffer = buffer.offset(1);
                    *fresh912 = 0i32 as JOCTET
                }
                let mut c_453: JOCTET = 0;
                put_bits -= 8i32;
                c_453 = (put_buffer >> put_bits) as JOCTET;
                let fresh913 = buffer;
                buffer = buffer.offset(1);
                *fresh913 = c_453;
                if c_453 as c_int == 0xffi32 {
                    let fresh914 = buffer;
                    buffer = buffer.offset(1);
                    *fresh914 = 0i32 as JOCTET
                }
                let mut c_454: JOCTET = 0;
                put_bits -= 8i32;
                c_454 = (put_buffer >> put_bits) as JOCTET;
                let fresh915 = buffer;
                buffer = buffer.offset(1);
                *fresh915 = c_454;
                if c_454 as c_int == 0xffi32 {
                    let fresh916 = buffer;
                    buffer = buffer.offset(1);
                    *fresh916 = 0i32 as JOCTET
                }
                let mut c_455: JOCTET = 0;
                put_bits -= 8i32;
                c_455 = (put_buffer >> put_bits) as JOCTET;
                let fresh917 = buffer;
                buffer = buffer.offset(1);
                *fresh917 = c_455;
                if c_455 as c_int == 0xffi32 {
                    let fresh918 = buffer;
                    buffer = buffer.offset(1);
                    *fresh918 = 0i32 as JOCTET
                }
                let mut c_456: JOCTET = 0;
                put_bits -= 8i32;
                c_456 = (put_buffer >> put_bits) as JOCTET;
                let fresh919 = buffer;
                buffer = buffer.offset(1);
                *fresh919 = c_456;
                if c_456 as c_int == 0xffi32 {
                    let fresh920 = buffer;
                    buffer = buffer.offset(1);
                    *fresh920 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_457: JOCTET = 0;
            put_bits -= 8i32;
            c_457 = (put_buffer >> put_bits) as JOCTET;
            let fresh921 = buffer;
            buffer = buffer.offset(1);
            *fresh921 = c_457;
            if c_457 as c_int == 0xffi32 {
                let fresh922 = buffer;
                buffer = buffer.offset(1);
                *fresh922 = 0i32 as JOCTET
            }
            let mut c_458: JOCTET = 0;
            put_bits -= 8i32;
            c_458 = (put_buffer >> put_bits) as JOCTET;
            let fresh923 = buffer;
            buffer = buffer.offset(1);
            *fresh923 = c_458;
            if c_458 as c_int == 0xffi32 {
                let fresh924 = buffer;
                buffer = buffer.offset(1);
                *fresh924 = 0i32 as JOCTET
            }
            let mut c_459: JOCTET = 0;
            put_bits -= 8i32;
            c_459 = (put_buffer >> put_bits) as JOCTET;
            let fresh925 = buffer;
            buffer = buffer.offset(1);
            *fresh925 = c_459;
            if c_459 as c_int == 0xffi32 {
                let fresh926 = buffer;
                buffer = buffer.offset(1);
                *fresh926 = 0i32 as JOCTET
            }
            let mut c_460: JOCTET = 0;
            put_bits -= 8i32;
            c_460 = (put_buffer >> put_bits) as JOCTET;
            let fresh927 = buffer;
            buffer = buffer.offset(1);
            *fresh927 = c_460;
            if c_460 as c_int == 0xffi32 {
                let fresh928 = buffer;
                buffer = buffer.offset(1);
                *fresh928 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(44isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_461: JOCTET = 0;
                put_bits -= 8i32;
                c_461 = (put_buffer >> put_bits) as JOCTET;
                let fresh929 = buffer;
                buffer = buffer.offset(1);
                *fresh929 = c_461;
                if c_461 as c_int == 0xffi32 {
                    let fresh930 = buffer;
                    buffer = buffer.offset(1);
                    *fresh930 = 0i32 as JOCTET
                }
                let mut c_462: JOCTET = 0;
                put_bits -= 8i32;
                c_462 = (put_buffer >> put_bits) as JOCTET;
                let fresh931 = buffer;
                buffer = buffer.offset(1);
                *fresh931 = c_462;
                if c_462 as c_int == 0xffi32 {
                    let fresh932 = buffer;
                    buffer = buffer.offset(1);
                    *fresh932 = 0i32 as JOCTET
                }
                let mut c_463: JOCTET = 0;
                put_bits -= 8i32;
                c_463 = (put_buffer >> put_bits) as JOCTET;
                let fresh933 = buffer;
                buffer = buffer.offset(1);
                *fresh933 = c_463;
                if c_463 as c_int == 0xffi32 {
                    let fresh934 = buffer;
                    buffer = buffer.offset(1);
                    *fresh934 = 0i32 as JOCTET
                }
                let mut c_464: JOCTET = 0;
                put_bits -= 8i32;
                c_464 = (put_buffer >> put_bits) as JOCTET;
                let fresh935 = buffer;
                buffer = buffer.offset(1);
                *fresh935 = c_464;
                if c_464 as c_int == 0xffi32 {
                    let fresh936 = buffer;
                    buffer = buffer.offset(1);
                    *fresh936 = 0i32 as JOCTET
                }
                let mut c_465: JOCTET = 0;
                put_bits -= 8i32;
                c_465 = (put_buffer >> put_bits) as JOCTET;
                let fresh937 = buffer;
                buffer = buffer.offset(1);
                *fresh937 = c_465;
                if c_465 as c_int == 0xffi32 {
                    let fresh938 = buffer;
                    buffer = buffer.offset(1);
                    *fresh938 = 0i32 as JOCTET
                }
                let mut c_466: JOCTET = 0;
                put_bits -= 8i32;
                c_466 = (put_buffer >> put_bits) as JOCTET;
                let fresh939 = buffer;
                buffer = buffer.offset(1);
                *fresh939 = c_466;
                if c_466 as c_int == 0xffi32 {
                    let fresh940 = buffer;
                    buffer = buffer.offset(1);
                    *fresh940 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_467: JOCTET = 0;
            put_bits -= 8i32;
            c_467 = (put_buffer >> put_bits) as JOCTET;
            let fresh941 = buffer;
            buffer = buffer.offset(1);
            *fresh941 = c_467;
            if c_467 as c_int == 0xffi32 {
                let fresh942 = buffer;
                buffer = buffer.offset(1);
                *fresh942 = 0i32 as JOCTET
            }
            let mut c_468: JOCTET = 0;
            put_bits -= 8i32;
            c_468 = (put_buffer >> put_bits) as JOCTET;
            let fresh943 = buffer;
            buffer = buffer.offset(1);
            *fresh943 = c_468;
            if c_468 as c_int == 0xffi32 {
                let fresh944 = buffer;
                buffer = buffer.offset(1);
                *fresh944 = 0i32 as JOCTET
            }
            let mut c_469: JOCTET = 0;
            put_bits -= 8i32;
            c_469 = (put_buffer >> put_bits) as JOCTET;
            let fresh945 = buffer;
            buffer = buffer.offset(1);
            *fresh945 = c_469;
            if c_469 as c_int == 0xffi32 {
                let fresh946 = buffer;
                buffer = buffer.offset(1);
                *fresh946 = 0i32 as JOCTET
            }
            let mut c_470: JOCTET = 0;
            put_bits -= 8i32;
            c_470 = (put_buffer >> put_bits) as JOCTET;
            let fresh947 = buffer;
            buffer = buffer.offset(1);
            *fresh947 = c_470;
            if c_470 as c_int == 0xffi32 {
                let fresh948 = buffer;
                buffer = buffer.offset(1);
                *fresh948 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(51isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_471: JOCTET = 0;
                put_bits -= 8i32;
                c_471 = (put_buffer >> put_bits) as JOCTET;
                let fresh949 = buffer;
                buffer = buffer.offset(1);
                *fresh949 = c_471;
                if c_471 as c_int == 0xffi32 {
                    let fresh950 = buffer;
                    buffer = buffer.offset(1);
                    *fresh950 = 0i32 as JOCTET
                }
                let mut c_472: JOCTET = 0;
                put_bits -= 8i32;
                c_472 = (put_buffer >> put_bits) as JOCTET;
                let fresh951 = buffer;
                buffer = buffer.offset(1);
                *fresh951 = c_472;
                if c_472 as c_int == 0xffi32 {
                    let fresh952 = buffer;
                    buffer = buffer.offset(1);
                    *fresh952 = 0i32 as JOCTET
                }
                let mut c_473: JOCTET = 0;
                put_bits -= 8i32;
                c_473 = (put_buffer >> put_bits) as JOCTET;
                let fresh953 = buffer;
                buffer = buffer.offset(1);
                *fresh953 = c_473;
                if c_473 as c_int == 0xffi32 {
                    let fresh954 = buffer;
                    buffer = buffer.offset(1);
                    *fresh954 = 0i32 as JOCTET
                }
                let mut c_474: JOCTET = 0;
                put_bits -= 8i32;
                c_474 = (put_buffer >> put_bits) as JOCTET;
                let fresh955 = buffer;
                buffer = buffer.offset(1);
                *fresh955 = c_474;
                if c_474 as c_int == 0xffi32 {
                    let fresh956 = buffer;
                    buffer = buffer.offset(1);
                    *fresh956 = 0i32 as JOCTET
                }
                let mut c_475: JOCTET = 0;
                put_bits -= 8i32;
                c_475 = (put_buffer >> put_bits) as JOCTET;
                let fresh957 = buffer;
                buffer = buffer.offset(1);
                *fresh957 = c_475;
                if c_475 as c_int == 0xffi32 {
                    let fresh958 = buffer;
                    buffer = buffer.offset(1);
                    *fresh958 = 0i32 as JOCTET
                }
                let mut c_476: JOCTET = 0;
                put_bits -= 8i32;
                c_476 = (put_buffer >> put_bits) as JOCTET;
                let fresh959 = buffer;
                buffer = buffer.offset(1);
                *fresh959 = c_476;
                if c_476 as c_int == 0xffi32 {
                    let fresh960 = buffer;
                    buffer = buffer.offset(1);
                    *fresh960 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_477: JOCTET = 0;
            put_bits -= 8i32;
            c_477 = (put_buffer >> put_bits) as JOCTET;
            let fresh961 = buffer;
            buffer = buffer.offset(1);
            *fresh961 = c_477;
            if c_477 as c_int == 0xffi32 {
                let fresh962 = buffer;
                buffer = buffer.offset(1);
                *fresh962 = 0i32 as JOCTET
            }
            let mut c_478: JOCTET = 0;
            put_bits -= 8i32;
            c_478 = (put_buffer >> put_bits) as JOCTET;
            let fresh963 = buffer;
            buffer = buffer.offset(1);
            *fresh963 = c_478;
            if c_478 as c_int == 0xffi32 {
                let fresh964 = buffer;
                buffer = buffer.offset(1);
                *fresh964 = 0i32 as JOCTET
            }
            let mut c_479: JOCTET = 0;
            put_bits -= 8i32;
            c_479 = (put_buffer >> put_bits) as JOCTET;
            let fresh965 = buffer;
            buffer = buffer.offset(1);
            *fresh965 = c_479;
            if c_479 as c_int == 0xffi32 {
                let fresh966 = buffer;
                buffer = buffer.offset(1);
                *fresh966 = 0i32 as JOCTET
            }
            let mut c_480: JOCTET = 0;
            put_bits -= 8i32;
            c_480 = (put_buffer >> put_bits) as JOCTET;
            let fresh967 = buffer;
            buffer = buffer.offset(1);
            *fresh967 = c_480;
            if c_480 as c_int == 0xffi32 {
                let fresh968 = buffer;
                buffer = buffer.offset(1);
                *fresh968 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(58isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_481: JOCTET = 0;
                put_bits -= 8i32;
                c_481 = (put_buffer >> put_bits) as JOCTET;
                let fresh969 = buffer;
                buffer = buffer.offset(1);
                *fresh969 = c_481;
                if c_481 as c_int == 0xffi32 {
                    let fresh970 = buffer;
                    buffer = buffer.offset(1);
                    *fresh970 = 0i32 as JOCTET
                }
                let mut c_482: JOCTET = 0;
                put_bits -= 8i32;
                c_482 = (put_buffer >> put_bits) as JOCTET;
                let fresh971 = buffer;
                buffer = buffer.offset(1);
                *fresh971 = c_482;
                if c_482 as c_int == 0xffi32 {
                    let fresh972 = buffer;
                    buffer = buffer.offset(1);
                    *fresh972 = 0i32 as JOCTET
                }
                let mut c_483: JOCTET = 0;
                put_bits -= 8i32;
                c_483 = (put_buffer >> put_bits) as JOCTET;
                let fresh973 = buffer;
                buffer = buffer.offset(1);
                *fresh973 = c_483;
                if c_483 as c_int == 0xffi32 {
                    let fresh974 = buffer;
                    buffer = buffer.offset(1);
                    *fresh974 = 0i32 as JOCTET
                }
                let mut c_484: JOCTET = 0;
                put_bits -= 8i32;
                c_484 = (put_buffer >> put_bits) as JOCTET;
                let fresh975 = buffer;
                buffer = buffer.offset(1);
                *fresh975 = c_484;
                if c_484 as c_int == 0xffi32 {
                    let fresh976 = buffer;
                    buffer = buffer.offset(1);
                    *fresh976 = 0i32 as JOCTET
                }
                let mut c_485: JOCTET = 0;
                put_bits -= 8i32;
                c_485 = (put_buffer >> put_bits) as JOCTET;
                let fresh977 = buffer;
                buffer = buffer.offset(1);
                *fresh977 = c_485;
                if c_485 as c_int == 0xffi32 {
                    let fresh978 = buffer;
                    buffer = buffer.offset(1);
                    *fresh978 = 0i32 as JOCTET
                }
                let mut c_486: JOCTET = 0;
                put_bits -= 8i32;
                c_486 = (put_buffer >> put_bits) as JOCTET;
                let fresh979 = buffer;
                buffer = buffer.offset(1);
                *fresh979 = c_486;
                if c_486 as c_int == 0xffi32 {
                    let fresh980 = buffer;
                    buffer = buffer.offset(1);
                    *fresh980 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_487: JOCTET = 0;
            put_bits -= 8i32;
            c_487 = (put_buffer >> put_bits) as JOCTET;
            let fresh981 = buffer;
            buffer = buffer.offset(1);
            *fresh981 = c_487;
            if c_487 as c_int == 0xffi32 {
                let fresh982 = buffer;
                buffer = buffer.offset(1);
                *fresh982 = 0i32 as JOCTET
            }
            let mut c_488: JOCTET = 0;
            put_bits -= 8i32;
            c_488 = (put_buffer >> put_bits) as JOCTET;
            let fresh983 = buffer;
            buffer = buffer.offset(1);
            *fresh983 = c_488;
            if c_488 as c_int == 0xffi32 {
                let fresh984 = buffer;
                buffer = buffer.offset(1);
                *fresh984 = 0i32 as JOCTET
            }
            let mut c_489: JOCTET = 0;
            put_bits -= 8i32;
            c_489 = (put_buffer >> put_bits) as JOCTET;
            let fresh985 = buffer;
            buffer = buffer.offset(1);
            *fresh985 = c_489;
            if c_489 as c_int == 0xffi32 {
                let fresh986 = buffer;
                buffer = buffer.offset(1);
                *fresh986 = 0i32 as JOCTET
            }
            let mut c_490: JOCTET = 0;
            put_bits -= 8i32;
            c_490 = (put_buffer >> put_bits) as JOCTET;
            let fresh987 = buffer;
            buffer = buffer.offset(1);
            *fresh987 = c_490;
            if c_490 as c_int == 0xffi32 {
                let fresh988 = buffer;
                buffer = buffer.offset(1);
                *fresh988 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(59isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_491: JOCTET = 0;
                put_bits -= 8i32;
                c_491 = (put_buffer >> put_bits) as JOCTET;
                let fresh989 = buffer;
                buffer = buffer.offset(1);
                *fresh989 = c_491;
                if c_491 as c_int == 0xffi32 {
                    let fresh990 = buffer;
                    buffer = buffer.offset(1);
                    *fresh990 = 0i32 as JOCTET
                }
                let mut c_492: JOCTET = 0;
                put_bits -= 8i32;
                c_492 = (put_buffer >> put_bits) as JOCTET;
                let fresh991 = buffer;
                buffer = buffer.offset(1);
                *fresh991 = c_492;
                if c_492 as c_int == 0xffi32 {
                    let fresh992 = buffer;
                    buffer = buffer.offset(1);
                    *fresh992 = 0i32 as JOCTET
                }
                let mut c_493: JOCTET = 0;
                put_bits -= 8i32;
                c_493 = (put_buffer >> put_bits) as JOCTET;
                let fresh993 = buffer;
                buffer = buffer.offset(1);
                *fresh993 = c_493;
                if c_493 as c_int == 0xffi32 {
                    let fresh994 = buffer;
                    buffer = buffer.offset(1);
                    *fresh994 = 0i32 as JOCTET
                }
                let mut c_494: JOCTET = 0;
                put_bits -= 8i32;
                c_494 = (put_buffer >> put_bits) as JOCTET;
                let fresh995 = buffer;
                buffer = buffer.offset(1);
                *fresh995 = c_494;
                if c_494 as c_int == 0xffi32 {
                    let fresh996 = buffer;
                    buffer = buffer.offset(1);
                    *fresh996 = 0i32 as JOCTET
                }
                let mut c_495: JOCTET = 0;
                put_bits -= 8i32;
                c_495 = (put_buffer >> put_bits) as JOCTET;
                let fresh997 = buffer;
                buffer = buffer.offset(1);
                *fresh997 = c_495;
                if c_495 as c_int == 0xffi32 {
                    let fresh998 = buffer;
                    buffer = buffer.offset(1);
                    *fresh998 = 0i32 as JOCTET
                }
                let mut c_496: JOCTET = 0;
                put_bits -= 8i32;
                c_496 = (put_buffer >> put_bits) as JOCTET;
                let fresh999 = buffer;
                buffer = buffer.offset(1);
                *fresh999 = c_496;
                if c_496 as c_int == 0xffi32 {
                    let fresh1000 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1000 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_497: JOCTET = 0;
            put_bits -= 8i32;
            c_497 = (put_buffer >> put_bits) as JOCTET;
            let fresh1001 = buffer;
            buffer = buffer.offset(1);
            *fresh1001 = c_497;
            if c_497 as c_int == 0xffi32 {
                let fresh1002 = buffer;
                buffer = buffer.offset(1);
                *fresh1002 = 0i32 as JOCTET
            }
            let mut c_498: JOCTET = 0;
            put_bits -= 8i32;
            c_498 = (put_buffer >> put_bits) as JOCTET;
            let fresh1003 = buffer;
            buffer = buffer.offset(1);
            *fresh1003 = c_498;
            if c_498 as c_int == 0xffi32 {
                let fresh1004 = buffer;
                buffer = buffer.offset(1);
                *fresh1004 = 0i32 as JOCTET
            }
            let mut c_499: JOCTET = 0;
            put_bits -= 8i32;
            c_499 = (put_buffer >> put_bits) as JOCTET;
            let fresh1005 = buffer;
            buffer = buffer.offset(1);
            *fresh1005 = c_499;
            if c_499 as c_int == 0xffi32 {
                let fresh1006 = buffer;
                buffer = buffer.offset(1);
                *fresh1006 = 0i32 as JOCTET
            }
            let mut c_500: JOCTET = 0;
            put_bits -= 8i32;
            c_500 = (put_buffer >> put_bits) as JOCTET;
            let fresh1007 = buffer;
            buffer = buffer.offset(1);
            *fresh1007 = c_500;
            if c_500 as c_int == 0xffi32 {
                let fresh1008 = buffer;
                buffer = buffer.offset(1);
                *fresh1008 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(52isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_501: JOCTET = 0;
                put_bits -= 8i32;
                c_501 = (put_buffer >> put_bits) as JOCTET;
                let fresh1009 = buffer;
                buffer = buffer.offset(1);
                *fresh1009 = c_501;
                if c_501 as c_int == 0xffi32 {
                    let fresh1010 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1010 = 0i32 as JOCTET
                }
                let mut c_502: JOCTET = 0;
                put_bits -= 8i32;
                c_502 = (put_buffer >> put_bits) as JOCTET;
                let fresh1011 = buffer;
                buffer = buffer.offset(1);
                *fresh1011 = c_502;
                if c_502 as c_int == 0xffi32 {
                    let fresh1012 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1012 = 0i32 as JOCTET
                }
                let mut c_503: JOCTET = 0;
                put_bits -= 8i32;
                c_503 = (put_buffer >> put_bits) as JOCTET;
                let fresh1013 = buffer;
                buffer = buffer.offset(1);
                *fresh1013 = c_503;
                if c_503 as c_int == 0xffi32 {
                    let fresh1014 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1014 = 0i32 as JOCTET
                }
                let mut c_504: JOCTET = 0;
                put_bits -= 8i32;
                c_504 = (put_buffer >> put_bits) as JOCTET;
                let fresh1015 = buffer;
                buffer = buffer.offset(1);
                *fresh1015 = c_504;
                if c_504 as c_int == 0xffi32 {
                    let fresh1016 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1016 = 0i32 as JOCTET
                }
                let mut c_505: JOCTET = 0;
                put_bits -= 8i32;
                c_505 = (put_buffer >> put_bits) as JOCTET;
                let fresh1017 = buffer;
                buffer = buffer.offset(1);
                *fresh1017 = c_505;
                if c_505 as c_int == 0xffi32 {
                    let fresh1018 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1018 = 0i32 as JOCTET
                }
                let mut c_506: JOCTET = 0;
                put_bits -= 8i32;
                c_506 = (put_buffer >> put_bits) as JOCTET;
                let fresh1019 = buffer;
                buffer = buffer.offset(1);
                *fresh1019 = c_506;
                if c_506 as c_int == 0xffi32 {
                    let fresh1020 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1020 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_507: JOCTET = 0;
            put_bits -= 8i32;
            c_507 = (put_buffer >> put_bits) as JOCTET;
            let fresh1021 = buffer;
            buffer = buffer.offset(1);
            *fresh1021 = c_507;
            if c_507 as c_int == 0xffi32 {
                let fresh1022 = buffer;
                buffer = buffer.offset(1);
                *fresh1022 = 0i32 as JOCTET
            }
            let mut c_508: JOCTET = 0;
            put_bits -= 8i32;
            c_508 = (put_buffer >> put_bits) as JOCTET;
            let fresh1023 = buffer;
            buffer = buffer.offset(1);
            *fresh1023 = c_508;
            if c_508 as c_int == 0xffi32 {
                let fresh1024 = buffer;
                buffer = buffer.offset(1);
                *fresh1024 = 0i32 as JOCTET
            }
            let mut c_509: JOCTET = 0;
            put_bits -= 8i32;
            c_509 = (put_buffer >> put_bits) as JOCTET;
            let fresh1025 = buffer;
            buffer = buffer.offset(1);
            *fresh1025 = c_509;
            if c_509 as c_int == 0xffi32 {
                let fresh1026 = buffer;
                buffer = buffer.offset(1);
                *fresh1026 = 0i32 as JOCTET
            }
            let mut c_510: JOCTET = 0;
            put_bits -= 8i32;
            c_510 = (put_buffer >> put_bits) as JOCTET;
            let fresh1027 = buffer;
            buffer = buffer.offset(1);
            *fresh1027 = c_510;
            if c_510 as c_int == 0xffi32 {
                let fresh1028 = buffer;
                buffer = buffer.offset(1);
                *fresh1028 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(45isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_511: JOCTET = 0;
                put_bits -= 8i32;
                c_511 = (put_buffer >> put_bits) as JOCTET;
                let fresh1029 = buffer;
                buffer = buffer.offset(1);
                *fresh1029 = c_511;
                if c_511 as c_int == 0xffi32 {
                    let fresh1030 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1030 = 0i32 as JOCTET
                }
                let mut c_512: JOCTET = 0;
                put_bits -= 8i32;
                c_512 = (put_buffer >> put_bits) as JOCTET;
                let fresh1031 = buffer;
                buffer = buffer.offset(1);
                *fresh1031 = c_512;
                if c_512 as c_int == 0xffi32 {
                    let fresh1032 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1032 = 0i32 as JOCTET
                }
                let mut c_513: JOCTET = 0;
                put_bits -= 8i32;
                c_513 = (put_buffer >> put_bits) as JOCTET;
                let fresh1033 = buffer;
                buffer = buffer.offset(1);
                *fresh1033 = c_513;
                if c_513 as c_int == 0xffi32 {
                    let fresh1034 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1034 = 0i32 as JOCTET
                }
                let mut c_514: JOCTET = 0;
                put_bits -= 8i32;
                c_514 = (put_buffer >> put_bits) as JOCTET;
                let fresh1035 = buffer;
                buffer = buffer.offset(1);
                *fresh1035 = c_514;
                if c_514 as c_int == 0xffi32 {
                    let fresh1036 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1036 = 0i32 as JOCTET
                }
                let mut c_515: JOCTET = 0;
                put_bits -= 8i32;
                c_515 = (put_buffer >> put_bits) as JOCTET;
                let fresh1037 = buffer;
                buffer = buffer.offset(1);
                *fresh1037 = c_515;
                if c_515 as c_int == 0xffi32 {
                    let fresh1038 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1038 = 0i32 as JOCTET
                }
                let mut c_516: JOCTET = 0;
                put_bits -= 8i32;
                c_516 = (put_buffer >> put_bits) as JOCTET;
                let fresh1039 = buffer;
                buffer = buffer.offset(1);
                *fresh1039 = c_516;
                if c_516 as c_int == 0xffi32 {
                    let fresh1040 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1040 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_517: JOCTET = 0;
            put_bits -= 8i32;
            c_517 = (put_buffer >> put_bits) as JOCTET;
            let fresh1041 = buffer;
            buffer = buffer.offset(1);
            *fresh1041 = c_517;
            if c_517 as c_int == 0xffi32 {
                let fresh1042 = buffer;
                buffer = buffer.offset(1);
                *fresh1042 = 0i32 as JOCTET
            }
            let mut c_518: JOCTET = 0;
            put_bits -= 8i32;
            c_518 = (put_buffer >> put_bits) as JOCTET;
            let fresh1043 = buffer;
            buffer = buffer.offset(1);
            *fresh1043 = c_518;
            if c_518 as c_int == 0xffi32 {
                let fresh1044 = buffer;
                buffer = buffer.offset(1);
                *fresh1044 = 0i32 as JOCTET
            }
            let mut c_519: JOCTET = 0;
            put_bits -= 8i32;
            c_519 = (put_buffer >> put_bits) as JOCTET;
            let fresh1045 = buffer;
            buffer = buffer.offset(1);
            *fresh1045 = c_519;
            if c_519 as c_int == 0xffi32 {
                let fresh1046 = buffer;
                buffer = buffer.offset(1);
                *fresh1046 = 0i32 as JOCTET
            }
            let mut c_520: JOCTET = 0;
            put_bits -= 8i32;
            c_520 = (put_buffer >> put_bits) as JOCTET;
            let fresh1047 = buffer;
            buffer = buffer.offset(1);
            *fresh1047 = c_520;
            if c_520 as c_int == 0xffi32 {
                let fresh1048 = buffer;
                buffer = buffer.offset(1);
                *fresh1048 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(38isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_521: JOCTET = 0;
                put_bits -= 8i32;
                c_521 = (put_buffer >> put_bits) as JOCTET;
                let fresh1049 = buffer;
                buffer = buffer.offset(1);
                *fresh1049 = c_521;
                if c_521 as c_int == 0xffi32 {
                    let fresh1050 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1050 = 0i32 as JOCTET
                }
                let mut c_522: JOCTET = 0;
                put_bits -= 8i32;
                c_522 = (put_buffer >> put_bits) as JOCTET;
                let fresh1051 = buffer;
                buffer = buffer.offset(1);
                *fresh1051 = c_522;
                if c_522 as c_int == 0xffi32 {
                    let fresh1052 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1052 = 0i32 as JOCTET
                }
                let mut c_523: JOCTET = 0;
                put_bits -= 8i32;
                c_523 = (put_buffer >> put_bits) as JOCTET;
                let fresh1053 = buffer;
                buffer = buffer.offset(1);
                *fresh1053 = c_523;
                if c_523 as c_int == 0xffi32 {
                    let fresh1054 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1054 = 0i32 as JOCTET
                }
                let mut c_524: JOCTET = 0;
                put_bits -= 8i32;
                c_524 = (put_buffer >> put_bits) as JOCTET;
                let fresh1055 = buffer;
                buffer = buffer.offset(1);
                *fresh1055 = c_524;
                if c_524 as c_int == 0xffi32 {
                    let fresh1056 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1056 = 0i32 as JOCTET
                }
                let mut c_525: JOCTET = 0;
                put_bits -= 8i32;
                c_525 = (put_buffer >> put_bits) as JOCTET;
                let fresh1057 = buffer;
                buffer = buffer.offset(1);
                *fresh1057 = c_525;
                if c_525 as c_int == 0xffi32 {
                    let fresh1058 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1058 = 0i32 as JOCTET
                }
                let mut c_526: JOCTET = 0;
                put_bits -= 8i32;
                c_526 = (put_buffer >> put_bits) as JOCTET;
                let fresh1059 = buffer;
                buffer = buffer.offset(1);
                *fresh1059 = c_526;
                if c_526 as c_int == 0xffi32 {
                    let fresh1060 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1060 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_527: JOCTET = 0;
            put_bits -= 8i32;
            c_527 = (put_buffer >> put_bits) as JOCTET;
            let fresh1061 = buffer;
            buffer = buffer.offset(1);
            *fresh1061 = c_527;
            if c_527 as c_int == 0xffi32 {
                let fresh1062 = buffer;
                buffer = buffer.offset(1);
                *fresh1062 = 0i32 as JOCTET
            }
            let mut c_528: JOCTET = 0;
            put_bits -= 8i32;
            c_528 = (put_buffer >> put_bits) as JOCTET;
            let fresh1063 = buffer;
            buffer = buffer.offset(1);
            *fresh1063 = c_528;
            if c_528 as c_int == 0xffi32 {
                let fresh1064 = buffer;
                buffer = buffer.offset(1);
                *fresh1064 = 0i32 as JOCTET
            }
            let mut c_529: JOCTET = 0;
            put_bits -= 8i32;
            c_529 = (put_buffer >> put_bits) as JOCTET;
            let fresh1065 = buffer;
            buffer = buffer.offset(1);
            *fresh1065 = c_529;
            if c_529 as c_int == 0xffi32 {
                let fresh1066 = buffer;
                buffer = buffer.offset(1);
                *fresh1066 = 0i32 as JOCTET
            }
            let mut c_530: JOCTET = 0;
            put_bits -= 8i32;
            c_530 = (put_buffer >> put_bits) as JOCTET;
            let fresh1067 = buffer;
            buffer = buffer.offset(1);
            *fresh1067 = c_530;
            if c_530 as c_int == 0xffi32 {
                let fresh1068 = buffer;
                buffer = buffer.offset(1);
                *fresh1068 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(31isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_531: JOCTET = 0;
                put_bits -= 8i32;
                c_531 = (put_buffer >> put_bits) as JOCTET;
                let fresh1069 = buffer;
                buffer = buffer.offset(1);
                *fresh1069 = c_531;
                if c_531 as c_int == 0xffi32 {
                    let fresh1070 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1070 = 0i32 as JOCTET
                }
                let mut c_532: JOCTET = 0;
                put_bits -= 8i32;
                c_532 = (put_buffer >> put_bits) as JOCTET;
                let fresh1071 = buffer;
                buffer = buffer.offset(1);
                *fresh1071 = c_532;
                if c_532 as c_int == 0xffi32 {
                    let fresh1072 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1072 = 0i32 as JOCTET
                }
                let mut c_533: JOCTET = 0;
                put_bits -= 8i32;
                c_533 = (put_buffer >> put_bits) as JOCTET;
                let fresh1073 = buffer;
                buffer = buffer.offset(1);
                *fresh1073 = c_533;
                if c_533 as c_int == 0xffi32 {
                    let fresh1074 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1074 = 0i32 as JOCTET
                }
                let mut c_534: JOCTET = 0;
                put_bits -= 8i32;
                c_534 = (put_buffer >> put_bits) as JOCTET;
                let fresh1075 = buffer;
                buffer = buffer.offset(1);
                *fresh1075 = c_534;
                if c_534 as c_int == 0xffi32 {
                    let fresh1076 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1076 = 0i32 as JOCTET
                }
                let mut c_535: JOCTET = 0;
                put_bits -= 8i32;
                c_535 = (put_buffer >> put_bits) as JOCTET;
                let fresh1077 = buffer;
                buffer = buffer.offset(1);
                *fresh1077 = c_535;
                if c_535 as c_int == 0xffi32 {
                    let fresh1078 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1078 = 0i32 as JOCTET
                }
                let mut c_536: JOCTET = 0;
                put_bits -= 8i32;
                c_536 = (put_buffer >> put_bits) as JOCTET;
                let fresh1079 = buffer;
                buffer = buffer.offset(1);
                *fresh1079 = c_536;
                if c_536 as c_int == 0xffi32 {
                    let fresh1080 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1080 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_537: JOCTET = 0;
            put_bits -= 8i32;
            c_537 = (put_buffer >> put_bits) as JOCTET;
            let fresh1081 = buffer;
            buffer = buffer.offset(1);
            *fresh1081 = c_537;
            if c_537 as c_int == 0xffi32 {
                let fresh1082 = buffer;
                buffer = buffer.offset(1);
                *fresh1082 = 0i32 as JOCTET
            }
            let mut c_538: JOCTET = 0;
            put_bits -= 8i32;
            c_538 = (put_buffer >> put_bits) as JOCTET;
            let fresh1083 = buffer;
            buffer = buffer.offset(1);
            *fresh1083 = c_538;
            if c_538 as c_int == 0xffi32 {
                let fresh1084 = buffer;
                buffer = buffer.offset(1);
                *fresh1084 = 0i32 as JOCTET
            }
            let mut c_539: JOCTET = 0;
            put_bits -= 8i32;
            c_539 = (put_buffer >> put_bits) as JOCTET;
            let fresh1085 = buffer;
            buffer = buffer.offset(1);
            *fresh1085 = c_539;
            if c_539 as c_int == 0xffi32 {
                let fresh1086 = buffer;
                buffer = buffer.offset(1);
                *fresh1086 = 0i32 as JOCTET
            }
            let mut c_540: JOCTET = 0;
            put_bits -= 8i32;
            c_540 = (put_buffer >> put_bits) as JOCTET;
            let fresh1087 = buffer;
            buffer = buffer.offset(1);
            *fresh1087 = c_540;
            if c_540 as c_int == 0xffi32 {
                let fresh1088 = buffer;
                buffer = buffer.offset(1);
                *fresh1088 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(39isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_541: JOCTET = 0;
                put_bits -= 8i32;
                c_541 = (put_buffer >> put_bits) as JOCTET;
                let fresh1089 = buffer;
                buffer = buffer.offset(1);
                *fresh1089 = c_541;
                if c_541 as c_int == 0xffi32 {
                    let fresh1090 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1090 = 0i32 as JOCTET
                }
                let mut c_542: JOCTET = 0;
                put_bits -= 8i32;
                c_542 = (put_buffer >> put_bits) as JOCTET;
                let fresh1091 = buffer;
                buffer = buffer.offset(1);
                *fresh1091 = c_542;
                if c_542 as c_int == 0xffi32 {
                    let fresh1092 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1092 = 0i32 as JOCTET
                }
                let mut c_543: JOCTET = 0;
                put_bits -= 8i32;
                c_543 = (put_buffer >> put_bits) as JOCTET;
                let fresh1093 = buffer;
                buffer = buffer.offset(1);
                *fresh1093 = c_543;
                if c_543 as c_int == 0xffi32 {
                    let fresh1094 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1094 = 0i32 as JOCTET
                }
                let mut c_544: JOCTET = 0;
                put_bits -= 8i32;
                c_544 = (put_buffer >> put_bits) as JOCTET;
                let fresh1095 = buffer;
                buffer = buffer.offset(1);
                *fresh1095 = c_544;
                if c_544 as c_int == 0xffi32 {
                    let fresh1096 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1096 = 0i32 as JOCTET
                }
                let mut c_545: JOCTET = 0;
                put_bits -= 8i32;
                c_545 = (put_buffer >> put_bits) as JOCTET;
                let fresh1097 = buffer;
                buffer = buffer.offset(1);
                *fresh1097 = c_545;
                if c_545 as c_int == 0xffi32 {
                    let fresh1098 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1098 = 0i32 as JOCTET
                }
                let mut c_546: JOCTET = 0;
                put_bits -= 8i32;
                c_546 = (put_buffer >> put_bits) as JOCTET;
                let fresh1099 = buffer;
                buffer = buffer.offset(1);
                *fresh1099 = c_546;
                if c_546 as c_int == 0xffi32 {
                    let fresh1100 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1100 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_547: JOCTET = 0;
            put_bits -= 8i32;
            c_547 = (put_buffer >> put_bits) as JOCTET;
            let fresh1101 = buffer;
            buffer = buffer.offset(1);
            *fresh1101 = c_547;
            if c_547 as c_int == 0xffi32 {
                let fresh1102 = buffer;
                buffer = buffer.offset(1);
                *fresh1102 = 0i32 as JOCTET
            }
            let mut c_548: JOCTET = 0;
            put_bits -= 8i32;
            c_548 = (put_buffer >> put_bits) as JOCTET;
            let fresh1103 = buffer;
            buffer = buffer.offset(1);
            *fresh1103 = c_548;
            if c_548 as c_int == 0xffi32 {
                let fresh1104 = buffer;
                buffer = buffer.offset(1);
                *fresh1104 = 0i32 as JOCTET
            }
            let mut c_549: JOCTET = 0;
            put_bits -= 8i32;
            c_549 = (put_buffer >> put_bits) as JOCTET;
            let fresh1105 = buffer;
            buffer = buffer.offset(1);
            *fresh1105 = c_549;
            if c_549 as c_int == 0xffi32 {
                let fresh1106 = buffer;
                buffer = buffer.offset(1);
                *fresh1106 = 0i32 as JOCTET
            }
            let mut c_550: JOCTET = 0;
            put_bits -= 8i32;
            c_550 = (put_buffer >> put_bits) as JOCTET;
            let fresh1107 = buffer;
            buffer = buffer.offset(1);
            *fresh1107 = c_550;
            if c_550 as c_int == 0xffi32 {
                let fresh1108 = buffer;
                buffer = buffer.offset(1);
                *fresh1108 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(46isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_551: JOCTET = 0;
                put_bits -= 8i32;
                c_551 = (put_buffer >> put_bits) as JOCTET;
                let fresh1109 = buffer;
                buffer = buffer.offset(1);
                *fresh1109 = c_551;
                if c_551 as c_int == 0xffi32 {
                    let fresh1110 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1110 = 0i32 as JOCTET
                }
                let mut c_552: JOCTET = 0;
                put_bits -= 8i32;
                c_552 = (put_buffer >> put_bits) as JOCTET;
                let fresh1111 = buffer;
                buffer = buffer.offset(1);
                *fresh1111 = c_552;
                if c_552 as c_int == 0xffi32 {
                    let fresh1112 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1112 = 0i32 as JOCTET
                }
                let mut c_553: JOCTET = 0;
                put_bits -= 8i32;
                c_553 = (put_buffer >> put_bits) as JOCTET;
                let fresh1113 = buffer;
                buffer = buffer.offset(1);
                *fresh1113 = c_553;
                if c_553 as c_int == 0xffi32 {
                    let fresh1114 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1114 = 0i32 as JOCTET
                }
                let mut c_554: JOCTET = 0;
                put_bits -= 8i32;
                c_554 = (put_buffer >> put_bits) as JOCTET;
                let fresh1115 = buffer;
                buffer = buffer.offset(1);
                *fresh1115 = c_554;
                if c_554 as c_int == 0xffi32 {
                    let fresh1116 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1116 = 0i32 as JOCTET
                }
                let mut c_555: JOCTET = 0;
                put_bits -= 8i32;
                c_555 = (put_buffer >> put_bits) as JOCTET;
                let fresh1117 = buffer;
                buffer = buffer.offset(1);
                *fresh1117 = c_555;
                if c_555 as c_int == 0xffi32 {
                    let fresh1118 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1118 = 0i32 as JOCTET
                }
                let mut c_556: JOCTET = 0;
                put_bits -= 8i32;
                c_556 = (put_buffer >> put_bits) as JOCTET;
                let fresh1119 = buffer;
                buffer = buffer.offset(1);
                *fresh1119 = c_556;
                if c_556 as c_int == 0xffi32 {
                    let fresh1120 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1120 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_557: JOCTET = 0;
            put_bits -= 8i32;
            c_557 = (put_buffer >> put_bits) as JOCTET;
            let fresh1121 = buffer;
            buffer = buffer.offset(1);
            *fresh1121 = c_557;
            if c_557 as c_int == 0xffi32 {
                let fresh1122 = buffer;
                buffer = buffer.offset(1);
                *fresh1122 = 0i32 as JOCTET
            }
            let mut c_558: JOCTET = 0;
            put_bits -= 8i32;
            c_558 = (put_buffer >> put_bits) as JOCTET;
            let fresh1123 = buffer;
            buffer = buffer.offset(1);
            *fresh1123 = c_558;
            if c_558 as c_int == 0xffi32 {
                let fresh1124 = buffer;
                buffer = buffer.offset(1);
                *fresh1124 = 0i32 as JOCTET
            }
            let mut c_559: JOCTET = 0;
            put_bits -= 8i32;
            c_559 = (put_buffer >> put_bits) as JOCTET;
            let fresh1125 = buffer;
            buffer = buffer.offset(1);
            *fresh1125 = c_559;
            if c_559 as c_int == 0xffi32 {
                let fresh1126 = buffer;
                buffer = buffer.offset(1);
                *fresh1126 = 0i32 as JOCTET
            }
            let mut c_560: JOCTET = 0;
            put_bits -= 8i32;
            c_560 = (put_buffer >> put_bits) as JOCTET;
            let fresh1127 = buffer;
            buffer = buffer.offset(1);
            *fresh1127 = c_560;
            if c_560 as c_int == 0xffi32 {
                let fresh1128 = buffer;
                buffer = buffer.offset(1);
                *fresh1128 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(53isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_561: JOCTET = 0;
                put_bits -= 8i32;
                c_561 = (put_buffer >> put_bits) as JOCTET;
                let fresh1129 = buffer;
                buffer = buffer.offset(1);
                *fresh1129 = c_561;
                if c_561 as c_int == 0xffi32 {
                    let fresh1130 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1130 = 0i32 as JOCTET
                }
                let mut c_562: JOCTET = 0;
                put_bits -= 8i32;
                c_562 = (put_buffer >> put_bits) as JOCTET;
                let fresh1131 = buffer;
                buffer = buffer.offset(1);
                *fresh1131 = c_562;
                if c_562 as c_int == 0xffi32 {
                    let fresh1132 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1132 = 0i32 as JOCTET
                }
                let mut c_563: JOCTET = 0;
                put_bits -= 8i32;
                c_563 = (put_buffer >> put_bits) as JOCTET;
                let fresh1133 = buffer;
                buffer = buffer.offset(1);
                *fresh1133 = c_563;
                if c_563 as c_int == 0xffi32 {
                    let fresh1134 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1134 = 0i32 as JOCTET
                }
                let mut c_564: JOCTET = 0;
                put_bits -= 8i32;
                c_564 = (put_buffer >> put_bits) as JOCTET;
                let fresh1135 = buffer;
                buffer = buffer.offset(1);
                *fresh1135 = c_564;
                if c_564 as c_int == 0xffi32 {
                    let fresh1136 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1136 = 0i32 as JOCTET
                }
                let mut c_565: JOCTET = 0;
                put_bits -= 8i32;
                c_565 = (put_buffer >> put_bits) as JOCTET;
                let fresh1137 = buffer;
                buffer = buffer.offset(1);
                *fresh1137 = c_565;
                if c_565 as c_int == 0xffi32 {
                    let fresh1138 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1138 = 0i32 as JOCTET
                }
                let mut c_566: JOCTET = 0;
                put_bits -= 8i32;
                c_566 = (put_buffer >> put_bits) as JOCTET;
                let fresh1139 = buffer;
                buffer = buffer.offset(1);
                *fresh1139 = c_566;
                if c_566 as c_int == 0xffi32 {
                    let fresh1140 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1140 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_567: JOCTET = 0;
            put_bits -= 8i32;
            c_567 = (put_buffer >> put_bits) as JOCTET;
            let fresh1141 = buffer;
            buffer = buffer.offset(1);
            *fresh1141 = c_567;
            if c_567 as c_int == 0xffi32 {
                let fresh1142 = buffer;
                buffer = buffer.offset(1);
                *fresh1142 = 0i32 as JOCTET
            }
            let mut c_568: JOCTET = 0;
            put_bits -= 8i32;
            c_568 = (put_buffer >> put_bits) as JOCTET;
            let fresh1143 = buffer;
            buffer = buffer.offset(1);
            *fresh1143 = c_568;
            if c_568 as c_int == 0xffi32 {
                let fresh1144 = buffer;
                buffer = buffer.offset(1);
                *fresh1144 = 0i32 as JOCTET
            }
            let mut c_569: JOCTET = 0;
            put_bits -= 8i32;
            c_569 = (put_buffer >> put_bits) as JOCTET;
            let fresh1145 = buffer;
            buffer = buffer.offset(1);
            *fresh1145 = c_569;
            if c_569 as c_int == 0xffi32 {
                let fresh1146 = buffer;
                buffer = buffer.offset(1);
                *fresh1146 = 0i32 as JOCTET
            }
            let mut c_570: JOCTET = 0;
            put_bits -= 8i32;
            c_570 = (put_buffer >> put_bits) as JOCTET;
            let fresh1147 = buffer;
            buffer = buffer.offset(1);
            *fresh1147 = c_570;
            if c_570 as c_int == 0xffi32 {
                let fresh1148 = buffer;
                buffer = buffer.offset(1);
                *fresh1148 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(60isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_571: JOCTET = 0;
                put_bits -= 8i32;
                c_571 = (put_buffer >> put_bits) as JOCTET;
                let fresh1149 = buffer;
                buffer = buffer.offset(1);
                *fresh1149 = c_571;
                if c_571 as c_int == 0xffi32 {
                    let fresh1150 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1150 = 0i32 as JOCTET
                }
                let mut c_572: JOCTET = 0;
                put_bits -= 8i32;
                c_572 = (put_buffer >> put_bits) as JOCTET;
                let fresh1151 = buffer;
                buffer = buffer.offset(1);
                *fresh1151 = c_572;
                if c_572 as c_int == 0xffi32 {
                    let fresh1152 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1152 = 0i32 as JOCTET
                }
                let mut c_573: JOCTET = 0;
                put_bits -= 8i32;
                c_573 = (put_buffer >> put_bits) as JOCTET;
                let fresh1153 = buffer;
                buffer = buffer.offset(1);
                *fresh1153 = c_573;
                if c_573 as c_int == 0xffi32 {
                    let fresh1154 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1154 = 0i32 as JOCTET
                }
                let mut c_574: JOCTET = 0;
                put_bits -= 8i32;
                c_574 = (put_buffer >> put_bits) as JOCTET;
                let fresh1155 = buffer;
                buffer = buffer.offset(1);
                *fresh1155 = c_574;
                if c_574 as c_int == 0xffi32 {
                    let fresh1156 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1156 = 0i32 as JOCTET
                }
                let mut c_575: JOCTET = 0;
                put_bits -= 8i32;
                c_575 = (put_buffer >> put_bits) as JOCTET;
                let fresh1157 = buffer;
                buffer = buffer.offset(1);
                *fresh1157 = c_575;
                if c_575 as c_int == 0xffi32 {
                    let fresh1158 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1158 = 0i32 as JOCTET
                }
                let mut c_576: JOCTET = 0;
                put_bits -= 8i32;
                c_576 = (put_buffer >> put_bits) as JOCTET;
                let fresh1159 = buffer;
                buffer = buffer.offset(1);
                *fresh1159 = c_576;
                if c_576 as c_int == 0xffi32 {
                    let fresh1160 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1160 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_577: JOCTET = 0;
            put_bits -= 8i32;
            c_577 = (put_buffer >> put_bits) as JOCTET;
            let fresh1161 = buffer;
            buffer = buffer.offset(1);
            *fresh1161 = c_577;
            if c_577 as c_int == 0xffi32 {
                let fresh1162 = buffer;
                buffer = buffer.offset(1);
                *fresh1162 = 0i32 as JOCTET
            }
            let mut c_578: JOCTET = 0;
            put_bits -= 8i32;
            c_578 = (put_buffer >> put_bits) as JOCTET;
            let fresh1163 = buffer;
            buffer = buffer.offset(1);
            *fresh1163 = c_578;
            if c_578 as c_int == 0xffi32 {
                let fresh1164 = buffer;
                buffer = buffer.offset(1);
                *fresh1164 = 0i32 as JOCTET
            }
            let mut c_579: JOCTET = 0;
            put_bits -= 8i32;
            c_579 = (put_buffer >> put_bits) as JOCTET;
            let fresh1165 = buffer;
            buffer = buffer.offset(1);
            *fresh1165 = c_579;
            if c_579 as c_int == 0xffi32 {
                let fresh1166 = buffer;
                buffer = buffer.offset(1);
                *fresh1166 = 0i32 as JOCTET
            }
            let mut c_580: JOCTET = 0;
            put_bits -= 8i32;
            c_580 = (put_buffer >> put_bits) as JOCTET;
            let fresh1167 = buffer;
            buffer = buffer.offset(1);
            *fresh1167 = c_580;
            if c_580 as c_int == 0xffi32 {
                let fresh1168 = buffer;
                buffer = buffer.offset(1);
                *fresh1168 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(61isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_581: JOCTET = 0;
                put_bits -= 8i32;
                c_581 = (put_buffer >> put_bits) as JOCTET;
                let fresh1169 = buffer;
                buffer = buffer.offset(1);
                *fresh1169 = c_581;
                if c_581 as c_int == 0xffi32 {
                    let fresh1170 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1170 = 0i32 as JOCTET
                }
                let mut c_582: JOCTET = 0;
                put_bits -= 8i32;
                c_582 = (put_buffer >> put_bits) as JOCTET;
                let fresh1171 = buffer;
                buffer = buffer.offset(1);
                *fresh1171 = c_582;
                if c_582 as c_int == 0xffi32 {
                    let fresh1172 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1172 = 0i32 as JOCTET
                }
                let mut c_583: JOCTET = 0;
                put_bits -= 8i32;
                c_583 = (put_buffer >> put_bits) as JOCTET;
                let fresh1173 = buffer;
                buffer = buffer.offset(1);
                *fresh1173 = c_583;
                if c_583 as c_int == 0xffi32 {
                    let fresh1174 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1174 = 0i32 as JOCTET
                }
                let mut c_584: JOCTET = 0;
                put_bits -= 8i32;
                c_584 = (put_buffer >> put_bits) as JOCTET;
                let fresh1175 = buffer;
                buffer = buffer.offset(1);
                *fresh1175 = c_584;
                if c_584 as c_int == 0xffi32 {
                    let fresh1176 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1176 = 0i32 as JOCTET
                }
                let mut c_585: JOCTET = 0;
                put_bits -= 8i32;
                c_585 = (put_buffer >> put_bits) as JOCTET;
                let fresh1177 = buffer;
                buffer = buffer.offset(1);
                *fresh1177 = c_585;
                if c_585 as c_int == 0xffi32 {
                    let fresh1178 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1178 = 0i32 as JOCTET
                }
                let mut c_586: JOCTET = 0;
                put_bits -= 8i32;
                c_586 = (put_buffer >> put_bits) as JOCTET;
                let fresh1179 = buffer;
                buffer = buffer.offset(1);
                *fresh1179 = c_586;
                if c_586 as c_int == 0xffi32 {
                    let fresh1180 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1180 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_587: JOCTET = 0;
            put_bits -= 8i32;
            c_587 = (put_buffer >> put_bits) as JOCTET;
            let fresh1181 = buffer;
            buffer = buffer.offset(1);
            *fresh1181 = c_587;
            if c_587 as c_int == 0xffi32 {
                let fresh1182 = buffer;
                buffer = buffer.offset(1);
                *fresh1182 = 0i32 as JOCTET
            }
            let mut c_588: JOCTET = 0;
            put_bits -= 8i32;
            c_588 = (put_buffer >> put_bits) as JOCTET;
            let fresh1183 = buffer;
            buffer = buffer.offset(1);
            *fresh1183 = c_588;
            if c_588 as c_int == 0xffi32 {
                let fresh1184 = buffer;
                buffer = buffer.offset(1);
                *fresh1184 = 0i32 as JOCTET
            }
            let mut c_589: JOCTET = 0;
            put_bits -= 8i32;
            c_589 = (put_buffer >> put_bits) as JOCTET;
            let fresh1185 = buffer;
            buffer = buffer.offset(1);
            *fresh1185 = c_589;
            if c_589 as c_int == 0xffi32 {
                let fresh1186 = buffer;
                buffer = buffer.offset(1);
                *fresh1186 = 0i32 as JOCTET
            }
            let mut c_590: JOCTET = 0;
            put_bits -= 8i32;
            c_590 = (put_buffer >> put_bits) as JOCTET;
            let fresh1187 = buffer;
            buffer = buffer.offset(1);
            *fresh1187 = c_590;
            if c_590 as c_int == 0xffi32 {
                let fresh1188 = buffer;
                buffer = buffer.offset(1);
                *fresh1188 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(54isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_591: JOCTET = 0;
                put_bits -= 8i32;
                c_591 = (put_buffer >> put_bits) as JOCTET;
                let fresh1189 = buffer;
                buffer = buffer.offset(1);
                *fresh1189 = c_591;
                if c_591 as c_int == 0xffi32 {
                    let fresh1190 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1190 = 0i32 as JOCTET
                }
                let mut c_592: JOCTET = 0;
                put_bits -= 8i32;
                c_592 = (put_buffer >> put_bits) as JOCTET;
                let fresh1191 = buffer;
                buffer = buffer.offset(1);
                *fresh1191 = c_592;
                if c_592 as c_int == 0xffi32 {
                    let fresh1192 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1192 = 0i32 as JOCTET
                }
                let mut c_593: JOCTET = 0;
                put_bits -= 8i32;
                c_593 = (put_buffer >> put_bits) as JOCTET;
                let fresh1193 = buffer;
                buffer = buffer.offset(1);
                *fresh1193 = c_593;
                if c_593 as c_int == 0xffi32 {
                    let fresh1194 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1194 = 0i32 as JOCTET
                }
                let mut c_594: JOCTET = 0;
                put_bits -= 8i32;
                c_594 = (put_buffer >> put_bits) as JOCTET;
                let fresh1195 = buffer;
                buffer = buffer.offset(1);
                *fresh1195 = c_594;
                if c_594 as c_int == 0xffi32 {
                    let fresh1196 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1196 = 0i32 as JOCTET
                }
                let mut c_595: JOCTET = 0;
                put_bits -= 8i32;
                c_595 = (put_buffer >> put_bits) as JOCTET;
                let fresh1197 = buffer;
                buffer = buffer.offset(1);
                *fresh1197 = c_595;
                if c_595 as c_int == 0xffi32 {
                    let fresh1198 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1198 = 0i32 as JOCTET
                }
                let mut c_596: JOCTET = 0;
                put_bits -= 8i32;
                c_596 = (put_buffer >> put_bits) as JOCTET;
                let fresh1199 = buffer;
                buffer = buffer.offset(1);
                *fresh1199 = c_596;
                if c_596 as c_int == 0xffi32 {
                    let fresh1200 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1200 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_597: JOCTET = 0;
            put_bits -= 8i32;
            c_597 = (put_buffer >> put_bits) as JOCTET;
            let fresh1201 = buffer;
            buffer = buffer.offset(1);
            *fresh1201 = c_597;
            if c_597 as c_int == 0xffi32 {
                let fresh1202 = buffer;
                buffer = buffer.offset(1);
                *fresh1202 = 0i32 as JOCTET
            }
            let mut c_598: JOCTET = 0;
            put_bits -= 8i32;
            c_598 = (put_buffer >> put_bits) as JOCTET;
            let fresh1203 = buffer;
            buffer = buffer.offset(1);
            *fresh1203 = c_598;
            if c_598 as c_int == 0xffi32 {
                let fresh1204 = buffer;
                buffer = buffer.offset(1);
                *fresh1204 = 0i32 as JOCTET
            }
            let mut c_599: JOCTET = 0;
            put_bits -= 8i32;
            c_599 = (put_buffer >> put_bits) as JOCTET;
            let fresh1205 = buffer;
            buffer = buffer.offset(1);
            *fresh1205 = c_599;
            if c_599 as c_int == 0xffi32 {
                let fresh1206 = buffer;
                buffer = buffer.offset(1);
                *fresh1206 = 0i32 as JOCTET
            }
            let mut c_600: JOCTET = 0;
            put_bits -= 8i32;
            c_600 = (put_buffer >> put_bits) as JOCTET;
            let fresh1207 = buffer;
            buffer = buffer.offset(1);
            *fresh1207 = c_600;
            if c_600 as c_int == 0xffi32 {
                let fresh1208 = buffer;
                buffer = buffer.offset(1);
                *fresh1208 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(47isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_601: JOCTET = 0;
                put_bits -= 8i32;
                c_601 = (put_buffer >> put_bits) as JOCTET;
                let fresh1209 = buffer;
                buffer = buffer.offset(1);
                *fresh1209 = c_601;
                if c_601 as c_int == 0xffi32 {
                    let fresh1210 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1210 = 0i32 as JOCTET
                }
                let mut c_602: JOCTET = 0;
                put_bits -= 8i32;
                c_602 = (put_buffer >> put_bits) as JOCTET;
                let fresh1211 = buffer;
                buffer = buffer.offset(1);
                *fresh1211 = c_602;
                if c_602 as c_int == 0xffi32 {
                    let fresh1212 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1212 = 0i32 as JOCTET
                }
                let mut c_603: JOCTET = 0;
                put_bits -= 8i32;
                c_603 = (put_buffer >> put_bits) as JOCTET;
                let fresh1213 = buffer;
                buffer = buffer.offset(1);
                *fresh1213 = c_603;
                if c_603 as c_int == 0xffi32 {
                    let fresh1214 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1214 = 0i32 as JOCTET
                }
                let mut c_604: JOCTET = 0;
                put_bits -= 8i32;
                c_604 = (put_buffer >> put_bits) as JOCTET;
                let fresh1215 = buffer;
                buffer = buffer.offset(1);
                *fresh1215 = c_604;
                if c_604 as c_int == 0xffi32 {
                    let fresh1216 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1216 = 0i32 as JOCTET
                }
                let mut c_605: JOCTET = 0;
                put_bits -= 8i32;
                c_605 = (put_buffer >> put_bits) as JOCTET;
                let fresh1217 = buffer;
                buffer = buffer.offset(1);
                *fresh1217 = c_605;
                if c_605 as c_int == 0xffi32 {
                    let fresh1218 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1218 = 0i32 as JOCTET
                }
                let mut c_606: JOCTET = 0;
                put_bits -= 8i32;
                c_606 = (put_buffer >> put_bits) as JOCTET;
                let fresh1219 = buffer;
                buffer = buffer.offset(1);
                *fresh1219 = c_606;
                if c_606 as c_int == 0xffi32 {
                    let fresh1220 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1220 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_607: JOCTET = 0;
            put_bits -= 8i32;
            c_607 = (put_buffer >> put_bits) as JOCTET;
            let fresh1221 = buffer;
            buffer = buffer.offset(1);
            *fresh1221 = c_607;
            if c_607 as c_int == 0xffi32 {
                let fresh1222 = buffer;
                buffer = buffer.offset(1);
                *fresh1222 = 0i32 as JOCTET
            }
            let mut c_608: JOCTET = 0;
            put_bits -= 8i32;
            c_608 = (put_buffer >> put_bits) as JOCTET;
            let fresh1223 = buffer;
            buffer = buffer.offset(1);
            *fresh1223 = c_608;
            if c_608 as c_int == 0xffi32 {
                let fresh1224 = buffer;
                buffer = buffer.offset(1);
                *fresh1224 = 0i32 as JOCTET
            }
            let mut c_609: JOCTET = 0;
            put_bits -= 8i32;
            c_609 = (put_buffer >> put_bits) as JOCTET;
            let fresh1225 = buffer;
            buffer = buffer.offset(1);
            *fresh1225 = c_609;
            if c_609 as c_int == 0xffi32 {
                let fresh1226 = buffer;
                buffer = buffer.offset(1);
                *fresh1226 = 0i32 as JOCTET
            }
            let mut c_610: JOCTET = 0;
            put_bits -= 8i32;
            c_610 = (put_buffer >> put_bits) as JOCTET;
            let fresh1227 = buffer;
            buffer = buffer.offset(1);
            *fresh1227 = c_610;
            if c_610 as c_int == 0xffi32 {
                let fresh1228 = buffer;
                buffer = buffer.offset(1);
                *fresh1228 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(55isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_611: JOCTET = 0;
                put_bits -= 8i32;
                c_611 = (put_buffer >> put_bits) as JOCTET;
                let fresh1229 = buffer;
                buffer = buffer.offset(1);
                *fresh1229 = c_611;
                if c_611 as c_int == 0xffi32 {
                    let fresh1230 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1230 = 0i32 as JOCTET
                }
                let mut c_612: JOCTET = 0;
                put_bits -= 8i32;
                c_612 = (put_buffer >> put_bits) as JOCTET;
                let fresh1231 = buffer;
                buffer = buffer.offset(1);
                *fresh1231 = c_612;
                if c_612 as c_int == 0xffi32 {
                    let fresh1232 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1232 = 0i32 as JOCTET
                }
                let mut c_613: JOCTET = 0;
                put_bits -= 8i32;
                c_613 = (put_buffer >> put_bits) as JOCTET;
                let fresh1233 = buffer;
                buffer = buffer.offset(1);
                *fresh1233 = c_613;
                if c_613 as c_int == 0xffi32 {
                    let fresh1234 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1234 = 0i32 as JOCTET
                }
                let mut c_614: JOCTET = 0;
                put_bits -= 8i32;
                c_614 = (put_buffer >> put_bits) as JOCTET;
                let fresh1235 = buffer;
                buffer = buffer.offset(1);
                *fresh1235 = c_614;
                if c_614 as c_int == 0xffi32 {
                    let fresh1236 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1236 = 0i32 as JOCTET
                }
                let mut c_615: JOCTET = 0;
                put_bits -= 8i32;
                c_615 = (put_buffer >> put_bits) as JOCTET;
                let fresh1237 = buffer;
                buffer = buffer.offset(1);
                *fresh1237 = c_615;
                if c_615 as c_int == 0xffi32 {
                    let fresh1238 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1238 = 0i32 as JOCTET
                }
                let mut c_616: JOCTET = 0;
                put_bits -= 8i32;
                c_616 = (put_buffer >> put_bits) as JOCTET;
                let fresh1239 = buffer;
                buffer = buffer.offset(1);
                *fresh1239 = c_616;
                if c_616 as c_int == 0xffi32 {
                    let fresh1240 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1240 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_617: JOCTET = 0;
            put_bits -= 8i32;
            c_617 = (put_buffer >> put_bits) as JOCTET;
            let fresh1241 = buffer;
            buffer = buffer.offset(1);
            *fresh1241 = c_617;
            if c_617 as c_int == 0xffi32 {
                let fresh1242 = buffer;
                buffer = buffer.offset(1);
                *fresh1242 = 0i32 as JOCTET
            }
            let mut c_618: JOCTET = 0;
            put_bits -= 8i32;
            c_618 = (put_buffer >> put_bits) as JOCTET;
            let fresh1243 = buffer;
            buffer = buffer.offset(1);
            *fresh1243 = c_618;
            if c_618 as c_int == 0xffi32 {
                let fresh1244 = buffer;
                buffer = buffer.offset(1);
                *fresh1244 = 0i32 as JOCTET
            }
            let mut c_619: JOCTET = 0;
            put_bits -= 8i32;
            c_619 = (put_buffer >> put_bits) as JOCTET;
            let fresh1245 = buffer;
            buffer = buffer.offset(1);
            *fresh1245 = c_619;
            if c_619 as c_int == 0xffi32 {
                let fresh1246 = buffer;
                buffer = buffer.offset(1);
                *fresh1246 = 0i32 as JOCTET
            }
            let mut c_620: JOCTET = 0;
            put_bits -= 8i32;
            c_620 = (put_buffer >> put_bits) as JOCTET;
            let fresh1247 = buffer;
            buffer = buffer.offset(1);
            *fresh1247 = c_620;
            if c_620 as c_int == 0xffi32 {
                let fresh1248 = buffer;
                buffer = buffer.offset(1);
                *fresh1248 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(62isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_621: JOCTET = 0;
                put_bits -= 8i32;
                c_621 = (put_buffer >> put_bits) as JOCTET;
                let fresh1249 = buffer;
                buffer = buffer.offset(1);
                *fresh1249 = c_621;
                if c_621 as c_int == 0xffi32 {
                    let fresh1250 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1250 = 0i32 as JOCTET
                }
                let mut c_622: JOCTET = 0;
                put_bits -= 8i32;
                c_622 = (put_buffer >> put_bits) as JOCTET;
                let fresh1251 = buffer;
                buffer = buffer.offset(1);
                *fresh1251 = c_622;
                if c_622 as c_int == 0xffi32 {
                    let fresh1252 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1252 = 0i32 as JOCTET
                }
                let mut c_623: JOCTET = 0;
                put_bits -= 8i32;
                c_623 = (put_buffer >> put_bits) as JOCTET;
                let fresh1253 = buffer;
                buffer = buffer.offset(1);
                *fresh1253 = c_623;
                if c_623 as c_int == 0xffi32 {
                    let fresh1254 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1254 = 0i32 as JOCTET
                }
                let mut c_624: JOCTET = 0;
                put_bits -= 8i32;
                c_624 = (put_buffer >> put_bits) as JOCTET;
                let fresh1255 = buffer;
                buffer = buffer.offset(1);
                *fresh1255 = c_624;
                if c_624 as c_int == 0xffi32 {
                    let fresh1256 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1256 = 0i32 as JOCTET
                }
                let mut c_625: JOCTET = 0;
                put_bits -= 8i32;
                c_625 = (put_buffer >> put_bits) as JOCTET;
                let fresh1257 = buffer;
                buffer = buffer.offset(1);
                *fresh1257 = c_625;
                if c_625 as c_int == 0xffi32 {
                    let fresh1258 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1258 = 0i32 as JOCTET
                }
                let mut c_626: JOCTET = 0;
                put_bits -= 8i32;
                c_626 = (put_buffer >> put_bits) as JOCTET;
                let fresh1259 = buffer;
                buffer = buffer.offset(1);
                *fresh1259 = c_626;
                if c_626 as c_int == 0xffi32 {
                    let fresh1260 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1260 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_627: JOCTET = 0;
            put_bits -= 8i32;
            c_627 = (put_buffer >> put_bits) as JOCTET;
            let fresh1261 = buffer;
            buffer = buffer.offset(1);
            *fresh1261 = c_627;
            if c_627 as c_int == 0xffi32 {
                let fresh1262 = buffer;
                buffer = buffer.offset(1);
                *fresh1262 = 0i32 as JOCTET
            }
            let mut c_628: JOCTET = 0;
            put_bits -= 8i32;
            c_628 = (put_buffer >> put_bits) as JOCTET;
            let fresh1263 = buffer;
            buffer = buffer.offset(1);
            *fresh1263 = c_628;
            if c_628 as c_int == 0xffi32 {
                let fresh1264 = buffer;
                buffer = buffer.offset(1);
                *fresh1264 = 0i32 as JOCTET
            }
            let mut c_629: JOCTET = 0;
            put_bits -= 8i32;
            c_629 = (put_buffer >> put_bits) as JOCTET;
            let fresh1265 = buffer;
            buffer = buffer.offset(1);
            *fresh1265 = c_629;
            if c_629 as c_int == 0xffi32 {
                let fresh1266 = buffer;
                buffer = buffer.offset(1);
                *fresh1266 = 0i32 as JOCTET
            }
            let mut c_630: JOCTET = 0;
            put_bits -= 8i32;
            c_630 = (put_buffer >> put_bits) as JOCTET;
            let fresh1267 = buffer;
            buffer = buffer.offset(1);
            *fresh1267 = c_630;
            if c_630 as c_int == 0xffi32 {
                let fresh1268 = buffer;
                buffer = buffer.offset(1);
                *fresh1268 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    temp = *block.offset(63isize) as c_int;
    if temp == 0i32 {
        r += 1
    } else {
        temp2 = temp;
        temp3 = temp
            >> (CHAR_BIT as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                .wrapping_sub(1i32 as c_ulong);
        temp ^= temp3;
        temp -= temp3;
        temp2 += temp3;
        nbits = jpeg_nbits_table[temp as usize] as c_int;
        while r > 15i32 {
            if put_bits > 47i32 {
                let mut c_631: JOCTET = 0;
                put_bits -= 8i32;
                c_631 = (put_buffer >> put_bits) as JOCTET;
                let fresh1269 = buffer;
                buffer = buffer.offset(1);
                *fresh1269 = c_631;
                if c_631 as c_int == 0xffi32 {
                    let fresh1270 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1270 = 0i32 as JOCTET
                }
                let mut c_632: JOCTET = 0;
                put_bits -= 8i32;
                c_632 = (put_buffer >> put_bits) as JOCTET;
                let fresh1271 = buffer;
                buffer = buffer.offset(1);
                *fresh1271 = c_632;
                if c_632 as c_int == 0xffi32 {
                    let fresh1272 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1272 = 0i32 as JOCTET
                }
                let mut c_633: JOCTET = 0;
                put_bits -= 8i32;
                c_633 = (put_buffer >> put_bits) as JOCTET;
                let fresh1273 = buffer;
                buffer = buffer.offset(1);
                *fresh1273 = c_633;
                if c_633 as c_int == 0xffi32 {
                    let fresh1274 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1274 = 0i32 as JOCTET
                }
                let mut c_634: JOCTET = 0;
                put_bits -= 8i32;
                c_634 = (put_buffer >> put_bits) as JOCTET;
                let fresh1275 = buffer;
                buffer = buffer.offset(1);
                *fresh1275 = c_634;
                if c_634 as c_int == 0xffi32 {
                    let fresh1276 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1276 = 0i32 as JOCTET
                }
                let mut c_635: JOCTET = 0;
                put_bits -= 8i32;
                c_635 = (put_buffer >> put_bits) as JOCTET;
                let fresh1277 = buffer;
                buffer = buffer.offset(1);
                *fresh1277 = c_635;
                if c_635 as c_int == 0xffi32 {
                    let fresh1278 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1278 = 0i32 as JOCTET
                }
                let mut c_636: JOCTET = 0;
                put_bits -= 8i32;
                c_636 = (put_buffer >> put_bits) as JOCTET;
                let fresh1279 = buffer;
                buffer = buffer.offset(1);
                *fresh1279 = c_636;
                if c_636 as c_int == 0xffi32 {
                    let fresh1280 = buffer;
                    buffer = buffer.offset(1);
                    *fresh1280 = 0i32 as JOCTET
                }
            }
            put_bits += size_0xf0;
            put_buffer = put_buffer << size_0xf0 | code_0xf0 as c_ulong;
            r -= 16i32
        }
        temp3 = (r << 4i32) + nbits;
        code = (*actbl).ehufco[temp3 as usize] as c_int;
        size = (*actbl).ehufsi[temp3 as usize] as c_int;
        temp2 = (temp2 as c_long & ((1i32 as JLONG) << nbits) - 1i32 as c_long) as c_int;
        if put_bits > 31i32 {
            let mut c_637: JOCTET = 0;
            put_bits -= 8i32;
            c_637 = (put_buffer >> put_bits) as JOCTET;
            let fresh1281 = buffer;
            buffer = buffer.offset(1);
            *fresh1281 = c_637;
            if c_637 as c_int == 0xffi32 {
                let fresh1282 = buffer;
                buffer = buffer.offset(1);
                *fresh1282 = 0i32 as JOCTET
            }
            let mut c_638: JOCTET = 0;
            put_bits -= 8i32;
            c_638 = (put_buffer >> put_bits) as JOCTET;
            let fresh1283 = buffer;
            buffer = buffer.offset(1);
            *fresh1283 = c_638;
            if c_638 as c_int == 0xffi32 {
                let fresh1284 = buffer;
                buffer = buffer.offset(1);
                *fresh1284 = 0i32 as JOCTET
            }
            let mut c_639: JOCTET = 0;
            put_bits -= 8i32;
            c_639 = (put_buffer >> put_bits) as JOCTET;
            let fresh1285 = buffer;
            buffer = buffer.offset(1);
            *fresh1285 = c_639;
            if c_639 as c_int == 0xffi32 {
                let fresh1286 = buffer;
                buffer = buffer.offset(1);
                *fresh1286 = 0i32 as JOCTET
            }
            let mut c_640: JOCTET = 0;
            put_bits -= 8i32;
            c_640 = (put_buffer >> put_bits) as JOCTET;
            let fresh1287 = buffer;
            buffer = buffer.offset(1);
            *fresh1287 = c_640;
            if c_640 as c_int == 0xffi32 {
                let fresh1288 = buffer;
                buffer = buffer.offset(1);
                *fresh1288 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong;
        put_bits += nbits;
        put_buffer = put_buffer << nbits | temp2 as c_ulong;
        r = 0i32
    }
    if r > 0i32 {
        code = (*actbl).ehufco[0usize] as c_int;
        size = (*actbl).ehufsi[0usize] as c_int;
        if put_bits > 47i32 {
            let mut c_641: JOCTET = 0;
            put_bits -= 8i32;
            c_641 = (put_buffer >> put_bits) as JOCTET;
            let fresh1289 = buffer;
            buffer = buffer.offset(1);
            *fresh1289 = c_641;
            if c_641 as c_int == 0xffi32 {
                let fresh1290 = buffer;
                buffer = buffer.offset(1);
                *fresh1290 = 0i32 as JOCTET
            }
            let mut c_642: JOCTET = 0;
            put_bits -= 8i32;
            c_642 = (put_buffer >> put_bits) as JOCTET;
            let fresh1291 = buffer;
            buffer = buffer.offset(1);
            *fresh1291 = c_642;
            if c_642 as c_int == 0xffi32 {
                let fresh1292 = buffer;
                buffer = buffer.offset(1);
                *fresh1292 = 0i32 as JOCTET
            }
            let mut c_643: JOCTET = 0;
            put_bits -= 8i32;
            c_643 = (put_buffer >> put_bits) as JOCTET;
            let fresh1293 = buffer;
            buffer = buffer.offset(1);
            *fresh1293 = c_643;
            if c_643 as c_int == 0xffi32 {
                let fresh1294 = buffer;
                buffer = buffer.offset(1);
                *fresh1294 = 0i32 as JOCTET
            }
            let mut c_644: JOCTET = 0;
            put_bits -= 8i32;
            c_644 = (put_buffer >> put_bits) as JOCTET;
            let fresh1295 = buffer;
            buffer = buffer.offset(1);
            *fresh1295 = c_644;
            if c_644 as c_int == 0xffi32 {
                let fresh1296 = buffer;
                buffer = buffer.offset(1);
                *fresh1296 = 0i32 as JOCTET
            }
            let mut c_645: JOCTET = 0;
            put_bits -= 8i32;
            c_645 = (put_buffer >> put_bits) as JOCTET;
            let fresh1297 = buffer;
            buffer = buffer.offset(1);
            *fresh1297 = c_645;
            if c_645 as c_int == 0xffi32 {
                let fresh1298 = buffer;
                buffer = buffer.offset(1);
                *fresh1298 = 0i32 as JOCTET
            }
            let mut c_646: JOCTET = 0;
            put_bits -= 8i32;
            c_646 = (put_buffer >> put_bits) as JOCTET;
            let fresh1299 = buffer;
            buffer = buffer.offset(1);
            *fresh1299 = c_646;
            if c_646 as c_int == 0xffi32 {
                let fresh1300 = buffer;
                buffer = buffer.offset(1);
                *fresh1300 = 0i32 as JOCTET
            }
        }
        put_bits += size;
        put_buffer = put_buffer << size | code as c_ulong
    }
    (*state).cur.put_buffer = put_buffer;
    (*state).cur.put_bits = put_bits;
    if 0 != localbuf {
        bytes = buffer.wrapping_offset_from(_buffer.as_mut_ptr()) as c_long as size_t;
        buffer = _buffer.as_mut_ptr();
        while bytes > 0i32 as c_ulong {
            bytestocopy = if bytes < (*state).free_in_buffer {
                bytes
            } else {
                (*state).free_in_buffer
            };
            memcpy(
                (*state).next_output_byte as *mut c_void,
                buffer as *const c_void,
                bytestocopy,
            );
            (*state).next_output_byte = (*state).next_output_byte.offset(bytestocopy as isize);
            buffer = buffer.offset(bytestocopy as isize);
            (*state).free_in_buffer =
                ((*state).free_in_buffer as c_ulong).wrapping_sub(bytestocopy) as size_t as size_t;
            if (*state).free_in_buffer == 0i32 as c_ulong {
                if 0 == dump_buffer(state) {
                    return FALSE;
                }
            }
            bytes = (bytes as c_ulong).wrapping_sub(bytestocopy) as size_t as size_t
        }
    } else {
        (*state).free_in_buffer = ((*state).free_in_buffer as c_ulong).wrapping_sub(
            buffer.wrapping_offset_from((*state).next_output_byte) as c_long as c_ulong,
        ) as size_t as size_t;
        (*state).next_output_byte = buffer
    }
    return TRUE;
}
/*
 * Emit a restart marker & resynchronize predictions.
 */
unsafe extern "C" fn emit_restart(
    mut state: *mut working_state,
    mut restart_num: c_int,
) -> boolean {
    let mut ci: c_int = 0;
    if 0 == flush_bits(state) {
        return FALSE;
    }
    let fresh1301 = (*state).next_output_byte;
    (*state).next_output_byte = (*state).next_output_byte.offset(1);
    *fresh1301 = 0xffi32 as JOCTET;
    (*state).free_in_buffer = (*state).free_in_buffer.wrapping_sub(1);
    if (*state).free_in_buffer == 0i32 as c_ulong {
        if 0 == dump_buffer(state) {
            return 0i32;
        }
    }
    let fresh1302 = (*state).next_output_byte;
    (*state).next_output_byte = (*state).next_output_byte.offset(1);
    *fresh1302 = (0xd0i32 + restart_num) as JOCTET;
    (*state).free_in_buffer = (*state).free_in_buffer.wrapping_sub(1);
    if (*state).free_in_buffer == 0i32 as c_ulong {
        if 0 == dump_buffer(state) {
            return 0i32;
        }
    }
    ci = 0i32;
    while ci < (*(*state).cinfo).comps_in_scan {
        (*state).cur.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    return TRUE;
}
/* Forward declarations */

/*
 * Encode and output one MCU's worth of Huffman-compressed coefficients.
 */
unsafe extern "C" fn encode_mcu_huff(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut state: working_state = working_state {
        next_output_byte: 0 as *mut JOCTET,
        free_in_buffer: 0,
        cur: savable_state {
            put_buffer: 0,
            put_bits: 0,
            last_dc_val: [0; 4],
        },
        cinfo: 0 as *mut jpeg_compress_struct,
    };
    let mut blkn: c_int = 0;
    let mut ci: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    state.next_output_byte = (*(*cinfo).dest).next_output_byte;
    state.free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    state.cur = (*entropy).saved;
    state.cinfo = cinfo;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if 0 == emit_restart(&mut state, (*entropy).next_restart_num) {
                return FALSE;
            }
        }
    }
    if 0 != (*entropy).simd {
        blkn = 0i32;
        while blkn < (*cinfo).blocks_in_MCU {
            ci = (*cinfo).MCU_membership[blkn as usize];
            compptr = (*cinfo).cur_comp_info[ci as usize];
            if 0 == encode_one_block_simd(
                &mut state,
                (*(*MCU_data.offset(blkn as isize)).offset(0isize)).as_mut_ptr(),
                state.cur.last_dc_val[ci as usize],
                (*entropy).dc_derived_tbls[(*compptr).dc_tbl_no as usize],
                (*entropy).ac_derived_tbls[(*compptr).ac_tbl_no as usize],
            ) {
                return FALSE;
            }
            state.cur.last_dc_val[ci as usize] =
                (*(*MCU_data.offset(blkn as isize)).offset(0isize))[0usize] as c_int;
            blkn += 1
        }
    } else {
        blkn = 0i32;
        while blkn < (*cinfo).blocks_in_MCU {
            ci = (*cinfo).MCU_membership[blkn as usize];
            compptr = (*cinfo).cur_comp_info[ci as usize];
            if 0 == encode_one_block(
                &mut state,
                (*(*MCU_data.offset(blkn as isize)).offset(0isize)).as_mut_ptr(),
                state.cur.last_dc_val[ci as usize],
                (*entropy).dc_derived_tbls[(*compptr).dc_tbl_no as usize],
                (*entropy).ac_derived_tbls[(*compptr).ac_tbl_no as usize],
            ) {
                return FALSE;
            }
            state.cur.last_dc_val[ci as usize] =
                (*(*MCU_data.offset(blkn as isize)).offset(0isize))[0usize] as c_int;
            blkn += 1
        }
    }
    (*(*cinfo).dest).next_output_byte = state.next_output_byte;
    (*(*cinfo).dest).free_in_buffer = state.free_in_buffer;
    (*entropy).saved = state.cur;
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
 * Finish up at the end of a Huffman-compressed scan.
 */
unsafe extern "C" fn finish_pass_huff(mut cinfo: j_compress_ptr) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut state: working_state = working_state {
        next_output_byte: 0 as *mut JOCTET,
        free_in_buffer: 0,
        cur: savable_state {
            put_buffer: 0,
            put_bits: 0,
            last_dc_val: [0; 4],
        },
        cinfo: 0 as *mut jpeg_compress_struct,
    };
    state.next_output_byte = (*(*cinfo).dest).next_output_byte;
    state.free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    state.cur = (*entropy).saved;
    state.cinfo = cinfo;
    if 0 == flush_bits(&mut state) {
        (*(*cinfo).err).msg_code = JERR_CANT_SUSPEND as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*(*cinfo).dest).next_output_byte = state.next_output_byte;
    (*(*cinfo).dest).free_in_buffer = state.free_in_buffer;
    (*entropy).saved = state.cur;
}
/*
 * Huffman coding optimization.
 *
 * We first scan the supplied data and count the number of uses of each symbol
 * that is to be Huffman-coded. (This process MUST agree with the code above.)
 * Then we build a Huffman coding tree for the observed counts.
 * Symbols which are not needed at all for the particular image are not
 * assigned any code, which saves space in the DHT marker as well as in
 * the compressed data.
 */

/* Process a single block's worth of coefficients */
unsafe extern "C" fn htest_one_block(
    mut cinfo: j_compress_ptr,
    mut block: JCOEFPTR,
    mut last_dc_val: c_int,
    mut dc_counts: *mut c_long,
    mut ac_counts: *mut c_long,
) {
    let mut temp: c_int = 0;
    let mut nbits: c_int = 0;
    let mut k: c_int = 0;
    let mut r: c_int = 0;
    temp = *block.offset(0isize) as c_int - last_dc_val;
    if temp < 0i32 {
        temp = -temp
    }
    nbits = 0i32;
    while 0 != temp {
        nbits += 1;
        temp >>= 1i32
    }
    if nbits > MAX_COEF_BITS + 1i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_DCT_COEF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    let ref mut fresh1303 = *dc_counts.offset(nbits as isize);
    *fresh1303 += 1;
    r = 0i32;
    k = 1i32;
    while k < DCTSIZE2 {
        temp = *block.offset(*jpeg_natural_order.as_ptr().offset(k as isize) as isize) as c_int;
        if temp == 0i32 {
            r += 1
        } else {
            while r > 15i32 {
                let ref mut fresh1304 = *ac_counts.offset(0xf0i32 as isize);
                *fresh1304 += 1;
                r -= 16i32
            }
            if temp < 0i32 {
                temp = -temp
            }
            nbits = 1i32;
            loop {
                temp >>= 1i32;
                if !(0 != temp) {
                    break;
                }
                nbits += 1
            }
            if nbits > MAX_COEF_BITS {
                (*(*cinfo).err).msg_code = JERR_BAD_DCT_COEF as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            let ref mut fresh1305 = *ac_counts.offset(((r << 4i32) + nbits) as isize);
            *fresh1305 += 1;
            r = 0i32
        }
        k += 1
    }
    if r > 0i32 {
        let ref mut fresh1306 = *ac_counts.offset(0isize);
        *fresh1306 += 1
    };
}
/*
 * Trial-encode one MCU's worth of Huffman-compressed coefficients.
 * No data is actually output, so no suspension return is possible.
 */
unsafe extern "C" fn encode_mcu_gather(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut blkn: c_int = 0;
    let mut ci: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                (*entropy).saved.last_dc_val[ci as usize] = 0i32;
                ci += 1
            }
            (*entropy).restarts_to_go = (*cinfo).restart_interval
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        htest_one_block(
            cinfo,
            (*(*MCU_data.offset(blkn as isize)).offset(0isize)).as_mut_ptr(),
            (*entropy).saved.last_dc_val[ci as usize],
            (*entropy).dc_count_ptrs[(*compptr).dc_tbl_no as usize],
            (*entropy).ac_count_ptrs[(*compptr).ac_tbl_no as usize],
        );
        (*entropy).saved.last_dc_val[ci as usize] =
            (*(*MCU_data.offset(blkn as isize)).offset(0isize))[0usize] as c_int;
        blkn += 1
    }
    return TRUE;
}
/* Generate an optimal table definition given the specified counts */

/*
 * Generate the best Huffman code table for the given counts, fill htbl.
 * Note this is also used by jcphuff.c.
 *
 * The JPEG standard requires that no symbol be assigned a codeword of all
 * one bits (so that padding bits added at the end of a compressed segment
 * can't look like a valid code).  Because of the canonical ordering of
 * codewords, this just means that there must be an unused slot in the
 * longest codeword length category.  Annex K (Clause K.2) of
 * Rec. ITU-T T.81 (1992) | ISO/IEC 10918-1:1994 suggests reserving such a slot
 * by pretending that symbol 256 is a valid symbol with count 1.  In theory
 * that's not optimal; giving it count zero but including it in the symbol set
 * anyway should give a better Huffman code.  But the theoretically better code
 * actually seems to come out worse in practice, because it produces more
 * all-ones bytes (which incur stuffed zero bytes in the final file).  In any
 * case the difference is tiny.
 *
 * The JPEG standard requires Huffman codes to be no more than 16 bits long.
 * If some symbols have a very small but nonzero probability, the Huffman tree
 * must be adjusted to meet the code length restriction.  We currently use
 * the adjustment method suggested in JPEG section K.2.  This method is *not*
 * optimal; it may not choose the best possible limited-length code.  But
 * typically only very-low-frequency symbols will be given less-than-optimal
 * lengths, so the code is almost optimal.  Experimental comparisons against
 * an optimal limited-length-code algorithm indicate that the difference is
 * microscopic --- usually less than a hundredth of a percent of total size.
 * So the extra complexity of an optimal algorithm doesn't seem worthwhile.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_gen_optimal_table(
    mut cinfo: j_compress_ptr,
    mut htbl: *mut JHUFF_TBL,
    mut freq: *mut c_long,
) {
    /* bits[k] = # of symbols with code length k */
    let mut bits: [UINT8; 33] = [0; 33];
    /* codesize[k] = code length of symbol k */
    let mut codesize: [c_int; 257] = [0; 257];
    /* next symbol in current branch of tree */
    let mut others: [c_int; 257] = [0; 257];
    let mut c1: c_int = 0;
    let mut c2: c_int = 0;
    let mut p: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut v: c_long = 0;
    memset(
        bits.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[UINT8; 33]>() as c_ulong,
    );
    memset(
        codesize.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_int; 257]>() as c_ulong,
    );
    i = 0i32;
    while i < 257i32 {
        others[i as usize] = -1i32;
        i += 1
    }
    *freq.offset(256isize) = 1i32 as c_long;
    loop {
        c1 = -1i32;
        v = 1000000000i64;
        i = 0i32;
        while i <= 256i32 {
            if 0 != *freq.offset(i as isize) && *freq.offset(i as isize) <= v {
                v = *freq.offset(i as isize);
                c1 = i
            }
            i += 1
        }
        c2 = -1i32;
        v = 1000000000i64;
        i = 0i32;
        while i <= 256i32 {
            if 0 != *freq.offset(i as isize) && *freq.offset(i as isize) <= v && i != c1 {
                v = *freq.offset(i as isize);
                c2 = i
            }
            i += 1
        }
        /* Done if we've merged everything into one frequency */
        if c2 < 0i32 {
            break;
        }
        *freq.offset(c1 as isize) += *freq.offset(c2 as isize);
        *freq.offset(c2 as isize) = 0i32 as c_long;
        codesize[c1 as usize] += 1;
        while others[c1 as usize] >= 0i32 {
            c1 = others[c1 as usize];
            codesize[c1 as usize] += 1
        }
        others[c1 as usize] = c2;
        codesize[c2 as usize] += 1;
        while others[c2 as usize] >= 0i32 {
            c2 = others[c2 as usize];
            codesize[c2 as usize] += 1
        }
    }
    i = 0i32;
    while i <= 256i32 {
        if 0 != codesize[i as usize] {
            if codesize[i as usize] > MAX_CLEN {
                (*(*cinfo).err).msg_code = JERR_HUFF_CLEN_OVERFLOW as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            bits[codesize[i as usize] as usize] =
                bits[codesize[i as usize] as usize].wrapping_add(1)
        }
        i += 1
    }
    i = MAX_CLEN;
    while i > 16i32 {
        while bits[i as usize] as c_int > 0i32 {
            j = i - 2i32;
            while bits[j as usize] as c_int == 0i32 {
                j -= 1
            }
            bits[i as usize] = (bits[i as usize] as c_int - 2i32) as UINT8;
            bits[(i - 1i32) as usize] = bits[(i - 1i32) as usize].wrapping_add(1);
            bits[(j + 1i32) as usize] = (bits[(j + 1i32) as usize] as c_int + 2i32) as UINT8;
            bits[j as usize] = bits[j as usize].wrapping_sub(1)
        }
        i -= 1
    }
    while bits[i as usize] as c_int == 0i32 {
        i -= 1
    }
    bits[i as usize] = bits[i as usize].wrapping_sub(1);
    memcpy(
        (*htbl).bits.as_mut_ptr() as *mut c_void,
        bits.as_mut_ptr() as *const c_void,
        ::std::mem::size_of::<[UINT8; 17]>() as c_ulong,
    );
    p = 0i32;
    i = 1i32;
    while i <= MAX_CLEN {
        j = 0i32;
        while j <= 255i32 {
            if codesize[j as usize] == i {
                (*htbl).huffval[p as usize] = j as UINT8;
                p += 1
            }
            j += 1
        }
        i += 1
    }
    (*htbl).sent_table = FALSE;
}
/* assumed maximum initial code length */
pub const MAX_CLEN: c_int = 32i32;
/*
 * Finish up a statistics-gathering pass and create the new Huffman tables.
 */
unsafe extern "C" fn finish_pass_gather(mut cinfo: j_compress_ptr) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: c_int = 0;
    let mut dctbl: c_int = 0;
    let mut actbl: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut htblptr: *mut *mut JHUFF_TBL = 0 as *mut *mut JHUFF_TBL;
    let mut did_dc: [boolean; 4] = [0; 4];
    let mut did_ac: [boolean; 4] = [0; 4];
    memset(
        did_dc.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[boolean; 4]>() as c_ulong,
    );
    memset(
        did_ac.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[boolean; 4]>() as c_ulong,
    );
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        dctbl = (*compptr).dc_tbl_no;
        actbl = (*compptr).ac_tbl_no;
        if 0 == did_dc[dctbl as usize] {
            htblptr = &mut *(*cinfo)
                .dc_huff_tbl_ptrs
                .as_mut_ptr()
                .offset(dctbl as isize) as *mut *mut JHUFF_TBL;
            if (*htblptr).is_null() {
                *htblptr = jpeg_alloc_huff_table(cinfo as j_common_ptr)
            }
            jpeg_gen_optimal_table(cinfo, *htblptr, (*entropy).dc_count_ptrs[dctbl as usize]);
            did_dc[dctbl as usize] = TRUE
        }
        if 0 == did_ac[actbl as usize] {
            htblptr = &mut *(*cinfo)
                .ac_huff_tbl_ptrs
                .as_mut_ptr()
                .offset(actbl as isize) as *mut *mut JHUFF_TBL;
            if (*htblptr).is_null() {
                *htblptr = jpeg_alloc_huff_table(cinfo as j_common_ptr)
            }
            jpeg_gen_optimal_table(cinfo, *htblptr, (*entropy).ac_count_ptrs[actbl as usize]);
            did_ac[actbl as usize] = TRUE
        }
        ci += 1
    }
}
/* ENTROPY_OPT_SUPPORTED */

/*
 * Module initialization routine for Huffman entropy encoding.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_huff_encoder(mut cinfo: j_compress_ptr) {
    let mut entropy: huff_entropy_ptr = 0 as *mut huff_entropy_encoder;
    let mut i: c_int = 0;
    entropy = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<huff_entropy_encoder>() as c_ulong,
    ) as huff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_encoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass_huff as unsafe extern "C" fn(_: j_compress_ptr, _: boolean) -> ());
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        (*entropy).ac_derived_tbls[i as usize] = NULL as *mut c_derived_tbl;
        (*entropy).dc_derived_tbls[i as usize] = (*entropy).ac_derived_tbls[i as usize];
        (*entropy).ac_count_ptrs[i as usize] = NULL as *mut c_long;
        (*entropy).dc_count_ptrs[i as usize] = (*entropy).ac_count_ptrs[i as usize];
        i += 1
    }
}
