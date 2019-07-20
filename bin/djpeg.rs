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
pub use crate::cderror_h::{
    C2RustUnnamed_91, JERR_BAD_CMAP_FILE, JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH, JERR_BMP_BADHEADER,
    JERR_BMP_BADPLANES, JERR_BMP_COLORSPACE, JERR_BMP_COMPRESSED, JERR_BMP_EMPTY, JERR_BMP_NOT,
    JERR_BMP_OUTOFRANGE, JERR_GIF_BUG, JERR_GIF_CODESIZE, JERR_GIF_COLORSPACE,
    JERR_GIF_IMAGENOTFOUND, JERR_GIF_NOT, JERR_PPM_COLORSPACE, JERR_PPM_NONNUMERIC, JERR_PPM_NOT,
    JERR_PPM_OUTOFRANGE, JERR_TGA_BADCMAP, JERR_TGA_BADPARMS, JERR_TGA_COLORSPACE,
    JERR_TOO_MANY_COLORS, JERR_UNGETC_FAILED, JERR_UNKNOWN_FORMAT, JERR_UNSUPPORTED_FORMAT,
    JMSG_FIRSTADDONCODE, JMSG_LASTADDONCODE, JTRC_BMP, JTRC_BMP_MAPPED, JTRC_BMP_OS2,
    JTRC_BMP_OS2_MAPPED, JTRC_GIF, JTRC_GIF_BADVERSION, JTRC_GIF_EXTENSION, JTRC_GIF_NONSQUARE,
    JTRC_PGM, JTRC_PGM_TEXT, JTRC_PPM, JTRC_PPM_TEXT, JTRC_TGA, JTRC_TGA_GRAY, JTRC_TGA_MAPPED,
    JWRN_GIF_BADDATA, JWRN_GIF_CHAR, JWRN_GIF_ENDCODE, JWRN_GIF_NOMOREDATA,
};
pub use crate::cdjpeg::{
    djpeg_dest_ptr, djpeg_dest_struct, jinit_write_bmp, jinit_write_gif, jinit_write_ppm,
    jinit_write_targa, keymatch, read_color_map, read_stdin, write_stdout, EXIT_WARNING,
    READ_BINARY, WRITE_BINARY,
};
pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jconfigint_h::{BUILD, PACKAGE_NAME, VERSION};
pub use crate::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_CreateDecompress, jpeg_color_deconverter,
    jpeg_color_quantizer, jpeg_common_struct, jpeg_component_info, jpeg_crop_scanline,
    jpeg_d_coef_controller, jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master,
    jpeg_decompress_struct, jpeg_destroy_decompress, jpeg_entropy_decoder, jpeg_error_mgr,
    jpeg_finish_decompress, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_mem_src, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_read_header, jpeg_read_icc_profile, jpeg_read_scanlines, jpeg_save_markers,
    jpeg_saved_marker_ptr, jpeg_set_marker_processor, jpeg_skip_scanlines, jpeg_source_mgr,
    jpeg_start_decompress, jpeg_std_error, jpeg_stdio_src, jpeg_upsampler, jvirt_barray_control,
    jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK,
    JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_DEFAULT, JDCT_FASTEST,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL,
    JPEG_APP0, JPEG_COM, JQUANT_TBL, JSAMPARRAY, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
    J_DITHER_MODE,
};
pub use crate::jversion_h::{JCOPYRIGHT, JVERSION};
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, exit, free, realloc,
    C2RustUnnamed_0, _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower,
    _ISprint, _ISpunct, _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, EXIT_FAILURE, EXIT_SUCCESS,
    FILE, _IO_FILE,
};
extern crate libc;
use mozjpeg::*;

/*
 * This list defines the known output image formats
 * (not all of which need be supported by a given version).
 * You can change the default output format by defining DEFAULT_FMT;
 * indeed, you had better do so if you undefine PPM_SUPPORTED.
 */
pub type IMAGE_FORMATS = libc::c_uint;
/* TIFF format */
pub const FMT_TIFF: IMAGE_FORMATS = 6;
/* Targa format */
pub const FMT_TARGA: IMAGE_FORMATS = 5;
/* RLE format */
pub const FMT_RLE: IMAGE_FORMATS = 4;
/* PPM/PGM (PBMPLUS formats) */
pub const FMT_PPM: IMAGE_FORMATS = 3;
/* BMP format (OS/2 flavor) */
pub const FMT_OS2: IMAGE_FORMATS = 2;
/* GIF format */
pub const FMT_GIF: IMAGE_FORMATS = 1;
/* BMP format (Windows flavor) */
pub const FMT_BMP: IMAGE_FORMATS = 0;
/*
 * djpeg.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2013 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010-2011, 2013-2017, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a command-line user interface for the JPEG decompressor.
 * It should work on any system with Unix- or MS-DOS-style command lines.
 *
 * Two different command line styles are permitted, depending on the
 * compile-time switch TWO_FILE_COMMANDLINE:
 *      djpeg [options]  inputfile outputfile
 *      djpeg [options]  [inputfile]
 * In the second style, output is always to standard output, which you'd
 * normally redirect to a file or pipe to some other program.  Input is
 * either from a named file or from standard input (typically redirected).
 * The second style is convenient on Unix but is unhelpful on systems that
 * don't support pipes.  Also, you MUST use the first style if your system
 * doesn't do binary I/O to stdin/stdout.
 * To simplify script writing, the "-outfile" switch is provided.  The syntax
 *      djpeg [options]  -outfile outputfile  inputfile
 * works regardless of which command line style is used.
 */
