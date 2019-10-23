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























































































use std::prelude::v1::*;use crate::src::md5::md5::MD5File;use crate::stdlib::{__errno_location, fclose, fopen, fwrite, memset, printf,
                    snprintf, strcasecmp, strerror, unlink};use libc::{c_int, c_ulong, c_char, c_void, c_long, c_uchar};use mozjpeg::*;pub use crate::jmorecfg_h::JSAMPLE;pub use crate::cmyk_h::{cmyk_to_rgb, rgb_to_cmyk};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE, exit, free,
                        malloc, random, RAND_MAX};pub use crate::src::turbojpeg::{tjAlloc, tjAlphaOffset, tjBlueOffset,
                                tjBufSize, tjBufSizeYUV2, tjCompress2,
                                tjCompressFromYUV, tjDecodeYUV, tjDecompress2,
                                tjDecompressHeader2, tjDecompressToYUV2,
                                tjDestroy, tjEncodeYUV3, tjFree,
                                tjGetErrorStr, tjGetScalingFactors,
                                tjGreenOffset, tjInitCompress,
                                tjInitDecompress, tjLoadImage, tjMCUHeight,
                                tjMCUWidth, tjPixelSize, tjRedOffset,
                                tjSaveImage, tjhandle, tjscalingfactor,
                                TJFLAG_BOTTOMUP, TJFLAG_FASTUPSAMPLE,
                                TJFLAG_NOREALLOC, TJPF, TJPF_ABGR, TJPF_ARGB,
                                TJPF_BGR, TJPF_BGRA, TJPF_BGRX, TJPF_CMYK,
                                TJPF_GRAY, TJPF_RGB, TJPF_RGBA, TJPF_RGBX,
                                TJPF_UNKNOWN, TJPF_XBGR, TJPF_XRGB, TJSAMP,
                                TJSAMP_411, TJSAMP_420, TJSAMP_422,
                                TJSAMP_440, TJSAMP_444, TJSAMP_GRAY, TJ_NUMPF,
                                TJ_NUMSAMP};pub use crate::stddef_h::{size_t, NULL};
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

pub unsafe extern "C" fn usage(mut progName: *mut c_char) {
    printf(
        
        b"\nUSAGE: %s [options]\n\n\x00".as_ptr() as *const c_char,
        progName,
    );
    printf(b"Options:\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-yuv = test YUV encoding/decoding support\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-noyuvpad = do not pad each line of each Y, U, and V plane to the nearest\n\x00".as_ptr() as *const c_char,
    );
    printf(b"            4-byte boundary\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-alloc = test automatic buffer allocation\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-bmp = tjLoadImage()/tjSaveImage() unit test\n\n\x00".as_ptr() as *const c_char,
    );
    exit(1i32);
}
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

pub static mut subName: [*const c_char; 6] = [
    
    b"444\x00".as_ptr() as *const c_char,
    
    b"422\x00".as_ptr() as *const c_char,
    
    b"420\x00".as_ptr() as *const c_char,
    
    b"GRAY\x00".as_ptr() as *const c_char,
    
    b"440\x00".as_ptr() as *const c_char,
    
    b"411\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut pixFormatStr: [*const c_char; 12] = [
    
    b"RGB\x00".as_ptr() as *const c_char,
    
    b"BGR\x00".as_ptr() as *const c_char,
    
    b"RGBX\x00".as_ptr() as *const c_char,
    
    b"BGRX\x00".as_ptr() as *const c_char,
    
    b"XBGR\x00".as_ptr() as *const c_char,
    
    b"XRGB\x00".as_ptr() as *const c_char,
    
    b"Grayscale\x00".as_ptr() as *const c_char,
    
    b"RGBA\x00".as_ptr() as *const c_char,
    
    b"BGRA\x00".as_ptr() as *const c_char,
    
    b"ABGR\x00".as_ptr() as *const c_char,
    
    b"ARGB\x00".as_ptr() as *const c_char,
    
    b"CMYK\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut _3byteFormats: [c_int; 2] = [
    
    TJPF_RGB,
    
    TJPF_BGR,
];
#[no_mangle]

pub static mut _4byteFormats: [c_int; 5] = [
    
    TJPF_RGBX,
    
    TJPF_BGRX,
    
    TJPF_XBGR,
    
    TJPF_XRGB,
    
    TJPF_CMYK,
];
#[no_mangle]

pub static mut _onlyGray: [c_int; 1] = [TJPF_GRAY];
#[no_mangle]

pub static mut _onlyRGB: [c_int; 1] = [TJPF_RGB];
#[no_mangle]

pub static mut doYUV: c_int = 0i32;
#[no_mangle]

pub static mut alloc: c_int = 0i32;
#[no_mangle]

pub static mut pad: c_int = 4i32;
#[no_mangle]

pub static mut exitStatus: c_int = 0i32;
#[no_mangle]

