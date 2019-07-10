use libc::c_int;use ::libc;
/* Version ID for the JPEG library.
 * Might be useful for tests like "#if JPEG_LIB_VERSION >= 60".
 */

/* libjpeg-turbo version */

/* libjpeg-turbo version in integer form */

/* Support arithmetic encoding */

/* #undef C_ARITH_CODING_SUPPORTED */

/* Support arithmetic decoding */

/* #undef D_ARITH_CODING_SUPPORTED */

/* Support in-memory source/destination managers */

/* Use accelerated SIMD routines. */

/*
 * Define BITS_IN_JSAMPLE as either
 *   8   for 8-bit sample values (the usual setting)
 *   12  for 12-bit sample values
 * Only 8 and 12 are legal data precisions for lossy JPEG according to the
 * JPEG standard, and the IJG code does not support anything else!
 * We do not support run-time selection of data precision, sorry.
 */

/* use 8 or 12 */
pub const BITS_IN_JSAMPLE: c_int = 8i32;
/* Version ID for the JPEG library.
 * Might be useful for tests like "#if JPEG_LIB_VERSION >= 60".
 */
pub const JPEG_LIB_VERSION: c_int = 62i32;
