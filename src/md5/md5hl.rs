use libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__blkcnt_t;
pub use crate::stdlib::__blksize_t;
pub use crate::stdlib::__dev_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__ino_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__nlink_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__syscall_slong_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::timespec;

pub use crate::src::md5::md5::uint32;
pub use crate::src::md5::md5::MD5Context;
pub use crate::src::md5::md5::MD5Final;
pub use crate::src::md5::md5::MD5Init;
pub use crate::src::md5::md5::MD5Update;
pub use crate::src::md5::md5::MD5_CTX;
use crate::stdlib::__errno_location;
pub use crate::stdlib::__fxstat;
use crate::stdlib::close;
pub use crate::stdlib::fstat;
use crate::stdlib::lseek;
use crate::stdlib::malloc;
use crate::stdlib::open;
use crate::stdlib::read;
pub use crate::stdlib::stat;
pub use crate::stdlib::O_RDONLY;
pub use crate::stdlib::SEEK_SET;
pub use crate::stdlib::_STAT_VER;
pub use crate::stdlib::_STAT_VER_LINUX;
/* mdXhl.c
 * ----------------------------------------------------------------------------
 * "THE BEER-WARE LICENSE" (Revision 42):
 * <phk@FreeBSD.org> wrote this file.  As long as you retain this notice you
 * can do whatever you want with this stuff. If we meet some day, and you think
 * this stuff is worth it, you can buy me a beer in return.   Poul-Henning Kamp
 * ----------------------------------------------------------------------------
 * libjpeg-turbo Modifications:
 * Copyright (C)2016, 2018 D. R. Commander.  All Rights Reserved.
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
 * ----------------------------------------------------------------------------
 */

pub const LENGTH: libc::c_int = 16i32;
#[no_mangle]

pub unsafe extern "C" fn MD5End(
    mut ctx: *mut crate::src::md5::md5::MD5_CTX,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut digest: [libc::c_uchar; 16] = [0; 16];
    static mut hex: [libc::c_char; 17] = [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 0,
    ];
    if buf.is_null() {
        buf = crate::stdlib::malloc((2i32 * LENGTH + 1i32) as libc::c_ulong) as *mut libc::c_char
    }
    if buf.is_null() {
        return ::std::ptr::null_mut::< libc::c_char>();
    }
    crate::src::md5::md5::MD5Final(digest.as_mut_ptr(), ctx);
    i = 0i32;
    while i < LENGTH {
        *buf.offset((i + i) as isize) = hex[(digest[i as usize] as libc::c_int >> 4i32) as usize];
        *buf.offset((i + i + 1i32) as isize) =
            hex[(digest[i as usize] as libc::c_int & 0xfi32) as usize];
        i += 1
    }
    *buf.offset((i + i) as isize) =  '\u{0}' as libc::c_char;
    return buf;
}
#[no_mangle]

pub unsafe extern "C" fn MD5File(
    mut filename: *const libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    return MD5FileChunk(
        filename,
        buf,
        0i64,
        0i64,
    );
}
#[no_mangle]

pub unsafe extern "C" fn MD5FileChunk(
    mut filename: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut ofs: crate::stdlib::off_t,
    mut len: crate::stdlib::off_t,
) -> *mut libc::c_char {
    let mut buffer: [libc::c_uchar; 8192] = [0; 8192];
    let mut ctx: crate::src::md5::md5::MD5_CTX = crate::src::md5::md5::MD5_CTX {
        buf: [0; 4],
        bits: [0; 2],
        in_0: [0; 64],
    };
    let mut stbuf: crate::stdlib::stat = crate::stdlib::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut f: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut n: crate::stdlib::off_t = 0;
    crate::src::md5::md5::MD5Init(&mut ctx);
    f = crate::stdlib::open(filename, crate::stdlib::O_RDONLY);
    if f < 0i32 {
        return ::std::ptr::null_mut::< libc::c_char>();
    }
    if crate::stdlib::fstat(f, &mut stbuf) < 0i32 {
        return ::std::ptr::null_mut::< libc::c_char>();
    }
    if ofs > stbuf.st_size {
        ofs = stbuf.st_size
    }
    if len == 0i64 || len > stbuf.st_size - ofs {
        len = stbuf.st_size - ofs
    }
    if crate::stdlib::lseek(f, ofs, crate::stdlib::SEEK_SET) < 0i64 {
        return ::std::ptr::null_mut::< libc::c_char>();
    }
    n = len;
    i = 0i32;
    while n > 0i64 {
        if n as libc::c_ulong > ::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong {
            i = crate::stdlib::read(
                f,
                buffer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_uchar; 8192]>() as libc::c_ulong,
            ) as libc::c_int
        } else {
            i = crate::stdlib::read(
                f,
                buffer.as_mut_ptr() as *mut libc::c_void,
                n as crate::stddef_h::size_t,
            ) as libc::c_int
        }
        if i < 0i32 {
            break;
        }
        crate::src::md5::md5::MD5Update(&mut ctx, buffer.as_mut_ptr(), i as libc::c_uint);
        n -= i as libc::c_long
    }
    e = *crate::stdlib::__errno_location();
    crate::stdlib::close(f);
    *crate::stdlib::__errno_location() = e;
    if i < 0i32 {
        return ::std::ptr::null_mut::< libc::c_char>();
    }
    return MD5End(&mut ctx, buf);
}
#[no_mangle]

pub unsafe extern "C" fn MD5Data(
    mut data: *const libc::c_void,
    mut len: libc::c_uint,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ctx: crate::src::md5::md5::MD5_CTX = crate::src::md5::md5::MD5_CTX {
        buf: [0; 4],
        bits: [0; 2],
        in_0: [0; 64],
    };
    crate::src::md5::md5::MD5Init(&mut ctx);
    crate::src::md5::md5::MD5Update(&mut ctx, data as *mut libc::c_uchar, len);
    return MD5End(&mut ctx, buf);
}
