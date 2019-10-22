pub use super::jchuff::{c_derived_tbl, MAX_COEF_BITS};
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
use super::simd::x86_64::jsimd::{
    jsimd_can_convsamp, jsimd_can_convsamp_float, jsimd_can_fdct_float, jsimd_can_fdct_ifast,
    jsimd_can_fdct_islow, jsimd_can_quantize, jsimd_can_quantize_float, jsimd_convsamp,
    jsimd_convsamp_float, jsimd_fdct_float, jsimd_fdct_ifast, jsimd_fdct_islow, jsimd_quantize,
    jsimd_quantize_float,
};
pub use crate::jdct_h::{
    jpeg_fdct_float, jpeg_fdct_ifast, jpeg_fdct_islow, DCTELEM, UDCTELEM, UDCTELEM2,
};
pub use crate::jmorecfg_h::{
    boolean, CENTERJSAMPLE, INT16, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8,
};
pub use crate::jpeg_nbits_table_h::jpeg_nbits_table;
pub use crate::jpegint_h::{
    jpeg_natural_order, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_writer, jpeg_memory_mgr,
    jpeg_progress_mgr, jpeg_scan_info, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, DCTSIZE2, JBLOCK,
    JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
    JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
    JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
    JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
    J_COLOR_SPACE, J_DCT_METHOD, NUM_QUANT_TBLS,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::{abs, ceilf, free, malloc, pow};
use libc::{self, c_double, c_float, c_int, c_uint, c_ulong, c_void, intptr_t};

pub type float_preprocess_method_ptr =
    Option<unsafe extern "C" fn(_: *mut c_float, _: *const JQUANT_TBL) -> ()>;

pub type preprocess_method_ptr =
    Option<unsafe extern "C" fn(_: *mut DCTELEM, _: *const JQUANT_TBL) -> ()>;

pub type my_fdct_ptr = *mut my_fdct_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_fdct_controller {
    pub pub_0: jpeg_forward_dct,
    pub dct: forward_DCT_method_ptr,
    pub convsamp: convsamp_method_ptr,
    pub preprocess: preprocess_method_ptr,
    pub quantize: quantize_method_ptr,
    pub divisors: [*mut DCTELEM; 4],
    pub workspace: *mut DCTELEM,
    pub float_dct: float_DCT_method_ptr,
    pub float_convsamp: float_convsamp_method_ptr,
    pub float_preprocess: float_preprocess_method_ptr,
    pub float_quantize: float_quantize_method_ptr,
    pub float_divisors: [*mut c_float; 4],
    pub float_workspace: *mut c_float,
}

pub type float_quantize_method_ptr =
    Option<unsafe extern "C" fn(_: JCOEFPTR, _: *mut c_float, _: *mut c_float) -> ()>;

pub type float_convsamp_method_ptr =
    Option<unsafe extern "C" fn(_: JSAMPARRAY, _: JDIMENSION, _: *mut c_float) -> ()>;

pub type float_DCT_method_ptr = Option<unsafe extern "C" fn(_: *mut c_float) -> ()>;

pub type quantize_method_ptr =
    Option<unsafe extern "C" fn(_: JCOEFPTR, _: *mut DCTELEM, _: *mut DCTELEM) -> ()>;

pub type convsamp_method_ptr =
    Option<unsafe extern "C" fn(_: JSAMPARRAY, _: JDIMENSION, _: *mut DCTELEM) -> ()>;
/*
 * jcdctmgr.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 1999-2006, MIYASAKA Masaru.
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2011, 2014-2015 D. R. Commander
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains the forward-DCT management logic.
 * This code selects a particular DCT implementation to be used,
 * and it performs related housekeeping chores including coefficient
 * quantization.
 */
/* Private subobject for this module */

pub type forward_DCT_method_ptr = Option<unsafe extern "C" fn(_: *mut DCTELEM) -> ()>;
/*
 * Find the highest bit in an integer through binary search.
 */

unsafe extern "C" fn flss(mut val: UINT16) -> c_int {
    let mut bit: c_int = 0;
    bit = 16i32;
    if val == 0 {
        return 0i32;
    }
    if val as c_int & 0xff00i32 == 0 {
        bit -= 8i32;
        val = ((val as c_int) << 8i32) as UINT16
    }
    if val as c_int & 0xf000i32 == 0 {
        bit -= 4i32;
        val = ((val as c_int) << 4i32) as UINT16
    }
    if val as c_int & 0xc000i32 == 0 {
        bit -= 2i32;
        val = ((val as c_int) << 2i32) as UINT16
    }
    if val as c_int & 0x8000i32 == 0 {
        bit -= 1i32;
        val = ((val as c_int) << 1i32) as UINT16
    }
    return bit;
}
/*
 * Compute values to do a division using reciprocal.
 *
 * This implementation is based on an algorithm described in
 *   "How to optimize for the Pentium family of microprocessors"
 *   (http://www.agner.org/assem/).
 * More information about the basic algorithm can be found in
 * the paper "Integer Division Using Reciprocals" by Robert Alverson.
 *
 * The basic idea is to replace x/d by x * d^-1. In order to store
 * d^-1 with enough precision we shift it left a few places. It turns
 * out that this algoright gives just enough precision, and also fits
 * into DCTELEM:
 *
 *   b = (the number of significant bits in divisor) - 1
 *   r = (word size) + b
 *   f = 2^r / divisor
 *
 * f will not be an integer for most cases, so we need to compensate
 * for the rounding error introduced:
 *
 *   no fractional part:
 *
 *       result = input >> r
 *
 *   fractional part of f < 0.5:
 *
 *       round f down to nearest integer
 *       result = ((input + 1) * f) >> r
 *
 *   fractional part of f > 0.5:
 *
 *       round f up to nearest integer
 *       result = (input * f) >> r
 *
 * This is the original algorithm that gives truncated results. But we
 * want properly rounded results, so we replace "input" with
 * "input + divisor/2".
 *
 * In order to allow SIMD implementations we also tweak the values to
 * allow the same calculation to be made at all times:
 *
 *   dctbl[0] = f rounded to nearest integer
 *   dctbl[1] = divisor / 2 (+ 1 if fractional part of f < 0.5)
 *   dctbl[2] = 1 << ((word size) * 2 - r)
 *   dctbl[3] = r - (word size)
 *
 * dctbl[2] is for stupid instruction sets where the shift operation
 * isn't member wise (e.g. MMX).
 *
 * The reason dctbl[2] and dctbl[3] reduce the shift with (word size)
 * is that most SIMD implementations have a "multiply and store top
 * half" operation.
 *
 * Lastly, we store each of the values in their own table instead
 * of in a consecutive manner, yet again in order to allow SIMD
 * routines.
 */

unsafe extern "C" fn compute_reciprocal(mut divisor: UINT16, mut dtbl: *mut DCTELEM) -> c_int {
    let mut fq: UDCTELEM2 = 0;
    let mut fr: UDCTELEM2 = 0;
    let mut c: UDCTELEM = 0;
    let mut b: c_int = 0;
    let mut r: c_int = 0;
    if divisor as c_int == 1i32 {
        /* divisor == 1 means unquantized, so these reciprocal/correction/shift
         * values will cause the C quantization algorithm to act like the
         * identity function.  Since only the C quantization algorithm is used in
         * these cases, the scale value is irrelevant.
         */
        *dtbl.offset((DCTSIZE2 * 0i32) as isize) = 1i32 as DCTELEM; /* reciprocal */
        *dtbl.offset((DCTSIZE2 * 1i32) as isize) = 0i32 as DCTELEM; /* correction */
        *dtbl.offset((DCTSIZE2 * 2i32) as isize) = 1i32 as DCTELEM; /* scale */
        *dtbl.offset((DCTSIZE2 * 3i32) as isize) = -((::std::mem::size_of::<DCTELEM>() as c_ulong)
            .wrapping_mul(8i32 as c_ulong)
            as DCTELEM as c_int) as DCTELEM; /* shift */
        return 0i32;
    } /* for rounding */
    b = flss(divisor) - 1i32; /* fractional part is < 0.5 */
    r = (::std::mem::size_of::<DCTELEM>() as c_ulong)
        .wrapping_mul(8i32 as c_ulong)
        .wrapping_add(b as c_ulong) as c_int;
    fq = ((1i32 as UDCTELEM2) << r).wrapping_div(divisor as c_uint);
    fr = ((1i32 as UDCTELEM2) << r).wrapping_rem(divisor as c_uint);
    c = (divisor as c_int / 2i32) as UDCTELEM;
    if fr == 0i32 as c_uint {
        /* divisor is power of two */
        /* fq will be one bit too large to fit in DCTELEM, so adjust */
        fq >>= 1i32; /* fractional part is > 0.5 */
        r -= 1
    } else if fr <= (divisor as c_uint).wrapping_div(2u32) {
        c = c.wrapping_add(1)
    } else {
        fq = fq.wrapping_add(1)
    } /* reciprocal */
    *dtbl.offset((DCTSIZE2 * 0i32) as isize) = fq as DCTELEM; /* correction + roundfactor */
    *dtbl.offset((DCTSIZE2 * 1i32) as isize) = c as DCTELEM; /* scale */
    *dtbl.offset((DCTSIZE2 * 2i32) as isize) = (1i32
        << (::std::mem::size_of::<DCTELEM>() as c_ulong)
            .wrapping_mul(8i32 as c_ulong)
            .wrapping_mul(2i32 as c_ulong)
            .wrapping_sub(r as c_ulong)) as DCTELEM; /* shift */
    *dtbl.offset((DCTSIZE2 * 3i32) as isize) = (r as DCTELEM as c_ulong)
        .wrapping_sub((::std::mem::size_of::<DCTELEM>() as c_ulong).wrapping_mul(8i32 as c_ulong))
        as DCTELEM;
    if r <= 16i32 {
        return 0i32;
    } else {
        return 1i32;
    };
}
/*
 * Initialize for a processing pass.
 * Verify that all referenced Q-tables are present, and set up
 * the divisor table for each one.
 * In the current implementation, DCT of all components is done during
 * the first pass, even if only some components will be output in the
 * first scan.  Hence all components should be examined here.
 */

unsafe extern "C" fn start_pass_fdctmgr(mut cinfo: j_compress_ptr) {
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut ci: c_int = 0;
    let mut qtblno: c_int = 0;
    let mut i: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut qtbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut dtbl: *mut DCTELEM = 0 as *mut DCTELEM;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        qtblno = (*compptr).quant_tbl_no;
        /* Make sure specified quantization table is present */
        if qtblno < 0i32
            || qtblno >= NUM_QUANT_TBLS
            || (*cinfo).quant_tbl_ptrs[qtblno as usize].is_null()
        {
            (*(*cinfo).err).msg_code = super::jerror::JERR_NO_QUANT_TABLE as c_int;
            (*(*cinfo).err).msg_parm.i[0] = qtblno;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        qtbl = (*cinfo).quant_tbl_ptrs[qtblno as usize];
        /* Compute divisors for this quant table */
        /* We may do this more than once for same table, but it's not a big deal */
        match (*cinfo).dct_method as c_uint {
            0 => {
                /* For LL&M IDCT method, divisors are equal to raw quantization
                 * coefficients multiplied by 8 (to counteract scaling).
                 */
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        JPOOL_IMAGE,
                        ((DCTSIZE2 * 4i32) as c_ulong)
                            .wrapping_mul(::std::mem::size_of::<DCTELEM>() as c_ulong),
                    ) as *mut DCTELEM
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0i32;
                while i < DCTSIZE2 {
                    if compute_reciprocal(
                        (((*qtbl).quantval[i as usize] as c_int) << 3i32) as UINT16,
                        &mut *dtbl.offset(i as isize),
                    ) == 0
                        && (*fdct).quantize
                            == Some(
                                super::simd::x86_64::jsimd::jsimd_quantize
                                    as unsafe extern "C" fn(
                                        _: JCOEFPTR,
                                        _: *mut DCTELEM,
                                        _: *mut DCTELEM,
                                    )
                                        -> (),
                            )
                    {
                        (*fdct).quantize = Some(
                            quantize
                                as unsafe extern "C" fn(
                                    _: JCOEFPTR,
                                    _: *mut DCTELEM,
                                    _: *mut DCTELEM,
                                ) -> (),
                        )
                    }
                    i += 1
                }
            }
            1 => {
                /* For AA&N IDCT method, divisors are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 * We apply a further scale factor of 8.
                 */
                static mut aanscales: [INT16; 64] = [
                    16384i32 as INT16,
                    22725i32 as INT16,
                    21407i32 as INT16,
                    19266i32 as INT16,
                    16384i32 as INT16,
                    12873i32 as INT16,
                    8867i32 as INT16,
                    4520i32 as INT16,
                    22725i32 as INT16,
                    31521i32 as INT16,
                    29692i32 as INT16,
                    26722i32 as INT16,
                    22725i32 as INT16,
                    17855i32 as INT16,
                    12299i32 as INT16,
                    6270i32 as INT16,
                    21407i32 as INT16,
                    29692i32 as INT16,
                    27969i32 as INT16,
                    25172i32 as INT16,
                    21407i32 as INT16,
                    16819i32 as INT16,
                    11585i32 as INT16,
                    5906i32 as INT16,
                    19266i32 as INT16,
                    26722i32 as INT16,
                    25172i32 as INT16,
                    22654i32 as INT16,
                    19266i32 as INT16,
                    15137i32 as INT16,
                    10426i32 as INT16,
                    5315i32 as INT16,
                    16384i32 as INT16,
                    22725i32 as INT16,
                    21407i32 as INT16,
                    19266i32 as INT16,
                    16384i32 as INT16,
                    12873i32 as INT16,
                    8867i32 as INT16,
                    4520i32 as INT16,
                    12873i32 as INT16,
                    17855i32 as INT16,
                    16819i32 as INT16,
                    15137i32 as INT16,
                    12873i32 as INT16,
                    10114i32 as INT16,
                    6967i32 as INT16,
                    3552i32 as INT16,
                    8867i32 as INT16,
                    12299i32 as INT16,
                    11585i32 as INT16,
                    10426i32 as INT16,
                    8867i32 as INT16,
                    6967i32 as INT16,
                    4799i32 as INT16,
                    2446i32 as INT16,
                    4520i32 as INT16,
                    6270i32 as INT16,
                    5906i32 as INT16,
                    5315i32 as INT16,
                    4520i32 as INT16,
                    3552i32 as INT16,
                    2446i32 as INT16,
                    1247i32 as INT16,
                ];
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        JPOOL_IMAGE,
                        ((DCTSIZE2 * 4i32) as c_ulong)
                            .wrapping_mul(::std::mem::size_of::<DCTELEM>() as c_ulong),
                    ) as *mut DCTELEM
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0i32;
                while i < DCTSIZE2 {
                    if compute_reciprocal(
                        ((*qtbl).quantval[i as usize] as JLONG * aanscales[i as usize] as JLONG
                            + ((1i32 as JLONG) << 14i32 - 3i32 - 1i32)
                            >> 14i32 - 3i32) as UINT16,
                        &mut *dtbl.offset(i as isize),
                    ) == 0
                        && (*fdct).quantize
                            == Some(
                                super::simd::x86_64::jsimd::jsimd_quantize
                                    as unsafe extern "C" fn(
                                        _: JCOEFPTR,
                                        _: *mut DCTELEM,
                                        _: *mut DCTELEM,
                                    )
                                        -> (),
                            )
                    {
                        (*fdct).quantize = Some(
                            quantize
                                as unsafe extern "C" fn(
                                    _: JCOEFPTR,
                                    _: *mut DCTELEM,
                                    _: *mut DCTELEM,
                                ) -> (),
                        )
                    }
                    i += 1
                }
            }
            2 => {
                /* For float AA&N IDCT method, divisors are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 * We apply a further scale factor of 8.
                 * What's actually stored is 1/divisor so that the inner loop can
                 * use a multiplication rather than a division.
                 */
                let mut fdtbl: *mut c_float = 0 as *mut c_float;
                let mut row: c_int = 0;
                let mut col: c_int = 0;
                static mut aanscalefactor: [c_double; 8] = [
                    1.0f64,
                    1.387039845f64,
                    1.306562965f64,
                    1.175875602f64,
                    1.0f64,
                    0.785694958f64,
                    0.541196100f64,
                    0.275899379f64,
                ];
                if (*fdct).float_divisors[qtblno as usize].is_null() {
                    (*fdct).float_divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        JPOOL_IMAGE,
                        (DCTSIZE2 as c_ulong)
                            .wrapping_mul(::std::mem::size_of::<c_float>() as c_ulong),
                    ) as *mut c_float
                }
                fdtbl = (*fdct).float_divisors[qtblno as usize];
                i = 0i32;
                row = 0i32;
                while row < DCTSIZE {
                    col = 0i32;
                    while col < DCTSIZE {
                        *fdtbl.offset(i as isize) = (1.0f64
                            / ((*qtbl).quantval[i as usize] as c_double
                                * aanscalefactor[row as usize]
                                * aanscalefactor[col as usize]
                                * 8.0f64))
                            as c_float;
                        i += 1;
                        col += 1
                    }
                    row += 1
                }
            }
            _ => {
                (*(*cinfo).err).msg_code = super::jerror::JERR_NOT_COMPILED as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn catmull_rom(
    value1: DCTELEM,
    value2: DCTELEM,
    value3: DCTELEM,
    value4: DCTELEM,
    t: c_float,
    mut size: c_int,
) -> c_float {
    let tan1: c_int = (value3 as c_int - value1 as c_int) * size;
    let tan2: c_int = (value4 as c_int - value2 as c_int) * size;
    let t2: c_float = t * t;
    let t3: c_float = t2 * t;
    let f1: c_float = 2.0f32 * t3 - 3.0f32 * t2 + 1.0f32;
    let f2: c_float = -2.0f32 * t3 + 3.0f32 * t2;
    let f3: c_float = t3 - 2.0f32 * t2 + t;
    let f4: c_float = t3 - t2;
    return value2 as c_int as c_float * f1
        + tan1 as c_float * f3
        + value3 as c_int as c_float * f2
        + tan2 as c_float * f4;
}
/* * Prevents visible ringing artifacts near hard edges on white backgrounds.

 1. JPEG can encode samples with higher values than it's possible to display (higher than 255 in RGB),
    and the decoder will always clamp values to 0-255. To encode 255 you can use any value >= 255,
    and distortions of the out-of-range values won't be visible as long as they decode to anything >= 255.

 2. From DCT perspective pixels in a block are a waveform. Hard edges form square waves (bad).
    Edges with white are similar to waveform clipping, and anti-clipping algorithms can turn square waves
    into softer ones that compress better.

*/

unsafe extern "C" fn preprocess_deringing(
    mut data: *mut DCTELEM,
    mut quantization_table: *const JQUANT_TBL,
) {
    let maxsample: DCTELEM = (255i32 - CENTERJSAMPLE) as DCTELEM;
    let size: c_int = DCTSIZE * DCTSIZE;
    /* Decoders don't handle overflow of DC very well, so calculate
    maximum overflow that is safe to do without increasing DC out of range */
    let mut sum: c_int = 0i32;
    let mut maxsample_count: c_int = 0i32;
    let mut i: c_int = 0;
    let mut maxovershoot: DCTELEM = 0;
    let mut n: c_int = 0;
    i = 0i32;
    while i < size {
        sum += *data.offset(i as isize) as c_int;
        if *data.offset(i as isize) as c_int >= maxsample as c_int {
            maxsample_count += 1
        }
        i += 1
    }
    /* If nothing reaches max value there's nothing to overshoot
    and if the block is completely flat, it's already the best case. */
    if maxsample_count == 0 || maxsample_count == size {
        return;
    }
    /* Too much overshoot is not good: increased amplitude will cost bits, and the cost is proportional to quantization (here using DC quant as a rough guide). */
    maxovershoot = (maxsample as c_int
        + (if (if 31i32 < 2i32 * (*quantization_table).quantval[0] as c_int {
            31i32
        } else {
            (2i32) * (*quantization_table).quantval[0] as c_int
        }) < (maxsample as c_int * size - sum) / maxsample_count
        {
            (if 31i32 < 2i32 * (*quantization_table).quantval[0] as c_int {
                31i32
            } else {
                (2i32) * (*quantization_table).quantval[0] as c_int
            })
        } else {
            (maxsample as c_int * size - sum) / maxsample_count
        })) as DCTELEM;
    n = 0i32;
    loop {
        let mut start: c_int = 0;
        let mut end: c_int = 0;
        let mut length: c_int = 0;
        let mut f1: DCTELEM = 0;
        let mut f2: DCTELEM = 0;
        let mut l1: DCTELEM = 0;
        let mut l2: DCTELEM = 0;
        let mut fslope: DCTELEM = 0;
        let mut lslope: DCTELEM = 0;
        let mut step: c_float = 0.;
        let mut position: c_float = 0.;
        /* Pixels are traversed in zig-zag order to process them as a line */
        if (*data.offset(*jpeg_natural_order.as_ptr().offset(n as isize) as isize) as c_int)
            < maxsample as c_int
        {
            n += 1
        } else {
            /* Find a run of maxsample pixels. Start is the first pixel inside the range, end the first pixel outside. */
            start = n;
            loop {
                n += 1;
                if !(n < size
                    && *data.offset(*jpeg_natural_order.as_ptr().offset(n as isize) as isize)
                        as c_int
                        >= maxsample as c_int)
                {
                    break;
                }
            }
            end = n;
            /* the run will be replaced with a catmull-rom interpolation of values from the edges */
            /* Find suitable upward slope from pixels around edges of the run.
            Just feeding nearby pixels as catmull rom points isn't good enough,
            as slope with one sample before the edge may have been flattened by clipping,
            and slope of two samples before the edge could be downward. */
            f1 = *data.offset(*jpeg_natural_order.as_ptr().offset(if start >= 1i32 {
                (start) - 1i32
            } else {
                0i32
            } as isize) as isize);
            f2 = *data.offset(*jpeg_natural_order.as_ptr().offset(if start >= 2i32 {
                (start) - 2i32
            } else {
                0i32
            } as isize) as isize);
            l1 = *data.offset(*jpeg_natural_order.as_ptr().offset(if end < size - 1i32 {
                end
            } else {
                (size) - 1i32
            } as isize) as isize);
            l2 = *data.offset(*jpeg_natural_order.as_ptr().offset(if end < size - 2i32 {
                (end) + 1i32
            } else {
                (size) - 1i32
            } as isize) as isize);
            fslope = if f1 as c_int - f2 as c_int > maxsample as c_int - f1 as c_int {
                (f1 as c_int) - f2 as c_int
            } else {
                (maxsample as c_int) - f1 as c_int
            } as DCTELEM;
            lslope = if l1 as c_int - l2 as c_int > maxsample as c_int - l1 as c_int {
                (l1 as c_int) - l2 as c_int
            } else {
                (maxsample as c_int) - l1 as c_int
            } as DCTELEM;
            /* if slope at the start/end is unknown, just make the curve symmetric */
            if start == 0i32 {
                fslope = lslope
            }
            if end == size {
                lslope = fslope
            }
            /* The curve fits better if first and last pixel is omitted */
            length = end - start;
            step = 1.0f32 / (length + 1i32) as c_float;
            position = step;
            i = start;
            while i < end {
                let mut tmp: DCTELEM = ceilf(catmull_rom(
                    (maxsample as c_int - fslope as c_int) as DCTELEM,
                    maxsample,
                    maxsample,
                    (maxsample as c_int - lslope as c_int) as DCTELEM,
                    position,
                    length,
                )) as DCTELEM;
                *data.offset(*jpeg_natural_order.as_ptr().offset(i as isize) as isize) =
                    if (tmp as c_int) < maxovershoot as c_int {
                        tmp as c_int
                    } else {
                        maxovershoot as c_int
                    } as DCTELEM;
                i += 1;
                position += step
            }
            n += 1
        }
        if !(n < size) {
            break;
        }
    }
}
/*
 Float version of preprocess_deringing()
*/

unsafe extern "C" fn float_preprocess_deringing(
    mut data: *mut c_float,
    mut quantization_table: *const JQUANT_TBL,
) {
    let maxsample: c_float = (255i32 - CENTERJSAMPLE) as c_float;
    let size: c_int = DCTSIZE * DCTSIZE;
    let mut sum: c_float = 0i32 as c_float;
    let mut maxsample_count: c_int = 0i32;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut maxovershoot: c_float = 0.;
    i = 0i32;
    while i < size {
        sum += *data.offset(i as isize);
        if *data.offset(i as isize) >= maxsample {
            maxsample_count += 1
        }
        i += 1
    }
    if maxsample_count == 0 || maxsample_count == size {
        return;
    }
    maxovershoot = maxsample
        + (if ((if 31i32 < 2i32 * (*quantization_table).quantval[0] as c_int {
            31i32
        } else {
            (2i32) * (*quantization_table).quantval[0] as c_int
        }) as c_float)
            < (maxsample * size as c_float - sum) / maxsample_count as c_float
        {
            (if 31i32 < 2i32 * (*quantization_table).quantval[0] as c_int {
                31i32
            } else {
                (2i32) * (*quantization_table).quantval[0] as c_int
            }) as c_float
        } else {
            (maxsample * size as c_float - sum) / maxsample_count as c_float
        });
    n = 0i32;
    loop {
        let mut start: c_int = 0;
        let mut end: c_int = 0;
        let mut length: c_int = 0;
        let mut f1: c_float = 0.;
        let mut f2: c_float = 0.;
        let mut l1: c_float = 0.;
        let mut l2: c_float = 0.;
        let mut fslope: c_float = 0.;
        let mut lslope: c_float = 0.;
        let mut step: c_float = 0.;
        let mut position: c_float = 0.;
        if *data.offset(*jpeg_natural_order.as_ptr().offset(n as isize) as isize) < maxsample {
            n += 1
        } else {
            start = n;
            loop {
                n += 1;
                if !(n < size
                    && *data.offset(*jpeg_natural_order.as_ptr().offset(n as isize) as isize)
                        >= maxsample)
                {
                    break;
                }
            }
            end = n;
            f1 = *data.offset(*jpeg_natural_order.as_ptr().offset(if start >= 1i32 {
                (start) - 1i32
            } else {
                0i32
            } as isize) as isize);
            f2 = *data.offset(*jpeg_natural_order.as_ptr().offset(if start >= 2i32 {
                (start) - 2i32
            } else {
                0i32
            } as isize) as isize);
            l1 = *data.offset(*jpeg_natural_order.as_ptr().offset(if end < size - 1i32 {
                end
            } else {
                (size) - 1i32
            } as isize) as isize);
            l2 = *data.offset(*jpeg_natural_order.as_ptr().offset(if end < size - 2i32 {
                (end) + 1i32
            } else {
                (size) - 1i32
            } as isize) as isize);
            fslope = if f1 - f2 > maxsample - f1 {
                (f1) - f2
            } else {
                (maxsample) - f1
            };
            lslope = if l1 - l2 > maxsample - l1 {
                (l1) - l2
            } else {
                (maxsample) - l1
            };
            if start == 0i32 {
                fslope = lslope
            }
            if end == size {
                lslope = fslope
            }
            length = end - start;
            step = 1.0f32 / (length + 1i32) as c_float;
            position = step;
            i = start;
            while i < end {
                let mut tmp: c_float = catmull_rom(
                    (maxsample - fslope) as DCTELEM,
                    maxsample as DCTELEM,
                    maxsample as DCTELEM,
                    (maxsample - lslope) as DCTELEM,
                    position,
                    length,
                );
                *data.offset(*jpeg_natural_order.as_ptr().offset(i as isize) as isize) =
                    if tmp < maxovershoot {
                        tmp
                    } else {
                        maxovershoot
                    };
                i += 1;
                position += step
            }
            n += 1
        }
        if !(n < size) {
            break;
        }
    }
}
/*
 * Load data into workspace, applying unsigned->signed conversion.
 */

unsafe extern "C" fn convsamp(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut DCTELEM,
) {
    let mut workspaceptr: *mut DCTELEM = 0 as *mut DCTELEM;
    let mut elemptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut elemr: c_int = 0;
    workspaceptr = workspace;
    elemr = 0i32;
    while elemr < DCTSIZE {
        elemptr = (*sample_data.offset(elemr as isize)).offset(start_col as isize);
        /* unroll the inner loop */
        let fresh0 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh1 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh1 = (*fresh0 as c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh2 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh3 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh3 = (*fresh2 as c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh4 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh5 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh5 = (*fresh4 as c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh6 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh7 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh7 = (*fresh6 as c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh8 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh9 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh9 = (*fresh8 as c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh10 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh11 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh11 = (*fresh10 as c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh12 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh13 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh13 = (*fresh12 as c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh14 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh15 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh15 = (*fresh14 as c_int - CENTERJSAMPLE) as DCTELEM;
        elemr += 1
    }
}
/*
 * Quantize/descale the coefficients, and store into coef_blocks[].
 */

unsafe extern "C" fn quantize(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut DCTELEM,
    mut workspace: *mut DCTELEM,
) {
    let mut i: c_int = 0;
    let mut temp: DCTELEM = 0;
    let mut output_ptr: JCOEFPTR = coef_block;
    let mut recip: UDCTELEM = 0;
    let mut corr: UDCTELEM = 0;
    let mut shift: c_int = 0;
    let mut product: UDCTELEM2 = 0;
    i = 0i32;
    while i < DCTSIZE2 {
        temp = *workspace.offset(i as isize);
        recip = *divisors.offset((i + DCTSIZE2 * 0i32) as isize) as UDCTELEM;
        corr = *divisors.offset((i + DCTSIZE2 * 1i32) as isize) as UDCTELEM;
        shift = *divisors.offset((i + DCTSIZE2 * 3i32) as isize) as c_int;
        if (temp as c_int) < 0i32 {
            temp = -(temp as c_int) as DCTELEM;
            product = ((temp as c_int + corr as c_int) as UDCTELEM2).wrapping_mul(recip as c_uint);
            product >>= (shift as c_ulong).wrapping_add(
                (::std::mem::size_of::<DCTELEM>() as c_ulong).wrapping_mul(8i32 as c_ulong),
            );
            temp = product as DCTELEM;
            temp = -(temp as c_int) as DCTELEM
        } else {
            product = ((temp as c_int + corr as c_int) as UDCTELEM2).wrapping_mul(recip as c_uint);
            product >>= (shift as c_ulong).wrapping_add(
                (::std::mem::size_of::<DCTELEM>() as c_ulong).wrapping_mul(8i32 as c_ulong),
            );
            temp = product as DCTELEM
        }
        *output_ptr.offset(i as isize) = temp;
        i += 1
    }
}
/*
 * Perform forward DCT on one or more blocks of a component.
 *
 * The input samples are taken from the sample_data[] array starting at
 * position start_row/start_col, and moving to the right for any additional
 * blocks. The quantized coefficients are returned in coef_blocks[].
 */

unsafe extern "C" fn forward_DCT(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut sample_data: JSAMPARRAY,
    mut coef_blocks: JBLOCKROW,
    mut start_row: JDIMENSION,
    mut start_col: JDIMENSION,
    mut num_blocks: JDIMENSION,
    mut dst: JBLOCKROW,
)
/* This version is used for integer DCT implementations. */
{
    /* This routine is heavily used, so it's worth coding it tightly. */
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut divisors: *mut DCTELEM = (*fdct).divisors[(*compptr).quant_tbl_no as usize];
    let mut qtbl: *mut JQUANT_TBL = (*cinfo).quant_tbl_ptrs[(*compptr).quant_tbl_no as usize];
    let mut workspace: *mut DCTELEM = 0 as *mut DCTELEM;
    let mut bi: JDIMENSION = 0;
    /* Make sure the compiler doesn't look up these every pass */
    let mut do_dct: forward_DCT_method_ptr = (*fdct).dct; /* fold in the vertical offset once */
    let mut do_convsamp: convsamp_method_ptr = (*fdct).convsamp;
    let mut do_preprocess: preprocess_method_ptr = (*fdct).preprocess;
    let mut do_quantize: quantize_method_ptr = (*fdct).quantize;
    workspace = (*fdct).workspace;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0i32 as JDIMENSION;
    while bi < num_blocks {
        /* Load data into workspace, applying unsigned->signed conversion */
        Some(do_convsamp.expect("non-null function pointer")).expect("non-null function pointer")(
            sample_data,
            start_col,
            workspace,
        );
        if do_preprocess.is_some() {
            Some(do_preprocess.expect("non-null function pointer"))
                .expect("non-null function pointer")(workspace, qtbl);
        }
        /* Perform the DCT */
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace,
        );
        /* Save unquantized transform coefficients for later trellis quantization */
        if !dst.is_null() {
            let mut i: c_int = 0;
            if (*cinfo).dct_method as c_uint == JDCT_IFAST as c_int as c_uint {
                static mut aanscales: [INT16; 64] = [
                    16384i32 as INT16,
                    22725i32 as INT16,
                    21407i32 as INT16,
                    19266i32 as INT16,
                    16384i32 as INT16,
                    12873i32 as INT16,
                    8867i32 as INT16,
                    4520i32 as INT16,
                    22725i32 as INT16,
                    31521i32 as INT16,
                    29692i32 as INT16,
                    26722i32 as INT16,
                    22725i32 as INT16,
                    17855i32 as INT16,
                    12299i32 as INT16,
                    6270i32 as INT16,
                    21407i32 as INT16,
                    29692i32 as INT16,
                    27969i32 as INT16,
                    25172i32 as INT16,
                    21407i32 as INT16,
                    16819i32 as INT16,
                    11585i32 as INT16,
                    5906i32 as INT16,
                    19266i32 as INT16,
                    26722i32 as INT16,
                    25172i32 as INT16,
                    22654i32 as INT16,
                    19266i32 as INT16,
                    15137i32 as INT16,
                    10426i32 as INT16,
                    5315i32 as INT16,
                    16384i32 as INT16,
                    22725i32 as INT16,
                    21407i32 as INT16,
                    19266i32 as INT16,
                    16384i32 as INT16,
                    12873i32 as INT16,
                    8867i32 as INT16,
                    4520i32 as INT16,
                    12873i32 as INT16,
                    17855i32 as INT16,
                    16819i32 as INT16,
                    15137i32 as INT16,
                    12873i32 as INT16,
                    10114i32 as INT16,
                    6967i32 as INT16,
                    3552i32 as INT16,
                    8867i32 as INT16,
                    12299i32 as INT16,
                    11585i32 as INT16,
                    10426i32 as INT16,
                    8867i32 as INT16,
                    6967i32 as INT16,
                    4799i32 as INT16,
                    2446i32 as INT16,
                    4520i32 as INT16,
                    6270i32 as INT16,
                    5906i32 as INT16,
                    5315i32 as INT16,
                    4520i32 as INT16,
                    3552i32 as INT16,
                    2446i32 as INT16,
                    1247i32 as INT16,
                ];
                i = 0i32;
                while i < DCTSIZE2 {
                    let mut x: c_int = *workspace.offset(i as isize) as c_int;
                    let mut s: c_int = aanscales[i as usize] as c_int;
                    x = if x >= 0i32 {
                        (x * 32768i32 + s) / (2i32 * s)
                    } else {
                        (x * 32768i32 - s) / (2i32 * s)
                    };
                    (*dst.offset(bi as isize))[i as usize] = x as JCOEF;
                    i += 1
                }
            } else {
                i = 0i32;
                while i < DCTSIZE2 {
                    (*dst.offset(bi as isize))[i as usize] = *workspace.offset(i as isize);
                    i += 1
                }
            }
        }
        /* Quantize/descale the coefficients, and store into coef_blocks[] */
        Some(do_quantize.expect("non-null function pointer")).expect("non-null function pointer")(
            (*coef_blocks.offset(bi as isize)).as_mut_ptr(),
            divisors,
            workspace,
        );
        if do_preprocess.is_some() {
            let mut i_0: c_int = 0;
            let mut maxval: c_int = (1i32 << super::jchuff::MAX_COEF_BITS) - 1i32;
            i_0 = 0i32;
            while i_0 < 64i32 {
                if ((*coef_blocks.offset(bi as isize))[i_0 as usize] as c_int) < -maxval {
                    (*coef_blocks.offset(bi as isize))[i_0 as usize] = -maxval as JCOEF
                }
                if (*coef_blocks.offset(bi as isize))[i_0 as usize] as c_int > maxval {
                    (*coef_blocks.offset(bi as isize))[i_0 as usize] = maxval as JCOEF
                }
                i_0 += 1
            }
        }
        bi = bi.wrapping_add(1);
        start_col =
            (start_col as c_uint).wrapping_add(DCTSIZE as c_uint) as JDIMENSION as JDIMENSION
    }
}

unsafe extern "C" fn convsamp_float(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut c_float,
) {
    let mut workspaceptr: *mut c_float = 0 as *mut c_float;
    let mut elemptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut elemr: c_int = 0;
    workspaceptr = workspace;
    elemr = 0i32;
    while elemr < DCTSIZE {
        elemptr = (*sample_data.offset(elemr as isize)).offset(start_col as isize);
        /* unroll the inner loop */
        let fresh16 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh17 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh17 = (*fresh16 as c_int - CENTERJSAMPLE) as c_float;
        let fresh18 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh19 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh19 = (*fresh18 as c_int - CENTERJSAMPLE) as c_float;
        let fresh20 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh21 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh21 = (*fresh20 as c_int - CENTERJSAMPLE) as c_float;
        let fresh22 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh23 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh23 = (*fresh22 as c_int - CENTERJSAMPLE) as c_float;
        let fresh24 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh25 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh25 = (*fresh24 as c_int - CENTERJSAMPLE) as c_float;
        let fresh26 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh27 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh27 = (*fresh26 as c_int - CENTERJSAMPLE) as c_float;
        let fresh28 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh29 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh29 = (*fresh28 as c_int - CENTERJSAMPLE) as c_float;
        let fresh30 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh31 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh31 = (*fresh30 as c_int - CENTERJSAMPLE) as c_float;
        elemr += 1
    }
}

unsafe extern "C" fn quantize_float(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut c_float,
    mut workspace: *mut c_float,
) {
    let mut temp: c_float = 0.;
    let mut i: c_int = 0;
    let mut output_ptr: JCOEFPTR = coef_block;
    i = 0i32;
    while i < DCTSIZE2 {
        /* Apply the quantization and scaling factor */
        temp = *workspace.offset(i as isize) * *divisors.offset(i as isize);
        /* Round to nearest integer.
         * Since C does not specify the direction of rounding for negative
         * quotients, we have to force the dividend positive for portability.
         * The maximum coefficient size is +-16K (for 12-bit data), so this
         * code should work for either 16-bit or 32-bit ints.
         */
        *output_ptr.offset(i as isize) =
            ((temp + 16384.5f64 as c_float) as c_int - 16384i32) as JCOEF;
        i += 1
    }
}

unsafe extern "C" fn forward_DCT_float(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut sample_data: JSAMPARRAY,
    mut coef_blocks: JBLOCKROW,
    mut start_row: JDIMENSION,
    mut start_col: JDIMENSION,
    mut num_blocks: JDIMENSION,
    mut dst: JBLOCKROW,
)
/* This version is used for floating-point DCT implementations. */
{
    /* This routine is heavily used, so it's worth coding it tightly. */
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut divisors: *mut c_float = (*fdct).float_divisors[(*compptr).quant_tbl_no as usize];
    let mut qtbl: *mut JQUANT_TBL = (*cinfo).quant_tbl_ptrs[(*compptr).quant_tbl_no as usize];
    let mut workspace: *mut c_float = 0 as *mut c_float;
    let mut bi: JDIMENSION = 0;
    let mut v: c_float = 0.;
    let mut x: c_int = 0;
    /* Make sure the compiler doesn't look up these every pass */
    let mut do_dct: float_DCT_method_ptr = (*fdct).float_dct; /* fold in the vertical offset once */
    let mut do_convsamp: float_convsamp_method_ptr = (*fdct).float_convsamp;
    let mut do_preprocess: float_preprocess_method_ptr = (*fdct).float_preprocess;
    let mut do_quantize: float_quantize_method_ptr = (*fdct).float_quantize;
    workspace = (*fdct).float_workspace;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0i32 as JDIMENSION;
    while bi < num_blocks {
        /* Load data into workspace, applying unsigned->signed conversion */
        Some(do_convsamp.expect("non-null function pointer")).expect("non-null function pointer")(
            sample_data,
            start_col,
            workspace,
        );
        if do_preprocess.is_some() {
            Some(do_preprocess.expect("non-null function pointer"))
                .expect("non-null function pointer")(workspace, qtbl);
        }
        /* Perform the DCT */
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace,
        );
        /* Save unquantized transform coefficients for later trellis quantization */
        /* Currently save as integer values. Could save float values but would require */
        /* modifications to memory allocation and trellis quantization */
        if !dst.is_null() {
            let mut i: c_int = 0;
            static mut aanscalefactor: [c_double; 8] = [
                1.0f64,
                1.387039845f64,
                1.306562965f64,
                1.175875602f64,
                1.0f64,
                0.785694958f64,
                0.541196100f64,
                0.275899379f64,
            ];
            i = 0i32;
            while i < DCTSIZE2 {
                v = *workspace.offset(i as isize);
                v = (v as c_double / aanscalefactor[(i % 8i32) as usize]) as c_float;
                v = (v as c_double / aanscalefactor[(i / 8i32) as usize]) as c_float;
                x = if v as c_double >= 0.0f64 {
                    (v as c_double + 0.5f64) as c_int
                } else {
                    (v as c_double - 0.5f64) as c_int
                };
                (*dst.offset(bi as isize))[i as usize] = x as JCOEF;
                i += 1
            }
        }
        /* Quantize/descale the coefficients, and store into coef_blocks[] */
        Some(do_quantize.expect("non-null function pointer")).expect("non-null function pointer")(
            (*coef_blocks.offset(bi as isize)).as_mut_ptr(),
            divisors,
            workspace,
        );
        if do_preprocess.is_some() {
            let mut i_0: c_int = 0;
            let mut maxval: c_int = (1i32 << super::jchuff::MAX_COEF_BITS) - 1i32;
            i_0 = 0i32;
            while i_0 < 64i32 {
                if ((*coef_blocks.offset(bi as isize))[i_0 as usize] as c_int) < -maxval {
                    (*coef_blocks.offset(bi as isize))[i_0 as usize] = -maxval as JCOEF
                }
                if (*coef_blocks.offset(bi as isize))[i_0 as usize] as c_int > maxval {
                    (*coef_blocks.offset(bi as isize))[i_0 as usize] = maxval as JCOEF
                }
                i_0 += 1
            }
        }
        bi = bi.wrapping_add(1);
        start_col =
            (start_col as c_uint).wrapping_add(DCTSIZE as c_uint) as JDIMENSION as JDIMENSION
    }
}
/* DCT_FLOAT_SUPPORTED */

static mut jpeg_lambda_weights_flat: [c_float; 64] = [
    1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, 1.0f32, 1.0f32,
];

static mut jpeg_lambda_weights_csf_luma: [c_float; 64] = [
    3.35630f32, 3.59892f32, 3.20921f32, 2.28102f32, 1.42378f32, 0.88079f32, 0.58190f32, 0.43454f32,
    3.59893f32, 3.21284f32, 2.71282f32, 1.98092f32, 1.30506f32, 0.83852f32, 0.56346f32, 0.42146f32,
    3.20921f32, 2.71282f32, 2.12574f32, 1.48616f32, 0.99660f32, 0.66132f32, 0.45610f32, 0.34609f32,
    2.28102f32, 1.98092f32, 1.48616f32, 0.97492f32, 0.64622f32, 0.43812f32, 0.31074f32, 0.24072f32,
    1.42378f32, 1.30506f32, 0.99660f32, 0.64623f32, 0.42051f32, 0.28446f32, 0.20380f32, 0.15975f32,
    0.88079f32, 0.83852f32, 0.66132f32, 0.43812f32, 0.28446f32, 0.19092f32, 0.13635f32, 0.10701f32,
    0.58190f32, 0.56346f32, 0.45610f32, 0.31074f32, 0.20380f32, 0.13635f32, 0.09674f32, 0.07558f32,
    0.43454f32, 0.42146f32, 0.34609f32, 0.24072f32, 0.15975f32, 0.10701f32, 0.07558f32, 0.05875f32,
];

unsafe extern "C" fn get_num_dc_trellis_candidates(mut dc_quantval: c_int) -> c_int {
    /* Higher qualities can tolerate higher DC distortion */
    return if 9i32 < 2i32 + 60i32 / dc_quantval | 1i32 {
        9i32
    } else {
        (2i32 + 60i32 / dc_quantval) | 1i32
    }; /* position of last nonzero coefficient */
}
#[no_mangle]

pub unsafe extern "C" fn quantize_trellis(
    mut cinfo: j_compress_ptr,
    mut dctbl: *mut super::jchuff::c_derived_tbl,
    mut actbl: *mut super::jchuff::c_derived_tbl,
    mut coef_blocks: JBLOCKROW,
    mut src: JBLOCKROW,
    mut num_blocks: JDIMENSION,
    mut qtbl: *mut JQUANT_TBL,
    mut norm_src: *mut c_double,
    mut norm_coef: *mut c_double,
    mut last_dc_val: *mut JCOEF,
    mut coef_blocks_above: JBLOCKROW,
    mut src_above: JBLOCKROW,
) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut k: c_int = 0;
    let mut l: c_int = 0;
    let mut accumulated_zero_dist: [c_float; 64] = [0.; 64];
    let mut accumulated_cost: [c_float; 64] = [0.; 64];
    let mut run_start: [c_int; 64] = [0; 64];
    let mut bi: c_int = 0;
    let mut best_cost: c_float = 0.;
    let mut last_coeff_idx: c_int = 0;
    let mut norm: c_float = 0.0f64 as c_float;
    let mut lambda_base: c_float = 0.;
    let mut lambda: c_float = 0.;
    let mut lambda_dc: c_float = 0.;
    let mut lambda_tbl: *const c_float = if (*(*cinfo).master).use_lambda_weight_tbl != 0 {
        jpeg_lambda_weights_csf_luma.as_ptr()
    } else {
        jpeg_lambda_weights_flat.as_ptr()
    };
    let mut Ss: c_int = 0;
    let mut Se: c_int = 0;
    let mut accumulated_zero_block_cost: *mut c_float = NULL as *mut c_float;
    let mut accumulated_block_cost: *mut c_float = NULL as *mut c_float;
    let mut block_run_start: *mut c_int = NULL as *mut c_int;
    let mut requires_eob: *mut c_int = NULL as *mut c_int;
    let mut has_eob: c_int = 0;
    let mut cost_all_zeros: c_float = 0.;
    let mut best_cost_skip: c_float = 0.;
    let mut cost: c_float = 0.;
    let mut zero_run: c_int = 0;
    let mut run_bits: c_int = 0;
    let mut rate: c_int = 0;
    let mut accumulated_dc_cost: [*mut c_float; 9] = [0 as *mut c_float; 9];
    let mut dc_cost_backtrack: [*mut c_int; 9] = [0 as *mut c_int; 9];
    let mut dc_candidate: [*mut JCOEF; 9] = [0 as *mut JCOEF; 9];
    let mut mode: c_int = 1i32;
    let mut lambda_table: [c_float; 64] = [0.; 64];
    let dc_trellis_candidates: c_int = get_num_dc_trellis_candidates((*qtbl).quantval[0] as c_int);
    Ss = (*cinfo).Ss;
    Se = (*cinfo).Se;
    if Ss == 0i32 {
        Ss = 1i32
    }
    if Se < Ss {
        return;
    }
    if (*(*cinfo).master).trellis_eob_opt != 0 {
        accumulated_zero_block_cost = malloc(
            (num_blocks.wrapping_add(1i32 as c_uint) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_float>() as c_ulong),
        ) as *mut c_float;
        accumulated_block_cost = malloc(
            (num_blocks.wrapping_add(1i32 as c_uint) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_float>() as c_ulong),
        ) as *mut c_float;
        block_run_start =
            malloc((num_blocks as c_ulong).wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong))
                as *mut c_int;
        requires_eob = malloc(
            (num_blocks.wrapping_add(1i32 as c_uint) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
        ) as *mut c_int;
        if accumulated_zero_block_cost.is_null()
            || accumulated_block_cost.is_null()
            || block_run_start.is_null()
            || requires_eob.is_null()
        {
            (*(*cinfo).err).msg_code = super::jerror::JERR_OUT_OF_MEMORY as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        *accumulated_zero_block_cost.offset(0) = 0i32 as c_float;
        *accumulated_block_cost.offset(0) = 0i32 as c_float;
        *requires_eob.offset(0) = 0i32
    }
    if (*(*cinfo).master).trellis_quant_dc != 0 {
        i = 0i32;
        while i < dc_trellis_candidates {
            accumulated_dc_cost[i as usize] = malloc(
                (num_blocks as c_ulong).wrapping_mul(::std::mem::size_of::<c_float>() as c_ulong),
            ) as *mut c_float;
            dc_cost_backtrack[i as usize] = malloc(
                (num_blocks as c_ulong).wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
            ) as *mut c_int;
            dc_candidate[i as usize] = malloc(
                (num_blocks as c_ulong).wrapping_mul(::std::mem::size_of::<JCOEF>() as c_ulong),
            ) as *mut JCOEF;
            if accumulated_dc_cost[i as usize].is_null()
                || dc_cost_backtrack[i as usize].is_null()
                || dc_candidate[i as usize].is_null()
            {
                (*(*cinfo).err).msg_code = super::jerror::JERR_OUT_OF_MEMORY as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            i += 1
        }
    }
    norm = 0.0f64 as c_float;
    i = 1i32;
    while i < DCTSIZE2 {
        norm += ((*qtbl).quantval[i as usize] as c_int * (*qtbl).quantval[i as usize] as c_int)
            as c_float;
        i += 1
    }
    norm = (norm as c_double / 63.0f64) as c_float;
    if mode == 1i32 {
        lambda_base = 1.0f64 as c_float;
        lambda_tbl = lambda_table.as_mut_ptr();
        i = 0i32;
        while i < DCTSIZE2 {
            lambda_table[i as usize] = (1.0f64
                / ((*qtbl).quantval[i as usize] as c_int * (*qtbl).quantval[i as usize] as c_int)
                    as c_double) as c_float;
            i += 1
        }
    } else {
        lambda_base = (1.0f64 / norm as c_double) as c_float
    }
    bi = 0i32;
    while (bi as c_uint) < num_blocks {
        norm = 0.0f64 as c_float;
        i = 1i32;
        while i < DCTSIZE2 {
            norm += ((*src.offset(bi as isize))[i as usize] as c_int
                * (*src.offset(bi as isize))[i as usize] as c_int) as c_float;
            i += 1
        }
        norm = (norm as c_double / 63.0f64) as c_float;
        if (*(*cinfo).master).lambda_log_scale2 as c_double > 0.0f64 {
            lambda = (pow(2.0f64, (*(*cinfo).master).lambda_log_scale1 as c_double)
                * lambda_base as c_double
                / (pow(2.0f64, (*(*cinfo).master).lambda_log_scale2 as c_double)
                    + norm as c_double)) as c_float
        } else {
            lambda = (pow(
                2.0f64,
                (*(*cinfo).master).lambda_log_scale1 as c_double - 12.0f64,
            ) * lambda_base as c_double) as c_float
        }
        lambda_dc = lambda * *lambda_tbl.offset(0);
        accumulated_zero_dist[(Ss - 1i32) as usize] = 0.0f64 as c_float;
        accumulated_cost[(Ss - 1i32) as usize] = 0.0f64 as c_float;
        /* Do DC coefficient */
        if (*(*cinfo).master).trellis_quant_dc != 0 {
            let mut sign: c_int = (*src.offset(bi as isize))[0] as c_int >> 31i32; /* quantized value (round nearest) */
            let mut x: c_int = abs((*src.offset(bi as isize))[0] as c_int);
            let mut q: c_int = 8i32 * (*qtbl).quantval[0] as c_int;
            let mut qval: c_int = 0;
            let mut dc_candidate_dist: c_float = 0.;
            qval = (x + q / 2i32) / q;
            k = 0i32;
            while k < dc_trellis_candidates {
                let mut delta: c_int = 0;
                let mut dc_delta: c_int = 0;
                let mut bits: c_int = 0;
                *dc_candidate[k as usize].offset(bi as isize) =
                    (qval - dc_trellis_candidates / 2i32 + k) as JCOEF;
                if *dc_candidate[k as usize].offset(bi as isize) as c_int
                    >= 1i32 << super::jchuff::MAX_COEF_BITS
                {
                    *dc_candidate[k as usize].offset(bi as isize) =
                        ((1i32 << super::jchuff::MAX_COEF_BITS) - 1i32) as JCOEF
                }
                if *dc_candidate[k as usize].offset(bi as isize) as c_int
                    <= -(1i32 << super::jchuff::MAX_COEF_BITS)
                {
                    *dc_candidate[k as usize].offset(bi as isize) =
                        (-(1i32 << super::jchuff::MAX_COEF_BITS) + 1i32) as JCOEF
                }
                delta = *dc_candidate[k as usize].offset(bi as isize) as c_int * q - x;
                dc_candidate_dist = (delta * delta) as c_float * lambda_dc;
                let ref mut fresh32 = *dc_candidate[k as usize].offset(bi as isize);
                *fresh32 = (*fresh32 as c_int * (1i32 + 2i32 * sign)) as JCOEF;
                /* Take into account DC differences */
                if !coef_blocks_above.is_null()
                    && !src_above.is_null()
                    && (*(*cinfo).master).trellis_delta_dc_weight as c_double > 0.0f64
                {
                    let mut dc_above_orig: c_int = 0;
                    let mut dc_above_recon: c_int = 0;
                    let mut dc_orig: c_int = 0;
                    let mut dc_recon: c_int = 0;
                    let mut vertical_dist: c_float = 0.;
                    dc_above_orig = (*src_above.offset(bi as isize))[0] as c_int;
                    dc_above_recon = (*coef_blocks_above.offset(bi as isize))[0] as c_int * q;
                    dc_orig = (*src.offset(bi as isize))[0] as c_int;
                    dc_recon = *dc_candidate[k as usize].offset(bi as isize) as c_int * q;
                    /* delta is difference of vertical gradients */
                    delta = dc_above_orig - dc_orig - (dc_above_recon - dc_recon);
                    vertical_dist = (delta * delta) as c_float * lambda_dc;
                    dc_candidate_dist += (*(*cinfo).master).trellis_delta_dc_weight
                        * (vertical_dist - dc_candidate_dist)
                }
                if bi == 0i32 {
                    dc_delta = *dc_candidate[k as usize].offset(bi as isize) as c_int
                        - *last_dc_val as c_int;
                    /* Derive number of suffix bits */
                    bits = 0i32;
                    dc_delta = abs(dc_delta);
                    while dc_delta != 0 {
                        dc_delta >>= 1i32;
                        bits += 1
                    }
                    cost = (bits + (*dctbl).ehufsi[bits as usize] as c_int) as c_float
                        + dc_candidate_dist;
                    *accumulated_dc_cost[k as usize].offset(0) = cost;
                    *dc_cost_backtrack[k as usize].offset(0) = -1i32
                } else {
                    l = 0i32;
                    while l < dc_trellis_candidates {
                        dc_delta = *dc_candidate[k as usize].offset(bi as isize) as c_int
                            - *dc_candidate[l as usize].offset((bi - 1i32) as isize) as c_int;
                        /* Derive number of suffix bits */
                        bits = 0i32;
                        dc_delta = abs(dc_delta);
                        while dc_delta != 0 {
                            dc_delta >>= 1i32;
                            bits += 1
                        }
                        cost = (bits + (*dctbl).ehufsi[bits as usize] as c_int) as c_float
                            + dc_candidate_dist
                            + *accumulated_dc_cost[l as usize].offset((bi - 1i32) as isize);
                        if l == 0i32 || cost < *accumulated_dc_cost[k as usize].offset(bi as isize)
                        {
                            *accumulated_dc_cost[k as usize].offset(bi as isize) = cost;
                            *dc_cost_backtrack[k as usize].offset(bi as isize) = l
                        }
                        l += 1
                    }
                }
                k += 1
            }
        }
        /* Do AC coefficients */
        i = Ss; /* quantized value (round nearest) */
        while i <= Se {
            let mut z: c_int = *jpeg_natural_order.as_ptr().offset(i as isize); /* Shouldn't be needed */
            let mut sign_0: c_int = (*src.offset(bi as isize))[z as usize] as c_int >> 31i32;
            let mut x_0: c_int = abs((*src.offset(bi as isize))[z as usize] as c_int);
            let mut q_0: c_int = 8i32 * (*qtbl).quantval[z as usize] as c_int;
            let mut candidate: [c_int; 16] = [0; 16];
            let mut candidate_bits: [c_int; 16] = [0; 16];
            let mut candidate_dist: [c_float; 16] = [0.; 16];
            let mut num_candidates: c_int = 0;
            let mut qval_0: c_int = 0;
            accumulated_zero_dist[i as usize] =
                (x_0 * x_0) as c_float * lambda * *lambda_tbl.offset(z as isize)
                    + accumulated_zero_dist[(i - 1i32) as usize];
            qval_0 = (x_0 + q_0 / 2i32) / q_0;
            if qval_0 == 0i32 {
                (*coef_blocks.offset(bi as isize))[z as usize] = 0i32 as JCOEF;
                accumulated_cost[i as usize] = 1e38f64 as c_float
            } else {
                if qval_0 >= 1i32 << super::jchuff::MAX_COEF_BITS {
                    qval_0 = (1i32 << super::jchuff::MAX_COEF_BITS) - 1i32
                }
                num_candidates = jpeg_nbits_table[qval_0 as usize] as c_int;
                k = 0i32;
                while k < num_candidates {
                    let mut delta_0: c_int = 0;
                    candidate[k as usize] = if k < num_candidates - 1i32 {
                        (2i32 << k) - 1i32
                    } else {
                        qval_0
                    };
                    delta_0 = candidate[k as usize] * q_0 - x_0;
                    candidate_bits[k as usize] = k + 1i32;
                    candidate_dist[k as usize] =
                        (delta_0 * delta_0) as c_float * lambda * *lambda_tbl.offset(z as isize);
                    k += 1
                }
                accumulated_cost[i as usize] = 1e38f64 as c_float;
                j = Ss - 1i32;
                while j < i {
                    let mut zz: c_int = *jpeg_natural_order.as_ptr().offset(j as isize);
                    if !(j != Ss - 1i32
                        && (*coef_blocks.offset(bi as isize))[zz as usize] as c_int == 0i32)
                    {
                        zero_run = i - 1i32 - j;
                        if !(zero_run >> 4i32 != 0
                            && (*actbl).ehufsi[0xf0i32 as usize] as c_int == 0i32)
                        {
                            run_bits =
                                (zero_run >> 4i32) * (*actbl).ehufsi[0xf0i32 as usize] as c_int;
                            zero_run &= 15i32;
                            k = 0i32;
                            while k < num_candidates {
                                let mut coef_bits: c_int = (*actbl).ehufsi
                                    [(16i32 * zero_run + candidate_bits[k as usize]) as usize]
                                    as c_int;
                                if !(coef_bits == 0i32) {
                                    rate = coef_bits + candidate_bits[k as usize] + run_bits;
                                    cost = rate as c_float + candidate_dist[k as usize];
                                    cost += accumulated_zero_dist[(i - 1i32) as usize]
                                        - accumulated_zero_dist[j as usize]
                                        + accumulated_cost[j as usize];
                                    if cost < accumulated_cost[i as usize] {
                                        (*coef_blocks.offset(bi as isize))[z as usize] =
                                            ((candidate[k as usize] ^ sign_0) - sign_0) as JCOEF;
                                        accumulated_cost[i as usize] = cost;
                                        run_start[i as usize] = j
                                    }
                                }
                                k += 1
                            }
                        }
                    }
                    j += 1
                }
            }
            i += 1
        }
        last_coeff_idx = Ss - 1i32;
        best_cost = accumulated_zero_dist[Se as usize] + (*actbl).ehufsi[0] as c_int as c_float;
        cost_all_zeros = accumulated_zero_dist[Se as usize];
        best_cost_skip = cost_all_zeros;
        i = Ss;
        while i <= Se {
            let mut z_0: c_int = *jpeg_natural_order.as_ptr().offset(i as isize);
            if (*coef_blocks.offset(bi as isize))[z_0 as usize] as c_int != 0i32 {
                let mut cost_0: c_float = accumulated_cost[i as usize]
                    + accumulated_zero_dist[Se as usize]
                    - accumulated_zero_dist[i as usize];
                let mut cost_wo_eob: c_float = cost_0;
                if i < Se {
                    cost_0 += (*actbl).ehufsi[0] as c_int as c_float
                }
                if cost_0 < best_cost {
                    best_cost = cost_0;
                    last_coeff_idx = i;
                    best_cost_skip = cost_wo_eob
                }
            }
            i += 1
        }
        has_eob = (last_coeff_idx < Se) as c_int + (last_coeff_idx == Ss - 1i32) as c_int;
        /* Zero out coefficients that are part of runs */
        i = Se; /* cost of coding a nonzero block */
        while i >= Ss {
            while i > last_coeff_idx {
                let mut z_1: c_int = *jpeg_natural_order.as_ptr().offset(i as isize);
                (*coef_blocks.offset(bi as isize))[z_1 as usize] = 0i32 as JCOEF;
                i -= 1
            }
            last_coeff_idx = run_start[i as usize];
            i -= 1
        }
        if (*(*cinfo).master).trellis_eob_opt != 0 {
            *accumulated_zero_block_cost.offset((bi + 1i32) as isize) =
                *accumulated_zero_block_cost.offset(bi as isize);
            *accumulated_zero_block_cost.offset((bi + 1i32) as isize) += cost_all_zeros;
            *requires_eob.offset((bi + 1i32) as isize) = has_eob;
            best_cost = 1e38f64 as c_float;
            if has_eob != 2i32 {
                i = 0i32;
                while i <= bi {
                    let mut zero_block_run: c_int = 0;
                    let mut nbits: c_int = 0;
                    let mut cost_1: c_float = 0.;
                    if !(*requires_eob.offset(i as isize) == 2i32) {
                        cost_1 = best_cost_skip;
                        cost_1 += *accumulated_zero_block_cost.offset(bi as isize);
                        cost_1 -= *accumulated_zero_block_cost.offset(i as isize);
                        cost_1 += *accumulated_block_cost.offset(i as isize);
                        zero_block_run = bi - i + *requires_eob.offset(i as isize);
                        nbits = jpeg_nbits_table[zero_block_run as usize] as c_int;
                        cost_1 +=
                            ((*actbl).ehufsi[(16i32 * nbits) as usize] as c_int + nbits) as c_float;
                        if cost_1 < best_cost {
                            *block_run_start.offset(bi as isize) = i;
                            best_cost = cost_1;
                            *accumulated_block_cost.offset((bi + 1i32) as isize) = cost_1
                        }
                    }
                    i += 1
                }
            }
        }
        bi += 1
    }
    if (*(*cinfo).master).trellis_eob_opt != 0 {
        let mut last_block: c_int = num_blocks as c_int;
        best_cost = 1e38f64 as c_float;
        i = 0i32;
        while i as c_uint <= num_blocks {
            let mut zero_block_run_0: c_int = 0;
            let mut nbits_0: c_int = 0;
            let mut cost_2: c_float = 0.0f64 as c_float;
            if !(*requires_eob.offset(i as isize) == 2i32) {
                cost_2 += *accumulated_zero_block_cost.offset(num_blocks as isize);
                cost_2 -= *accumulated_zero_block_cost.offset(i as isize);
                zero_block_run_0 = num_blocks
                    .wrapping_sub(i as c_uint)
                    .wrapping_add(*requires_eob.offset(i as isize) as c_uint)
                    as c_int;
                nbits_0 = jpeg_nbits_table[zero_block_run_0 as usize] as c_int;
                cost_2 +=
                    ((*actbl).ehufsi[(16i32 * nbits_0) as usize] as c_int + nbits_0) as c_float;
                if cost_2 < best_cost {
                    best_cost = cost_2;
                    last_block = i
                }
            }
            i += 1
        }
        last_block -= 1;
        bi = num_blocks.wrapping_sub(1i32 as c_uint) as c_int;
        while bi >= 0i32 {
            while bi > last_block {
                j = Ss;
                while j <= Se {
                    let mut z_2: c_int = *jpeg_natural_order.as_ptr().offset(j as isize);
                    (*coef_blocks.offset(bi as isize))[z_2 as usize] = 0i32 as JCOEF;
                    j += 1
                }
                bi -= 1
            }
            last_block = *block_run_start.offset(bi as isize) - 1i32;
            bi -= 1
        }
        free(accumulated_zero_block_cost as *mut c_void);
        free(accumulated_block_cost as *mut c_void);
        free(block_run_start as *mut c_void);
        free(requires_eob as *mut c_void);
    }
    if (*(*cinfo).master).trellis_q_opt != 0 {
        bi = 0i32;
        while (bi as c_uint) < num_blocks {
            i = 1i32;
            while i < DCTSIZE2 {
                *norm_src.offset(i as isize) += ((*src.offset(bi as isize))[i as usize] as c_int
                    * (*coef_blocks.offset(bi as isize))[i as usize] as c_int)
                    as c_double;
                *norm_coef.offset(i as isize) += (8i32
                    * (*coef_blocks.offset(bi as isize))[i as usize] as c_int
                    * (*coef_blocks.offset(bi as isize))[i as usize] as c_int)
                    as c_double;
                i += 1
            }
            bi += 1
        }
    }
    if (*(*cinfo).master).trellis_quant_dc != 0 {
        j = 0i32;
        i = 1i32;
        while i < dc_trellis_candidates {
            if *accumulated_dc_cost[i as usize]
                .offset(num_blocks.wrapping_sub(1i32 as c_uint) as isize)
                < *accumulated_dc_cost[j as usize]
                    .offset(num_blocks.wrapping_sub(1i32 as c_uint) as isize)
            {
                j = i
            }
            i += 1
        }
        bi = num_blocks.wrapping_sub(1i32 as c_uint) as c_int;
        while bi >= 0i32 {
            (*coef_blocks.offset(bi as isize))[0] = *dc_candidate[j as usize].offset(bi as isize);
            j = *dc_cost_backtrack[j as usize].offset(bi as isize);
            bi -= 1
        }
        /* Save DC predictor */
        *last_dc_val = (*coef_blocks.offset(num_blocks.wrapping_sub(1i32 as c_uint) as isize))[0];
        i = 0i32;
        while i < dc_trellis_candidates {
            free(accumulated_dc_cost[i as usize] as *mut c_void);
            free(dc_cost_backtrack[i as usize] as *mut c_void);
            free(dc_candidate[i as usize] as *mut c_void);
            i += 1
        }
    };
}
/*
 * Initialize FDCT manager.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_forward_dct(mut cinfo: j_compress_ptr) {
    let mut fdct: my_fdct_ptr = 0 as *mut my_fdct_controller;
    let mut i: c_int = 0;
    fdct = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_fdct_controller>() as c_ulong,
    ) as my_fdct_ptr;
    (*cinfo).fdct = fdct as *mut jpeg_forward_dct;
    (*fdct).pub_0.start_pass =
        Some(start_pass_fdctmgr as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    /* First determine the DCT... */
    match (*cinfo).dct_method as c_uint {
        0 => {
            (*fdct).pub_0.forward_DCT = Some(
                forward_DCT
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: JBLOCKROW,
                        _: JDIMENSION,
                        _: JDIMENSION,
                        _: JDIMENSION,
                        _: JBLOCKROW,
                    ) -> (),
            );
            if super::simd::x86_64::jsimd::jsimd_can_fdct_islow() != 0 {
                (*fdct).dct = Some(
                    super::simd::x86_64::jsimd::jsimd_fdct_islow
                        as unsafe extern "C" fn(_: *mut DCTELEM) -> (),
                )
            } else {
                (*fdct).dct = Some(jpeg_fdct_islow as unsafe extern "C" fn(_: *mut DCTELEM) -> ())
            }
        }
        1 => {
            (*fdct).pub_0.forward_DCT = Some(
                forward_DCT
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: JBLOCKROW,
                        _: JDIMENSION,
                        _: JDIMENSION,
                        _: JDIMENSION,
                        _: JBLOCKROW,
                    ) -> (),
            );
            if super::simd::x86_64::jsimd::jsimd_can_fdct_ifast() != 0 {
                (*fdct).dct = Some(
                    super::simd::x86_64::jsimd::jsimd_fdct_ifast
                        as unsafe extern "C" fn(_: *mut DCTELEM) -> (),
                )
            } else {
                (*fdct).dct = Some(jpeg_fdct_ifast as unsafe extern "C" fn(_: *mut DCTELEM) -> ())
            }
        }
        2 => {
            (*fdct).pub_0.forward_DCT = Some(
                forward_DCT_float
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: JBLOCKROW,
                        _: JDIMENSION,
                        _: JDIMENSION,
                        _: JDIMENSION,
                        _: JBLOCKROW,
                    ) -> (),
            );
            if super::simd::x86_64::jsimd::jsimd_can_fdct_float() != 0 {
                (*fdct).float_dct = Some(
                    super::simd::x86_64::jsimd::jsimd_fdct_float
                        as unsafe extern "C" fn(_: *mut c_float) -> (),
                )
            } else {
                (*fdct).float_dct =
                    Some(jpeg_fdct_float as unsafe extern "C" fn(_: *mut c_float) -> ())
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_NOT_COMPILED as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    /* ...then the supporting stages. */
    match (*cinfo).dct_method as c_uint {
        0 | 1 => {
            if super::simd::x86_64::jsimd::jsimd_can_convsamp() != 0 {
                (*fdct).convsamp = Some(
                    super::simd::x86_64::jsimd::jsimd_convsamp
                        as unsafe extern "C" fn(
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                            _: *mut DCTELEM,
                        ) -> (),
                )
            } else {
                (*fdct).convsamp = Some(
                    convsamp
                        as unsafe extern "C" fn(
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                            _: *mut DCTELEM,
                        ) -> (),
                )
            }
            if (*(*cinfo).master).overshoot_deringing != 0 {
                (*fdct).preprocess = Some(
                    preprocess_deringing
                        as unsafe extern "C" fn(_: *mut DCTELEM, _: *const JQUANT_TBL) -> (),
                )
            } else {
                (*fdct).preprocess =
                    ::std::mem::transmute::<intptr_t, preprocess_method_ptr>(NULL as intptr_t)
            }
            if super::simd::x86_64::jsimd::jsimd_can_quantize() != 0 {
                (*fdct).quantize = Some(
                    super::simd::x86_64::jsimd::jsimd_quantize
                        as unsafe extern "C" fn(
                            _: JCOEFPTR,
                            _: *mut DCTELEM,
                            _: *mut DCTELEM,
                        ) -> (),
                )
            } else {
                (*fdct).quantize = Some(
                    quantize
                        as unsafe extern "C" fn(
                            _: JCOEFPTR,
                            _: *mut DCTELEM,
                            _: *mut DCTELEM,
                        ) -> (),
                )
            }
        }
        2 => {
            if super::simd::x86_64::jsimd::jsimd_can_convsamp_float() != 0 {
                (*fdct).float_convsamp = Some(
                    super::simd::x86_64::jsimd::jsimd_convsamp_float
                        as unsafe extern "C" fn(
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                            _: *mut c_float,
                        ) -> (),
                )
            } else {
                (*fdct).float_convsamp = Some(
                    convsamp_float
                        as unsafe extern "C" fn(
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                            _: *mut c_float,
                        ) -> (),
                )
            }
            if (*(*cinfo).master).overshoot_deringing != 0 {
                (*fdct).float_preprocess = Some(
                    float_preprocess_deringing
                        as unsafe extern "C" fn(_: *mut c_float, _: *const JQUANT_TBL) -> (),
                )
            } else {
                (*fdct).float_preprocess =
                    ::std::mem::transmute::<intptr_t, float_preprocess_method_ptr>(NULL as intptr_t)
            }
            if super::simd::x86_64::jsimd::jsimd_can_quantize_float() != 0 {
                (*fdct).float_quantize = Some(
                    super::simd::x86_64::jsimd::jsimd_quantize_float
                        as unsafe extern "C" fn(
                            _: JCOEFPTR,
                            _: *mut c_float,
                            _: *mut c_float,
                        ) -> (),
                )
            } else {
                (*fdct).float_quantize = Some(
                    quantize_float
                        as unsafe extern "C" fn(
                            _: JCOEFPTR,
                            _: *mut c_float,
                            _: *mut c_float,
                        ) -> (),
                )
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_NOT_COMPILED as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    /* Allocate workspace memory */
    if (*cinfo).dct_method as c_uint == JDCT_FLOAT as c_int as c_uint {
        (*fdct).float_workspace = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (::std::mem::size_of::<c_float>() as c_ulong).wrapping_mul(DCTSIZE2 as c_ulong),
        ) as *mut c_float
    } else {
        (*fdct).workspace = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (::std::mem::size_of::<DCTELEM>() as c_ulong).wrapping_mul(DCTSIZE2 as c_ulong),
        ) as *mut DCTELEM
    }
    /* Mark divisor tables unallocated */
    i = 0i32;
    while i < NUM_QUANT_TBLS {
        (*fdct).divisors[i as usize] = NULL as *mut DCTELEM;
        (*fdct).float_divisors[i as usize] = NULL as *mut c_float;
        i += 1
    }
}
