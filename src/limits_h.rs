use ::libc;
use libc::c_int;
pub const INT_MAX: c_int = __INT_MAX__;
pub const INT_MIN: c_int = -__INT_MAX__ - 1i32;
use crate::internal::__INT_MAX__;
pub const CHAR_BIT: c_int = 8i32;
