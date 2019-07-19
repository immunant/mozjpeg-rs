use libc::{self, c_int};
pub const INT_MAX: c_int = __INT_MAX__;
pub const INT_MIN: c_int = -__INT_MAX__ - 1i32;
use crate::__INT_MAX__;
pub const CHAR_BIT: c_int = 8i32;
