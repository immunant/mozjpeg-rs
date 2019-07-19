pub use crate::md5::{uint32, MD5Context, MD5Final, MD5Init, MD5Update, MD5_CTX};
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    __blkcnt_t, __blksize_t, __dev_t, __fxstat, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
    __ssize_t, __syscall_slong_t, __time_t, __uid_t, fstat, off_t, ssize_t, stat, timespec,
    O_RDONLY, SEEK_SET, _STAT_VER, _STAT_VER_LINUX,
};
use crate::stdlib::{__errno_location, close, lseek, malloc, open, read};
use libc::{self, c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
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
pub const LENGTH: c_int = 16i32;
#[no_mangle]
pub unsafe extern "C" fn MD5End(mut ctx: *mut MD5_CTX, mut buf: *mut c_char) -> *mut c_char {
    let mut i: c_int = 0;
    let mut digest: [c_uchar; 16] = [0; 16];
    static mut hex: [c_char; 17] = [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 0,
    ];
    if buf.is_null() {
        buf = malloc((2i32 * LENGTH + 1i32) as c_ulong) as *mut c_char
    }
    if buf.is_null() {
        return 0 as *mut c_char;
    }
    MD5Final(digest.as_mut_ptr(), ctx);
    i = 0i32;
    while i < LENGTH {
        *buf.offset((i + i) as isize) = hex[(digest[i as usize] as c_int >> 4i32) as usize];
        *buf.offset((i + i + 1i32) as isize) = hex[(digest[i as usize] as c_int & 0xfi32) as usize];
        i += 1
    }
    *buf.offset((i + i) as isize) = '\u{0}' as i32 as c_char;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn MD5File(mut filename: *const c_char, mut buf: *mut c_char) -> *mut c_char {
    return MD5FileChunk(filename, buf, 0i32 as off_t, 0i32 as off_t);
}
#[no_mangle]
pub unsafe extern "C" fn MD5FileChunk(
    mut filename: *const c_char,
    mut buf: *mut c_char,
    mut ofs: off_t,
    mut len: off_t,
) -> *mut c_char {
    let mut buffer: [c_uchar; 8192] = [0; 8192];
    let mut ctx: MD5_CTX = MD5_CTX {
        buf: [0; 4],
        bits: [0; 2],
        in_0: [0; 64],
    };
    let mut stbuf: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut f: c_int = 0;
    let mut i: c_int = 0;
    let mut e: c_int = 0;
    let mut n: off_t = 0;
    MD5Init(&mut ctx);
    f = open(filename, O_RDONLY);
    if f < 0i32 {
        return 0 as *mut c_char;
    }
    if fstat(f, &mut stbuf) < 0i32 {
        return 0 as *mut c_char;
    }
    if ofs > stbuf.st_size {
        ofs = stbuf.st_size
    }
    if len == 0i32 as c_long || len > stbuf.st_size - ofs {
        len = stbuf.st_size - ofs
    }
    if lseek(f, ofs, SEEK_SET) < 0i32 as c_long {
        return 0 as *mut c_char;
    }
    n = len;
    i = 0i32;
    while n > 0i32 as c_long {
        if n as c_ulong > ::std::mem::size_of::<[c_uchar; 8192]>() as c_ulong {
            i = read(
                f,
                buffer.as_mut_ptr() as *mut c_void,
                ::std::mem::size_of::<[c_uchar; 8192]>() as c_ulong,
            ) as c_int
        } else {
            i = read(f, buffer.as_mut_ptr() as *mut c_void, n as size_t) as c_int
        }
        if i < 0i32 {
            break;
        }
        MD5Update(&mut ctx, buffer.as_mut_ptr(), i as c_uint);
        n -= i as c_long
    }
    e = *__errno_location();
    close(f);
    *__errno_location() = e;
    if i < 0i32 {
        return 0 as *mut c_char;
    }
    return MD5End(&mut ctx, buf);
}
#[no_mangle]
pub unsafe extern "C" fn MD5Data(
    mut data: *const c_void,
    mut len: c_uint,
    mut buf: *mut c_char,
) -> *mut c_char {
    let mut ctx: MD5_CTX = MD5_CTX {
        buf: [0; 4],
        bits: [0; 2],
        in_0: [0; 64],
    };
    MD5Init(&mut ctx);
    MD5Update(&mut ctx, data as *mut c_uchar, len);
    return MD5End(&mut ctx, buf);
}
