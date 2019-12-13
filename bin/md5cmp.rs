#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
pub mod md5_h {
    extern "C" {
        #[no_mangle]
        pub fn MD5File(_: *const libc::c_char, _: *mut libc::c_char) -> *mut libc::c_char;
    }
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;

    pub const NULL: libc::c_int = 0 as libc::c_int;
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub static mut stderr: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn perror(__s: *const libc::c_char);
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        pub type _IO_wide_data;

        pub type _IO_codecvt;

        pub type _IO_marker;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: crate::stddef_h::size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }

    pub type _IO_lock_t = ();
    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}
use ::mozjpeg::*;

use crate::md5_h::MD5File;
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
    if argc < 3 as libc::c_int {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"USAGE: %s <correct MD5 sum> <file>\n\x00" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return -(1 as libc::c_int);
    }
    if crate::stdlib::strlen(*argv.offset(1 as libc::c_int as isize))
        != 32 as libc::c_int as libc::c_ulong
    {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"WARNING: MD5 hash size is wrong.\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    md5sum = crate::md5_h::MD5File(*argv.offset(2 as libc::c_int as isize), buf.as_mut_ptr());
    if md5sum.is_null() {
        crate::stdlib::perror(b"Could not obtain MD5 sum\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if crate::stdlib::strcasecmp(md5sum, *argv.offset(1 as libc::c_int as isize)) == 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: OK\n\x00" as *const u8 as *const libc::c_char,
            *argv.offset(2 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    } else {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: FAILED.  Checksum is %s\n\x00" as *const u8 as *const libc::c_char,
            *argv.offset(2 as libc::c_int as isize),
            md5sum,
        );
        return -(1 as libc::c_int);
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
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
