use ::libc;

#[c2rust::header_src = "/usr/include/ctype.h:16"]
pub mod ctype_h {

    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stddef_h::size_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::src::cdjpeg::ctype_h::tolower;
pub use crate::stdlib::C2RustUnnamed_0;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__ctype_tolower_loc;
use crate::stdlib::stdin;
use crate::stdlib::stdout;
/*
 * cdjpeg.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains common support routines used by the IJG application
 * programs (cjpeg, djpeg, jpegtran).
 */
/* to declare isupper(), tolower() */
/*
 * Optional progress monitor: display a percent-done figure on stderr.
 */
/*
 * Case-insensitive matching of possibly-abbreviated keyword switches.
 * keyword is the constant keyword (must be lower case already),
 * minchars is length of minimum legal abbreviation.
 */
#[no_mangle]

pub unsafe extern "C" fn keymatch(
    mut arg: *mut libc::c_char,
    mut keyword: *const libc::c_char,
    mut minchars: libc::c_int,
) -> crate::jmorecfg_h::boolean {
    let mut ca: libc::c_int = 0; /* arg longer than keyword, no good */
    let mut ck: libc::c_int = 0;
    let mut nmatched: libc::c_int = 0 as libc::c_int;
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
            return crate::jmorecfg_h::FALSE;
        }
        if *(*crate::stdlib::__ctype_b_loc()).offset(ca as isize) as libc::c_int
            & crate::stdlib::_ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            /* count matched characters */
            /* force arg to lcase (assume ck is already) */
            ca = ({
                let mut __res: libc::c_int = 0; /* no good */
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = ca;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(ca)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(ca as isize)
                }
                __res
            })
        }
        if ca != ck {
            return crate::jmorecfg_h::FALSE;
        }
        nmatched += 1
    }
    /* reached end of argument; fail if it's too short for unique abbrev */
    if nmatched < minchars {
        return crate::jmorecfg_h::FALSE;
    }
    return crate::jmorecfg_h::TRUE;
    /* A-OK */
}
/*
 * Routines to establish binary I/O mode for stdin and stdout.
 * Non-Unix systems often require some hacking to get out of text mode.
 */
#[no_mangle]

pub unsafe extern "C" fn read_stdin() -> *mut crate::stdlib::FILE {
    let mut input_file: *mut crate::stdlib::FILE = crate::stdlib::stdin;
    /* need to hack file mode? */
    /* need to re-open in binary mode? */
    return input_file;
}
/*
 * cdjpeg.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2017, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg file.
 *
 * This file contains common declarations for the sample applications
 * cjpeg and djpeg.  It is NOT used by the core JPEG library.
 */
/* define proper options in jconfig.h */
/* cjpeg.c,djpeg.c need to see xxx_SUPPORTED */
/*
 * Object interface for cjpeg's source file decoding modules
 */
/*
 * Object interface for djpeg's output file encoding modules
 */
/* start_output is called after jpeg_start_decompress finishes.
 * The color map will be ready at this time, if one is needed.
 */
/* Emit the specified number of pixel rows from the buffer. */
/* Finish up at the end of the image. */
/* Re-calculate buffer dimensions based on output dimensions (for use with
partial image decompression.)  If this is NULL, then the output format
does not support partial image decompression (BMP and RLE, in particular,
cannot support partial decompression because they use an inversion buffer
to write the image in bottom-up order.) */
/* Target file spec; filled in by djpeg.c after object is created. */
/* Output pixel-row buffer.  Created by module init or start_output.
 * Width is cinfo->output_width * cinfo->output_components;
 * height is buffer_height.
 */
/*
 * cjpeg/djpeg may need to perform extra passes to convert to or from
 * the source/destination file format.  The JPEG library does not know
 * about these passes, but we'd like them to be counted by the progress
 * monitor.  We use an expanded progress monitor object to hold the
 * additional pass count.
 */
/* fields known to JPEG library */
/* extra passes completed */
/* total extra */
/* last printed percentage stored here to avoid multiple printouts */
/* Module selection routines for I/O modules. */
/* cjpeg support routines (in rdswitch.c) */
/* djpeg support routines (in rdcolmap.c) */
/* common support routines (in cdjpeg.c) */
#[no_mangle]

pub unsafe extern "C" fn write_stdout() -> *mut crate::stdlib::FILE {
    let mut output_file: *mut crate::stdlib::FILE = crate::stdlib::stdout;
    /* need to hack file mode? */
    /* need to re-open in binary mode? */
    return output_file;
}
