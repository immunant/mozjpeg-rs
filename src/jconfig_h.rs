use libc::c_int;
pub const BITS_IN_JSAMPLE: c_int = 8i32;
/* Version ID for the JPEG library.
 * Might be useful for tests like "#if JPEG_LIB_VERSION >= 60".
 */
pub const JPEG_LIB_VERSION: c_int = 62i32;
