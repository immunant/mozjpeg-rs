use libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
/*
 * jutils.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code
 * relevant to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains tables and miscellaneous utility routines needed
 * for both compression and decompression.
 * Note we prefix all global names with "j" to minimize conflicts with
 * a surrounding application.
 */
/*
 * jpeg_zigzag_order[i] is the zigzag-order position of the i'th element
 * of a DCT block read in natural order (left to right, top to bottom).
 */
/* This table is not actually needed in v6a */
/*
 * jpeg_natural_order[i] is the natural-order position of the i'th element
 * of zigzag order.
 *
 * When reading corrupted data, the Huffman decoders could attempt
 * to reference an entry beyond the end of this array (if the decoded
 * zero run length reaches past the end of the block).  To prevent
 * wild stores without adding an inner-loop test, we put some extra
 * "63"s after the real entries.  This will cause the extra coefficient
 * to be stored in location 63 of the block, not somewhere random.
 * The worst case would be a run-length of 15, which means we need 16
 * fake entries.
 */
#[no_mangle]

pub static mut jpeg_natural_order: [libc::c_int; 80] = [
    0i32, 1i32, 8i32, 16i32, 9i32, 2i32, 3i32, 10i32, 17i32, 24i32, 32i32, 25i32, 18i32, 11i32,
    4i32, 5i32, 12i32, 19i32, 26i32, 33i32, 40i32, 48i32, 41i32, 34i32, 27i32, 20i32, 13i32, 6i32,
    7i32, 14i32, 21i32, 28i32, 35i32, 42i32, 49i32, 56i32, 57i32, 50i32, 43i32, 36i32, 29i32,
    22i32, 15i32, 23i32, 30i32, 37i32, 44i32, 51i32, 58i32, 59i32, 52i32, 45i32, 38i32, 31i32,
    39i32, 46i32, 53i32, 60i32, 61i32, 54i32, 47i32, 55i32, 62i32, 63i32, 63i32, 63i32, 63i32,
    63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32,
];
/*
 * Arithmetic utilities
 */
#[no_mangle]

pub unsafe extern "C" fn jdiv_round_up(mut a: libc::c_long, mut b: libc::c_long) -> libc::c_long
/* Compute a/b rounded up to next integer, ie, ceil(a/b) */
/* Assumes a >= 0, b > 0 */ {
    return (a + b - 1i64) / b;
}
#[no_mangle]

pub unsafe extern "C" fn jround_up(mut a: libc::c_long, mut b: libc::c_long) -> libc::c_long
/* Compute a rounded up to next multiple of b, ie, ceil(a/b)*b */
/* Assumes a >= 0, b > 0 */ {
    a += b - 1i64;
    return a - a % b;
}
#[no_mangle]

pub unsafe extern "C" fn jcopy_sample_rows(
    mut input_array: crate::jpeglib_h::JSAMPARRAY,
    mut source_row: libc::c_int,
    mut output_array: crate::jpeglib_h::JSAMPARRAY,
    mut dest_row: libc::c_int,
    mut num_rows: libc::c_int,
    mut num_cols: crate::jmorecfg_h::JDIMENSION,
)
/* Copy some rows of samples from one place to another.
 * num_rows rows are copied from input_array[source_row++]
 * to output_array[dest_row++]; these areas may overlap for duplication.
 * The source and destination arrays must be at least as wide as num_cols.
 */
{
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut count: crate::stddef_h::size_t = num_cols as libc::c_ulong *
    ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong;
    let mut row: libc::c_int = 0;
    input_array = input_array.offset(source_row as isize);
    output_array = output_array.offset(dest_row as isize);
    row = num_rows;
    while row > 0i32 {
        let fresh0 = input_array;
        input_array = input_array.offset(1);
        inptr = *fresh0;
        let fresh1 = output_array;
        output_array = output_array.offset(1);
        outptr = *fresh1;
        crate::stdlib::memcpy(
            outptr as *mut libc::c_void,
            inptr as *const libc::c_void,
            count,
        );
        row -= 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn jcopy_block_row(
    mut input_row: crate::jpeglib_h::JBLOCKROW,
    mut output_row: crate::jpeglib_h::JBLOCKROW,
    mut num_blocks: crate::jmorecfg_h::JDIMENSION,
)
/* Copy a row of coefficient blocks from one place to another. */
{
    crate::stdlib::memcpy(
        output_row as *mut libc::c_void,
        input_row as *const libc::c_void,
        num_blocks as libc::c_ulong *
    (64u64 *
         ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong),
    );
}
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Compression module initialization routines */
/* Decompression module initialization routines */
/* Memory manager initialization */
/* Utility routines in jutils.c */
#[no_mangle]

pub unsafe extern "C" fn jzero_far(
    mut target: *mut libc::c_void,
    mut bytestozero: crate::stddef_h::size_t,
)
/* Zero out a chunk of memory. */
/* This might be sample-array data, block-array data, or alloc_large data. */
{
    crate::stdlib::memset(target, 0i32, bytestozero);
}