/* <stdlib.h> should declare free() */
/* to declare isprint() */
/* command-line reader for Macintosh */
/* Create the add-on message string table. */
static mut cdjpeg_message_table: [*const libc::c_char; 47] = [
    0 as *const libc::c_char,
    b"Unsupported BMP colormap format\x00" as *const u8 as *const libc::c_char,
    b"Only 8- and 24-bit BMP files are supported\x00" as *const u8 as *const libc::c_char,
    b"Invalid BMP file: bad header length\x00" as *const u8 as *const libc::c_char,
    b"Invalid BMP file: biPlanes not equal to 1\x00" as *const u8 as *const libc::c_char,
    b"BMP output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Sorry, compressed BMPs not yet supported\x00" as *const u8 as *const libc::c_char,
    b"Empty BMP image\x00" as *const u8 as *const libc::c_char,
    b"Not a BMP file - does not start with BM\x00" as *const u8 as *const libc::c_char,
    b"Numeric value out of range in BMP file\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 24-bit BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 8-bit colormapped BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 24-bit OS2 BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 8-bit colormapped OS2 BMP image\x00" as *const u8 as *const libc::c_char,
    b"GIF output got confused\x00" as *const u8 as *const libc::c_char,
    b"Bogus GIF codesize %d\x00" as *const u8 as *const libc::c_char,
    b"GIF output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Too few images in GIF file\x00" as *const u8 as *const libc::c_char,
    b"Not a GIF file\x00" as *const u8 as *const libc::c_char,
    b"%ux%ux%d GIF image\x00" as *const u8 as *const libc::c_char,
    b"Warning: unexpected GIF version number \'%c%c%c\'\x00" as *const u8 as *const libc::c_char,
    b"Ignoring GIF extension block of type 0x%02x\x00" as *const u8 as *const libc::c_char,
    b"Caution: nonsquare pixels in input\x00" as *const u8 as *const libc::c_char,
    b"Corrupt data in GIF file\x00" as *const u8 as *const libc::c_char,
    b"Bogus char 0x%02x in GIF file, ignoring\x00" as *const u8 as *const libc::c_char,
    b"Premature end of GIF image\x00" as *const u8 as *const libc::c_char,
    b"Ran out of GIF bits\x00" as *const u8 as *const libc::c_char,
    b"PPM output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Nonnumeric data in PPM file\x00" as *const u8 as *const libc::c_char,
    b"Not a PPM/PGM file\x00" as *const u8 as *const libc::c_char,
    b"Numeric value out of range in PPM file\x00" as *const u8 as *const libc::c_char,
    b"%ux%u PGM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u text PGM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u PPM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u text PPM image\x00" as *const u8 as *const libc::c_char,
    b"Unsupported Targa colormap format\x00" as *const u8 as *const libc::c_char,
    b"Invalid or unsupported Targa file\x00" as *const u8 as *const libc::c_char,
    b"Targa output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"%ux%u RGB Targa image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u grayscale Targa image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u colormapped Targa image\x00" as *const u8 as *const libc::c_char,
    b"Color map file is invalid or of unsupported format\x00" as *const u8 as *const libc::c_char,
    b"Output file format cannot handle %d colormap entries\x00" as *const u8 as *const libc::c_char,
    b"ungetc failed\x00" as *const u8 as *const libc::c_char,
    b"MozJPEG can\'t read the image (PNG support is disabled in this build)\x00" as *const u8
        as *const libc::c_char,
    b"Unsupported output file format\x00" as *const u8 as *const libc::c_char,
    crate::stddef_h::NULL as *const libc::c_char,
];
/* so can override from CFLAGS in Makefile */
pub const DEFAULT_FMT: libc::c_int = FMT_PPM as libc::c_int;
static mut requested_fmt: IMAGE_FORMATS = FMT_BMP;
/*
 * Argument-parsing code.
 * The switch parser is designed to be useful with DOS-style command line
 * syntax, ie, intermixed switches and file names, where only the switches
 * to the left of a given file name affect processing of that file.
 * The main program in this file doesn't actually use this capability...
 */