pub unsafe extern "C" fn initBuf(
    mut buf: *mut c_uchar,
    mut w: c_int,
    mut h: c_int,
    mut pf: c_int,
    mut flags: c_int,
) {
     let mut index:  c_int =  0; let mut row:  c_int =  0; let mut col:  c_int =  0; let mut halfway:  c_int =  16i32;let mut roffset: c_int = tjRedOffset[pf as usize];
    let mut goffset: c_int = tjGreenOffset[pf as usize];
    let mut boffset: c_int = tjBlueOffset[pf as usize];
    let mut ps: c_int = tjPixelSize[pf as usize];
    
    
    
    
    if pf ==  TJPF_GRAY {
        memset(
            buf as *mut c_void,
            0i32,
            (w * h * ps) as c_ulong,
        );
        row = 0i32;
        while row < h {
            col = 0i32;
            while col < w {
                if flags & TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8i32 + col / 8i32) % 2i32 == 0i32 {
                    *buf.offset(index as isize) =
                        if row < halfway { 255i32 } else { 0i32 } as c_uchar
                } else {
                    *buf.offset(index as isize) =
                        if row < halfway { 76i32 } else { 226i32 } as c_uchar
                }
                col += 1
            }
            row += 1
        }
    } else if pf ==  TJPF_CMYK {
        memset(
            buf as *mut c_void,
            255i32,
            (w * h * ps) as c_ulong,
        );
        row = 0i32;
        while row < h {
            col = 0i32;
            while col < w {
                if flags & TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8i32 + col / 8i32) % 2i32 == 0i32 {
                    if row >= halfway {
                        *buf.offset((index * ps + 3i32) as isize) = 0u8
                    }
                } else {
                    *buf.offset((index * ps + 2i32) as isize) = 0u8;
                    if row < halfway {
                        *buf.offset((index * ps + 1i32) as isize) = 0u8
                    }
                }
                col += 1
            }
            row += 1
        }
    } else {
        memset(
            buf as *mut c_void,
            0i32,
            (w * h * ps) as c_ulong,
        );
        row = 0i32;
        while row < h {
            col = 0i32;
            while col < w {
                if flags & TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8i32 + col / 8i32) % 2i32 == 0i32 {
                    if row < halfway {
                        *buf.offset((index * ps + roffset) as isize) = 255u8;
                        *buf.offset((index * ps + goffset) as isize) = 255u8;
                        *buf.offset((index * ps + boffset) as isize) = 255u8
                    }
                } else {
                    *buf.offset((index * ps + roffset) as isize) = 255u8;
                    if row >= halfway {
                        *buf.offset((index * ps + goffset) as isize) = 255u8
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
    mut buf: *mut c_uchar,
    mut w: c_int,
    mut h: c_int,
    mut pf: c_int,
    mut subsamp: c_int,
    mut sf: tjscalingfactor,
    mut flags: c_int,
) -> c_int {
     let mut index:  c_int =  0; let mut row:  c_int =  0; let mut col:  c_int =  0; let mut retval:  c_int =  1i32;
    let mut roffset: c_int = tjRedOffset[pf as usize];
    let mut goffset: c_int = tjGreenOffset[pf as usize];
    let mut boffset: c_int = tjBlueOffset[pf as usize];
    let mut aoffset: c_int = tjAlphaOffset[pf as usize];
    let mut ps: c_int = tjPixelSize[pf as usize];
    
    
    
    
    let mut halfway: c_int = 16i32 * sf.num / sf.denom;
    let mut blocksize: c_int = 8i32 * sf.num / sf.denom;
    if pf ==  TJPF_GRAY {
        boffset = 0i32;
        goffset = boffset;
        roffset = goffset
    }
    if pf ==  TJPF_CMYK {
         let mut current_block:  u64;row = 0i32;
        's_40: loop {
            if !(row < h) {
                current_block = 15514718523126015390;
                break;
            }
            col = 0i32;
            while col < w {
                
                
                
                    
                if flags & TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                
                
                
                 let mut c:   c_uchar =  *buf.offset((index * ps) as isize); let mut m:   c_uchar =  *buf.offset((index * ps + 1i32) as isize); let mut y:   c_uchar =  *buf.offset((index * ps + 2i32) as isize); let mut k:   c_uchar =  *buf.offset((index * ps + 3i32) as isize);
                if (row / blocksize + col / blocksize) % 2i32 == 0i32 {
                    if (c as c_int) < 254i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"c\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            c as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if (m as c_int) < 254i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"m\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            m as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if (y as c_int) < 254i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"y\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            y as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if row < halfway {
                        if (k as c_int) < 254i32 {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"k\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                k as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            current_block = 10090817204669828264;
                            break 's_40;
                        }
                    } else if k as c_int > 1i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"k\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            k as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    }
                } else if (c as c_int) < 254i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"c\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        c as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if y as c_int > 1i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"y\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        y as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if (k as c_int) < 254i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"k\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        k as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if row < halfway {
                    if m as c_int > 1i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"m\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            m as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 10090817204669828264;
                        break 's_40;
                    }
                } else if (m as c_int) < 254i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"m\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        m as c_int,
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
                
                
                
                    
                if flags & TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1i32) * w + col
                } else {
                    index = row * w + col
                }
                
                
                
                 let mut r:   c_uchar =  *buf.offset((index * ps + roffset) as isize); let mut g:   c_uchar =  *buf.offset((index * ps + goffset) as isize); let mut b:   c_uchar =  *buf.offset((index * ps + boffset) as isize); let mut a:   c_uchar =
     if aoffset >= 0i32 {
                    *buf.offset((index * ps + aoffset) as isize) as c_int
                } else {
                    0xffi32
                } as c_uchar;
                if (row / blocksize + col / blocksize) % 2i32 == 0i32 {
                    if row < halfway {
                        if (r as c_int) < 254i32 {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"r\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                r as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (g as c_int) < 254i32 {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"g\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                g as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (b as c_int) < 254i32 {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"b\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                b as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        }
                    } else if r as c_int > 1i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"r\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            r as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if g as c_int > 1i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"g\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            g as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if b as c_int > 1i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"b\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            b as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    }
                } else if subsamp == TJSAMP_GRAY as c_int {
                    if row < halfway {
                        if (r as c_int) < 76i32 - 1i32 || r as c_int > 76i32 + 1i32 {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"r\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                76i32,
                                r as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (g as c_int) < 76i32 - 1i32
                            || g as c_int > 76i32 + 1i32
                        {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"g\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                76i32,
                                g as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        } else if (b as c_int) < 76i32 - 1i32
                            || b as c_int > 76i32 + 1i32
                        {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"b\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                76i32,
                                b as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_400;
                        }
                    } else if (r as c_int) < 226i32 - 1i32 || r as c_int > 226i32 + 1i32
                    {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"r\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            226i32,
                            r as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if (g as c_int) < 226i32 - 1i32 || g as c_int > 226i32 + 1i32
                    {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"g\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            226i32,
                            g as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if (b as c_int) < 226i32 - 1i32 || b as c_int > 226i32 + 1i32
                    {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"b\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            226i32,
                            b as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    }
                } else if row < halfway {
                    if (r as c_int) < 254i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"r\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            r as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if g as c_int > 1i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"g\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            g as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    } else if b as c_int > 1i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"b\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            b as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        break 's_400;
                    }
                } else if (r as c_int) < 254i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"r\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        r as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    break 's_400;
                } else if (g as c_int) < 254i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"g\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        g as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    break 's_400;
                } else if b as c_int > 1i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"b\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        b as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    break 's_400;
                }
                if (a as c_int) < 254i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"a\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        a as c_int,
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
                if pf ==  TJPF_CMYK {
                    printf(
                        
                        b"%.3d/%.3d/%.3d/%.3d \x00".as_ptr() as *const c_char,
                        *buf.offset(((row * w + col) * ps) as isize) as c_int,
                        *buf.offset(((row * w + col) * ps + 1i32) as isize) as c_int,
                        *buf.offset(((row * w + col) * ps + 2i32) as isize) as c_int,
                        *buf.offset(((row * w + col) * ps + 3i32) as isize) as c_int,
                    );
                } else {
                    printf(
                        
                        b"%.3d/%.3d/%.3d \x00".as_ptr() as *const c_char,
                        *buf.offset(((row * w + col) * ps + roffset) as isize) as c_int,
                        *buf.offset(((row * w + col) * ps + goffset) as isize) as c_int,
                        *buf.offset(((row * w + col) * ps + boffset) as isize) as c_int,
                    );
                }
                col += 1
            }
            printf(b"\n\x00".as_ptr() as *const c_char);
            row += 1
        }
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn checkBufYUV(
    mut buf: *mut c_uchar,
    mut w: c_int,
    mut h: c_int,
    mut subsamp: c_int,
    mut sf: tjscalingfactor,
) -> c_int {
    
    
     let mut current_block:  u64;  let mut col:  c_int =  0; let mut retval:  c_int =  1i32;
    let mut hsf: c_int = tjMCUWidth[subsamp as usize] / 8i32;
    let mut vsf: c_int = tjMCUHeight[subsamp as usize] / 8i32;
    let mut pw: c_int = w + hsf - 1i32 & !(hsf - 1i32);
    let mut ph: c_int = h + vsf - 1i32 & !(vsf - 1i32);
    let mut cw: c_int = pw / hsf;
    let mut ch: c_int = ph / vsf;
    let mut ypitch: c_int = pw + pad - 1i32 & !(pad - 1i32);
    let mut uvpitch: c_int = cw + pad - 1i32 & !(pad - 1i32);
    
    let mut halfway: c_int = 16i32 * sf.num / sf.denom;
    let mut blocksize: c_int = 8i32 * sf.num / sf.denom;
     let mut row:   c_int =  0i32;
    's_27: loop {
        if !(row < ph) {
            current_block = 1836292691772056875;
            break;
        }
        col = 0i32;
        while col < pw {
            let mut y: c_uchar = *buf.offset((ypitch * row + col) as isize);
            if (row / blocksize + col / blocksize) % 2i32 == 0i32 {
                if row < halfway {
                    if (y as c_int) < 254i32 {
                        printf(
                            
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                as *const c_char,
                            
                            b"y\x00".as_ptr() as *const c_char,
                            row,
                            col,
                            y as c_int,
                        );
                        retval = 0i32;
                        exitStatus = -1i32;
                        current_block = 17379669344980314800;
                        break 's_27;
                    }
                } else if y as c_int > 1i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"y\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        y as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 17379669344980314800;
                    break 's_27;
                }
            } else if row < halfway {
                if (y as c_int) < 76i32 - 1i32 || y as c_int > 76i32 + 1i32 {
                    printf(
                        
                        b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                            as *const c_char,
                        
                        b"y\x00".as_ptr() as *const c_char,
                        row,
                        col,
                        76i32,
                        y as c_int,
                    );
                    retval = 0i32;
                    exitStatus = -1i32;
                    current_block = 17379669344980314800;
                    break 's_27;
                }
            } else if (y as c_int) < 226i32 - 1i32 || y as c_int > 226i32 + 1i32 {
                printf(
                    
                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                        as *const c_char,
                    
                    b"y\x00".as_ptr() as *const c_char,
                    row,
                    col,
                    226i32,
                    y as c_int,
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
            if subsamp != TJSAMP_GRAY as c_int {
                let mut halfway_0: c_int = 16i32 / vsf * sf.num / sf.denom;
                row = 0i32;
                's_193: while row < ch {
                    col = 0i32;
                    while col < cw {
                        let mut u: c_uchar =
                            *buf.offset((ypitch * ph + (uvpitch * row + col)) as isize);
                        let mut v: c_uchar = *buf
                            .offset((ypitch * ph + uvpitch * ch + (uvpitch * row + col)) as isize);
                        if (row * vsf / blocksize + col * hsf / blocksize) % 2i32 == 0i32 {
                            if (u as c_int) < 128i32 - 1i32
                                || u as c_int > 128i32 + 1i32
                            {
                                printf(
                                    
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                        as *const c_char,
                                    
                                    b"u\x00".as_ptr() as *const c_char,
                                    row,
                                    col,
                                    128i32,
                                    u as c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            } else if (v as c_int) < 128i32 - 1i32
                                || v as c_int > 128i32 + 1i32
                            {
                                printf(
                                    
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                        as *const c_char,
                                    
                                    b"v\x00".as_ptr() as *const c_char,
                                    row,
                                    col,
                                    128i32,
                                    v as c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            }
                        } else if row < halfway_0 {
                            if (u as c_int) < 85i32 - 1i32 || u as c_int > 85i32 + 1i32
                            {
                                printf(
                                    
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                        as *const c_char,
                                    
                                    b"u\x00".as_ptr() as *const c_char,
                                    row,
                                    col,
                                    85i32,
                                    u as c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            } else if (v as c_int) < 254i32 {
                                printf(
                                    
                                    b"\nComp. %s at %d,%d should be 255, not %d\n\x00".as_ptr()
                                        as *const c_char,
                                    
                                    b"v\x00".as_ptr() as *const c_char,
                                    row,
                                    col,
                                    v as c_int,
                                );
                                retval = 0i32;
                                exitStatus = -1i32;
                                break 's_193;
                            }
                        } else if u as c_int > 1i32 {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be 0, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"u\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                u as c_int,
                            );
                            retval = 0i32;
                            exitStatus = -1i32;
                            break 's_193;
                        } else if (v as c_int) < 149i32 - 1i32
                            || v as c_int > 149i32 + 1i32
                        {
                            printf(
                                
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00".as_ptr()
                                    as *const c_char,
                                
                                b"v\x00".as_ptr() as *const c_char,
                                row,
                                col,
                                149i32,
                                v as c_int,
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
                printf(
                    
                    b"%.3d \x00".as_ptr() as *const c_char,
                    *buf.offset((ypitch * row + col) as isize) as c_int,
                );
                col += 1
            }
            printf(b"\n\x00".as_ptr() as *const c_char);
            row += 1
        }
        printf(b"\n\x00".as_ptr() as *const c_char);
        row = 0i32;
        while row < ch {
            col = 0i32;
            while col < cw {
                printf(
                    
                    b"%.3d \x00".as_ptr() as *const c_char,
                    *buf.offset((ypitch * ph + (uvpitch * row + col)) as isize) as c_int,
                );
                col += 1
            }
            printf(b"\n\x00".as_ptr() as *const c_char);
            row += 1
        }
        printf(b"\n\x00".as_ptr() as *const c_char);
        row = 0i32;
        while row < ch {
            col = 0i32;
            while col < cw {
                printf(
                    
                    b"%.3d \x00".as_ptr() as *const c_char,
                    *buf.offset((ypitch * ph + uvpitch * ch + (uvpitch * row + col)) as isize)
                        as c_int,
                );
                col += 1
            }
            printf(b"\n\x00".as_ptr() as *const c_char);
            row += 1
        }
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn writeJPEG(
    mut jpegBuf: *mut c_uchar,
    mut jpegSize: c_ulong,
    mut filename: *mut c_char,
) {
    let mut file: *mut FILE =
        fopen(filename,  b"wb\x00".as_ptr() as *const c_char);
    if file.is_null()
        || fwrite(
            jpegBuf as *const c_void,
            jpegSize,
            1u64,
            file,
        ) != 1u64
    {
        printf(
            
            b"ERROR: Could not write to %s.\n%s\n\x00".as_ptr() as *const c_char,
            filename,
            strerror(*__errno_location()),
        );
        exitStatus = -1i32
    }
    if !file.is_null() {
        fclose(file);
    };
}
#[no_mangle]

pub unsafe extern "C" fn compTest(
    mut handle: tjhandle,
    mut dstBuf: *mut *mut c_uchar,
    mut dstSize: *mut c_ulong,
    mut w: c_int,
    mut h: c_int,
    mut pf: c_int,
    mut basename: *mut c_char,
    mut subsamp: c_int,
    mut jpegQual: c_int,
    mut flags: c_int,
) {
    
    
    let mut srcBuf: *mut c_uchar = NULL as *mut c_uchar;
    let mut yuvBuf: *mut c_uchar = NULL as *mut c_uchar;
    let mut pfStr: *const c_char = pixFormatStr[pf as usize];
    let mut buStrLong: *const c_char = if flags & TJFLAG_BOTTOMUP != 0
    {
        
        b"Bottom-Up\x00".as_ptr() as *const c_char
    } else {
        
        b"Top-Down \x00".as_ptr() as *const c_char
    };
    let mut buStr: *const c_char = if flags & TJFLAG_BOTTOMUP != 0 {
        
        b"BU\x00".as_ptr() as *const c_char
    } else {
        
        b"TD\x00".as_ptr() as *const c_char
    };
    srcBuf = malloc(
        (w * h * tjPixelSize[pf as usize]) as c_ulong,
    ) as *mut c_uchar;
    if srcBuf.is_null() {
        printf(
            
            b"ERROR: %s\n\x00".as_ptr() as *const c_char,
            
            b"Memory allocation failure\x00".as_ptr() as *const c_char,
        );
        exitStatus = -1i32
    } else {
         let mut current_block:  u64;initBuf(srcBuf, w, h, pf, flags);
        if !(*dstBuf).is_null() && *dstSize > 0u64 {
            memset(*dstBuf as *mut c_void, 0i32, *dstSize);
        }
        if alloc == 0 {
            flags |= TJFLAG_NOREALLOC
        }
        if doYUV != 0 {
            let mut yuvSize: c_ulong =
                tjBufSizeYUV2(w, pad, h, subsamp);
            let mut sf: tjscalingfactor = {
                 let mut init =
     tjscalingfactor{num:  1i32, denom:  1i32,};
                init
            };
            let mut handle2: tjhandle =
                tjInitCompress();
            if handle2.is_null() {
                printf(
                    
                    b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                    tjGetErrorStr(),
                );
                exitStatus = -1i32;
                current_block = 860601949763470011;
            } else {
                yuvBuf = malloc(yuvSize) as *mut c_uchar;
                if yuvBuf.is_null() {
                    printf(
                        
                        b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                        
                        b"Memory allocation failure\x00".as_ptr() as *const c_char,
                    );
                    exitStatus = -1i32;
                    current_block = 860601949763470011;
                } else {
                    memset(yuvBuf as *mut c_void, 0i32, yuvSize);
                    printf(
                        
                        b"%s %s -> YUV %s ... \x00".as_ptr() as *const c_char,
                        pfStr,
                        buStrLong,
                        subNameLong[subsamp as usize],
                    );
                    if tjEncodeYUV3(
                        handle2, srcBuf, w, 0i32, h, pf, yuvBuf, pad, subsamp, flags,
                    ) == -1i32
                    {
                        printf(
                            
                            b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                            tjGetErrorStr(),
                        );
                        exitStatus = -1i32;
                        current_block = 860601949763470011;
                    } else {
                        tjDestroy(handle2);
                        if checkBufYUV(yuvBuf, w, h, subsamp, sf) != 0 {
                            printf(
                                
                                b"Passed.\n\x00".as_ptr() as *const c_char,
                            );
                        } else {
                            printf(
                                
                                b"FAILED!\n\x00".as_ptr() as *const c_char,
                            );
                        }
                        printf(
                            
                            b"YUV %s %s -> JPEG Q%d ... \x00".as_ptr() as *const c_char,
                            subNameLong[subsamp as usize],
                            buStrLong,
                            jpegQual,
                        );
                        if tjCompressFromYUV(
                            handle, yuvBuf, w, pad, h, subsamp, dstBuf, dstSize, jpegQual, flags,
                        ) == -1i32
                        {
                            printf(
                                
                                b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                                tjGetErrorStr(),
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
            printf(
                
                b"%s %s -> %s Q%d ... \x00".as_ptr() as *const c_char,
                pfStr,
                buStrLong,
                subNameLong[subsamp as usize],
                jpegQual,
            );
            if tjCompress2(
                handle, srcBuf, w, 0i32, h, pf, dstBuf, dstSize, subsamp, jpegQual, flags,
            ) == -1i32
            {
                printf(
                    
                    b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                    tjGetErrorStr(),
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
                 let mut tempStr:  [c_char; 1024] =  [0; 1024];snprintf(
                    tempStr.as_mut_ptr(),
                    1024u64,
                    
                    b"%s_enc_%s_%s_%s_Q%d.jpg\x00".as_ptr() as *const c_char,
                    basename,
                    pfStr,
                    buStr,
                    subName[subsamp as usize],
                    jpegQual,
                );
                writeJPEG(*dstBuf, *dstSize, tempStr.as_mut_ptr());
                printf(
                    
                    b"Done.\n  Result in %s\n\x00".as_ptr() as *const c_char,
                    tempStr.as_mut_ptr(),
                );
            }
        }
    }
    if !yuvBuf.is_null() {
        free(yuvBuf as *mut c_void);
    }
    if !srcBuf.is_null() {
        free(srcBuf as *mut c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn _decompTest(
    mut handle: tjhandle,
    mut jpegBuf: *mut c_uchar,
    mut jpegSize: c_ulong,
    mut w: c_int,
    mut h: c_int,
    mut pf: c_int,
    mut _basename: *mut c_char,
    mut subsamp: c_int,
    mut flags: c_int,
    mut sf: tjscalingfactor,
) {
     let mut _hdrw:  c_int =  0i32; let mut _hdrh:  c_int =  0i32;
    let mut dstBuf: *mut c_uchar = NULL as *mut c_uchar;
    let mut yuvBuf: *mut c_uchar = NULL as *mut c_uchar;
    
    
    let mut _hdrsubsamp: c_int = -1i32;
    let mut scaledWidth: c_int = (w * sf.num + sf.denom - 1i32) / sf.denom;
    let mut scaledHeight: c_int = (h * sf.num + sf.denom - 1i32) / sf.denom;
    
    if tjDecompressHeader2(
        handle,
        jpegBuf,
        jpegSize,
        &mut _hdrw,
        &mut _hdrh,
        &mut _hdrsubsamp,
    ) == -1i32
    {
        printf(
            
            b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
            tjGetErrorStr(),
        );
        exitStatus = -1i32
    } else if _hdrw != w || _hdrh != h || _hdrsubsamp != subsamp {
        printf(
            
            b"ERROR: %s\n\x00".as_ptr() as *const c_char,
            
            b"Incorrect JPEG header\x00".as_ptr() as *const c_char,
        );
        exitStatus = -1i32
    } else {
          let mut dstSize:   c_ulong =
     (scaledWidth * scaledHeight * tjPixelSize[pf as usize])
            as c_ulong;
        dstBuf = malloc(dstSize) as *mut c_uchar;
        if dstBuf.is_null() {
            printf(
                
                b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                
                b"Memory allocation failure\x00".as_ptr() as *const c_char,
            );
            exitStatus = -1i32
        } else {
             let mut current_block:  u64;memset(dstBuf as *mut c_void, 0i32, dstSize);
            if doYUV != 0 {
                let mut yuvSize: c_ulong =
                    tjBufSizeYUV2(scaledWidth, pad, scaledHeight, subsamp);
                let mut handle2: tjhandle =
                    tjInitDecompress();
                if handle2.is_null() {
                    printf(
                        
                        b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                        tjGetErrorStr(),
                    );
                    exitStatus = -1i32;
                    current_block = 14297773496329963019;
                } else {
                    yuvBuf = malloc(yuvSize) as *mut c_uchar;
                    if yuvBuf.is_null() {
                        printf(
                            
                            b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                            
                            b"Memory allocation failure\x00".as_ptr() as *const c_char,
                        );
                        exitStatus = -1i32;
                        current_block = 14297773496329963019;
                    } else {
                        memset(yuvBuf as *mut c_void, 0i32, yuvSize);
                        printf(
                            
                            b"JPEG -> YUV %s \x00".as_ptr() as *const c_char,
                            subNameLong[subsamp as usize],
                        );
                        if sf.num != 1i32 || sf.denom != 1i32 {
                            printf(
                                
                                b"%d/%d ... \x00".as_ptr() as *const c_char,
                                sf.num,
                                sf.denom,
                            );
                        } else {
                            printf(b"... \x00".as_ptr() as *const c_char);
                        }
                        if tjDecompressToYUV2(
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
                            printf(
                                
                                b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                                tjGetErrorStr(),
                            );
                            exitStatus = -1i32;
                            current_block = 14297773496329963019;
                        } else {
                            if checkBufYUV(yuvBuf, scaledWidth, scaledHeight, subsamp, sf) != 0 {
                                printf(
                                    
                                    b"Passed.\n\x00".as_ptr() as *const c_char,
                                );
                            } else {
                                printf(
                                    
                                    b"FAILED!\n\x00".as_ptr() as *const c_char,
                                );
                            }
                            printf(
                                
                                b"YUV %s -> %s %s ... \x00".as_ptr() as *const c_char,
                                subNameLong[subsamp as usize],
                                pixFormatStr[pf as usize],
                                if flags & TJFLAG_BOTTOMUP != 0 {
                                    
                                    b"Bottom-Up\x00".as_ptr() as *const c_char
                                } else {
                                    
                                    b"Top-Down \x00".as_ptr() as *const c_char
                                },
                            );
                            if tjDecodeYUV(
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
                                printf(
                                    
                                    b"TurboJPEG ERROR:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    tjGetErrorStr(),
                                );
                                exitStatus = -1i32;
                                current_block = 14297773496329963019;
                            } else {
                                tjDestroy(handle2);
                                current_block = 15594839951440953787;
                            }
                        }
                    }
                }
            } else {
                printf(
                    
                    b"JPEG -> %s %s \x00".as_ptr() as *const c_char,
                    pixFormatStr[pf as usize],
                    if flags & TJFLAG_BOTTOMUP != 0 {
                        
                        b"Bottom-Up\x00".as_ptr() as *const c_char
                    } else {
                        
                        b"Top-Down \x00".as_ptr() as *const c_char
                    },
                );
                if sf.num != 1i32 || sf.denom != 1i32 {
                    printf(
                        
                        b"%d/%d ... \x00".as_ptr() as *const c_char,
                        sf.num,
                        sf.denom,
                    );
                } else {
                    printf(b"... \x00".as_ptr() as *const c_char);
                }
                if tjDecompress2(
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
                    printf(
                        
                        b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                        tjGetErrorStr(),
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
                    if checkBuf(dstBuf, scaledWidth, scaledHeight, pf, subsamp, sf, flags) != 0 {
                        printf(b"Passed.\x00".as_ptr() as *const c_char);
                    } else {
                        printf(b"FAILED!\x00".as_ptr() as *const c_char);
                    }
                    printf(b"\n\x00".as_ptr() as *const c_char);
                }
            }
        }
    }
    if !yuvBuf.is_null() {
        free(yuvBuf as *mut c_void);
    }
    if !dstBuf.is_null() {
        free(dstBuf as *mut c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn decompTest(
    mut handle: tjhandle,
    mut jpegBuf: *mut c_uchar,
    mut jpegSize: c_ulong,
    mut w: c_int,
    mut h: c_int,
    mut pf: c_int,
    mut basename: *mut c_char,
    mut subsamp: c_int,
    mut flags: c_int,
) {
    
     let mut n:  c_int =  0i32;
    let mut sf: *mut tjscalingfactor =
        tjGetScalingFactors(&mut n);
    if sf.is_null() || n == 0 {
        printf(
            
            b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
            tjGetErrorStr(),
        );
        exitStatus = -1i32
    } else {
          let mut i:   c_int =  0i32;
        while i < n {
            if subsamp == TJSAMP_444 as c_int
                || subsamp == TJSAMP_GRAY as c_int
                || subsamp == TJSAMP_411 as c_int
                    && (*sf.offset(i as isize)).num == 1i32
                    && ((*sf.offset(i as isize)).denom == 2i32
                        || (*sf.offset(i as isize)).denom == 1i32)
                || subsamp != TJSAMP_411 as c_int
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
    mut w: c_int,
    mut h: c_int,
    mut formats: *const c_int,
    mut nformats: c_int,
    mut subsamp: c_int,
    mut basename: *mut c_char,
) {
     let mut current_block:  u64; let mut size:  c_ulong =  0u64;
    let mut chandle: tjhandle = NULL as *mut c_void;
    let mut dhandle: tjhandle = NULL as *mut c_void;
    let mut dstBuf: *mut c_uchar = NULL as *mut c_uchar;
    
    
    
    
    if alloc == 0 {
        size = tjBufSize(w, h, subsamp)
    }
    if size != 0u64 {
        dstBuf = tjAlloc(size as c_int);
        if dstBuf.is_null() {
            printf(
                
                b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                
                b"Memory allocation failure.\x00".as_ptr() as *const c_char,
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
            chandle = tjInitCompress();
            if chandle.is_null() || {
                dhandle = tjInitDecompress();
                dhandle.is_null()
            } {
                printf(
                    
                    b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                    tjGetErrorStr(),
                );
                exitStatus = -1i32
            } else {
                  let mut pfi:   c_int =  0i32;
                while pfi < nformats {
                      let mut i:   c_int =  0i32;
                    while i < 2i32 {
                          let mut flags:  c_int =  0i32;
                        if subsamp == TJSAMP_422 as c_int
                            || subsamp == TJSAMP_420 as c_int
                            || subsamp == TJSAMP_440 as c_int
                            || subsamp == TJSAMP_411 as c_int
                        {
                            flags |= TJFLAG_FASTUPSAMPLE
                        }
                        if i == 1i32 {
                            flags |= TJFLAG_BOTTOMUP
                        }
                         let mut pf:   c_int =  *formats.offset(pfi as isize);
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
                        if pf >=  TJPF_RGBX
                            && pf <=  TJPF_XRGB
                        {
                            printf(b"\n\x00".as_ptr() as *const c_char);
                            decompTest(
                                dhandle,
                                dstBuf,
                                size,
                                w,
                                h,
                                pf + (TJPF_RGBA
                                    -  TJPF_RGBX),
                                basename,
                                subsamp,
                                flags,
                            );
                        }
                        printf(b"\n\x00".as_ptr() as *const c_char);
                        i += 1
                    }
                    pfi += 1
                }
                printf(
                    
                    b"--------------------\n\n\x00".as_ptr() as *const c_char,
                );
            }
        }
        _ => {}
    }
    if !chandle.is_null() {
        tjDestroy(chandle);
    }
    if !dhandle.is_null() {
        tjDestroy(dhandle);
    }
    if !dstBuf.is_null() {
        tjFree(dstBuf);
    };
}
#[no_mangle]

pub unsafe extern "C" fn bufSizeTest() {
    
    
    
    
    
    let mut srcBuf: *mut c_uchar = NULL as *mut c_uchar;
    let mut dstBuf: *mut c_uchar = NULL as *mut c_uchar;
    let mut handle: tjhandle = NULL as *mut c_void;
    
    handle = tjInitCompress();
    if handle.is_null() {
        printf(
            
            b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
            tjGetErrorStr(),
        );
        exitStatus = -1i32
    } else {
         let mut current_block:  u64; printf(
            
            b"Buffer size regression test\n\x00".as_ptr() as *const c_char,
        );
         let mut subsamp:   c_int =  0i32;
        's_43: loop {
             if !(subsamp < TJ_NUMSAMP) {
                current_block = 6040267449472925966;
                break;
            }
             let mut w:   c_int =  1i32;
            while w < 48i32 {
                 let mut maxh: c_int = if w == 1i32 { 2048i32 } else { 48i32 };
                 let mut h:   c_int =  1i32;
                while h < maxh {
                    if h % 100i32 == 0i32 {
                        printf(
                            
                            b"%.4d x %.4d\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x00".as_ptr() as *const c_char,
                            w,
                            h,
                        );
                    }
                    srcBuf = malloc((w * h * 4i32) as c_ulong)
                        as *mut c_uchar;
                    if srcBuf.is_null() {
                        printf(
                            
                            b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                            
                            b"Memory allocation failure\x00".as_ptr() as *const c_char,
                        );
                        exitStatus = -1i32;
                        current_block = 17868673386502678986;
                        break 's_43;
                    } else {
                          let mut dstSize:  c_ulong =  0u64;if alloc == 0 || doYUV != 0 {
                            if doYUV != 0 {
                                dstSize = tjBufSizeYUV2(w, pad, h, subsamp)
                            } else {
                                dstSize = tjBufSize(w, h, subsamp)
                            }
                            dstBuf = tjAlloc(dstSize as c_int);
                            if dstBuf.is_null() {
                                printf(
                                    
                                    b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                                    
                                    b"Memory allocation failure\x00".as_ptr()
                                        as *const c_char,
                                );
                                exitStatus = -1i32;
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                        }
                         let mut i:   c_int =  0i32;
                        while i < w * h * 4i32 {
                            if random()
                                < (RAND_MAX / 2i32) as c_long
                            {
                                *srcBuf.offset(i as isize) = 0u8
                            } else {
                                *srcBuf.offset(i as isize) = 255u8
                            }
                            i += 1
                        }
                        if doYUV != 0 {
                            if tjEncodeYUV3(
                                handle,
                                srcBuf,
                                w,
                                0i32,
                                h,
                                
                                TJPF_BGRX,
                                dstBuf,
                                pad,
                                subsamp,
                                0i32,
                            ) == -1i32
                            {
                                printf(
                                    
                                    b"TurboJPEG ERROR:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    tjGetErrorStr(),
                                );
                                exitStatus = -1i32;
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                        } else if tjCompress2(
                            handle,
                            srcBuf,
                            w,
                            0i32,
                            h,
                            
                            TJPF_BGRX,
                            &mut dstBuf,
                            &mut dstSize,
                            subsamp,
                            100i32,
                            if alloc != 0 { 0i32 } else { 1024i32 },
                        ) == -1i32
                        {
                            printf(
                                
                                b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                                tjGetErrorStr(),
                            );
                            exitStatus = -1i32;
                            current_block = 17868673386502678986;
                            break 's_43;
                        }
                        free(srcBuf as *mut c_void);
                        srcBuf = NULL as *mut c_uchar;
                        if alloc == 0 || doYUV != 0 {
                            tjFree(dstBuf);
                            dstBuf = NULL as *mut c_uchar
                        }
                        srcBuf = malloc((h * w * 4i32) as c_ulong)
                            as *mut c_uchar;
                        if srcBuf.is_null() {
                            printf(
                                
                                b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                                
                                b"Memory allocation failure\x00".as_ptr()
                                    as *const c_char,
                            );
                            exitStatus = -1i32;
                            current_block = 17868673386502678986;
                            break 's_43;
                        } else {
                            if alloc == 0 || doYUV != 0 {
                                if doYUV != 0 {
                                    dstSize =
                                        tjBufSizeYUV2(h, pad, w, subsamp)
                                } else {
                                    dstSize = tjBufSize(h, w, subsamp)
                                }
                                dstBuf = tjAlloc(dstSize as c_int);
                                if dstBuf.is_null() {
                                    printf(
                                        
                                        b"ERROR: %s\n\x00".as_ptr() as *const c_char,
                                        
                                        b"Memory allocation failure\x00".as_ptr()
                                            as *const c_char,
                                    );
                                    exitStatus = -1i32;
                                    current_block = 17868673386502678986;
                                    break 's_43;
                                }
                            }
                            i = 0i32;
                            while i < h * w * 4i32 {
                                if random()
                                    < (RAND_MAX / 2i32) as c_long
                                {
                                    *srcBuf.offset(i as isize) = 0u8
                                } else {
                                    *srcBuf.offset(i as isize) = 255u8
                                }
                                i += 1
                            }
                            if doYUV != 0 {
                                if tjEncodeYUV3(
                                    handle,
                                    srcBuf,
                                    h,
                                    0i32,
                                    w,
                                    
                                    TJPF_BGRX,
                                    dstBuf,
                                    pad,
                                    subsamp,
                                    0i32,
                                ) == -1i32
                                {
                                    printf(
                                        
                                        b"TurboJPEG ERROR:\n%s\n\x00".as_ptr()
                                            as *const c_char,
                                        tjGetErrorStr(),
                                    );
                                    exitStatus = -1i32;
                                    current_block = 17868673386502678986;
                                    break 's_43;
                                }
                            } else if tjCompress2(
                                handle,
                                srcBuf,
                                h,
                                0i32,
                                w,
                                
                                TJPF_BGRX,
                                &mut dstBuf,
                                &mut dstSize,
                                subsamp,
                                100i32,
                                if alloc != 0 { 0i32 } else { 1024i32 },
                            ) == -1i32
                            {
                                printf(
                                    
                                    b"TurboJPEG ERROR:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    tjGetErrorStr(),
                                );
                                exitStatus = -1i32;
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                            free(srcBuf as *mut c_void);
                            srcBuf = NULL as *mut c_uchar;
                            if alloc == 0 || doYUV != 0 {
                                tjFree(dstBuf);
                                dstBuf = NULL as *mut c_uchar
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
                printf(b"Done.      \n\x00".as_ptr() as *const c_char);
            }
        }
    }
    if !srcBuf.is_null() {
        free(srcBuf as *mut c_void);
    }
    if !dstBuf.is_null() {
        tjFree(dstBuf);
    }
    if !handle.is_null() {
        tjDestroy(handle);
    };
}
#[no_mangle]

pub unsafe extern "C" fn initBitmap(
    mut buf: *mut c_uchar,
    mut width: c_int,
    mut pitch: c_int,
    mut height: c_int,
    mut pf: c_int,
    mut flags: c_int,
) {
     let mut roffset: c_int = tjRedOffset[pf as usize];
    let mut goffset: c_int = tjGreenOffset[pf as usize];
    let mut boffset: c_int = tjBlueOffset[pf as usize];
    let mut ps: c_int = tjPixelSize[pf as usize];
    
    
     let mut j:   c_int =  0i32;
    while j < height {
         let mut row: c_int = if flags & TJFLAG_BOTTOMUP != 0 {
            (height - j) - 1i32
        } else {
            j
        };
         let mut i:   c_int =  0i32;
        while i < width {
            let mut r: c_uchar = (i * 256i32 / width % 256i32) as c_uchar;
            let mut g: c_uchar = (j * 256i32 / height % 256i32) as c_uchar;
            let mut b: c_uchar =
                ((j * 256i32 / height + i * 256i32 / width) % 256i32) as c_uchar;
            memset(
                &mut *buf.offset((row * pitch + i * ps) as isize) as *mut c_uchar
                    as *mut c_void,
                0i32,
                ps as c_ulong,
            );
            if pf ==  TJPF_GRAY {
                *buf.offset((row * pitch + i * ps) as isize) = b
            } else if pf ==  TJPF_CMYK {
                rgb_to_cmyk(
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
    mut buf: *mut c_uchar,
    mut width: c_int,
    mut pitch: c_int,
    mut height: c_int,
    mut pf: c_int,
    mut flags: c_int,
    mut gray2rgb: c_int,
) -> c_int {
     let mut roffset: c_int = tjRedOffset[pf as usize];
    let mut goffset: c_int = tjGreenOffset[pf as usize];
    let mut boffset: c_int = tjBlueOffset[pf as usize];
    let mut aoffset: c_int = tjAlphaOffset[pf as usize];
    let mut ps: c_int = tjPixelSize[pf as usize];
    
    
     let mut j:   c_int =  0i32;
    while j < height {
         let mut row: c_int = if flags & TJFLAG_BOTTOMUP != 0 {
            (height - j) - 1i32
        } else {
            j
        };
         let mut i:   c_int =  0i32;
        while i < width {
            let mut r: c_uchar = (i * 256i32 / width % 256i32) as c_uchar;
            let mut g: c_uchar = (j * 256i32 / height % 256i32) as c_uchar;
            let mut b: c_uchar =
                ((j * 256i32 / height + i * 256i32 / width) % 256i32) as c_uchar;
            if pf ==  TJPF_GRAY {
                if *buf.offset((row * pitch + i * ps) as isize) as c_int != b as c_int {
                    return 0i32;
                }
            } else if pf ==  TJPF_CMYK {
                
                
                 let mut rf:  c_uchar =  0; let mut gf:  c_uchar =  0; let mut bf:  c_uchar =  0;
                cmyk_to_rgb(
                    *buf.offset((row * pitch + i * ps + 0i32) as isize),
                    *buf.offset((row * pitch + i * ps + 1i32) as isize),
                    *buf.offset((row * pitch + i * ps + 2i32) as isize),
                    *buf.offset((row * pitch + i * ps + 3i32) as isize),
                    &mut rf,
                    &mut gf,
                    &mut bf,
                );
                if gray2rgb != 0 {
                    if rf as c_int != b as c_int
                        || gf as c_int != b as c_int
                        || bf as c_int != b as c_int
                    {
                        return 0i32;
                    }
                } else if rf as c_int != r as c_int
                    || gf as c_int != g as c_int
                    || bf as c_int != b as c_int
                {
                    return 0i32;
                }
            } else {
                if gray2rgb != 0 {
                    if *buf.offset((row * pitch + i * ps + roffset) as isize) as c_int
                        != b as c_int
                        || *buf.offset((row * pitch + i * ps + goffset) as isize) as c_int
                            != b as c_int
                        || *buf.offset((row * pitch + i * ps + boffset) as isize) as c_int
                            != b as c_int
                    {
                        return 0i32;
                    }
                } else if *buf.offset((row * pitch + i * ps + roffset) as isize) as c_int
                    != r as c_int
                    || *buf.offset((row * pitch + i * ps + goffset) as isize) as c_int
                        != g as c_int
                    || *buf.offset((row * pitch + i * ps + boffset) as isize) as c_int
                        != b as c_int
                {
                    return 0i32;
                }
                if aoffset >= 0i32
                    && *buf.offset((row * pitch + i * ps + aoffset) as isize) as c_int
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
    mut ext: *const c_char,
    mut width: c_int,
    mut align: c_int,
    mut height: c_int,
    mut pf: c_int,
    mut flags: c_int,
) -> c_int {
    
    
    
     let mut retval:  c_int =  0i32; let mut md5ref:  *mut c_char =  ::std::ptr::null_mut::< c_char>();
    let mut ps: c_int = tjPixelSize[pf as usize];
    let mut pitch: c_int = width * ps + align - 1i32 & !(align - 1i32);
    
    
    
    let mut pixelFormat: c_int = pf;
    let mut buf: *mut c_uchar = NULL as *mut c_uchar;
    
    if pf ==  TJPF_GRAY {
        md5ref =
            if strcasecmp(ext,  b"ppm\x00".as_ptr() as *const c_char) == 0 {
                
                b"112c682e82ce5de1cca089e20d60000b\x00".as_ptr() as *const c_char
            } else {
                
                b"51976530acf75f02beddf5d21149101d\x00".as_ptr() as *const c_char
            } as *mut c_char
    } else {
        md5ref =
            if strcasecmp(ext,  b"ppm\x00".as_ptr() as *const c_char) == 0 {
                
                b"c0c9f772b464d1896326883a5c79c545\x00".as_ptr() as *const c_char
            } else {
                
                b"6d659071b9bfcdee2def22cb58ddadca\x00".as_ptr() as *const c_char
            } as *mut c_char
    }
    buf = tjAlloc(pitch * height);
    if buf.is_null() {
        printf(
            
            b"ERROR: %s\n\x00".as_ptr() as *const c_char,
            
            b"Could not allocate memory\x00".as_ptr() as *const c_char,
        );
        exitStatus = -1i32
    } else {
         let mut filename:  [c_char; 80] =  [0; 80];initBitmap(buf, width, pitch, height, pf, flags);
        snprintf(
            filename.as_mut_ptr(),
            80u64,
            
            b"test_bmp_%s_%d_%s.%s\x00".as_ptr() as *const c_char,
            pixFormatStr[pf as usize],
            align,
            if flags & TJFLAG_BOTTOMUP != 0 {
                
                b"bu\x00".as_ptr() as *const c_char
            } else {
                
                b"td\x00".as_ptr() as *const c_char
            },
            ext,
        );
        if tjSaveImage(
            filename.as_mut_ptr(),
            buf,
            width,
            pitch,
            height,
            pf,
            flags,
        ) == -1i32
        {
            printf(
                
                b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                tjGetErrorStr(),
            );
            exitStatus = -1i32
        } else {
              let mut md5buf:  [c_char; 65] =  [0; 65]; let mut md5sum:   *mut c_char =
     MD5File(filename.as_mut_ptr(), md5buf.as_mut_ptr());
            if strcasecmp(md5sum, md5ref) != 0 {
                printf(
                    
                    b"\n%s has an MD5 sum of %s.\n   Should be %s.\n\x00".as_ptr()
                        as *const c_char,
                    filename.as_mut_ptr(),
                    md5sum,
                    md5ref,
                );
                exitStatus = -1i32
            } else {
                 let mut loadWidth:  c_int =  0i32; let mut loadHeight:  c_int =  0i32;tjFree(buf);
                buf = NULL as *mut c_uchar;
                buf = tjLoadImage(
                    filename.as_mut_ptr(),
                    &mut loadWidth,
                    align,
                    &mut loadHeight,
                    &mut pf,
                    flags,
                );
                if buf.is_null() {
                    printf(
                        
                        b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                        tjGetErrorStr(),
                    );
                    exitStatus = -1i32
                } else if width != loadWidth || height != loadHeight {
                    printf(
                        
                        b"\n   Image dimensions of %s are bogus\n\x00".as_ptr()
                            as *const c_char,
                        filename.as_mut_ptr(),
                    );
                    retval = -1i32
                } else if cmpBitmap(buf, width, pitch, height, pf, flags, 0i32) == 0 {
                    printf(
                        
                        b"\n   Pixel data in %s is bogus\n\x00".as_ptr() as *const c_char,
                        filename.as_mut_ptr(),
                    );
                    retval = -1i32
                } else {
                     let mut current_block:  u64;if pf ==  TJPF_GRAY {
                        tjFree(buf);
                        buf = NULL as *mut c_uchar;
                        pf =  TJPF_XBGR;
                        buf = tjLoadImage(
                            filename.as_mut_ptr(),
                            &mut loadWidth,
                            align,
                            &mut loadHeight,
                            &mut pf,
                            flags,
                        );
                        if buf.is_null() {
                            printf(
                                
                                b"TurboJPEG ERROR:\n%s\n\x00".as_ptr() as *const c_char,
                                tjGetErrorStr(),
                            );
                            exitStatus = -1i32;
                            current_block = 14417489546151714667;
                        } else {
                            pitch = width * tjPixelSize[pf as usize] + align
                                - 1i32
                                & !(align - 1i32);
                            if cmpBitmap(buf, width, pitch, height, pf, flags, 1i32) == 0 {
                                printf(
                                    
                                    b"\n   Converting %s to RGB failed\n\x00".as_ptr()
                                        as *const c_char,
                                    filename.as_mut_ptr(),
                                );
                                retval = -1i32;
                                current_block = 14417489546151714667;
                            } else {
                                tjFree(buf);
                                buf = NULL as *mut c_uchar;
                                pf =  TJPF_CMYK;
                                buf = tjLoadImage(
                                    filename.as_mut_ptr(),
                                    &mut loadWidth,
                                    align,
                                    &mut loadHeight,
                                    &mut pf,
                                    flags,
                                );
                                if buf.is_null() {
                                    printf(
                                        
                                        b"TurboJPEG ERROR:\n%s\n\x00".as_ptr()
                                            as *const c_char,
                                        tjGetErrorStr(),
                                    );
                                    exitStatus = -1i32;
                                    current_block = 14417489546151714667;
                                } else {
                                    pitch = width * tjPixelSize[pf as usize]
                                        + align
                                        - 1i32
                                        & !(align - 1i32);
                                    if cmpBitmap(buf, width, pitch, height, pf, flags, 1i32) == 0 {
                                        printf(
                                            
                                            b"\n   Converting %s to CMYK failed\n\x00".as_ptr()
                                                as *const c_char,
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
                            /* Verify that tjLoadImage() returns the proper "preferred" pixel format for
                            the file type. */
                            tjFree(buf);
                            buf = NULL as *mut c_uchar;
                            pf = pixelFormat;
                            pixelFormat =  TJPF_UNKNOWN;
                            buf = tjLoadImage(
                                filename.as_mut_ptr(),
                                &mut loadWidth,
                                align,
                                &mut loadHeight,
                                &mut pixelFormat,
                                flags,
                            );
                            if buf.is_null() {
                                printf(
                                    
                                    b"TurboJPEG ERROR:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    tjGetErrorStr(),
                                );
                                exitStatus = -1i32
                            } else {
                                if pf ==  TJPF_GRAY
                                    && pixelFormat
                                        !=  TJPF_GRAY
                                    || pf !=  TJPF_GRAY
                                        && strcasecmp(
                                            ext,
                                            
                                            b"bmp\x00".as_ptr() as *const c_char,
                                        ) == 0
                                        && pixelFormat
                                            !=  TJPF_BGR
                                    || pf !=  TJPF_GRAY
                                        && strcasecmp(
                                            ext,
                                            
                                            b"ppm\x00".as_ptr() as *const c_char,
                                        ) == 0
                                        && pixelFormat
                                            !=  TJPF_RGB
                                {
                                    printf(b"\n   tjLoadImage() returned unexpected pixel format: %s\n\x00".as_ptr() as
                                               *const c_char,
                                           pixFormatStr[pixelFormat as
                                                            usize]);
                                    retval = -1i32
                                }
                                unlink(filename.as_mut_ptr());
                            }
                        }
                    }
                }
            }
        }
    }
    if !buf.is_null() {
        tjFree(buf);
    }
    if exitStatus < 0i32 {
        return exitStatus;
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn bmpTest() -> c_int {
    
    
    
     
     let mut align:   c_int =  1i32;
    while align <= 8i32 {
          let mut format:   c_int =  0i32;
        while format < TJ_NUMPF {
             let mut width:  c_int =  35i32; let mut height:  c_int =  39i32;printf(
                
                b"%s Top-Down BMP (row alignment = %d bytes)  ...  \x00".as_ptr()
                    as *const c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                
                b"bmp\x00".as_ptr() as *const c_char,
                width,
                align,
                height,
                format,
                0i32,
            ) == -1i32
            {
                return -1i32;
            }
            printf(b"OK.\n\x00".as_ptr() as *const c_char);
            printf(
                
                b"%s Top-Down PPM (row alignment = %d bytes)  ...  \x00".as_ptr()
                    as *const c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                
                b"ppm\x00".as_ptr() as *const c_char,
                width,
                align,
                height,
                format,
                TJFLAG_BOTTOMUP,
            ) == -1i32
            {
                return -1i32;
            }
            printf(b"OK.\n\x00".as_ptr() as *const c_char);
            printf(
                
                b"%s Bottom-Up BMP (row alignment = %d bytes)  ...  \x00".as_ptr()
                    as *const c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                
                b"bmp\x00".as_ptr() as *const c_char,
                width,
                align,
                height,
                format,
                0i32,
            ) == -1i32
            {
                return -1i32;
            }
            printf(b"OK.\n\x00".as_ptr() as *const c_char);
            printf(
                
                b"%s Bottom-Up PPM (row alignment = %d bytes)  ...  \x00".as_ptr()
                    as *const c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                
                b"ppm\x00".as_ptr() as *const c_char,
                width,
                align,
                height,
                format,
                TJFLAG_BOTTOMUP,
            ) == -1i32
            {
                return -1i32;
            }
            printf(b"OK.\n\x00".as_ptr() as *const c_char);
            format += 1
        }
        align *= 2i32
    }
    return 0i32;
}

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    
     let mut num4bf:  c_int =  5i32;
    if argc > 1i32 {
          let mut i:   c_int =  1i32;
        while i < argc {
            if strcasecmp(
                *argv.offset(i as isize),
                
                b"-yuv\x00".as_ptr() as *const c_char,
            ) == 0
            {
                doYUV = 1i32
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-noyuvpad\x00".as_ptr() as *const c_char,
            ) == 0
            {
                pad = 1i32
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-alloc\x00".as_ptr() as *const c_char,
            ) == 0
            {
                alloc = 1i32
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-bmp\x00".as_ptr() as *const c_char,
            ) == 0
            {
                return bmpTest();
            } else {
                usage(*argv.offset(0));
            }
            i += 1
        }
    }
    if alloc != 0 {
        printf(
            
            b"Testing automatic buffer allocation\n\x00".as_ptr() as *const c_char,
        );
    }
    if doYUV != 0 {
        num4bf = 4i32
    }
    doTest(
        35i32,
        39i32,
        _3byteFormats.as_ptr(),
        2i32,
        TJSAMP_444 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        39i32,
        41i32,
        _4byteFormats.as_ptr(),
        num4bf,
        TJSAMP_444 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        41i32,
        35i32,
        _3byteFormats.as_ptr(),
        2i32,
        TJSAMP_422 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        35i32,
        39i32,
        _4byteFormats.as_ptr(),
        num4bf,
        TJSAMP_422 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        39i32,
        41i32,
        _3byteFormats.as_ptr(),
        2i32,
        TJSAMP_420 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        41i32,
        35i32,
        _4byteFormats.as_ptr(),
        num4bf,
        TJSAMP_420 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        35i32,
        39i32,
        _3byteFormats.as_ptr(),
        2i32,
        TJSAMP_440 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        39i32,
        41i32,
        _4byteFormats.as_ptr(),
        num4bf,
        TJSAMP_440 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        41i32,
        35i32,
        _3byteFormats.as_ptr(),
        2i32,
        TJSAMP_411 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        35i32,
        39i32,
        _4byteFormats.as_ptr(),
        num4bf,
        TJSAMP_411 as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        39i32,
        41i32,
        _onlyGray.as_ptr(),
        1i32,
        TJSAMP_GRAY as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        41i32,
        35i32,
        _3byteFormats.as_ptr(),
        2i32,
        TJSAMP_GRAY as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    doTest(
        35i32,
        39i32,
        _4byteFormats.as_ptr(),
        4i32,
        TJSAMP_GRAY as c_int,
        
        
        
        b"test\x00".as_ptr() as *mut c_char,
    );
    bufSizeTest();
    if doYUV != 0 {
        printf(
            
            b"\n--------------------\n\n\x00".as_ptr() as *const c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            TJSAMP_444 as c_int,
            
            
            
            b"test_yuv0\x00".as_ptr() as *mut c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            TJSAMP_422 as c_int,
            
            
            
            b"test_yuv0\x00".as_ptr() as *mut c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            TJSAMP_420 as c_int,
            
            
            
            b"test_yuv0\x00".as_ptr() as *mut c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            TJSAMP_440 as c_int,
            
            
            
            b"test_yuv0\x00".as_ptr() as *mut c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            TJSAMP_411 as c_int,
            
            
            
            b"test_yuv0\x00".as_ptr() as *mut c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyRGB.as_ptr(),
            1i32,
            TJSAMP_GRAY as c_int,
            
            
            
            b"test_yuv0\x00".as_ptr() as *mut c_char,
        );
        doTest(
            48i32,
            48i32,
            _onlyGray.as_ptr(),
            1i32,
            TJSAMP_GRAY as c_int,
            
            
            
            b"test_yuv0\x00".as_ptr() as *mut c_char,
        );
    }
    return exitStatus;
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
