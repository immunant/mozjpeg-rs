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



























































































use std::prelude::v1::*;use std::prelude::v1;use libc::{c_long, c_char, c_int, c_ulong, c_uchar, c_short, c_double,
           c_void};use mozjpeg::*;use crate::stdlib::{__errno_location, memset, strcasecmp, strerror, strlen,
                    strncasecmp, strrchr};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE, atoi, exit,
                        fclose, fopen, fread, fseek, ftell, fwrite, printf,
                        sscanf, strtol, SEEK_END, SEEK_SET};pub use crate::src::turbojpeg::{tjAlloc, tjCompress2, tjDecompress2,
                                tjDecompressHeader3, tjDestroy, tjFree,
                                tjGetErrorStr2, tjGetScalingFactors,
                                tjInitCompress, tjInitDecompress,
                                tjInitTransform, tjLoadImage, tjPixelSize,
                                tjSaveImage, tjTransform, tjhandle, tjregion,
                                tjscalingfactor, tjtransform,
                                TJFLAG_ACCURATEDCT, TJFLAG_FASTDCT,
                                TJFLAG_FASTUPSAMPLE, TJPF, TJPF_ABGR,
                                TJPF_ARGB, TJPF_BGR, TJPF_BGRA, TJPF_BGRX,
                                TJPF_CMYK, TJPF_GRAY, TJPF_RGB, TJPF_RGBA,
                                TJPF_RGBX, TJPF_UNKNOWN, TJPF_XBGR, TJPF_XRGB,
                                TJSAMP, TJSAMP_411, TJSAMP_420, TJSAMP_422,
                                TJSAMP_440, TJSAMP_444, TJSAMP_GRAY, TJXOP,
                                TJXOPT_CROP, TJXOPT_GRAY, TJXOPT_TRIM,
                                TJXOP_HFLIP, TJXOP_NONE, TJXOP_ROT180,
                                TJXOP_ROT270, TJXOP_ROT90, TJXOP_TRANSPOSE,
                                TJXOP_TRANSVERSE, TJXOP_VFLIP};pub use crate::stddef_h::{size_t, NULL, NULL_0};
/*
 * Copyright (C)2011-2012, 2014-2015, 2017 D. R. Commander.
 *                                         All Rights Reserved.
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
 * This program demonstrates how to compress, decompress, and transform JPEG
 * images using the TurboJPEG C API
 */

pub const DEFAULT_SUBSAMP: c_int = TJSAMP_444 as c_int;

pub const DEFAULT_QUALITY: c_int = 95i32;
#[no_mangle]

pub static mut subsampName: [*const c_char; 6] = [
    
    b"4:4:4\x00".as_ptr() as *const c_char,
    
    b"4:2:2\x00".as_ptr() as *const c_char,
    
    b"4:2:0\x00".as_ptr() as *const c_char,
    
    b"Grayscale\x00".as_ptr() as *const c_char,
    
    b"4:4:0\x00".as_ptr() as *const c_char,
    
    b"4:1:1\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut colorspaceName: [*const c_char; 5] = [
    
    b"RGB\x00".as_ptr() as *const c_char,
    
    b"YCbCr\x00".as_ptr() as *const c_char,
    
    b"GRAY\x00".as_ptr() as *const c_char,
    
    b"CMYK\x00".as_ptr() as *const c_char,
    
    b"YCCK\x00".as_ptr() as *const c_char,
];
#[no_mangle]

pub static mut scalingFactors: *mut tjscalingfactor =
    NULL_0 as *mut tjscalingfactor;
#[no_mangle]

pub static mut numScalingFactors: c_int = 0i32;
/* DCT filter example.  This produces a negative of the image. */
#[no_mangle]