/* program name for error messages */
static mut progname: *const libc::c_char = 0 as *const libc::c_char;
/* for -icc switch */
static mut icc_filename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* for -outfile switch */
static mut outfilename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* for -memsrc switch */
#[no_mangle]
pub static mut memsrc: crate::jmorecfg_h::boolean = 0;
#[no_mangle]
pub static mut skip: crate::jmorecfg_h::boolean = 0;
#[no_mangle]
pub static mut crop: crate::jmorecfg_h::boolean = 0;
#[no_mangle]
pub static mut skip_start: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]
pub static mut skip_end: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]
pub static mut crop_x: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]
pub static mut crop_y: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]
pub static mut crop_width: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]
pub static mut crop_height: crate::jmorecfg_h::JDIMENSION = 0;
pub const INPUT_BUF_SIZE: libc::c_int = 4096i32;
unsafe extern "C" fn usage() {
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"usage: %s [switches] \x00" as *const u8 as *const libc::c_char,
        progname,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"[inputfile]\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches (names may be abbreviated):\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -colors N      Reduce image to no more than N colors\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -fast          Fast, low-quality processing\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -grayscale     Force grayscale output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -rgb           Force RGB output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -rgb565        Force RGB565 output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -scale M/N     Scale output image by fraction M/N, eg, 1/8\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -bmp           Select BMP output format (Windows style)%s\n\x00" as *const u8
            as *const libc::c_char,
        if DEFAULT_FMT == FMT_BMP as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -gif           Select GIF output format%s\n\x00" as *const u8 as *const libc::c_char,
        if DEFAULT_FMT == FMT_GIF as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -os2           Select BMP output format (OS/2 style)%s\n\x00" as *const u8
            as *const libc::c_char,
        if DEFAULT_FMT == FMT_OS2 as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -pnm           Select PBMPLUS (PPM/PGM) output format%s\n\x00" as *const u8
            as *const libc::c_char,
        if DEFAULT_FMT == FMT_PPM as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -targa         Select Targa output format%s\n\x00" as *const u8 as *const libc::c_char,
        if DEFAULT_FMT == FMT_TARGA as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches for advanced users:\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dct int       Use integer DCT method%s\n\x00" as *const u8 as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_ISLOW as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dct fast      Use fast integer DCT (less accurate)%s\n\x00" as *const u8
            as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_IFAST as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dct float     Use floating-point DCT method%s\n\x00" as *const u8
            as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_FLOAT as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dither fs     Use F-S dithering (default)\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dither none   Don\'t use dithering in quantization\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dither ordered  Use ordered dither (medium speed, quality)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -icc FILE      Extract ICC profile to FILE\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -map FILE      Map to colors used in named image file\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -nosmooth      Don\'t use high-quality upsampling\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -onepass       Use 1-pass quantization (fast, low quality)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -maxmemory N   Maximum memory to use (in kbytes)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -outfile name  Specify name for output file\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -memsrc        Load input file into memory before decompressing\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -skip Y0,Y1    Decompress all rows except those between Y0 and Y1 (inclusive)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -crop WxH+X+Y  Decompress only a rectangular subregion of the image\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 [requires PBMPLUS (PPM/PGM), GIF, or Targa output format]\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -verbose  or  -debug   Emit debug output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -version       Print version information and exit\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
}
unsafe extern "C" fn parse_switches(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut last_file_arg_seen: libc::c_int,
    mut for_real: crate::jmorecfg_h::boolean,
) -> libc::c_int {
    let mut argn: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    requested_fmt = DEFAULT_FMT as IMAGE_FORMATS;
    icc_filename = crate::stddef_h::NULL as *mut libc::c_char;
    outfilename = crate::stddef_h::NULL as *mut libc::c_char;
    memsrc = crate::jmorecfg_h::FALSE;
    skip = crate::jmorecfg_h::FALSE;
    crop = crate::jmorecfg_h::FALSE;
    (*(*cinfo).err).trace_level = 0i32;
    argn = 1i32;
    while argn < argc {
        arg = *argv.offset(argn as isize);
        if *arg as libc::c_int != '-' as i32 {
            /* Not a switch, must be a file name argument */
            if argn <= last_file_arg_seen {
                outfilename = crate::stddef_h::NULL as *mut libc::c_char
            } else {
                /* ignore this name if previously processed */
                /* else done parsing switches */
                break;
            }
        } else {
            arg = arg.offset(1isize);
            if 0 != crate::cdjpeg::keymatch(
                arg,
                b"bmp\x00" as *const u8 as *const libc::c_char,
                1i32,
            ) {
                requested_fmt = FMT_BMP
            } else if 0
                != crate::cdjpeg::keymatch(
                    arg,
                    b"colors\x00" as *const u8 as *const libc::c_char,
                    1i32,
                )
                || 0 != crate::cdjpeg::keymatch(
                    arg,
                    b"colours\x00" as *const u8 as *const libc::c_char,
                    1i32,
                )
                || 0 != crate::cdjpeg::keymatch(
                    arg,
                    b"quantize\x00" as *const u8 as *const libc::c_char,
                    1i32,
                )
                || 0 != crate::cdjpeg::keymatch(
                    arg,
                    b"quantise\x00" as *const u8 as *const libc::c_char,
                    1i32,
                )
            {
                let mut val: libc::c_int = 0;
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if crate::stdlib::sscanf(
                    *argv.offset(argn as isize),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    &mut val as *mut libc::c_int,
                ) != 1i32
                {
                    usage();
                }
                (*cinfo).desired_number_of_colors = val;
                (*cinfo).quantize_colors = crate::jmorecfg_h::TRUE
            } else if 0
                != crate::cdjpeg::keymatch(
                    arg,
                    b"dct\x00" as *const u8 as *const libc::c_char,
                    2i32,
                )
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if 0 != crate::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"int\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_ISLOW
                } else if 0
                    != crate::cdjpeg::keymatch(
                        *argv.offset(argn as isize),
                        b"fast\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_IFAST
                } else if 0
                    != crate::cdjpeg::keymatch(
                        *argv.offset(argn as isize),
                        b"float\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_FLOAT
                } else {
                    usage();
                }
            } else if 0
                != crate::cdjpeg::keymatch(
                    arg,
                    b"dither\x00" as *const u8 as *const libc::c_char,
                    2i32,
                )
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if 0 != crate::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"fs\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_FS
                } else if 0
                    != crate::cdjpeg::keymatch(
                        *argv.offset(argn as isize),
                        b"none\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_NONE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        *argv.offset(argn as isize),
                        b"ordered\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_ORDERED
                } else {
                    usage();
                }
            } else if 0
                != crate::cdjpeg::keymatch(
                    arg,
                    b"debug\x00" as *const u8 as *const libc::c_char,
                    1i32,
                )
                || 0 != crate::cdjpeg::keymatch(
                    arg,
                    b"verbose\x00" as *const u8 as *const libc::c_char,
                    1i32,
                )
            {
                static mut printed_version: crate::jmorecfg_h::boolean = crate::jmorecfg_h::FALSE;
                if 0 == printed_version {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s version %s (build %s)\n\x00" as *const u8 as *const libc::c_char,
                        crate::jconfigint_h::PACKAGE_NAME.as_ptr(),
                        crate::jconfigint_h::VERSION.as_ptr(),
                        crate::jconfigint_h::BUILD.as_ptr(),
                    );
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s\n\n\x00" as *const u8 as *const libc::c_char,
                        crate::jversion_h::JCOPYRIGHT.as_ptr(),
                    );
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"Emulating The Independent JPEG Group\'s software, version %s\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        crate::jversion_h::JVERSION.as_ptr(),
                    );
                    printed_version = crate::jmorecfg_h::TRUE
                }
                (*(*cinfo).err).trace_level += 1
            } else if 0
                != crate::cdjpeg::keymatch(
                    arg,
                    b"version\x00" as *const u8 as *const libc::c_char,
                    4i32,
                )
            {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s version %s (build %s)\n\x00" as *const u8 as *const libc::c_char,
                    crate::jconfigint_h::PACKAGE_NAME.as_ptr(),
                    crate::jconfigint_h::VERSION.as_ptr(),
                    crate::jconfigint_h::BUILD.as_ptr(),
                );
                crate::stdlib::exit(crate::stdlib::EXIT_SUCCESS);
            } else {
                if 0 != crate::cdjpeg::keymatch(
                    arg,
                    b"fast\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) {
                    (*cinfo).two_pass_quantize = crate::jmorecfg_h::FALSE;
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_ORDERED;
                    if 0 == (*cinfo).quantize_colors {
                        (*cinfo).desired_number_of_colors = 216i32
                    }
                    (*cinfo).dct_method =
                        crate::jpeglib_h::JDCT_FASTEST as crate::jpeglib_h::J_DCT_METHOD;
                    (*cinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"gif\x00" as *const u8 as *const libc::c_char,
                        1i32,
                    )
                {
                    requested_fmt = FMT_GIF
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"grayscale\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                    || 0 != crate::cdjpeg::keymatch(
                        arg,
                        b"greyscale\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_GRAYSCALE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"rgb\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_RGB
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"rgb565\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_RGB565
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"icc\x00" as *const u8 as *const libc::c_char,
                        1i32,
                    )
                {
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    icc_filename = *argv.offset(argn as isize);
                    crate::jpeglib_h::jpeg_save_markers(
                        cinfo,
                        crate::jpeglib_h::JPEG_APP0 + 2i32,
                        0xffffi32 as libc::c_uint,
                    );
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"map\x00" as *const u8 as *const libc::c_char,
                        3i32,
                    )
                {
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if 0 != for_real {
                        let mut mapfile: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
                        mapfile = crate::stdlib::fopen(
                            *argv.offset(argn as isize),
                            crate::cdjpeg::READ_BINARY.as_ptr(),
                        );
                        if mapfile.is_null() {
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                                progname,
                                *argv.offset(argn as isize),
                            );
                            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
                        }
                        crate::cdjpeg::read_color_map(cinfo, mapfile);
                        crate::stdlib::fclose(mapfile);
                        (*cinfo).quantize_colors = crate::jmorecfg_h::TRUE
                    }
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"maxmemory\x00" as *const u8 as *const libc::c_char,
                        3i32,
                    )
                {
                    let mut lval: libc::c_long = 0;
                    let mut ch: libc::c_char = 'x' as i32 as libc::c_char;
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%ld%c\x00" as *const u8 as *const libc::c_char,
                        &mut lval as *mut libc::c_long,
                        &mut ch as *mut libc::c_char,
                    ) < 1i32
                    {
                        usage();
                    }
                    if ch as libc::c_int == 'm' as i32 || ch as libc::c_int == 'M' as i32 {
                        lval *= 1000i64
                    }
                    (*(*cinfo).mem).max_memory_to_use = lval * 1000i64
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"nosmooth\x00" as *const u8 as *const libc::c_char,
                        3i32,
                    )
                {
                    (*cinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"onepass\x00" as *const u8 as *const libc::c_char,
                        3i32,
                    )
                {
                    (*cinfo).two_pass_quantize = crate::jmorecfg_h::FALSE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"os2\x00" as *const u8 as *const libc::c_char,
                        3i32,
                    )
                {
                    requested_fmt = FMT_OS2
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"outfile\x00" as *const u8 as *const libc::c_char,
                        4i32,
                    )
                {
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    outfilename = *argv.offset(argn as isize)
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"memsrc\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    memsrc = crate::jmorecfg_h::TRUE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"pnm\x00" as *const u8 as *const libc::c_char,
                        1i32,
                    )
                    || 0 != crate::cdjpeg::keymatch(
                        arg,
                        b"ppm\x00" as *const u8 as *const libc::c_char,
                        1i32,
                    )
                {
                    requested_fmt = FMT_PPM
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"rle\x00" as *const u8 as *const libc::c_char,
                        1i32,
                    )
                {
                    requested_fmt = FMT_RLE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"scale\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%u/%u\x00" as *const u8 as *const libc::c_char,
                        &mut (*cinfo).scale_num as *mut libc::c_uint,
                        &mut (*cinfo).scale_denom as *mut libc::c_uint,
                    ) != 2i32
                    {
                        usage();
                    }
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"skip\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%u,%u\x00" as *const u8 as *const libc::c_char,
                        &mut skip_start as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut skip_end as *mut crate::jmorecfg_h::JDIMENSION,
                    ) != 2i32
                        || skip_start > skip_end
                    {
                        usage();
                    }
                    skip = crate::jmorecfg_h::TRUE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"crop\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    )
                {
                    let mut c: libc::c_char = 0;
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%u%c%u+%u+%u\x00" as *const u8 as *const libc::c_char,
                        &mut crop_width as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut c as *mut libc::c_char,
                        &mut crop_height as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut crop_x as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut crop_y as *mut crate::jmorecfg_h::JDIMENSION,
                    ) != 5i32
                        || c as libc::c_int != 'X' as i32 && c as libc::c_int != 'x' as i32
                        || crop_width < 1i32 as libc::c_uint
                        || crop_height < 1i32 as libc::c_uint
                    {
                        usage();
                    }
                    crop = crate::jmorecfg_h::TRUE
                } else if 0
                    != crate::cdjpeg::keymatch(
                        arg,
                        b"targa\x00" as *const u8 as *const libc::c_char,
                        1i32,
                    )
                {
                    requested_fmt = FMT_TARGA
                } else {
                    usage();
                }
            }
        }
        argn += 1
    }
    return argn;
}
/*
 * Marker processor for COM and interesting APPn markers.
 * This replaces the library's built-in processor, which just skips the marker.
 * We want to print out the marker as text, to the extent possible.
 * Note this code relies on a non-suspending data source.
 */
