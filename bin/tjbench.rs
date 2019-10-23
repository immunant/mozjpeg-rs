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


use std::prelude::v1;use libc::c_double;use libc::c_ulong;use libc::c_void;use libc::c_long;use libc::c_int;use libc::c_short;use libc::c_uchar;use libc::c_char;use mozjpeg::*;


pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stddef_h::NULL_0;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::src::turbojpeg::tjAlloc;
pub use crate::src::turbojpeg::tjBlueOffset;
pub use crate::src::turbojpeg::tjBufSize;
pub use crate::src::turbojpeg::tjBufSizeYUV2;
pub use crate::src::turbojpeg::tjCompress2;
pub use crate::src::turbojpeg::tjCompressFromYUV;
pub use crate::src::turbojpeg::tjDecodeYUV;
pub use crate::src::turbojpeg::tjDecompress2;
pub use crate::src::turbojpeg::tjDecompressHeader3;
pub use crate::src::turbojpeg::tjDecompressToYUV2;
pub use crate::src::turbojpeg::tjDestroy;
pub use crate::src::turbojpeg::tjEncodeYUV3;
pub use crate::src::turbojpeg::tjFree;
pub use crate::src::turbojpeg::tjGetErrorCode;
pub use crate::src::turbojpeg::tjGetErrorStr;
pub use crate::src::turbojpeg::tjGetErrorStr2;
pub use crate::src::turbojpeg::tjGetScalingFactors;
pub use crate::src::turbojpeg::tjGreenOffset;
pub use crate::src::turbojpeg::tjInitCompress;
pub use crate::src::turbojpeg::tjInitDecompress;
pub use crate::src::turbojpeg::tjInitTransform;
pub use crate::src::turbojpeg::tjLoadImage;
pub use crate::src::turbojpeg::tjMCUHeight;
pub use crate::src::turbojpeg::tjMCUWidth;
pub use crate::src::turbojpeg::tjPixelSize;
pub use crate::src::turbojpeg::tjRedOffset;
pub use crate::src::turbojpeg::tjSaveImage;
pub use crate::src::turbojpeg::tjTransform;
pub use crate::src::turbojpeg::tjhandle;
pub use crate::src::turbojpeg::tjregion;
pub use crate::src::turbojpeg::tjscalingfactor;
pub use crate::src::turbojpeg::tjtransform;
pub use crate::src::turbojpeg::TJCS_YCbCr;
pub use crate::src::turbojpeg::TJCS;
pub use crate::src::turbojpeg::TJCS_CMYK;
pub use crate::src::turbojpeg::TJCS_GRAY;
pub use crate::src::turbojpeg::TJCS_RGB;
pub use crate::src::turbojpeg::TJCS_YCCK;
pub use crate::src::turbojpeg::TJERR;
pub use crate::src::turbojpeg::TJERR_FATAL;
pub use crate::src::turbojpeg::TJERR_WARNING;
pub use crate::src::turbojpeg::TJFLAG_ACCURATEDCT;
pub use crate::src::turbojpeg::TJFLAG_BOTTOMUP;
pub use crate::src::turbojpeg::TJFLAG_FASTDCT;
pub use crate::src::turbojpeg::TJFLAG_FASTUPSAMPLE;
pub use crate::src::turbojpeg::TJFLAG_NOREALLOC;
pub use crate::src::turbojpeg::TJFLAG_PROGRESSIVE;
pub use crate::src::turbojpeg::TJFLAG_STOPONWARNING;
pub use crate::src::turbojpeg::TJPF;
pub use crate::src::turbojpeg::TJPF_ABGR;
pub use crate::src::turbojpeg::TJPF_ARGB;
pub use crate::src::turbojpeg::TJPF_BGR;
pub use crate::src::turbojpeg::TJPF_BGRA;
pub use crate::src::turbojpeg::TJPF_BGRX;
pub use crate::src::turbojpeg::TJPF_CMYK;
pub use crate::src::turbojpeg::TJPF_GRAY;
pub use crate::src::turbojpeg::TJPF_RGB;
pub use crate::src::turbojpeg::TJPF_RGBA;
pub use crate::src::turbojpeg::TJPF_RGBX;
pub use crate::src::turbojpeg::TJPF_UNKNOWN;
pub use crate::src::turbojpeg::TJPF_XBGR;
pub use crate::src::turbojpeg::TJPF_XRGB;
pub use crate::src::turbojpeg::TJSAMP;
pub use crate::src::turbojpeg::TJSAMP_411;
pub use crate::src::turbojpeg::TJSAMP_420;
pub use crate::src::turbojpeg::TJSAMP_422;
pub use crate::src::turbojpeg::TJSAMP_440;
pub use crate::src::turbojpeg::TJSAMP_444;
pub use crate::src::turbojpeg::TJSAMP_GRAY;
pub use crate::src::turbojpeg::TJXOP;
pub use crate::src::turbojpeg::TJXOPT_COPYNONE;
pub use crate::src::turbojpeg::TJXOPT_CROP;
pub use crate::src::turbojpeg::TJXOPT_GRAY;
pub use crate::src::turbojpeg::TJXOPT_NOOUTPUT;
pub use crate::src::turbojpeg::TJXOPT_TRIM;
pub use crate::src::turbojpeg::TJXOP_HFLIP;
pub use crate::src::turbojpeg::TJXOP_NONE;
pub use crate::src::turbojpeg::TJXOP_ROT180;
pub use crate::src::turbojpeg::TJXOP_ROT270;
pub use crate::src::turbojpeg::TJXOP_ROT90;
pub use crate::src::turbojpeg::TJXOP_TRANSPOSE;
pub use crate::src::turbojpeg::TJXOP_TRANSVERSE;
pub use crate::src::turbojpeg::TJXOP_VFLIP;
pub use crate::src::turbojpeg::TJ_GRAYSCALE;
pub use crate::src::turbojpeg::TJ_NUMSAMP;
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
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use crate::stdlib::printf;
pub use crate::stdlib::puts;
pub use crate::stdlib::snprintf;
pub use crate::stdlib::sscanf;
use crate::stdlib::strcasecmp;
use crate::stdlib::strchr;
use crate::stdlib::strerror;
use crate::stdlib::strlen;
use crate::stdlib::strncmp;
use crate::stdlib::strncpy;
use crate::stdlib::strrchr;
pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;

pub use crate::jpeglib_h::JMSG_LENGTH_MAX;
use crate::src::tjutil::getTime;
pub use crate::stdlib::__ctype_toupper_loc;
use crate::stdlib::__errno_location;
use crate::stdlib::ceil;
use crate::stdlib::fabs;
use crate::stdlib::log10;
pub use crate::stdlib::toupper;
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

pub static mut tjErrorStr: [c_char; 200] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]

pub static mut tjErrorMsg: [c_char; 200] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]

pub static mut tjErrorLine: c_int = -1i32;
#[no_mangle]

pub static mut tjErrorCode: c_int = -1i32;
#[no_mangle]

pub static mut flags: c_int = TJFLAG_NOREALLOC;
#[no_mangle]

pub static mut compOnly: c_int = 0i32;
#[no_mangle]

pub static mut decompOnly: c_int = 0i32;
#[no_mangle]

pub static mut doYUV: c_int = 0i32;
#[no_mangle]

pub static mut quiet: c_int = 0i32;
#[no_mangle]

pub static mut doTile: c_int = 0i32;
#[no_mangle]

pub static mut pf: c_int =  TJPF_BGR;
#[no_mangle]

pub static mut yuvPad: c_int = 1i32;
#[no_mangle]

pub static mut doWrite: c_int = 1i32;
#[no_mangle]

pub static mut ext: *mut c_char =
    
    
    
    b"ppm\x00".as_ptr() as *mut c_char;
#[no_mangle]

