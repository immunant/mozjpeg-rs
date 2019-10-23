#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(main)]


use mozjpeg::*;


use crate::src::md5::md5::MD5File;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
use crate::stdlib::fprintf;
use crate::stdlib::perror;
use crate::stdlib::stderr;
use crate::stdlib::strcasecmp;
use crate::stdlib::strlen;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
/*
 * Copyright (C)2013, 2016 D. R. Commander.  All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 * - Neither the name of the libjpeg-turbo Project nor the names of its
 *   contributors may be used to endorse or promote products derived from this
 *   software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS",
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut md5sum: *mut libc::c_char = crate::stddef_h::NULL as *mut libc::c_char;
    let mut buf: [libc::c_char; 65] = [0; 65];
    if argc < 3i32 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            
            b"USAGE: %s <correct MD5 sum> <file>\n\x00".as_ptr() as *const libc::c_char,
            *argv.offset(0),
        );
        return -1i32;
    }
    if crate::stdlib::strlen(*argv.offset(1)) != 32u64 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            
            b"WARNING: MD5 hash size is wrong.\n\x00".as_ptr() as *const libc::c_char,
        );
    }
    md5sum = crate::src::md5::md5::MD5File(*argv.offset(2), buf.as_mut_ptr());
    if md5sum.is_null() {
        crate::stdlib::perror(b"Could not obtain MD5 sum\x00".as_ptr() as *const libc::c_char);
        return -1i32;
    }
    if crate::stdlib::strcasecmp(md5sum, *argv.offset(1)) == 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            
            b"%s: OK\n\x00".as_ptr() as *const libc::c_char,
            *argv.offset(2),
        );
        return 0i32;
    } else {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            
            b"%s: FAILED.  Checksum is %s\n\x00".as_ptr() as *const libc::c_char,
            *argv.offset(2),
            md5sum,
        );
        return -1i32;
    };
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            
            args.as_mut_ptr(),
        ))
    }
}
