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


use mozjpeg::*;


pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stddef_h::NULL_0;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::src::turbojpeg::tjAlloc;
pub use crate::src::turbojpeg::tjCompress2;
pub use crate::src::turbojpeg::tjDecompress2;
pub use crate::src::turbojpeg::tjDecompressHeader3;
pub use crate::src::turbojpeg::tjDestroy;
pub use crate::src::turbojpeg::tjFree;
pub use crate::src::turbojpeg::tjGetErrorStr2;
pub use crate::src::turbojpeg::tjGetScalingFactors;
pub use crate::src::turbojpeg::tjInitCompress;
pub use crate::src::turbojpeg::tjInitDecompress;
pub use crate::src::turbojpeg::tjInitTransform;
pub use crate::src::turbojpeg::tjLoadImage;
pub use crate::src::turbojpeg::tjPixelSize;
pub use crate::src::turbojpeg::tjSaveImage;
pub use crate::src::turbojpeg::tjTransform;
pub use crate::src::turbojpeg::tjhandle;
pub use crate::src::turbojpeg::tjregion;
pub use crate::src::turbojpeg::tjscalingfactor;
pub use crate::src::turbojpeg::tjtransform;
pub use crate::src::turbojpeg::TJFLAG_ACCURATEDCT;
pub use crate::src::turbojpeg::TJFLAG_FASTDCT;
pub use crate::src::turbojpeg::TJFLAG_FASTUPSAMPLE;
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
pub use crate::src::turbojpeg::TJXOPT_CROP;
pub use crate::src::turbojpeg::TJXOPT_GRAY;
pub use crate::src::turbojpeg::TJXOPT_TRIM;
pub use crate::src::turbojpeg::TJXOP_HFLIP;
pub use crate::src::turbojpeg::TJXOP_NONE;
pub use crate::src::turbojpeg::TJXOP_ROT180;
pub use crate::src::turbojpeg::TJXOP_ROT270;
pub use crate::src::turbojpeg::TJXOP_ROT90;
pub use crate::src::turbojpeg::TJXOP_TRANSPOSE;
pub use crate::src::turbojpeg::TJXOP_TRANSVERSE;
pub use crate::src::turbojpeg::TJXOP_VFLIP;
use crate::stdlib::__errno_location;
pub use crate::stdlib::atoi;
pub use crate::stdlib::exit;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fread;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::fwrite;
use crate::stdlib::memset;
pub use crate::stdlib::printf;
pub use crate::stdlib::sscanf;
use crate::stdlib::strcasecmp;
use crate::stdlib::strerror;
use crate::stdlib::strlen;
use crate::stdlib::strncasecmp;
use crate::stdlib::strrchr;
pub use crate::stdlib::strtol;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;
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

pub const DEFAULT_SUBSAMP: libc::c_int = crate::src::turbojpeg::TJSAMP_444 as libc::c_int;

pub const DEFAULT_QUALITY: libc::c_int = 95i32;
#[no_mangle]

