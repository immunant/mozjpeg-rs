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




















use std::prelude::v1::*;use mozjpeg::*;use crate::src::md5::md5::MD5File;use crate::stdlib::{fprintf, perror, stderr, strcasecmp, strlen};use std::prelude::v1;use libc::{c_int, c_char};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE};pub use crate::stddef_h::{size_t, NULL};
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

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
     let mut buf:  [c_char; 65] =  [0; 65];let mut md5sum: *mut c_char = NULL as *mut c_char;
    
    if argc < 3i32 {
        fprintf(
            stderr,
            
            b"USAGE: %s <correct MD5 sum> <file>\n\x00".as_ptr() as *const c_char,
            *argv.offset(0),
        );
        return -1i32;
    }
    if strlen(*argv.offset(1)) != 32u64 {
        fprintf(
            stderr,
            
            b"WARNING: MD5 hash size is wrong.\n\x00".as_ptr() as *const c_char,
        );
    }
    md5sum = MD5File(*argv.offset(2), buf.as_mut_ptr());
    if md5sum.is_null() {
        perror(b"Could not obtain MD5 sum\x00".as_ptr() as *const c_char);
        return -1i32;
    }
    if strcasecmp(md5sum, *argv.offset(1)) == 0 {
        fprintf(
            stderr,
            
            b"%s: OK\n\x00".as_ptr() as *const c_char,
            *argv.offset(2),
        );
        return 0i32;
    } else {
        fprintf(
            stderr,
            
            b"%s: FAILED.  Checksum is %s\n\x00".as_ptr() as *const c_char,
            *argv.offset(2),
            md5sum,
        );
        return -1i32;
    };
}
#[main]
pub fn main() {
     let mut args:  Vec<*mut c_char> =  Vec::new();
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
            (args.len() - 1) as c_int,
            
            args.as_mut_ptr(),
        ))
    }
}
