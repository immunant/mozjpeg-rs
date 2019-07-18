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

pub use crate::jpeglib_h::JMSG_LENGTH_MAX;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stddef_h::NULL_0;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__ctype_toupper_loc;

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::abs;
pub use crate::stdlib::atof;
pub use crate::stdlib::atoi;

pub use crate::stdlib::exit;

pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fread;
pub use crate::stdlib::free;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::fwrite;

pub use crate::stdlib::malloc;


pub use crate::stdlib::printf;
pub use crate::stdlib::puts;
pub use crate::stdlib::snprintf;
pub use crate::stdlib::sscanf;







pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
pub use crate::stdlib::toupper;
pub use crate::stdlib::FILE;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;
pub use crate::stdlib::_IO_FILE;

pub use crate::turbojpeg::tjAlloc;
pub use crate::turbojpeg::tjBlueOffset;
pub use crate::turbojpeg::tjBufSize;
pub use crate::turbojpeg::tjBufSizeYUV2;
pub use crate::turbojpeg::tjCompress2;
pub use crate::turbojpeg::tjCompressFromYUV;
pub use crate::turbojpeg::tjDecodeYUV;
pub use crate::turbojpeg::tjDecompress2;
pub use crate::turbojpeg::tjDecompressHeader3;
pub use crate::turbojpeg::tjDecompressToYUV2;
pub use crate::turbojpeg::tjDestroy;
pub use crate::turbojpeg::tjEncodeYUV3;
pub use crate::turbojpeg::tjFree;
pub use crate::turbojpeg::tjGetErrorCode;
pub use crate::turbojpeg::tjGetErrorStr;
pub use crate::turbojpeg::tjGetErrorStr2;
pub use crate::turbojpeg::tjGetScalingFactors;
pub use crate::turbojpeg::tjGreenOffset;
pub use crate::turbojpeg::tjInitCompress;
pub use crate::turbojpeg::tjInitDecompress;
pub use crate::turbojpeg::tjInitTransform;
pub use crate::turbojpeg::tjLoadImage;
pub use crate::turbojpeg::tjMCUHeight;
pub use crate::turbojpeg::tjMCUWidth;
pub use crate::turbojpeg::tjPixelSize;
pub use crate::turbojpeg::tjRedOffset;
pub use crate::turbojpeg::tjSaveImage;
pub use crate::turbojpeg::tjTransform;
pub use crate::turbojpeg::tjhandle;
pub use crate::turbojpeg::tjregion;
pub use crate::turbojpeg::tjscalingfactor;
pub use crate::turbojpeg::tjtransform;
pub use crate::turbojpeg::TJCS_YCbCr;
pub use crate::turbojpeg::TJCS;
pub use crate::turbojpeg::TJCS_CMYK;
pub use crate::turbojpeg::TJCS_GRAY;
pub use crate::turbojpeg::TJCS_RGB;
pub use crate::turbojpeg::TJCS_YCCK;
pub use crate::turbojpeg::TJERR;
pub use crate::turbojpeg::TJERR_FATAL;
pub use crate::turbojpeg::TJERR_WARNING;
pub use crate::turbojpeg::TJFLAG_ACCURATEDCT;
pub use crate::turbojpeg::TJFLAG_BOTTOMUP;
pub use crate::turbojpeg::TJFLAG_FASTDCT;
pub use crate::turbojpeg::TJFLAG_FASTUPSAMPLE;
pub use crate::turbojpeg::TJFLAG_NOREALLOC;
pub use crate::turbojpeg::TJFLAG_PROGRESSIVE;
pub use crate::turbojpeg::TJFLAG_STOPONWARNING;
pub use crate::turbojpeg::TJPF;
pub use crate::turbojpeg::TJPF_ABGR;
pub use crate::turbojpeg::TJPF_ARGB;
pub use crate::turbojpeg::TJPF_BGR;
pub use crate::turbojpeg::TJPF_BGRA;
pub use crate::turbojpeg::TJPF_BGRX;
pub use crate::turbojpeg::TJPF_CMYK;
pub use crate::turbojpeg::TJPF_GRAY;
pub use crate::turbojpeg::TJPF_RGB;
pub use crate::turbojpeg::TJPF_RGBA;
pub use crate::turbojpeg::TJPF_RGBX;
pub use crate::turbojpeg::TJPF_UNKNOWN;
pub use crate::turbojpeg::TJPF_XBGR;
pub use crate::turbojpeg::TJPF_XRGB;
pub use crate::turbojpeg::TJSAMP;
pub use crate::turbojpeg::TJSAMP_411;
pub use crate::turbojpeg::TJSAMP_420;
pub use crate::turbojpeg::TJSAMP_422;
pub use crate::turbojpeg::TJSAMP_440;
pub use crate::turbojpeg::TJSAMP_444;
pub use crate::turbojpeg::TJSAMP_GRAY;
pub use crate::turbojpeg::TJXOP;
pub use crate::turbojpeg::TJXOPT_COPYNONE;
pub use crate::turbojpeg::TJXOPT_CROP;
pub use crate::turbojpeg::TJXOPT_GRAY;
pub use crate::turbojpeg::TJXOPT_NOOUTPUT;
pub use crate::turbojpeg::TJXOPT_TRIM;
pub use crate::turbojpeg::TJXOP_HFLIP;
pub use crate::turbojpeg::TJXOP_NONE;
pub use crate::turbojpeg::TJXOP_ROT180;
pub use crate::turbojpeg::TJXOP_ROT270;
pub use crate::turbojpeg::TJXOP_ROT90;
pub use crate::turbojpeg::TJXOP_TRANSPOSE;
pub use crate::turbojpeg::TJXOP_TRANSVERSE;
pub use crate::turbojpeg::TJXOP_VFLIP;
pub use crate::turbojpeg::TJ_GRAYSCALE;
pub use crate::turbojpeg::TJ_NUMSAMP;
extern crate libc;
use mozjpeg::*;