pub unsafe extern "C" fn customFilter(
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
#[no_mangle]

pub unsafe extern "C" fn usage(mut programName: *mut c_char) {
     
    printf(
        
        b"\nUSAGE: %s <Input image> <Output image> [options]\n\n\x00".as_ptr()
            as *const c_char,
        programName,
    );
    printf(
        
        b"Input and output images can be in Windows BMP or PBMPLUS (PPM/PGM) format.  If\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"either filename ends in a .jpg extension, then the TurboJPEG API will be used\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"to compress or decompress the image.\n\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"Compression Options (used if the output image is a JPEG image)\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"--------------------------------------------------------------\n\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-subsamp <444|422|420|gray> = Apply this level of chrominance subsampling when\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     compressing the output image.  The default is to use the same level of\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     subsampling as in the input image, if the input image is also a JPEG\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     image, or to use grayscale if the input image is a grayscale non-JPEG\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     image, or to use %s subsampling otherwise.\n\n\x00".as_ptr()
            as *const c_char,
        subsampName[DEFAULT_SUBSAMP as usize],
    );
    printf(
        
        b"-q <1-100> = Compress the output image with this JPEG quality level\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"     (default = %d).\n\n\x00".as_ptr() as *const c_char,
        DEFAULT_QUALITY,
    );
    printf(
        
        b"Decompression Options (used if the input image is a JPEG image)\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"---------------------------------------------------------------\n\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"-scale M/N = Scale the input image by a factor of M/N when decompressing it.\n\x00".as_ptr() as *const c_char,
    );
    printf(b"(M/N = \x00".as_ptr() as *const c_char);
     let mut i:   c_int =  0i32;
    while i < numScalingFactors {
        printf(
            
            b"%d/%d\x00".as_ptr() as *const c_char,
            (*scalingFactors.offset(i as isize)).num,
            (*scalingFactors.offset(i as isize)).denom,
        );
        if numScalingFactors == 2i32 && i != numScalingFactors - 1i32 {
            printf(b" or \x00".as_ptr() as *const c_char);
        } else if numScalingFactors > 2i32 {
            if i != numScalingFactors - 1i32 {
                printf(b", \x00".as_ptr() as *const c_char);
            }
            if i == numScalingFactors - 2i32 {
                printf(b"or \x00".as_ptr() as *const c_char);
            }
        }
        i += 1
    }
    printf(b")\n\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270 =\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"     Perform one of these lossless transform operations on the input image\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     prior to decompressing it (these options are mutually exclusive.)\n\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-grayscale = Perform lossless grayscale conversion on the input image prior\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     to decompressing it (can be combined with the other transform operations\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     above.)\n\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-crop WxH+X+Y = Perform lossless cropping on the input image prior to\n\x00".as_ptr()
            as *const c_char,
    );
    printf(
        
        b"     decompressing it.  X and Y specify the upper left corner of the cropping\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     region, and W and H specify the width and height of the cropping region.\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     X and Y must be evenly divible by the MCU block size (8x8 if the input\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     image was compressed using no subsampling or grayscale, 16x8 if it was\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     compressed using 4:2:2 subsampling, or 16x16 if it was compressed using\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     4:2:0 subsampling.)\n\n\x00".as_ptr() as *const c_char);
    printf(b"General Options\n\x00".as_ptr() as *const c_char);
    printf(b"---------------\n\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-fastupsample = Use the fastest chrominance upsampling algorithm available in\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"     the underlying codec.\n\n\x00".as_ptr() as *const c_char,
    );
    printf(
        
        b"-fastdct = Use the fastest DCT/IDCT algorithms available in the underlying\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     codec.\n\n\x00".as_ptr() as *const c_char);
    printf(
        
        b"-accuratedct = Use the most accurate DCT/IDCT algorithms available in the\n\x00".as_ptr() as *const c_char,
    );
    printf(b"     underlying codec.\n\n\x00".as_ptr() as *const c_char);
    exit(1i32);
}

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
     let mut retval:  c_int =  0i32;
    let mut scalingFactor: tjscalingfactor = {
         let mut init =
     tjscalingfactor{num:  1i32, denom:  1i32,};
        init
    };
    let mut outSubsamp: c_int = -1i32;
    let mut outQual: c_int = -1i32;
    let mut xform: tjtransform = tjtransform{r:  tjregion{x:  0, y:  0, w:  0, h:  0,},
            op:  0,
            options:  0,
            data:  ::std::ptr::null_mut::< c_void>(),
            customFilter:  None,};
    
    
    
    
    
    let mut jpegFile: *mut FILE =
        NULL_0 as *mut FILE;
    let mut imgBuf: *mut c_uchar = NULL_0 as *mut c_uchar;
    let mut jpegBuf: *mut c_uchar = NULL_0 as *mut c_uchar;
    
    
    let mut pixelFormat: c_int =  TJPF_UNKNOWN;
    let mut tjInstance: tjhandle =
        NULL_0 as *mut c_void;
    scalingFactors = tjGetScalingFactors(&mut numScalingFactors);
    if scalingFactors.is_null() {
        printf(
            
            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
            175i32,
            
            b"getting scaling factors\x00".as_ptr() as *const c_char,
            tjGetErrorStr2(tjInstance),
        );
        retval = -1i32
    } else {
         let mut current_block:  u64; let mut flags:  c_int =  0i32; let mut width:  c_int =  0; let mut height:  c_int =  0;   memset(
            &mut xform as *mut tjtransform as *mut c_void,
            0i32,
            ::std::mem::size_of::<tjtransform>() as c_ulong,
        );
        if argc < 3i32 {
            usage(*argv.offset(0));
        }
         let mut i:   c_int =  3i32;
        while i < argc {
            if strncasecmp(
                *argv.offset(i as isize),
                
                b"-sc\x00".as_ptr() as *const c_char,
                3u64,
            ) == 0
                && i < argc - 1i32
            {
                
                
                
                 let mut match_0:  c_int =  0i32; let mut temp1:  c_int =  0i32; let mut temp2:  c_int =  0i32; 
                i += 1;
                if sscanf(
                    *argv.offset(i as isize),
                    
                    b"%d/%d\x00".as_ptr() as *const c_char,
                    &mut temp1 as *mut c_int,
                    &mut temp2 as *mut c_int,
                ) < 2i32
                {
                    usage(*argv.offset(0));
                }
                 let mut j:   c_int =  0i32;
                while j < numScalingFactors {
                    if temp1 as c_double / temp2 as c_double
                        == (*scalingFactors.offset(j as isize)).num as c_double
                            / (*scalingFactors.offset(j as isize)).denom as c_double
                    {
                        scalingFactor = *scalingFactors.offset(j as isize);
                        match_0 = 1i32;
                        break;
                    } else {
                        j += 1
                    }
                }
                if match_0 != 1i32 {
                    usage(*argv.offset(0));
                }
            } else if strncasecmp(
                *argv.offset(i as isize),
                
                b"-su\x00".as_ptr() as *const c_char,
                3u64,
            ) == 0
                && i < argc - 1i32
            {
                i += 1;
                if strncasecmp(
                    *argv.offset(i as isize),
                    
                    b"g\x00".as_ptr() as *const c_char,
                    1u64,
                ) == 0
                {
                    outSubsamp = TJSAMP_GRAY as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"444\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    outSubsamp = TJSAMP_444 as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"422\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    outSubsamp = TJSAMP_422 as c_int
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    
                    b"420\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    outSubsamp = TJSAMP_420 as c_int
                } else {
                    usage(*argv.offset(0));
                }
            } else if strncasecmp(
                *argv.offset(i as isize),
                
                b"-q\x00".as_ptr() as *const c_char,
                2u64,
            ) == 0
                && i < argc - 1i32
            {
                i += 1;
                outQual = atoi(*argv.offset(i as isize));
                if outQual < 1i32 || outQual > 100i32 {
                    usage(*argv.offset(0));
                }
            } else if strncasecmp(
                *argv.offset(i as isize),
                
                b"-g\x00".as_ptr() as *const c_char,
                2u64,
            ) == 0
            {
                xform.options |= TJXOPT_GRAY
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-hflip\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.op = TJXOP_HFLIP as c_int
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-vflip\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.op = TJXOP_VFLIP as c_int
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-transpose\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.op = TJXOP_TRANSPOSE as c_int
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-transverse\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.op = TJXOP_TRANSVERSE as c_int
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-rot90\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.op = TJXOP_ROT90 as c_int
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-rot180\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.op = TJXOP_ROT180 as c_int
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-rot270\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.op = TJXOP_ROT270 as c_int
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-custom\x00".as_ptr() as *const c_char,
            ) == 0
            {
                xform.customFilter = Some(
                    customFilter
                        as unsafe extern "C" fn(
                            _: *mut c_short,
                            _: tjregion,
                            _: tjregion,
                            _: c_int,
                            _: c_int,
                            _: *mut tjtransform,
                        ) -> c_int,
                )
            } else if strncasecmp(
                *argv.offset(i as isize),
                
                b"-c\x00".as_ptr() as *const c_char,
                2u64,
            ) == 0
                && i < argc - 1i32
            {
                i += 1;
                if sscanf(
                    *argv.offset(i as isize),
                    
                    b"%dx%d+%d+%d\x00".as_ptr() as *const c_char,
                    &mut xform.r.w as *mut c_int,
                    &mut xform.r.h as *mut c_int,
                    &mut xform.r.x as *mut c_int,
                    &mut xform.r.y as *mut c_int,
                ) < 4i32
                    || xform.r.x < 0i32
                    || xform.r.y < 0i32
                    || xform.r.w < 1i32
                    || xform.r.h < 1i32
                {
                    usage(*argv.offset(0));
                }
                xform.options |= TJXOPT_CROP
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-fastupsample\x00".as_ptr() as *const c_char,
            ) == 0
            {
                printf(
                    
                    b"Using fast upsampling code\n\x00".as_ptr() as *const c_char,
                );
                flags |= TJFLAG_FASTUPSAMPLE
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-fastdct\x00".as_ptr() as *const c_char,
            ) == 0
            {
                printf(
                    
                    b"Using fastest DCT/IDCT algorithm\n\x00".as_ptr() as *const c_char,
                );
                flags |= TJFLAG_FASTDCT
            } else if strcasecmp(
                *argv.offset(i as isize),
                
                b"-accuratedct\x00".as_ptr() as *const c_char,
            ) == 0
            {
                printf(
                    
                    b"Using most accurate DCT/IDCT algorithm\n\x00".as_ptr()
                        as *const c_char,
                );
                flags |= TJFLAG_ACCURATEDCT
            } else {
                usage(*argv.offset(0));
            }
            i += 1
        }
        
         let mut inFormat:   *mut c_char =
     strrchr(*argv.offset(1), '.' as i32); let mut outFormat:   *mut c_char =
     strrchr(*argv.offset(2), '.' as i32);
        if inFormat.is_null()
            || outFormat.is_null()
            || strlen(inFormat) < 2u64
            || strlen(outFormat) < 2u64
        {
            usage(*argv.offset(0));
        }
        inFormat = &mut *inFormat.offset(1) as *mut c_char;
        outFormat = &mut *outFormat.offset(1) as *mut c_char;
        if strcasecmp(inFormat,  b"jpg\x00".as_ptr() as *const c_char) == 0
        {
            
            
             let mut size:  c_long =  0;
            let mut doTransform: c_int =
                (xform.op != TJXOP_NONE as c_int
                    || xform.options != 0i32
                    || xform.customFilter.is_some()) as c_int;
            
            /* Read the JPEG file into memory. */
            jpegFile = fopen(
                *argv.offset(1),
                
                b"rb\x00".as_ptr() as *const c_char,
            );
            if jpegFile.is_null() {
                printf(
                    
                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                    269i32,
                    
                    b"opening input file\x00".as_ptr() as *const c_char,
                    strerror(*__errno_location()),
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else if fseek(jpegFile, 0i64, SEEK_END)
                < 0i32
                || {
                    size = ftell(jpegFile);
                    (size) < 0i64
                }
                || fseek(jpegFile, 0i64, SEEK_SET)
                    < 0i32
            {
                printf(
                    
                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                    272i32,
                    
                    b"determining input file size\x00".as_ptr() as *const c_char,
                    strerror(*__errno_location()),
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else if size == 0i64 {
                printf(
                    
                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                    274i32,
                    
                    b"determining input file size\x00".as_ptr() as *const c_char,
                    
                    b"Input file contains no data\x00".as_ptr() as *const c_char,
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else {
                  let mut jpegSize:   c_ulong =  size as c_ulong;
                jpegBuf = tjAlloc(jpegSize as c_int);
                if jpegBuf.is_null() {
                    printf(
                        
                        b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                        277i32,
                        
                        b"allocating JPEG buffer\x00".as_ptr() as *const c_char,
                        strerror(*__errno_location()),
                    );
                    retval = -1i32;
                    current_block = 16288987300638808654;
                } else if fread(
                    jpegBuf as *mut c_void,
                    jpegSize,
                    1u64,
                    jpegFile,
                ) < 1u64
                {
                    printf(
                        
                        b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                        279i32,
                        
                        b"reading input file\x00".as_ptr() as *const c_char,
                        strerror(*__errno_location()),
                    );
                    retval = -1i32;
                    current_block = 16288987300638808654;
                } else {
                    fclose(jpegFile);
                    jpegFile = NULL_0 as *mut FILE;
                    if doTransform != 0 {
                        /* Transform it. */
                        let mut dstBuf: *mut c_uchar =
                            NULL_0 as *mut c_uchar; /* Dynamically allocate the JPEG buffer */
                        
                        tjInstance = tjInitTransform();
                        if tjInstance.is_null() {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                288i32,
                                
                                b"initializing transformer\x00".as_ptr() as *const c_char,
                                tjGetErrorStr2(tjInstance),
                            );
                            retval = -1i32;
                            current_block = 16288987300638808654;
                        } else {
                             let mut dstSize:  c_ulong =  0u64;xform.options |= TJXOPT_TRIM;
                            if tjTransform(
                                tjInstance,
                                jpegBuf,
                                jpegSize,
                                1i32,
                                &mut dstBuf,
                                &mut dstSize,
                                &mut xform,
                                flags,
                            ) < 0i32
                            {
                                printf(
                                    
                                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    292i32,
                                    
                                    b"transforming input image\x00".as_ptr()
                                        as *const c_char,
                                    tjGetErrorStr2(tjInstance),
                                );
                                retval = -1i32;
                                current_block = 16288987300638808654;
                            } else {
                                tjFree(jpegBuf);
                                jpegBuf = dstBuf;
                                jpegSize = dstSize;
                                current_block = 7244994750255146185;
                            }
                        }
                    } else {
                        tjInstance = tjInitDecompress();
                        if tjInstance.is_null() {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                298i32,
                                
                                b"initializing decompressor\x00".as_ptr()
                                    as *const c_char,
                                tjGetErrorStr2(tjInstance),
                            );
                            retval = -1i32;
                            current_block = 16288987300638808654;
                        } else {
                            current_block = 7244994750255146185;
                        }
                    }
                    match current_block {
                        16288987300638808654 => {}
                        _ => {
                             let mut inSubsamp:  c_int =  0; let mut inColorspace:  c_int =  0;if tjDecompressHeader3(
                                tjInstance,
                                jpegBuf,
                                jpegSize,
                                &mut width,
                                &mut height,
                                &mut inSubsamp,
                                &mut inColorspace,
                            ) < 0i32
                            {
                                printf(
                                    
                                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                        as *const c_char,
                                    303i32,
                                    
                                    b"reading JPEG header\x00".as_ptr() as *const c_char,
                                    tjGetErrorStr2(tjInstance),
                                );
                                retval = -1i32;
                                current_block = 16288987300638808654;
                            } else {
                                printf(b"%s Image:  %d x %d pixels, %s subsampling, %s colorspace\n\x00".as_ptr() as
                                           *const c_char,
                                       if doTransform != 0 {
                                           
                                           b"Transformed\x00".as_ptr() as
                                               *const c_char
                                       } else {
                                           
                                           b"Input\x00".as_ptr() as
                                               *const c_char
                                       }, width, height,
                                       subsampName[inSubsamp as usize],
                                       colorspaceName[inColorspace as usize]);
                                if strcasecmp(
                                    outFormat,
                                    
                                    b"jpg\x00".as_ptr() as *const c_char,
                                ) == 0
                                    && doTransform != 0
                                    && scalingFactor.num == 1i32
                                    && scalingFactor.denom == 1i32
                                    && outSubsamp < 0i32
                                    && outQual < 0i32
                                {
                                    /* Input image has been transformed, and no re-compression options
                                    have been selected.  Write the transformed image to disk and exit. */
                                    jpegFile = fopen(
                                        *argv.offset(2),
                                        
                                        b"wb\x00".as_ptr() as *const c_char,
                                    );
                                    if jpegFile.is_null() {
                                        printf(
                                            
                                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                as *const c_char,
                                            315i32,
                                            
                                            b"opening output file\x00".as_ptr()
                                                as *const c_char,
                                            strerror(
                                                *__errno_location(),
                                            ),
                                        );
                                        retval = -1i32
                                    } else if fwrite(
                                        jpegBuf as *const c_void,
                                        jpegSize,
                                        1u64,
                                        jpegFile,
                                    ) < 1u64
                                    {
                                        printf(
                                            
                                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                as *const c_char,
                                            317i32,
                                            
                                            b"writing output file\x00".as_ptr()
                                                as *const c_char,
                                            strerror(
                                                *__errno_location(),
                                            ),
                                        );
                                        retval = -1i32
                                    } else {
                                        fclose(jpegFile);
                                        jpegFile =
                                            NULL_0 as *mut FILE
                                    }
                                    current_block = 16288987300638808654;
                                } else {
                                    /* Scaling and/or a non-JPEG output image format and/or compression options
                                    have been selected, so we need to decompress the input/transformed
                                    image. */
                                    width = (width * scalingFactor.num + scalingFactor.denom
                                        - 1i32)
                                        / scalingFactor.denom;
                                    height = (height * scalingFactor.num + scalingFactor.denom
                                        - 1i32)
                                        / scalingFactor.denom;
                                    if outSubsamp < 0i32 {
                                        outSubsamp = inSubsamp
                                    }
                                    pixelFormat =  TJPF_BGRX;
                                    imgBuf = tjAlloc(
                                        width
                                            * height
                                            * tjPixelSize
                                                [pixelFormat as usize],
                                    );
                                    if imgBuf.is_null() {
                                        printf(
                                            
                                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                as *const c_char,
                                            333i32,
                                            
                                            b"allocating uncompressed image buffer\x00".as_ptr()
                                                as *const c_char,
                                            strerror(
                                                *__errno_location(),
                                            ),
                                        );
                                        retval = -1i32;
                                        current_block = 16288987300638808654;
                                    } else if tjDecompress2(
                                        tjInstance,
                                        jpegBuf,
                                        jpegSize,
                                        imgBuf,
                                        width,
                                        0i32,
                                        height,
                                        pixelFormat,
                                        flags,
                                    ) < 0i32
                                    {
                                        printf(
                                            
                                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                                as *const c_char,
                                            337i32,
                                            
                                            b"decompressing JPEG image\x00".as_ptr()
                                                as *const c_char,
                                            tjGetErrorStr2(tjInstance),
                                        );
                                        retval = -1i32;
                                        current_block = 16288987300638808654;
                                    } else {
                                        tjFree(jpegBuf);
                                        jpegBuf = NULL_0 as *mut c_uchar;
                                        tjDestroy(tjInstance);
                                        tjInstance = NULL_0 as *mut c_void;
                                        current_block = 3098209481605707636;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            /* Input image is not a JPEG image.  Load it into memory. */
            imgBuf = tjLoadImage(
                *argv.offset(1),
                &mut width,
                1i32,
                &mut height,
                &mut pixelFormat,
                0i32,
            );
            if imgBuf.is_null() {
                printf(
                    
                    b"ERROR in line %d while %s:\n%s\n\x00".as_ptr() as *const c_char,
                    344i32,
                    
                    b"loading input image\x00".as_ptr() as *const c_char,
                    tjGetErrorStr2(tjInstance),
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else {
                if outSubsamp < 0i32 {
                    if pixelFormat ==  TJPF_GRAY {
                        outSubsamp = TJSAMP_GRAY as c_int
                    } else {
                        outSubsamp = TJSAMP_444 as c_int
                    }
                }
                printf(
                    
                    b"Input Image:  %d x %d pixels\n\x00".as_ptr() as *const c_char,
                    width,
                    height,
                );
                current_block = 3098209481605707636;
            }
        }
        match current_block {
            16288987300638808654 => {}
            _ => {
                printf(
                    
                    b"Output Image (%s):  %d x %d pixels\x00".as_ptr() as *const c_char,
                    outFormat,
                    width,
                    height,
                );
                if strcasecmp(
                    outFormat,
                    
                    b"jpg\x00".as_ptr() as *const c_char,
                ) == 0
                {
                    /* Output image format is JPEG.  Compress the uncompressed image. */
                     let mut jpegSize_0:  c_ulong =  0u64;let mut jpegBuf_0: *mut c_uchar =
                        NULL_0 as *mut c_uchar; /* Dynamically allocate the JPEG buffer */
                    
                    if outQual < 0i32 {
                        outQual = DEFAULT_QUALITY
                    }
                    printf(
                        
                        b", %s subsampling, quality = %d\n\x00".as_ptr() as *const c_char,
                        subsampName[outSubsamp as usize],
                        outQual,
                    );
                    tjInstance = tjInitCompress();
                    if tjInstance.is_null() {
                        printf(
                            
                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                as *const c_char,
                            367i32,
                            
                            b"initializing compressor\x00".as_ptr() as *const c_char,
                            tjGetErrorStr2(tjInstance),
                        );
                        retval = -1i32
                    } else if tjCompress2(
                        tjInstance,
                        imgBuf,
                        width,
                        0i32,
                        height,
                        pixelFormat,
                        &mut jpegBuf_0,
                        &mut jpegSize_0,
                        outSubsamp,
                        outQual,
                        flags,
                    ) < 0i32
                    {
                        printf(
                            
                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                as *const c_char,
                            370i32,
                            
                            b"compressing image\x00".as_ptr() as *const c_char,
                            tjGetErrorStr2(tjInstance),
                        );
                        retval = -1i32
                    } else {
                        tjDestroy(tjInstance);
                        tjInstance = NULL_0 as *mut c_void;
                        /* Write the JPEG image to disk. */
                        jpegFile = fopen(
                            *argv.offset(2),
                            
                            b"wb\x00".as_ptr() as *const c_char,
                        );
                        if jpegFile.is_null() {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                375i32,
                                
                                b"opening output file\x00".as_ptr() as *const c_char,
                                strerror(*__errno_location()),
                            );
                            retval = -1i32
                        } else if fwrite(
                            jpegBuf_0 as *const c_void,
                            jpegSize_0,
                            1u64,
                            jpegFile,
                        ) < 1u64
                        {
                            printf(
                                
                                b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                    as *const c_char,
                                377i32,
                                
                                b"writing output file\x00".as_ptr() as *const c_char,
                                strerror(*__errno_location()),
                            );
                            retval = -1i32
                        } else {
                            tjDestroy(tjInstance);
                            tjInstance = NULL_0 as *mut c_void;
                            fclose(jpegFile);
                            jpegFile = NULL_0 as *mut FILE;
                            tjFree(jpegBuf_0);
                            jpegBuf_0 = NULL_0 as *mut c_uchar
                        }
                    }
                } else {
                    /* Output image format is not JPEG.  Save the uncompressed image
                    directly to disk. */
                    printf(b"\n\x00".as_ptr() as *const c_char);
                    if tjSaveImage(
                        *argv.offset(2),
                        imgBuf,
                        width,
                        0i32,
                        height,
                        pixelFormat,
                        0i32,
                    ) < 0i32
                    {
                        printf(
                            
                            b"ERROR in line %d while %s:\n%s\n\x00".as_ptr()
                                as *const c_char,
                            386i32,
                            
                            b"saving output image\x00".as_ptr() as *const c_char,
                            tjGetErrorStr2(tjInstance),
                        );
                        retval = -1i32
                    }
                }
            }
        }
    }
    if !imgBuf.is_null() {
        tjFree(imgBuf);
    }
    if !tjInstance.is_null() {
        tjDestroy(tjInstance);
    }
    if !jpegBuf.is_null() {
        tjFree(jpegBuf);
    }
    if !jpegFile.is_null() {
        fclose(jpegFile);
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
