pub const SIZEOF_SIZE_T: libc::c_int = 8i32;
/* libjpeg-turbo build number */
pub const BUILD: [libc::c_char; 9] =
    unsafe { *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"20191022\x00") };
/* Compiler's inline keyword */

/* How to obtain function inlining. */

/* Define to the full name of this package. */
pub const PACKAGE_NAME: [libc::c_char; 8] =
    unsafe { *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"mozjpeg\x00") };
/* Version number of package */
pub const VERSION: [libc::c_char; 6] =
    unsafe { *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"4.0.0\x00") };
