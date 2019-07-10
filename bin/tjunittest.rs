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

pub use crate::cmyk_h::cmyk_to_rgb;
pub use crate::cmyk_h::rgb_to_cmyk;
pub use crate::jmorecfg_h::JSAMPLE;
use crate::md5::MD5File;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
use crate::stdlib::__errno_location;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::exit;
use crate::stdlib::fclose;
use crate::stdlib::fopen;
pub use crate::stdlib::free;
use crate::stdlib::fwrite;
pub use crate::stdlib::malloc;
use crate::stdlib::memset;
use crate::stdlib::printf;
pub use crate::stdlib::random;
use crate::stdlib::snprintf;
use crate::stdlib::strcasecmp;
use crate::stdlib::strerror;
use crate::stdlib::unlink;
pub use crate::stdlib::FILE;
pub use crate::stdlib::RAND_MAX;
pub use crate::stdlib::_IO_FILE;
pub use crate::turbojpeg::tjAlloc;
pub use crate::turbojpeg::tjAlphaOffset;
pub use crate::turbojpeg::tjBlueOffset;
pub use crate::turbojpeg::tjBufSize;
pub use crate::turbojpeg::tjBufSizeYUV2;
pub use crate::turbojpeg::tjCompress2;
pub use crate::turbojpeg::tjCompressFromYUV;
pub use crate::turbojpeg::tjDecodeYUV;
pub use crate::turbojpeg::tjDecompress2;
pub use crate::turbojpeg::tjDecompressHeader2;
pub use crate::turbojpeg::tjDecompressToYUV2;
pub use crate::turbojpeg::tjDestroy;
pub use crate::turbojpeg::tjEncodeYUV3;
pub use crate::turbojpeg::tjFree;
pub use crate::turbojpeg::tjGetErrorStr;
pub use crate::turbojpeg::tjGetScalingFactors;
pub use crate::turbojpeg::tjGreenOffset;
pub use crate::turbojpeg::tjInitCompress;
pub use crate::turbojpeg::tjInitDecompress;
pub use crate::turbojpeg::tjLoadImage;
pub use crate::turbojpeg::tjMCUHeight;
pub use crate::turbojpeg::tjMCUWidth;
pub use crate::turbojpeg::tjPixelSize;
pub use crate::turbojpeg::tjRedOffset;
pub use crate::turbojpeg::tjSaveImage;
pub use crate::turbojpeg::tjhandle;
pub use crate::turbojpeg::tjscalingfactor;
pub use crate::turbojpeg::TJFLAG_BOTTOMUP;
pub use crate::turbojpeg::TJFLAG_FASTUPSAMPLE;
pub use crate::turbojpeg::TJFLAG_NOREALLOC;
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
pub use crate::turbojpeg::TJ_NUMPF;
pub use crate::turbojpeg::TJ_NUMSAMP;
extern crate libc;
use mozjpeg::*;

/*
 * Copyright (C)2009-2014, 2017-2018 D. R. Commander.  All Rights Reserved.
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
/*
 * This program tests the various code paths in the TurboJPEG C Wrapper
 */
