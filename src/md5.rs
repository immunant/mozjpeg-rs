use libc::c_char;
use libc::c_uchar;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
extern "C" {
    #[no_mangle]
    pub fn MD5File(_: *const c_char, _: *mut c_char) -> *mut c_char;
}
use libc;

/*
 * libjpeg-turbo Modifications:
 * Copyright (C)2018 D. R. Commander.  All Rights Reserved.
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
/*  On machines where "long" is 64 bits, we need to declare
uint32 as something guaranteed to be 32 bits.  */
pub type uint32 = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MD5Context {
    pub buf: [uint32; 4],
    pub bits: [uint32; 2],
    pub in_0: [c_uchar; 64],
}
pub type MD5_CTX = MD5Context;
/*
 * This code implements the MD5 message-digest algorithm.
 * The algorithm is due to Ron Rivest.  This code was
 * written by Colin Plumb in 1993, no copyright is claimed.
 * This code is in the public domain; do with it what you wish.
 *
 * Equivalent code is available from RSA Data Security, Inc.
 * This code has been tested against that, and is equivalent,
 * except that you don't need to include two pages of legalese
 * with every copy.
 *
 * To compute the message digest of a chunk of bytes, declare an
 * MD5Context structure, pass it to MD5Init, call MD5Update as
 * needed on buffers full of bytes, and then call MD5Final, which
 * will fill a supplied 16-byte array with the digest.
 * ----------------------------------------------------------------------------
 * libjpeg-turbo Modifications:
 * Copyright (C)2018, D. R. Commander.  All Rights Reserved.
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
/* for memcpy() */
/* Nothing */
/*
 * Start MD5 accumulation.  Set bit count to 0 and buffer to mysterious
 * initialization constants.
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Init(mut ctx: *mut MD5Context) {
    (*ctx).buf[0usize] = 0x67452301i32 as uint32;
    (*ctx).buf[1usize] = 0xefcdab89u32;
    (*ctx).buf[2usize] = 0x98badcfeu32;
    (*ctx).buf[3usize] = 0x10325476i32 as uint32;
    (*ctx).bits[0usize] = 0i32 as uint32;
    (*ctx).bits[1usize] = 0i32 as uint32;
}
/*
 * Update context to reflect the concatenation of another buffer full
 * of bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Update(
    mut ctx: *mut MD5Context,
    mut buf: *mut c_uchar,
    mut len: c_uint,
) {
    let mut t: uint32 = 0;
    t = (*ctx).bits[0usize];
    (*ctx).bits[0usize] = t.wrapping_add(len << 3i32);
    if (*ctx).bits[0usize] < t {
        (*ctx).bits[1usize] = (*ctx).bits[1usize].wrapping_add(1)
    }
    (*ctx).bits[1usize] =
        ((*ctx).bits[1usize] as c_uint).wrapping_add(len >> 29i32) as uint32 as uint32;
    t = t >> 3i32 & 0x3fi32 as c_uint;
    if 0 != t {
        let mut p: *mut c_uchar = (*ctx).in_0.as_mut_ptr().offset(t as isize);
        t = (64i32 as c_uint).wrapping_sub(t);
        if len < t {
            memcpy(p as *mut c_void, buf as *const c_void, len as c_ulong);
            return;
        }
        memcpy(p as *mut c_void, buf as *const c_void, t as c_ulong);
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut uint32,
        );
        buf = buf.offset(t as isize);
        len = len.wrapping_sub(t)
    }
    while len >= 64i32 as c_uint {
        memcpy(
            (*ctx).in_0.as_mut_ptr() as *mut c_void,
            buf as *const c_void,
            64i32 as c_ulong,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut uint32,
        );
        buf = buf.offset(64isize);
        len = len.wrapping_sub(64i32 as c_uint)
    }
    memcpy(
        (*ctx).in_0.as_mut_ptr() as *mut c_void,
        buf as *const c_void,
        len as c_ulong,
    );
}
/*
 * Final wrapup - pad to 64-byte boundary with the bit pattern
 * 1 0* (64-bit count of bits processed, MSB-first)
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Final(mut digest: *mut c_uchar, mut ctx: *mut MD5Context) {
    let mut count: c_uint = 0;
    let mut p: *mut c_uchar = 0 as *mut c_uchar;
    let mut in32: *mut uint32 = (*ctx).in_0.as_mut_ptr() as *mut uint32;
    count = (*ctx).bits[0usize] >> 3i32 & 0x3fi32 as c_uint;
    p = (*ctx).in_0.as_mut_ptr().offset(count as isize);
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = 0x80i32 as c_uchar;
    count = ((64i32 - 1i32) as c_uint).wrapping_sub(count);
    if count < 8i32 as c_uint {
        memset(p as *mut c_void, 0i32, count as c_ulong);
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut uint32,
        );
        memset(
            (*ctx).in_0.as_mut_ptr() as *mut c_void,
            0i32,
            56i32 as c_ulong,
        );
    } else {
        memset(
            p as *mut c_void,
            0i32,
            count.wrapping_sub(8i32 as c_uint) as c_ulong,
        );
    }
    *in32.offset(14isize) = (*ctx).bits[0usize];
    *in32.offset(15isize) = (*ctx).bits[1usize];
    MD5Transform(
        (*ctx).buf.as_mut_ptr(),
        (*ctx).in_0.as_mut_ptr() as *mut uint32,
    );
    memcpy(
        digest as *mut c_void,
        (*ctx).buf.as_mut_ptr() as *const c_void,
        16i32 as c_ulong,
    );
    memset(
        ctx as *mut c_void,
        0i32,
        ::std::mem::size_of::<MD5Context>() as c_ulong,
    );
}
use crate::stdlib::memcpy;
use crate::stdlib::memset;
/* The four core functions - F1 is optimized somewhat */
/* #define F1(x, y, z) (x & y | ~x & z) */
/* This is the central step in the MD5 algorithm. */
/*
 * The core of the MD5 algorithm, this alters an existing MD5 hash to
 * reflect the addition of 16 longwords of new data.  MD5Update blocks
 * the data and converts bytes into longwords for this routine.
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Transform(mut buf: *mut uint32, mut in_0: *mut uint32) {
    let mut a: uint32 = 0;
    let mut b: uint32 = 0;
    let mut c: uint32 = 0;
    let mut d: uint32 = 0;
    a = *buf.offset(0isize);
    b = *buf.offset(1isize);
    c = *buf.offset(2isize);
    d = *buf.offset(3isize);
    a = (a as c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(0isize))
            .wrapping_add(0xd76aa478u32),
    ) as uint32 as uint32;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(1isize))
            .wrapping_add(0xe8c7b756u32),
    ) as uint32 as uint32;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(2isize))
            .wrapping_add(0x242070dbi32 as c_uint),
    ) as uint32 as uint32;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(3isize))
            .wrapping_add(0xc1bdceeeu32),
    ) as uint32 as uint32;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(4isize))
            .wrapping_add(0xf57c0fafu32),
    ) as uint32 as uint32;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(5isize))
            .wrapping_add(0x4787c62ai32 as c_uint),
    ) as uint32 as uint32;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(6isize))
            .wrapping_add(0xa8304613u32),
    ) as uint32 as uint32;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(7isize))
            .wrapping_add(0xfd469501u32),
    ) as uint32 as uint32;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(8isize))
            .wrapping_add(0x698098d8i32 as c_uint),
    ) as uint32 as uint32;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(9isize))
            .wrapping_add(0x8b44f7afu32),
    ) as uint32 as uint32;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(10isize))
            .wrapping_add(0xffff5bb1u32),
    ) as uint32 as uint32;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(11isize))
            .wrapping_add(0x895cd7beu32),
    ) as uint32 as uint32;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(12isize))
            .wrapping_add(0x6b901122i32 as c_uint),
    ) as uint32 as uint32;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(13isize))
            .wrapping_add(0xfd987193u32),
    ) as uint32 as uint32;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(14isize))
            .wrapping_add(0xa679438eu32),
    ) as uint32 as uint32;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(15isize))
            .wrapping_add(0x49b40821i32 as c_uint),
    ) as uint32 as uint32;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(1isize))
            .wrapping_add(0xf61e2562u32),
    ) as uint32 as uint32;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(6isize))
            .wrapping_add(0xc040b340u32),
    ) as uint32 as uint32;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(11isize))
            .wrapping_add(0x265e5a51i32 as c_uint),
    ) as uint32 as uint32;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(0isize))
            .wrapping_add(0xe9b6c7aau32),
    ) as uint32 as uint32;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(5isize))
            .wrapping_add(0xd62f105du32),
    ) as uint32 as uint32;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(10isize))
            .wrapping_add(0x2441453i32 as c_uint),
    ) as uint32 as uint32;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(15isize))
            .wrapping_add(0xd8a1e681u32),
    ) as uint32 as uint32;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(4isize))
            .wrapping_add(0xe7d3fbc8u32),
    ) as uint32 as uint32;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(9isize))
            .wrapping_add(0x21e1cde6i32 as c_uint),
    ) as uint32 as uint32;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(14isize))
            .wrapping_add(0xc33707d6u32),
    ) as uint32 as uint32;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(3isize))
            .wrapping_add(0xf4d50d87u32),
    ) as uint32 as uint32;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(8isize))
            .wrapping_add(0x455a14edi32 as c_uint),
    ) as uint32 as uint32;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(13isize))
            .wrapping_add(0xa9e3e905u32),
    ) as uint32 as uint32;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(2isize))
            .wrapping_add(0xfcefa3f8u32),
    ) as uint32 as uint32;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(7isize))
            .wrapping_add(0x676f02d9i32 as c_uint),
    ) as uint32 as uint32;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(12isize))
            .wrapping_add(0x8d2a4c8au32),
    ) as uint32 as uint32;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(5isize))
            .wrapping_add(0xfffa3942u32),
    ) as uint32 as uint32;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(8isize))
            .wrapping_add(0x8771f681u32),
    ) as uint32 as uint32;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(11isize))
            .wrapping_add(0x6d9d6122i32 as c_uint),
    ) as uint32 as uint32;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(14isize))
            .wrapping_add(0xfde5380cu32),
    ) as uint32 as uint32;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(1isize))
            .wrapping_add(0xa4beea44u32),
    ) as uint32 as uint32;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(4isize))
            .wrapping_add(0x4bdecfa9i32 as c_uint),
    ) as uint32 as uint32;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(7isize))
            .wrapping_add(0xf6bb4b60u32),
    ) as uint32 as uint32;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(10isize))
            .wrapping_add(0xbebfbc70u32),
    ) as uint32 as uint32;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(13isize))
            .wrapping_add(0x289b7ec6i32 as c_uint),
    ) as uint32 as uint32;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(0isize))
            .wrapping_add(0xeaa127fau32),
    ) as uint32 as uint32;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(3isize))
            .wrapping_add(0xd4ef3085u32),
    ) as uint32 as uint32;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(6isize))
            .wrapping_add(0x4881d05i32 as c_uint),
    ) as uint32 as uint32;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(9isize))
            .wrapping_add(0xd9d4d039u32),
    ) as uint32 as uint32;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(12isize))
            .wrapping_add(0xe6db99e5u32),
    ) as uint32 as uint32;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(15isize))
            .wrapping_add(0x1fa27cf8i32 as c_uint),
    ) as uint32 as uint32;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(2isize))
            .wrapping_add(0xc4ac5665u32),
    ) as uint32 as uint32;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(0isize))
            .wrapping_add(0xf4292244u32),
    ) as uint32 as uint32;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(7isize))
            .wrapping_add(0x432aff97i32 as c_uint),
    ) as uint32 as uint32;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(14isize))
            .wrapping_add(0xab9423a7u32),
    ) as uint32 as uint32;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(5isize))
            .wrapping_add(0xfc93a039u32),
    ) as uint32 as uint32;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(12isize))
            .wrapping_add(0x655b59c3i32 as c_uint),
    ) as uint32 as uint32;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(3isize))
            .wrapping_add(0x8f0ccc92u32),
    ) as uint32 as uint32;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(10isize))
            .wrapping_add(0xffeff47du32),
    ) as uint32 as uint32;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(1isize))
            .wrapping_add(0x85845dd1u32),
    ) as uint32 as uint32;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(8isize))
            .wrapping_add(0x6fa87e4fi32 as c_uint),
    ) as uint32 as uint32;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(15isize))
            .wrapping_add(0xfe2ce6e0u32),
    ) as uint32 as uint32;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(6isize))
            .wrapping_add(0xa3014314u32),
    ) as uint32 as uint32;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(13isize))
            .wrapping_add(0x4e0811a1i32 as c_uint),
    ) as uint32 as uint32;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    a = (a as c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(4isize))
            .wrapping_add(0xf7537e82u32),
    ) as uint32 as uint32;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as c_uint).wrapping_add(b) as uint32 as uint32;
    d = (d as c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(11isize))
            .wrapping_add(0xbd3af235u32),
    ) as uint32 as uint32;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as c_uint).wrapping_add(a) as uint32 as uint32;
    c = (c as c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(2isize))
            .wrapping_add(0x2ad7d2bbi32 as c_uint),
    ) as uint32 as uint32;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as c_uint).wrapping_add(d) as uint32 as uint32;
    b = (b as c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(9isize))
            .wrapping_add(0xeb86d391u32),
    ) as uint32 as uint32;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as c_uint).wrapping_add(c) as uint32 as uint32;
    let ref mut fresh1 = *buf.offset(0isize);
    *fresh1 = (*fresh1 as c_uint).wrapping_add(a) as uint32 as uint32;
    let ref mut fresh2 = *buf.offset(1isize);
    *fresh2 = (*fresh2 as c_uint).wrapping_add(b) as uint32 as uint32;
    let ref mut fresh3 = *buf.offset(2isize);
    *fresh3 = (*fresh3 as c_uint).wrapping_add(c) as uint32 as uint32;
    let ref mut fresh4 = *buf.offset(3isize);
    *fresh4 = (*fresh4 as c_uint).wrapping_add(d) as uint32 as uint32;
}