/*
 * Copyright (C)2009-2018 D. R. Commander.  All Rights Reserved.
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
#[no_mangle]
pub static mut tjErrorStr: [libc::c_char; 200] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]
pub static mut tjErrorMsg: [libc::c_char; 200] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]
pub static mut tjErrorLine: libc::c_int = -1i32;
#[no_mangle]
pub static mut tjErrorCode: libc::c_int = -1i32;
#[no_mangle]
pub static mut flags: libc::c_int = crate::turbojpeg::TJFLAG_NOREALLOC;
#[no_mangle]
pub static mut compOnly: libc::c_int = 0i32;
#[no_mangle]
pub static mut decompOnly: libc::c_int = 0i32;
#[no_mangle]
pub static mut doYUV: libc::c_int = 0i32;
#[no_mangle]
pub static mut quiet: libc::c_int = 0i32;
#[no_mangle]
pub static mut doTile: libc::c_int = 0i32;
#[no_mangle]
pub static mut pf: libc::c_int = crate::turbojpeg::TJPF_BGR as libc::c_int;
#[no_mangle]
pub static mut yuvPad: libc::c_int = 1i32;
#[no_mangle]
pub static mut doWrite: libc::c_int = 1i32;
#[no_mangle]
pub static mut ext: *mut libc::c_char =
    b"ppm\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut pixFormatStr: [*const libc::c_char; 12] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"BGR\x00" as *const u8 as *const libc::c_char,
    b"RGBX\x00" as *const u8 as *const libc::c_char,
    b"BGRX\x00" as *const u8 as *const libc::c_char,
    b"XBGR\x00" as *const u8 as *const libc::c_char,
    b"XRGB\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut subNameLong: [*const libc::c_char; 6] = [
    b"4:4:4\x00" as *const u8 as *const libc::c_char,
    b"4:2:2\x00" as *const u8 as *const libc::c_char,
    b"4:2:0\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"4:4:0\x00" as *const u8 as *const libc::c_char,
    b"4:1:1\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut csName: [*const libc::c_char; 5] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"YCbCr\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
    b"YCCK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut subName: [*const libc::c_char; 6] = [
    b"444\x00" as *const u8 as *const libc::c_char,
    b"422\x00" as *const u8 as *const libc::c_char,
    b"420\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"440\x00" as *const u8 as *const libc::c_char,
    b"411\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut scalingFactors: *mut crate::turbojpeg::tjscalingfactor =
    crate::stddef_h::NULL_0 as *mut crate::turbojpeg::tjscalingfactor;
#[no_mangle]
pub static mut sf: crate::turbojpeg::tjscalingfactor = crate::turbojpeg::tjscalingfactor {
    num: 1i32,
    denom: 1i32,
};
#[no_mangle]
pub static mut nsf: libc::c_int = 0i32;
#[no_mangle]
pub static mut xformOp: libc::c_int = crate::turbojpeg::TJXOP_NONE as libc::c_int;
#[no_mangle]
pub static mut xformOpt: libc::c_int = 0i32;
#[no_mangle]
pub static mut customFilter: Option<
    unsafe extern "C" fn(
        _: *mut libc::c_short,
        _: crate::turbojpeg::tjregion,
        _: crate::turbojpeg::tjregion,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut crate::turbojpeg::tjtransform,
    ) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut benchTime: libc::c_double = 5.0f64;
#[no_mangle]
pub static mut warmup: libc::c_double = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn formatName(
    mut subsamp: libc::c_int,
    mut cs: libc::c_int,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    if cs == crate::turbojpeg::TJCS_YCbCr as libc::c_int {
        return subNameLong[subsamp as usize] as *mut libc::c_char;
    } else if cs == crate::turbojpeg::TJCS_YCCK as libc::c_int
        || cs == crate::turbojpeg::TJCS_CMYK as libc::c_int
    {
        crate::stdlib::snprintf(
            buf,
            80i32 as libc::c_ulong,
            b"%s %s\x00" as *const u8 as *const libc::c_char,
            csName[cs as usize],
            subNameLong[subsamp as usize],
        );
        return buf;
    } else {
        return csName[cs as usize] as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn sigfig(
    mut val: libc::c_double,
    mut figs: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut format: [libc::c_char; 80] = [0; 80];
    let mut digitsAfterDecimal: libc::c_int =
        figs - crate::stdlib::ceil(crate::stdlib::log10(crate::stdlib::fabs(val))) as libc::c_int;
    if digitsAfterDecimal < 1i32 {
        crate::stdlib::snprintf(
            format.as_mut_ptr(),
            80i32 as libc::c_ulong,
            b"%%.0f\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        crate::stdlib::snprintf(
            format.as_mut_ptr(),
            80i32 as libc::c_ulong,
            b"%%.%df\x00" as *const u8 as *const libc::c_char,
            digitsAfterDecimal,
        );
    }
    crate::stdlib::snprintf(buf, len as libc::c_ulong, format.as_mut_ptr(), val);
    return buf;
}
/* Custom DCT filter which produces a negative of the image */
#[no_mangle]
pub unsafe extern "C" fn dummyDCTFilter(
    mut coeffs: *mut libc::c_short,
    mut arrayRegion: crate::turbojpeg::tjregion,
    mut _planeRegion: crate::turbojpeg::tjregion,
    mut _componentIndex: libc::c_int,
    mut _transformIndex: libc::c_int,
    mut _transform: *mut crate::turbojpeg::tjtransform,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < arrayRegion.w * arrayRegion.h {
        *coeffs.offset(i as isize) = -(*coeffs.offset(i as isize) as libc::c_int) as libc::c_short;
        i += 1
    }
    return 0i32;
}
/* Decompression test */
#[no_mangle]
pub unsafe extern "C" fn decomp(
    mut srcBuf: *mut libc::c_uchar,
    mut jpegBuf: *mut *mut libc::c_uchar,
    mut jpegSize: *mut libc::c_ulong,
    mut dstBuf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut subsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut fileName: *mut libc::c_char,
    mut tilew: libc::c_int,
    mut tileh: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tempStr: [libc::c_char; 1024] = [0; 1024];
    let mut sizeStr: [libc::c_char; 20] =
        *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        );
    let mut qualStr: [libc::c_char; 6] =
        *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"\x00\x00\x00\x00\x00\x00");
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut handle: crate::turbojpeg::tjhandle = crate::stddef_h::NULL_0 as *mut libc::c_void;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut iter: libc::c_int = 0i32;
    let mut dstBufAlloc: libc::c_int = 0i32;
    let mut retval: libc::c_int = 0i32;
    let mut elapsed: libc::c_double = 0.;
    let mut elapsedDecode: libc::c_double = 0.;
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut scaledw: libc::c_int = (w * sf.num + sf.denom - 1i32) / sf.denom;
    let mut scaledh: libc::c_int = (h * sf.num + sf.denom - 1i32) / sf.denom;
    let mut pitch: libc::c_int = scaledw * ps;
    let mut ntilesw: libc::c_int = (w + tilew - 1i32) / tilew;
    let mut ntilesh: libc::c_int = (h + tileh - 1i32) / tileh;
    let mut dstPtr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dstPtr2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    if jpegQual > 0i32 {
        crate::stdlib::snprintf(
            qualStr.as_mut_ptr(),
            6i32 as libc::c_ulong,
            b"_Q%d\x00" as *const u8 as *const libc::c_char,
            jpegQual,
        );
        qualStr[5usize] = 0i32 as libc::c_char
    }
    handle = crate::turbojpeg::tjInitDecompress();
    if handle.is_null() {
        let mut _tjErrorCode: libc::c_int = crate::turbojpeg::tjGetErrorCode(handle);
        let mut _tjErrorStr: *mut libc::c_char = crate::turbojpeg::tjGetErrorStr2(handle);
        if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
            && _tjErrorCode == crate::turbojpeg::TJERR_WARNING as libc::c_int
        {
            if 0 != crate::stdlib::strncmp(
                tjErrorStr.as_mut_ptr(),
                _tjErrorStr,
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            ) || 0
                != crate::stdlib::strncmp(
                    tjErrorMsg.as_mut_ptr(),
                    b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                )
                || tjErrorCode != _tjErrorCode
                || tjErrorLine != 160i32
            {
                crate::stdlib::strncpy(
                    tjErrorStr.as_mut_ptr(),
                    _tjErrorStr,
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                );
                crate::stdlib::strncpy(
                    tjErrorMsg.as_mut_ptr(),
                    b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                );
                tjErrorCode = _tjErrorCode;
                tjErrorLine = 160i32;
                crate::stdlib::printf(
                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    160i32,
                    b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                    _tjErrorStr,
                );
            }
            current_block = 11194104282611034094;
        } else {
            crate::stdlib::printf(
                b"%s in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                if _tjErrorCode == crate::turbojpeg::TJERR_WARNING as libc::c_int {
                    b"WARNING\x00" as *const u8 as *const libc::c_char
                } else {
                    b"ERROR\x00" as *const u8 as *const libc::c_char
                },
                160i32,
                b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                _tjErrorStr,
            );
            retval = -1i32;
            current_block = 126259514807107346;
        }
    } else {
        current_block = 11194104282611034094;
    }
    match current_block {
        11194104282611034094 => {
            if dstBuf.is_null() {
                dstBuf =
                    crate::stdlib::malloc((pitch * scaledh) as libc::c_ulong) as *mut libc::c_uchar;
                if dstBuf.is_null() {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        164i32,
                        b"allocating destination buffer\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -1i32;
                    current_block = 126259514807107346;
                } else {
                    dstBufAlloc = 1i32;
                    current_block = 8693738493027456495;
                }
            } else {
                current_block = 8693738493027456495;
            }
            match current_block {
                126259514807107346 => {}
                _ => {
                    crate::stdlib::memset(
                        dstBuf as *mut libc::c_void,
                        127i32,
                        (pitch * scaledh) as libc::c_ulong,
                    );
                    if 0 != doYUV {
                        let mut width: libc::c_int = if 0 != doTile { tilew } else { scaledw };
                        let mut height: libc::c_int = if 0 != doTile { tileh } else { scaledh };
                        let mut yuvSize: libc::c_int =
                            crate::turbojpeg::tjBufSizeYUV2(width, yuvPad, height, subsamp)
                                as libc::c_int;
                        yuvBuf =
                            crate::stdlib::malloc(yuvSize as libc::c_ulong) as *mut libc::c_uchar;
                        if yuvBuf.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                177i32,
                                b"allocating YUV buffer\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -1i32;
                            current_block = 126259514807107346;
                        } else {
                            crate::stdlib::memset(
                                yuvBuf as *mut libc::c_void,
                                127i32,
                                yuvSize as libc::c_ulong,
                            );
                            current_block = 11743904203796629665;
                        }
                    } else {
                        current_block = 11743904203796629665;
                    }
                    match current_block {
                        126259514807107346 => {}
                        _ => {
                            iter = -1i32;
                            elapsedDecode = 0.0f64;
                            elapsed = elapsedDecode;
                            's_213: loop {
                                let mut tile: libc::c_int = 0i32;
                                let mut start: libc::c_double = crate::tjutil::getTime();
                                row = 0i32;
                                dstPtr = dstBuf;
                                while row < ntilesh {
                                    col = 0i32;
                                    dstPtr2 = dstPtr;
                                    while col < ntilesw {
                                        let mut width_0: libc::c_int = if 0 != doTile {
                                            if tilew < w - col * tilew {
                                                tilew
                                            } else {
                                                w - col * tilew
                                            }
                                        } else {
                                            scaledw
                                        };
                                        let mut height_0: libc::c_int = if 0 != doTile {
                                            if tileh < h - row * tileh {
                                                tileh
                                            } else {
                                                h - row * tileh
                                            }
                                        } else {
                                            scaledh
                                        };
                                        if 0 != doYUV {
                                            let mut startDecode: libc::c_double = 0.;
                                            if crate::turbojpeg::tjDecompressToYUV2(
                                                handle,
                                                *jpegBuf.offset(tile as isize),
                                                *jpegSize.offset(tile as isize),
                                                yuvBuf,
                                                width_0,
                                                yuvPad,
                                                height_0,
                                                flags,
                                            ) == -1i32
                                            {
                                                let mut _tjErrorCode_0: libc::c_int =
                                                    crate::turbojpeg::tjGetErrorCode(handle);
                                                let mut _tjErrorStr_0: *mut libc::c_char =
                                                    crate::turbojpeg::tjGetErrorStr2(handle);
                                                if 0 == flags
                                                    & crate::turbojpeg::TJFLAG_STOPONWARNING
                                                    && _tjErrorCode_0
                                                        == crate::turbojpeg::TJERR_WARNING
                                                            as libc::c_int
                                                {
                                                    if 0 != crate::stdlib::strncmp(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_0,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    ) || 0
                                                        != crate::stdlib::strncmp(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecompressToYUV2()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        )
                                                        || tjErrorCode != _tjErrorCode_0
                                                        || tjErrorLine != 200i32
                                                    {
                                                        crate::stdlib::strncpy(
                                                            tjErrorStr.as_mut_ptr(),
                                                            _tjErrorStr_0,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        crate::stdlib::strncpy(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecompressToYUV2()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        tjErrorCode = _tjErrorCode_0;
                                                        tjErrorLine = 200i32;
                                                        crate::stdlib::printf(b"WARNING in line %d while %s:\n%s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   200i32,
                                                                   b"executing tjDecompressToYUV2()\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   _tjErrorStr_0);
                                                    }
                                                } else {
                                                    crate::stdlib::printf(
                                                        b"%s in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        if _tjErrorCode_0
                                                            == crate::turbojpeg::TJERR_WARNING
                                                                as libc::c_int
                                                        {
                                                            b"WARNING\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"ERROR\x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                        200i32,
                                                        b"executing tjDecompressToYUV2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        _tjErrorStr_0,
                                                    );
                                                    retval = -1i32;
                                                    current_block = 126259514807107346;
                                                    break 's_213;
                                                }
                                            }
                                            startDecode = crate::tjutil::getTime();
                                            if crate::turbojpeg::tjDecodeYUV(
                                                handle, yuvBuf, yuvPad, subsamp, dstPtr2, width_0,
                                                pitch, height_0, pf, flags,
                                            ) == -1i32
                                            {
                                                let mut _tjErrorCode_1: libc::c_int =
                                                    crate::turbojpeg::tjGetErrorCode(handle);
                                                let mut _tjErrorStr_1: *mut libc::c_char =
                                                    crate::turbojpeg::tjGetErrorStr2(handle);
                                                if 0 == flags
                                                    & crate::turbojpeg::TJFLAG_STOPONWARNING
                                                    && _tjErrorCode_1
                                                        == crate::turbojpeg::TJERR_WARNING
                                                            as libc::c_int
                                                {
                                                    if 0 != crate::stdlib::strncmp(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_1,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    ) || 0
                                                        != crate::stdlib::strncmp(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecodeYUV()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        )
                                                        || tjErrorCode != _tjErrorCode_1
                                                        || tjErrorLine != 204i32
                                                    {
                                                        crate::stdlib::strncpy(
                                                            tjErrorStr.as_mut_ptr(),
                                                            _tjErrorStr_1,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        crate::stdlib::strncpy(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecodeYUV()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        tjErrorCode = _tjErrorCode_1;
                                                        tjErrorLine = 204i32;
                                                        crate::stdlib::printf(b"WARNING in line %d while %s:\n%s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   204i32,
                                                                   b"executing tjDecodeYUV()\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   _tjErrorStr_1);
                                                    }
                                                } else {
                                                    crate::stdlib::printf(
                                                        b"%s in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        if _tjErrorCode_1
                                                            == crate::turbojpeg::TJERR_WARNING
                                                                as libc::c_int
                                                        {
                                                            b"WARNING\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"ERROR\x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                        204i32,
                                                        b"executing tjDecodeYUV()\x00" as *const u8
                                                            as *const libc::c_char,
                                                        _tjErrorStr_1,
                                                    );
                                                    retval = -1i32;
                                                    current_block = 126259514807107346;
                                                    break 's_213;
                                                }
                                            }
                                            if iter >= 0i32 {
                                                elapsedDecode +=
                                                    crate::tjutil::getTime() - startDecode
                                            }
                                        } else if crate::turbojpeg::tjDecompress2(
                                            handle,
                                            *jpegBuf.offset(tile as isize),
                                            *jpegSize.offset(tile as isize),
                                            dstPtr2,
                                            width_0,
                                            pitch,
                                            height_0,
                                            pf,
                                            flags,
                                        ) == -1i32
                                        {
                                            let mut _tjErrorCode_2: libc::c_int =
                                                crate::turbojpeg::tjGetErrorCode(handle);
                                            let mut _tjErrorStr_2: *mut libc::c_char =
                                                crate::turbojpeg::tjGetErrorStr2(handle);
                                            if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                                                && _tjErrorCode_2
                                                    == crate::turbojpeg::TJERR_WARNING
                                                        as libc::c_int
                                            {
                                                if 0 != crate::stdlib::strncmp(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_2,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                ) || 0
                                                    != crate::stdlib::strncmp(
                                                        tjErrorMsg.as_mut_ptr(),
                                                        b"executing tjDecompress2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    )
                                                    || tjErrorCode != _tjErrorCode_2
                                                    || tjErrorLine != 209i32
                                                {
                                                    crate::stdlib::strncpy(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_2,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    );
                                                    crate::stdlib::strncpy(
                                                        tjErrorMsg.as_mut_ptr(),
                                                        b"executing tjDecompress2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    );
                                                    tjErrorCode = _tjErrorCode_2;
                                                    tjErrorLine = 209i32;
                                                    crate::stdlib::printf(
                                                        b"WARNING in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        209i32,
                                                        b"executing tjDecompress2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        _tjErrorStr_2,
                                                    );
                                                }
                                            } else {
                                                crate::stdlib::printf(
                                                    b"%s in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    if _tjErrorCode_2
                                                        == crate::turbojpeg::TJERR_WARNING
                                                            as libc::c_int
                                                    {
                                                        b"WARNING\x00" as *const u8
                                                            as *const libc::c_char
                                                    } else {
                                                        b"ERROR\x00" as *const u8
                                                            as *const libc::c_char
                                                    },
                                                    209i32,
                                                    b"executing tjDecompress2()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_2,
                                                );
                                                retval = -1i32;
                                                current_block = 126259514807107346;
                                                break 's_213;
                                            }
                                        }
                                        col += 1;
                                        tile += 1;
                                        dstPtr2 = dstPtr2.offset((ps * tilew) as isize)
                                    }
                                    row += 1;
                                    dstPtr = dstPtr.offset((pitch * tileh) as isize)
                                }
                                elapsed += crate::tjutil::getTime() - start;
                                if iter >= 0i32 {
                                    iter += 1;
                                    if elapsed >= benchTime {
                                        current_block = 14851765859726653900;
                                        break;
                                    }
                                } else if elapsed >= warmup {
                                    iter = 0i32;
                                    elapsedDecode = 0.0f64;
                                    elapsed = elapsedDecode
                                }
                            }
                            match current_block {
                                126259514807107346 => {}
                                _ => {
                                    if 0 != doYUV {
                                        elapsed -= elapsedDecode
                                    }
                                    if crate::turbojpeg::tjDestroy(handle) == -1i32 {
                                        let mut _tjErrorCode_3: libc::c_int =
                                            crate::turbojpeg::tjGetErrorCode(handle);
                                        let mut _tjErrorStr_3: *mut libc::c_char =
                                            crate::turbojpeg::tjGetErrorStr2(handle);
                                        if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                                            && _tjErrorCode_3
                                                == crate::turbojpeg::TJERR_WARNING as libc::c_int
                                        {
                                            if 0 != crate::stdlib::strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_3,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            ) || 0
                                                != crate::stdlib::strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjDestroy()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                )
                                                || tjErrorCode != _tjErrorCode_3
                                                || tjErrorLine != 223i32
                                            {
                                                crate::stdlib::strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_3,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                crate::stdlib::strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjDestroy()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_3;
                                                tjErrorLine = 223i32;
                                                crate::stdlib::printf(
                                                    b"WARNING in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    223i32,
                                                    b"executing tjDestroy()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_3,
                                                );
                                            }
                                            current_block = 17917672080766325409;
                                        } else {
                                            crate::stdlib::printf(
                                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                    as *const libc::c_char,
                                                if _tjErrorCode_3
                                                    == crate::turbojpeg::TJERR_WARNING
                                                        as libc::c_int
                                                {
                                                    b"WARNING\x00" as *const u8
                                                        as *const libc::c_char
                                                } else {
                                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                                },
                                                223i32,
                                                b"executing tjDestroy()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_3,
                                            );
                                            retval = -1i32;
                                            current_block = 126259514807107346;
                                        }
                                    } else {
                                        current_block = 17917672080766325409;
                                    }
                                    match current_block {
                                        126259514807107346 => {}
                                        _ => {
                                            handle = crate::stddef_h::NULL_0 as *mut libc::c_void;
                                            if 0 != quiet {
                                                crate::stdlib::printf(
                                                    b"%-6s%s\x00" as *const u8
                                                        as *const libc::c_char,
                                                    sigfig(
                                                        (w * h) as libc::c_double / 1000000.0f64
                                                            * iter as libc::c_double
                                                            / elapsed,
                                                        4i32,
                                                        tempStr.as_mut_ptr(),
                                                        1024i32,
                                                    ),
                                                    if quiet == 2i32 {
                                                        b"\n\x00" as *const u8
                                                            as *const libc::c_char
                                                    } else {
                                                        b"  \x00" as *const u8
                                                            as *const libc::c_char
                                                    },
                                                );
                                                if 0 != doYUV {
                                                    crate::stdlib::printf(
                                                        b"%s\n\x00" as *const u8
                                                            as *const libc::c_char,
                                                        sigfig(
                                                            (w * h) as libc::c_double
                                                                / 1000000.0f64
                                                                * iter as libc::c_double
                                                                / elapsedDecode,
                                                            4i32,
                                                            tempStr.as_mut_ptr(),
                                                            1024i32,
                                                        ),
                                                    );
                                                } else if quiet != 2i32 {
                                                    crate::stdlib::printf(
                                                        b"\n\x00" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                            } else {
                                                crate::stdlib::printf(
                                                    b"%s --> Frame rate:         %f fps\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    if 0 != doYUV {
                                                        b"Decomp to YUV\x00" as *const u8
                                                            as *const libc::c_char
                                                    } else {
                                                        b"Decompress   \x00" as *const u8
                                                            as *const libc::c_char
                                                    },
                                                    iter as libc::c_double / elapsed,
                                                );
                                                crate::stdlib::printf(b"                  Throughput:         %f Megapixels/sec\n\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       (w * h) as
                                                           libc::c_double /
                                                           1000000.0f64 *
                                                           iter as
                                                               libc::c_double
                                                           / elapsed);
                                                if 0 != doYUV {
                                                    crate::stdlib::printf(b"YUV Decode    --> Frame rate:         %f fps\n\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           iter as
                                                               libc::c_double
                                                               /
                                                               elapsedDecode);
                                                    crate::stdlib::printf(b"                  Throughput:         %f Megapixels/sec\n\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           (w * h) as
                                                               libc::c_double
                                                               / 1000000.0f64
                                                               *
                                                               iter as
                                                                   libc::c_double
                                                               /
                                                               elapsedDecode);
                                                }
                                            }
                                            if !(0 == doWrite) {
                                                if sf.num != 1i32 || sf.denom != 1i32 {
                                                    crate::stdlib::snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20i32 as libc::c_ulong,
                                                        b"%d_%d\x00" as *const u8
                                                            as *const libc::c_char,
                                                        sf.num,
                                                        sf.denom,
                                                    );
                                                } else if tilew != w || tileh != h {
                                                    crate::stdlib::snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20i32 as libc::c_ulong,
                                                        b"%dx%d\x00" as *const u8
                                                            as *const libc::c_char,
                                                        tilew,
                                                        tileh,
                                                    );
                                                } else {
                                                    crate::stdlib::snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20i32 as libc::c_ulong,
                                                        b"full\x00" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                                if 0 != decompOnly {
                                                    crate::stdlib::snprintf(
                                                        tempStr.as_mut_ptr(),
                                                        1024i32 as libc::c_ulong,
                                                        b"%s_%s.%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        fileName,
                                                        sizeStr.as_mut_ptr(),
                                                        ext,
                                                    );
                                                } else {
                                                    crate::stdlib::snprintf(
                                                        tempStr.as_mut_ptr(),
                                                        1024i32 as libc::c_ulong,
                                                        b"%s_%s%s_%s.%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        fileName,
                                                        subName[subsamp as usize],
                                                        qualStr.as_mut_ptr(),
                                                        sizeStr.as_mut_ptr(),
                                                        ext,
                                                    );
                                                }
                                                if crate::turbojpeg::tjSaveImage(
                                                    tempStr.as_mut_ptr(),
                                                    dstBuf,
                                                    scaledw,
                                                    0i32,
                                                    scaledh,
                                                    pf,
                                                    flags,
                                                ) == -1i32
                                                {
                                                    crate::stdlib::printf(
                                                        b"ERROR in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        263i32,
                                                        b"saving bitmap\x00" as *const u8
                                                            as *const libc::c_char,
                                                        crate::turbojpeg::tjGetErrorStr2(
                                                            crate::stddef_h::NULL_0
                                                                as *mut libc::c_void,
                                                        ),
                                                    );
                                                    retval = -1i32
                                                } else {
                                                    ptr = crate::stdlib::strrchr(
                                                        tempStr.as_mut_ptr(),
                                                        '.' as i32,
                                                    );
                                                    crate::stdlib::snprintf(
                                                        ptr,
                                                        (1024i32 as libc::c_long
                                                            - ptr.wrapping_offset_from(
                                                                tempStr.as_mut_ptr(),
                                                            )
                                                                as libc::c_long)
                                                            as libc::c_ulong,
                                                        b"-err.%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        ext,
                                                    );
                                                    if !srcBuf.is_null()
                                                        && sf.num == 1i32
                                                        && sf.denom == 1i32
                                                    {
                                                        if 0 == quiet {
                                                            crate::stdlib::printf(b"Compression error written to %s.\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   tempStr.as_mut_ptr());
                                                        }
                                                        if subsamp == crate::turbojpeg::TJ_GRAYSCALE
                                                        {
                                                            let mut index: libc::c_int = 0;
                                                            let mut index2: libc::c_int = 0;
                                                            row = 0i32;
                                                            index = 0i32;
                                                            while row < h {
                                                                col = 0i32;
                                                                index2 = index;
                                                                while col < w {
                                                                    let mut rindex:
                                                                            libc::c_int =
                                                                        index2
                                                                            +
                                                                            crate::turbojpeg::tjRedOffset[pf
                                                                                            as
                                                                                            usize];
                                                                    let mut gindex:
                                                                            libc::c_int =
                                                                        index2
                                                                            +
                                                                            crate::turbojpeg::tjGreenOffset[pf
                                                                                              as
                                                                                              usize];
                                                                    let mut bindex:
                                                                            libc::c_int =
                                                                        index2
                                                                            +
                                                                            crate::turbojpeg::tjBlueOffset[pf
                                                                                             as
                                                                                             usize];
                                                                    let mut y: libc::c_int =
                                                                        (*srcBuf
                                                                            .offset(rindex as isize)
                                                                            as libc::c_double
                                                                            * 0.299f64
                                                                            + *srcBuf.offset(
                                                                                gindex as isize,
                                                                            )
                                                                                as libc::c_double
                                                                                * 0.587f64
                                                                            + *srcBuf.offset(
                                                                                bindex as isize,
                                                                            )
                                                                                as libc::c_double
                                                                                * 0.114f64
                                                                            + 0.5f64)
                                                                            as libc::c_int;
                                                                    if y > 255i32 {
                                                                        y = 255i32
                                                                    }
                                                                    if y < 0i32 {
                                                                        y = 0i32
                                                                    }
                                                                    *dstBuf
                                                                        .offset(rindex as isize) =
                                                                        crate::stdlib::abs(
                                                                            *dstBuf.offset(
                                                                                rindex as isize,
                                                                            )
                                                                                as libc::c_int
                                                                                - y,
                                                                        )
                                                                            as libc::c_uchar;
                                                                    *dstBuf
                                                                        .offset(gindex as isize) =
                                                                        crate::stdlib::abs(
                                                                            *dstBuf.offset(
                                                                                gindex as isize,
                                                                            )
                                                                                as libc::c_int
                                                                                - y,
                                                                        )
                                                                            as libc::c_uchar;
                                                                    *dstBuf
                                                                        .offset(bindex as isize) =
                                                                        crate::stdlib::abs(
                                                                            *dstBuf.offset(
                                                                                bindex as isize,
                                                                            )
                                                                                as libc::c_int
                                                                                - y,
                                                                        )
                                                                            as libc::c_uchar;
                                                                    col += 1;
                                                                    index2 += ps
                                                                }
                                                                row += 1;
                                                                index += pitch
                                                            }
                                                        } else {
                                                            row = 0i32;
                                                            while row < h {
                                                                col = 0i32;
                                                                while col < w * ps {
                                                                    *dstBuf.offset(
                                                                        (pitch * row + col)
                                                                            as isize,
                                                                    ) = crate::stdlib::abs(
                                                                        *dstBuf.offset(
                                                                            (pitch * row + col)
                                                                                as isize,
                                                                        )
                                                                            as libc::c_int
                                                                            - *srcBuf.offset(
                                                                                (pitch * row + col)
                                                                                    as isize,
                                                                            )
                                                                                as libc::c_int,
                                                                    )
                                                                        as libc::c_uchar;
                                                                    col += 1
                                                                }
                                                                row += 1
                                                            }
                                                        }
                                                        if crate::turbojpeg::tjSaveImage(
                                                            tempStr.as_mut_ptr(),
                                                            dstBuf,
                                                            w,
                                                            0i32,
                                                            h,
                                                            pf,
                                                            flags,
                                                        ) == -1i32
                                                        {
                                                            crate::stdlib::printf(b"ERROR in line %d while %s:\n%s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   294i32,
                                                                   b"saving bitmap\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   crate::turbojpeg::tjGetErrorStr2(crate::stddef_h::NULL_0
                                                                                      as
                                                                                      *mut libc::c_void));
                                                            retval = -1i32
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
    }
    if !handle.is_null() {
        crate::turbojpeg::tjDestroy(handle);
    }
    if !dstBuf.is_null() && 0 != dstBufAlloc {
        crate::stdlib::free(dstBuf as *mut libc::c_void);
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn fullTest(
    mut srcBuf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut subsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut fileName: *mut libc::c_char,
) -> libc::c_int {
    let mut tempStr: [libc::c_char; 1024] = [0; 1024];
    let mut tempStr2: [libc::c_char; 80] = [0; 80];
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut handle: crate::turbojpeg::tjhandle = crate::stddef_h::NULL_0 as *mut libc::c_void;
    let mut jpegBuf: *mut *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut tmpBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut srcPtr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut srcPtr2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut start: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut elapsedEncode: libc::c_double = 0.;
    let mut totalJpegSize: libc::c_int = 0i32;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tilew: libc::c_int = w;
    let mut tileh: libc::c_int = h;
    let mut retval: libc::c_int = 0i32;
    let mut iter: libc::c_int = 0;
    let mut yuvSize: libc::c_int = 0i32;
    let mut jpegSize: *mut libc::c_ulong = crate::stddef_h::NULL_0 as *mut libc::c_ulong;
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut ntilesw: libc::c_int = 1i32;
    let mut ntilesh: libc::c_int = 1i32;
    let mut pitch: libc::c_int = w * ps;
    let mut pfStr: *const libc::c_char = pixFormatStr[pf as usize];
    tmpBuf = crate::stdlib::malloc((pitch * h) as libc::c_ulong) as *mut libc::c_uchar;
    if tmpBuf.is_null() {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            323i32,
            b"allocating temporary image buffer\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        retval = -1i32
    } else {
        if 0 == quiet {
            crate::stdlib::printf(
                b">>>>>  %s (%s) <--> JPEG %s Q%d  <<<<<\n\x00" as *const u8 as *const libc::c_char,
                pfStr,
                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                    b"Bottom-up\x00" as *const u8 as *const libc::c_char
                } else {
                    b"Top-down\x00" as *const u8 as *const libc::c_char
                },
                subNameLong[subsamp as usize],
                jpegQual,
            );
        }
        tilew = if 0 != doTile { 8i32 } else { w };
        tileh = if 0 != doTile { 8i32 } else { h };
        's_73: loop {
            if tilew > w {
                tilew = w
            }
            if tileh > h {
                tileh = h
            }
            ntilesw = (w + tilew - 1i32) / tilew;
            ntilesh = (h + tileh - 1i32) / tileh;
            jpegBuf = crate::stdlib::malloc(
                (::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong)
                    .wrapping_mul(ntilesw as libc::c_ulong)
                    .wrapping_mul(ntilesh as libc::c_ulong),
            ) as *mut *mut libc::c_uchar;
            if jpegBuf.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    339i32,
                    b"allocating JPEG tile array\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                retval = -1i32;
                break;
            } else {
                crate::stdlib::memset(
                    jpegBuf as *mut libc::c_void,
                    0i32,
                    (::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong)
                        .wrapping_mul(ntilesw as libc::c_ulong)
                        .wrapping_mul(ntilesh as libc::c_ulong),
                );
                jpegSize = crate::stdlib::malloc(
                    (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(ntilesw as libc::c_ulong)
                        .wrapping_mul(ntilesh as libc::c_ulong),
                ) as *mut libc::c_ulong;
                if jpegSize.is_null() {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        343i32,
                        b"allocating JPEG size array\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -1i32;
                    break;
                } else {
                    crate::stdlib::memset(
                        jpegSize as *mut libc::c_void,
                        0i32,
                        (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(ntilesw as libc::c_ulong)
                            .wrapping_mul(ntilesh as libc::c_ulong),
                    );
                    if flags & crate::turbojpeg::TJFLAG_NOREALLOC != 0i32 {
                        i = 0i32;
                        while i < ntilesw * ntilesh {
                            let ref mut fresh0 = *jpegBuf.offset(i as isize);
                            *fresh0 = crate::turbojpeg::tjAlloc(crate::turbojpeg::tjBufSize(
                                tilew, tileh, subsamp,
                            )
                                as libc::c_int);
                            if (*fresh0).is_null() {
                                crate::stdlib::printf(
                                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    350i32,
                                    b"allocating JPEG tiles\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                                );
                                retval = -1i32;
                                break 's_73;
                            } else {
                                i += 1
                            }
                        }
                    }
                    if quiet == 1i32 {
                        crate::stdlib::printf(
                            b"%-4s (%s)  %-5s    %-3d   \x00" as *const u8 as *const libc::c_char,
                            pfStr,
                            if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                                b"BU\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"TD\x00" as *const u8 as *const libc::c_char
                            },
                            subNameLong[subsamp as usize],
                            jpegQual,
                        );
                    }
                    i = 0i32;
                    while i < h {
                        crate::stdlib::memcpy(
                            &mut *tmpBuf.offset((pitch * i) as isize) as *mut libc::c_uchar
                                as *mut libc::c_void,
                            &mut *srcBuf.offset((w * ps * i) as isize) as *mut libc::c_uchar
                                as *const libc::c_void,
                            (w * ps) as libc::c_ulong,
                        );
                        i += 1
                    }
                    handle = crate::turbojpeg::tjInitCompress();
                    if handle.is_null() {
                        let mut _tjErrorCode: libc::c_int =
                            crate::turbojpeg::tjGetErrorCode(handle);
                        let mut _tjErrorStr: *mut libc::c_char =
                            crate::turbojpeg::tjGetErrorStr2(handle);
                        if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                            && _tjErrorCode == crate::turbojpeg::TJERR_WARNING as libc::c_int
                        {
                            if 0 != crate::stdlib::strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr,
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            ) || 0
                                != crate::stdlib::strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjInitCompress()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                )
                                || tjErrorCode != _tjErrorCode
                                || tjErrorLine != 361i32
                            {
                                crate::stdlib::strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                crate::stdlib::strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjInitCompress()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                tjErrorCode = _tjErrorCode;
                                tjErrorLine = 361i32;
                                crate::stdlib::printf(
                                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    361i32,
                                    b"executing tjInitCompress()\x00" as *const u8
                                        as *const libc::c_char,
                                    _tjErrorStr,
                                );
                            }
                        } else {
                            crate::stdlib::printf(
                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                if _tjErrorCode == crate::turbojpeg::TJERR_WARNING as libc::c_int {
                                    b"WARNING\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                },
                                361i32,
                                b"executing tjInitCompress()\x00" as *const u8
                                    as *const libc::c_char,
                                _tjErrorStr,
                            );
                            retval = -1i32;
                            break;
                        }
                    }
                    if 0 != doYUV {
                        yuvSize = crate::turbojpeg::tjBufSizeYUV2(tilew, yuvPad, tileh, subsamp)
                            as libc::c_int;
                        yuvBuf =
                            crate::stdlib::malloc(yuvSize as libc::c_ulong) as *mut libc::c_uchar;
                        if yuvBuf.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                366i32,
                                b"allocating YUV buffer\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -1i32;
                            break;
                        } else {
                            crate::stdlib::memset(
                                yuvBuf as *mut libc::c_void,
                                127i32,
                                yuvSize as libc::c_ulong,
                            );
                        }
                    }
                    iter = -1i32;
                    elapsedEncode = 0.0f64;
                    elapsed = elapsedEncode;
                    loop {
                        let mut tile: libc::c_int = 0i32;
                        totalJpegSize = 0i32;
                        start = crate::tjutil::getTime();
                        row = 0i32;
                        srcPtr = srcBuf;
                        while row < ntilesh {
                            col = 0i32;
                            srcPtr2 = srcPtr;
                            while col < ntilesw {
                                let mut width: libc::c_int = if tilew < w - col * tilew {
                                    tilew
                                } else {
                                    w - col * tilew
                                };
                                let mut height: libc::c_int = if tileh < h - row * tileh {
                                    tileh
                                } else {
                                    h - row * tileh
                                };
                                if 0 != doYUV {
                                    let mut startEncode: libc::c_double = crate::tjutil::getTime();
                                    if crate::turbojpeg::tjEncodeYUV3(
                                        handle, srcPtr2, width, pitch, height, pf, yuvBuf, yuvPad,
                                        subsamp, flags,
                                    ) == -1i32
                                    {
                                        let mut _tjErrorCode_0: libc::c_int =
                                            crate::turbojpeg::tjGetErrorCode(handle);
                                        let mut _tjErrorStr_0: *mut libc::c_char =
                                            crate::turbojpeg::tjGetErrorStr2(handle);
                                        if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                                            && _tjErrorCode_0
                                                == crate::turbojpeg::TJERR_WARNING as libc::c_int
                                        {
                                            if 0 != crate::stdlib::strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_0,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            ) || 0
                                                != crate::stdlib::strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjEncodeYUV3()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                )
                                                || tjErrorCode != _tjErrorCode_0
                                                || tjErrorLine != 390i32
                                            {
                                                crate::stdlib::strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_0,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                crate::stdlib::strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjEncodeYUV3()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_0;
                                                tjErrorLine = 390i32;
                                                crate::stdlib::printf(
                                                    b"WARNING in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    390i32,
                                                    b"executing tjEncodeYUV3()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_0,
                                                );
                                            }
                                        } else {
                                            crate::stdlib::printf(
                                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                    as *const libc::c_char,
                                                if _tjErrorCode_0
                                                    == crate::turbojpeg::TJERR_WARNING
                                                        as libc::c_int
                                                {
                                                    b"WARNING\x00" as *const u8
                                                        as *const libc::c_char
                                                } else {
                                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                                },
                                                390i32,
                                                b"executing tjEncodeYUV3()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_0,
                                            );
                                            retval = -1i32;
                                            break 's_73;
                                        }
                                    }
                                    if iter >= 0i32 {
                                        elapsedEncode += crate::tjutil::getTime() - startEncode
                                    }
                                    if crate::turbojpeg::tjCompressFromYUV(
                                        handle,
                                        yuvBuf,
                                        width,
                                        yuvPad,
                                        height,
                                        subsamp,
                                        &mut *jpegBuf.offset(tile as isize),
                                        &mut *jpegSize.offset(tile as isize),
                                        jpegQual,
                                        flags,
                                    ) == -1i32
                                    {
                                        let mut _tjErrorCode_1: libc::c_int =
                                            crate::turbojpeg::tjGetErrorCode(handle);
                                        let mut _tjErrorStr_1: *mut libc::c_char =
                                            crate::turbojpeg::tjGetErrorStr2(handle);
                                        if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                                            && _tjErrorCode_1
                                                == crate::turbojpeg::TJERR_WARNING as libc::c_int
                                        {
                                            if 0 != crate::stdlib::strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_1,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            ) || 0
                                                != crate::stdlib::strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjCompressFromYUV()\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                )
                                                || tjErrorCode != _tjErrorCode_1
                                                || tjErrorLine != 395i32
                                            {
                                                crate::stdlib::strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_1,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                crate::stdlib::strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjCompressFromYUV()\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_1;
                                                tjErrorLine = 395i32;
                                                crate::stdlib::printf(
                                                    b"WARNING in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    395i32,
                                                    b"executing tjCompressFromYUV()\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_1,
                                                );
                                            }
                                        } else {
                                            crate::stdlib::printf(
                                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                    as *const libc::c_char,
                                                if _tjErrorCode_1
                                                    == crate::turbojpeg::TJERR_WARNING
                                                        as libc::c_int
                                                {
                                                    b"WARNING\x00" as *const u8
                                                        as *const libc::c_char
                                                } else {
                                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                                },
                                                395i32,
                                                b"executing tjCompressFromYUV()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_1,
                                            );
                                            retval = -1i32;
                                            break 's_73;
                                        }
                                    }
                                } else if crate::turbojpeg::tjCompress2(
                                    handle,
                                    srcPtr2,
                                    width,
                                    pitch,
                                    height,
                                    pf,
                                    &mut *jpegBuf.offset(tile as isize),
                                    &mut *jpegSize.offset(tile as isize),
                                    subsamp,
                                    jpegQual,
                                    flags,
                                ) == -1i32
                                {
                                    let mut _tjErrorCode_2: libc::c_int =
                                        crate::turbojpeg::tjGetErrorCode(handle);
                                    let mut _tjErrorStr_2: *mut libc::c_char =
                                        crate::turbojpeg::tjGetErrorStr2(handle);
                                    if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                                        && _tjErrorCode_2
                                            == crate::turbojpeg::TJERR_WARNING as libc::c_int
                                    {
                                        if 0 != crate::stdlib::strncmp(
                                            tjErrorStr.as_mut_ptr(),
                                            _tjErrorStr_2,
                                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                        ) || 0
                                            != crate::stdlib::strncmp(
                                                tjErrorMsg.as_mut_ptr(),
                                                b"executing tjCompress2()\x00" as *const u8
                                                    as *const libc::c_char,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            )
                                            || tjErrorCode != _tjErrorCode_2
                                            || tjErrorLine != 400i32
                                        {
                                            crate::stdlib::strncpy(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_2,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            );
                                            crate::stdlib::strncpy(
                                                tjErrorMsg.as_mut_ptr(),
                                                b"executing tjCompress2()\x00" as *const u8
                                                    as *const libc::c_char,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            );
                                            tjErrorCode = _tjErrorCode_2;
                                            tjErrorLine = 400i32;
                                            crate::stdlib::printf(
                                                b"WARNING in line %d while %s:\n%s\n\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                400i32,
                                                b"executing tjCompress2()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_2,
                                            );
                                        }
                                    } else {
                                        crate::stdlib::printf(
                                            b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            if _tjErrorCode_2
                                                == crate::turbojpeg::TJERR_WARNING as libc::c_int
                                            {
                                                b"WARNING\x00" as *const u8 as *const libc::c_char
                                            } else {
                                                b"ERROR\x00" as *const u8 as *const libc::c_char
                                            },
                                            400i32,
                                            b"executing tjCompress2()\x00" as *const u8
                                                as *const libc::c_char,
                                            _tjErrorStr_2,
                                        );
                                        retval = -1i32;
                                        break 's_73;
                                    }
                                }
                                totalJpegSize = (totalJpegSize as libc::c_ulong)
                                    .wrapping_add(*jpegSize.offset(tile as isize))
                                    as libc::c_int
                                    as libc::c_int;
                                col += 1;
                                tile += 1;
                                srcPtr2 = srcPtr2.offset((ps * tilew) as isize)
                            }
                            row += 1;
                            srcPtr = srcPtr.offset((pitch * tileh) as isize)
                        }
                        elapsed += crate::tjutil::getTime() - start;
                        if iter >= 0i32 {
                            iter += 1;
                            if elapsed >= benchTime {
                                break;
                            }
                        } else if elapsed >= warmup {
                            iter = 0i32;
                            elapsedEncode = 0.0f64;
                            elapsed = elapsedEncode
                        }
                    }
                    if 0 != doYUV {
                        elapsed -= elapsedEncode
                    }
                    if crate::turbojpeg::tjDestroy(handle) == -1i32 {
                        let mut _tjErrorCode_3: libc::c_int =
                            crate::turbojpeg::tjGetErrorCode(handle);
                        let mut _tjErrorStr_3: *mut libc::c_char =
                            crate::turbojpeg::tjGetErrorStr2(handle);
                        if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                            && _tjErrorCode_3 == crate::turbojpeg::TJERR_WARNING as libc::c_int
                        {
                            if 0 != crate::stdlib::strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr_3,
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            ) || 0
                                != crate::stdlib::strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDestroy()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                )
                                || tjErrorCode != _tjErrorCode_3
                                || tjErrorLine != 416i32
                            {
                                crate::stdlib::strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr_3,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                crate::stdlib::strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDestroy()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                tjErrorCode = _tjErrorCode_3;
                                tjErrorLine = 416i32;
                                crate::stdlib::printf(
                                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    416i32,
                                    b"executing tjDestroy()\x00" as *const u8
                                        as *const libc::c_char,
                                    _tjErrorStr_3,
                                );
                            }
                        } else {
                            crate::stdlib::printf(
                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                if _tjErrorCode_3 == crate::turbojpeg::TJERR_WARNING as libc::c_int
                                {
                                    b"WARNING\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                },
                                416i32,
                                b"executing tjDestroy()\x00" as *const u8 as *const libc::c_char,
                                _tjErrorStr_3,
                            );
                            retval = -1i32;
                            break;
                        }
                    }
                    handle = crate::stddef_h::NULL_0 as *mut libc::c_void;
                    if quiet == 1i32 {
                        crate::stdlib::printf(
                            b"%-5d  %-5d   \x00" as *const u8 as *const libc::c_char,
                            tilew,
                            tileh,
                        );
                    }
                    if 0 != quiet {
                        if 0 != doYUV {
                            crate::stdlib::printf(
                                b"%-6s%s\x00" as *const u8 as *const libc::c_char,
                                sigfig(
                                    (w * h) as libc::c_double / 1000000.0f64
                                        * iter as libc::c_double
                                        / elapsedEncode,
                                    4i32,
                                    tempStr.as_mut_ptr(),
                                    1024i32,
                                ),
                                if quiet == 2i32 {
                                    b"\n\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"  \x00" as *const u8 as *const libc::c_char
                                },
                            );
                        }
                        crate::stdlib::printf(
                            b"%-6s%s\x00" as *const u8 as *const libc::c_char,
                            sigfig(
                                (w * h) as libc::c_double / 1000000.0f64 * iter as libc::c_double
                                    / elapsed,
                                4i32,
                                tempStr.as_mut_ptr(),
                                1024i32,
                            ),
                            if quiet == 2i32 {
                                b"\n\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"  \x00" as *const u8 as *const libc::c_char
                            },
                        );
                        crate::stdlib::printf(
                            b"%-6s%s\x00" as *const u8 as *const libc::c_char,
                            sigfig(
                                (w * h * ps) as libc::c_double / totalJpegSize as libc::c_double,
                                4i32,
                                tempStr2.as_mut_ptr(),
                                80i32,
                            ),
                            if quiet == 2i32 {
                                b"\n\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"  \x00" as *const u8 as *const libc::c_char
                            },
                        );
                    } else {
                        crate::stdlib::printf(
                            b"\n%s size: %d x %d\n\x00" as *const u8 as *const libc::c_char,
                            if 0 != doTile {
                                b"Tile\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"Image\x00" as *const u8 as *const libc::c_char
                            },
                            tilew,
                            tileh,
                        );
                        if 0 != doYUV {
                            crate::stdlib::printf(
                                b"Encode YUV    --> Frame rate:         %f fps\n\x00" as *const u8
                                    as *const libc::c_char,
                                iter as libc::c_double / elapsedEncode,
                            );
                            crate::stdlib::printf(
                                b"                  Output image size:  %d bytes\n\x00" as *const u8
                                    as *const libc::c_char,
                                yuvSize,
                            );
                            crate::stdlib::printf(
                                b"                  Compression ratio:  %f:1\n\x00" as *const u8
                                    as *const libc::c_char,
                                (w * h * ps) as libc::c_double / yuvSize as libc::c_double,
                            );
                            crate::stdlib::printf(
                                b"                  Throughput:         %f Megapixels/sec\n\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                (w * h) as libc::c_double / 1000000.0f64 * iter as libc::c_double
                                    / elapsedEncode,
                            );
                            crate::stdlib::printf(
                                b"                  Output bit stream:  %f Megabits/sec\n\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                yuvSize as libc::c_double * 8.0f64 / 1000000.0f64
                                    * iter as libc::c_double
                                    / elapsedEncode,
                            );
                        }
                        crate::stdlib::printf(
                            b"%s --> Frame rate:         %f fps\n\x00" as *const u8
                                as *const libc::c_char,
                            if 0 != doYUV {
                                b"Comp from YUV\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"Compress     \x00" as *const u8 as *const libc::c_char
                            },
                            iter as libc::c_double / elapsed,
                        );
                        crate::stdlib::printf(
                            b"                  Output image size:  %d bytes\n\x00" as *const u8
                                as *const libc::c_char,
                            totalJpegSize,
                        );
                        crate::stdlib::printf(
                            b"                  Compression ratio:  %f:1\n\x00" as *const u8
                                as *const libc::c_char,
                            (w * h * ps) as libc::c_double / totalJpegSize as libc::c_double,
                        );
                        crate::stdlib::printf(
                            b"                  Throughput:         %f Megapixels/sec\n\x00"
                                as *const u8 as *const libc::c_char,
                            (w * h) as libc::c_double / 1000000.0f64 * iter as libc::c_double
                                / elapsed,
                        );
                        crate::stdlib::printf(
                            b"                  Output bit stream:  %f Megabits/sec\n\x00"
                                as *const u8 as *const libc::c_char,
                            totalJpegSize as libc::c_double * 8.0f64 / 1000000.0f64
                                * iter as libc::c_double
                                / elapsed,
                        );
                    }
                    if tilew == w && tileh == h && 0 != doWrite {
                        crate::stdlib::snprintf(
                            tempStr.as_mut_ptr(),
                            1024i32 as libc::c_ulong,
                            b"%s_%s_Q%d.jpg\x00" as *const u8 as *const libc::c_char,
                            fileName,
                            subName[subsamp as usize],
                            jpegQual,
                        );
                        file = crate::stdlib::fopen(
                            tempStr.as_mut_ptr(),
                            b"wb\x00" as *const u8 as *const libc::c_char,
                        );
                        if file.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                463i32,
                                b"opening reference image\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -1i32;
                            break;
                        } else if crate::stdlib::fwrite(
                            *jpegBuf.offset(0isize) as *const libc::c_void,
                            *jpegSize.offset(0isize),
                            1i32 as libc::c_ulong,
                            file,
                        ) != 1i32 as libc::c_ulong
                        {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                465i32,
                                b"writing reference image\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -1i32;
                            break;
                        } else {
                            crate::stdlib::fclose(file);
                            file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
                            if 0 == quiet {
                                crate::stdlib::printf(
                                    b"Reference image written to %s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    tempStr.as_mut_ptr(),
                                );
                            }
                        }
                    }
                    /* Decompression test */
                    if 0 == compOnly {
                        if decomp(
                            srcBuf, jpegBuf, jpegSize, tmpBuf, w, h, subsamp, jpegQual, fileName,
                            tilew, tileh,
                        ) == -1i32
                        {
                            break;
                        }
                    }
                    i = 0i32;
                    while i < ntilesw * ntilesh {
                        if !(*jpegBuf.offset(i as isize)).is_null() {
                            crate::turbojpeg::tjFree(*jpegBuf.offset(i as isize));
                        }
                        let ref mut fresh1 = *jpegBuf.offset(i as isize);
                        *fresh1 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                        i += 1
                    }
                    crate::stdlib::free(jpegBuf as *mut libc::c_void);
                    jpegBuf = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
                    crate::stdlib::free(jpegSize as *mut libc::c_void);
                    jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong;
                    if 0 != doYUV {
                        crate::stdlib::free(yuvBuf as *mut libc::c_void);
                        yuvBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
                    }
                    if tilew == w && tileh == h {
                        break;
                    }
                    tilew *= 2i32;
                    tileh *= 2i32
                }
            }
        }
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
        file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE
    }
    if !jpegBuf.is_null() {
        i = 0i32;
        while i < ntilesw * ntilesh {
            if !(*jpegBuf.offset(i as isize)).is_null() {
                crate::turbojpeg::tjFree(*jpegBuf.offset(i as isize));
            }
            let ref mut fresh2 = *jpegBuf.offset(i as isize);
            *fresh2 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
            i += 1
        }
        crate::stdlib::free(jpegBuf as *mut libc::c_void);
        jpegBuf = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
        yuvBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
    }
    if !jpegSize.is_null() {
        crate::stdlib::free(jpegSize as *mut libc::c_void);
        jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong
    }
    if !tmpBuf.is_null() {
        crate::stdlib::free(tmpBuf as *mut libc::c_void);
        tmpBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
    }
    if !handle.is_null() {
        crate::turbojpeg::tjDestroy(handle);
        handle = crate::stddef_h::NULL_0 as *mut libc::c_void
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn decompTest(mut fileName: *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut handle: crate::turbojpeg::tjhandle = crate::stddef_h::NULL_0 as *mut libc::c_void;
    let mut jpegBuf: *mut *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut jpegSize: *mut libc::c_ulong = crate::stddef_h::NULL_0 as *mut libc::c_ulong;
    let mut srcSize: libc::c_ulong = 0;
    let mut totalJpegSize: libc::c_ulong = 0;
    let mut t: *mut crate::turbojpeg::tjtransform =
        crate::stddef_h::NULL_0 as *mut crate::turbojpeg::tjtransform;
    let mut start: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut tile: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut retval: libc::c_int = 0i32;
    let mut decompsrc: libc::c_int = 0i32;
    let mut temp: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut tempStr: [libc::c_char; 80] = [0; 80];
    let mut tempStr2: [libc::c_char; 80] = [0; 80];
    /* Original image */
    let mut w: libc::c_int = 0i32;
    let mut h: libc::c_int = 0i32;
    let mut tilew: libc::c_int = 0;
    let mut tileh: libc::c_int = 0;
    let mut ntilesw: libc::c_int = 1i32;
    let mut ntilesh: libc::c_int = 1i32;
    let mut subsamp: libc::c_int = -1i32;
    let mut cs: libc::c_int = -1i32;
    /* Transformed image */
    let mut tw: libc::c_int = 0;
    let mut th: libc::c_int = 0;
    let mut ttilew: libc::c_int = 0;
    let mut ttileh: libc::c_int = 0;
    let mut tntilesw: libc::c_int = 0;
    let mut tntilesh: libc::c_int = 0;
    let mut tsubsamp: libc::c_int = 0;
    file = crate::stdlib::fopen(fileName, b"rb\x00" as *const u8 as *const libc::c_char);
    if file.is_null() {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            524i32,
            b"opening file\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        retval = -1i32
    } else if crate::stdlib::fseek(file, 0i32 as libc::c_long, crate::stdlib::SEEK_END) < 0i32 || {
        srcSize = crate::stdlib::ftell(file) as libc::c_ulong;
        srcSize == -1i32 as libc::c_ulong
    } {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            527i32,
            b"determining file size\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        retval = -1i32
    } else {
        srcBuf = crate::stdlib::malloc(srcSize) as *mut libc::c_uchar;
        if srcBuf.is_null() {
            crate::stdlib::printf(
                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                529i32,
                b"allocating memory\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -1i32
        } else if crate::stdlib::fseek(file, 0i32 as libc::c_long, crate::stdlib::SEEK_SET) < 0i32 {
            crate::stdlib::printf(
                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                531i32,
                b"setting file position\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -1i32
        } else if crate::stdlib::fread(
            srcBuf as *mut libc::c_void,
            srcSize,
            1i32 as libc::c_ulong,
            file,
        ) < 1i32 as libc::c_ulong
        {
            crate::stdlib::printf(
                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                533i32,
                b"reading JPEG data\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -1i32
        } else {
            crate::stdlib::fclose(file);
            file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
            temp = crate::stdlib::strrchr(fileName, '.' as i32);
            if !temp.is_null() {
                *temp = '\u{0}' as i32 as libc::c_char
            }
            handle = crate::turbojpeg::tjInitTransform();
            if handle.is_null() {
                let mut _tjErrorCode: libc::c_int = crate::turbojpeg::tjGetErrorCode(handle);
                let mut _tjErrorStr: *mut libc::c_char = crate::turbojpeg::tjGetErrorStr2(handle);
                if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                    && _tjErrorCode == crate::turbojpeg::TJERR_WARNING as libc::c_int
                {
                    if 0 != crate::stdlib::strncmp(
                        tjErrorStr.as_mut_ptr(),
                        _tjErrorStr,
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    ) || 0
                        != crate::stdlib::strncmp(
                            tjErrorMsg.as_mut_ptr(),
                            b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        )
                        || tjErrorCode != _tjErrorCode
                        || tjErrorLine != 540i32
                    {
                        crate::stdlib::strncpy(
                            tjErrorStr.as_mut_ptr(),
                            _tjErrorStr,
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        );
                        crate::stdlib::strncpy(
                            tjErrorMsg.as_mut_ptr(),
                            b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        );
                        tjErrorCode = _tjErrorCode;
                        tjErrorLine = 540i32;
                        crate::stdlib::printf(
                            b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            540i32,
                            b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                            _tjErrorStr,
                        );
                    }
                    current_block = 9441801433784995173;
                } else {
                    crate::stdlib::printf(
                        b"%s in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        if _tjErrorCode == crate::turbojpeg::TJERR_WARNING as libc::c_int {
                            b"WARNING\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"ERROR\x00" as *const u8 as *const libc::c_char
                        },
                        540i32,
                        b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                        _tjErrorStr,
                    );
                    retval = -1i32;
                    current_block = 18327231901424809080;
                }
            } else {
                current_block = 9441801433784995173;
            }
            match current_block {
                18327231901424809080 => {}
                _ => {
                    if crate::turbojpeg::tjDecompressHeader3(
                        handle,
                        srcBuf,
                        srcSize,
                        &mut w,
                        &mut h,
                        &mut subsamp,
                        &mut cs,
                    ) == -1i32
                    {
                        let mut _tjErrorCode_0: libc::c_int =
                            crate::turbojpeg::tjGetErrorCode(handle);
                        let mut _tjErrorStr_0: *mut libc::c_char =
                            crate::turbojpeg::tjGetErrorStr2(handle);
                        if 0 == flags & crate::turbojpeg::TJFLAG_STOPONWARNING
                            && _tjErrorCode_0 == crate::turbojpeg::TJERR_WARNING as libc::c_int
                        {
                            if 0 != crate::stdlib::strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr_0,
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            ) || 0
                                != crate::stdlib::strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDecompressHeader3()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                )
                                || tjErrorCode != _tjErrorCode_0
                                || tjErrorLine != 543i32
                            {
                                crate::stdlib::strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr_0,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                crate::stdlib::strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDecompressHeader3()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                tjErrorCode = _tjErrorCode_0;
                                tjErrorLine = 543i32;
                                crate::stdlib::printf(
                                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    543i32,
                                    b"executing tjDecompressHeader3()\x00" as *const u8
                                        as *const libc::c_char,
                                    _tjErrorStr_0,
                                );
                            }
                            current_block = 168769493162332264;
                        } else {
                            crate::stdlib::printf(
                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                if _tjErrorCode_0 == crate::turbojpeg::TJERR_WARNING as libc::c_int
                                {
                                    b"WARNING\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                },
                                543i32,
                                b"executing tjDecompressHeader3()\x00" as *const u8
                                    as *const libc::c_char,
                                _tjErrorStr_0,
                            );
                            retval = -1i32;
                            current_block = 18327231901424809080;
                        }
                    } else {
                        current_block = 168769493162332264;
                    }
                    match current_block {
                        18327231901424809080 => {}
                        _ => {
                            if cs == crate::turbojpeg::TJCS_YCCK as libc::c_int
                                || cs == crate::turbojpeg::TJCS_CMYK as libc::c_int
                            {
                                pf = crate::turbojpeg::TJPF_CMYK as libc::c_int;
                                ps = crate::turbojpeg::tjPixelSize[pf as usize]
                            }
                            if quiet == 1i32 {
                                crate::stdlib::printf(
                                    b"All performance values in Mpixels/sec\n\n\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                crate::stdlib::printf(b"Bitmap     JPEG   JPEG     %s  %s   Xform   Comp    Decomp  \x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if 0 != doTile {
                                           b"Tile \x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"Image\x00" as *const u8 as
                                               *const libc::c_char
                                       },
                                       if 0 != doTile {
                                           b"Tile \x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"Image\x00" as *const u8 as
                                               *const libc::c_char
                                       });
                                if 0 != doYUV {
                                    crate::stdlib::printf(
                                        b"Decode\x00" as *const u8 as *const libc::c_char,
                                    );
                                }
                                crate::stdlib::printf(
                                    b"\n\x00" as *const u8 as *const libc::c_char,
                                );
                                crate::stdlib::printf(b"Format     CS     Subsamp  Width  Height  Perf    Ratio   Perf    \x00"
                                           as *const u8 as
                                           *const libc::c_char);
                                if 0 != doYUV {
                                    crate::stdlib::printf(
                                        b"Perf\x00" as *const u8 as *const libc::c_char,
                                    );
                                }
                                crate::stdlib::printf(
                                    b"\n\n\x00" as *const u8 as *const libc::c_char,
                                );
                            } else if 0 == quiet {
                                crate::stdlib::printf(
                                    b">>>>>  JPEG %s --> %s (%s)  <<<<<\n\x00" as *const u8
                                        as *const libc::c_char,
                                    formatName(subsamp, cs, tempStr.as_mut_ptr()),
                                    pixFormatStr[pf as usize],
                                    if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                                        b"Bottom-up\x00" as *const u8 as *const libc::c_char
                                    } else {
                                        b"Top-down\x00" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            tilew = if 0 != doTile { 16i32 } else { w };
                            tileh = if 0 != doTile { 16i32 } else { h };
                            's_381: loop {
                                if tilew > w {
                                    tilew = w
                                }
                                if tileh > h {
                                    tileh = h
                                }
                                ntilesw = (w + tilew - 1i32) / tilew;
                                ntilesh = (h + tileh - 1i32) / tileh;
                                jpegBuf = crate::stdlib::malloc(
                                    (::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong)
                                        .wrapping_mul(ntilesw as libc::c_ulong)
                                        .wrapping_mul(ntilesh as libc::c_ulong),
                                )
                                    as *mut *mut libc::c_uchar;
                                if jpegBuf.is_null() {
                                    crate::stdlib::printf(
                                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                            as *const libc::c_char,
                                        571i32,
                                        b"allocating JPEG tile array\x00" as *const u8
                                            as *const libc::c_char,
                                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                                    );
                                    retval = -1i32;
                                    break;
                                } else {
                                    crate::stdlib::memset(
                                        jpegBuf as *mut libc::c_void,
                                        0i32,
                                        (::std::mem::size_of::<*mut libc::c_uchar>()
                                            as libc::c_ulong)
                                            .wrapping_mul(ntilesw as libc::c_ulong)
                                            .wrapping_mul(ntilesh as libc::c_ulong),
                                    );
                                    jpegSize = crate::stdlib::malloc(
                                        (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(ntilesw as libc::c_ulong)
                                            .wrapping_mul(ntilesh as libc::c_ulong),
                                    )
                                        as *mut libc::c_ulong;
                                    if jpegSize.is_null() {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            575i32,
                                            b"allocating JPEG size array\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -1i32;
                                        break;
                                    } else {
                                        crate::stdlib::memset(
                                            jpegSize as *mut libc::c_void,
                                            0i32,
                                            (::std::mem::size_of::<libc::c_ulong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(ntilesw as libc::c_ulong)
                                                .wrapping_mul(ntilesh as libc::c_ulong),
                                        );
                                        if flags & crate::turbojpeg::TJFLAG_NOREALLOC != 0i32
                                            || 0 == doTile
                                        {
                                            i = 0i32;
                                            while i < ntilesw * ntilesh {
                                                let ref mut fresh3 = *jpegBuf.offset(i as isize);
                                                *fresh3 = crate::turbojpeg::tjAlloc(
                                                    crate::turbojpeg::tjBufSize(
                                                        tilew, tileh, subsamp,
                                                    )
                                                        as libc::c_int,
                                                );
                                                if (*fresh3).is_null() {
                                                    crate::stdlib::printf(
                                                        b"ERROR in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        582i32,
                                                        b"allocating JPEG tiles\x00" as *const u8
                                                            as *const libc::c_char,
                                                        crate::stdlib::strerror(
                                                            *crate::stdlib::__errno_location(),
                                                        ),
                                                    );
                                                    retval = -1i32;
                                                    break 's_381;
                                                } else {
                                                    i += 1
                                                }
                                            }
                                        }
                                        tw = w;
                                        th = h;
                                        ttilew = tilew;
                                        ttileh = tileh;
                                        if 0 == quiet {
                                            crate::stdlib::printf(
                                                b"\n%s size: %d x %d\x00" as *const u8
                                                    as *const libc::c_char,
                                                if 0 != doTile {
                                                    b"Tile\x00" as *const u8 as *const libc::c_char
                                                } else {
                                                    b"Image\x00" as *const u8 as *const libc::c_char
                                                },
                                                ttilew,
                                                ttileh,
                                            );
                                            if sf.num != 1i32 || sf.denom != 1i32 {
                                                crate::stdlib::printf(
                                                    b" --> %d x %d\x00" as *const u8
                                                        as *const libc::c_char,
                                                    (tw * sf.num + sf.denom - 1i32) / sf.denom,
                                                    (th * sf.num + sf.denom - 1i32) / sf.denom,
                                                );
                                            }
                                            crate::stdlib::printf(
                                                b"\n\x00" as *const u8 as *const libc::c_char,
                                            );
                                        } else if quiet == 1i32 {
                                            crate::stdlib::printf(
                                                b"%-4s (%s)  %-5s  %-5s    \x00" as *const u8
                                                    as *const libc::c_char,
                                                pixFormatStr[pf as usize],
                                                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                                                    b"BU\x00" as *const u8 as *const libc::c_char
                                                } else {
                                                    b"TD\x00" as *const u8 as *const libc::c_char
                                                },
                                                csName[cs as usize],
                                                subNameLong[subsamp as usize],
                                            );
                                            crate::stdlib::printf(
                                                b"%-5d  %-5d   \x00" as *const u8
                                                    as *const libc::c_char,
                                                tilew,
                                                tileh,
                                            );
                                        }
                                        tsubsamp = subsamp;
                                        if 0 != doTile
                                            || xformOp
                                                != crate::turbojpeg::TJXOP_NONE as libc::c_int
                                            || xformOpt != 0i32
                                            || customFilter.is_some()
                                        {
                                            t =
                                                crate::stdlib::malloc(
                                                    (::std::mem::size_of::<
                                                        crate::turbojpeg::tjtransform,
                                                    >(
                                                    )
                                                        as libc::c_ulong)
                                                        .wrapping_mul(ntilesw as libc::c_ulong)
                                                        .wrapping_mul(ntilesh as libc::c_ulong),
                                                )
                                                    as *mut crate::turbojpeg::tjtransform;
                                            if t.is_null() {
                                                crate::stdlib::printf(
                                                    b"ERROR in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    602i32,
                                                    b"allocating image transform array\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    crate::stdlib::strerror(
                                                        *crate::stdlib::__errno_location(),
                                                    ),
                                                );
                                                retval = -1i32;
                                                break;
                                            } else {
                                                if xformOp
                                                    == crate::turbojpeg::TJXOP_TRANSPOSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_TRANSVERSE
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT90
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT270
                                                            as libc::c_int
                                                {
                                                    tw = h;
                                                    th = w;
                                                    ttilew = tileh;
                                                    ttileh = tilew
                                                }
                                                if 0 != xformOpt & crate::turbojpeg::TJXOPT_GRAY {
                                                    tsubsamp = crate::turbojpeg::TJ_GRAYSCALE
                                                }
                                                if xformOp
                                                    == crate::turbojpeg::TJXOP_HFLIP as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT180
                                                            as libc::c_int
                                                {
                                                    tw = tw
                                                        - tw % crate::turbojpeg::tjMCUWidth
                                                            [tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == crate::turbojpeg::TJXOP_VFLIP as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT180
                                                            as libc::c_int
                                                {
                                                    th = th
                                                        - th % crate::turbojpeg::tjMCUHeight
                                                            [tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == crate::turbojpeg::TJXOP_TRANSVERSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT90
                                                            as libc::c_int
                                                {
                                                    tw = tw
                                                        - tw % crate::turbojpeg::tjMCUHeight
                                                            [tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == crate::turbojpeg::TJXOP_TRANSVERSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT270
                                                            as libc::c_int
                                                {
                                                    th = th
                                                        - th % crate::turbojpeg::tjMCUWidth
                                                            [tsubsamp as usize]
                                                }
                                                tntilesw = (tw + ttilew - 1i32) / ttilew;
                                                tntilesh = (th + ttileh - 1i32) / ttileh;
                                                if xformOp
                                                    == crate::turbojpeg::TJXOP_TRANSPOSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_TRANSVERSE
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT90
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg::TJXOP_ROT270
                                                            as libc::c_int
                                                {
                                                    if tsubsamp
                                                        == crate::turbojpeg::TJSAMP_422
                                                            as libc::c_int
                                                    {
                                                        tsubsamp = crate::turbojpeg::TJSAMP_440
                                                            as libc::c_int
                                                    } else if tsubsamp
                                                        == crate::turbojpeg::TJSAMP_440
                                                            as libc::c_int
                                                    {
                                                        tsubsamp = crate::turbojpeg::TJSAMP_422
                                                            as libc::c_int
                                                    }
                                                }
                                                row = 0i32;
                                                tile = 0i32;
                                                while row < tntilesh {
                                                    col = 0i32;
                                                    while col < tntilesw {
                                                        (*t.offset(tile as isize)).r.w =
                                                            if ttilew < tw - col * ttilew {
                                                                ttilew
                                                            } else {
                                                                tw - col * ttilew
                                                            };
                                                        (*t.offset(tile as isize)).r.h =
                                                            if ttileh < th - row * ttileh {
                                                                ttileh
                                                            } else {
                                                                th - row * ttileh
                                                            };
                                                        (*t.offset(tile as isize)).r.x =
                                                            col * ttilew;
                                                        (*t.offset(tile as isize)).r.y =
                                                            row * ttileh;
                                                        (*t.offset(tile as isize)).op = xformOp;
                                                        (*t.offset(tile as isize)).options =
                                                            xformOpt
                                                                | crate::turbojpeg::TJXOPT_TRIM;
                                                        let ref mut fresh4 =
                                                            (*t.offset(tile as isize)).customFilter;
                                                        *fresh4 = customFilter;
                                                        if 0 != (*t.offset(tile as isize)).options
                                                            & crate::turbojpeg::TJXOPT_NOOUTPUT
                                                            && !(*jpegBuf.offset(tile as isize))
                                                                .is_null()
                                                        {
                                                            crate::turbojpeg::tjFree(
                                                                *jpegBuf.offset(tile as isize),
                                                            );
                                                            let ref mut fresh5 =
                                                                *jpegBuf.offset(tile as isize);
                                                            *fresh5 = crate::stddef_h::NULL_0
                                                                as *mut libc::c_uchar
                                                        }
                                                        col += 1;
                                                        tile += 1
                                                    }
                                                    row += 1
                                                }
                                                iter = -1i32;
                                                elapsed = 0.0f64;
                                                loop {
                                                    start = crate::tjutil::getTime();
                                                    if crate::turbojpeg::tjTransform(
                                                        handle,
                                                        srcBuf,
                                                        srcSize,
                                                        tntilesw * tntilesh,
                                                        jpegBuf,
                                                        jpegSize,
                                                        t,
                                                        flags,
                                                    ) == -1i32
                                                    {
                                                        let mut _tjErrorCode_1: libc::c_int =
                                                            crate::turbojpeg::tjGetErrorCode(
                                                                handle,
                                                            );
                                                        let mut _tjErrorStr_1: *mut libc::c_char =
                                                            crate::turbojpeg::tjGetErrorStr2(
                                                                handle,
                                                            );
                                                        if 0 == flags
                                                            & crate::turbojpeg::TJFLAG_STOPONWARNING
                                                            && _tjErrorCode_1
                                                                == crate::turbojpeg::TJERR_WARNING
                                                                    as libc::c_int
                                                        {
                                                            if 0 != crate::stdlib::strncmp(
                                                                tjErrorStr.as_mut_ptr(),
                                                                _tjErrorStr_1,
                                                                crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                    as libc::c_ulong,
                                                            ) || 0 != crate::stdlib::strncmp(
                                                                tjErrorMsg.as_mut_ptr(),
                                                                b"executing tjTransform()\x00"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                    as libc::c_ulong,
                                                            ) || tjErrorCode != _tjErrorCode_1
                                                                || tjErrorLine != 648i32
                                                            {
                                                                crate::stdlib::strncpy(tjErrorStr.as_mut_ptr(),
                                                                            _tjErrorStr_1,
                                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                                as
                                                                                libc::c_ulong);
                                                                crate::stdlib::strncpy(tjErrorMsg.as_mut_ptr(),
                                                                            b"executing tjTransform()\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                                as
                                                                                libc::c_ulong);
                                                                tjErrorCode = _tjErrorCode_1;
                                                                tjErrorLine = 648i32;
                                                                crate::stdlib::printf(b"WARNING in line %d while %s:\n%s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           648i32,
                                                                           b"executing tjTransform()\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           _tjErrorStr_1);
                                                            }
                                                        } else {
                                                            crate::stdlib::printf(b"%s in line %d while %s:\n%s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       if _tjErrorCode_1
                                                                              ==
                                                                              crate::turbojpeg::TJERR_WARNING
                                                                                  as
                                                                                  libc::c_int
                                                                          {
                                                                           b"WARNING\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       } else {
                                                                           b"ERROR\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       },
                                                                       648i32,
                                                                       b"executing tjTransform()\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       _tjErrorStr_1);
                                                            retval = -1i32;
                                                            break 's_381;
                                                        }
                                                    }
                                                    elapsed += crate::tjutil::getTime() - start;
                                                    if iter >= 0i32 {
                                                        iter += 1;
                                                        if elapsed >= benchTime {
                                                            break;
                                                        }
                                                    } else if elapsed >= warmup {
                                                        iter = 0i32;
                                                        elapsed = 0.0f64
                                                    }
                                                }
                                                crate::stdlib::free(t as *mut libc::c_void);
                                                t = crate::stddef_h::NULL_0
                                                    as *mut crate::turbojpeg::tjtransform;
                                                tile = 0i32;
                                                totalJpegSize = 0i32 as libc::c_ulong;
                                                while tile < tntilesw * tntilesh {
                                                    totalJpegSize = totalJpegSize.wrapping_add(
                                                        *jpegSize.offset(tile as isize),
                                                    );
                                                    tile += 1
                                                }
                                                if 0 != quiet {
                                                    crate::stdlib::printf(
                                                        b"%-6s%s%-6s%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        sigfig(
                                                            (w * h) as libc::c_double
                                                                / 1000000.0f64
                                                                / elapsed,
                                                            4i32,
                                                            tempStr.as_mut_ptr(),
                                                            80i32,
                                                        ),
                                                        if quiet == 2i32 {
                                                            b"\n\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"  \x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                        sigfig(
                                                            (w * h * ps) as libc::c_double
                                                                / totalJpegSize as libc::c_double,
                                                            4i32,
                                                            tempStr2.as_mut_ptr(),
                                                            80i32,
                                                        ),
                                                        if quiet == 2i32 {
                                                            b"\n\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"  \x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                    );
                                                } else if 0 == quiet {
                                                    crate::stdlib::printf(b"Transform     --> Frame rate:         %f fps\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               1.0f64 /
                                                                   elapsed);
                                                    crate::stdlib::printf(b"                  Output image size:  %lu bytes\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               totalJpegSize);
                                                    crate::stdlib::printf(b"                  Compression ratio:  %f:1\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (w * h * ps) as
                                                                   libc::c_double
                                                                   /
                                                                   totalJpegSize
                                                                       as
                                                                       libc::c_double);
                                                    crate::stdlib::printf(b"                  Throughput:         %f Megapixels/sec\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (w * h) as
                                                                   libc::c_double
                                                                   /
                                                                   1000000.0f64
                                                                   / elapsed);
                                                    crate::stdlib::printf(b"                  Output bit stream:  %f Megabits/sec\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               totalJpegSize
                                                                   as
                                                                   libc::c_double
                                                                   * 8.0f64 /
                                                                   1000000.0f64
                                                                   / elapsed);
                                                }
                                            }
                                        } else {
                                            if quiet == 1i32 {
                                                crate::stdlib::printf(
                                                    b"N/A     N/A     \x00" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            crate::turbojpeg::tjFree(*jpegBuf.offset(0isize));
                                            let ref mut fresh6 = *jpegBuf.offset(0isize);
                                            *fresh6 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                                            decompsrc = 1i32
                                        }
                                        if w == tilew {
                                            ttilew = tw
                                        }
                                        if h == tileh {
                                            ttileh = th
                                        }
                                        if 0 == xformOpt & crate::turbojpeg::TJXOPT_NOOUTPUT {
                                            if decomp(
                                                crate::stddef_h::NULL_0 as *mut libc::c_uchar,
                                                if 0 != decompsrc {
                                                    &mut srcBuf
                                                } else {
                                                    jpegBuf
                                                },
                                                if 0 != decompsrc {
                                                    &mut srcSize
                                                } else {
                                                    jpegSize
                                                },
                                                crate::stddef_h::NULL_0 as *mut libc::c_uchar,
                                                tw,
                                                th,
                                                tsubsamp,
                                                0i32,
                                                fileName,
                                                ttilew,
                                                ttileh,
                                            ) == -1i32
                                            {
                                                break;
                                            }
                                        } else if quiet == 1i32 {
                                            crate::stdlib::printf(
                                                b"N/A\n\x00" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        i = 0i32;
                                        while i < ntilesw * ntilesh {
                                            crate::turbojpeg::tjFree(*jpegBuf.offset(i as isize));
                                            let ref mut fresh7 = *jpegBuf.offset(i as isize);
                                            *fresh7 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                                            i += 1
                                        }
                                        crate::stdlib::free(jpegBuf as *mut libc::c_void);
                                        jpegBuf =
                                            crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
                                        if !jpegSize.is_null() {
                                            crate::stdlib::free(jpegSize as *mut libc::c_void);
                                            jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong
                                        }
                                        if tilew == w && tileh == h {
                                            break;
                                        }
                                        tilew *= 2i32;
                                        tileh *= 2i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
        file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE
    }
    if !jpegBuf.is_null() {
        i = 0i32;
        while i < ntilesw * ntilesh {
            if !(*jpegBuf.offset(i as isize)).is_null() {
                crate::turbojpeg::tjFree(*jpegBuf.offset(i as isize));
            }
            let ref mut fresh8 = *jpegBuf.offset(i as isize);
            *fresh8 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
            i += 1
        }
        crate::stdlib::free(jpegBuf as *mut libc::c_void);
        jpegBuf = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar
    }
    if !jpegSize.is_null() {
        crate::stdlib::free(jpegSize as *mut libc::c_void);
        jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong
    }
    if !srcBuf.is_null() {
        crate::stdlib::free(srcBuf as *mut libc::c_void);
        srcBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
    }
    if !t.is_null() {
        crate::stdlib::free(t as *mut libc::c_void);
        t = crate::stddef_h::NULL_0 as *mut crate::turbojpeg::tjtransform
    }
    if !handle.is_null() {
        crate::turbojpeg::tjDestroy(handle);
        handle = crate::stddef_h::NULL_0 as *mut libc::c_void
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut progName: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    crate::stdlib::printf(
        b"USAGE: %s\n\x00" as *const u8 as *const libc::c_char,
        progName,
    );
    crate::stdlib::printf(
        b"       <Inputfile (BMP|PPM)> <Quality> [options]\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"       %s\n\x00" as *const u8 as *const libc::c_char,
        progName,
    );
    crate::stdlib::printf(
        b"       <Inputfile (JPG)> [options]\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"Options:\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-alloc = Dynamically allocate JPEG image buffers\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-bmp = Generate output images in Windows Bitmap format (default = PPM)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-bottomup = Test bottom-up compression/decompression\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-tile = Test performance of the codec when the image is encoded as separate\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     tiles of varying sizes.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-rgb, -bgr, -rgbx, -bgrx, -xbgr, -xrgb =\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     Test the specified color conversion path in the codec (default = BGR)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-cmyk = Indirectly test YCCK JPEG compression/decompression (the source\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     and destination bitmaps are still RGB.  The conversion is done\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     internally prior to compression or after decompression.)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-fastupsample = Use the fastest chrominance upsampling algorithm available in\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     the underlying codec\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-fastdct = Use the fastest DCT/IDCT algorithms available in the underlying\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     codec\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-accuratedct = Use the most accurate DCT/IDCT algorithms available in the\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     underlying codec\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-progressive = Use progressive entropy coding in JPEG images generated by\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compression and transform operations.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-subsamp <s> = When testing JPEG compression, this option specifies the level\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     of chrominance subsampling to use (<s> = 444, 422, 440, 420, 411, or\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     GRAY).  The default is to test Grayscale, 4:2:0, 4:2:2, and 4:4:4 in\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     sequence.\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-quiet = Output results in tabular rather than verbose format\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-yuv = Test YUV encoding/decoding functions\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-yuvpad <p> = If testing YUV encoding/decoding, this specifies the number of\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     bytes to which each row of each plane in the intermediate YUV image is\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     padded (default = 1)\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-scale M/N = Scale down the width/height of the decompressed JPEG image by a\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     factor of M/N (M/N = \x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < nsf {
        crate::stdlib::printf(
            b"%d/%d\x00" as *const u8 as *const libc::c_char,
            (*scalingFactors.offset(i as isize)).num,
            (*scalingFactors.offset(i as isize)).denom,
        );
        if nsf == 2i32 && i != nsf - 1i32 {
            crate::stdlib::printf(b" or \x00" as *const u8 as *const libc::c_char);
        } else if nsf > 2i32 {
            if i != nsf - 1i32 {
                crate::stdlib::printf(b", \x00" as *const u8 as *const libc::c_char);
            }
            if i == nsf - 2i32 {
                crate::stdlib::printf(b"or \x00" as *const u8 as *const libc::c_char);
            }
        }
        if i % 8i32 == 0i32 && i != 0i32 {
            crate::stdlib::printf(b"\n     \x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    crate::stdlib::printf(b")\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270 =\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     Perform the corresponding lossless transform prior to\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     decompression (these options are mutually exclusive)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-grayscale = Perform lossless grayscale conversion prior to decompression\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     test (can be combined with the other transforms above)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-copynone = Do not copy any extra markers (including EXIF and ICC profile data)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     when transforming the image.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-benchtime <t> = Run each benchmark for at least <t> seconds (default = 5.0)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-warmup <t> = Run each benchmark for <t> seconds (default = 1.0) prior to\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     starting the timer, in order to prime the caches and thus improve the\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     consistency of the results.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-componly = Stop after running compression tests.  Do not test decompression.\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-nowrite = Do not write reference or output images (improves consistency of\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     performance measurements.)\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-stoponwarning = Immediately discontinue the current\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compression/decompression/transform operation if the underlying codec\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     throws a warning (non-fatal error)\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"NOTE:  If the quality is specified as a range (e.g. 90-100), a separate\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"test will be performed for all quality values in the range.\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::exit(1i32);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut w: libc::c_int = 0i32;
    let mut h: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut minQual: libc::c_int = -1i32;
    let mut maxQual: libc::c_int = -1i32;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut minArg: libc::c_int = 2i32;
    let mut retval: libc::c_int = 0i32;
    let mut subsamp: libc::c_int = -1i32;
    scalingFactors = crate::turbojpeg::tjGetScalingFactors(&mut nsf);
    if scalingFactors.is_null() || nsf == 0i32 {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            804i32,
            b"executing tjGetScalingFactors()\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg::tjGetErrorStr(),
        );
        retval = -1i32
    } else {
        if argc < minArg {
            usage(*argv.offset(0isize));
        }
        temp = crate::stdlib::strrchr(*argv.offset(1isize), '.' as i32);
        if !temp.is_null() {
            if 0 == crate::stdlib::strcasecmp(temp, b".bmp\x00" as *const u8 as *const libc::c_char)
            {
                ext = b"bmp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            if 0 == crate::stdlib::strcasecmp(temp, b".jpg\x00" as *const u8 as *const libc::c_char)
                || 0 == crate::stdlib::strcasecmp(
                    temp,
                    b".jpeg\x00" as *const u8 as *const libc::c_char,
                )
            {
                decompOnly = 1i32
            }
        }
        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
        if 0 == decompOnly {
            minArg = 3i32;
            if argc < minArg {
                usage(*argv.offset(0isize));
            }
            minQual = crate::stdlib::atoi(*argv.offset(2isize));
            if minQual < 1i32 || minQual > 100i32 {
                crate::stdlib::puts(
                    b"ERROR: Quality must be between 1 and 100.\x00" as *const u8
                        as *const libc::c_char,
                );
                crate::stdlib::exit(1i32);
            }
            temp = crate::stdlib::strchr(*argv.offset(2isize), '-' as i32);
            if !(!temp.is_null()
                && crate::stdlib::strlen(temp) > 1i32 as libc::c_ulong
                && crate::stdlib::sscanf(
                    &mut *temp.offset(1isize) as *mut libc::c_char,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    &mut maxQual as *mut libc::c_int,
                ) == 1i32
                && maxQual > minQual
                && maxQual >= 1i32
                && maxQual <= 100i32)
            {
                maxQual = minQual
            }
        }
        if argc > minArg {
            i = minArg;
            while i < argc {
                if 0 == crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-tile\x00" as *const u8 as *const libc::c_char,
                ) {
                    doTile = 1i32;
                    xformOpt |= crate::turbojpeg::TJXOPT_CROP
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-fastupsample\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    crate::stdlib::printf(
                        b"Using fast upsampling code\n\n\x00" as *const u8 as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg::TJFLAG_FASTUPSAMPLE
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-fastdct\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    crate::stdlib::printf(
                        b"Using fastest DCT/IDCT algorithm\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg::TJFLAG_FASTDCT
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-accuratedct\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    crate::stdlib::printf(
                        b"Using most accurate DCT/IDCT algorithm\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg::TJFLAG_ACCURATEDCT
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-progressive\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    crate::stdlib::printf(
                        b"Using progressive entropy coding\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg::TJFLAG_PROGRESSIVE
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-rgb\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    pf = crate::turbojpeg::TJPF_RGB as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-rgbx\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    pf = crate::turbojpeg::TJPF_RGBX as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-bgr\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    pf = crate::turbojpeg::TJPF_BGR as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-bgrx\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    pf = crate::turbojpeg::TJPF_BGRX as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-xbgr\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    pf = crate::turbojpeg::TJPF_XBGR as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-xrgb\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    pf = crate::turbojpeg::TJPF_XRGB as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-cmyk\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    pf = crate::turbojpeg::TJPF_CMYK as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-bottomup\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    flags |= crate::turbojpeg::TJFLAG_BOTTOMUP
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-quiet\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    quiet = 1i32
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-qq\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    quiet = 2i32
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-scale\x00" as *const u8 as *const libc::c_char,
                    )
                    && i < argc - 1i32
                {
                    let mut temp1: libc::c_int = 0i32;
                    let mut temp2: libc::c_int = 0i32;
                    let mut match_0: libc::c_int = 0i32;
                    i += 1;
                    if crate::stdlib::sscanf(
                        *argv.offset(i as isize),
                        b"%d/%d\x00" as *const u8 as *const libc::c_char,
                        &mut temp1 as *mut libc::c_int,
                        &mut temp2 as *mut libc::c_int,
                    ) == 2i32
                    {
                        j = 0i32;
                        while j < nsf {
                            if temp1 as libc::c_double / temp2 as libc::c_double
                                == (*scalingFactors.offset(j as isize)).num as libc::c_double
                                    / (*scalingFactors.offset(j as isize)).denom as libc::c_double
                            {
                                sf = *scalingFactors.offset(j as isize);
                                match_0 = 1i32;
                                break;
                            } else {
                                j += 1
                            }
                        }
                        if 0 == match_0 {
                            usage(*argv.offset(0isize));
                        }
                    } else {
                        usage(*argv.offset(0isize));
                    }
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-hflip\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOp = crate::turbojpeg::TJXOP_HFLIP as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-vflip\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOp = crate::turbojpeg::TJXOP_VFLIP as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-transpose\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOp = crate::turbojpeg::TJXOP_TRANSPOSE as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-transverse\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOp = crate::turbojpeg::TJXOP_TRANSVERSE as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-rot90\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOp = crate::turbojpeg::TJXOP_ROT90 as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-rot180\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOp = crate::turbojpeg::TJXOP_ROT180 as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-rot270\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOp = crate::turbojpeg::TJXOP_ROT270 as libc::c_int
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-grayscale\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOpt |= crate::turbojpeg::TJXOPT_GRAY
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-custom\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    customFilter = Some(
                        dummyDCTFilter
                            as unsafe extern "C" fn(
                                _: *mut libc::c_short,
                                _: crate::turbojpeg::tjregion,
                                _: crate::turbojpeg::tjregion,
                                _: libc::c_int,
                                _: libc::c_int,
                                _: *mut crate::turbojpeg::tjtransform,
                            ) -> libc::c_int,
                    )
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-nooutput\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOpt |= crate::turbojpeg::TJXOPT_NOOUTPUT
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-copynone\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    xformOpt |= crate::turbojpeg::TJXOPT_COPYNONE
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-benchtime\x00" as *const u8 as *const libc::c_char,
                    )
                    && i < argc - 1i32
                {
                    i += 1;
                    let mut temp_0: libc::c_double = crate::stdlib::atof(*argv.offset(i as isize));
                    if temp_0 > 0.0f64 {
                        benchTime = temp_0
                    } else {
                        usage(*argv.offset(0isize));
                    }
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-warmup\x00" as *const u8 as *const libc::c_char,
                    )
                    && i < argc - 1i32
                {
                    i += 1;
                    let mut temp_1: libc::c_double = crate::stdlib::atof(*argv.offset(i as isize));
                    if temp_1 >= 0.0f64 {
                        warmup = temp_1
                    } else {
                        usage(*argv.offset(0isize));
                    }
                    crate::stdlib::printf(
                        b"Warmup time = %.1f seconds\n\n\x00" as *const u8 as *const libc::c_char,
                        warmup,
                    );
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-alloc\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    flags &= !crate::turbojpeg::TJFLAG_NOREALLOC
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-bmp\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    ext = b"bmp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-yuv\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    crate::stdlib::printf(
                        b"Testing YUV planar encoding/decoding\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    doYUV = 1i32
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-yuvpad\x00" as *const u8 as *const libc::c_char,
                    )
                    && i < argc - 1i32
                {
                    i += 1;
                    let mut temp_2: libc::c_int = crate::stdlib::atoi(*argv.offset(i as isize));
                    if temp_2 >= 1i32 {
                        yuvPad = temp_2
                    }
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-subsamp\x00" as *const u8 as *const libc::c_char,
                    )
                    && i < argc - 1i32
                {
                    i += 1;
                    if ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1i32 as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int =
                                    *(*argv.offset(i as isize)).offset(0isize) as libc::c_int;
                                __res = if __c < -128i32 || __c > 255i32 {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                }
                            } else {
                                __res = crate::stdlib::toupper(
                                    *(*argv.offset(i as isize)).offset(0isize) as libc::c_int,
                                )
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc())
                                .offset(*(*argv.offset(i as isize)).offset(0isize) as libc::c_int
                                    as isize)
                        }
                        __res
                    }) == 'G' as i32
                    {
                        subsamp = crate::turbojpeg::TJSAMP_GRAY as libc::c_int
                    } else {
                        let mut temp_3: libc::c_int = crate::stdlib::atoi(*argv.offset(i as isize));
                        match temp_3 {
                            444 => subsamp = crate::turbojpeg::TJSAMP_444 as libc::c_int,
                            422 => subsamp = crate::turbojpeg::TJSAMP_422 as libc::c_int,
                            440 => subsamp = crate::turbojpeg::TJSAMP_440 as libc::c_int,
                            420 => subsamp = crate::turbojpeg::TJSAMP_420 as libc::c_int,
                            411 => subsamp = crate::turbojpeg::TJSAMP_411 as libc::c_int,
                            _ => {}
                        }
                    }
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-componly\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    compOnly = 1i32
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-nowrite\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    doWrite = 0i32
                } else if 0
                    == crate::stdlib::strcasecmp(
                        *argv.offset(i as isize),
                        b"-stoponwarning\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    flags |= crate::turbojpeg::TJFLAG_STOPONWARNING
                } else {
                    usage(*argv.offset(0isize));
                }
                i += 1
            }
        }
        if (sf.num != 1i32 || sf.denom != 1i32) && 0 != doTile {
            crate::stdlib::printf(
                b"Disabling tiled compression/decompression tests, because those tests do not\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            crate::stdlib::printf(
                b"work when scaled decompression is enabled.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            doTile = 0i32
        }
        if flags & crate::turbojpeg::TJFLAG_NOREALLOC == 0i32 && 0 != doTile {
            crate::stdlib::printf(
                b"Disabling tiled compression/decompression tests, because those tests do not\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            crate::stdlib::printf(
                b"work when dynamic JPEG buffer allocation is enabled.\n\n\x00" as *const u8
                    as *const libc::c_char,
            );
            doTile = 0i32
        }
        if 0 == decompOnly {
            srcBuf = crate::turbojpeg::tjLoadImage(
                *argv.offset(1isize),
                &mut w,
                1i32,
                &mut h,
                &mut pf,
                flags,
            );
            if srcBuf.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    962i32,
                    b"loading bitmap\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg::tjGetErrorStr2(crate::stddef_h::NULL_0 as *mut libc::c_void),
                );
                retval = -1i32;
                current_block = 15940078839392993310;
            } else {
                temp = crate::stdlib::strrchr(*argv.offset(1isize), '.' as i32);
                if !temp.is_null() {
                    *temp = '\u{0}' as i32 as libc::c_char
                }
                current_block = 11359721434352816539;
            }
        } else {
            current_block = 11359721434352816539;
        }
        match current_block {
            15940078839392993310 => {}
            _ => {
                if quiet == 1i32 && 0 == decompOnly {
                    crate::stdlib::printf(
                        b"All performance values in Mpixels/sec\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    crate::stdlib::printf(
                        b"Bitmap     JPEG     JPEG  %s  %s   \x00" as *const u8
                            as *const libc::c_char,
                        if 0 != doTile {
                            b"Tile \x00" as *const u8 as *const libc::c_char
                        } else {
                            b"Image\x00" as *const u8 as *const libc::c_char
                        },
                        if 0 != doTile {
                            b"Tile \x00" as *const u8 as *const libc::c_char
                        } else {
                            b"Image\x00" as *const u8 as *const libc::c_char
                        },
                    );
                    if 0 != doYUV {
                        crate::stdlib::printf(b"Encode  \x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(
                        b"Comp    Comp    Decomp  \x00" as *const u8 as *const libc::c_char,
                    );
                    if 0 != doYUV {
                        crate::stdlib::printf(b"Decode\x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    crate::stdlib::printf(
                        b"Format     Subsamp  Qual  Width  Height  \x00" as *const u8
                            as *const libc::c_char,
                    );
                    if 0 != doYUV {
                        crate::stdlib::printf(b"Perf    \x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(
                        b"Perf    Ratio   Perf    \x00" as *const u8 as *const libc::c_char,
                    );
                    if 0 != doYUV {
                        crate::stdlib::printf(b"Perf\x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
                }
                if 0 != decompOnly {
                    decompTest(*argv.offset(1isize));
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                } else if subsamp >= 0i32 && subsamp < crate::turbojpeg::TJ_NUMSAMP {
                    i = maxQual;
                    while i >= minQual {
                        fullTest(srcBuf, w, h, subsamp, i, *argv.offset(1isize));
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                } else {
                    if pf != crate::turbojpeg::TJPF_CMYK as libc::c_int {
                        i = maxQual;
                        while i >= minQual {
                            fullTest(
                                srcBuf,
                                w,
                                h,
                                crate::turbojpeg::TJSAMP_GRAY as libc::c_int,
                                i,
                                *argv.offset(1isize),
                            );
                            i -= 1
                        }
                        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            crate::turbojpeg::TJSAMP_420 as libc::c_int,
                            i,
                            *argv.offset(1isize),
                        );
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            crate::turbojpeg::TJSAMP_422 as libc::c_int,
                            i,
                            *argv.offset(1isize),
                        );
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            crate::turbojpeg::TJSAMP_444 as libc::c_int,
                            i,
                            *argv.offset(1isize),
                        );
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    if !srcBuf.is_null() {
        crate::turbojpeg::tjFree(srcBuf);
    }
    return retval;
}
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