#[no_mangle]
pub unsafe extern "C" fn usage(mut progName: *mut libc::c_char) {
    crate::stdlib::printf(
        b"\nUSAGE: %s [options]\n\n\x00" as *const u8 as *const libc::c_char,
        progName,
    );
    crate::stdlib::printf(b"Options:\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-yuv = test YUV encoding/decoding support\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-noyuvpad = do not pad each line of each Y, U, and V plane to the nearest\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"            4-byte boundary\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-alloc = test automatic buffer allocation\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-bmp = tjLoadImage()/tjSaveImage() unit test\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::exit(1i32);
}
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
pub static mut subName: [*const libc::c_char; 6] = [
    b"444\x00" as *const u8 as *const libc::c_char,
    b"422\x00" as *const u8 as *const libc::c_char,
    b"420\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"440\x00" as *const u8 as *const libc::c_char,
    b"411\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut pixFormatStr: [*const libc::c_char; 12] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"BGR\x00" as *const u8 as *const libc::c_char,
    b"RGBX\x00" as *const u8 as *const libc::c_char,
    b"BGRX\x00" as *const u8 as *const libc::c_char,
    b"XBGR\x00" as *const u8 as *const libc::c_char,
    b"XRGB\x00" as *const u8 as *const libc::c_char,
    b"Grayscale\x00" as *const u8 as *const libc::c_char,
    b"RGBA\x00" as *const u8 as *const libc::c_char,
    b"BGRA\x00" as *const u8 as *const libc::c_char,
    b"ABGR\x00" as *const u8 as *const libc::c_char,
    b"ARGB\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut _3byteFormats: [libc::c_int; 2] = [
    crate::turbojpeg::TJPF_RGB as libc::c_int,
    crate::turbojpeg::TJPF_BGR as libc::c_int,
];
#[no_mangle]
pub static mut _4byteFormats: [libc::c_int; 5] = [
    crate::turbojpeg::TJPF_RGBX as libc::c_int,
    crate::turbojpeg::TJPF_BGRX as libc::c_int,
    crate::turbojpeg::TJPF_XBGR as libc::c_int,
    crate::turbojpeg::TJPF_XRGB as libc::c_int,
    crate::turbojpeg::TJPF_CMYK as libc::c_int,
];
#[no_mangle]
pub static mut _onlyGray: [libc::c_int; 1] = [crate::turbojpeg::TJPF_GRAY as libc::c_int];
#[no_mangle]
pub static mut _onlyRGB: [libc::c_int; 1] = [crate::turbojpeg::TJPF_RGB as libc::c_int];
#[no_mangle]
pub static mut doYUV: libc::c_int = 0i32;
#[no_mangle]
pub static mut alloc: libc::c_int = 0i32;
#[no_mangle]
pub static mut pad: libc::c_int = 4i32;
#[no_mangle]
pub static mut exitStatus: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn initBuf(
    mut buf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut roffset: libc::c_int = crate::turbojpeg::tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = crate::turbojpeg::tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = crate::turbojpeg::tjBlueOffset[pf as usize];
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut index: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut halfway: libc::c_int = 16i32;
    if pf == crate::turbojpeg::TJPF_GRAY as libc::c_int {
        crate::stdlib::memset(
            buf as *mut libc::c_void,
            0i32,
            (w * h * ps) as libc::c_ulong,
        );
        row = 0i32;
        while row < h {
            col = 0i32;
            while col < w {
                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8i32 + col / 8i32) % 2i32 == 0i32 {
                    *buf.offset(index as isize) =
                        (if row < halfway { 255i32 } else { 0i32 }) as libc::c_uchar
                } else {
                    *buf.offset(index as isize) =
                        (if row < halfway { 76i32 } else { 226i32 }) as libc::c_uchar
                }
                col += 1
            }
            row += 1
        }
    } else if pf == crate::turbojpeg::TJPF_CMYK as libc::c_int {
        crate::stdlib::memset(
            buf as *mut libc::c_void,
            255i32,
            (w * h * ps) as libc::c_ulong,
        );
        row = 0i32;
        while row < h {
            col = 0i32;
            while col < w {
                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8i32 + col / 8i32) % 2i32 == 0i32 {
                    if row >= halfway {
                        *buf.offset((index * ps + 3i32) as isize) = 0i32 as libc::c_uchar
                    }
                } else {
                    *buf.offset((index * ps + 2i32) as isize) = 0i32 as libc::c_uchar;
                    if row < halfway {
                        *buf.offset((index * ps + 1i32) as isize) = 0i32 as libc::c_uchar
                    }
                }
                col += 1
            }
            row += 1
        }
    } else {
        crate::stdlib::memset(
            buf as *mut libc::c_void,
            0i32,
            (w * h * ps) as libc::c_ulong,
        );
        row = 0i32;
        while row < h {
            col = 0i32;
            while col < w {
                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8i32 + col / 8i32) % 2i32 == 0i32 {
                    if row < halfway {
                        *buf.offset((index * ps + roffset) as isize) = 255i32 as libc::c_uchar;
                        *buf.offset((index * ps + goffset) as isize) = 255i32 as libc::c_uchar;
                        *buf.offset((index * ps + boffset) as isize) = 255i32 as libc::c_uchar
                    }
                } else {
                    *buf.offset((index * ps + roffset) as isize) = 255i32 as libc::c_uchar;
                    if row >= halfway {
                        *buf.offset((index * ps + goffset) as isize) = 255i32 as libc::c_uchar
                    }
                }
                col += 1
            }
            row += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkBuf(
    mut buf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut subsamp: libc::c_int,
    mut sf: crate::turbojpeg::tjscalingfactor,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut roffset: libc::c_int = crate::turbojpeg::tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = crate::turbojpeg::tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = crate::turbojpeg::tjBlueOffset[pf as usize];
    let mut aoffset: libc::c_int = crate::turbojpeg::tjAlphaOffset[pf as usize];
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut index: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut retval: libc::c_int = 1i32;
    let mut halfway: libc::c_int = 16i32 * sf.num / sf.denom;
    let mut blocksize: libc::c_int = 8i32 * sf.num / sf.denom;
    if pf == crate::turbojpeg::TJPF_GRAY as libc::c_int {
        boffset = 0i32;
        goffset = boffset;
        roffset = goffset
    }
    if pf == crate::turbojpeg::TJPF_CMYK as libc::c_int {
        row = 0i32;
        's_40: loop {
            if !(row < h) {
                current_block = 15514718523126015390;
                break;
            }
            col = 0i32;
            while col < w {
                let mut c: libc::c_uchar = 0;
                let mut m: libc::c_uchar = 0;
                let mut y: libc::c_uchar = 0;
                let mut k: libc::c_uchar = 0;
                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                c = *buf.offset((index * ps) as isize);
                m = *buf.offset((index * ps + 1i32) as isize);
                y = *buf.offset((index * ps + 2i32) as isize);
                k = *buf.offset((index * ps + 3i32) as isize);
                if (row / blocksize + col / blocksize) % 2i32 == 0i32 {
                    if (c as libc::c_int) < 254i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"c\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            c as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if (m as libc::c_int) < 254i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"m\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            m as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if (y as libc::c_int) < 254i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"y\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            y as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if row < halfway {
                        if (k as libc::c_int) < 254i32 {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"k\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                k as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            current_block = 10090817204669828264;
                            break 's_40;
                        }
                    } else if k as libc::c_int > 1i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"k\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            k as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    }
                } else if (c as libc::c_int) < 254i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"c\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        c as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if y as libc::c_int > 1i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"y\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        y as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if (k as libc::c_int) < 254i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"k\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        k as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if row < halfway {
                    if m as libc::c_int > 1i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"m\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            m as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    }
                } else if (m as libc::c_int) < 254i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"m\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        m as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 10090817204669828264;
                    break 's_40;
                }
                col += 1
            }
            row += 1
        }
        match current_block {
            10090817204669828264 => {}
            _ => return 1i32,
        }
    } else {
        row = 0i32;
        's_400: while row < h {
            col = 0i32;
            while col < w {
                let mut r: libc::c_uchar = 0;
                let mut g: libc::c_uchar = 0;
                let mut b: libc::c_uchar = 0;
                let mut a: libc::c_uchar = 0;
                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                r = *buf.offset((index * ps + roffset) as isize);
                g = *buf.offset((index * ps + goffset) as isize);
                b = *buf.offset((index * ps + boffset) as isize);
                a = (if aoffset >= 0i32 {
                    *buf.offset((index * ps + aoffset) as isize) as libc::c_int
                } else {
                    0xffi32
                }) as libc::c_uchar;
                if (row / blocksize + col / blocksize) % 2i32 == 0i32 {
                    if row < halfway {
                        if (r as libc::c_int) < 254i32 {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"r\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                r as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (g as libc::c_int) < 254i32 {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"g\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                g as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (b as libc::c_int) < 254i32 {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"b\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                b as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        }
                    } else if r as libc::c_int > 1i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"r\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            r as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if g as libc::c_int > 1i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"g\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            g as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if b as libc::c_int > 1i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"b\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            b as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    }
                } else if subsamp == crate::turbojpeg::TJSAMP_GRAY as libc::c_int {
                    if row < halfway {
                        if (r as libc::c_int) < 76i32 - 1i32 || r as libc::c_int > 76i32 + 1i32 {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"r\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                76i32,
                                r as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (g as libc::c_int) < 76i32 - 1i32
                            || g as libc::c_int > 76i32 + 1i32
                        {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"g\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                76i32,
                                g as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (b as libc::c_int) < 76i32 - 1i32
                            || b as libc::c_int > 76i32 + 1i32
                        {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"b\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                76i32,
                                b as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        }
                    } else if (r as libc::c_int) < 226i32 - 1i32 || r as libc::c_int > 226i32 + 1i32
                    {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"r\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            226i32,
                            r as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if (g as libc::c_int) < 226i32 - 1i32 || g as libc::c_int > 226i32 + 1i32
                    {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"g\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            226i32,
                            g as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if (b as libc::c_int) < 226i32 - 1i32 || b as libc::c_int > 226i32 + 1i32
                    {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"b\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            226i32,
                            b as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    }
                } else if row < halfway {
                    if (r as libc::c_int) < 254i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"r\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            r as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if g as libc::c_int > 1i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"g\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            g as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if b as libc::c_int > 1i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"b\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            b as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    }
                } else if (r as libc::c_int) < 254i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"r\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        r as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    break 's_400;
                } else if (g as libc::c_int) < 254i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"g\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        g as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    break 's_400;
                } else if b as libc::c_int > 1i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"b\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        b as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    break 's_400;
                }
                if (a as libc::c_int) < 254i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"a\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        a as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    break 's_400;
                } else {
                    col += 1
                }
            }
            row += 1
        }
    }
    if retval == 0i32 {
        row = 0i32;
        while row < h {
            col = 0i32;
            while col < w {
                if pf == crate::turbojpeg::TJPF_CMYK as libc::c_int {
                    crate::stdlib::printf(
                        b"%.3d/%.3d/%.3d/%.3d \x00" as *const u8 as *const libc::c_char,
                        *buf.offset(((row * w + col) * ps) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + 1i32) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + 2i32) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + 3i32) as isize) as libc::c_int,
                    );
                } else {
                    crate::stdlib::printf(
                        b"%.3d/%.3d/%.3d \x00" as *const u8 as *const libc::c_char,
                        *buf.offset(((row * w + col) * ps + roffset) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + goffset) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + boffset) as isize) as libc::c_int,
                    );
                }
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn checkBufYUV(
    mut buf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut subsamp: libc::c_int,
    mut sf: crate::turbojpeg::tjscalingfactor,
) -> libc::c_int {
    let mut current_block: u64;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut hsf: libc::c_int = crate::turbojpeg::tjMCUWidth[subsamp as usize] / 8i32;
    let mut vsf: libc::c_int = crate::turbojpeg::tjMCUHeight[subsamp as usize] / 8i32;
    let mut pw: libc::c_int = w + hsf - 1i32 & !(hsf - 1i32);
    let mut ph: libc::c_int = h + vsf - 1i32 & !(vsf - 1i32);
    let mut cw: libc::c_int = pw / hsf;
    let mut ch: libc::c_int = ph / vsf;
    let mut ypitch: libc::c_int = pw + pad - 1i32 & !(pad - 1i32);
    let mut uvpitch: libc::c_int = cw + pad - 1i32 & !(pad - 1i32);
    let mut retval: libc::c_int = 1i32;
    let mut halfway: libc::c_int = 16i32 * sf.num / sf.denom;
    let mut blocksize: libc::c_int = 8i32 * sf.num / sf.denom;
    row = 0i32;
    's_27: loop {
        if !(row < ph) {
            current_block = 1836292691772056875;
            break;
        }
        col = 0i32;
        while col < pw {
            let mut y: libc::c_uchar = *buf.offset((ypitch * row + col) as isize);
            if (row / blocksize + col / blocksize) % 2i32 == 0i32 {
                if row < halfway {
                    if (y as libc::c_int) < 254i32 {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"y\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            y as libc::c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 17379669344980314800;
                        break 's_27;
                    }
                } else if y as libc::c_int > 1i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"y\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        y as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 17379669344980314800;
                    break 's_27;
                }
            } else if row < halfway {
                if (y as libc::c_int) < 76i32 - 1i32 || y as libc::c_int > 76i32 + 1i32 {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"y\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        76i32,
                        y as libc::c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 17379669344980314800;
                    break 's_27;
                }
            } else if (y as libc::c_int) < 226i32 - 1i32 || y as libc::c_int > 226i32 + 1i32 {
                crate::stdlib::printf(
                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                        as *const libc::c_char,
                    b"y\x00" as *const u8 as *const libc::c_char,
                    row,
                    col,
                    226i32,
                    y as libc::c_int,
                );
                retval = 0i32;
                exitStatus = -1i32;
                current_block = 17379669344980314800;
                break 's_27;
            }
            col += 1
        }
        row += 1
    }
    match current_block {
        1836292691772056875 => {
            if subsamp != crate::turbojpeg::TJSAMP_GRAY as libc::c_int {
                let mut halfway_0: libc::c_int = 16i32 / vsf * sf.num / sf.denom;
                row = 0i32;
                's_193: while row < ch {
                    col = 0i32;
                    while col < cw {
                        let mut u: libc::c_uchar =
                            *buf.offset((ypitch * ph + (uvpitch * row + col)) as isize);
                        let mut v: libc::c_uchar = *buf
                            .offset((ypitch * ph + uvpitch * ch + (uvpitch * row + col)) as isize);
                        if (row * vsf / blocksize + col * hsf / blocksize) % 2i32 == 0i32 {
                            if (u as libc::c_int) < 128i32 - 1i32
                                || u as libc::c_int > 128i32 + 1i32
                            {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"u\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    128i32,
                                    u as libc::c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            } else if (v as libc::c_int) < 128i32 - 1i32
                                || v as libc::c_int > 128i32 + 1i32
                            {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"v\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    128i32,
                                    v as libc::c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            }
                        } else if row < halfway_0 {
                            if (u as libc::c_int) < 85i32 - 1i32 || u as libc::c_int > 85i32 + 1i32
                            {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"u\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    85i32,
                                    u as libc::c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            } else if (v as libc::c_int) < 254i32 {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"v\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    v as libc::c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            }
                        } else if u as libc::c_int > 1i32 {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"u\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                u as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_193;
                        } else if (v as libc::c_int) < 149i32 - 1i32
                            || v as libc::c_int > 149i32 + 1i32
                        {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"v\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                149i32,
                                v as libc::c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_193;
                        }
                        col += 1
                    }
                    row += 1
                }
            }
        }
        _ => {}
    }
    if retval == 0i32 {
        row = 0i32;
        while row < ph {
            col = 0i32;
            while col < pw {
                crate::stdlib::printf(
                    b"%.3d \x00" as *const u8 as *const libc::c_char,
                    *buf.offset((ypitch * row + col) as isize) as libc::c_int,
                );
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
        row = 0i32;
        while row < ch {
            col = 0i32;
            while col < cw {
                crate::stdlib::printf(
                    b"%.3d \x00" as *const u8 as *const libc::c_char,
                    *buf.offset((ypitch * ph + (uvpitch * row + col)) as isize) as libc::c_int,
                );
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
        row = 0i32;
        while row < ch {
            col = 0i32;
            while col < cw {
                crate::stdlib::printf(
                    b"%.3d \x00" as *const u8 as *const libc::c_char,
                    *buf.offset((ypitch * ph + uvpitch * ch + (uvpitch * row + col)) as isize)
                        as libc::c_int,
                );
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn writeJPEG(
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut filename: *mut libc::c_char,
) {
    let mut file: *mut crate::stdlib::FILE =
        crate::stdlib::fopen(filename, b"wb\x00" as *const u8 as *const libc::c_char);
    if file.is_null()
        || crate::stdlib::fwrite(
            jpegBuf as *const libc::c_void,
            jpegSize,
            1i32 as libc::c_ulong,
            file,
        ) != 1i32 as libc::c_ulong
    {
        crate::stdlib::printf(
            b"ERROR: Could not write to %s.\n%s\n\x00" as *const u8 as *const libc::c_char,
            filename,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        exitStatus = -1i32
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
    };
}
#[no_mangle]
pub unsafe extern "C" fn compTest(
    mut handle: crate::turbojpeg::tjhandle,
    mut dstBuf: *mut *mut libc::c_uchar,
    mut dstSize: *mut libc::c_ulong,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut basename: *mut libc::c_char,
    mut subsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut current_block: u64;
    let mut tempStr: [libc::c_char; 1024] = [0; 1024];
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut pfStr: *const libc::c_char = pixFormatStr[pf as usize];
    let mut buStrLong: *const libc::c_char = if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
        b"Bottom-Up\x00" as *const u8 as *const libc::c_char
    } else {
        b"Top-Down \x00" as *const u8 as *const libc::c_char
    };
    let mut buStr: *const libc::c_char = if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
        b"BU\x00" as *const u8 as *const libc::c_char
    } else {
        b"TD\x00" as *const u8 as *const libc::c_char
    };
    srcBuf = crate::stdlib::malloc(
        (w * h * crate::turbojpeg::tjPixelSize[pf as usize]) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if srcBuf.is_null() {
        crate::stdlib::printf(
            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
            b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
        );
        exitStatus = -1i32
    } else {
        initBuf(srcBuf, w, h, pf, flags);
        if !(*dstBuf).is_null() && *dstSize > 0i32 as libc::c_ulong {
            crate::stdlib::memset(*dstBuf as *mut libc::c_void, 0i32, *dstSize);
        }
        if 0 == alloc {
            flags |= crate::turbojpeg::TJFLAG_NOREALLOC
        }
        if 0 != doYUV {
            let mut yuvSize: libc::c_ulong = crate::turbojpeg::tjBufSizeYUV2(w, pad, h, subsamp);
            let mut sf: crate::turbojpeg::tjscalingfactor = crate::turbojpeg::tjscalingfactor {
                num: 1i32,
                denom: 1i32,
            };
            let mut handle2: crate::turbojpeg::tjhandle = crate::turbojpeg::tjInitCompress();
            if handle2.is_null() {
                crate::stdlib::printf(
                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg::tjGetErrorStr(),
                );
                exitStatus = -1i32;
                current_block = 860601949763470011;
            } else {
                yuvBuf = crate::stdlib::malloc(yuvSize) as *mut libc::c_uchar;
                if yuvBuf.is_null() {
                    crate::stdlib::printf(
                        b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                        b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
                    );
                    exitStatus = -1i32;
                    current_block = 860601949763470011;
                } else {
                    crate::stdlib::memset(yuvBuf as *mut libc::c_void, 0i32, yuvSize);
                    crate::stdlib::printf(
                        b"%s %s -> YUV %s ... \x00" as *const u8 as *const libc::c_char,
                        pfStr,
                        buStrLong,
                        subNameLong[subsamp as usize],
                    );
                    if crate::turbojpeg::tjEncodeYUV3(
                        handle2, srcBuf, w, 0i32, h, pf, yuvBuf, pad, subsamp, flags,
                    ) == -1i32
                    {
                        crate::stdlib::printf(
                            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                            crate::turbojpeg::tjGetErrorStr(),
                        );
                        exitStatus = -1i32;
                        current_block = 860601949763470011;
                    } else {
                        crate::turbojpeg::tjDestroy(handle2);
                        if 0 != checkBufYUV(yuvBuf, w, h, subsamp, sf) {
                            crate::stdlib::printf(
                                b"Passed.\n\x00" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            crate::stdlib::printf(
                                b"FAILED!\n\x00" as *const u8 as *const libc::c_char,
                            );
                        }
                        crate::stdlib::printf(
                            b"YUV %s %s -> JPEG Q%d ... \x00" as *const u8 as *const libc::c_char,
                            subNameLong[subsamp as usize],
                            buStrLong,
                            jpegQual,
                        );
                        if crate::turbojpeg::tjCompressFromYUV(
                            handle, yuvBuf, w, pad, h, subsamp, dstBuf, dstSize, jpegQual, flags,
                        ) == -1i32
                        {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg::tjGetErrorStr(),
                            );
                            exitStatus = -1i32;
                            current_block = 860601949763470011;
                        } else {
                            current_block = 10095721787123848864;
                        }
                    }
                }
            }
        } else {
            crate::stdlib::printf(
                b"%s %s -> %s Q%d ... \x00" as *const u8 as *const libc::c_char,
                pfStr,
                buStrLong,
                subNameLong[subsamp as usize],
                jpegQual,
            );
            if crate::turbojpeg::tjCompress2(
                handle, srcBuf, w, 0i32, h, pf, dstBuf, dstSize, subsamp, jpegQual, flags,
            ) == -1i32
            {
                crate::stdlib::printf(
                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg::tjGetErrorStr(),
                );
                exitStatus = -1i32;
                current_block = 860601949763470011;
            } else {
                current_block = 10095721787123848864;
            }
        }
        match current_block {
            860601949763470011 => {}
            _ => {
                crate::stdlib::snprintf(
                    tempStr.as_mut_ptr(),
                    1024i32 as libc::c_ulong,
                    b"%s_enc_%s_%s_%s_Q%d.jpg\x00" as *const u8 as *const libc::c_char,
                    basename,
                    pfStr,
                    buStr,
                    subName[subsamp as usize],
                    jpegQual,
                );
                writeJPEG(*dstBuf, *dstSize, tempStr.as_mut_ptr());
                crate::stdlib::printf(
                    b"Done.\n  Result in %s\n\x00" as *const u8 as *const libc::c_char,
                    tempStr.as_mut_ptr(),
                );
            }
        }
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
    }
    if !srcBuf.is_null() {
        crate::stdlib::free(srcBuf as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _decompTest(
    mut handle: crate::turbojpeg::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut basename: *mut libc::c_char,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
    mut sf: crate::turbojpeg::tjscalingfactor,
) {
    let mut current_block: u64;
    let mut dstBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut _hdrw: libc::c_int = 0i32;
    let mut _hdrh: libc::c_int = 0i32;
    let mut _hdrsubsamp: libc::c_int = -1i32;
    let mut scaledWidth: libc::c_int = (w * sf.num + sf.denom - 1i32) / sf.denom;
    let mut scaledHeight: libc::c_int = (h * sf.num + sf.denom - 1i32) / sf.denom;
    let mut dstSize: libc::c_ulong = 0i32 as libc::c_ulong;
    if crate::turbojpeg::tjDecompressHeader2(
        handle,
        jpegBuf,
        jpegSize,
        &mut _hdrw,
        &mut _hdrh,
        &mut _hdrsubsamp,
    ) == -1i32
    {
        crate::stdlib::printf(
            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg::tjGetErrorStr(),
        );
        exitStatus = -1i32
    } else if _hdrw != w || _hdrh != h || _hdrsubsamp != subsamp {
        crate::stdlib::printf(
            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
            b"Incorrect JPEG header\x00" as *const u8 as *const libc::c_char,
        );
        exitStatus = -1i32
    } else {
        dstSize = (scaledWidth * scaledHeight * crate::turbojpeg::tjPixelSize[pf as usize])
            as libc::c_ulong;
        dstBuf = crate::stdlib::malloc(dstSize) as *mut libc::c_uchar;
        if dstBuf.is_null() {
            crate::stdlib::printf(
                b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
            );
            exitStatus = -1i32
        } else {
            crate::stdlib::memset(dstBuf as *mut libc::c_void, 0i32, dstSize);
            if 0 != doYUV {
                let mut yuvSize: libc::c_ulong =
                    crate::turbojpeg::tjBufSizeYUV2(scaledWidth, pad, scaledHeight, subsamp);
                let mut handle2: crate::turbojpeg::tjhandle = crate::turbojpeg::tjInitDecompress();
                if handle2.is_null() {
                    crate::stdlib::printf(
                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        crate::turbojpeg::tjGetErrorStr(),
                    );
                    exitStatus = -1i32;
                    current_block = 14297773496329963019;
                } else {
                    yuvBuf = crate::stdlib::malloc(yuvSize) as *mut libc::c_uchar;
                    if yuvBuf.is_null() {
                        crate::stdlib::printf(
                            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                            b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
                        );
                        exitStatus = -1i32;
                        current_block = 14297773496329963019;
                    } else {
                        crate::stdlib::memset(yuvBuf as *mut libc::c_void, 0i32, yuvSize);
                        crate::stdlib::printf(
                            b"JPEG -> YUV %s \x00" as *const u8 as *const libc::c_char,
                            subNameLong[subsamp as usize],
                        );
                        if sf.num != 1i32 || sf.denom != 1i32 {
                            crate::stdlib::printf(
                                b"%d/%d ... \x00" as *const u8 as *const libc::c_char,
                                sf.num,
                                sf.denom,
                            );
                        } else {
                            crate::stdlib::printf(b"... \x00" as *const u8 as *const libc::c_char);
                        }
                        if crate::turbojpeg::tjDecompressToYUV2(
                            handle,
                            jpegBuf,
                            jpegSize,
                            yuvBuf,
                            scaledWidth,
                            pad,
                            scaledHeight,
                            flags,
                        ) == -1i32
                        {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg::tjGetErrorStr(),
                            );
                            exitStatus = -1i32;
                            current_block = 14297773496329963019;
                        } else {
                            if 0 != checkBufYUV(yuvBuf, scaledWidth, scaledHeight, subsamp, sf) {
                                crate::stdlib::printf(
                                    b"Passed.\n\x00" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                crate::stdlib::printf(
                                    b"FAILED!\n\x00" as *const u8 as *const libc::c_char,
                                );
                            }
                            crate::stdlib::printf(
                                b"YUV %s -> %s %s ... \x00" as *const u8 as *const libc::c_char,
                                subNameLong[subsamp as usize],
                                pixFormatStr[pf as usize],
                                if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                                    b"Bottom-Up\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"Top-Down \x00" as *const u8 as *const libc::c_char
                                },
                            );
                            if crate::turbojpeg::tjDecodeYUV(
                                handle2,
                                yuvBuf,
                                pad,
                                subsamp,
                                dstBuf,
                                scaledWidth,
                                0i32,
                                scaledHeight,
                                pf,
                                flags,
                            ) == -1i32
                            {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg::tjGetErrorStr(),
                                );
                                exitStatus = -1i32;
                                current_block = 14297773496329963019;
                            } else {
                                crate::turbojpeg::tjDestroy(handle2);
                                current_block = 15594839951440953787;
                            }
                        }
                    }
                }
            } else {
                crate::stdlib::printf(
                    b"JPEG -> %s %s \x00" as *const u8 as *const libc::c_char,
                    pixFormatStr[pf as usize],
                    if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                        b"Bottom-Up\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"Top-Down \x00" as *const u8 as *const libc::c_char
                    },
                );
                if sf.num != 1i32 || sf.denom != 1i32 {
                    crate::stdlib::printf(
                        b"%d/%d ... \x00" as *const u8 as *const libc::c_char,
                        sf.num,
                        sf.denom,
                    );
                } else {
                    crate::stdlib::printf(b"... \x00" as *const u8 as *const libc::c_char);
                }
                if crate::turbojpeg::tjDecompress2(
                    handle,
                    jpegBuf,
                    jpegSize,
                    dstBuf,
                    scaledWidth,
                    0i32,
                    scaledHeight,
                    pf,
                    flags,
                ) == -1i32
                {
                    crate::stdlib::printf(
                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        crate::turbojpeg::tjGetErrorStr(),
                    );
                    exitStatus = -1i32;
                    current_block = 14297773496329963019;
                } else {
                    current_block = 15594839951440953787;
                }
            }
            match current_block {
                14297773496329963019 => {}
                _ => {
                    if 0 != checkBuf(dstBuf, scaledWidth, scaledHeight, pf, subsamp, sf, flags) {
                        crate::stdlib::printf(b"Passed.\x00" as *const u8 as *const libc::c_char);
                    } else {
                        crate::stdlib::printf(b"FAILED!\x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
    }
    if !dstBuf.is_null() {
        crate::stdlib::free(dstBuf as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn decompTest(
    mut handle: crate::turbojpeg::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut basename: *mut libc::c_char,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0i32;
    let mut sf: *mut crate::turbojpeg::tjscalingfactor =
        crate::turbojpeg::tjGetScalingFactors(&mut n);
    if sf.is_null() || 0 == n {
        crate::stdlib::printf(
            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg::tjGetErrorStr(),
        );
        exitStatus = -1i32
    } else {
        i = 0i32;
        while i < n {
            if subsamp == crate::turbojpeg::TJSAMP_444 as libc::c_int
                || subsamp == crate::turbojpeg::TJSAMP_GRAY as libc::c_int
                || subsamp == crate::turbojpeg::TJSAMP_411 as libc::c_int
                    && (*sf.offset(i as isize)).num == 1i32
                    && ((*sf.offset(i as isize)).denom == 2i32
                        || (*sf.offset(i as isize)).denom == 1i32)
                || subsamp != crate::turbojpeg::TJSAMP_411 as libc::c_int
                    && (*sf.offset(i as isize)).num == 1i32
                    && ((*sf.offset(i as isize)).denom == 4i32
                        || (*sf.offset(i as isize)).denom == 2i32
                        || (*sf.offset(i as isize)).denom == 1i32)
            {
                _decompTest(
                    handle,
                    jpegBuf,
                    jpegSize,
                    w,
                    h,
                    pf,
                    basename,
                    subsamp,
                    flags,
                    *sf.offset(i as isize),
                );
            }
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn doTest(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut formats: *const libc::c_int,
    mut nformats: libc::c_int,
    mut subsamp: libc::c_int,
    mut basename: *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut chandle: crate::turbojpeg::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut dhandle: crate::turbojpeg::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut dstBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut size: libc::c_ulong = 0i32 as libc::c_ulong;
    let mut pfi: libc::c_int = 0;
    let mut pf: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if 0 == alloc {
        size = crate::turbojpeg::tjBufSize(w, h, subsamp)
    }
    if size != 0i32 as libc::c_ulong {
        dstBuf = crate::turbojpeg::tjAlloc(size as libc::c_int);
        if dstBuf.is_null() {
            crate::stdlib::printf(
                b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                b"Memory allocation failure.\x00" as *const u8 as *const libc::c_char,
            );
            exitStatus = -1i32;
            current_block = 15679700284600549218;
        } else {
            current_block = 1394248824506584008;
        }
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            chandle = crate::turbojpeg::tjInitCompress();
            if chandle.is_null() || {
                dhandle = crate::turbojpeg::tjInitDecompress();
                dhandle.is_null()
            } {
                crate::stdlib::printf(
                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg::tjGetErrorStr(),
                );
                exitStatus = -1i32
            } else {
                pfi = 0i32;
                while pfi < nformats {
                    i = 0i32;
                    while i < 2i32 {
                        let mut flags: libc::c_int = 0i32;
                        if subsamp == crate::turbojpeg::TJSAMP_422 as libc::c_int
                            || subsamp == crate::turbojpeg::TJSAMP_420 as libc::c_int
                            || subsamp == crate::turbojpeg::TJSAMP_440 as libc::c_int
                            || subsamp == crate::turbojpeg::TJSAMP_411 as libc::c_int
                        {
                            flags |= crate::turbojpeg::TJFLAG_FASTUPSAMPLE
                        }
                        if i == 1i32 {
                            flags |= crate::turbojpeg::TJFLAG_BOTTOMUP
                        }
                        pf = *formats.offset(pfi as isize);
                        compTest(
                            chandle,
                            &mut dstBuf,
                            &mut size,
                            w,
                            h,
                            pf,
                            basename,
                            subsamp,
                            100i32,
                            flags,
                        );
                        decompTest(dhandle, dstBuf, size, w, h, pf, basename, subsamp, flags);
                        if pf >= crate::turbojpeg::TJPF_RGBX as libc::c_int
                            && pf <= crate::turbojpeg::TJPF_XRGB as libc::c_int
                        {
                            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                            decompTest(
                                dhandle,
                                dstBuf,
                                size,
                                w,
                                h,
                                pf + (crate::turbojpeg::TJPF_RGBA as libc::c_int
                                    - crate::turbojpeg::TJPF_RGBX as libc::c_int),
                                basename,
                                subsamp,
                                flags,
                            );
                        }
                        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                        i += 1
                    }
                    pfi += 1
                }
                crate::stdlib::printf(
                    b"--------------------\n\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        _ => {}
    }
    if !chandle.is_null() {
        crate::turbojpeg::tjDestroy(chandle);
    }
    if !dhandle.is_null() {
        crate::turbojpeg::tjDestroy(dhandle);
    }
    if !dstBuf.is_null() {
        crate::turbojpeg::tjFree(dstBuf);
    };
}
#[no_mangle]
pub unsafe extern "C" fn bufSizeTest() {
    let mut current_block: u64;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut subsamp: libc::c_int = 0;
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut dstBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut handle: crate::turbojpeg::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut dstSize: libc::c_ulong = 0i32 as libc::c_ulong;
    handle = crate::turbojpeg::tjInitCompress();
    if handle.is_null() {
        crate::stdlib::printf(
            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg::tjGetErrorStr(),
        );
        exitStatus = -1i32
    } else {
        crate::stdlib::printf(
            b"Buffer size regression test\n\x00" as *const u8 as *const libc::c_char,
        );
        subsamp = 0i32;
        's_43: loop {
            if !(subsamp < crate::turbojpeg::TJ_NUMSAMP) {
                current_block = 6040267449472925966;
                break;
            }
            w = 1i32;
            while w < 48i32 {
                let mut maxh: libc::c_int = if w == 1i32 { 2048i32 } else { 48i32 };
                h = 1i32;
                while h < maxh {
                    if h % 100i32 == 0i32 {
                        crate::stdlib::printf(
                            b"%.4d x %.4d\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x00"
                                as *const u8 as *const libc::c_char,
                            w,
                            h,
                        );
                    }
                    srcBuf = crate::stdlib::malloc((w * h * 4i32) as libc::c_ulong)
                        as *mut libc::c_uchar;
                    if srcBuf.is_null() {
                        crate::stdlib::printf(
                            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                            b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
                        );
                        exitStatus = -1i32;
                        current_block = 17868673386502678986;
                        break 's_43;
                    } else {
                        if 0 == alloc || 0 != doYUV {
                            if 0 != doYUV {
                                dstSize = crate::turbojpeg::tjBufSizeYUV2(w, pad, h, subsamp)
                            } else {
                                dstSize = crate::turbojpeg::tjBufSize(w, h, subsamp)
                            }
                            dstBuf = crate::turbojpeg::tjAlloc(dstSize as libc::c_int);
                            if dstBuf.is_null() {
                                crate::stdlib::printf(
                                    b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                                    b"Memory allocation failure\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                exitStatus = -1i32;
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                        }
                        i = 0i32;
                        while i < w * h * 4i32 {
                            if crate::stdlib::random()
                                < (crate::stdlib::RAND_MAX / 2i32) as libc::c_long
                            {
                                *srcBuf.offset(i as isize) = 0i32 as libc::c_uchar
                            } else {
                                *srcBuf.offset(i as isize) = 255i32 as libc::c_uchar
                            }
                            i += 1
                        }
                        if 0 != doYUV {
                            if crate::turbojpeg::tjEncodeYUV3(
                                handle,
                                srcBuf,
                                w,
                                0i32,
                                h,
                                crate::turbojpeg::TJPF_BGRX as libc::c_int,
                                dstBuf,
                                pad,
                                subsamp,
                                0i32,
                            ) == -1i32
                            {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg::tjGetErrorStr(),
                                );
                                exitStatus = -1i32;
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                        } else if crate::turbojpeg::tjCompress2(
                            handle,
                            srcBuf,
                            w,
                            0i32,
                            h,
                            crate::turbojpeg::TJPF_BGRX as libc::c_int,
                            &mut dstBuf,
                            &mut dstSize,
                            subsamp,
                            100i32,
                            (if 0 != alloc { 0i32 } else { 1024i32 }),
                        ) == -1i32
                        {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg::tjGetErrorStr(),
                            );
                            exitStatus = -1i32;
                            current_block = 17868673386502678986;
                            break 's_43;
                        }
                        crate::stdlib::free(srcBuf as *mut libc::c_void);
                        srcBuf = crate::stddef_h::NULL as *mut libc::c_uchar;
                        if 0 == alloc || 0 != doYUV {
                            crate::turbojpeg::tjFree(dstBuf);
                            dstBuf = crate::stddef_h::NULL as *mut libc::c_uchar
                        }
                        srcBuf = crate::stdlib::malloc((h * w * 4i32) as libc::c_ulong)
                            as *mut libc::c_uchar;
                        if srcBuf.is_null() {
                            crate::stdlib::printf(
                                b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                                b"Memory allocation failure\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            exitStatus = -1i32;
                            current_block = 17868673386502678986;
                            break 's_43;
                        } else {
                            if 0 == alloc || 0 != doYUV {
                                if 0 != doYUV {
                                    dstSize = crate::turbojpeg::tjBufSizeYUV2(h, pad, w, subsamp)
                                } else {
                                    dstSize = crate::turbojpeg::tjBufSize(h, w, subsamp)
                                }
                                dstBuf = crate::turbojpeg::tjAlloc(dstSize as libc::c_int);
                                if dstBuf.is_null() {
                                    crate::stdlib::printf(
                                        b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                                        b"Memory allocation failure\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    exitStatus = -1i32;
                                    current_block = 17868673386502678986;
                                    break 's_43;
                                }
                            }
                            i = 0i32;
                            while i < h * w * 4i32 {
                                if crate::stdlib::random()
                                    < (crate::stdlib::RAND_MAX / 2i32) as libc::c_long
                                {
                                    *srcBuf.offset(i as isize) = 0i32 as libc::c_uchar
                                } else {
                                    *srcBuf.offset(i as isize) = 255i32 as libc::c_uchar
                                }
                                i += 1
                            }
                            if 0 != doYUV {
                                if crate::turbojpeg::tjEncodeYUV3(
                                    handle,
                                    srcBuf,
                                    h,
                                    0i32,
                                    w,
                                    crate::turbojpeg::TJPF_BGRX as libc::c_int,
                                    dstBuf,
                                    pad,
                                    subsamp,
                                    0i32,
                                ) == -1i32
                                {
                                    crate::stdlib::printf(
                                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                            as *const libc::c_char,
                                        crate::turbojpeg::tjGetErrorStr(),
                                    );
                                    exitStatus = -1i32;
                                    current_block = 17868673386502678986;
                                    break 's_43;
                                }
                            } else if crate::turbojpeg::tjCompress2(
                                handle,
                                srcBuf,
                                h,
                                0i32,
                                w,
                                crate::turbojpeg::TJPF_BGRX as libc::c_int,
                                &mut dstBuf,
                                &mut dstSize,
                                subsamp,
                                100i32,
                                (if 0 != alloc { 0i32 } else { 1024i32 }),
                            ) == -1i32
                            {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg::tjGetErrorStr(),
                                );
                                exitStatus = -1i32;
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                            crate::stdlib::free(srcBuf as *mut libc::c_void);
                            srcBuf = crate::stddef_h::NULL as *mut libc::c_uchar;
                            if 0 == alloc || 0 != doYUV {
                                crate::turbojpeg::tjFree(dstBuf);
                                dstBuf = crate::stddef_h::NULL as *mut libc::c_uchar
                            }
                            h += 1
                        }
                    }
                }
                w += 1
            }
            subsamp += 1
        }
        match current_block {
            17868673386502678986 => {}
            _ => {
                crate::stdlib::printf(b"Done.      \n\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    if !srcBuf.is_null() {
        crate::stdlib::free(srcBuf as *mut libc::c_void);
    }
    if !dstBuf.is_null() {
        crate::turbojpeg::tjFree(dstBuf);
    }
    if !handle.is_null() {
        crate::turbojpeg::tjDestroy(handle);
    };
}
#[no_mangle]
pub unsafe extern "C" fn initBitmap(
    mut buf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut roffset: libc::c_int = crate::turbojpeg::tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = crate::turbojpeg::tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = crate::turbojpeg::tjBlueOffset[pf as usize];
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0i32;
    while j < height {
        let mut row: libc::c_int = if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
            height - j - 1i32
        } else {
            j
        };
        i = 0i32;
        while i < width {
            let mut r: libc::c_uchar = (i * 256i32 / width % 256i32) as libc::c_uchar;
            let mut g: libc::c_uchar = (j * 256i32 / height % 256i32) as libc::c_uchar;
            let mut b: libc::c_uchar =
                ((j * 256i32 / height + i * 256i32 / width) % 256i32) as libc::c_uchar;
            crate::stdlib::memset(
                &mut *buf.offset((row * pitch + i * ps) as isize) as *mut libc::c_uchar
                    as *mut libc::c_void,
                0i32,
                ps as libc::c_ulong,
            );
            if pf == crate::turbojpeg::TJPF_GRAY as libc::c_int {
                *buf.offset((row * pitch + i * ps) as isize) = b
            } else if pf == crate::turbojpeg::TJPF_CMYK as libc::c_int {
                crate::cmyk_h::rgb_to_cmyk(
                    r,
                    g,
                    b,
                    &mut *buf.offset((row * pitch + i * ps + 0i32) as isize),
                    &mut *buf.offset((row * pitch + i * ps + 1i32) as isize),
                    &mut *buf.offset((row * pitch + i * ps + 2i32) as isize),
                    &mut *buf.offset((row * pitch + i * ps + 3i32) as isize),
                );
            } else {
                *buf.offset((row * pitch + i * ps + roffset) as isize) = r;
                *buf.offset((row * pitch + i * ps + goffset) as isize) = g;
                *buf.offset((row * pitch + i * ps + boffset) as isize) = b
            }
            i += 1
        }
        j += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmpBitmap(
    mut buf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
    mut gray2rgb: libc::c_int,
) -> libc::c_int {
    let mut roffset: libc::c_int = crate::turbojpeg::tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = crate::turbojpeg::tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = crate::turbojpeg::tjBlueOffset[pf as usize];
    let mut aoffset: libc::c_int = crate::turbojpeg::tjAlphaOffset[pf as usize];
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0i32;
    while j < height {
        let mut row: libc::c_int = if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
            height - j - 1i32
        } else {
            j
        };
        i = 0i32;
        while i < width {
            let mut r: libc::c_uchar = (i * 256i32 / width % 256i32) as libc::c_uchar;
            let mut g: libc::c_uchar = (j * 256i32 / height % 256i32) as libc::c_uchar;
            let mut b: libc::c_uchar =
                ((j * 256i32 / height + i * 256i32 / width) % 256i32) as libc::c_uchar;
            if pf == crate::turbojpeg::TJPF_GRAY as libc::c_int {
                if *buf.offset((row * pitch + i * ps) as isize) as libc::c_int != b as libc::c_int {
                    return 0i32;
                }
            } else if pf == crate::turbojpeg::TJPF_CMYK as libc::c_int {
                let mut rf: libc::c_uchar = 0;
                let mut gf: libc::c_uchar = 0;
                let mut bf: libc::c_uchar = 0;
                crate::cmyk_h::cmyk_to_rgb(
                    *buf.offset((row * pitch + i * ps + 0i32) as isize),
                    *buf.offset((row * pitch + i * ps + 1i32) as isize),
                    *buf.offset((row * pitch + i * ps + 2i32) as isize),
                    *buf.offset((row * pitch + i * ps + 3i32) as isize),
                    &mut rf,
                    &mut gf,
                    &mut bf,
                );
                if 0 != gray2rgb {
                    if rf as libc::c_int != b as libc::c_int
                        || gf as libc::c_int != b as libc::c_int
                        || bf as libc::c_int != b as libc::c_int
                    {
                        return 0i32;
                    }
                } else if rf as libc::c_int != r as libc::c_int
                    || gf as libc::c_int != g as libc::c_int
                    || bf as libc::c_int != b as libc::c_int
                {
                    return 0i32;
                }
            } else {
                if 0 != gray2rgb {
                    if *buf.offset((row * pitch + i * ps + roffset) as isize) as libc::c_int
                        != b as libc::c_int
                        || *buf.offset((row * pitch + i * ps + goffset) as isize) as libc::c_int
                            != b as libc::c_int
                        || *buf.offset((row * pitch + i * ps + boffset) as isize) as libc::c_int
                            != b as libc::c_int
                    {
                        return 0i32;
                    }
                } else if *buf.offset((row * pitch + i * ps + roffset) as isize) as libc::c_int
                    != r as libc::c_int
                    || *buf.offset((row * pitch + i * ps + goffset) as isize) as libc::c_int
                        != g as libc::c_int
                    || *buf.offset((row * pitch + i * ps + boffset) as isize) as libc::c_int
                        != b as libc::c_int
                {
                    return 0i32;
                }
                if aoffset >= 0i32
                    && *buf.offset((row * pitch + i * ps + aoffset) as isize) as libc::c_int
                        != 0xffi32
                {
                    return 0i32;
                }
            }
            i += 1
        }
        j += 1
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn doBmpTest(
    mut ext: *const libc::c_char,
    mut width: libc::c_int,
    mut align: libc::c_int,
    mut height: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut filename: [libc::c_char; 80] = [0; 80];
    let mut md5sum: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut md5buf: [libc::c_char; 65] = [0; 65];
    let mut ps: libc::c_int = crate::turbojpeg::tjPixelSize[pf as usize];
    let mut pitch: libc::c_int = width * ps + align - 1i32 & !(align - 1i32);
    let mut loadWidth: libc::c_int = 0i32;
    let mut loadHeight: libc::c_int = 0i32;
    let mut retval: libc::c_int = 0i32;
    let mut pixelFormat: libc::c_int = pf;
    let mut buf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut md5ref: *mut libc::c_char = 0 as *mut libc::c_char;
    if pf == crate::turbojpeg::TJPF_GRAY as libc::c_int {
        md5ref = (if 0
            == crate::stdlib::strcasecmp(ext, b"ppm\x00" as *const u8 as *const libc::c_char)
        {
            b"112c682e82ce5de1cca089e20d60000b\x00" as *const u8 as *const libc::c_char
        } else {
            b"51976530acf75f02beddf5d21149101d\x00" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char
    } else {
        md5ref = (if 0
            == crate::stdlib::strcasecmp(ext, b"ppm\x00" as *const u8 as *const libc::c_char)
        {
            b"c0c9f772b464d1896326883a5c79c545\x00" as *const u8 as *const libc::c_char
        } else {
            b"6d659071b9bfcdee2def22cb58ddadca\x00" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char
    }
    buf = crate::turbojpeg::tjAlloc(pitch * height);
    if buf.is_null() {
        crate::stdlib::printf(
            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
            b"Could not allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        exitStatus = -1i32
    } else {
        initBitmap(buf, width, pitch, height, pf, flags);
        crate::stdlib::snprintf(
            filename.as_mut_ptr(),
            80i32 as libc::c_ulong,
            b"test_bmp_%s_%d_%s.%s\x00" as *const u8 as *const libc::c_char,
            pixFormatStr[pf as usize],
            align,
            if 0 != flags & crate::turbojpeg::TJFLAG_BOTTOMUP {
                b"bu\x00" as *const u8 as *const libc::c_char
            } else {
                b"td\x00" as *const u8 as *const libc::c_char
            },
            ext,
        );
        if crate::turbojpeg::tjSaveImage(
            filename.as_mut_ptr(),
            buf,
            width,
            pitch,
            height,
            pf,
            flags,
        ) == -1i32
        {
            crate::stdlib::printf(
                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                crate::turbojpeg::tjGetErrorStr(),
            );
            exitStatus = -1i32
        } else {
            md5sum = crate::md5::MD5File(filename.as_mut_ptr(), md5buf.as_mut_ptr());
            if 0 != crate::stdlib::strcasecmp(md5sum, md5ref) {
                crate::stdlib::printf(
                    b"\n%s has an MD5 sum of %s.\n   Should be %s.\n\x00" as *const u8
                        as *const libc::c_char,
                    filename.as_mut_ptr(),
                    md5sum,
                    md5ref,
                );
                exitStatus = -1i32
            } else {
                crate::turbojpeg::tjFree(buf);
                buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                buf = crate::turbojpeg::tjLoadImage(
                    filename.as_mut_ptr(),
                    &mut loadWidth,
                    align,
                    &mut loadHeight,
                    &mut pf,
                    flags,
                );
                if buf.is_null() {
                    crate::stdlib::printf(
                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        crate::turbojpeg::tjGetErrorStr(),
                    );
                    exitStatus = -1i32
                } else if width != loadWidth || height != loadHeight {
                    crate::stdlib::printf(
                        b"\n   Image dimensions of %s are bogus\n\x00" as *const u8
                            as *const libc::c_char,
                        filename.as_mut_ptr(),
                    );
                    retval = -1i32
                } else if 0 == cmpBitmap(buf, width, pitch, height, pf, flags, 0i32) {
                    crate::stdlib::printf(
                        b"\n   Pixel data in %s is bogus\n\x00" as *const u8 as *const libc::c_char,
                        filename.as_mut_ptr(),
                    );
                    retval = -1i32
                } else {
                    if pf == crate::turbojpeg::TJPF_GRAY as libc::c_int {
                        crate::turbojpeg::tjFree(buf);
                        buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                        pf = crate::turbojpeg::TJPF_XBGR as libc::c_int;
                        buf = crate::turbojpeg::tjLoadImage(
                            filename.as_mut_ptr(),
                            &mut loadWidth,
                            align,
                            &mut loadHeight,
                            &mut pf,
                            flags,
                        );
                        if buf.is_null() {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg::tjGetErrorStr(),
                            );
                            exitStatus = -1i32;
                            current_block = 14417489546151714667;
                        } else {
                            pitch = width * crate::turbojpeg::tjPixelSize[pf as usize] + align
                                - 1i32
                                & !(align - 1i32);
                            if 0 == cmpBitmap(buf, width, pitch, height, pf, flags, 1i32) {
                                crate::stdlib::printf(
                                    b"\n   Converting %s to RGB failed\n\x00" as *const u8
                                        as *const libc::c_char,
                                    filename.as_mut_ptr(),
                                );
                                retval = -1i32;
                                current_block = 14417489546151714667;
                            } else {
                                crate::turbojpeg::tjFree(buf);
                                buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                                pf = crate::turbojpeg::TJPF_CMYK as libc::c_int;
                                buf = crate::turbojpeg::tjLoadImage(
                                    filename.as_mut_ptr(),
                                    &mut loadWidth,
                                    align,
                                    &mut loadHeight,
                                    &mut pf,
                                    flags,
                                );
                                if buf.is_null() {
                                    crate::stdlib::printf(
                                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                            as *const libc::c_char,
                                        crate::turbojpeg::tjGetErrorStr(),
                                    );
                                    exitStatus = -1i32;
                                    current_block = 14417489546151714667;
                                } else {
                                    pitch = width * crate::turbojpeg::tjPixelSize[pf as usize]
                                        + align
                                        - 1i32
                                        & !(align - 1i32);
                                    if 0 == cmpBitmap(buf, width, pitch, height, pf, flags, 1i32) {
                                        crate::stdlib::printf(
                                            b"\n   Converting %s to CMYK failed\n\x00" as *const u8
                                                as *const libc::c_char,
                                            filename.as_mut_ptr(),
                                        );
                                        retval = -1i32;
                                        current_block = 14417489546151714667;
                                    } else {
                                        current_block = 12930649117290160518;
                                    }
                                }
                            }
                        }
                    } else {
                        current_block = 12930649117290160518;
                    }
                    match current_block {
                        14417489546151714667 => {}
                        _ => {
                            crate::turbojpeg::tjFree(buf);
                            buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                            pf = pixelFormat;
                            pixelFormat = crate::turbojpeg::TJPF_UNKNOWN as libc::c_int;
                            buf = crate::turbojpeg::tjLoadImage(
                                filename.as_mut_ptr(),
                                &mut loadWidth,
                                align,
                                &mut loadHeight,
                                &mut pixelFormat,
                                flags,
                            );
                            if buf.is_null() {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg::tjGetErrorStr(),
                                );
                                exitStatus = -1i32
                            } else {
                                if pf == crate::turbojpeg::TJPF_GRAY as libc::c_int
                                    && pixelFormat != crate::turbojpeg::TJPF_GRAY as libc::c_int
                                    || pf != crate::turbojpeg::TJPF_GRAY as libc::c_int
                                        && 0 == crate::stdlib::strcasecmp(
                                            ext,
                                            b"bmp\x00" as *const u8 as *const libc::c_char,
                                        )
                                        && pixelFormat != crate::turbojpeg::TJPF_BGR as libc::c_int
                                    || pf != crate::turbojpeg::TJPF_GRAY as libc::c_int
                                        && 0 == crate::stdlib::strcasecmp(
                                            ext,
                                            b"ppm\x00" as *const u8 as *const libc::c_char,
                                        )
                                        && pixelFormat != crate::turbojpeg::TJPF_RGB as libc::c_int
                                {
                                    crate::stdlib::printf(b"\n   tjLoadImage() returned unexpected pixel format: %s\n\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           pixFormatStr[pixelFormat as
                                                            usize]);
                                    retval = -1i32
                                }
                                crate::stdlib::unlink(filename.as_mut_ptr());
                            }
                        }
                    }
                }
            }
        }
    }
    if !buf.is_null() {
        crate::turbojpeg::tjFree(buf);
    }
    if exitStatus < 0i32 {
        return exitStatus;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn bmpTest() -> libc::c_int {
    let mut align: libc::c_int = 0;
    let mut width: libc::c_int = 35i32;
    let mut height: libc::c_int = 39i32;
    let mut format: libc::c_int = 0;
    align = 1i32;
    while align <= 8i32 {
        format = 0i32;
        while format < crate::turbojpeg::TJ_NUMPF {
            crate::stdlib::printf(
                b"%s Top-Down BMP (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"bmp\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                0i32,
            ) == -1i32
            {
                return -1i32;
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            crate::stdlib::printf(
                b"%s Top-Down PPM (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"ppm\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                crate::turbojpeg::TJFLAG_BOTTOMUP,
            ) == -1i32
            {
                return -1i32;
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            crate::stdlib::printf(
                b"%s Bottom-Up BMP (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"bmp\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                0i32,
            ) == -1i32
            {
                return -1i32;
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            crate::stdlib::printf(
                b"%s Bottom-Up PPM (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"ppm\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                crate::turbojpeg::TJFLAG_BOTTOMUP,
            ) == -1i32
            {
                return -1i32;
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            format += 1
        }
        align *= 2i32
    }
    return 0i32;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num4bf: libc::c_int = 5i32;
    if argc > 1i32 {
        i = 1i32;
        while i < argc {
            if 0 == crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-yuv\x00" as *const u8 as *const libc::c_char,
            ) {
                doYUV = 1i32
            } else if 0
                == crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-noyuvpad\x00" as *const u8 as *const libc::c_char,
                )
            {
                pad = 1i32
            } else if 0
                == crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-alloc\x00" as *const u8 as *const libc::c_char,
                )
            {
                alloc = 1i32
            } else if 0
                == crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-bmp\x00" as *const u8 as *const libc::c_char,
                )
            {
                return bmpTest();
            } else {
                usage(*argv.offset(0isize));
            }
            i += 1
        }
    }
    if 0 != alloc {
        crate::stdlib::printf(
            b"Testing automatic buffer allocation\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if 0 != doYUV {
        num4bf = 4i32
    }
    doTest(
        35i32,
        39i32,
        _3byteFormats.as_ptr(),
        2i32,
        crate::turbojpeg::TJSAMP_444 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39i32,
        41i32,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg::TJSAMP_444 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41i32,
        35i32,
        _3byteFormats.as_ptr(),
        2i32,
        crate::turbojpeg::TJSAMP_422 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35i32,
        39i32,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg::TJSAMP_422 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39i32,
        41i32,
        _3byteFormats.as_ptr(),
        2i32,
        crate::turbojpeg::TJSAMP_420 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41i32,
        35i32,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg::TJSAMP_420 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35i32,
        39i32,
        _3byteFormats.as_ptr(),
        2i32,
        crate::turbojpeg::TJSAMP_440 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39i32,
        41i32,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg::TJSAMP_440 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41i32,
        35i32,
        _3byteFormats.as_ptr(),
        2i32,
        crate::turbojpeg::TJSAMP_411 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35i32,
        39i32,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg::TJSAMP_411 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39i32,
        41i32,
        _onlyGray.as_ptr(),
        1i32,
        crate::turbojpeg::TJSAMP_GRAY as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41i32,
        35i32,
        _3byteFormats.as_ptr(),
        2i32,
        crate::turbojpeg::TJSAMP_GRAY as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35i32,
        39i32,
        _4byteFormats.as_ptr(),
        4i32,
        crate::turbojpeg::TJSAMP_GRAY as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    bufSizeTest();
    if 0 != doYUV {
        crate::stdlib::printf(
            b"\n--------------------\n\n\x00" as *const u8 as *const libc::c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            crate::turbojpeg::TJSAMP_444 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            crate::turbojpeg::TJSAMP_422 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            crate::turbojpeg::TJSAMP_420 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            crate::turbojpeg::TJSAMP_440 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            crate::turbojpeg::TJSAMP_411 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            crate::turbojpeg::TJSAMP_GRAY as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyGray.as_ptr(),
            1i32,
            crate::turbojpeg::TJSAMP_GRAY as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return exitStatus;
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