unsafe extern "C" fn jpeg_getc(mut cinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_uint {
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    if (*datasrc).bytes_in_buffer == 0i32 as libc::c_ulong {
        if 0 == (*datasrc)
            .fill_input_buffer
            .expect("non-null function pointer")(cinfo)
        {
            (*(*cinfo).err).msg_code = crate::jerror::JERR_CANT_SUSPEND as libc::c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    (*datasrc).bytes_in_buffer = (*datasrc).bytes_in_buffer.wrapping_sub(1);
    let fresh0 = (*datasrc).next_input_byte;
    (*datasrc).next_input_byte = (*datasrc).next_input_byte.offset(1);
    return *fresh0 as libc::c_uint;
}
unsafe extern "C" fn print_text_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut traceit: crate::jmorecfg_h::boolean =
        ((*(*cinfo).err).trace_level >= 1i32) as libc::c_int;
    let mut length: libc::c_long = 0;
    let mut ch: libc::c_uint = 0;
    let mut lastch: libc::c_uint = 0i32 as libc::c_uint;
    length = (jpeg_getc(cinfo) << 8i32) as libc::c_long;
    length += jpeg_getc(cinfo) as libc::c_long;
    length -= 2i32 as libc::c_long;
    if 0 != traceit {
        if (*cinfo).unread_marker == crate::jpeglib_h::JPEG_COM {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"Comment, length %ld:\n\x00" as *const u8 as *const libc::c_char,
                length,
            );
        } else {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"APP%d, length %ld:\n\x00" as *const u8 as *const libc::c_char,
                (*cinfo).unread_marker - crate::jpeglib_h::JPEG_APP0,
                length,
            );
        }
    }
    loop {
        length -= 1;
        if !(length >= 0i32 as libc::c_long) {
            break;
        }
        ch = jpeg_getc(cinfo);
        if 0 != traceit {
            if ch == '\r' as i32 as libc::c_uint {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                );
            } else if ch == '\n' as i32 as libc::c_uint {
                if lastch != '\r' as i32 as libc::c_uint {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char,
                    );
                }
            } else if ch == '\\' as i32 as libc::c_uint {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"\\\\\x00" as *const u8 as *const libc::c_char,
                );
            } else if 0
                != *(*crate::stdlib::__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & crate::stdlib::_ISprint as libc::c_int as libc::c_ushort as libc::c_int
            {
                crate::stdlib::putc(ch as libc::c_int, crate::stdlib::stderr);
            } else {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"\\%03o\x00" as *const u8 as *const libc::c_char,
                    ch,
                );
            }
            lastch = ch
        }
    }
    if 0 != traceit {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    return crate::jmorecfg_h::TRUE;
}
/*
 * The main program.
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut cinfo: crate::jpeglib_h::jpeg_decompress_struct =
        crate::jpeglib_h::jpeg_decompress_struct {
            err: 0 as *mut crate::jpeglib_h::jpeg_error_mgr,
            mem: 0 as *mut crate::jpeglib_h::jpeg_memory_mgr,
            progress: 0 as *mut crate::jpeglib_h::jpeg_progress_mgr,
            client_data: 0 as *mut libc::c_void,
            is_decompressor: 0,
            global_state: 0,
            src: 0 as *mut crate::jpeglib_h::jpeg_source_mgr,
            image_width: 0,
            image_height: 0,
            num_components: 0,
            jpeg_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            out_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            scale_num: 0,
            scale_denom: 0,
            output_gamma: 0.,
            buffered_image: 0,
            raw_data_out: 0,
            dct_method: crate::jpeglib_h::JDCT_ISLOW,
            do_fancy_upsampling: 0,
            do_block_smoothing: 0,
            quantize_colors: 0,
            dither_mode: crate::jpeglib_h::JDITHER_NONE,
            two_pass_quantize: 0,
            desired_number_of_colors: 0,
            enable_1pass_quant: 0,
            enable_external_quant: 0,
            enable_2pass_quant: 0,
            output_width: 0,
            output_height: 0,
            out_color_components: 0,
            output_components: 0,
            rec_outbuf_height: 0,
            actual_number_of_colors: 0,
            colormap: 0 as *mut crate::jpeglib_h::JSAMPROW,
            output_scanline: 0,
            input_scan_number: 0,
            input_iMCU_row: 0,
            output_scan_number: 0,
            output_iMCU_row: 0,
            coef_bits: 0 as *mut [libc::c_int; 64],
            quant_tbl_ptrs: [0 as *mut crate::jpeglib_h::JQUANT_TBL; 4],
            dc_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            ac_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            data_precision: 0,
            comp_info: 0 as *mut crate::jpeglib_h::jpeg_component_info,
            progressive_mode: 0,
            arith_code: 0,
            arith_dc_L: [0; 16],
            arith_dc_U: [0; 16],
            arith_ac_K: [0; 16],
            restart_interval: 0,
            saw_JFIF_marker: 0,
            JFIF_major_version: 0,
            JFIF_minor_version: 0,
            density_unit: 0,
            X_density: 0,
            Y_density: 0,
            saw_Adobe_marker: 0,
            Adobe_transform: 0,
            CCIR601_sampling: 0,
            marker_list: 0 as *mut crate::jpeglib_h::jpeg_marker_struct,
            max_h_samp_factor: 0,
            max_v_samp_factor: 0,
            min_DCT_scaled_size: 0,
            total_iMCU_rows: 0,
            sample_range_limit: 0 as *mut crate::jmorecfg_h::JSAMPLE,
            comps_in_scan: 0,
            cur_comp_info: [0 as *mut crate::jpeglib_h::jpeg_component_info; 4],
            MCUs_per_row: 0,
            MCU_rows_in_scan: 0,
            blocks_in_MCU: 0,
            MCU_membership: [0; 10],
            Ss: 0,
            Se: 0,
            Ah: 0,
            Al: 0,
            unread_marker: 0,
            master: 0 as *mut crate::jpeglib_h::jpeg_decomp_master,
            main: 0 as *mut crate::jpeglib_h::jpeg_d_main_controller,
            coef: 0 as *mut crate::jpeglib_h::jpeg_d_coef_controller,
            post: 0 as *mut crate::jpeglib_h::jpeg_d_post_controller,
            inputctl: 0 as *mut crate::jpeglib_h::jpeg_input_controller,
            marker: 0 as *mut crate::jpeglib_h::jpeg_marker_reader,
            entropy: 0 as *mut crate::jpeglib_h::jpeg_entropy_decoder,
            idct: 0 as *mut crate::jpeglib_h::jpeg_inverse_dct,
            upsample: 0 as *mut crate::jpeglib_h::jpeg_upsampler,
            cconvert: 0 as *mut crate::jpeglib_h::jpeg_color_deconverter,
            cquantize: 0 as *mut crate::jpeglib_h::jpeg_color_quantizer,
        };
    let mut jerr: crate::jpeglib_h::jpeg_error_mgr = crate::jpeglib_h::jpeg_error_mgr {
        error_exit: None,
        emit_message: None,
        output_message: None,
        format_message: None,
        reset_error_mgr: None,
        msg_code: 0,
        msg_parm: crate::jpeglib_h::C2RustUnnamed_2 { i: [0; 8] },
        trace_level: 0,
        num_warnings: 0,
        jpeg_message_table: 0 as *const *const libc::c_char,
        last_jpeg_message: 0,
        addon_message_table: 0 as *const *const libc::c_char,
        first_addon_message: 0,
        last_addon_message: 0,
    };
    let mut file_index: libc::c_int = 0;
    let mut dest_mgr: crate::cdjpeg::djpeg_dest_ptr =
        crate::stddef_h::NULL as crate::cdjpeg::djpeg_dest_ptr;
    let mut input_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut output_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut inbuffer: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut insize: libc::c_ulong = 0i32 as libc::c_ulong;
    let mut num_scanlines: crate::jmorecfg_h::JDIMENSION = 0;
    progname = *argv.offset(0isize);
    if progname.is_null() || *progname.offset(0isize) as libc::c_int == 0i32 {
        progname = b"djpeg\x00" as *const u8 as *const libc::c_char
    }
    cinfo.err = crate::jpeglib_h::jpeg_std_error(&mut jerr);
    crate::jpeglib_h::jpeg_CreateDecompress(
        &mut cinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_decompress_struct>() as libc::c_ulong,
    );
    jerr.addon_message_table = cdjpeg_message_table.as_ptr();
    jerr.first_addon_message = crate::cderror_h::JMSG_FIRSTADDONCODE as libc::c_int;
    jerr.last_addon_message = crate::cderror_h::JMSG_LASTADDONCODE as libc::c_int;
    crate::jpeglib_h::jpeg_set_marker_processor(
        &mut cinfo,
        crate::jpeglib_h::JPEG_COM,
        Some(
            print_text_marker
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        ),
    );
    crate::jpeglib_h::jpeg_set_marker_processor(
        &mut cinfo,
        crate::jpeglib_h::JPEG_APP0 + 12i32,
        Some(
            print_text_marker
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        ),
    );
    file_index = parse_switches(&mut cinfo, argc, argv, 0i32, crate::jmorecfg_h::FALSE);
    if file_index < argc - 1i32 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: only one input file\n\x00" as *const u8 as *const libc::c_char,
            progname,
        );
        usage();
    }
    if file_index < argc {
        input_file = crate::stdlib::fopen(
            *argv.offset(file_index as isize),
            crate::cdjpeg::READ_BINARY.as_ptr(),
        );
        if input_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                *argv.offset(file_index as isize),
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
    } else {
        input_file = crate::cdjpeg::read_stdin()
    }
    if !outfilename.is_null() {
        output_file = crate::stdlib::fopen(outfilename, crate::cdjpeg::WRITE_BINARY.as_ptr());
        if output_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                outfilename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
    } else {
        output_file = crate::cdjpeg::write_stdout()
    }
    if 0 != memsrc {
        let mut nbytes: crate::stddef_h::size_t = 0;
        loop {
            inbuffer = crate::stdlib::realloc(
                inbuffer as *mut libc::c_void,
                insize.wrapping_add(INPUT_BUF_SIZE as libc::c_ulong),
            ) as *mut libc::c_uchar;
            if inbuffer.is_null() {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: memory allocation failure\n\x00" as *const u8 as *const libc::c_char,
                    progname,
                );
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            }
            nbytes = crate::stdlib::fread(
                &mut *inbuffer.offset(insize as isize) as *mut libc::c_uchar as *mut libc::c_void,
                1i32 as crate::stddef_h::size_t,
                4096i32 as crate::stddef_h::size_t,
                input_file,
            );
            if nbytes < INPUT_BUF_SIZE as libc::c_ulong && 0 != crate::stdlib::ferror(input_file) {
                if file_index < argc {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s: can\'t read from %s\n\x00" as *const u8 as *const libc::c_char,
                        progname,
                        *argv.offset(file_index as isize),
                    );
                } else {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s: can\'t read from stdin\n\x00" as *const u8 as *const libc::c_char,
                        progname,
                    );
                }
            }
            insize = insize.wrapping_add(nbytes);
            if !(nbytes == INPUT_BUF_SIZE as libc::c_ulong) {
                break;
            }
        }
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Compressed size:  %lu bytes\n\x00" as *const u8 as *const libc::c_char,
            insize,
        );
        crate::jpeglib_h::jpeg_mem_src(&mut cinfo, inbuffer, insize);
    } else {
        crate::jpeglib_h::jpeg_stdio_src(&mut cinfo, input_file);
    }
    crate::jpeglib_h::jpeg_read_header(&mut cinfo, crate::jmorecfg_h::TRUE);
    file_index = parse_switches(&mut cinfo, argc, argv, 0i32, crate::jmorecfg_h::TRUE);
    match requested_fmt as libc::c_uint {
        0 => {
            dest_mgr = crate::cdjpeg::jinit_write_bmp(
                &mut cinfo,
                crate::jmorecfg_h::FALSE,
                crate::jmorecfg_h::TRUE,
            )
        }
        2 => {
            dest_mgr = crate::cdjpeg::jinit_write_bmp(
                &mut cinfo,
                crate::jmorecfg_h::TRUE,
                crate::jmorecfg_h::TRUE,
            )
        }
        1 => dest_mgr = crate::cdjpeg::jinit_write_gif(&mut cinfo),
        3 => dest_mgr = crate::cdjpeg::jinit_write_ppm(&mut cinfo),
        5 => dest_mgr = crate::cdjpeg::jinit_write_targa(&mut cinfo),
        _ => {
            (*cinfo.err).msg_code = crate::cderror_h::JERR_UNSUPPORTED_FORMAT as libc::c_int;
            (*cinfo.err).error_exit.expect("non-null function pointer")(
                &mut cinfo as *mut crate::jpeglib_h::jpeg_decompress_struct
                    as crate::jpeglib_h::j_common_ptr,
            );
        }
    }
    (*dest_mgr).output_file = output_file;
    crate::jpeglib_h::jpeg_start_decompress(&mut cinfo);
    if 0 != skip {
        let mut tmp: crate::jmorecfg_h::JDIMENSION = 0;
        if skip_end > cinfo.output_height.wrapping_sub(1i32 as libc::c_uint) {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: skip region exceeds image height %d\n\x00" as *const u8
                    as *const libc::c_char,
                progname,
                cinfo.output_height,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        tmp = cinfo.output_height;
        cinfo.output_height = (cinfo.output_height as libc::c_uint).wrapping_sub(
            skip_end
                .wrapping_sub(skip_start)
                .wrapping_add(1i32 as libc::c_uint),
        ) as crate::jmorecfg_h::JDIMENSION
            as crate::jmorecfg_h::JDIMENSION;
        (*dest_mgr).start_output.expect("non-null function pointer")(&mut cinfo, dest_mgr);
        cinfo.output_height = tmp;
        while cinfo.output_scanline < skip_start {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            (*dest_mgr)
                .put_pixel_rows
                .expect("non-null function pointer")(
                &mut cinfo, dest_mgr, num_scanlines
            );
        }
        crate::jpeglib_h::jpeg_skip_scanlines(
            &mut cinfo,
            skip_end
                .wrapping_sub(skip_start)
                .wrapping_add(1i32 as libc::c_uint),
        );
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            (*dest_mgr)
                .put_pixel_rows
                .expect("non-null function pointer")(
                &mut cinfo, dest_mgr, num_scanlines
            );
        }
    } else if 0 != crop {
        let mut tmp_0: crate::jmorecfg_h::JDIMENSION = 0;
        if crop_x.wrapping_add(crop_width) > cinfo.output_width
            || crop_y.wrapping_add(crop_height) > cinfo.output_height
        {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: crop dimensions exceed image dimensions %d x %d\n\x00" as *const u8
                    as *const libc::c_char,
                progname,
                cinfo.output_width,
                cinfo.output_height,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        crate::jpeglib_h::jpeg_crop_scanline(&mut cinfo, &mut crop_x, &mut crop_width);
        if (*dest_mgr).calc_buffer_dimensions.is_some() {
            (*dest_mgr)
                .calc_buffer_dimensions
                .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        } else {
            (*cinfo.err).msg_code = crate::cderror_h::JERR_UNSUPPORTED_FORMAT as libc::c_int;
            (*cinfo.err).error_exit.expect("non-null function pointer")(
                &mut cinfo as *mut crate::jpeglib_h::jpeg_decompress_struct
                    as crate::jpeglib_h::j_common_ptr,
            );
        }
        tmp_0 = cinfo.output_height;
        cinfo.output_height = crop_height;
        (*dest_mgr).start_output.expect("non-null function pointer")(&mut cinfo, dest_mgr);
        cinfo.output_height = tmp_0;
        crate::jpeglib_h::jpeg_skip_scanlines(&mut cinfo, crop_y);
        while cinfo.output_scanline < crop_y.wrapping_add(crop_height) {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            (*dest_mgr)
                .put_pixel_rows
                .expect("non-null function pointer")(
                &mut cinfo, dest_mgr, num_scanlines
            );
        }
        crate::jpeglib_h::jpeg_skip_scanlines(
            &mut cinfo,
            cinfo
                .output_height
                .wrapping_sub(crop_y)
                .wrapping_sub(crop_height),
        );
    } else {
        (*dest_mgr).start_output.expect("non-null function pointer")(&mut cinfo, dest_mgr);
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            (*dest_mgr)
                .put_pixel_rows
                .expect("non-null function pointer")(
                &mut cinfo, dest_mgr, num_scanlines
            );
        }
    }
    if !icc_filename.is_null() {
        let mut icc_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
        let mut icc_profile: *mut crate::jmorecfg_h::JOCTET = 0 as *mut crate::jmorecfg_h::JOCTET;
        let mut icc_len: libc::c_uint = 0;
        icc_file = crate::stdlib::fopen(icc_filename, crate::cdjpeg::WRITE_BINARY.as_ptr());
        if icc_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        if 0 != crate::jpeglib_h::jpeg_read_icc_profile(&mut cinfo, &mut icc_profile, &mut icc_len)
        {
            if crate::stdlib::fwrite(
                icc_profile as *const libc::c_void,
                icc_len as libc::c_ulong,
                1i32 as libc::c_ulong,
                icc_file,
            ) < 1i32 as libc::c_ulong
            {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: can\'t read ICC profile from %s\n\x00" as *const u8
                        as *const libc::c_char,
                    progname,
                    icc_filename,
                );
                crate::stdlib::free(icc_profile as *mut libc::c_void);
                crate::stdlib::fclose(icc_file);
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            }
            crate::stdlib::free(icc_profile as *mut libc::c_void);
            crate::stdlib::fclose(icc_file);
        } else if (*cinfo.err).msg_code != crate::jerror::JWRN_BOGUS_ICC as libc::c_int {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: no ICC profile data in JPEG file\n\x00" as *const u8 as *const libc::c_char,
                progname,
            );
        }
    }
    (*dest_mgr)
        .finish_output
        .expect("non-null function pointer")(&mut cinfo, dest_mgr);
    crate::jpeglib_h::jpeg_finish_decompress(&mut cinfo);
    crate::jpeglib_h::jpeg_destroy_decompress(&mut cinfo);
    if input_file != crate::stdlib::stdin {
        crate::stdlib::fclose(input_file);
    }
    if output_file != crate::stdlib::stdout {
        crate::stdlib::fclose(output_file);
    }
    if 0 != memsrc && !inbuffer.is_null() {
        crate::stdlib::free(inbuffer as *mut libc::c_void);
    }
    crate::stdlib::exit(if 0 != jerr.num_warnings {
        crate::cdjpeg::EXIT_WARNING
    } else {
        crate::stdlib::EXIT_SUCCESS
    });
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
