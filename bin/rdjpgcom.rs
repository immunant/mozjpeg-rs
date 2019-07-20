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
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    C2RustUnnamed_0, _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _ISalnum, _ISalpha,
    _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct, _ISspace, _ISupper,
    _ISxdigit, __ctype_b_loc, __ctype_tolower_loc, __int32_t, __off64_t, __off_t, exit, fopen,
    fprintf, getc, printf, putc, setlocale, stderr, stdin, stdout, tolower, EOF, EXIT_FAILURE,
    EXIT_SUCCESS, FILE, LC_CTYPE, _IO_FILE, __LC_CTYPE,
};
extern crate libc;
use mozjpeg::*;

/*
 * rdjpgcom.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * Modified 2009 by Bill Allombert, Guido Vollbeding.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a very simple stand-alone application that displays
 * the text in COM (comment) markers in a JFIF file.
 * This may be useful as an example of the minimum logic needed to parse
 * JPEG markers.
 */
/* to get the command-line config symbols */
/* Bill Allombert: use locale for isprint */
/* to declare isupper(), tolower() */
/* command-line reader for Macintosh */
/* define mode parameters for fopen() */
pub const READ_BINARY: [libc::c_char; 3] =
    unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"rb\x00") };
/* define exit() codes if not provided */
/*
 * These macros are used to read the input file.
 * To reuse this code in another application, you might need to change these.
 */
/* input JPEG file */
static mut infile: *mut crate::stdlib::FILE =
    0 as *const crate::stdlib::FILE as *mut crate::stdlib::FILE;
/* Return next input byte, or EOF if no more */
/* Error exit handler */
/* Read one byte, testing for EOF */
unsafe extern "C" fn read_1_byte() -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = crate::stdlib::getc(infile);
    if c == crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Premature EOF in JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    return c;
}
/* Read 2 bytes, convert to unsigned int */
/* All 2-byte quantities in JPEG markers are MSB first */
unsafe extern "C" fn read_2_bytes() -> libc::c_uint {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    c1 = crate::stdlib::getc(infile);
    if c1 == crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Premature EOF in JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    c2 = crate::stdlib::getc(infile);
    if c2 == crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Premature EOF in JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    return ((c1 as libc::c_uint) << 8i32).wrapping_add(c2 as libc::c_uint);
}
/* Start Of Image (beginning of datastream) */
pub const M_SOI: libc::c_int = 0xd8i32;
/*
 * Find the next JPEG marker and return its marker code.
 * We expect at least one FF byte, possibly more if the compressor used FFs
 * to pad the file.
 * There could also be non-FF garbage between markers.  The treatment of such
 * garbage is unspecified; we choose to skip over it but emit a warning msg.
 * NB: this routine must not be used after seeing SOS marker, since it will
 * not deal correctly with FF/00 sequences in the compressed image data...
 */
unsafe extern "C" fn next_marker() -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut discarded_bytes: libc::c_int = 0i32;
    c = read_1_byte();
    while c != 0xffi32 {
        discarded_bytes += 1;
        c = read_1_byte()
    }
    loop {
        c = read_1_byte();
        if !(c == 0xffi32) {
            break;
        }
    }
    if discarded_bytes != 0i32 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Warning: garbage data found in JPEG file\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    return c;
}
/*
 * Read the initial marker, which should be SOI.
 * For a JFIF file, the first two bytes of the file should be literally
 * 0xFF M_SOI.  To be more general, we could use next_marker, but if the
 * input file weren't actually JPEG at all, next_marker might read the whole
 * file and then return a misleading error message...
 */
unsafe extern "C" fn first_marker() -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    c1 = crate::stdlib::getc(infile);
    c2 = crate::stdlib::getc(infile);
    if c1 != 0xffi32 || c2 != M_SOI {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Not a JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    return c2;
}
/*
 * Most types of marker are followed by a variable-length parameter segment.
 * This routine skips over the parameters for any marker we don't otherwise
 * want to process.
 * Note that we MUST skip the parameter segment explicitly in order not to
 * be fooled by 0xFF bytes that might appear within the parameter segment;
 * such bytes do NOT introduce new markers.
 */
