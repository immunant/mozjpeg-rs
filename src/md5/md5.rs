extern "C" {
    #[no_mangle]
    pub fn MD5File(_: *const libc::c_char, _: *mut libc::c_char) -> *mut libc::c_char;
}
use libc;

use crate::stdlib::memcpy;
use crate::stdlib::memset;
// =============== BEGIN md5_h ================
pub type uint32 = libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MD5Context {
    pub buf: [crate::src::md5::md5::uint32; 4],
    pub bits: [crate::src::md5::md5::uint32; 2],
    pub in_0: [libc::c_uchar; 64],
}

pub type MD5_CTX = crate::src::md5::md5::MD5Context;
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

pub unsafe extern "C" fn MD5Init(mut ctx: *mut crate::src::md5::md5::MD5Context) {
    (*ctx).buf[0] = 0x67452301u32;
    (*ctx).buf[1] = 0xefcdab89u32;
    (*ctx).buf[2] = 0x98badcfeu32;
    (*ctx).buf[3] = 0x10325476u32;
    (*ctx).bits[0] = 0u32;
    (*ctx).bits[1] = 0u32;
}
/*
 * Update context to reflect the concatenation of another buffer full
 * of bytes.
 */
#[no_mangle]

pub unsafe extern "C" fn MD5Update(
    mut ctx: *mut crate::src::md5::md5::MD5Context,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_uint,
) {
     let mut t:  crate::src::md5::md5::uint32 =  0;
    /* Update bitcount */
    t = (*ctx).bits[0]; /* Carry from low to high */
    (*ctx).bits[0] =  t + (((len << 3i32))); /* Bytes already in shsInfo->data */
    if (*ctx).bits[0] < t {
        (*ctx).bits[1] =  (*ctx).bits[1] + 1
    }
    (*ctx).bits[1] = (*ctx).bits[1] + (((len >> 29i32)));
    t = t >> 3i32 & 0x3fu32;
    /* Handle any leading odd-sized chunks */
    if t != 0 {
        let mut p: *mut libc::c_uchar = (*ctx).in_0.as_mut_ptr().offset(t as isize);
        t = 64u32 - t;
        if len < t {
            crate::stdlib::memcpy(
                p as *mut libc::c_void,
                buf as *const libc::c_void,
                len as libc::c_ulong,
            );
            return;
        }
        crate::stdlib::memcpy(
            p as *mut libc::c_void,
            buf as *const libc::c_void,
            t as libc::c_ulong,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut crate::src::md5::md5::uint32,
        );
        buf = buf.offset(t as isize);
        len =  len - t
    }
    /* Process data in 64-byte chunks */
    while len >= 64u32 {
        crate::stdlib::memcpy(
            (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            64u64,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut crate::src::md5::md5::uint32,
        );
        buf = buf.offset(64);
        len =  len - 64u32
    }
    /* Handle any remaining bytes of data. */
    crate::stdlib::memcpy(
        (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
        buf as *const libc::c_void,
        len as libc::c_ulong,
    );
}
/*
 * Final wrapup - pad to 64-byte boundary with the bit pattern
 * 1 0* (64-bit count of bits processed, MSB-first)
 */
#[no_mangle]

pub unsafe extern "C" fn MD5Final(
    mut digest: *mut libc::c_uchar,
    mut ctx: *mut crate::src::md5::md5::MD5Context,
) {
    
     let mut count:  libc::c_uint =  0; let mut p:  *mut libc::c_uchar =  ::std::ptr::null_mut::< libc::c_uchar>();
    let mut in32: *mut crate::src::md5::md5::uint32 =
        (*ctx).in_0.as_mut_ptr() as *mut crate::src::md5::md5::uint32;
    /* Compute number of bytes mod 64 */
    count = (*ctx).bits[0] >> 3i32 & 0x3fu32;
    /* Set the first char of padding to 0x80.  This is safe since there is
    always at least one byte free */
    p = (*ctx).in_0.as_mut_ptr().offset(count as isize);
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = 0x80u8;
    /* Bytes of padding needed to make 64 bytes */
    count = (64i32 - 1i32) as libc::c_uint - count;
    /* Pad out to 56 mod 64 */
    if count < 8u32 {
        /* Two lots of padding:  Pad the first block to 64 bytes */
        crate::stdlib::memset(p as *mut libc::c_void, 0i32, count as libc::c_ulong);
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut crate::src::md5::md5::uint32,
        );
        /* Now fill the next block with 56 bytes */
        crate::stdlib::memset(
            (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
            0i32,
            56u64,
        );
    } else {
        /* Pad block to 56 bytes */
        crate::stdlib::memset(
            p as *mut libc::c_void,
            0i32,
            (
            count - 8u32) as libc::c_ulong,
        );
    }
    /* Append length in bits and transform */
    *in32.offset(14) = (*ctx).bits[0];
    *in32.offset(15) = (*ctx).bits[1];
    MD5Transform(
        (*ctx).buf.as_mut_ptr(),
        (*ctx).in_0.as_mut_ptr() as *mut crate::src::md5::md5::uint32,
    );
    crate::stdlib::memcpy(
        digest as *mut libc::c_void,
        (*ctx).buf.as_mut_ptr() as *const libc::c_void,
        16u64,
    );
    crate::stdlib::memset(
        ctx as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::src::md5::md5::MD5Context>() as libc::c_ulong,
    );
    /* In case it's sensitive */
}
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
/* The four core functions - F1 is optimized somewhat */
/* #define F1(x, y, z) (x & y | ~x & z) */
/* This is the central step in the MD5 algorithm. */
/*
 * The core of the MD5 algorithm, this alters an existing MD5 hash to
 * reflect the addition of 16 longwords of new data.  MD5Update blocks
 * the data and converts bytes into longwords for this routine.
 */
#[no_mangle]

pub unsafe extern "C" fn MD5Transform(
    mut buf: *mut crate::src::md5::md5::uint32,
    mut in_0: *mut crate::src::md5::md5::uint32,
) {
    
    
    
     let mut a:  crate::src::md5::md5::uint32 =  0; let mut b:  crate::src::md5::md5::uint32 =  0; let mut c:  crate::src::md5::md5::uint32 =  0; let mut d:  crate::src::md5::md5::uint32 =  0;
    a = *buf.offset(0);
    b = *buf.offset(1);
    c = *buf.offset(2);
    d = *buf.offset(3);
    a = a + ((((d ^ b & (c ^ d)))) + *in_0.offset(0) + 0xd76aa478u32);
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = a + b;
    d = d + ((((c ^ a & (b ^ c)))) + *in_0.offset(1) + 0xe8c7b756u32);
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = d + a;
    c = c +
    ((((b ^ d & (a ^ b)))) + *in_0.offset(2) + 0x242070dbu32);
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = c + d;
    b = b + ((((a ^ c & (d ^ a)))) + *in_0.offset(3) + 0xc1bdceeeu32);
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = b + c;
    a = a + ((((d ^ b & (c ^ d)))) + *in_0.offset(4) + 0xf57c0fafu32);
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = a + b;
    d = d +
    ((((c ^ a & (b ^ c)))) + *in_0.offset(5) + 0x4787c62au32);
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = d + a;
    c = c + ((((b ^ d & (a ^ b)))) + *in_0.offset(6) + 0xa8304613u32);
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = c + d;
    b = b + ((((a ^ c & (d ^ a)))) + *in_0.offset(7) + 0xfd469501u32);
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = b + c;
    a = a +
    ((((d ^ b & (c ^ d)))) + *in_0.offset(8) + 0x698098d8u32);
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = a + b;
    d = d + ((((c ^ a & (b ^ c)))) + *in_0.offset(9) + 0x8b44f7afu32);
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = d + a;
    c = c + ((((b ^ d & (a ^ b)))) + *in_0.offset(10) + 0xffff5bb1u32);
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = c + d;
    b = b + ((((a ^ c & (d ^ a)))) + *in_0.offset(11) + 0x895cd7beu32);
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = b + c;
    a = a +
    ((((d ^ b & (c ^ d)))) + *in_0.offset(12) + 0x6b901122u32);
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = a + b;
    d = d + ((((c ^ a & (b ^ c)))) + *in_0.offset(13) + 0xfd987193u32);
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = d + a;
    c = c + ((((b ^ d & (a ^ b)))) + *in_0.offset(14) + 0xa679438eu32);
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = c + d;
    b = b +
    ((((a ^ c & (d ^ a)))) + *in_0.offset(15) + 0x49b40821u32);
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = b + c;
    a = a + ((((c ^ d & (b ^ c)))) + *in_0.offset(1) + 0xf61e2562u32);
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = a + b;
    d = d + ((((b ^ c & (a ^ b)))) + *in_0.offset(6) + 0xc040b340u32);
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = d + a;
    c = c +
    ((((a ^ b & (d ^ a)))) + *in_0.offset(11) + 0x265e5a51u32);
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = c + d;
    b = b + ((((d ^ a & (c ^ d)))) + *in_0.offset(0) + 0xe9b6c7aau32);
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = b + c;
    a = a + ((((c ^ d & (b ^ c)))) + *in_0.offset(5) + 0xd62f105du32);
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = a + b;
    d = d +
    ((((b ^ c & (a ^ b)))) + *in_0.offset(10) + 0x2441453u32);
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = d + a;
    c = c + ((((a ^ b & (d ^ a)))) + *in_0.offset(15) + 0xd8a1e681u32);
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = c + d;
    b = b + ((((d ^ a & (c ^ d)))) + *in_0.offset(4) + 0xe7d3fbc8u32);
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = b + c;
    a = a +
    ((((c ^ d & (b ^ c)))) + *in_0.offset(9) + 0x21e1cde6u32);
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = a + b;
    d = d + ((((b ^ c & (a ^ b)))) + *in_0.offset(14) + 0xc33707d6u32);
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = d + a;
    c = c + ((((a ^ b & (d ^ a)))) + *in_0.offset(3) + 0xf4d50d87u32);
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = c + d;
    b = b +
    ((((d ^ a & (c ^ d)))) + *in_0.offset(8) + 0x455a14edu32);
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = b + c;
    a = a + ((((c ^ d & (b ^ c)))) + *in_0.offset(13) + 0xa9e3e905u32);
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = a + b;
    d = d + ((((b ^ c & (a ^ b)))) + *in_0.offset(2) + 0xfcefa3f8u32);
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = d + a;
    c = c +
    ((((a ^ b & (d ^ a)))) + *in_0.offset(7) + 0x676f02d9u32);
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = c + d;
    b = b + ((((d ^ a & (c ^ d)))) + *in_0.offset(12) + 0x8d2a4c8au32);
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = b + c;
    a = a + ((((b ^ c ^ d))) + *in_0.offset(5) + 0xfffa3942u32);
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = a + b;
    d = d + ((((a ^ b ^ c))) + *in_0.offset(8) + 0x8771f681u32);
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = d + a;
    c = c +
    ((((d ^ a ^ b))) + *in_0.offset(11) + 0x6d9d6122u32);
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = c + d;
    b = b + ((((c ^ d ^ a))) + *in_0.offset(14) + 0xfde5380cu32);
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = b + c;
    a = a + ((((b ^ c ^ d))) + *in_0.offset(1) + 0xa4beea44u32);
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = a + b;
    d = d +
    ((((a ^ b ^ c))) + *in_0.offset(4) + 0x4bdecfa9u32);
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = d + a;
    c = c + ((((d ^ a ^ b))) + *in_0.offset(7) + 0xf6bb4b60u32);
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = c + d;
    b = b + ((((c ^ d ^ a))) + *in_0.offset(10) + 0xbebfbc70u32);
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = b + c;
    a = a +
    ((((b ^ c ^ d))) + *in_0.offset(13) + 0x289b7ec6u32);
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = a + b;
    d = d + ((((a ^ b ^ c))) + *in_0.offset(0) + 0xeaa127fau32);
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = d + a;
    c = c + ((((d ^ a ^ b))) + *in_0.offset(3) + 0xd4ef3085u32);
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = c + d;
    b = b +
    ((((c ^ d ^ a))) + *in_0.offset(6) + 0x4881d05u32);
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = b + c;
    a = a + ((((b ^ c ^ d))) + *in_0.offset(9) + 0xd9d4d039u32);
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = a + b;
    d = d + ((((a ^ b ^ c))) + *in_0.offset(12) + 0xe6db99e5u32);
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = d + a;
    c = c +
    ((((d ^ a ^ b))) + *in_0.offset(15) + 0x1fa27cf8u32);
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = c + d;
    b = b + ((((c ^ d ^ a))) + *in_0.offset(2) + 0xc4ac5665u32);
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = b + c;
    a = a + ((((c ^ (b | !d)))) + *in_0.offset(0) + 0xf4292244u32);
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = a + b;
    d = d +
    ((((b ^ (a | !c)))) + *in_0.offset(7) + 0x432aff97u32);
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = d + a;
    c = c + ((((a ^ (d | !b)))) + *in_0.offset(14) + 0xab9423a7u32);
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = c + d;
    b = b + ((((d ^ (c | !a)))) + *in_0.offset(5) + 0xfc93a039u32);
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = b + c;
    a = a +
    ((((c ^ (b | !d)))) + *in_0.offset(12) + 0x655b59c3u32);
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = a + b;
    d = d + ((((b ^ (a | !c)))) + *in_0.offset(3) + 0x8f0ccc92u32);
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = d + a;
    c = c + ((((a ^ (d | !b)))) + *in_0.offset(10) + 0xffeff47du32);
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = c + d;
    b = b + ((((d ^ (c | !a)))) + *in_0.offset(1) + 0x85845dd1u32);
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = b + c;
    a = a +
    ((((c ^ (b | !d)))) + *in_0.offset(8) + 0x6fa87e4fu32);
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = a + b;
    d = d + ((((b ^ (a | !c)))) + *in_0.offset(15) + 0xfe2ce6e0u32);
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = d + a;
    c = c + ((((a ^ (d | !b)))) + *in_0.offset(6) + 0xa3014314u32);
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = c + d;
    b = b +
    ((((d ^ (c | !a)))) + *in_0.offset(13) + 0x4e0811a1u32);
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = b + c;
    a = a + ((((c ^ (b | !d)))) + *in_0.offset(4) + 0xf7537e82u32);
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = a + b;
    d = d + ((((b ^ (a | !c)))) + *in_0.offset(11) + 0xbd3af235u32);
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = d + a;
    c = c +
    ((((a ^ (d | !b)))) + *in_0.offset(2) + 0x2ad7d2bbu32);
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = c + d;
    b = b + ((((d ^ (c | !a)))) + *in_0.offset(9) + 0xeb86d391u32);
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = b + c;
    let ref mut fresh1 = *buf.offset(0);
    *fresh1 = *fresh1 + a;
    let ref mut fresh2 = *buf.offset(1);
    *fresh2 = *fresh2 + b;
    let ref mut fresh3 = *buf.offset(2);
    *fresh3 = *fresh3 + c;
    let ref mut fresh4 = *buf.offset(3);
    *fresh4 = *fresh4 + d;
}