pub static mut pixFormatStr: [*const c_char; 12] = [
    
    b"RGB\x00".as_ptr() as *const c_char,
    
    b"BGR\x00".as_ptr() as *const c_char,
    
    b"RGBX\x00".as_ptr() as *const c_char,
    
    b"BGRX\x00".as_ptr() as *const c_char,
    
    b"XBGR\x00".as_ptr() as *const c_char,
    
    b"XRGB\x00".as_ptr() as *const c_char,
    
    b"GRAY\x00".as_ptr() as *const c_char,
    
    b"\x00".as_ptr() as *const c_char,
    
    b"\x00".as_ptr() as *const c_char,
    
    b"\x00".as_ptr() as *const c_char,
    
    b"\x00".as_ptr() as *const c_char,
    
    b"CMYK\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut subNameLong: [*const c_char; 6] = [
    
    b"4:4:4\x00".as_ptr() as *const c_char,
    
    b"4:2:2\x00".as_ptr() as *const c_char,
    
    b"4:2:0\x00".as_ptr() as *const c_char,
    
    b"GRAY\x00".as_ptr() as *const c_char,
    
    b"4:4:0\x00".as_ptr() as *const c_char,
    
    b"4:1:1\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut csName: [*const c_char; 5] = [
    
    b"RGB\x00".as_ptr() as *const c_char,
    
    b"YCbCr\x00".as_ptr() as *const c_char,
    
    b"GRAY\x00".as_ptr() as *const c_char,
    
    b"CMYK\x00".as_ptr() as *const c_char,
    
    b"YCCK\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut subName: [*const c_char; 6] = [
    
    b"444\x00".as_ptr() as *const c_char,
    
    b"422\x00".as_ptr() as *const c_char,
    
    b"420\x00".as_ptr() as *const c_char,
    
    b"GRAY\x00".as_ptr() as *const c_char,
    
    b"440\x00".as_ptr() as *const c_char,
    
    b"411\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut scalingFactors: *mut tjscalingfactor =
    NULL_0 as *mut tjscalingfactor;
#[no_mangle]

pub static mut sf: tjscalingfactor = {
     let mut init =
     tjscalingfactor{num:  1i32, denom:  1i32,};
    init
};
#[no_mangle]

pub static mut nsf: c_int = 0i32;
#[no_mangle]

pub static mut xformOp: c_int = TJXOP_NONE as c_int;
#[no_mangle]

pub static mut xformOpt: c_int = 0i32;
#[no_mangle]

pub static mut customFilter: Option<
    unsafe extern "C" fn(
        _: *mut c_short,
        _: tjregion,
        _: tjregion,
        _: c_int,
        _: c_int,
        _: *mut tjtransform,
    ) -> c_int,
> = None;
#[no_mangle]

pub static mut benchTime: c_double = 5.0f64;
#[no_mangle]

pub static mut warmup: c_double = 1.0f64;
#[no_mangle]

pub unsafe extern "C" fn formatName(
    mut subsamp: c_int,
    mut cs: c_int,
    mut buf: *mut c_char,
) -> *mut c_char {
    if cs == TJCS_YCbCr as c_int {
        return subNameLong[subsamp as usize] as *mut c_char;
    } else if cs == TJCS_YCCK as c_int
        || cs == TJCS_CMYK as c_int
    {
        snprintf(
            buf,
            80u64,
            
            b"%s %s\x00".as_ptr() as *const c_char,
            csName[cs as usize],
            subNameLong[subsamp as usize],
        );
        return buf;
    } else {
        return csName[cs as usize] as *mut c_char;
    };
}
#[no_mangle]

pub unsafe extern "C" fn sigfig(
    mut val: c_double,
    mut figs: c_int,
    mut buf: *mut c_char,
    mut len: c_int,
) -> *mut c_char {
     let mut format:  [c_char; 80] =  [0; 80];
    let mut digitsAfterDecimal: c_int =
        figs - ceil(log10(fabs(val))) as c_int;
    if digitsAfterDecimal < 1i32 {
        snprintf(
            format.as_mut_ptr(),
            80u64,
            
            b"%%.0f\x00".as_ptr() as *const c_char,
        );
    } else {
        snprintf(
            format.as_mut_ptr(),
            80u64,
            
            b"%%.%df\x00".as_ptr() as *const c_char,
            digitsAfterDecimal,
        );
    }
    snprintf(buf, len as c_ulong, format.as_mut_ptr(), val);
    return buf;
}
/* Custom DCT filter which produces a negative of the image */
#[no_mangle]

pub unsafe extern "C" fn dummyDCTFilter(
    mut coeffs: *mut c_short,
    mut arrayRegion: tjregion,
    mut planeRegion: tjregion,
    mut componentIndex: c_int,
    mut transformIndex: c_int,
    mut transform: *mut tjtransform,
) -> c_int {
     
     let mut i:   c_int =  0i32;
    while i < arrayRegion.w * arrayRegion.h {
        *coeffs.offset(i as isize) = -(*coeffs.offset(i as isize) as c_int) as c_short;
        i += 1
    }
    return 0i32;
}
/* Decompression test */
#[no_mangle]

pub unsafe extern "C" fn decomp(
    mut srcBuf: *mut c_uchar,
    mut jpegBuf: *mut *mut c_uchar,
    mut jpegSize: *mut c_ulong,
    mut dstBuf: *mut c_uchar,
    mut w: c_int,
    mut h: c_int,
    mut subsamp: c_int,
    mut jpegQual: c_int,
    mut fileName: *mut c_char,
    mut tilew: c_int,
    mut tileh: c_int,
) -> c_int {
    
     let mut current_block:  u64; let mut dstBufAlloc:  c_int =  0i32; let mut retval:  c_int =  0i32;
    let mut sizeStr: [c_char; 20] =
        *::std::mem::transmute::<&[u8; 20], &mut [c_char; 20]>(
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        );
    let mut qualStr: [c_char; 6] =
        *::std::mem::transmute::<&[u8; 6], &mut [c_char; 6]>(b"\x00\x00\x00\x00\x00\x00");
    
    let mut file: *mut FILE = NULL_0 as *mut FILE;
    let mut handle: tjhandle = NULL_0 as *mut c_void;
    
    
    
    
    
    
    
    let mut ps: c_int = tjPixelSize[pf as usize];
    let mut scaledw: c_int = (w * sf.num + sf.denom - 1i32) / sf.denom;
    let mut scaledh: c_int = (h * sf.num + sf.denom - 1i32) / sf.denom;
    let mut pitch: c_int = scaledw * ps;
    let mut ntilesw: c_int = (w + tilew - 1i32) / tilew;
    let mut ntilesh: c_int = (h + tileh - 1i32) / tileh;
    
    
    let mut yuvBuf: *mut c_uchar = NULL_0 as *mut c_uchar;
    if jpegQual > 0i32 {
        snprintf(
            qualStr.as_mut_ptr(),
            6u64,
            
            b"_Q%d\x00".as_ptr() as *const c_char,
            jpegQual,
        );
        qualStr[5] = 0i8
    }
    handle = tjInitDecompress();
    if handle.is_null() {
        let mut _tjErrorCode: c_int = tjGetErrorCode(handle);
        let mut _tjErrorStr: *mut c_char = tjGetErrorStr2(handle);
        if flags & TJFLAG_STOPONWARNING == 0
            && _tjErrorCode == TJERR_WARNING as c_int
        {
            if strncmp(
                tjErrorStr.as_mut_ptr(),
                _tjErrorStr,
                JMSG_LENGTH_MAX as c_ulong,
            ) != 0
                || strncmp(
                    tjErrorMsg.as_mut_ptr(),
                    
                    b"executing tjInitDecompress()\x00".as_ptr() as *const c_char,
                    JMSG_LENGTH_MAX as c_ulong,
                ) != 0
                || tjErrorCode != _tjErrorCode
                || tjErrorLine != 160i32
            {
                strncpy(
                    tjErrorStr.as_mut_ptr(),
                    _tjErrorStr,
                    JMSG_LENGTH_MAX as c_ulong,
                );
                strncpy(
                    tjErrorMsg.as_mut_ptr(),
                    
                    b"executing tjInitDecompress()\x00".as_ptr() as *const c_char,
                    JMSG_LENGTH_MAX as c_ulong,
                );
                tjErrorCode = _tjErrorCode;
                tjErrorLine = 160i32;
                printf(
                    
                    b"WARNING in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                    160i32,
                    
                    b"executing tjInitDecompress()\x00".as_ptr() as *const c_char,
                    _tjErrorStr,
                );
            }
            current_block = 11194104282611034094;
        } else {
            printf(
                
                b"%s in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                if _tjErrorCode == TJERR_WARNING as c_int {
                    
                    b"WARNING\x00".as_ptr() as *const c_char
                } else {
                    
                    b"ERROR\x00".as_ptr() as *const c_char
                },
                160i32,
                
                b"executing tjInitDecompress()\x00".as_ptr() as *const c_char,
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
                    malloc((pitch * scaledh) as c_ulong) as *mut c_uchar;
                if dstBuf.is_null() {
                    printf(
                        
                        b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                        164i32,
                        
                        b"allocating destination buffer\x00".as_ptr() as *const c_char,
                        strerror(*__errno_location()),
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
                    /* Set the destination buffer to gray so we know whether the decompressor
                    attempted to write to it */
                    memset(
                        dstBuf as *mut c_void,
                        127i32,
                        (pitch * scaledh) as c_ulong,
                    );
                    if doYUV != 0 {
                        let mut width: c_int = if doTile != 0 { tilew } else { scaledw };
                        let mut height: c_int = if doTile != 0 { tileh } else { scaledh };
                        let mut yuvSize: c_int =
                            tjBufSizeYUV2(width, yuvPad, height, subsamp)
                                as c_int;
                        yuvBuf =
                            malloc(yuvSize as c_ulong) as *mut c_uchar;
                        if yuvBuf.is_null() {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                177i32,
                                
                                b"allocating YUV buffer\x00".as_ptr() as *const c_char,
                                strerror(*__errno_location()),
                            );
                            retval = -1i32;
                            current_block = 126259514807107346;
                        } else {
                            memset(
                                yuvBuf as *mut c_void,
                                127i32,
                                yuvSize as c_ulong,
                            );
                            current_block = 11743904203796629665;
                        }
                    } else {
                        current_block = 11743904203796629665;
                    }
                    match current_block {
                        126259514807107346 => {}
                        _ => {
                            /* Benchmark */
                             let mut row:  c_int =  0; let mut col:  c_int =  0;   
                            
                             let mut iter:   c_int =  -1i32; let mut elapsedDecode:   c_double =  0.0f64; let mut elapsed:   c_double =  elapsedDecode;
                            's_213: loop {
                                 
                                let mut start: c_double = getTime();
                                row = 0i32;
                                 let mut dstPtr:   *mut c_uchar =  dstBuf;
                                while row < ntilesh {
                                     col = 0i32;
                                     let mut dstPtr2:   *mut c_uchar =  dstPtr;
                                    while col < ntilesw {
                                         let mut tile:  c_int =  0i32;let mut width_0: c_int = if doTile != 0 {
                                            if tilew < w - col * tilew {
                                                tilew
                                            } else {
                                                (w) - col * tilew
                                            }
                                        } else {
                                            scaledw
                                        };
                                        let mut height_0: c_int = if doTile != 0 {
                                            if tileh < h - row * tileh {
                                                tileh
                                            } else {
                                                (h) - row * tileh
                                            }
                                        } else {
                                            scaledh
                                        };
                                        if doYUV != 0 {
                                             
                                            if tjDecompressToYUV2(
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
                                                let mut _tjErrorCode_0: c_int =
                                                    tjGetErrorCode(handle);
                                                let mut _tjErrorStr_0: *mut c_char =
                                                    tjGetErrorStr2(handle);
                                                if flags
                                                    & TJFLAG_STOPONWARNING
                                                    == 0
                                                    && _tjErrorCode_0
                                                        == TJERR_WARNING
                                                            as c_int
                                                {
                                                    if strncmp(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_0,
                                                        JMSG_LENGTH_MAX
                                                            as c_ulong,
                                                    ) != 0
                                                        || strncmp(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            
                                                            b"executing tjDecompressToYUV2()\x00".as_ptr()
                                                                as *const c_char,
                                                            JMSG_LENGTH_MAX
                                                                as c_ulong,
                                                        ) != 0
                                                        || tjErrorCode != _tjErrorCode_0
                                                        || tjErrorLine != 200i32
                                                    {
                                                        strncpy(
                                                            tjErrorStr.as_mut_ptr(),
                                                            _tjErrorStr_0,
                                                            JMSG_LENGTH_MAX
                                                                as c_ulong,
                                                        );
                                                        strncpy(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            
                                                            b"executing tjDecompressToYUV2()\x00".as_ptr()
                                                                as *const c_char,
                                                            JMSG_LENGTH_MAX
                                                                as c_ulong,
                                                        );
                                                        tjErrorCode = _tjErrorCode_0;
                                                        tjErrorLine = 200i32;
                                                        printf(b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                                       as
                                                                       *const c_char,
                                                                   200i32,
                                                                   
                                                                   b"executing tjDecompressToYUV2()\x00".as_ptr()
                                                                       as
                                                                       *const c_char,
                                                                   _tjErrorStr_0);
                                                    }
                                                } else {
                                                    printf(
                                                        
                                                        b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                            as *const c_char,
                                                        if _tjErrorCode_0
                                                            == TJERR_WARNING
                                                                as c_int
                                                        {
                                                            
                                                            b"WARNING\x00".as_ptr()
                                                                as *const c_char
                                                        } else {
                                                            
                                                            b"ERROR\x00".as_ptr()
                                                                as *const c_char
                                                        },
                                                        200i32,
                                                        
                                                        b"executing tjDecompressToYUV2()\x00".as_ptr()
                                                            as *const c_char,
                                                        _tjErrorStr_0,
                                                    );
                                                    retval = -1i32;
                                                    current_block = 126259514807107346;
                                                    break 's_213;
                                                }
                                            }
                                             let mut startDecode:   c_double =  getTime();
                                            if tjDecodeYUV(
                                                handle, yuvBuf, yuvPad, subsamp, dstPtr2, width_0,
                                                pitch, height_0, pf, flags,
                                            ) == -1i32
                                            {
                                                let mut _tjErrorCode_1: c_int =
                                                    tjGetErrorCode(handle);
                                                let mut _tjErrorStr_1: *mut c_char =
                                                    tjGetErrorStr2(handle);
                                                if flags
                                                    & TJFLAG_STOPONWARNING
                                                    == 0
                                                    && _tjErrorCode_1
                                                        == TJERR_WARNING
                                                            as c_int
                                                {
                                                    if strncmp(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_1,
                                                        JMSG_LENGTH_MAX
                                                            as c_ulong,
                                                    ) != 0
                                                        || strncmp(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            
                                                            b"executing tjDecodeYUV()\x00".as_ptr()
                                                                as *const c_char,
                                                            JMSG_LENGTH_MAX
                                                                as c_ulong,
                                                        ) != 0
                                                        || tjErrorCode != _tjErrorCode_1
                                                        || tjErrorLine != 204i32
                                                    {
                                                        strncpy(
                                                            tjErrorStr.as_mut_ptr(),
                                                            _tjErrorStr_1,
                                                            JMSG_LENGTH_MAX
                                                                as c_ulong,
                                                        );
                                                        strncpy(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            
                                                            b"executing tjDecodeYUV()\x00".as_ptr()
                                                                as *const c_char,
                                                            JMSG_LENGTH_MAX
                                                                as c_ulong,
                                                        );
                                                        tjErrorCode = _tjErrorCode_1;
                                                        tjErrorLine = 204i32;
                                                        printf(b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                                       as
                                                                       *const c_char,
                                                                   204i32,
                                                                   
                                                                   b"executing tjDecodeYUV()\x00".as_ptr()
                                                                       as
                                                                       *const c_char,
                                                                   _tjErrorStr_1);
                                                    }
                                                } else {
                                                    printf(
                                                        
                                                        b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                            as *const c_char,
                                                        if _tjErrorCode_1
                                                            == TJERR_WARNING
                                                                as c_int
                                                        {
                                                            
                                                            b"WARNING\x00".as_ptr()
                                                                as *const c_char
                                                        } else {
                                                            
                                                            b"ERROR\x00".as_ptr()
                                                                as *const c_char
                                                        },
                                                        204i32,
                                                        
                                                        b"executing tjDecodeYUV()\x00".as_ptr()
                                                            as *const c_char,
                                                        _tjErrorStr_1,
                                                    );
                                                    retval = -1i32;
                                                    current_block = 126259514807107346;
                                                    break 's_213;
                                                }
                                            }
                                            if iter >= 0i32 {
                                                elapsedDecode +=
                                                    getTime() - startDecode
                                            }
                                        } else if tjDecompress2(
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
                                            let mut _tjErrorCode_2: c_int =
                                                tjGetErrorCode(handle);
                                            let mut _tjErrorStr_2: *mut c_char =
                                                tjGetErrorStr2(handle);
                                            if flags & TJFLAG_STOPONWARNING
                                                == 0
                                                && _tjErrorCode_2
                                                    == TJERR_WARNING
                                                        as c_int
                                            {
                                                if strncmp(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_2,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                ) != 0
                                                    || strncmp(
                                                        tjErrorMsg.as_mut_ptr(),
                                                        
                                                        b"executing tjDecompress2()\x00".as_ptr()
                                                            as *const c_char,
                                                        JMSG_LENGTH_MAX
                                                            as c_ulong,
                                                    ) != 0
                                                    || tjErrorCode != _tjErrorCode_2
                                                    || tjErrorLine != 209i32
                                                {
                                                    strncpy(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_2,
                                                        JMSG_LENGTH_MAX
                                                            as c_ulong,
                                                    );
                                                    strncpy(
                                                        tjErrorMsg.as_mut_ptr(),
                                                        
                                                        b"executing tjDecompress2()\x00".as_ptr()
                                                            as *const c_char,
                                                        JMSG_LENGTH_MAX
                                                            as c_ulong,
                                                    );
                                                    tjErrorCode = _tjErrorCode_2;
                                                    tjErrorLine = 209i32;
                                                    printf(
                                                        
                                                        b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                            as *const c_char,
                                                        209i32,
                                                        
                                                        b"executing tjDecompress2()\x00".as_ptr()
                                                            as *const c_char,
                                                        _tjErrorStr_2,
                                                    );
                                                }
                                            } else {
                                                printf(
                                                    
                                                    b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                        as *const c_char,
                                                    if _tjErrorCode_2
                                                        == TJERR_WARNING
                                                            as c_int
                                                    {
                                                        
                                                        b"WARNING\x00".as_ptr()
                                                            as *const c_char
                                                    } else {
                                                        
                                                        b"ERROR\x00".as_ptr()
                                                            as *const c_char
                                                    },
                                                    209i32,
                                                    
                                                    b"executing tjDecompress2()\x00".as_ptr()
                                                        as *const c_char,
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
                                elapsed += getTime() - start;
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
                                    if doYUV != 0 {
                                        elapsed -= elapsedDecode
                                    }
                                    if tjDestroy(handle) == -1i32 {
                                        let mut _tjErrorCode_3: c_int =
                                            tjGetErrorCode(handle);
                                        let mut _tjErrorStr_3: *mut c_char =
                                            tjGetErrorStr2(handle);
                                        if flags & TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_3
                                                == TJERR_WARNING
                                                    as c_int
                                        {
                                            if strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_3,
                                                JMSG_LENGTH_MAX as c_ulong,
                                            ) != 0
                                                || strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    
                                                    b"executing tjDestroy()\x00".as_ptr()
                                                        as *const c_char,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_3
                                                || tjErrorLine != 223i32
                                            {
                                                strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_3,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                );
                                                strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    
                                                    b"executing tjDestroy()\x00".as_ptr()
                                                        as *const c_char,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_3;
                                                tjErrorLine = 223i32;
                                                printf(
                                                    
                                                    b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                        as *const c_char,
                                                    223i32,
                                                    
                                                    b"executing tjDestroy()\x00".as_ptr()
                                                        as *const c_char,
                                                    _tjErrorStr_3,
                                                );
                                            }
                                            current_block = 17917672080766325409;
                                        } else {
                                            printf(
                                                
                                                b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                    as *const c_char,
                                                if _tjErrorCode_3
                                                    == TJERR_WARNING
                                                        as c_int
                                                {
                                                    
                                                    b"WARNING\x00".as_ptr()
                                                        as *const c_char
                                                } else {
                                                    
                                                    b"ERROR\x00".as_ptr() as *const c_char
                                                },
                                                223i32,
                                                
                                                b"executing tjDestroy()\x00".as_ptr()
                                                    as *const c_char,
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
                                             let mut tempStr:  [c_char; 1024] =  [0; 1024];handle = NULL_0 as *mut c_void;
                                            if quiet != 0 {
                                                printf(
                                                    
                                                    b"%-6s%s\x00".as_ptr()
                                                        as *const c_char,
                                                    sigfig(
                                                        (w * h) as c_double / 1000000.0f64
                                                            * iter as c_double
                                                            / elapsed,
                                                        4i32,
                                                        tempStr.as_mut_ptr(),
                                                        1024i32,
                                                    ),
                                                    if quiet == 2i32 {
                                                        
                                                        b"\n\x00".as_ptr()
                                                            as *const c_char
                                                    } else {
                                                        
                                                        b"  \x00".as_ptr()
                                                            as *const c_char
                                                    },
                                                );
                                                if doYUV != 0 {
                                                    printf(
                                                        
                                                        b"%s\n\x00".as_ptr()
                                                            as *const c_char,
                                                        sigfig(
                                                            (w * h) as c_double
                                                                / 1000000.0f64
                                                                * iter as c_double
                                                                / elapsedDecode,
                                                            4i32,
                                                            tempStr.as_mut_ptr(),
                                                            1024i32,
                                                        ),
                                                    );
                                                } else if quiet != 2i32 {
                                                    printf(
                                                        
                                                        b"\n\x00".as_ptr()
                                                            as *const c_char,
                                                    );
                                                }
                                            } else {
                                                printf(
                                                    
                                                    b"%s --> Frame rate:         %f fps\n\x00".as_ptr()
                                                        as *const c_char,
                                                    if doYUV != 0 {
                                                        
                                                        b"Decomp to YUV\x00".as_ptr()
                                                            as *const c_char
                                                    } else {
                                                        
                                                        b"Decompress   \x00".as_ptr()
                                                            as *const c_char
                                                    },
                                                    iter as c_double / elapsed,
                                                );
                                                printf(b"                  Throughput:         %f Megapixels/sec\n\x00".as_ptr() as
                                                           *const c_char,
                                                       (w * h) as
                                                           c_double /
                                                           1000000.0f64 *
                                                           iter as
                                                               c_double
                                                           / elapsed);
                                                if doYUV != 0 {
                                                    printf(b"YUV Decode    --> Frame rate:         %f fps\n\x00".as_ptr() as
                                                               *const c_char,
                                                           iter as
                                                               c_double
                                                               /
                                                               elapsedDecode);
                                                    printf(b"                  Throughput:         %f Megapixels/sec\n\x00".as_ptr() as
                                                               *const c_char,
                                                           (w * h) as
                                                               c_double
                                                               / 1000000.0f64
                                                               *
                                                               iter as
                                                                   c_double
                                                               /
                                                               elapsedDecode);
                                                }
                                            }
                                            if !(doWrite == 0) {
                                                if sf.num != 1i32 || sf.denom != 1i32 {
                                                    snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20u64,
                                                        
                                                        b"%d_%d\x00".as_ptr()
                                                            as *const c_char,
                                                        sf.num,
                                                        sf.denom,
                                                    );
                                                } else if tilew != w || tileh != h {
                                                    snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20u64,
                                                        
                                                        b"%dx%d\x00".as_ptr()
                                                            as *const c_char,
                                                        tilew,
                                                        tileh,
                                                    );
                                                } else {
                                                    snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20u64,
                                                        
                                                        b"full\x00".as_ptr()
                                                            as *const c_char,
                                                    );
                                                }
                                                if decompOnly != 0 {
                                                    snprintf(
                                                        tempStr.as_mut_ptr(),
                                                        1024u64,
                                                        
                                                        b"%s_%s.%s\x00".as_ptr()
                                                            as *const c_char,
                                                        fileName,
                                                        sizeStr.as_mut_ptr(),
                                                        ext,
                                                    );
                                                } else {
                                                    snprintf(
                                                        tempStr.as_mut_ptr(),
                                                        1024u64,
                                                        
                                                        b"%s_%s%s_%s.%s\x00".as_ptr()
                                                            as *const c_char,
                                                        fileName,
                                                        subName[subsamp as usize],
                                                        qualStr.as_mut_ptr(),
                                                        sizeStr.as_mut_ptr(),
                                                        ext,
                                                    );
                                                }
                                                if tjSaveImage(
                                                    tempStr.as_mut_ptr(),
                                                    dstBuf,
                                                    scaledw,
                                                    0i32,
                                                    scaledh,
                                                    pf,
                                                    flags,
                                                ) == -1i32
                                                {
                                                    printf(
                                                        
                                                        b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                            as *const c_char,
                                                        263i32,
                                                        
                                                        b"saving bitmap\x00".as_ptr()
                                                            as *const c_char,
                                                        tjGetErrorStr2(
                                                            NULL_0
                                                                as *mut c_void,
                                                        ),
                                                    );
                                                    retval = -1i32
                                                } else {
                                                      let mut ptr:   *mut c_char =
     strrchr(
                                                        tempStr.as_mut_ptr(),
                                                        '.' as i32,
                                                    );
                                                    snprintf(
                                                        ptr,
                                                        (1024i64
                                                            - ptr.wrapping_offset_from(
                                                                tempStr.as_mut_ptr(),
                                                            )
                                                                as c_long)
                                                            as c_ulong,
                                                        
                                                        b"-err.%s\x00".as_ptr()
                                                            as *const c_char,
                                                        ext,
                                                    );
                                                    if !srcBuf.is_null()
                                                        && sf.num == 1i32
                                                        && sf.denom == 1i32
                                                    {
                                                        if quiet == 0 {
                                                            printf(b"Compression error written to %s.\n\x00".as_ptr()
                                                                       as
                                                                       *const c_char,
                                                                   tempStr.as_mut_ptr());
                                                        }
                                                        if subsamp
                                                            == TJ_GRAYSCALE
                                                        {
                                                            
                                                             
                                                            row = 0i32;
                                                             let mut index:   c_int =  0i32;
                                                            while row < h {
                                                                 col = 0i32;
                                                                 let mut index2:   c_int =  index;
                                                                while col < w {
                                                                    let mut rindex:
                                                                            c_int =
                                                                        index2
                                                                            +
                                                                            tjRedOffset[pf
                                                                                            as
                                                                                            usize];
                                                                    let mut gindex:
                                                                            c_int =
                                                                        index2
                                                                            +
                                                                            tjGreenOffset[pf
                                                                                              as
                                                                                              usize];
                                                                    let mut bindex:
                                                                            c_int =
                                                                        index2
                                                                            +
                                                                            tjBlueOffset[pf
                                                                                             as
                                                                                             usize];
                                                                    let mut y: c_int =
                                                                        (*srcBuf
                                                                            .offset(rindex as isize)
                                                                            as c_double
                                                                            * 0.299f64
                                                                            + *srcBuf.offset(
                                                                                gindex as isize,
                                                                            )
                                                                                as c_double
                                                                                * 0.587f64
                                                                            + *srcBuf.offset(
                                                                                bindex as isize,
                                                                            )
                                                                                as c_double
                                                                                * 0.114f64
                                                                            + 0.5f64)
                                                                            as c_int;
                                                                    if y > 255i32 {
                                                                        y = 255i32
                                                                    }
                                                                    if y < 0i32 {
                                                                        y = 0i32
                                                                    }
                                                                    *dstBuf
                                                                        .offset(rindex as isize) =
                                                                        abs(
                                                                            *dstBuf.offset(
                                                                                rindex as isize,
                                                                            )
                                                                                as c_int
                                                                                - y,
                                                                        )
                                                                            as c_uchar;
                                                                    *dstBuf
                                                                        .offset(gindex as isize) =
                                                                        abs(
                                                                            *dstBuf.offset(
                                                                                gindex as isize,
                                                                            )
                                                                                as c_int
                                                                                - y,
                                                                        )
                                                                            as c_uchar;
                                                                    *dstBuf
                                                                        .offset(bindex as isize) =
                                                                        abs(
                                                                            *dstBuf.offset(
                                                                                bindex as isize,
                                                                            )
                                                                                as c_int
                                                                                - y,
                                                                        )
                                                                            as c_uchar;
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
                                                                    ) = abs(
                                                                        *dstBuf.offset(
                                                                            (pitch * row + col)
                                                                                as isize,
                                                                        )
                                                                            as c_int
                                                                            - *srcBuf.offset(
                                                                                (pitch * row + col)
                                                                                    as isize,
                                                                            )
                                                                                as c_int,
                                                                    )
                                                                        as c_uchar;
                                                                    col += 1
                                                                }
                                                                row += 1
                                                            }
                                                        }
                                                        if tjSaveImage(
                                                            tempStr.as_mut_ptr(),
                                                            dstBuf,
                                                            w,
                                                            0i32,
                                                            h,
                                                            pf,
                                                            flags,
                                                        ) == -1i32
                                                        {
                                                            printf(b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                                       as
                                                                       *const c_char,
                                                                   294i32,
                                                                   
                                                                   b"saving bitmap\x00".as_ptr()
                                                                       as
                                                                       *const c_char,
                                                                   tjGetErrorStr2(NULL_0
                                                                                      as
                                                                                      *mut c_void));
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
        fclose(file);
    }
    if !handle.is_null() {
        tjDestroy(handle);
    }
    if !dstBuf.is_null() && dstBufAlloc != 0 {
        free(dstBuf as *mut c_void);
    }
    if !yuvBuf.is_null() {
        free(yuvBuf as *mut c_void);
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn fullTest(
    mut srcBuf: *mut c_uchar,
    mut w: c_int,
    mut h: c_int,
    mut subsamp: c_int,
    mut jpegQual: c_int,
    mut fileName: *mut c_char,
) -> c_int {
    
     let mut i:  c_int =  0; let mut retval:  c_int =  0i32; let mut ntilesw:  c_int =  1i32; let mut ntilesh:  c_int =  1i32;
    let mut file: *mut FILE = NULL_0 as *mut FILE;
    let mut handle: tjhandle = NULL_0 as *mut c_void;
    let mut jpegBuf: *mut *mut c_uchar = NULL_0 as *mut *mut c_uchar;
    let mut yuvBuf: *mut c_uchar = NULL_0 as *mut c_uchar;
    let mut tmpBuf: *mut c_uchar = NULL_0 as *mut c_uchar;
    
    
    
    
    
    
    
    
    
    let mut tilew: c_int = w;
    let mut tileh: c_int = h;
    
    
    
    let mut jpegSize: *mut c_ulong = NULL_0 as *mut c_ulong;
    let mut ps: c_int = tjPixelSize[pf as usize];
    
    
    let mut pitch: c_int = w * ps;
    let mut pfStr: *const c_char = pixFormatStr[pf as usize];
    tmpBuf = malloc((pitch * h) as c_ulong) as *mut c_uchar;
    if tmpBuf.is_null() {
        printf(
            
            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
            323i32,
            
            b"allocating temporary image buffer\x00".as_ptr() as *const c_char,
            strerror(*__errno_location()),
        );
        retval = -1i32
    } else {
        if quiet == 0 {
            printf(
                
                b">>>>>  %s (%s) <--> JPEG %s Q%d  <<<<<\n\x00".as_ptr() as *const c_char,
                pfStr,
                if flags & TJFLAG_BOTTOMUP != 0 {
                    
                    b"Bottom-up\x00".as_ptr() as *const c_char
                } else {
                    
                    b"Top-down\x00".as_ptr() as *const c_char
                },
                subNameLong[subsamp as usize],
                jpegQual,
            );
        }
        tilew = (if doTile != 0 { 8i32 } else { w });
        tileh = (if doTile != 0 { 8i32 } else { h });
        's_73: loop {
            if tilew > w {
                tilew = w
            }
            if tileh > h {
                tileh = h
            }
            ntilesw = (w + tilew - 1i32) / tilew;
            ntilesh = (h + tileh - 1i32) / tileh;
            jpegBuf = malloc(
                ::std::mem::size_of::<*mut c_uchar>() as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
            ) as *mut *mut c_uchar;
            if jpegBuf.is_null() {
                printf(
                    
                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                    339i32,
                    
                    b"allocating JPEG tile array\x00".as_ptr() as *const c_char,
                    strerror(*__errno_location()),
                );
                retval = -1i32;
                break;
            } else {
                memset(
                    jpegBuf as *mut c_void,
                    0i32,
                    ::std::mem::size_of::<*mut c_uchar>() as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                );
                jpegSize = malloc(
                    ::std::mem::size_of::<c_ulong>() as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                ) as *mut c_ulong;
                if jpegSize.is_null() {
                    printf(
                        
                        b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                        343i32,
                        
                        b"allocating JPEG size array\x00".as_ptr() as *const c_char,
                        strerror(*__errno_location()),
                    );
                    retval = -1i32;
                    break;
                } else {
                     let mut tempStr:  [c_char; 1024] =  [0; 1024];   let mut totalJpegSize:  c_int =  0i32;  let mut yuvSize:  c_int =  0i32;memset(
                        jpegSize as *mut c_void,
                        0i32,
                        ::std::mem::size_of::<c_ulong>() as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                    );
                    if flags & TJFLAG_NOREALLOC != 0i32 {
                        i = 0i32;
                        while i < ntilesw * ntilesh {
                            let ref mut fresh0 = *jpegBuf.offset(i as isize);
                            *fresh0 = tjAlloc(
                                tjBufSize(tilew, tileh, subsamp)
                                    as c_int,
                            );
                            if (*fresh0).is_null() {
                                printf(
                                    
                                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    350i32,
                                    
                                    b"allocating JPEG tiles\x00".as_ptr()
                                        as *const c_char,
                                    strerror(*__errno_location()),
                                );
                                retval = -1i32;
                                break 's_73;
                            } else {
                                i += 1
                            }
                        }
                    }
                    /* Compression test */
                    if quiet == 1i32 {
                        printf(
                            
                            b"%-4s (%s)  %-5s    %-3d   \x00".as_ptr() as *const c_char,
                            pfStr,
                            if flags & TJFLAG_BOTTOMUP != 0 {
                                
                                b"BU\x00".as_ptr() as *const c_char
                            } else {
                                
                                b"TD\x00".as_ptr() as *const c_char
                            },
                            subNameLong[subsamp as usize],
                            jpegQual,
                        );
                    }
                    i = 0i32;
                    while i < h {
                        memcpy(
                            &mut *tmpBuf.offset((pitch * i) as isize) as *mut c_uchar
                                as *mut c_void,
                            &mut *srcBuf.offset((w * ps * i) as isize) as *mut c_uchar
                                as *const c_void,
                            (w * ps) as c_ulong,
                        );
                        i += 1
                    }
                    handle = tjInitCompress();
                    if handle.is_null() {
                        let mut _tjErrorCode: c_int =
                            tjGetErrorCode(handle);
                        let mut _tjErrorStr: *mut c_char =
                            tjGetErrorStr2(handle);
                        if flags & TJFLAG_STOPONWARNING == 0
                            && _tjErrorCode == TJERR_WARNING as c_int
                        {
                            if strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr,
                                JMSG_LENGTH_MAX as c_ulong,
                            ) != 0
                                || strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    
                                    b"executing tjInitCompress()\x00".as_ptr()
                                        as *const c_char,
                                    JMSG_LENGTH_MAX as c_ulong,
                                ) != 0
                                || tjErrorCode != _tjErrorCode
                                || tjErrorLine != 361i32
                            {
                                strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr,
                                    JMSG_LENGTH_MAX as c_ulong,
                                );
                                strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    
                                    b"executing tjInitCompress()\x00".as_ptr()
                                        as *const c_char,
                                    JMSG_LENGTH_MAX as c_ulong,
                                );
                                tjErrorCode = _tjErrorCode;
                                tjErrorLine = 361i32;
                                printf(
                                    
                                    b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    361i32,
                                    
                                    b"executing tjInitCompress()\x00".as_ptr()
                                        as *const c_char,
                                    _tjErrorStr,
                                );
                            }
                        } else {
                            printf(
                                
                                b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                if _tjErrorCode
                                    == TJERR_WARNING as c_int
                                {
                                    
                                    b"WARNING\x00".as_ptr() as *const c_char
                                } else {
                                    
                                    b"ERROR\x00".as_ptr() as *const c_char
                                },
                                361i32,
                                
                                b"executing tjInitCompress()\x00".as_ptr()
                                    as *const c_char,
                                _tjErrorStr,
                            );
                            retval = -1i32;
                            break;
                        }
                    }
                    if doYUV != 0 {
                        yuvSize =
                            tjBufSizeYUV2(tilew, yuvPad, tileh, subsamp)
                                as c_int;
                        yuvBuf =
                            malloc(yuvSize as c_ulong) as *mut c_uchar;
                        if yuvBuf.is_null() {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                366i32,
                                
                                b"allocating YUV buffer\x00".as_ptr() as *const c_char,
                                strerror(*__errno_location()),
                            );
                            retval = -1i32;
                            break;
                        } else {
                            memset(
                                yuvBuf as *mut c_void,
                                127i32,
                                yuvSize as c_ulong,
                            );
                        }
                    }
                    
                    
                     let mut iter:   c_int =  -1i32; let mut elapsedEncode:   c_double =  0.0f64; let mut elapsed:   c_double =  elapsedEncode;
                    loop {
                           
                        totalJpegSize = 0i32;
                        
                        
                         let mut start:   c_double =  getTime(); let mut row:   c_int =  0i32; let mut srcPtr:   *mut c_uchar =  srcBuf;
                        while row < ntilesh {
                              
                             let mut col:   c_int =  0i32; let mut srcPtr2:   *mut c_uchar =  srcPtr;
                            while col < ntilesw {
                                 let mut tile:  c_int =  0i32;let mut width: c_int = if tilew < w - col * tilew {
                                    tilew
                                } else {
                                    (w) - col * tilew
                                };
                                let mut height: c_int = if tileh < h - row * tileh {
                                    tileh
                                } else {
                                    (h) - row * tileh
                                };
                                if doYUV != 0 {
                                    let mut startEncode: c_double =
                                        getTime();
                                    if tjEncodeYUV3(
                                        handle, srcPtr2, width, pitch, height, pf, yuvBuf, yuvPad,
                                        subsamp, flags,
                                    ) == -1i32
                                    {
                                        let mut _tjErrorCode_0: c_int =
                                            tjGetErrorCode(handle);
                                        let mut _tjErrorStr_0: *mut c_char =
                                            tjGetErrorStr2(handle);
                                        if flags & TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_0
                                                == TJERR_WARNING
                                                    as c_int
                                        {
                                            if strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_0,
                                                JMSG_LENGTH_MAX as c_ulong,
                                            ) != 0
                                                || strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    
                                                    b"executing tjEncodeYUV3()\x00".as_ptr()
                                                        as *const c_char,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_0
                                                || tjErrorLine != 390i32
                                            {
                                                strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_0,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                );
                                                strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    
                                                    b"executing tjEncodeYUV3()\x00".as_ptr()
                                                        as *const c_char,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_0;
                                                tjErrorLine = 390i32;
                                                printf(
                                                    
                                                    b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                        as *const c_char,
                                                    390i32,
                                                    
                                                    b"executing tjEncodeYUV3()\x00".as_ptr()
                                                        as *const c_char,
                                                    _tjErrorStr_0,
                                                );
                                            }
                                        } else {
                                            printf(
                                                
                                                b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                    as *const c_char,
                                                if _tjErrorCode_0
                                                    == TJERR_WARNING
                                                        as c_int
                                                {
                                                    
                                                    b"WARNING\x00".as_ptr()
                                                        as *const c_char
                                                } else {
                                                    
                                                    b"ERROR\x00".as_ptr() as *const c_char
                                                },
                                                390i32,
                                                
                                                b"executing tjEncodeYUV3()\x00".as_ptr()
                                                    as *const c_char,
                                                _tjErrorStr_0,
                                            );
                                            retval = -1i32;
                                            break 's_73;
                                        }
                                    }
                                    if iter >= 0i32 {
                                        elapsedEncode += getTime() - startEncode
                                    }
                                    if tjCompressFromYUV(
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
                                        let mut _tjErrorCode_1: c_int =
                                            tjGetErrorCode(handle);
                                        let mut _tjErrorStr_1: *mut c_char =
                                            tjGetErrorStr2(handle);
                                        if flags & TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_1
                                                == TJERR_WARNING
                                                    as c_int
                                        {
                                            if strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_1,
                                                JMSG_LENGTH_MAX as c_ulong,
                                            ) != 0
                                                || strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    
                                                    b"executing tjCompressFromYUV()\x00".as_ptr()
                                                        as *const c_char,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_1
                                                || tjErrorLine != 395i32
                                            {
                                                strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_1,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                );
                                                strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    
                                                    b"executing tjCompressFromYUV()\x00".as_ptr()
                                                        as *const c_char,
                                                    JMSG_LENGTH_MAX
                                                        as c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_1;
                                                tjErrorLine = 395i32;
                                                printf(
                                                    
                                                    b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                        as *const c_char,
                                                    395i32,
                                                    
                                                    b"executing tjCompressFromYUV()\x00".as_ptr()
                                                        as *const c_char,
                                                    _tjErrorStr_1,
                                                );
                                            }
                                        } else {
                                            printf(
                                                
                                                b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                    as *const c_char,
                                                if _tjErrorCode_1
                                                    == TJERR_WARNING
                                                        as c_int
                                                {
                                                    
                                                    b"WARNING\x00".as_ptr()
                                                        as *const c_char
                                                } else {
                                                    
                                                    b"ERROR\x00".as_ptr() as *const c_char
                                                },
                                                395i32,
                                                
                                                b"executing tjCompressFromYUV()\x00".as_ptr()
                                                    as *const c_char,
                                                _tjErrorStr_1,
                                            );
                                            retval = -1i32;
                                            break 's_73;
                                        }
                                    }
                                } else if tjCompress2(
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
                                    let mut _tjErrorCode_2: c_int =
                                        tjGetErrorCode(handle);
                                    let mut _tjErrorStr_2: *mut c_char =
                                        tjGetErrorStr2(handle);
                                    if flags & TJFLAG_STOPONWARNING == 0
                                        && _tjErrorCode_2
                                            == TJERR_WARNING as c_int
                                    {
                                        if strncmp(
                                            tjErrorStr.as_mut_ptr(),
                                            _tjErrorStr_2,
                                            JMSG_LENGTH_MAX as c_ulong,
                                        ) != 0
                                            || strncmp(
                                                tjErrorMsg.as_mut_ptr(),
                                                
                                                b"executing tjCompress2()\x00".as_ptr()
                                                    as *const c_char,
                                                JMSG_LENGTH_MAX as c_ulong,
                                            ) != 0
                                            || tjErrorCode != _tjErrorCode_2
                                            || tjErrorLine != 400i32
                                        {
                                            strncpy(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_2,
                                                JMSG_LENGTH_MAX as c_ulong,
                                            );
                                            strncpy(
                                                tjErrorMsg.as_mut_ptr(),
                                                
                                                b"executing tjCompress2()\x00".as_ptr()
                                                    as *const c_char,
                                                JMSG_LENGTH_MAX as c_ulong,
                                            );
                                            tjErrorCode = _tjErrorCode_2;
                                            tjErrorLine = 400i32;
                                            printf(
                                                
                                                b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                    as *const c_char,
                                                400i32,
                                                
                                                b"executing tjCompress2()\x00".as_ptr()
                                                    as *const c_char,
                                                _tjErrorStr_2,
                                            );
                                        }
                                    } else {
                                        printf(
                                            
                                            b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                as *const c_char,
                                            if _tjErrorCode_2
                                                == TJERR_WARNING
                                                    as c_int
                                            {
                                                
                                                b"WARNING\x00".as_ptr() as *const c_char
                                            } else {
                                                
                                                b"ERROR\x00".as_ptr() as *const c_char
                                            },
                                            400i32,
                                            
                                            b"executing tjCompress2()\x00".as_ptr()
                                                as *const c_char,
                                            _tjErrorStr_2,
                                        );
                                        retval = -1i32;
                                        break 's_73;
                                    }
                                }
                                totalJpegSize = (((totalJpegSize as c_ulong + *jpegSize.offset(tile as isize))))
                                    as c_int;
                                col += 1;
                                tile += 1;
                                srcPtr2 = srcPtr2.offset((ps * tilew) as isize)
                            }
                            row += 1;
                            srcPtr = srcPtr.offset((pitch * tileh) as isize)
                        }
                        elapsed += getTime() - start;
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
                    if doYUV != 0 {
                        elapsed -= elapsedEncode
                    }
                    if tjDestroy(handle) == -1i32 {
                        let mut _tjErrorCode_3: c_int =
                            tjGetErrorCode(handle);
                        let mut _tjErrorStr_3: *mut c_char =
                            tjGetErrorStr2(handle);
                        if flags & TJFLAG_STOPONWARNING == 0
                            && _tjErrorCode_3 == TJERR_WARNING as c_int
                        {
                            if strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr_3,
                                JMSG_LENGTH_MAX as c_ulong,
                            ) != 0
                                || strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    
                                    b"executing tjDestroy()\x00".as_ptr()
                                        as *const c_char,
                                    JMSG_LENGTH_MAX as c_ulong,
                                ) != 0
                                || tjErrorCode != _tjErrorCode_3
                                || tjErrorLine != 416i32
                            {
                                strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr_3,
                                    JMSG_LENGTH_MAX as c_ulong,
                                );
                                strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    
                                    b"executing tjDestroy()\x00".as_ptr()
                                        as *const c_char,
                                    JMSG_LENGTH_MAX as c_ulong,
                                );
                                tjErrorCode = _tjErrorCode_3;
                                tjErrorLine = 416i32;
                                printf(
                                    
                                    b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    416i32,
                                    
                                    b"executing tjDestroy()\x00".as_ptr()
                                        as *const c_char,
                                    _tjErrorStr_3,
                                );
                            }
                        } else {
                            printf(
                                
                                b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                if _tjErrorCode_3
                                    == TJERR_WARNING as c_int
                                {
                                    
                                    b"WARNING\x00".as_ptr() as *const c_char
                                } else {
                                    
                                    b"ERROR\x00".as_ptr() as *const c_char
                                },
                                416i32,
                                
                                b"executing tjDestroy()\x00".as_ptr() as *const c_char,
                                _tjErrorStr_3,
                            );
                            retval = -1i32;
                            break;
                        }
                    }
                    handle = NULL_0 as *mut c_void;
                    if quiet == 1i32 {
                        printf(
                            
                            b"%-5d  %-5d   \x00".as_ptr() as *const c_char,
                            tilew,
                            tileh,
                        );
                    }
                    if quiet != 0 {
                         let mut tempStr2:  [c_char; 80] =  [0; 80];if doYUV != 0 {
                            printf(
                                
                                b"%-6s%s\x00".as_ptr() as *const c_char,
                                sigfig(
                                    (w * h) as c_double / 1000000.0f64
                                        * iter as c_double
                                        / elapsedEncode,
                                    4i32,
                                    tempStr.as_mut_ptr(),
                                    1024i32,
                                ),
                                if quiet == 2i32 {
                                    
                                    b"\n\x00".as_ptr() as *const c_char
                                } else {
                                    
                                    b"  \x00".as_ptr() as *const c_char
                                },
                            );
                        }
                        printf(
                            
                            b"%-6s%s\x00".as_ptr() as *const c_char,
                            sigfig(
                                (w * h) as c_double / 1000000.0f64 * iter as c_double
                                    / elapsed,
                                4i32,
                                tempStr.as_mut_ptr(),
                                1024i32,
                            ),
                            if quiet == 2i32 {
                                
                                b"\n\x00".as_ptr() as *const c_char
                            } else {
                                
                                b"  \x00".as_ptr() as *const c_char
                            },
                        );
                        printf(
                            
                            b"%-6s%s\x00".as_ptr() as *const c_char,
                            sigfig(
                                (w * h * ps) as c_double / totalJpegSize as c_double,
                                4i32,
                                tempStr2.as_mut_ptr(),
                                80i32,
                            ),
                            if quiet == 2i32 {
                                
                                b"\n\x00".as_ptr() as *const c_char
                            } else {
                                
                                b"  \x00".as_ptr() as *const c_char
                            },
                        );
                    } else {
                        printf(
                            
                            b"\n%s size: %d x %d\n\x00".as_ptr() as *const c_char,
                            if doTile != 0 {
                                
                                b"Tile\x00".as_ptr() as *const c_char
                            } else {
                                
                                b"Image\x00".as_ptr() as *const c_char
                            },
                            tilew,
                            tileh,
                        );
                        if doYUV != 0 {
                            printf(
                                
                                b"Encode YUV    --> Frame rate:         %f fps\n\x00".as_ptr()
                                    as *const c_char,
                                iter as c_double / elapsedEncode,
                            );
                            printf(
                                
                                b"                  Output image size:  %d bytes\n\x00".as_ptr()
                                    as *const c_char,
                                yuvSize,
                            );
                            printf(
                                
                                b"                  Compression ratio:  %f:1\n\x00".as_ptr()
                                    as *const c_char,
                                (w * h * ps) as c_double / yuvSize as c_double,
                            );
                            printf(
                                
                                b"                  Throughput:         %f Megapixels/sec\n\x00".as_ptr()
                                    as *const c_char,
                                (w * h) as c_double / 1000000.0f64 * iter as c_double
                                    / elapsedEncode,
                            );
                            printf(
                                
                                b"                  Output bit stream:  %f Megabits/sec\n\x00".as_ptr()
                                    as *const c_char,
                                yuvSize as c_double * 8.0f64 / 1000000.0f64
                                    * iter as c_double
                                    / elapsedEncode,
                            );
                        }
                        printf(
                            
                            b"%s --> Frame rate:         %f fps\n\x00".as_ptr()
                                as *const c_char,
                            if doYUV != 0 {
                                
                                b"Comp from YUV\x00".as_ptr() as *const c_char
                            } else {
                                
                                b"Compress     \x00".as_ptr() as *const c_char
                            },
                            iter as c_double / elapsed,
                        );
                        printf(
                            
                            b"                  Output image size:  %d bytes\n\x00".as_ptr()
                                as *const c_char,
                            totalJpegSize,
                        );
                        printf(
                            
                            b"                  Compression ratio:  %f:1\n\x00".as_ptr()
                                as *const c_char,
                            (w * h * ps) as c_double / totalJpegSize as c_double,
                        );
                        printf(
                            
                            b"                  Throughput:         %f Megapixels/sec\n\x00".as_ptr() as *const c_char,
                            (w * h) as c_double / 1000000.0f64 * iter as c_double
                                / elapsed,
                        );
                        printf(
                            
                            b"                  Output bit stream:  %f Megabits/sec\n\x00".as_ptr() as *const c_char,
                            totalJpegSize as c_double * 8.0f64 / 1000000.0f64
                                * iter as c_double
                                / elapsed,
                        );
                    }
                    if tilew == w && tileh == h && doWrite != 0 {
                        snprintf(
                            tempStr.as_mut_ptr(),
                            1024u64,
                            
                            b"%s_%s_Q%d.jpg\x00".as_ptr() as *const c_char,
                            fileName,
                            subName[subsamp as usize],
                            jpegQual,
                        );
                        file = fopen(
                            tempStr.as_mut_ptr(),
                            
                            b"wb\x00".as_ptr() as *const c_char,
                        );
                        if file.is_null() {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                463i32,
                                
                                b"opening reference image\x00".as_ptr() as *const c_char,
                                strerror(*__errno_location()),
                            );
                            retval = -1i32;
                            break;
                        } else if fwrite(
                            *jpegBuf.offset(0) as *const c_void,
                            *jpegSize.offset(0),
                            1u64,
                            file,
                        ) != 1u64
                        {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                465i32,
                                
                                b"writing reference image\x00".as_ptr() as *const c_char,
                                strerror(*__errno_location()),
                            );
                            retval = -1i32;
                            break;
                        } else {
                            fclose(file);
                            file = NULL_0 as *mut FILE;
                            if quiet == 0 {
                                printf(
                                    
                                    b"Reference image written to %s\n\x00".as_ptr()
                                        as *const c_char,
                                    tempStr.as_mut_ptr(),
                                );
                            }
                        }
                    }
                    /* Decompression test */
                    if compOnly == 0 {
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
                            tjFree(*jpegBuf.offset(i as isize));
                        }
                        let ref mut fresh1 = *jpegBuf.offset(i as isize);
                        *fresh1 = NULL_0 as *mut c_uchar;
                        i += 1
                    }
                    free(jpegBuf as *mut c_void);
                    jpegBuf = NULL_0 as *mut *mut c_uchar;
                    free(jpegSize as *mut c_void);
                    jpegSize = NULL_0 as *mut c_ulong;
                    if doYUV != 0 {
                        free(yuvBuf as *mut c_void);
                        yuvBuf = NULL_0 as *mut c_uchar
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
        fclose(file);
        file = NULL_0 as *mut FILE
    }
    if !jpegBuf.is_null() {
        i = 0i32;
        while i < ntilesw * ntilesh {
            if !(*jpegBuf.offset(i as isize)).is_null() {
                tjFree(*jpegBuf.offset(i as isize));
            }
            let ref mut fresh2 = *jpegBuf.offset(i as isize);
            *fresh2 = NULL_0 as *mut c_uchar;
            i += 1
        }
        free(jpegBuf as *mut c_void);
        jpegBuf = NULL_0 as *mut *mut c_uchar
    }
    if !yuvBuf.is_null() {
        free(yuvBuf as *mut c_void);
        yuvBuf = NULL_0 as *mut c_uchar
    }
    if !jpegSize.is_null() {
        free(jpegSize as *mut c_void);
        jpegSize = NULL_0 as *mut c_ulong
    }
    if !tmpBuf.is_null() {
        free(tmpBuf as *mut c_void);
        tmpBuf = NULL_0 as *mut c_uchar
    }
    if !handle.is_null() {
        tjDestroy(handle);
        handle = NULL_0 as *mut c_void
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn decompTest(mut fileName: *mut c_char) -> c_int {
     let mut srcSize:  c_ulong =  0; let mut i:  c_int =  0; let mut retval:  c_int =  0i32; let mut ntilesw:  c_int =  1i32; let mut ntilesh:  c_int =  1i32;
    let mut file: *mut FILE = NULL_0 as *mut FILE;
    let mut handle: tjhandle = NULL_0 as *mut c_void;
    let mut jpegBuf: *mut *mut c_uchar = NULL_0 as *mut *mut c_uchar;
    let mut srcBuf: *mut c_uchar = NULL_0 as *mut c_uchar;
    let mut jpegSize: *mut c_ulong = NULL_0 as *mut c_ulong;
    
    
    let mut t: *mut tjtransform =
        NULL_0 as *mut tjtransform;
    
    
    let mut ps: c_int = tjPixelSize[pf as usize];
    
    
    
    
    
    
    
    let mut temp: *mut c_char = NULL_0 as *mut c_char;
    
    
    
    
    
    
    
    
    let mut subsamp: c_int = -1i32;
    let mut cs: c_int = -1i32;
    
    
    
    
    
    
    
    file = fopen(fileName,  b"rb\x00".as_ptr() as *const c_char);
    if file.is_null() {
        printf(
            
            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
            524i32,
            
            b"opening file\x00".as_ptr() as *const c_char,
            strerror(*__errno_location()),
        );
        retval = -1i32
    } else if fseek(file, 0i64, SEEK_END) < 0i32 || {
        srcSize = ftell(file) as c_ulong;
        (srcSize) == -1i32 as c_ulong
    } {
        printf(
            
            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
            527i32,
            
            b"determining file size\x00".as_ptr() as *const c_char,
            strerror(*__errno_location()),
        );
        retval = -1i32
    } else {
        srcBuf = malloc(srcSize) as *mut c_uchar;
        if srcBuf.is_null() {
            printf(
                
                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                529i32,
                
                b"allocating memory\x00".as_ptr() as *const c_char,
                strerror(*__errno_location()),
            );
            retval = -1i32
        } else if fseek(file, 0i64, SEEK_SET) < 0i32 {
            printf(
                
                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                531i32,
                
                b"setting file position\x00".as_ptr() as *const c_char,
                strerror(*__errno_location()),
            );
            retval = -1i32
        } else if fread(
            srcBuf as *mut c_void,
            srcSize,
            1u64,
            file,
        ) < 1u64
        {
            printf(
                
                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                533i32,
                
                b"reading JPEG data\x00".as_ptr() as *const c_char,
                strerror(*__errno_location()),
            );
            retval = -1i32
        } else {
             let mut current_block:  u64;fclose(file);
            file = NULL_0 as *mut FILE;
            temp = strrchr(fileName, '.' as i32);
            if !temp.is_null() {
                *temp =  '\u{0}' as c_char
            }
            handle = tjInitTransform();
            if handle.is_null() {
                let mut _tjErrorCode: c_int = tjGetErrorCode(handle);
                let mut _tjErrorStr: *mut c_char =
                    tjGetErrorStr2(handle);
                if flags & TJFLAG_STOPONWARNING == 0
                    && _tjErrorCode == TJERR_WARNING as c_int
                {
                    if strncmp(
                        tjErrorStr.as_mut_ptr(),
                        _tjErrorStr,
                        JMSG_LENGTH_MAX as c_ulong,
                    ) != 0
                        || strncmp(
                            tjErrorMsg.as_mut_ptr(),
                            
                            b"executing tjInitTransform()\x00".as_ptr() as *const c_char,
                            JMSG_LENGTH_MAX as c_ulong,
                        ) != 0
                        || tjErrorCode != _tjErrorCode
                        || tjErrorLine != 540i32
                    {
                        strncpy(
                            tjErrorStr.as_mut_ptr(),
                            _tjErrorStr,
                            JMSG_LENGTH_MAX as c_ulong,
                        );
                        strncpy(
                            tjErrorMsg.as_mut_ptr(),
                            
                            b"executing tjInitTransform()\x00".as_ptr() as *const c_char,
                            JMSG_LENGTH_MAX as c_ulong,
                        );
                        tjErrorCode = _tjErrorCode;
                        tjErrorLine = 540i32;
                        printf(
                            
                            b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                as *const c_char,
                            540i32,
                            
                            b"executing tjInitTransform()\x00".as_ptr() as *const c_char,
                            _tjErrorStr,
                        );
                    }
                    current_block = 9441801433784995173;
                } else {
                    printf(
                        
                        b"%s in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                        if _tjErrorCode == TJERR_WARNING as c_int {
                            
                            b"WARNING\x00".as_ptr() as *const c_char
                        } else {
                            
                            b"ERROR\x00".as_ptr() as *const c_char
                        },
                        540i32,
                        
                        b"executing tjInitTransform()\x00".as_ptr() as *const c_char,
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
                     let mut w:  c_int =  0i32; let mut h:  c_int =  0i32;if tjDecompressHeader3(
                        handle,
                        srcBuf,
                        srcSize,
                        &mut w,
                        &mut h,
                        &mut subsamp,
                        &mut cs,
                    ) == -1i32
                    {
                        let mut _tjErrorCode_0: c_int =
                            tjGetErrorCode(handle);
                        let mut _tjErrorStr_0: *mut c_char =
                            tjGetErrorStr2(handle);
                        if flags & TJFLAG_STOPONWARNING == 0
                            && _tjErrorCode_0 == TJERR_WARNING as c_int
                        {
                            if strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr_0,
                                JMSG_LENGTH_MAX as c_ulong,
                            ) != 0
                                || strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    
                                    b"executing tjDecompressHeader3()\x00".as_ptr()
                                        as *const c_char,
                                    JMSG_LENGTH_MAX as c_ulong,
                                ) != 0
                                || tjErrorCode != _tjErrorCode_0
                                || tjErrorLine != 543i32
                            {
                                strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr_0,
                                    JMSG_LENGTH_MAX as c_ulong,
                                );
                                strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    
                                    b"executing tjDecompressHeader3()\x00".as_ptr()
                                        as *const c_char,
                                    JMSG_LENGTH_MAX as c_ulong,
                                );
                                tjErrorCode = _tjErrorCode_0;
                                tjErrorLine = 543i32;
                                printf(
                                    
                                    b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    543i32,
                                    
                                    b"executing tjDecompressHeader3()\x00".as_ptr()
                                        as *const c_char,
                                    _tjErrorStr_0,
                                );
                            }
                            current_block = 168769493162332264;
                        } else {
                            printf(
                                
                                b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                if _tjErrorCode_0
                                    == TJERR_WARNING as c_int
                                {
                                    
                                    b"WARNING\x00".as_ptr() as *const c_char
                                } else {
                                    
                                    b"ERROR\x00".as_ptr() as *const c_char
                                },
                                543i32,
                                
                                b"executing tjDecompressHeader3()\x00".as_ptr()
                                    as *const c_char,
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
                             let mut tempStr:  [c_char; 80] =  [0; 80];  if cs == TJCS_YCCK as c_int
                                || cs == TJCS_CMYK as c_int
                            {
                                pf =  TJPF_CMYK;
                                ps = tjPixelSize[pf as usize]
                            }
                            if quiet == 1i32 {
                                printf(
                                    
                                    b"All performance values in Mpixels/sec\n\n\x00".as_ptr()
                                        as *const c_char,
                                );
                                printf(b"Bitmap     JPEG   JPEG     %s  %s   Xform   Comp    Decomp  \x00".as_ptr() as
                                           *const c_char,
                                       if doTile != 0 {
                                           
                                           b"Tile \x00".as_ptr() as
                                               *const c_char
                                       } else {
                                           
                                           b"Image\x00".as_ptr() as
                                               *const c_char
                                       },
                                       if doTile != 0 {
                                           
                                           b"Tile \x00".as_ptr() as
                                               *const c_char
                                       } else {
                                           
                                           b"Image\x00".as_ptr() as
                                               *const c_char
                                       });
                                if doYUV != 0 {
                                    printf(
                                        
                                        b"Decode\x00".as_ptr() as *const c_char,
                                    );
                                }
                                printf(
                                    
                                    b"\n\x00".as_ptr() as *const c_char,
                                );
                                printf(b"Format     CS     Subsamp  Width  Height  Perf    Ratio   Perf    \x00".as_ptr() as
                                           *const c_char);
                                if doYUV != 0 {
                                    printf(
                                        
                                        b"Perf\x00".as_ptr() as *const c_char,
                                    );
                                }
                                printf(
                                    
                                    b"\n\n\x00".as_ptr() as *const c_char,
                                );
                            } else if quiet == 0 {
                                printf(
                                    
                                    b">>>>>  JPEG %s --> %s (%s)  <<<<<\n\x00".as_ptr()
                                        as *const c_char,
                                    formatName(subsamp, cs, tempStr.as_mut_ptr()),
                                    pixFormatStr[pf as usize],
                                    if flags & TJFLAG_BOTTOMUP != 0 {
                                        
                                        b"Bottom-up\x00".as_ptr() as *const c_char
                                    } else {
                                        
                                        b"Top-down\x00".as_ptr() as *const c_char
                                    },
                                );
                            }
                            
                             let mut tilew:   c_int = if doTile != 0 { 16i32 } else { w }; let mut tileh:   c_int = if doTile != 0 { 16i32 } else { h };
                            's_381: loop {
                                if tilew > w {
                                    tilew = w
                                }
                                if tileh > h {
                                    tileh = h
                                }
                                ntilesw = (w + tilew - 1i32) / tilew;
                                ntilesh = (h + tileh - 1i32) / tileh;
                                jpegBuf = malloc(
                                    ::std::mem::size_of::<*mut c_uchar>() as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                                )
                                    as *mut *mut c_uchar;
                                if jpegBuf.is_null() {
                                    printf(
                                        
                                        b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                            as *const c_char,
                                        571i32,
                                        
                                        b"allocating JPEG tile array\x00".as_ptr()
                                            as *const c_char,
                                        strerror(*__errno_location()),
                                    );
                                    retval = -1i32;
                                    break;
                                } else {
                                    memset(
                                        jpegBuf as *mut c_void,
                                        0i32,
                                        ::std::mem::size_of::<*mut c_uchar>()
                                            as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                                    );
                                    jpegSize = malloc(
                                        ::std::mem::size_of::<c_ulong>() as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                                    )
                                        as *mut c_ulong;
                                    if jpegSize.is_null() {
                                        printf(
                                            
                                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                as *const c_char,
                                            575i32,
                                            
                                            b"allocating JPEG size array\x00".as_ptr()
                                                as *const c_char,
                                            strerror(
                                                *__errno_location(),
                                            ),
                                        );
                                        retval = -1i32;
                                        break;
                                    } else {
                                         let mut decompsrc:  c_int =  0i32;     memset(
                                            jpegSize as *mut c_void,
                                            0i32,
                                            ::std::mem::size_of::<c_ulong>()
                                                as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                                        );
                                        if flags & TJFLAG_NOREALLOC != 0i32
                                            || doTile == 0
                                        {
                                            i = 0i32;
                                            while i < ntilesw * ntilesh {
                                                let ref mut fresh3 = *jpegBuf.offset(i as isize);
                                                *fresh3 = tjAlloc(
                                                    tjBufSize(
                                                        tilew, tileh, subsamp,
                                                    )
                                                        as c_int,
                                                );
                                                if (*fresh3).is_null() {
                                                    printf(
                                                        
                                                        b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                            as *const c_char,
                                                        582i32,
                                                        
                                                        b"allocating JPEG tiles\x00".as_ptr()
                                                            as *const c_char,
                                                        strerror(
                                                            *__errno_location(),
                                                        ),
                                                    );
                                                    retval = -1i32;
                                                    break 's_381;
                                                } else {
                                                    i += 1
                                                }
                                            }
                                        }
                                        
                                        
                                        
                                         let mut tw:   c_int =  w; let mut th:   c_int =  h; let mut ttilew:   c_int =  tilew; let mut ttileh:   c_int =  tileh;
                                        if quiet == 0 {
                                            printf(
                                                
                                                b"\n%s size: %d x %d\x00".as_ptr()
                                                    as *const c_char,
                                                if doTile != 0 {
                                                    
                                                    b"Tile\x00".as_ptr() as *const c_char
                                                } else {
                                                    
                                                    b"Image\x00".as_ptr() as *const c_char
                                                },
                                                ttilew,
                                                ttileh,
                                            );
                                            if sf.num != 1i32 || sf.denom != 1i32 {
                                                printf(
                                                    
                                                    b" --> %d x %d\x00".as_ptr()
                                                        as *const c_char,
                                                    (tw * sf.num + sf.denom - 1i32) / sf.denom,
                                                    (th * sf.num + sf.denom - 1i32) / sf.denom,
                                                );
                                            }
                                            printf(
                                                
                                                b"\n\x00".as_ptr() as *const c_char,
                                            );
                                        } else if quiet == 1i32 {
                                            printf(
                                                
                                                b"%-4s (%s)  %-5s  %-5s    \x00".as_ptr()
                                                    as *const c_char,
                                                pixFormatStr[pf as usize],
                                                if flags & TJFLAG_BOTTOMUP
                                                    != 0
                                                {
                                                    
                                                    b"BU\x00".as_ptr() as *const c_char
                                                } else {
                                                    
                                                    b"TD\x00".as_ptr() as *const c_char
                                                },
                                                csName[cs as usize],
                                                subNameLong[subsamp as usize],
                                            );
                                            printf(
                                                
                                                b"%-5d  %-5d   \x00".as_ptr()
                                                    as *const c_char,
                                                tilew,
                                                tileh,
                                            );
                                        }
                                         let mut tsubsamp:   c_int =  subsamp;
                                        if doTile != 0
                                            || xformOp
                                                != TJXOP_NONE as c_int
                                            || xformOpt != 0i32
                                            || customFilter.is_some()
                                        {
                                            t = malloc(
                                                ::std::mem::size_of::<
                                                    tjtransform,
                                                >()
                                                    as c_ulong *
    ntilesw as c_ulong * ntilesh as c_ulong,
                                            )
                                                as *mut tjtransform;
                                            if t.is_null() {
                                                printf(
                                                    
                                                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                        as *const c_char,
                                                    602i32,
                                                    
                                                    b"allocating image transform array\x00".as_ptr()
                                                        as *const c_char,
                                                    strerror(
                                                        *__errno_location(),
                                                    ),
                                                );
                                                retval = -1i32;
                                                break;
                                            } else {
                                                       if xformOp
                                                    == TJXOP_TRANSPOSE
                                                        as c_int
                                                    || xformOp
                                                        == TJXOP_TRANSVERSE
                                                            as c_int
                                                    || xformOp
                                                        == TJXOP_ROT90
                                                            as c_int
                                                    || xformOp
                                                        == TJXOP_ROT270
                                                            as c_int
                                                {
                                                    tw = h;
                                                    th = w;
                                                    ttilew = tileh;
                                                    ttileh = tilew
                                                }
                                                if xformOpt & TJXOPT_GRAY
                                                    != 0
                                                {
                                                    tsubsamp = TJ_GRAYSCALE
                                                }
                                                if xformOp
                                                    == TJXOP_HFLIP
                                                        as c_int
                                                    || xformOp
                                                        == TJXOP_ROT180
                                                            as c_int
                                                {
                                                    tw -=  tw % tjMCUWidth
                                                            [tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == TJXOP_VFLIP
                                                        as c_int
                                                    || xformOp
                                                        == TJXOP_ROT180
                                                            as c_int
                                                {
                                                    th -=  th % tjMCUHeight
                                                            [tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == TJXOP_TRANSVERSE
                                                        as c_int
                                                    || xformOp
                                                        == TJXOP_ROT90
                                                            as c_int
                                                {
                                                    tw -=  tw % tjMCUHeight
                                                            [tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == TJXOP_TRANSVERSE
                                                        as c_int
                                                    || xformOp
                                                        == TJXOP_ROT270
                                                            as c_int
                                                {
                                                    th -=  th % tjMCUWidth
                                                            [tsubsamp as usize]
                                                }
                                                
                                                 let mut tntilesw:   c_int =  (tw + ttilew - 1i32) / ttilew; let mut tntilesh:   c_int =  (th + ttileh - 1i32) / ttileh;
                                                if xformOp
                                                    == TJXOP_TRANSPOSE
                                                        as c_int
                                                    || xformOp
                                                        == TJXOP_TRANSVERSE
                                                            as c_int
                                                    || xformOp
                                                        == TJXOP_ROT90
                                                            as c_int
                                                    || xformOp
                                                        == TJXOP_ROT270
                                                            as c_int
                                                {
                                                    if tsubsamp
                                                        == TJSAMP_422
                                                            as c_int
                                                    {
                                                        tsubsamp = TJSAMP_440
                                                            as c_int
                                                    } else if tsubsamp
                                                        == TJSAMP_440
                                                            as c_int
                                                    {
                                                        tsubsamp = TJSAMP_422
                                                            as c_int
                                                    }
                                                }
                                                
                                                 let mut row:   c_int =  0i32; let mut tile:   c_int =  0i32;
                                                while row < tntilesh {
                                                      let mut col:   c_int =  0i32;
                                                    while col < tntilesw {
                                                        (*t.offset(tile as isize)).r.w =
                                                            if ttilew < tw - col * ttilew {
                                                                ttilew
                                                            } else {
                                                                (tw) - col * ttilew
                                                            };
                                                        (*t.offset(tile as isize)).r.h =
                                                            if ttileh < th - row * ttileh {
                                                                ttileh
                                                            } else {
                                                                (th) - row * ttileh
                                                            };
                                                        (*t.offset(tile as isize)).r.x =
                                                            col * ttilew;
                                                        (*t.offset(tile as isize)).r.y =
                                                            row * ttileh;
                                                        (*t.offset(tile as isize)).op = xformOp;
                                                        (*t.offset(tile as
                                                                           isize)).options
                                                                =
                                                                xformOpt |
                                                                    TJXOPT_TRIM;
                                                        let ref mut fresh4 =
                                                            (*t.offset(tile as isize)).customFilter;
                                                        *fresh4 = customFilter;
                                                        if (*t.offset(tile as isize)).options
                                                            & TJXOPT_NOOUTPUT
                                                            != 0
                                                            && !(*jpegBuf.offset(tile as isize))
                                                                .is_null()
                                                        {
                                                            tjFree(
                                                                *jpegBuf.offset(tile as isize),
                                                            );
                                                            let ref mut fresh5 =
                                                                *jpegBuf.offset(tile as isize);
                                                            *fresh5 = NULL_0
                                                                as *mut c_uchar
                                                        }
                                                        col += 1;
                                                        tile += 1
                                                    }
                                                    row += 1
                                                }
                                                
                                                 let mut iter:   c_int =  -1i32; let mut elapsed:   c_double =  0.0f64;
                                                loop {
                                                      let mut start:   c_double =  getTime();
                                                    if tjTransform(
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
                                                        let mut _tjErrorCode_1: c_int =
                                                            tjGetErrorCode(
                                                                handle,
                                                            );
                                                        let mut _tjErrorStr_1: *mut c_char =
                                                            tjGetErrorStr2(
                                                                handle,
                                                            );
                                                        if flags &
                                                                   TJFLAG_STOPONWARNING
                                                                   == 0 &&
                                                                   _tjErrorCode_1
                                                                       ==
                                                                       TJERR_WARNING
                                                                           as
                                                                           c_int
                                                               {
                                                                if strncmp(tjErrorStr.as_mut_ptr(),
                                                                           _tjErrorStr_1,
                                                                           JMSG_LENGTH_MAX
                                                                               as
                                                                               c_ulong)
                                                                       != 0 ||
                                                                       strncmp(tjErrorMsg.as_mut_ptr(),
                                                                               
                                                                               b"executing tjTransform()\x00".as_ptr()
                                                                                   as
                                                                                   *const c_char,
                                                                               JMSG_LENGTH_MAX
                                                                                   as
                                                                                   c_ulong)
                                                                           !=
                                                                           0
                                                                       ||
                                                                       tjErrorCode
                                                                           !=
                                                                           _tjErrorCode_1
                                                                       ||
                                                                       tjErrorLine
                                                                           !=
                                                                           648i32
                                                                   {
                                                                    strncpy(tjErrorStr.as_mut_ptr(),
                                                                            _tjErrorStr_1,
                                                                            JMSG_LENGTH_MAX
                                                                                as
                                                                                c_ulong);
                                                                    strncpy(tjErrorMsg.as_mut_ptr(),
                                                                            
                                                                            b"executing tjTransform()\x00".as_ptr()
                                                                                as
                                                                                *const c_char,
                                                                            JMSG_LENGTH_MAX
                                                                                as
                                                                                c_ulong);
                                                                    tjErrorCode
                                                                        =
                                                                        _tjErrorCode_1;
                                                                    tjErrorLine
                                                                        =
                                                                        648i32;
                                                                    printf(b"WARNING in line %d while %s:\n%s\n\x00".as_ptr()
                                                                               as
                                                                               *const c_char,
                                                                           648i32,
                                                                           
                                                                           b"executing tjTransform()\x00".as_ptr()
                                                                               as
                                                                               *const c_char,
                                                                           _tjErrorStr_1);
                                                                }
                                                            } else {
                                                                printf(b"%s in line %d while %s:\n%s\n\x00".as_ptr()
                                                                           as
                                                                           *const c_char,
                                                                       if _tjErrorCode_1
                                                                              ==
                                                                              TJERR_WARNING
                                                                                  as
                                                                                  c_int
                                                                          {
                                                                           
                                                                           b"WARNING\x00".as_ptr()
                                                                               as
                                                                               *const c_char
                                                                       } else {
                                                                           
                                                                           b"ERROR\x00".as_ptr()
                                                                               as
                                                                               *const c_char
                                                                       },
                                                                       648i32,
                                                                       
                                                                       b"executing tjTransform()\x00".as_ptr()
                                                                           as
                                                                           *const c_char,
                                                                       _tjErrorStr_1);
                                                                retval =
                                                                    -1i32;
                                                                break 's_381 ;
                                                            }
                                                    }
                                                    elapsed +=
                                                        getTime() - start;
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
                                                free(t as *mut c_void);
                                                t = NULL_0
                                                    as *mut tjtransform;
                                                tile = 0i32;
                                                 let mut totalJpegSize:   c_ulong =  0u64;
                                                while tile < tntilesw * tntilesh {
                                                    totalJpegSize +=  
                                                        *jpegSize.offset(tile as isize);
                                                    tile += 1
                                                }
                                                if quiet != 0 {
                                                     let mut tempStr2:  [c_char; 80] =  [0; 80];printf(
                                                        
                                                        b"%-6s%s%-6s%s\x00".as_ptr()
                                                            as *const c_char,
                                                        sigfig(
                                                            (w * h) as c_double
                                                                / 1000000.0f64
                                                                / elapsed,
                                                            4i32,
                                                            tempStr.as_mut_ptr(),
                                                            80i32,
                                                        ),
                                                        if quiet == 2i32 {
                                                            
                                                            b"\n\x00".as_ptr()
                                                                as *const c_char
                                                        } else {
                                                            
                                                            b"  \x00".as_ptr()
                                                                as *const c_char
                                                        },
                                                        sigfig(
                                                            (w * h * ps) as c_double
                                                                / totalJpegSize as c_double,
                                                            4i32,
                                                            tempStr2.as_mut_ptr(),
                                                            80i32,
                                                        ),
                                                        if quiet == 2i32 {
                                                            
                                                            b"\n\x00".as_ptr()
                                                                as *const c_char
                                                        } else {
                                                            
                                                            b"  \x00".as_ptr()
                                                                as *const c_char
                                                        },
                                                    );
                                                } else if quiet == 0 {
                                                    printf(b"Transform     --> Frame rate:         %f fps\n\x00".as_ptr()
                                                                   as
                                                                   *const c_char,
                                                               1.0f64 /
                                                                   elapsed);
                                                    printf(b"                  Output image size:  %lu bytes\n\x00".as_ptr()
                                                                   as
                                                                   *const c_char,
                                                               totalJpegSize);
                                                    printf(b"                  Compression ratio:  %f:1\n\x00".as_ptr()
                                                                   as
                                                                   *const c_char,
                                                               (w * h * ps) as
                                                                   c_double
                                                                   /
                                                                   totalJpegSize
                                                                       as
                                                                       c_double);
                                                    printf(b"                  Throughput:         %f Megapixels/sec\n\x00".as_ptr()
                                                                   as
                                                                   *const c_char,
                                                               (w * h) as
                                                                   c_double
                                                                   /
                                                                   1000000.0f64
                                                                   / elapsed);
                                                    printf(b"                  Output bit stream:  %f Megabits/sec\n\x00".as_ptr()
                                                                   as
                                                                   *const c_char,
                                                               totalJpegSize
                                                                   as
                                                                   c_double
                                                                   * 8.0f64 /
                                                                   1000000.0f64
                                                                   / elapsed);
                                                }
                                            }
                                        } else {
                                            if quiet == 1i32 {
                                                printf(
                                                    
                                                    b"N/A     N/A     \x00".as_ptr()
                                                        as *const c_char,
                                                );
                                            }
                                            tjFree(*jpegBuf.offset(0));
                                            let ref mut fresh6 = *jpegBuf.offset(0);
                                            *fresh6 = NULL_0 as *mut c_uchar;
                                            decompsrc = 1i32
                                        }
                                        if w == tilew {
                                            ttilew = tw
                                        }
                                        if h == tileh {
                                            ttileh = th
                                        }
                                        if xformOpt & TJXOPT_NOOUTPUT == 0 {
                                            if decomp(
                                                NULL_0 as *mut c_uchar,
                                                (if decompsrc != 0 {
                                                    &mut srcBuf
                                                } else {
                                                    jpegBuf
                                                }),
                                                (if decompsrc != 0 {
                                                    &mut srcSize
                                                } else {
                                                    jpegSize
                                                }),
                                                NULL_0 as *mut c_uchar,
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
                                            printf(
                                                
                                                b"N/A\n\x00".as_ptr() as *const c_char,
                                            );
                                        }
                                        i = 0i32;
                                        while i < ntilesw * ntilesh {
                                            tjFree(
                                                *jpegBuf.offset(i as isize),
                                            );
                                            let ref mut fresh7 = *jpegBuf.offset(i as isize);
                                            *fresh7 = NULL_0 as *mut c_uchar;
                                            i += 1
                                        }
                                        free(jpegBuf as *mut c_void);
                                        jpegBuf =
                                            NULL_0 as *mut *mut c_uchar;
                                        if !jpegSize.is_null() {
                                            free(jpegSize as *mut c_void);
                                            jpegSize = NULL_0 as *mut c_ulong
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
        fclose(file);
        file = NULL_0 as *mut FILE
    }
    if !jpegBuf.is_null() {
        i = 0i32;
        while i < ntilesw * ntilesh {
            if !(*jpegBuf.offset(i as isize)).is_null() {
                tjFree(*jpegBuf.offset(i as isize));
            }
            let ref mut fresh8 = *jpegBuf.offset(i as isize);
            *fresh8 = NULL_0 as *mut c_uchar;
            i += 1
        }
        free(jpegBuf as *mut c_void);
        jpegBuf = NULL_0 as *mut *mut c_uchar
    }
    if !jpegSize.is_null() {
        free(jpegSize as *mut c_void);
        jpegSize = NULL_0 as *mut c_ulong
    }
    if !srcBuf.is_null() {
        free(srcBuf as *mut c_void);
        srcBuf = NULL_0 as *mut c_uchar
    }
    if !t.is_null() {
        free(t as *mut c_void);
        t = NULL_0 as *mut tjtransform
    }
    if !handle.is_null() {
        tjDestroy(handle);
        handle = NULL_0 as *mut c_void
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn usage(mut progName: *mut c_char) {
     
    printf(
        
        b"USAGE: %s\n\x00".as_ptr() as *const c_char,
        progName,
    );
    printf(
        
        b"       <Inputfile (BMP|PPM)> <Quality> [options]\n\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"       %s\n\x00".as_ptr() as *const c_char,
        progName,
    );
    printf(
        
        b"       <Inputfile (JPG)> [options]\n\n\x00".as_ptr() as *const c_char,
    );
    printf(b"Options:\n\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-alloc = Dynamically allocate JPEG image buffers\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-bmp = Generate output images in Windows Bitmap format (default = PPM)\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-bottomup = Test bottom-up compression/decompression\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-tile = Test performance of the codec when the image is encoded as separate\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     tiles of varying sizes.\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-rgb, -bgr, -rgbx, -bgrx, -xbgr, -xrgb =\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     Test the specified color conversion path in the codec (default = BGR)\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-cmyk = Indirectly test YCCK JPEG compression/decompression (the source\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     and destination bitmaps are still RGB.  The conversion is done\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"     internally prior to compression or after decompression.)\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-fastupsample = Use the fastest chrominance upsampling algorithm available in\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     the underlying codec\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-fastdct = Use the fastest DCT/IDCT algorithms available in the underlying\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     codec\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-accuratedct = Use the most accurate DCT/IDCT algorithms available in the\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     underlying codec\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-progressive = Use progressive entropy coding in JPEG images generated by\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     compression and transform operations.\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-subsamp <s> = When testing JPEG compression, this option specifies the level\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     of chrominance subsampling to use (<s> = 444, 422, 440, 420, 411, or\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     GRAY).  The default is to test Grayscale, 4:2:0, 4:2:2, and 4:4:4 in\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     sequence.\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-quiet = Output results in tabular rather than verbose format\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-yuv = Test YUV encoding/decoding functions\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-yuvpad <p> = If testing YUV encoding/decoding, this specifies the number of\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     bytes to which each row of each plane in the intermediate YUV image is\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     padded (default = 1)\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-scale M/N = Scale down the width/height of the decompressed JPEG image by a\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     factor of M/N (M/N = \x00".as_ptr() as *const c_char);
     let mut i:   c_int =  0i32;
    while i < nsf {
        printf(
            
            b"%d/%d\x00".as_ptr() as *const c_char,
            (*scalingFactors.offset(i as isize)).num,
            (*scalingFactors.offset(i as isize)).denom,
        );
        if nsf == 2i32 && i != nsf - 1i32 {
            printf(b" or \x00".as_ptr() as *const c_char);
        } else if nsf > 2i32 {
            if i != nsf - 1i32 {
                printf(b", \x00".as_ptr() as *const c_char);
            }
            if i == nsf - 2i32 {
                printf(b"or \x00".as_ptr() as *const c_char);
            }
        }
        if i % 8i32 == 0i32 && i != 0i32 {
            printf(b"\n     \x00".as_ptr() as *const c_char);
        }
        i += 1
    }
    printf(b")\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270 =\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"     Perform the corresponding lossless transform prior to\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"     decompression (these options are mutually exclusive)\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-grayscale = Perform lossless grayscale conversion prior to decompression\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     test (can be combined with the other transforms above)\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-copynone = Do not copy any extra markers (including EXIF and ICC profile data)\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     when transforming the image.\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-benchtime <t> = Run each benchmark for at least <t> seconds (default = 5.0)\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-warmup <t> = Run each benchmark for <t> seconds (default = 1.0) prior to\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     starting the timer, in order to prime the caches and thus improve the\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     consistency of the results.\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-componly = Stop after running compression tests.  Do not test decompression.\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-nowrite = Do not write reference or output images (improves consistency of\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     performance measurements.)\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-stoponwarning = Immediately discontinue the current\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"     compression/decompression/transform operation if the underlying codec\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     throws a warning (non-fatal error)\n\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"NOTE:  If the quality is specified as a range (e.g. 90-100), a separate\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"test will be performed for all quality values in the range.\n\n\x00".as_ptr()
            as *const c_char,
    );
    exit(1i32);
}

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
     let mut retval:  c_int =  0i32;
    let mut srcBuf: *mut c_uchar = NULL_0 as *mut c_uchar;
    
    
    
    
    let mut minQual: c_int = -1i32;
    let mut maxQual: c_int = -1i32;
    
    
    
    let mut subsamp: c_int = -1i32;
    scalingFactors = tjGetScalingFactors(&mut nsf);
    if scalingFactors.is_null() || nsf == 0i32 {
        printf(
            
            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
            804i32,
            
            b"executing tjGetScalingFactors()\x00".as_ptr() as *const c_char,
            tjGetErrorStr(),
        );
        retval = -1i32
    } else {
         let mut current_block:  u64; let mut w:  c_int =  0i32; let mut h:  c_int =  0i32; let mut i:  c_int =  0;  let mut minArg:  c_int =  2i32;if argc < minArg {
            usage(*argv.offset(0));
        }
         let mut temp:   *mut c_char =
     strrchr(*argv.offset(1), '.' as i32);
        if !temp.is_null() {
            if strcasecmp(temp,  b".bmp\x00".as_ptr() as *const c_char) == 0
            {
                ext =    b"bmp\x00".as_ptr() as *mut c_char
            }
            if strcasecmp(temp,  b".jpg\x00".as_ptr() as *const c_char) == 0
                || strcasecmp(temp,  b".jpeg\x00".as_ptr() as *const c_char)
                    == 0
            {
                decompOnly = 1i32
            }
        }
        printf(b"\n\x00".as_ptr() as *const c_char);
        if decompOnly == 0 {
            minArg = 3i32;
            if argc < minArg {
                usage(*argv.offset(0));
            }
            minQual = atoi(*argv.offset(2));
            if minQual < 1i32 || minQual > 100i32 {
                puts(
                    
                    b"ERROR: Quality must be between 1 and 100.\x00".as_ptr()
                        as *const c_char,
                );
                exit(1i32);
            }
            temp = strchr(*argv.offset(2), '-' as i32);
            if !(!temp.is_null()
                && strlen(temp) > 1u64
                && sscanf(
                    &mut *temp.offset(1) as *mut c_char,
                    
                    b"%d\x00".as_ptr() as *const c_char,
                    &mut maxQual as *mut c_int,
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
                if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-tile\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    doTile = 1i32;
                    xformOpt |= TJXOPT_CROP
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-fastupsample\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    printf(
                        
                        b"Using fast upsampling code\n\n\x00".as_ptr() as *const c_char,
                    );
                    flags |= TJFLAG_FASTUPSAMPLE
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-fastdct\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    printf(
                        
                        b"Using fastest DCT/IDCT algorithm\n\n\x00".as_ptr()
                            as *const c_char,
                    );
                    flags |= TJFLAG_FASTDCT
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-accuratedct\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    printf(
                        
                        b"Using most accurate DCT/IDCT algorithm\n\n\x00".as_ptr()
                            as *const c_char,
                    );
                    flags |= TJFLAG_ACCURATEDCT
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-progressive\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    printf(
                        
                        b"Using progressive entropy coding\n\n\x00".as_ptr()
                            as *const c_char,
                    );
                    flags |= TJFLAG_PROGRESSIVE
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-rgb\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    pf =  TJPF_RGB
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-rgbx\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    pf =  TJPF_RGBX
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-bgr\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    pf =  TJPF_BGR
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-bgrx\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    pf =  TJPF_BGRX
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-xbgr\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    pf =  TJPF_XBGR
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-xrgb\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    pf =  TJPF_XRGB
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-cmyk\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    pf =  TJPF_CMYK
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-bottomup\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    flags |= TJFLAG_BOTTOMUP
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-quiet\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    quiet = 1i32
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-qq\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    quiet = 2i32
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-scale\x00".as_ptr() as *const c_char,
                ) == 0
                    && i < argc - 1i32
                {
                    
                    
                     let mut temp1:  c_int =  0i32; let mut temp2:  c_int =  0i32;
                    i += 1;
                    if sscanf(
                        *argv.offset(i as isize),
                        
                        b"%d/%d\x00".as_ptr() as *const c_char,
                        &mut temp1 as *mut c_int,
                        &mut temp2 as *mut c_int,
                    ) == 2i32
                    {
                          let mut match_0:  c_int =  0i32; let mut j:   c_int =  0i32;
                        while j < nsf {
                            if temp1 as c_double / temp2 as c_double
                                == (*scalingFactors.offset(j as isize)).num as c_double
                                    / (*scalingFactors.offset(j as isize)).denom as c_double
                            {
                                sf = *scalingFactors.offset(j as isize);
                                match_0 = 1i32;
                                break;
                            } else {
                                j += 1
                            }
                        }
                        if match_0 == 0 {
                            usage(*argv.offset(0));
                        }
                    } else {
                        usage(*argv.offset(0));
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-hflip\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOp = TJXOP_HFLIP as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-vflip\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOp = TJXOP_VFLIP as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-transpose\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOp = TJXOP_TRANSPOSE as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-transverse\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOp = TJXOP_TRANSVERSE as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-rot90\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOp = TJXOP_ROT90 as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-rot180\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOp = TJXOP_ROT180 as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-rot270\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOp = TJXOP_ROT270 as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-grayscale\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOpt |= TJXOPT_GRAY
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-custom\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    customFilter = Some(
                        dummyDCTFilter
                            as unsafe extern "C" fn(
                                _: *mut c_short,
                                _: tjregion,
                                _: tjregion,
                                _: c_int,
                                _: c_int,
                                _: *mut tjtransform,
                            ) -> c_int,
                    )
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-nooutput\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOpt |= TJXOPT_NOOUTPUT
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-copynone\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    xformOpt |= TJXOPT_COPYNONE
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-benchtime\x00".as_ptr() as *const c_char,
                ) == 0
                    && i < argc - 1i32
                {
                    i += 1;
                    let mut temp_0: c_double = atof(*argv.offset(i as isize));
                    if temp_0 > 0.0f64 {
                        benchTime = temp_0
                    } else {
                        usage(*argv.offset(0));
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-warmup\x00".as_ptr() as *const c_char,
                ) == 0
                    && i < argc - 1i32
                {
                    i += 1;
                    let mut temp_1: c_double = atof(*argv.offset(i as isize));
                    if temp_1 >= 0.0f64 {
                        warmup = temp_1
                    } else {
                        usage(*argv.offset(0));
                    }
                    printf(
                        
                        b"Warmup time = %.1f seconds\n\n\x00".as_ptr() as *const c_char,
                        warmup,
                    );
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-alloc\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    flags &= !TJFLAG_NOREALLOC
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-bmp\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    ext =    b"bmp\x00".as_ptr() as *mut c_char
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-yuv\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    printf(
                        
                        b"Testing YUV planar encoding/decoding\n\n\x00".as_ptr()
                            as *const c_char,
                    );
                    doYUV = 1i32
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-yuvpad\x00".as_ptr() as *const c_char,
                ) == 0
                    && i < argc - 1i32
                {
                    i += 1;
                    let mut temp_2: c_int = atoi(*argv.offset(i as isize));
                    if temp_2 >= 1i32 {
                        yuvPad = temp_2
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-subsamp\x00".as_ptr() as *const c_char,
                ) == 0
                    && i < argc - 1i32
                {
                    i += 1;
                    if ({
                         let mut __res:  c_int =  0;
                        if ::std::mem::size_of::<c_char>() as c_ulong
                            > 1u64
                        {
                            if 0 != 0 {
                                let mut __c: c_int =
                                    *(*argv.offset(i as isize)).offset(0) as c_int;
                                __res = (if __c < -128i32 || __c > 255i32 {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                })
                            } else {
                                __res = toupper(
                                    *(*argv.offset(i as isize)).offset(0) as c_int,
                                )
                            }
                        } else {
                            __res =
                                *(*__ctype_toupper_loc())
                                    .offset(*(*argv.offset(i as isize)).offset(0) as c_int
                                        as isize)
                        }
                        __res
                    }) == 'G' as i32
                    {
                        subsamp = TJSAMP_GRAY as c_int
                    } else {
                        let mut temp_3: c_int = atoi(*argv.offset(i as isize));
                        match temp_3 {
                            444 => subsamp = TJSAMP_444 as c_int,
                            422 => subsamp = TJSAMP_422 as c_int,
                            440 => subsamp = TJSAMP_440 as c_int,
                            420 => subsamp = TJSAMP_420 as c_int,
                            411 => subsamp = TJSAMP_411 as c_int,
                            _ => {}
                        }
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-componly\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    compOnly = 1i32
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-nowrite\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    doWrite = 0i32
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"-stoponwarning\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    flags |= TJFLAG_STOPONWARNING
                } else {
                    usage(*argv.offset(0));
                }
                i += 1
            }
        }
        if (sf.num != 1i32 || sf.denom != 1i32) && doTile != 0 {
            printf(
                
                b"Disabling tiled compression/decompression tests, because those tests do not\n\x00".as_ptr() as *const c_char,
            );
            printf(
                
                b"work when scaled decompression is enabled.\n\x00".as_ptr()
                    as *const c_char,
            );
            doTile = 0i32
        }
        if flags & TJFLAG_NOREALLOC == 0i32 && doTile != 0 {
            printf(
                
                b"Disabling tiled compression/decompression tests, because those tests do not\n\x00".as_ptr() as *const c_char,
            );
            printf(
                
                b"work when dynamic JPEG buffer allocation is enabled.\n\n\x00".as_ptr()
                    as *const c_char,
            );
            doTile = 0i32
        }
        if decompOnly == 0 {
            srcBuf = tjLoadImage(
                *argv.offset(1),
                &mut w,
                1i32,
                &mut h,
                &mut pf,
                flags,
            );
            if srcBuf.is_null() {
                printf(
                    
                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                    962i32,
                    
                    b"loading bitmap\x00".as_ptr() as *const c_char,
                    tjGetErrorStr2(
                        NULL_0 as *mut c_void,
                    ),
                );
                retval = -1i32;
                current_block = 15940078839392993310;
            } else {
                temp = strrchr(*argv.offset(1), '.' as i32);
                if !temp.is_null() {
                    *temp =  '\u{0}' as c_char
                }
                current_block = 11359721434352816539;
            }
        } else {
            current_block = 11359721434352816539;
        }
        match current_block {
            15940078839392993310 => {}
            _ => {
                if quiet == 1i32 && decompOnly == 0 {
                    printf(
                        
                        b"All performance values in Mpixels/sec\n\n\x00".as_ptr()
                            as *const c_char,
                    );
                    printf(
                        
                        b"Bitmap     JPEG     JPEG  %s  %s   \x00".as_ptr()
                            as *const c_char,
                        if doTile != 0 {
                            
                            b"Tile \x00".as_ptr() as *const c_char
                        } else {
                            
                            b"Image\x00".as_ptr() as *const c_char
                        },
                        if doTile != 0 {
                            
                            b"Tile \x00".as_ptr() as *const c_char
                        } else {
                            
                            b"Image\x00".as_ptr() as *const c_char
                        },
                    );
                    if doYUV != 0 {
                        printf(b"Encode  \x00".as_ptr() as *const c_char);
                    }
                    printf(
                        
                        b"Comp    Comp    Decomp  \x00".as_ptr() as *const c_char,
                    );
                    if doYUV != 0 {
                        printf(b"Decode\x00".as_ptr() as *const c_char);
                    }
                    printf(b"\n\x00".as_ptr() as *const c_char);
                    printf(
                        
                        b"Format     Subsamp  Qual  Width  Height  \x00".as_ptr()
                            as *const c_char,
                    );
                    if doYUV != 0 {
                        printf(b"Perf    \x00".as_ptr() as *const c_char);
                    }
                    printf(
                        
                        b"Perf    Ratio   Perf    \x00".as_ptr() as *const c_char,
                    );
                    if doYUV != 0 {
                        printf(b"Perf\x00".as_ptr() as *const c_char);
                    }
                    printf(b"\n\n\x00".as_ptr() as *const c_char);
                }
                if decompOnly != 0 {
                    decompTest(*argv.offset(1));
                    printf(b"\n\x00".as_ptr() as *const c_char);
                } else if subsamp >= 0i32 && subsamp < TJ_NUMSAMP {
                    i = maxQual;
                    while i >= minQual {
                        fullTest(srcBuf, w, h, subsamp, i, *argv.offset(1));
                        i -= 1
                    }
                    printf(b"\n\x00".as_ptr() as *const c_char);
                } else {
                    if pf !=  TJPF_CMYK {
                        i = maxQual;
                        while i >= minQual {
                            fullTest(
                                srcBuf,
                                w,
                                h,
                                TJSAMP_GRAY as c_int,
                                i,
                                *argv.offset(1),
                            );
                            i -= 1
                        }
                        printf(b"\n\x00".as_ptr() as *const c_char);
                    }
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            TJSAMP_420 as c_int,
                            i,
                            *argv.offset(1),
                        );
                        i -= 1
                    }
                    printf(b"\n\x00".as_ptr() as *const c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            TJSAMP_422 as c_int,
                            i,
                            *argv.offset(1),
                        );
                        i -= 1
                    }
                    printf(b"\n\x00".as_ptr() as *const c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            TJSAMP_444 as c_int,
                            i,
                            *argv.offset(1),
                        );
                        i -= 1
                    }
                    printf(b"\n\x00".as_ptr() as *const c_char);
                }
            }
        }
    }
    if !srcBuf.is_null() {
        tjFree(srcBuf);
    }
    return retval;
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