unsafe extern "C" fn skip_variable() {
    let mut length: libc::c_uint = 0;
    length = read_2_bytes();
    if length < 2i32 as libc::c_uint {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Erroneous JPEG marker length\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    length = length.wrapping_sub(2i32 as libc::c_uint);
    while length > 0i32 as libc::c_uint {
        read_1_byte();
        length = length.wrapping_sub(1)
    }
}
/*
 * Process a COM marker.
 * We want to print out the marker contents as legible text;
 * we must guard against non-text junk and varying newline representations.
 */
unsafe extern "C" fn process_COM(mut raw: libc::c_int) {
    let mut length: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut lastch: libc::c_int = 0i32;
    crate::stdlib::setlocale(
        crate::stdlib::LC_CTYPE,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    length = read_2_bytes();
    if length < 2i32 as libc::c_uint {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Erroneous JPEG marker length\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    length = length.wrapping_sub(2i32 as libc::c_uint);
    while length > 0i32 as libc::c_uint {
        ch = read_1_byte();
        if 0 != raw {
            crate::stdlib::putc(ch, crate::stdlib::stdout);
        } else if ch == '\r' as i32 {
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
        } else if ch == '\n' as i32 {
            if lastch != '\r' as i32 {
                crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
        } else if ch == '\\' as i32 {
            crate::stdlib::printf(b"\\\\\x00" as *const u8 as *const libc::c_char);
        } else if 0
            != *(*crate::stdlib::__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & crate::stdlib::_ISprint as libc::c_int as libc::c_ushort as libc::c_int
        {
            crate::stdlib::putc(ch, crate::stdlib::stdout);
        } else {
            crate::stdlib::printf(b"\\%03o\x00" as *const u8 as *const libc::c_char, ch);
        }
        lastch = ch;
        length = length.wrapping_sub(1)
    }
    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::setlocale(
        crate::stdlib::LC_CTYPE,
        b"C\x00" as *const u8 as *const libc::c_char,
    );
}
/*
 * Process a SOFn marker.
 * This code is only needed if you want to know the image dimensions...
 */
unsafe extern "C" fn process_SOFn(mut marker: libc::c_int) {
    let mut length: libc::c_uint = 0;
    let mut image_height: libc::c_uint = 0;
    let mut image_width: libc::c_uint = 0;
    let mut data_precision: libc::c_int = 0;
    let mut num_components: libc::c_int = 0;
    let mut process: *const libc::c_char = 0 as *const libc::c_char;
    let mut ci: libc::c_int = 0;
    length = read_2_bytes();
    data_precision = read_1_byte();
    image_height = read_2_bytes();
    image_width = read_2_bytes();
    num_components = read_1_byte();
    match marker {
        192 => process = b"Baseline\x00" as *const u8 as *const libc::c_char,
        193 => process = b"Extended sequential\x00" as *const u8 as *const libc::c_char,
        194 => process = b"Progressive\x00" as *const u8 as *const libc::c_char,
        195 => process = b"Lossless\x00" as *const u8 as *const libc::c_char,
        197 => process = b"Differential sequential\x00" as *const u8 as *const libc::c_char,
        198 => process = b"Differential progressive\x00" as *const u8 as *const libc::c_char,
        199 => process = b"Differential lossless\x00" as *const u8 as *const libc::c_char,
        201 => {
            process =
                b"Extended sequential, arithmetic coding\x00" as *const u8 as *const libc::c_char
        }
        202 => process = b"Progressive, arithmetic coding\x00" as *const u8 as *const libc::c_char,
        203 => process = b"Lossless, arithmetic coding\x00" as *const u8 as *const libc::c_char,
        205 => {
            process = b"Differential sequential, arithmetic coding\x00" as *const u8
                as *const libc::c_char
        }
        206 => {
            process = b"Differential progressive, arithmetic coding\x00" as *const u8
                as *const libc::c_char
        }
        207 => {
            process =
                b"Differential lossless, arithmetic coding\x00" as *const u8 as *const libc::c_char
        }
        _ => process = b"Unknown\x00" as *const u8 as *const libc::c_char,
    }
    crate::stdlib::printf(
        b"JPEG image is %uw * %uh, %d color components, %d bits per sample\n\x00" as *const u8
            as *const libc::c_char,
        image_width,
        image_height,
        num_components,
        data_precision,
    );
    crate::stdlib::printf(
        b"JPEG process: %s\n\x00" as *const u8 as *const libc::c_char,
        process,
    );
    if length != (8i32 + num_components * 3i32) as libc::c_uint {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Bogus SOF marker length\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    ci = 0i32;
    while ci < num_components {
        read_1_byte();
        read_1_byte();
        read_1_byte();
        ci += 1
    }
}
/*
 * Parse the marker stream until SOS or EOI is seen;
 * display any COM markers.
 * While the companion program wrjpgcom will always insert COM markers before
 * SOFn, other implementations might not, so we scan to SOS before stopping.
 * If we were only interested in the image dimensions, we would stop at SOFn.
 * (Conversely, if we only cared about COM markers, there would be no need
 * for special code to handle SOFn; we could treat it like other markers.)
 */
unsafe extern "C" fn scan_JPEG_header(
    mut verbose: libc::c_int,
    mut raw: libc::c_int,
) -> libc::c_int {
    let mut marker: libc::c_int = 0;
    if first_marker() != M_SOI {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Expected SOI marker first\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    loop {
        marker = next_marker();
        let mut current_block_14: u64;
        match marker {
            192 => {
                /* Extended sequential, Huffman */
                current_block_14 = 11514485499954567250;
            }
            193 => {
                current_block_14 = 11514485499954567250;
            }
            194 => {
                current_block_14 = 18034400475116118263;
            }
            195 => {
                current_block_14 = 8778240750267069325;
            }
            197 => {
                current_block_14 = 18313468745464571188;
            }
            198 => {
                current_block_14 = 1854573226567202969;
            }
            199 => {
                current_block_14 = 7651479451671281396;
            }
            201 => {
                current_block_14 = 13107772826531509185;
            }
            202 => {
                current_block_14 = 13432579983630657721;
            }
            203 => {
                current_block_14 = 12810681100288518424;
            }
            205 => {
                current_block_14 = 7625007334740916051;
            }
            206 => {
                current_block_14 = 14683081201007839341;
            }
            207 => {
                current_block_14 = 5770943196170972893;
            }
            218 => return marker,
            217 => return marker,
            254 => {
                process_COM(raw);
                current_block_14 = 10652014663920648156;
            }
            236 => {
                if 0 != verbose {
                    crate::stdlib::printf(
                        b"APP12 contains:\n\x00" as *const u8 as *const libc::c_char,
                    );
                    process_COM(raw);
                } else {
                    skip_variable();
                }
                current_block_14 = 10652014663920648156;
            }
            _ => {
                skip_variable();
                current_block_14 = 10652014663920648156;
            }
        }
        match current_block_14 {
            11514485499954567250 => {
                /* Progressive, Huffman */
                current_block_14 = 18034400475116118263;
            }
            _ => {}
        }
        match current_block_14 {
            18034400475116118263 => {
                /* Lossless, Huffman */
                current_block_14 = 8778240750267069325;
            }
            _ => {}
        }
        match current_block_14 {
            8778240750267069325 => {
                /* Differential sequential, Huffman */
                current_block_14 = 18313468745464571188;
            }
            _ => {}
        }
        match current_block_14 {
            18313468745464571188 => {
                /* Differential progressive, Huffman */
                current_block_14 = 1854573226567202969;
            }
            _ => {}
        }
        match current_block_14 {
            1854573226567202969 => {
                /* Differential lossless, Huffman */
                current_block_14 = 7651479451671281396;
            }
            _ => {}
        }
        match current_block_14 {
            7651479451671281396 => {
                /* Extended sequential, arithmetic */
                current_block_14 = 13107772826531509185;
            }
            _ => {}
        }
        match current_block_14 {
            13107772826531509185 => {
                /* Progressive, arithmetic */
                current_block_14 = 13432579983630657721;
            }
            _ => {}
        }
        match current_block_14 {
            13432579983630657721 => {
                /* Lossless, arithmetic */
                current_block_14 = 12810681100288518424;
            }
            _ => {}
        }
        match current_block_14 {
            12810681100288518424 => {
                /* Differential sequential, arithmetic */
                current_block_14 = 7625007334740916051;
            }
            _ => {}
        }
        match current_block_14 {
            7625007334740916051 => {
                /* Differential progressive, arithmetic */
                current_block_14 = 14683081201007839341;
            }
            _ => {}
        }
        match current_block_14 {
            14683081201007839341 => {
                /* Differential lossless, arithmetic */
                current_block_14 = 5770943196170972893;
            }
            _ => {}
        }
        match current_block_14 {
            5770943196170972893 => {
                if 0 != verbose {
                    process_SOFn(marker);
                } else {
                    skip_variable();
                }
            }
            _ => {}
        }
    }
}
/* end loop */
/* Command line parsing code */
/* program name for error messages */
static mut progname: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn usage() {
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"rdjpgcom displays any textual comments in a JPEG file.\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Usage: %s [switches] [inputfile]\n\x00" as *const u8 as *const libc::c_char,
        progname,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches (names may be abbreviated):\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -raw        Display non-printable characters in comments (unsafe)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -verbose    Also display dimensions of JPEG image\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
}
unsafe extern "C" fn keymatch(
    mut arg: *mut libc::c_char,
    mut keyword: *const libc::c_char,
    mut minchars: libc::c_int,
) -> libc::c_int {
    let mut ca: libc::c_int = 0;
    let mut ck: libc::c_int = 0;
    let mut nmatched: libc::c_int = 0i32;
    loop {
        let fresh0 = arg;
        arg = arg.offset(1);
        ca = *fresh0 as libc::c_int;
        if !(ca != '\u{0}' as i32) {
            break;
        }
        let fresh1 = keyword;
        keyword = keyword.offset(1);
        ck = *fresh1 as libc::c_int;
        if ck == '\u{0}' as i32 {
            return 0i32;
        }
        if 0 != *(*crate::stdlib::__ctype_b_loc()).offset(ca as isize) as libc::c_int
            & crate::stdlib::_ISupper as libc::c_int as libc::c_ushort as libc::c_int
        {
            ca = {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong > 1i32 as libc::c_ulong {
                    if 0 != 0 {
                        let mut __c: libc::c_int = ca;
                        __res = if __c < -128i32 || __c > 255i32 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = crate::stdlib::tolower(ca)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(ca as isize)
                }
                __res
            }
        }
        if ca != ck {
            return 0i32;
        }
        nmatched += 1
    }
    if nmatched < minchars {
        return 0i32;
    }
    return 1i32;
}
/*
 * The main program.
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut argn: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verbose: libc::c_int = 0i32;
    let mut raw: libc::c_int = 0i32;
    progname = *argv.offset(0isize);
    if progname.is_null() || *progname.offset(0isize) as libc::c_int == 0i32 {
        progname = b"rdjpgcom\x00" as *const u8 as *const libc::c_char
    }
    argn = 1i32;
    while argn < argc {
        arg = *argv.offset(argn as isize);
        if *arg.offset(0isize) as libc::c_int != '-' as i32 {
            /* not switch, must be file name */
            break;
        } else {
            arg = arg.offset(1isize);
            if 0 != keymatch(
                arg,
                b"verbose\x00" as *const u8 as *const libc::c_char,
                1i32,
            ) {
                verbose += 1
            } else if 0 != keymatch(arg, b"raw\x00" as *const u8 as *const libc::c_char, 1i32) {
                raw = 1i32
            } else {
                usage();
            }
            argn += 1
        }
    }
    if argn < argc - 1i32 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: only one input file\n\x00" as *const u8 as *const libc::c_char,
            progname,
        );
        usage();
    }
    if argn < argc {
        infile = crate::stdlib::fopen(*argv.offset(argn as isize), READ_BINARY.as_ptr());
        if infile.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                *argv.offset(argn as isize),
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
    } else {
        infile = crate::stdlib::stdin
    }
    scan_JPEG_header(verbose, raw);
    crate::stdlib::exit(crate::stdlib::EXIT_SUCCESS);
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
