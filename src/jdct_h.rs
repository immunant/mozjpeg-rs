use libc::{c_float, c_int, c_short, c_uint, c_ushort};
extern "C" {
    #[no_mangle]
    pub fn jpeg_fdct_islow(data: *mut DCTELEM);

    #[no_mangle]
    pub fn jpeg_fdct_ifast(data: *mut DCTELEM);

    #[no_mangle]
    pub fn jpeg_fdct_float(data: *mut c_float);

    #[no_mangle]
    pub fn jpeg_idct_islow(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_ifast(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_float(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_7x7(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_6x6(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_5x5(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_4x4(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_3x3(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_2x2(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_1x1(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_9x9(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_10x10(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_11x11(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_12x12(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_13x13(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_14x14(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_15x15(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jpeg_idct_16x16(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );
}
// =============== BEGIN jdct_h ================

/* 16 bits is OK, use short if faster */
pub const IFAST_SCALE_BITS: c_int = 2i32;
/*
 * jdct.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This include file contains common declarations for the forward and
 * inverse DCT modules.  These declarations are private to the DCT managers
 * (jcdctmgr.c, jddctmgr.c) and the individual DCT algorithms.
 * The individual DCT algorithms are kept in separate files to ease
 * machine-dependent tuning (e.g., assembly coding).
 */

/*
 * A forward DCT routine is given a pointer to a work area of type DCTELEM[];
 * the DCT is to be performed in-place in that buffer.  Type DCTELEM is int
 * for 8-bit samples, JLONG for 12-bit samples.  (NOTE: Floating-point DCT
 * implementations use an array of type FAST_FLOAT, instead.)
 * The DCT inputs are expected to be signed (range +-CENTERJSAMPLE).
 * The DCT outputs are returned scaled up by a factor of 8; they therefore
 * have a range of +-8K for 8-bit data, +-128K for 12-bit data.  This
 * convention improves accuracy in integer implementations and saves some
 * work in floating-point ones.
 * Quantization of the output coefficients is done by jcdctmgr.c. This
 * step requires an unsigned type and also one with twice the bits.
 */
pub type DCTELEM = c_short;
pub type UDCTELEM2 = c_uint;
/* prefer 16 bit with SIMD for parellelism */
pub type UDCTELEM = c_ushort;
/* 16 bits is OK, use short if faster */

/* fractional bits in scale factors */
pub type FLOAT_MULT_TYPE = c_float;
/* short or int, whichever is faster */
pub type IFAST_MULT_TYPE = c_short;
/*
 * An inverse DCT routine is given a pointer to the input JBLOCK and a pointer
 * to an output sample array.  The routine must dequantize the input data as
 * well as perform the IDCT; for dequantization, it uses the multiplier table
 * pointed to by compptr->dct_table.  The output data is to be placed into the
 * sample array starting at a specified column.  (Any row offset needed will
 * be applied to the array pointer before it is passed to the IDCT code.)
 * Note that the number of samples emitted by the IDCT routine is
 * DCT_scaled_size * DCT_scaled_size.
 */

/* typedef inverse_DCT_method_ptr is declared in jpegint.h */

/*
 * Each IDCT routine has its own ideas about the best dct_table element type.
 */
pub type ISLOW_MULT_TYPE = c_short;

use crate::jmorecfg_h::{JDIMENSION, MAXJSAMPLE};
use crate::jpeglib_h::{
    j_decompress_ptr, jpeg_component_info, jpeg_decompress_struct, JCOEFPTR, JSAMPARRAY, JSAMPROW,
};
/* preferred floating type */

/*
 * Each IDCT routine is responsible for range-limiting its results and
 * converting them to unsigned form (0..MAXJSAMPLE).  The raw outputs could
 * be quite far out of range if the input data is corrupt, so a bulletproof
 * range-limiting step is required.  We use a mask-and-table-lookup method
 * to do the combined operations quickly.  See the comments with
 * prepare_range_limit_table (in jdmaster.c) for more info.
 */
pub const RANGE_MASK: c_int = MAXJSAMPLE * 4i32 + 3i32;
/*
 * Macros for handling fixed-point arithmetic; these are used by many
 * but not all of the DCT/IDCT modules.
 *
 * All values are expected to be of type JLONG.
 * Fractional constants are scaled left by CONST_BITS bits.
 * CONST_BITS is defined within each module using these macros,
 * and may differ from one module to the next.
 */
pub const ONE: c_int = 1i32;
