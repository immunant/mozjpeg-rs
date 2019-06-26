use libc;
use libc::c_int;
use libc::c_long;
use libc::c_ulong;
use libc::c_void;

pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::stddef_h::size_t;
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
pub static mut jpeg_natural_order: [c_int; 80] = [
    0i32, 1i32, 8i32, 16i32, 9i32, 2i32, 3i32, 10i32, 17i32, 24i32, 32i32, 25i32, 18i32, 11i32,
    4i32, 5i32, 12i32, 19i32, 26i32, 33i32, 40i32, 48i32, 41i32, 34i32, 27i32, 20i32, 13i32, 6i32,
    7i32, 14i32, 21i32, 28i32, 35i32, 42i32, 49i32, 56i32, 57i32, 50i32, 43i32, 36i32, 29i32,
    22i32, 15i32, 23i32, 30i32, 37i32, 44i32, 51i32, 58i32, 59i32, 52i32, 45i32, 38i32, 31i32,
    39i32, 46i32, 53i32, 60i32, 61i32, 54i32, 47i32, 55i32, 62i32, 63i32, 63i32, 63i32, 63i32,
    63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32, 63i32,
];
/* Utility routines in jutils.c */
/* extra entries for safety in decoder */
/*
 * Arithmetic utilities
 */
#[no_mangle]
pub unsafe extern "C" fn jdiv_round_up(mut a: c_long, mut b: c_long) -> c_long {
    return (a + b - 1i64) / b;
}
#[no_mangle]
pub unsafe extern "C" fn jround_up(mut a: c_long, mut b: c_long) -> c_long {
    a += b - 1i64;
    return a - a % b;
}
#[no_mangle]
pub unsafe extern "C" fn jcopy_sample_rows(
    mut input_array: JSAMPARRAY,
    mut source_row: c_int,
    mut output_array: JSAMPARRAY,
    mut dest_row: c_int,
    mut num_rows: c_int,
    mut num_cols: JDIMENSION,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut count: size_t =
        (num_cols as c_ulong).wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong);
    let mut row: c_int = 0;
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
        memcpy(outptr as *mut c_void, inptr as *const c_void, count);
        row -= 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn jcopy_block_row(
    mut input_row: JBLOCKROW,
    mut output_row: JBLOCKROW,
    mut num_blocks: JDIMENSION,
) {
    memcpy(
        output_row as *mut c_void,
        input_row as *const c_void,
        (num_blocks as c_ulong).wrapping_mul(
            (64i32 as c_ulong).wrapping_mul(::std::mem::size_of::<JCOEF>() as c_ulong),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn jzero_far(mut target: *mut c_void, mut bytestozero: size_t) {
    memset(target, 0i32, bytestozero);
}