pub static mut subsampName: [*const libc::c_char; 6] = [
    b"4:4:4\x00" as *const u8 as *const libc::c_char,
    b"4:2:2\x00" as *const u8 as *const libc::c_char,
    b"4:2:0\x00" as *const u8 as *const libc::c_char,
    b"Grayscale\x00" as *const u8 as *const libc::c_char,
    b"4:4:0\x00" as *const u8 as *const libc::c_char,
    b"4:1:1\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut colorspaceName: [*const libc::c_char; 5] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"YCbCr\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
    b"YCCK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut scalingFactors: *mut crate::src::turbojpeg::tjscalingfactor =
    crate::stddef_h::NULL_0 as *mut crate::src::turbojpeg::tjscalingfactor;
#[no_mangle]

pub static mut numScalingFactors: libc::c_int = 0i32;
/* DCT filter example.  This produces a negative of the image. */
#[no_mangle]

pub unsafe extern "C" fn customFilter(
    mut coeffs: *mut libc::c_short,
    mut arrayRegion: crate::src::turbojpeg::tjregion,
    mut planeRegion: crate::src::turbojpeg::tjregion,
    mut componentIndex: libc::c_int,
    mut transformIndex: libc::c_int,
    mut transform: *mut crate::src::turbojpeg::tjtransform,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < arrayRegion.w * arrayRegion.h {
        *coeffs.offset(i as isize) = -(*coeffs.offset(i as isize) as libc::c_int) as libc::c_short;
        i += 1
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn usage(mut programName: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    crate::stdlib::printf(
        b"\nUSAGE: %s <Input image> <Output image> [options]\n\n\x00" as *const u8
            as *const libc::c_char,
        programName,
    );
    crate::stdlib::printf(
        b"Input and output images can be in Windows BMP or PBMPLUS (PPM/PGM) format.  If\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"either filename ends in a .jpg extension, then the TurboJPEG API will be used\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"to compress or decompress the image.\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"Compression Options (used if the output image is a JPEG image)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"--------------------------------------------------------------\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-subsamp <444|422|420|gray> = Apply this level of chrominance subsampling when\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compressing the output image.  The default is to use the same level of\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     subsampling as in the input image, if the input image is also a JPEG\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     image, or to use grayscale if the input image is a grayscale non-JPEG\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     image, or to use %s subsampling otherwise.\n\n\x00" as *const u8
            as *const libc::c_char,
        subsampName[DEFAULT_SUBSAMP as usize],
    );
    crate::stdlib::printf(
        b"-q <1-100> = Compress the output image with this JPEG quality level\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     (default = %d).\n\n\x00" as *const u8 as *const libc::c_char,
        DEFAULT_QUALITY,
    );
    crate::stdlib::printf(
        b"Decompression Options (used if the input image is a JPEG image)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"---------------------------------------------------------------\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-scale M/N = Scale the input image by a factor of M/N when decompressing it.\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"(M/N = \x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < numScalingFactors {
        crate::stdlib::printf(
            b"%d/%d\x00" as *const u8 as *const libc::c_char,
            (*scalingFactors.offset(i as isize)).num,
            (*scalingFactors.offset(i as isize)).denom,
        );
        if numScalingFactors == 2i32 && i != numScalingFactors - 1i32 {
            crate::stdlib::printf(b" or \x00" as *const u8 as *const libc::c_char);
        } else if numScalingFactors > 2i32 {
            if i != numScalingFactors - 1i32 {
                crate::stdlib::printf(b", \x00" as *const u8 as *const libc::c_char);
            }
            if i == numScalingFactors - 2i32 {
                crate::stdlib::printf(b"or \x00" as *const u8 as *const libc::c_char);
            }
        }
        i += 1
    }
    crate::stdlib::printf(b")\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270 =\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     Perform one of these lossless transform operations on the input image\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     prior to decompressing it (these options are mutually exclusive.)\n\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-grayscale = Perform lossless grayscale conversion on the input image prior\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     to decompressing it (can be combined with the other transform operations\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     above.)\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-crop WxH+X+Y = Perform lossless cropping on the input image prior to\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     decompressing it.  X and Y specify the upper left corner of the cropping\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     region, and W and H specify the width and height of the cropping region.\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     X and Y must be evenly divible by the MCU block size (8x8 if the input\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     image was compressed using no subsampling or grayscale, 16x8 if it was\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compressed using 4:2:2 subsampling, or 16x16 if it was compressed using\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     4:2:0 subsampling.)\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(b"General Options\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(b"---------------\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-fastupsample = Use the fastest chrominance upsampling algorithm available in\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     the underlying codec.\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-fastdct = Use the fastest DCT/IDCT algorithms available in the underlying\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     codec.\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-accuratedct = Use the most accurate DCT/IDCT algorithms available in the\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     underlying codec.\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::exit(1i32);
}

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut scalingFactor: crate::src::turbojpeg::tjscalingfactor = {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 1i32,
            denom: 1i32,
        };
        init
    };
    let mut outSubsamp: libc::c_int = -1i32;
    let mut outQual: libc::c_int = -1i32;
    let mut xform: crate::src::turbojpeg::tjtransform = crate::src::turbojpeg::tjtransform {
        r: crate::src::turbojpeg::tjregion {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        },
        op: 0,
        options: 0,
        data: 0 as *mut libc::c_void,
        customFilter: None,
    };
    let mut flags: libc::c_int = 0i32;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut inFormat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outFormat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut jpegFile: *mut crate::stdlib::FILE =
        crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut imgBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut jpegBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut retval: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut pixelFormat: libc::c_int = crate::src::turbojpeg::TJPF_UNKNOWN as libc::c_int;
    let mut tjInstance: crate::src::turbojpeg::tjhandle =
        crate::stddef_h::NULL_0 as *mut libc::c_void;
    scalingFactors = crate::src::turbojpeg::tjGetScalingFactors(&mut numScalingFactors);
    if scalingFactors.is_null() {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            175i32,
            b"getting scaling factors\x00" as *const u8 as *const libc::c_char,
            crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
        );
        retval = -1i32
    } else {
        crate::stdlib::memset(
            &mut xform as *mut crate::src::turbojpeg::tjtransform as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::src::turbojpeg::tjtransform>() as libc::c_ulong,
        );
        if argc < 3i32 {
            usage(*argv.offset(0));
        }
        /* Parse arguments. */
        i = 3i32;
        while i < argc {
            if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-sc\x00" as *const u8 as *const libc::c_char,
                3i32 as libc::c_ulong,
            ) == 0
                && i < argc - 1i32
            {
                let mut match_0: libc::c_int = 0i32;
                let mut temp1: libc::c_int = 0i32;
                let mut temp2: libc::c_int = 0i32;
                let mut j: libc::c_int = 0;
                i += 1;
                if crate::stdlib::sscanf(
                    *argv.offset(i as isize),
                    b"%d/%d\x00" as *const u8 as *const libc::c_char,
                    &mut temp1 as *mut libc::c_int,
                    &mut temp2 as *mut libc::c_int,
                ) < 2i32
                {
                    usage(*argv.offset(0));
                }
                j = 0i32;
                while j < numScalingFactors {
                    if temp1 as libc::c_double / temp2 as libc::c_double
                        == (*scalingFactors.offset(j as isize)).num as libc::c_double
                            / (*scalingFactors.offset(j as isize)).denom as libc::c_double
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
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-su\x00" as *const u8 as *const libc::c_char,
                3i32 as libc::c_ulong,
            ) == 0
                && i < argc - 1i32
            {
                i += 1;
                if crate::stdlib::strncasecmp(
                    *argv.offset(i as isize),
                    b"g\x00" as *const u8 as *const libc::c_char,
                    1i32 as libc::c_ulong,
                ) == 0
                {
                    outSubsamp = crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"444\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    outSubsamp = crate::src::turbojpeg::TJSAMP_444 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"422\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    outSubsamp = crate::src::turbojpeg::TJSAMP_422 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"420\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    outSubsamp = crate::src::turbojpeg::TJSAMP_420 as libc::c_int
                } else {
                    usage(*argv.offset(0));
                }
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-q\x00" as *const u8 as *const libc::c_char,
                2i32 as libc::c_ulong,
            ) == 0
                && i < argc - 1i32
            {
                i += 1;
                outQual = crate::stdlib::atoi(*argv.offset(i as isize));
                if outQual < 1i32 || outQual > 100i32 {
                    usage(*argv.offset(0));
                }
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-g\x00" as *const u8 as *const libc::c_char,
                2i32 as libc::c_ulong,
            ) == 0
            {
                xform.options |= crate::src::turbojpeg::TJXOPT_GRAY
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-hflip\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::src::turbojpeg::TJXOP_HFLIP as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-vflip\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::src::turbojpeg::TJXOP_VFLIP as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-transpose\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::src::turbojpeg::TJXOP_TRANSPOSE as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-transverse\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::src::turbojpeg::TJXOP_TRANSVERSE as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-rot90\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::src::turbojpeg::TJXOP_ROT90 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-rot180\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::src::turbojpeg::TJXOP_ROT180 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-rot270\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::src::turbojpeg::TJXOP_ROT270 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-custom\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.customFilter = Some(
                    customFilter
                        as unsafe extern "C" fn(
                            _: *mut libc::c_short,
                            _: crate::src::turbojpeg::tjregion,
                            _: crate::src::turbojpeg::tjregion,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut crate::src::turbojpeg::tjtransform,
                        ) -> libc::c_int,
                )
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-c\x00" as *const u8 as *const libc::c_char,
                2i32 as libc::c_ulong,
            ) == 0
                && i < argc - 1i32
            {
                i += 1;
                if crate::stdlib::sscanf(
                    *argv.offset(i as isize),
                    b"%dx%d+%d+%d\x00" as *const u8 as *const libc::c_char,
                    &mut xform.r.w as *mut libc::c_int,
                    &mut xform.r.h as *mut libc::c_int,
                    &mut xform.r.x as *mut libc::c_int,
                    &mut xform.r.y as *mut libc::c_int,
                ) < 4i32
                    || xform.r.x < 0i32
                    || xform.r.y < 0i32
                    || xform.r.w < 1i32
                    || xform.r.h < 1i32
                {
                    usage(*argv.offset(0));
                }
                xform.options |= crate::src::turbojpeg::TJXOPT_CROP
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-fastupsample\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::stdlib::printf(
                    b"Using fast upsampling code\n\x00" as *const u8 as *const libc::c_char,
                );
                flags |= crate::src::turbojpeg::TJFLAG_FASTUPSAMPLE
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-fastdct\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::stdlib::printf(
                    b"Using fastest DCT/IDCT algorithm\n\x00" as *const u8 as *const libc::c_char,
                );
                flags |= crate::src::turbojpeg::TJFLAG_FASTDCT
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-accuratedct\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::stdlib::printf(
                    b"Using most accurate DCT/IDCT algorithm\n\x00" as *const u8
                        as *const libc::c_char,
                );
                flags |= crate::src::turbojpeg::TJFLAG_ACCURATEDCT
            } else {
                usage(*argv.offset(0));
            }
            i += 1
        }
        /* Determine input and output image formats based on file extensions. */
        inFormat = crate::stdlib::strrchr(*argv.offset(1), '.' as i32);
        outFormat = crate::stdlib::strrchr(*argv.offset(2), '.' as i32);
        if inFormat.is_null()
            || outFormat.is_null()
            || crate::stdlib::strlen(inFormat) < 2i32 as libc::c_ulong
            || crate::stdlib::strlen(outFormat) < 2i32 as libc::c_ulong
        {
            usage(*argv.offset(0));
        }
        inFormat = &mut *inFormat.offset(1) as *mut libc::c_char;
        outFormat = &mut *outFormat.offset(1) as *mut libc::c_char;
        if crate::stdlib::strcasecmp(inFormat, b"jpg\x00" as *const u8 as *const libc::c_char) == 0
        {
            /* Input image is a JPEG image.  Decompress and/or transform it. */
            let mut size: libc::c_long = 0;
            let mut inSubsamp: libc::c_int = 0;
            let mut inColorspace: libc::c_int = 0;
            let mut doTransform: libc::c_int =
                (xform.op != crate::src::turbojpeg::TJXOP_NONE as libc::c_int
                    || xform.options != 0i32
                    || xform.customFilter.is_some()) as libc::c_int;
            let mut jpegSize: libc::c_ulong = 0;
            /* Read the JPEG file into memory. */
            jpegFile = crate::stdlib::fopen(
                *argv.offset(1),
                b"rb\x00" as *const u8 as *const libc::c_char,
            );
            if jpegFile.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    269i32,
                    b"opening input file\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else if crate::stdlib::fseek(jpegFile, 0i32 as libc::c_long, crate::stdlib::SEEK_END)
                < 0i32
                || {
                    size = crate::stdlib::ftell(jpegFile);
                    (size) < 0i32 as libc::c_long
                }
                || crate::stdlib::fseek(jpegFile, 0i32 as libc::c_long, crate::stdlib::SEEK_SET)
                    < 0i32
            {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    272i32,
                    b"determining input file size\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else if size == 0i32 as libc::c_long {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    274i32,
                    b"determining input file size\x00" as *const u8 as *const libc::c_char,
                    b"Input file contains no data\x00" as *const u8 as *const libc::c_char,
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else {
                jpegSize = size as libc::c_ulong;
                jpegBuf = crate::src::turbojpeg::tjAlloc(jpegSize as libc::c_int);
                if jpegBuf.is_null() {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        277i32,
                        b"allocating JPEG buffer\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -1i32;
                    current_block = 16288987300638808654;
                } else if crate::stdlib::fread(
                    jpegBuf as *mut libc::c_void,
                    jpegSize,
                    1i32 as libc::c_ulong,
                    jpegFile,
                ) < 1i32 as libc::c_ulong
                {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        279i32,
                        b"reading input file\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -1i32;
                    current_block = 16288987300638808654;
                } else {
                    crate::stdlib::fclose(jpegFile);
                    jpegFile = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
                    if doTransform != 0 {
                        /* Transform it. */
                        let mut dstBuf: *mut libc::c_uchar =
                            crate::stddef_h::NULL_0 as *mut libc::c_uchar; /* Dynamically allocate the JPEG buffer */
                        let mut dstSize: libc::c_ulong = 0i32 as libc::c_ulong;
                        tjInstance = crate::src::turbojpeg::tjInitTransform();
                        if tjInstance.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                288i32,
                                b"initializing transformer\x00" as *const u8 as *const libc::c_char,
                                crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                            );
                            retval = -1i32;
                            current_block = 16288987300638808654;
                        } else {
                            xform.options |= crate::src::turbojpeg::TJXOPT_TRIM;
                            if crate::src::turbojpeg::tjTransform(
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
                                crate::stdlib::printf(
                                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    292i32,
                                    b"transforming input image\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                                );
                                retval = -1i32;
                                current_block = 16288987300638808654;
                            } else {
                                crate::src::turbojpeg::tjFree(jpegBuf);
                                jpegBuf = dstBuf;
                                jpegSize = dstSize;
                                current_block = 7244994750255146185;
                            }
                        }
                    } else {
                        tjInstance = crate::src::turbojpeg::tjInitDecompress();
                        if tjInstance.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                298i32,
                                b"initializing decompressor\x00" as *const u8
                                    as *const libc::c_char,
                                crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
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
                            if crate::src::turbojpeg::tjDecompressHeader3(
                                tjInstance,
                                jpegBuf,
                                jpegSize,
                                &mut width,
                                &mut height,
                                &mut inSubsamp,
                                &mut inColorspace,
                            ) < 0i32
                            {
                                crate::stdlib::printf(
                                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    303i32,
                                    b"reading JPEG header\x00" as *const u8 as *const libc::c_char,
                                    crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                                );
                                retval = -1i32;
                                current_block = 16288987300638808654;
                            } else {
                                crate::stdlib::printf(b"%s Image:  %d x %d pixels, %s subsampling, %s colorspace\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if doTransform != 0 {
                                           b"Transformed\x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"Input\x00" as *const u8 as
                                               *const libc::c_char
                                       }, width, height,
                                       subsampName[inSubsamp as usize],
                                       colorspaceName[inColorspace as usize]);
                                if crate::stdlib::strcasecmp(
                                    outFormat,
                                    b"jpg\x00" as *const u8 as *const libc::c_char,
                                ) == 0
                                    && doTransform != 0
                                    && scalingFactor.num == 1i32
                                    && scalingFactor.denom == 1i32
                                    && outSubsamp < 0i32
                                    && outQual < 0i32
                                {
                                    /* Input image has been transformed, and no re-compression options
                                    have been selected.  Write the transformed image to disk and exit. */
                                    jpegFile = crate::stdlib::fopen(
                                        *argv.offset(2),
                                        b"wb\x00" as *const u8 as *const libc::c_char,
                                    );
                                    if jpegFile.is_null() {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            315i32,
                                            b"opening output file\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -1i32
                                    } else if crate::stdlib::fwrite(
                                        jpegBuf as *const libc::c_void,
                                        jpegSize,
                                        1i32 as libc::c_ulong,
                                        jpegFile,
                                    ) < 1i32 as libc::c_ulong
                                    {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            317i32,
                                            b"writing output file\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -1i32
                                    } else {
                                        crate::stdlib::fclose(jpegFile);
                                        jpegFile =
                                            crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE
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
                                    pixelFormat = crate::src::turbojpeg::TJPF_BGRX as libc::c_int;
                                    imgBuf = crate::src::turbojpeg::tjAlloc(
                                        width
                                            * height
                                            * crate::src::turbojpeg::tjPixelSize
                                                [pixelFormat as usize],
                                    );
                                    if imgBuf.is_null() {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            333i32,
                                            b"allocating uncompressed image buffer\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -1i32;
                                        current_block = 16288987300638808654;
                                    } else if crate::src::turbojpeg::tjDecompress2(
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
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            337i32,
                                            b"decompressing JPEG image\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                                        );
                                        retval = -1i32;
                                        current_block = 16288987300638808654;
                                    } else {
                                        crate::src::turbojpeg::tjFree(jpegBuf);
                                        jpegBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                                        crate::src::turbojpeg::tjDestroy(tjInstance);
                                        tjInstance = crate::stddef_h::NULL_0 as *mut libc::c_void;
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
            imgBuf = crate::src::turbojpeg::tjLoadImage(
                *argv.offset(1),
                &mut width,
                1i32,
                &mut height,
                &mut pixelFormat,
                0i32,
            );
            if imgBuf.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    344i32,
                    b"loading input image\x00" as *const u8 as *const libc::c_char,
                    crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                );
                retval = -1i32;
                current_block = 16288987300638808654;
            } else {
                if outSubsamp < 0i32 {
                    if pixelFormat == crate::src::turbojpeg::TJPF_GRAY as libc::c_int {
                        outSubsamp = crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int
                    } else {
                        outSubsamp = crate::src::turbojpeg::TJSAMP_444 as libc::c_int
                    }
                }
                crate::stdlib::printf(
                    b"Input Image:  %d x %d pixels\n\x00" as *const u8 as *const libc::c_char,
                    width,
                    height,
                );
                current_block = 3098209481605707636;
            }
        }
        match current_block {
            16288987300638808654 => {}
            _ => {
                crate::stdlib::printf(
                    b"Output Image (%s):  %d x %d pixels\x00" as *const u8 as *const libc::c_char,
                    outFormat,
                    width,
                    height,
                );
                if crate::stdlib::strcasecmp(
                    outFormat,
                    b"jpg\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    /* Output image format is JPEG.  Compress the uncompressed image. */
                    let mut jpegBuf_0: *mut libc::c_uchar =
                        crate::stddef_h::NULL_0 as *mut libc::c_uchar; /* Dynamically allocate the JPEG buffer */
                    let mut jpegSize_0: libc::c_ulong = 0i32 as libc::c_ulong;
                    if outQual < 0i32 {
                        outQual = DEFAULT_QUALITY
                    }
                    crate::stdlib::printf(
                        b", %s subsampling, quality = %d\n\x00" as *const u8 as *const libc::c_char,
                        subsampName[outSubsamp as usize],
                        outQual,
                    );
                    tjInstance = crate::src::turbojpeg::tjInitCompress();
                    if tjInstance.is_null() {
                        crate::stdlib::printf(
                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            367i32,
                            b"initializing compressor\x00" as *const u8 as *const libc::c_char,
                            crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                        );
                        retval = -1i32
                    } else if crate::src::turbojpeg::tjCompress2(
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
                        crate::stdlib::printf(
                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            370i32,
                            b"compressing image\x00" as *const u8 as *const libc::c_char,
                            crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                        );
                        retval = -1i32
                    } else {
                        crate::src::turbojpeg::tjDestroy(tjInstance);
                        tjInstance = crate::stddef_h::NULL_0 as *mut libc::c_void;
                        /* Write the JPEG image to disk. */
                        jpegFile = crate::stdlib::fopen(
                            *argv.offset(2),
                            b"wb\x00" as *const u8 as *const libc::c_char,
                        );
                        if jpegFile.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                375i32,
                                b"opening output file\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -1i32
                        } else if crate::stdlib::fwrite(
                            jpegBuf_0 as *const libc::c_void,
                            jpegSize_0,
                            1i32 as libc::c_ulong,
                            jpegFile,
                        ) < 1i32 as libc::c_ulong
                        {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                377i32,
                                b"writing output file\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -1i32
                        } else {
                            crate::src::turbojpeg::tjDestroy(tjInstance);
                            tjInstance = crate::stddef_h::NULL_0 as *mut libc::c_void;
                            crate::stdlib::fclose(jpegFile);
                            jpegFile = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
                            crate::src::turbojpeg::tjFree(jpegBuf_0);
                            jpegBuf_0 = crate::stddef_h::NULL_0 as *mut libc::c_uchar
                        }
                    }
                } else {
                    /* Output image format is not JPEG.  Save the uncompressed image
                    directly to disk. */
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    if crate::src::turbojpeg::tjSaveImage(
                        *argv.offset(2),
                        imgBuf,
                        width,
                        0i32,
                        height,
                        pixelFormat,
                        0i32,
                    ) < 0i32
                    {
                        crate::stdlib::printf(
                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            386i32,
                            b"saving output image\x00" as *const u8 as *const libc::c_char,
                            crate::src::turbojpeg::tjGetErrorStr2(tjInstance),
                        );
                        retval = -1i32
                    }
                }
            }
        }
    }
    if !imgBuf.is_null() {
        crate::src::turbojpeg::tjFree(imgBuf);
    }
    if !tjInstance.is_null() {
        crate::src::turbojpeg::tjDestroy(tjInstance);
    }
    if !jpegBuf.is_null() {
        crate::src::turbojpeg::tjFree(jpegBuf);
    }
    if !jpegFile.is_null() {
        crate::stdlib::fclose(jpegFile);
    }
    return retval;
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
