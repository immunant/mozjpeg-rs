use libc::c_int;use libc::c_char;pub const SIZEOF_SIZE_T: c_int = 8i32;
use ::libc;
/* libjpeg-turbo build number */
pub const BUILD: [c_char; 9] =
    unsafe { *::std::mem::transmute::<&[u8; 9], &[c_char; 9]>(b"20190709\x00") };
/* Compiler's inline keyword */

/* How to obtain function inlining. */

/* Define to the full name of this package. */
pub const PACKAGE_NAME: [c_char; 8] =
    unsafe { *::std::mem::transmute::<&[u8; 8], &[c_char; 8]>(b"mozjpeg\x00") };
/* Version number of package */
pub const VERSION: [c_char; 6] =
    unsafe { *::std::mem::transmute::<&[u8; 6], &[c_char; 6]>(b"4.0.0\x00") };
