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

pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    C2RustUnnamed_0, _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _ISalnum, _ISalpha,
    _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct, _ISspace, _ISupper,
    _ISxdigit, __ctype_b_loc, __ctype_tolower_loc, __int32_t, __off64_t, __off_t, exit, fopen,
    fprintf, getc, printf, putc, setlocale, stderr, stdin, stdout, tolower, EOF, EXIT_FAILURE,
    EXIT_SUCCESS, FILE, LC_CTYPE, _IO_FILE, __LC_CTYPE,
};
use libc::{c_char, c_int, c_uint, c_ulong, c_ushort};
use mozjpeg::*;
use std::prelude::v1::*;
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

pub const READ_BINARY: [c_char; 3] =
    unsafe { *::std::mem::transmute::<&[u8; 3], &[c_char; 3]>(b"rb\x00") };
/* define exit() codes if not provided */
/*
 * These macros are used to read the input file.
 * To reuse this code in another application, you might need to change these.
 */

static mut infile: *mut FILE = ::std::ptr::null::<FILE>() as *mut FILE;
/* input JPEG file */
/* Return next input byte, or EOF if no more */
/* Error exit handler */
/* Read one byte, testing for EOF */

unsafe extern "C" fn read_1_byte() -> c_int {
    let mut c: c_int = getc(infile);
    if c == EOF {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Premature EOF in JPEG file\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    return c;
}
/* Read 2 bytes, convert to unsigned int */
/* All 2-byte quantities in JPEG markers are MSB first */

unsafe extern "C" fn read_2_bytes() -> c_uint {
    let mut c1: c_int = getc(infile);
    if c1 == EOF {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Premature EOF in JPEG file\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    let mut c2: c_int = getc(infile);
    if c2 == EOF {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Premature EOF in JPEG file\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    return ((c1 as c_uint) << 8i32) + c2 as c_uint;
}

pub const M_SOI: c_int = 0xd8i32;
/* COMment */
/*
 * Find the next JPEG marker and return its marker code.
 * We expect at least one FF byte, possibly more if the compressor used FFs
 * to pad the file.
 * There could also be non-FF garbage between markers.  The treatment of such
 * garbage is unspecified; we choose to skip over it but emit a warning msg.
 * NB: this routine must not be used after seeing SOS marker, since it will
 * not deal correctly with FF/00 sequences in the compressed image data...
 */

unsafe extern "C" fn next_marker() -> c_int {
    let mut discarded_bytes: c_int = 0i32;
    let mut c: c_int = read_1_byte();
    while c != 0xffi32 {
        discarded_bytes += 1;
        c = read_1_byte()
    }
    loop
    /* Get marker code byte, swallowing any duplicate FF bytes.  Extra FFs
     * are legal as pad bytes, so don't count them in discarded_bytes.
     */
    {
        c = read_1_byte();
        if !(c == 0xffi32) {
            break;
        }
    }
    if discarded_bytes != 0i32 {
        fprintf(
            stderr,
            b"Warning: garbage data found in JPEG file\n\x00".as_ptr() as *const c_char,
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

unsafe extern "C" fn first_marker() -> c_int {
    let mut c1: c_int = getc(infile);
    let mut c2: c_int = getc(infile);
    if c1 != 0xffi32 || c2 != M_SOI {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Not a JPEG file\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
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

unsafe extern "C" fn skip_variable()
/* Skip over an unknown or uninteresting variable-length marker */
{
    let mut length: c_uint = read_2_bytes();
    /* Length includes itself, so must be at least 2 */
    if length < 2u32 {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Erroneous JPEG marker length\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    length -= 2u32;
    /* Skip over the remaining bytes */
    while length > 0u32 {
        read_1_byte();
        length -= 1
    }
}
/*
 * Process a COM marker.
 * We want to print out the marker contents as legible text;
 * we must guard against non-text junk and varying newline representations.
 */

unsafe extern "C" fn process_COM(mut raw: c_int) {
    /* Bill Allombert: set locale properly for isprint */
    setlocale(LC_CTYPE, b"\x00".as_ptr() as *const c_char);
    let mut length: c_uint = read_2_bytes();
    /* Length includes itself, so must be at least 2 */
    if length < 2u32 {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Erroneous JPEG marker length\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    length -= 2u32;
    while length > 0u32 {
        let mut lastch: c_int = 0i32;
        let mut ch: c_int = read_1_byte();
        if raw != 0 {
            putc(ch, stdout);
        /* Emit the character in a readable form.
         * Nonprintables are converted to \nnn form,
         * while \ is converted to \\.
         * Newlines in CR, CR/LF, or LF form will be printed as one newline.
         */
        } else if ch == '\r' as i32 {
            printf(b"\n\x00".as_ptr() as *const c_char);
        } else if ch == '\n' as i32 {
            if lastch != '\r' as i32 {
                printf(b"\n\x00".as_ptr() as *const c_char);
            }
        } else if ch == '\\' as i32 {
            printf(b"\\\\\x00".as_ptr() as *const c_char);
        } else if *(*__ctype_b_loc()).offset(ch as isize) as c_int & _ISprint as c_ushort as c_int
            != 0
        {
            putc(ch, stdout);
        } else {
            printf(b"\\%03o\x00".as_ptr() as *const c_char, ch);
        }
        lastch = ch;
        length -= 1
    }
    printf(b"\n\x00".as_ptr() as *const c_char);
    /* Bill Allombert: revert to C locale */
    setlocale(LC_CTYPE, b"C\x00".as_ptr() as *const c_char);
}
/*
 * Process a SOFn marker.
 * This code is only needed if you want to know the image dimensions...
 */

unsafe extern "C" fn process_SOFn(mut marker: c_int) {
    let mut process: *const c_char = ::std::ptr::null::<c_char>();

    let mut length: c_uint = read_2_bytes();
    let mut data_precision: c_int = read_1_byte();
    let mut image_height: c_uint = read_2_bytes();
    let mut image_width: c_uint = read_2_bytes();
    let mut num_components: c_int = read_1_byte();
    match marker {
        192 => process = b"Baseline\x00".as_ptr() as *const c_char,
        193 => process = b"Extended sequential\x00".as_ptr() as *const c_char,
        194 => process = b"Progressive\x00".as_ptr() as *const c_char,
        195 => process = b"Lossless\x00".as_ptr() as *const c_char,
        197 => process = b"Differential sequential\x00".as_ptr() as *const c_char,
        198 => process = b"Differential progressive\x00".as_ptr() as *const c_char,
        199 => process = b"Differential lossless\x00".as_ptr() as *const c_char,
        201 => process = b"Extended sequential, arithmetic coding\x00".as_ptr() as *const c_char,
        202 => process = b"Progressive, arithmetic coding\x00".as_ptr() as *const c_char,
        203 => process = b"Lossless, arithmetic coding\x00".as_ptr() as *const c_char,
        205 => {
            process = b"Differential sequential, arithmetic coding\x00".as_ptr() as *const c_char
        }
        206 => {
            process = b"Differential progressive, arithmetic coding\x00".as_ptr() as *const c_char
        }
        207 => process = b"Differential lossless, arithmetic coding\x00".as_ptr() as *const c_char,
        _ => process = b"Unknown\x00".as_ptr() as *const c_char,
    }
    printf(
        b"JPEG image is %uw * %uh, %d color components, %d bits per sample\n\x00".as_ptr()
            as *const c_char,
        image_width,
        image_height,
        num_components,
        data_precision,
    );
    printf(b"JPEG process: %s\n\x00".as_ptr() as *const c_char, process);
    if length != (8i32 + num_components * 3i32) as c_uint {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Bogus SOF marker length\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    let mut ci: c_int = 0i32;
    while ci < num_components {
        read_1_byte();
        /* Quantization table number */
        read_1_byte(); /* H, V sampling factors */
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

unsafe extern "C" fn scan_JPEG_header(mut verbose: c_int, mut raw: c_int) -> c_int {
    /* Expect SOI at start of file */
    if first_marker() != M_SOI {
        fprintf(
            stderr,
            b"%s\n\x00".as_ptr() as *const c_char,
            b"Expected SOI marker first\x00".as_ptr() as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    loop
    /* Scan miscellaneous markers until we reach SOS. */
    {
        let mut current_block_14: u64;
        let mut marker: c_int = next_marker();

        match marker {
            192 => {
                /* Baseline */
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
            218 => {
                /* stop before hitting compressed data */
                return marker;
            }
            217 => {
                /* in case it's a tables-only JPEG stream */
                return marker;
            }
            254 => {
                process_COM(raw);
                current_block_14 = 10652014663920648156;
            }
            236 => {
                /* Some digital camera makers put useful textual information into
                 * APP12 markers, so we print those out too when in -verbose mode.
                 */
                if verbose != 0 {
                    printf(b"APP12 contains:\n\x00".as_ptr() as *const c_char); /* we assume it has a parameter count... */
                    process_COM(raw);
                } else {
                    skip_variable();
                }
                current_block_14 = 10652014663920648156;
            }
            _ => {
                /* Anything else just gets skipped */
                skip_variable();
                current_block_14 = 10652014663920648156;
            }
        }
        match current_block_14 {
            11514485499954567250 =>
            /* Extended sequential, Huffman */
            {
                current_block_14 = 18034400475116118263;
            }
            _ => {}
        }
        match current_block_14 {
            18034400475116118263 =>
            /* Progressive, Huffman */
            {
                current_block_14 = 8778240750267069325;
            }
            _ => {}
        }
        match current_block_14 {
            8778240750267069325 =>
            /* Lossless, Huffman */
            {
                current_block_14 = 18313468745464571188;
            }
            _ => {}
        }
        match current_block_14 {
            18313468745464571188 =>
            /* Differential sequential, Huffman */
            {
                current_block_14 = 1854573226567202969;
            }
            _ => {}
        }
        match current_block_14 {
            1854573226567202969 =>
            /* Differential progressive, Huffman */
            {
                current_block_14 = 7651479451671281396;
            }
            _ => {}
        }
        match current_block_14 {
            7651479451671281396 =>
            /* Differential lossless, Huffman */
            {
                current_block_14 = 13107772826531509185;
            }
            _ => {}
        }
        match current_block_14 {
            13107772826531509185 =>
            /* Extended sequential, arithmetic */
            {
                current_block_14 = 13432579983630657721;
            }
            _ => {}
        }
        match current_block_14 {
            13432579983630657721 =>
            /* Progressive, arithmetic */
            {
                current_block_14 = 12810681100288518424;
            }
            _ => {}
        }
        match current_block_14 {
            12810681100288518424 =>
            /* Lossless, arithmetic */
            {
                current_block_14 = 7625007334740916051;
            }
            _ => {}
        }
        match current_block_14 {
            7625007334740916051 =>
            /* Differential sequential, arithmetic */
            {
                current_block_14 = 14683081201007839341;
            }
            _ => {}
        }
        match current_block_14 {
            14683081201007839341 =>
            /* Differential progressive, arithmetic */
            {
                current_block_14 = 5770943196170972893;
            }
            _ => {}
        }
        match current_block_14 {
            5770943196170972893 =>
            /* Note that marker codes 0xC4, 0xC8, 0xCC are not, and must not be,
             * treated as SOFn.  C4 in particular is actually DHT.
             */
            /* Differential lossless, arithmetic */
            {
                if verbose != 0 {
                    process_SOFn(marker);
                } else {
                    skip_variable();
                }
            }
            _ => {}
        }
    }
    /* end loop */
}
/* Command line parsing code */

static mut progname: *const c_char = ::std::ptr::null::<c_char>();
/* program name for error messages */

unsafe extern "C" fn usage()
/* complain about bad command line */
{
    fprintf(
        stderr,
        b"rdjpgcom displays any textual comments in a JPEG file.\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        b"Usage: %s [switches] [inputfile]\n\x00".as_ptr() as *const c_char,
        progname,
    );
    fprintf(
        stderr,
        b"Switches (names may be abbreviated):\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        b"  -raw        Display non-printable characters in comments (unsafe)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        b"  -verbose    Also display dimensions of JPEG image\n\x00".as_ptr() as *const c_char,
    );
    exit(EXIT_FAILURE);
}

unsafe extern "C" fn keymatch(
    mut arg: *mut c_char,
    mut keyword: *const c_char,
    mut minchars: c_int,
) -> c_int
/* Case-insensitive matching of (possibly abbreviated) keyword switches. */
/* keyword is the constant keyword (must be lower case already), */
/* minchars is length of minimum legal abbreviation. */ {
    /* arg longer than keyword, no good */

    let mut nmatched: c_int = 0i32;
    loop {
        let fresh0 = arg;
        arg = arg.offset(1);
        let mut ca: c_int = *fresh0 as c_int;
        if !(ca != '\u{0}' as i32) {
            break;
        }
        let fresh1 = keyword;
        keyword = keyword.offset(1);
        let mut ck: c_int = *fresh1 as c_int;
        if ck == '\u{0}' as i32 {
            return 0i32;
        }
        if *(*__ctype_b_loc()).offset(ca as isize) as c_int & _ISupper as c_ushort as c_int != 0 {
            /* count matched characters */
            /* force arg to lcase (assume ck is already) */
            ca = {
                let mut __res: c_int = 0; /* no good */
                if ::std::mem::size_of::<c_int>() as c_ulong > 1u64 {
                    if 0 != 0 {
                        let mut __c: c_int = ca;
                        __res = if __c < -128i32 || __c > 255i32 {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(ca)
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(ca as isize)
                }
                __res
            }
        }
        if ca != ck {
            return 0i32;
        }
        nmatched += 1
    }
    /* reached end of argument; fail if it's too short for unique abbrev */
    if nmatched < minchars {
        return 0i32;
    }
    return 1i32;
    /* A-OK */
}
/*
 * The main program.
 */

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut verbose: c_int = 0i32;
    let mut raw: c_int = 0i32;
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0); /* in case C library doesn't provide it */
    if progname.is_null() || *progname.offset(0) as c_int == 0i32 {
        progname = b"rdjpgcom\x00".as_ptr() as *const c_char
    }
    let mut argn: c_int = 1i32; /* not switch, must be file name */
    while argn < argc {
        let mut arg: *mut c_char = *argv.offset(argn as isize); /* advance over '-' */
        if *arg.offset(0) as c_int != '-' as i32 {
            break;
        }
        arg = arg.offset(1);
        if keymatch(arg, b"verbose\x00".as_ptr() as *const c_char, 1i32) != 0 {
            verbose += 1
        } else if keymatch(arg, b"raw\x00".as_ptr() as *const c_char, 1i32) != 0 {
            raw = 1i32
        } else {
            usage();
        }
        argn += 1
    }
    /* Open the input file. */
    /* Unix style: expect zero or one file name */
    if argn < argc - 1i32 {
        fprintf(
            stderr,
            b"%s: only one input file\n\x00".as_ptr() as *const c_char,
            progname,
        );
        usage();
    }
    if argn < argc {
        infile = fopen(*argv.offset(argn as isize), READ_BINARY.as_ptr());
        if infile.is_null() {
            fprintf(
                stderr,
                b"%s: can\'t open %s\n\x00".as_ptr() as *const c_char,
                progname,
                *argv.offset(argn as isize),
            );
            exit(EXIT_FAILURE);
        }
    } else {
        /* default input file is stdin */
        /* need to hack file mode? */
        /* need to re-open in binary mode? */
        infile = stdin
    }
    /* Scan the JPEG headers. */
    scan_JPEG_header(verbose, raw);
    /* All done. */
    exit(EXIT_SUCCESS);
    /* suppress no-return-value warnings */
}
#[main]
pub fn main() {
    let mut args: Vec<*mut c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((args.len() - 1) as c_int, args.as_mut_ptr())) }
}
