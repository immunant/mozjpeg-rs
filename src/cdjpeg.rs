use libc::c_char;
use libc::c_int;
use libc::c_ulong;
use libc::c_ushort;
extern "C" {
    #[no_mangle]
    pub fn read_scan_script(cinfo: j_compress_ptr, filename: *mut c_char) -> boolean;

    #[no_mangle]
    pub fn jinit_write_gif(cinfo: j_decompress_ptr) -> djpeg_dest_ptr;

    #[no_mangle]
    pub fn jinit_write_targa(cinfo: j_decompress_ptr) -> djpeg_dest_ptr;
    /* djpeg support routines (in rdcolmap.c) */
    #[no_mangle]
    pub fn read_color_map(cinfo: j_decompress_ptr, infile: *mut FILE);
    /* Module selection routines for I/O modules. */
    #[no_mangle]
    pub fn jinit_read_jpeg(cinfo: j_compress_ptr) -> cjpeg_source_ptr;

    #[no_mangle]
    pub fn jinit_read_gif(cinfo: j_compress_ptr) -> cjpeg_source_ptr;

    #[no_mangle]
    pub fn jinit_read_targa(cinfo: j_compress_ptr) -> cjpeg_source_ptr;
    /* cjpeg support routines (in rdswitch.c) */
    #[no_mangle]
    pub fn read_quant_tables(
        cinfo: j_compress_ptr,
        filename: *mut c_char,
        force_baseline: boolean,
    ) -> boolean;

    #[no_mangle]
    pub fn set_quality_ratings(
        cinfo: j_compress_ptr,
        arg: *mut c_char,
        force_baseline: boolean,
    ) -> boolean;

    #[no_mangle]
    pub fn set_quant_slots(cinfo: j_compress_ptr, arg: *mut c_char) -> boolean;

    #[no_mangle]
    pub fn set_sample_factors(cinfo: j_compress_ptr, arg: *mut c_char) -> boolean;

    #[no_mangle]
    pub fn jinit_read_bmp(cinfo: j_compress_ptr, use_inversion_array: boolean) -> cjpeg_source_ptr;

    #[no_mangle]
    pub fn jinit_read_ppm(cinfo: j_compress_ptr) -> cjpeg_source_ptr;

    #[no_mangle]
    pub fn jinit_write_bmp(
        cinfo: j_decompress_ptr,
        is_os2: boolean,
        use_inversion_array: boolean,
    ) -> djpeg_dest_ptr;

    #[no_mangle]
    pub fn jinit_write_ppm(cinfo: j_decompress_ptr) -> djpeg_dest_ptr;
}
pub use crate::stdlib::C2RustUnnamed_0;
// =============== BEGIN cdjpeg_h ================
use crate::jmorecfg_h::JDIMENSION;
use crate::jpeglib_h::j_compress_ptr;
use crate::jpeglib_h::j_decompress_ptr;

use crate::jpeglib_h::jpeg_progress_mgr;
use crate::jpeglib_h::jpeg_saved_marker_ptr;
use crate::jpeglib_h::JSAMPARRAY;
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
pub type cjpeg_source_ptr = *mut cjpeg_source_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cjpeg_source_struct {
    pub start_input: Option<unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> ()>,
    pub get_pixel_rows:
        Option<unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION>,
    pub finish_input: Option<unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> ()>,
    pub input_file: *mut FILE,
    pub buffer: JSAMPARRAY,
    pub buffer_height: JDIMENSION,
    pub marker_list: jpeg_saved_marker_ptr,
}
/*
 * Object interface for djpeg's output file encoding modules
 */
pub type djpeg_dest_ptr = *mut djpeg_dest_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct djpeg_dest_struct {
    pub start_output: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> ()>,
    pub put_pixel_rows:
        Option<unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr, _: JDIMENSION) -> ()>,
    pub finish_output: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> ()>,
    pub calc_buffer_dimensions:
        Option<unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> ()>,
    pub output_file: *mut FILE,
    pub buffer: JSAMPARRAY,
    pub buffer_height: JDIMENSION,
}
// ================ END cdjpeg_h ================
pub use crate::jmorecfg_h::boolean;
pub use crate::stdlib::FILE;
// =============== BEGIN cdjpeg_h ================

/*
 * cjpeg/djpeg may need to perform extra passes to convert to or from
 * the source/destination file format.  The JPEG library does not know
 * about these passes, but we'd like them to be counted by the progress
 * monitor.  We use an expanded progress monitor object to hold the
 * additional pass count.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdjpeg_progress_mgr {
    pub pub_0: jpeg_progress_mgr,
    pub completed_extra_passes: c_int,
    pub total_extra_passes: c_int,
    pub percent_done: c_int,
}
pub type cd_progress_ptr = *mut cdjpeg_progress_mgr;
// ================ END cdjpeg_h ================
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::_IO_FILE;
use libc;
// =============== BEGIN cdjpeg_h ================

/* miscellaneous useful macros */

/* define mode parameters for fopen() */
pub const READ_BINARY: [c_char; 3] =
    unsafe { *::std::mem::transmute::<&[u8; 3], &[c_char; 3]>(b"rb\x00") };
pub const WRITE_BINARY: [c_char; 3] =
    unsafe { *::std::mem::transmute::<&[u8; 3], &[c_char; 3]>(b"wb\x00") };
/* define exit() codes if not provided */
pub const EXIT_WARNING: c_int = 2i32;
// ================ END cdjpeg_h ================
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::TRUE;
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
pub use crate::stdlib::tolower;
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
    mut arg: *mut c_char,
    mut keyword: *const c_char,
    mut minchars: c_int,
) -> boolean {
    let mut ca: c_int = 0;
    let mut ck: c_int = 0;
    let mut nmatched: c_int = 0i32;
    loop {
        let fresh0 = arg;
        arg = arg.offset(1);
        ca = *fresh0 as c_int;
        if !(ca != '\u{0}' as i32) {
            break;
        }
        let fresh1 = keyword;
        keyword = keyword.offset(1);
        ck = *fresh1 as c_int;
        if ck == '\u{0}' as i32 {
            return FALSE;
        }
        if 0 != *(*__ctype_b_loc()).offset(ca as isize) as c_int
            & _ISupper as c_int as c_ushort as c_int
        {
            ca = {
                let mut __res: c_int = 0;
                if ::std::mem::size_of::<c_int>() as c_ulong > 1i32 as c_ulong {
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
            return FALSE;
        }
        nmatched += 1
    }
    if nmatched < minchars {
        return FALSE;
    }
    return TRUE;
}
/*
 * Routines to establish binary I/O mode for stdin and stdout.
 * Non-Unix systems often require some hacking to get out of text mode.
 */
#[no_mangle]
pub unsafe extern "C" fn read_stdin() -> *mut FILE {
    let mut input_file: *mut FILE = stdin;
    return input_file;
}
#[no_mangle]
pub unsafe extern "C" fn write_stdout() -> *mut FILE {
    let mut output_file: *mut FILE = stdout;
    return output_file;
}
