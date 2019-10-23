use libc::c_uint;use libc::c_ulong;use libc::c_void;use libc::c_long;use libc::c_uchar;use libc::c_int;use libc;

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:25"]
pub mod jmorecfg_h {

    use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;use libc::c_int;use crate::jmorecfg_h::RGB_PIXELSIZE;pub static mut rgb_pixelsize: [c_int; 17] = [
        -1i32,
        -1i32,
        RGB_PIXELSIZE,
        -1i32,
        -1i32,
        -1i32,
        EXT_RGB_PIXELSIZE,
        EXT_RGBX_PIXELSIZE,
        EXT_BGR_PIXELSIZE,
        EXT_BGRX_PIXELSIZE,
        EXT_XBGR_PIXELSIZE,
        EXT_XRGB_PIXELSIZE,
        EXT_RGBX_PIXELSIZE,
        EXT_BGRX_PIXELSIZE,
        EXT_XBGR_PIXELSIZE,
        EXT_XRGB_PIXELSIZE,
        -1i32,
    ];

    /* JPEG_INTERNAL_OPTIONS */
    /* FAST_FLOAT should be either float or double, whichever is done faster
     * by your compiler.  (Note that this type is only used in the floating point
     * DCT routines, so it only matters if you've defined DCT_FLOAT_SUPPORTED.)
     */
    /* prefer 16-bit with SIMD for parellelism */
    /* On some machines (notably 68000 series) "int" is 32 bits, but multiplying
     * two 16-bit shorts is faster than multiplying two ints.  Define MULTIPLIER
     * as short on such a machine.  MULTIPLIER must be at least 16 bits wide.
     */
    /* Definitions for speed-related optimizations. */
}

/* Define to `unsigned int' if <sys/types.h> does not define. */

/* #undef size_t */

/* Define to empty if `const' does not conform to ANSI C. */

/* #undef const */

/* #undef __CHAR_UNSIGNED__ */

/* Define to 1 if type `char' is unsigned and you are not using gcc.  */

/* Define if your (broken) compiler shifts signed values as if they were
unsigned. */

/* #undef RIGHT_SHIFT_IS_UNSIGNED */

/* Compiler does not support pointers to undefined structures. */

/* #undef INCOMPLETE_TYPES_BROKEN */

/* Define to 1 if the system has the type `unsigned short'. */

/* Define to 1 if the system has the type `unsigned char'. */

/* Define if you have BSD-like bzero and bcopy in <strings.h> rather than
memset/memcpy in <string.h>. */

/* #undef NEED_BSD_STRINGS */

/* Define if you need to include <sys/types.h> to get size_t. */

/* Define to 1 if you have the <stdlib.h> header file. */

/* Define to 1 if you have the <stddef.h> header file. */

/* Define to 1 if you have the <locale.h> header file. */

/* use 8 or 12 */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::cderror_h::C2RustUnnamed_4;
pub use crate::cderror_h::JERR_BAD_CMAP_FILE;
pub use crate::cderror_h::JERR_BMP_BADCMAP;
pub use crate::cderror_h::JERR_BMP_BADDEPTH;
pub use crate::cderror_h::JERR_BMP_BADHEADER;
pub use crate::cderror_h::JERR_BMP_BADPLANES;
pub use crate::cderror_h::JERR_BMP_COLORSPACE;
pub use crate::cderror_h::JERR_BMP_COMPRESSED;
pub use crate::cderror_h::JERR_BMP_EMPTY;
pub use crate::cderror_h::JERR_BMP_NOT;
pub use crate::cderror_h::JERR_BMP_OUTOFRANGE;
pub use crate::cderror_h::JERR_GIF_BUG;
pub use crate::cderror_h::JERR_GIF_CODESIZE;
pub use crate::cderror_h::JERR_GIF_COLORSPACE;
pub use crate::cderror_h::JERR_GIF_IMAGENOTFOUND;
pub use crate::cderror_h::JERR_GIF_NOT;
pub use crate::cderror_h::JERR_PPM_COLORSPACE;
pub use crate::cderror_h::JERR_PPM_NONNUMERIC;
pub use crate::cderror_h::JERR_PPM_NOT;
pub use crate::cderror_h::JERR_PPM_OUTOFRANGE;
pub use crate::cderror_h::JERR_TGA_BADCMAP;
pub use crate::cderror_h::JERR_TGA_BADPARMS;
pub use crate::cderror_h::JERR_TGA_COLORSPACE;
pub use crate::cderror_h::JERR_TOO_MANY_COLORS;
pub use crate::cderror_h::JERR_UNGETC_FAILED;
pub use crate::cderror_h::JERR_UNKNOWN_FORMAT;
pub use crate::cderror_h::JERR_UNSUPPORTED_FORMAT;
pub use crate::cderror_h::JMSG_FIRSTADDONCODE;
pub use crate::cderror_h::JMSG_LASTADDONCODE;
pub use crate::cderror_h::JTRC_BMP;
pub use crate::cderror_h::JTRC_BMP_MAPPED;
pub use crate::cderror_h::JTRC_BMP_OS2;
pub use crate::cderror_h::JTRC_BMP_OS2_MAPPED;
pub use crate::cderror_h::JTRC_GIF;
pub use crate::cderror_h::JTRC_GIF_BADVERSION;
pub use crate::cderror_h::JTRC_GIF_EXTENSION;
pub use crate::cderror_h::JTRC_GIF_NONSQUARE;
pub use crate::cderror_h::JTRC_PGM;
pub use crate::cderror_h::JTRC_PGM_TEXT;
pub use crate::cderror_h::JTRC_PPM;
pub use crate::cderror_h::JTRC_PPM_TEXT;
pub use crate::cderror_h::JTRC_TGA;
pub use crate::cderror_h::JTRC_TGA_GRAY;
pub use crate::cderror_h::JTRC_TGA_MAPPED;
pub use crate::cderror_h::JWRN_GIF_BADDATA;
pub use crate::cderror_h::JWRN_GIF_CHAR;
pub use crate::cderror_h::JWRN_GIF_ENDCODE;
pub use crate::cderror_h::JWRN_GIF_NOMOREDATA;
pub use crate::cmyk_h::rgb_to_cmyk;
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::rgb_blue;
pub use crate::jmorecfg_h::rgb_green;
pub use crate::jmorecfg_h::rgb_red;
pub use crate::jmorecfg_h::EXT_BGRX_BLUE;
pub use crate::jmorecfg_h::EXT_BGRX_GREEN;
pub use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGRX_RED;
pub use crate::jmorecfg_h::EXT_BGR_BLUE;
pub use crate::jmorecfg_h::EXT_BGR_GREEN;
pub use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGR_RED;
pub use crate::jmorecfg_h::EXT_RGBX_BLUE;
pub use crate::jmorecfg_h::EXT_RGBX_GREEN;
pub use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGBX_RED;
pub use crate::jmorecfg_h::EXT_RGB_BLUE;
pub use crate::jmorecfg_h::EXT_RGB_GREEN;
pub use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGB_RED;
pub use crate::jmorecfg_h::EXT_XBGR_BLUE;
pub use crate::jmorecfg_h::EXT_XBGR_GREEN;
pub use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XBGR_RED;
pub use crate::jmorecfg_h::EXT_XRGB_BLUE;
pub use crate::jmorecfg_h::EXT_XRGB_GREEN;
pub use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XRGB_RED;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_PIXELSIZE;
pub use crate::jmorecfg_h::RGB_RED;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_EXT_ABGR;
pub use crate::jpeglib_h::JCS_EXT_ARGB;
pub use crate::jpeglib_h::JCS_EXT_BGR;
pub use crate::jpeglib_h::JCS_EXT_BGRA;
pub use crate::jpeglib_h::JCS_EXT_BGRX;
pub use crate::jpeglib_h::JCS_EXT_RGB;
pub use crate::jpeglib_h::JCS_EXT_RGBA;
pub use crate::jpeglib_h::JCS_EXT_RGBX;
pub use crate::jpeglib_h::JCS_EXT_XBGR;
pub use crate::jpeglib_h::JCS_EXT_XRGB;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_RGB565;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use super::cdjpeg::cjpeg_source_ptr;
pub use super::cdjpeg::cjpeg_source_struct;
pub use super::jerror::C2RustUnnamed_3;
pub use super::jerror::JERR_ARITH_NOTIMPL;
pub use super::jerror::JERR_BAD_ALIGN_TYPE;
pub use super::jerror::JERR_BAD_ALLOC_CHUNK;
pub use super::jerror::JERR_BAD_BUFFER_MODE;
pub use super::jerror::JERR_BAD_COMPONENT_ID;
pub use super::jerror::JERR_BAD_CROP_SPEC;
pub use super::jerror::JERR_BAD_DCTSIZE;
pub use super::jerror::JERR_BAD_DCT_COEF;
pub use super::jerror::JERR_BAD_HUFF_TABLE;
pub use super::jerror::JERR_BAD_IN_COLORSPACE;
pub use super::jerror::JERR_BAD_J_COLORSPACE;
pub use super::jerror::JERR_BAD_LENGTH;
pub use super::jerror::JERR_BAD_LIB_VERSION;
pub use super::jerror::JERR_BAD_MCU_SIZE;
pub use super::jerror::JERR_BAD_PARAM;
pub use super::jerror::JERR_BAD_PARAM_VALUE;
pub use super::jerror::JERR_BAD_POOL_ID;
pub use super::jerror::JERR_BAD_PRECISION;
pub use super::jerror::JERR_BAD_PROGRESSION;
pub use super::jerror::JERR_BAD_PROG_SCRIPT;
pub use super::jerror::JERR_BAD_SAMPLING;
pub use super::jerror::JERR_BAD_SCAN_SCRIPT;
pub use super::jerror::JERR_BAD_STATE;
pub use super::jerror::JERR_BAD_STRUCT_SIZE;
pub use super::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use super::jerror::JERR_BUFFER_SIZE;
pub use super::jerror::JERR_CANT_SUSPEND;
pub use super::jerror::JERR_CCIR601_NOTIMPL;
pub use super::jerror::JERR_COMPONENT_COUNT;
pub use super::jerror::JERR_CONVERSION_NOTIMPL;
pub use super::jerror::JERR_DAC_INDEX;
pub use super::jerror::JERR_DAC_VALUE;
pub use super::jerror::JERR_DHT_INDEX;
pub use super::jerror::JERR_DQT_INDEX;
pub use super::jerror::JERR_EMPTY_IMAGE;
pub use super::jerror::JERR_EMS_READ;
pub use super::jerror::JERR_EMS_WRITE;
pub use super::jerror::JERR_EOI_EXPECTED;
pub use super::jerror::JERR_FILE_READ;
pub use super::jerror::JERR_FILE_WRITE;
pub use super::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use super::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use super::jerror::JERR_HUFF_MISSING_CODE;
pub use super::jerror::JERR_IMAGE_TOO_BIG;
pub use super::jerror::JERR_INPUT_EMPTY;
pub use super::jerror::JERR_INPUT_EOF;
pub use super::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use super::jerror::JERR_MISSING_DATA;
pub use super::jerror::JERR_MODE_CHANGE;
pub use super::jerror::JERR_NOTIMPL;
pub use super::jerror::JERR_NOT_COMPILED;
pub use super::jerror::JERR_NO_BACKING_STORE;
pub use super::jerror::JERR_NO_HUFF_TABLE;
pub use super::jerror::JERR_NO_IMAGE;
pub use super::jerror::JERR_NO_QUANT_TABLE;
pub use super::jerror::JERR_NO_SOI;
pub use super::jerror::JERR_OUT_OF_MEMORY;
pub use super::jerror::JERR_QUANT_COMPONENTS;
pub use super::jerror::JERR_QUANT_FEW_COLORS;
pub use super::jerror::JERR_QUANT_MANY_COLORS;
pub use super::jerror::JERR_SOF_DUPLICATE;
pub use super::jerror::JERR_SOF_NO_SOS;
pub use super::jerror::JERR_SOF_UNSUPPORTED;
pub use super::jerror::JERR_SOI_DUPLICATE;
pub use super::jerror::JERR_SOS_NO_SOF;
pub use super::jerror::JERR_TFILE_CREATE;
pub use super::jerror::JERR_TFILE_READ;
pub use super::jerror::JERR_TFILE_SEEK;
pub use super::jerror::JERR_TFILE_WRITE;
pub use super::jerror::JERR_TOO_LITTLE_DATA;
pub use super::jerror::JERR_UNKNOWN_MARKER;
pub use super::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use super::jerror::JERR_VIRTUAL_BUG;
pub use super::jerror::JERR_WIDTH_OVERFLOW;
pub use super::jerror::JERR_XMS_READ;
pub use super::jerror::JERR_XMS_WRITE;
pub use super::jerror::JMSG_COPYRIGHT;
pub use super::jerror::JMSG_LASTMSGCODE;
pub use super::jerror::JMSG_NOMESSAGE;
pub use super::jerror::JMSG_VERSION;
pub use super::jerror::JTRC_16BIT_TABLES;
pub use super::jerror::JTRC_ADOBE;
pub use super::jerror::JTRC_APP0;
pub use super::jerror::JTRC_APP14;
pub use super::jerror::JTRC_DAC;
pub use super::jerror::JTRC_DHT;
pub use super::jerror::JTRC_DQT;
pub use super::jerror::JTRC_DRI;
pub use super::jerror::JTRC_EMS_CLOSE;
pub use super::jerror::JTRC_EMS_OPEN;
pub use super::jerror::JTRC_EOI;
pub use super::jerror::JTRC_HUFFBITS;
pub use super::jerror::JTRC_JFIF;
pub use super::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use super::jerror::JTRC_JFIF_EXTENSION;
pub use super::jerror::JTRC_JFIF_THUMBNAIL;
pub use super::jerror::JTRC_MISC_MARKER;
pub use super::jerror::JTRC_PARMLESS_MARKER;
pub use super::jerror::JTRC_QUANTVALS;
pub use super::jerror::JTRC_QUANT_3_NCOLORS;
pub use super::jerror::JTRC_QUANT_NCOLORS;
pub use super::jerror::JTRC_QUANT_SELECTED;
pub use super::jerror::JTRC_RECOVERY_ACTION;
pub use super::jerror::JTRC_RST;
pub use super::jerror::JTRC_SMOOTH_NOTIMPL;
pub use super::jerror::JTRC_SOF;
pub use super::jerror::JTRC_SOF_COMPONENT;
pub use super::jerror::JTRC_SOI;
pub use super::jerror::JTRC_SOS;
pub use super::jerror::JTRC_SOS_COMPONENT;
pub use super::jerror::JTRC_SOS_PARAMS;
pub use super::jerror::JTRC_TFILE_CLOSE;
pub use super::jerror::JTRC_TFILE_OPEN;
pub use super::jerror::JTRC_THUMB_JPEG;
pub use super::jerror::JTRC_THUMB_PALETTE;
pub use super::jerror::JTRC_THUMB_RGB;
pub use super::jerror::JTRC_UNKNOWN_IDS;
pub use super::jerror::JTRC_XMS_CLOSE;
pub use super::jerror::JTRC_XMS_OPEN;
pub use super::jerror::JWRN_ADOBE_XFORM;
pub use super::jerror::JWRN_BOGUS_ICC;
pub use super::jerror::JWRN_BOGUS_PROGRESSION;
pub use super::jerror::JWRN_EXTRANEOUS_DATA;
pub use super::jerror::JWRN_HIT_MARKER;
pub use super::jerror::JWRN_HUFF_BAD_CODE;
pub use super::jerror::JWRN_JFIF_MAJOR;
pub use super::jerror::JWRN_JPEG_EOF;
pub use super::jerror::JWRN_MUST_RESYNC;
pub use super::jerror::JWRN_NOT_SEQUENTIAL;
pub use super::jerror::JWRN_TOO_MUCH_DATA;
pub use jmorecfg_h::rgb_pixelsize;
pub use crate::stdlib::fread;
pub use crate::stdlib::getc;
pub use crate::stdlib::EOF;

pub type ppm_source_ptr = *mut ppm_source_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppm_source_struct {
    pub pub_0: super::cdjpeg::cjpeg_source_struct,
    pub iobuffer: *mut U_CHAR,
    pub pixrow: JSAMPROW,
    pub buffer_width: size_t,
    pub rescale: *mut JSAMPLE,
    pub maxval: c_uint,
}
/*
 * rdppm.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2009 by Bill Allombert, Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015-2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to read input images in PPM/PGM format.
 * The extended 2-byte-per-sample raw PPM/PGM formats are supported.
 * The PBMPLUS library is NOT required to compile this software
 * (but it is highly useful as a set of PPM image manipulation programs).
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume input from
 * an ordinary stdio stream.  They further assume that reading begins
 * at the start of the file; start_input may need work if the
 * user interface has already read some data (e.g., to determine that
 * the file is indeed PPM format).
 */
/* Portions of this code are based on the PBMPLUS library, which is:
**
** Copyright (C) 1988 by Jef Poskanzer.
**
** Permission to use, copy, modify, and distribute this software and its
** documentation for any purpose and without fee is hereby granted, provided
** that the above copyright notice appear in all copies and that both that
** copyright notice and this permission notice appear in supporting
** documentation.  This software is provided "as is" without express or
** implied warranty.
*/
/* Macros to deal with unsigned chars as efficiently as compiler allows */

pub type U_CHAR = c_uchar;
/* !HAVE_UNSIGNED_CHAR */
/* HAVE_UNSIGNED_CHAR */

static mut alpha_index: [c_int; 17] = [
    -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, 3i32, 3i32,
    0i32, 0i32, -1i32,
];

unsafe extern "C" fn pbm_getc(mut infile: *mut FILE) -> c_int
/* Read next char, skipping over any comments */
/* A comment/newline sequence is returned as a newline */ {
     
     let mut ch:   c_int =  getc(infile);
    if ch == '#' as i32 {
        loop {
            ch = getc(infile);
            if !(ch != '\n' as i32 && ch != EOF) {
                break;
            }
        }
    }
    return ch;
}

unsafe extern "C" fn read_pbm_integer(
    mut cinfo: j_compress_ptr,
    mut infile: *mut FILE,
    mut maxval: c_uint,
) -> c_uint
/* Read an unsigned decimal integer from the PPM file */
/* Swallows one trailing character after the integer */
/* Note that on a 16-bit-int machine, only values up to 64k can be read. */
/* This should not be a problem in practice. */ {
    
     let mut ch:  c_int =  0; 
    loop
    /* Skip any leading whitespace */
    {
        ch = pbm_getc(infile);
        if ch == EOF {
            (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
            break;
        }
    }
    if ch < '0' as i32 || ch > '9' as i32 {
        (*(*cinfo).err).msg_code = JERR_PPM_NONNUMERIC as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut val:   c_uint =  (ch - '0' as i32) as c_uint;
    loop {
        ch = pbm_getc(infile);
        if !(ch >= '0' as i32 && ch <= '9' as i32) {
            break;
        }
        val =  val * 10u32;
        val +=  (ch - '0' as i32) as c_uint
    }
    if val > maxval {
        (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return val;
}
/*
 * Read one row of pixels.
 *
 * We provide several different versions depending on input file format.
 * In all cases, input is scaled to the size of JSAMPLE.
 *
 * A really fast path is provided for reading byte/sample raw files with
 * maxval = MAXJSAMPLE, which is the normal case for 8-bit data.
 */

unsafe extern "C" fn get_text_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PGM files with any maxval */ {
      let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
        col -=  1
    }
    return 1u32;
}

unsafe extern "C" fn get_text_gray_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PGM files with any maxval and
   converting to extended RGB */ {
      let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let ref mut fresh1 = *ptr.offset(bindex as isize);
                *fresh1 = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                let ref mut fresh2 = *ptr.offset(gindex as isize);
                *fresh2 = *fresh1;
                *ptr.offset(rindex as isize) = *fresh2;
                *ptr.offset(aindex as isize) = 0xffu8;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let ref mut fresh3 = *ptr.offset(bindex as isize);
                *fresh3 = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                let ref mut fresh4 = *ptr.offset(gindex as isize);
                *fresh4 = *fresh3;
                *ptr.offset(rindex as isize) = *fresh4;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let ref mut fresh5 = *ptr.offset(bindex as isize);
            *fresh5 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let ref mut fresh6 = *ptr.offset(gindex as isize);
            *fresh6 = *fresh5;
            *ptr.offset(rindex as isize) = *fresh6;
            *ptr.offset(aindex as isize) = 0xffu8;
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let ref mut fresh7 = *ptr.offset(bindex as isize);
            *fresh7 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let ref mut fresh8 = *ptr.offset(gindex as isize);
            *fresh8 = *fresh7;
            *ptr.offset(rindex as isize) = *fresh8;
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_text_gray_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PGM files with any maxval and
   converting to CMYK */ {
      let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let mut gray: JSAMPLE =
                read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            rgb_to_cmyk(
                gray,
                gray,
                gray,
                ptr,
                ptr.offset(1),
                ptr.offset(2),
                ptr.offset(3),
            );
            ptr = ptr.offset(4);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let mut gray_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            rgb_to_cmyk(
                gray_0,
                gray_0,
                gray_0,
                ptr,
                ptr.offset(1),
                ptr.offset(2),
                ptr.offset(3),
            );
            ptr = ptr.offset(4);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_text_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PPM files with any maxval */ {
      let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0u32 {
                *ptr.offset(rindex as isize) =
                    read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(gindex as isize) =
                    read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(bindex as isize) =
                    read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(aindex as isize) = 0xffu8;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0u32 {
                *ptr.offset(rindex as isize) =
                    read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(gindex as isize) =
                    read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(bindex as isize) =
                    read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0u32 {
            *ptr.offset(rindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(gindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(bindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(aindex as isize) = 0xffu8;
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            *ptr.offset(rindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(gindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(bindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_text_rgb_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PPM files with any maxval and
   converting to CMYK */ {
      let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let mut r: JSAMPLE =
                read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            let mut g: JSAMPLE =
                read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            let mut b: JSAMPLE =
                read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            rgb_to_cmyk(r, g, b, ptr, ptr.offset(1), ptr.offset(2), ptr.offset(3));
            ptr = ptr.offset(4);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let mut r_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let mut g_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let mut b_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            rgb_to_cmyk(
                r_0,
                g_0,
                b_0,
                ptr,
                ptr.offset(1),
                ptr.offset(2),
                ptr.offset(3),
            );
            ptr = ptr.offset(4);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_scaled_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PGM files with any maxval */ {
       let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut bufferptr:   *mut U_CHAR =  (*source).iobuffer; let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
        let fresh9 = bufferptr;
        bufferptr = bufferptr.offset(1);
        let fresh10 = ptr;
        ptr = ptr.offset(1);
        *fresh10 = *rescale.offset(*fresh9 as c_int as isize);
        col -=  1
    }
    return 1u32;
}

unsafe extern "C" fn get_gray_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PGM files with any maxval
   and converting to extended RGB */ {
       let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut bufferptr:   *mut U_CHAR =  (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh11 = bufferptr;
                bufferptr = bufferptr.offset(1);
                let ref mut fresh12 = *ptr.offset(bindex as isize);
                *fresh12 = *fresh11;
                let ref mut fresh13 = *ptr.offset(gindex as isize);
                *fresh13 = *fresh12;
                *ptr.offset(rindex as isize) = *fresh13;
                *ptr.offset(aindex as isize) = 0xffu8;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh14 = bufferptr;
                bufferptr = bufferptr.offset(1);
                let ref mut fresh15 = *ptr.offset(bindex as isize);
                *fresh15 = *fresh14;
                let ref mut fresh16 = *ptr.offset(gindex as isize);
                *fresh16 = *fresh15;
                *ptr.offset(rindex as isize) = *fresh16;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh17 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let ref mut fresh18 = *ptr.offset(bindex as isize);
            *fresh18 = *rescale.offset(*fresh17 as c_int as isize);
            let ref mut fresh19 = *ptr.offset(gindex as isize);
            *fresh19 = *fresh18;
            *ptr.offset(rindex as isize) = *fresh19;
            *ptr.offset(aindex as isize) = 0xffu8;
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh20 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let ref mut fresh21 = *ptr.offset(bindex as isize);
            *fresh21 = *rescale.offset(*fresh20 as c_int as isize);
            let ref mut fresh22 = *ptr.offset(gindex as isize);
            *fresh22 = *fresh21;
            *ptr.offset(rindex as isize) = *fresh22;
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_gray_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PGM files with any maxval
   and converting to CMYK */ {
       let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut bufferptr:   *mut U_CHAR =  (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh23 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut gray: JSAMPLE = *fresh23;
            rgb_to_cmyk(
                gray,
                gray,
                gray,
                ptr,
                ptr.offset(1),
                ptr.offset(2),
                ptr.offset(3),
            );
            ptr = ptr.offset(4);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh24 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut gray_0: JSAMPLE =
                *rescale.offset(*fresh24 as c_int as isize);
            rgb_to_cmyk(
                gray_0,
                gray_0,
                gray_0,
                ptr,
                ptr.offset(1),
                ptr.offset(2),
                ptr.offset(3),
            );
            ptr = ptr.offset(4);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PPM files with any maxval */ {
       let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut bufferptr:   *mut U_CHAR =  (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh25 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(rindex as isize) = *fresh25;
                let fresh26 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(gindex as isize) = *fresh26;
                let fresh27 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(bindex as isize) = *fresh27;
                *ptr.offset(aindex as isize) = 0xffu8;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh28 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(rindex as isize) = *fresh28;
                let fresh29 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(gindex as isize) = *fresh29;
                let fresh30 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(bindex as isize) = *fresh30;
                ptr = ptr.offset(ps as isize);
                col -=  1
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh31 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(rindex as isize) = *rescale.offset(*fresh31 as c_int as isize);
            let fresh32 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(gindex as isize) = *rescale.offset(*fresh32 as c_int as isize);
            let fresh33 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(bindex as isize) = *rescale.offset(*fresh33 as c_int as isize);
            *ptr.offset(aindex as isize) = 0xffu8;
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh34 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(rindex as isize) = *rescale.offset(*fresh34 as c_int as isize);
            let fresh35 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(gindex as isize) = *rescale.offset(*fresh35 as c_int as isize);
            let fresh36 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(bindex as isize) = *rescale.offset(*fresh36 as c_int as isize);
            ptr = ptr.offset(ps as isize);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_rgb_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PPM files with any maxval and
   converting to CMYK */ {
       let mut col:  JDIMENSION =  0;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut bufferptr:   *mut U_CHAR =  (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh37 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut r: JSAMPLE = *fresh37;
            let fresh38 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut g: JSAMPLE = *fresh38;
            let fresh39 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut b: JSAMPLE = *fresh39;
            rgb_to_cmyk(r, g, b, ptr, ptr.offset(1), ptr.offset(2), ptr.offset(3));
            ptr = ptr.offset(4);
            col -=  1
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh40 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut r_0: JSAMPLE =
                *rescale.offset(*fresh40 as c_int as isize);
            let fresh41 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut g_0: JSAMPLE =
                *rescale.offset(*fresh41 as c_int as isize);
            let fresh42 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut b_0: JSAMPLE =
                *rescale.offset(*fresh42 as c_int as isize);
            rgb_to_cmyk(
                r_0,
                g_0,
                b_0,
                ptr,
                ptr.offset(1),
                ptr.offset(2),
                ptr.offset(3),
            );
            ptr = ptr.offset(4);
            col -=  1
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_raw_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format files with maxval = MAXJSAMPLE.
 * In this case we just read right into the JSAMPLE buffer!
 * Note that same code works for PPM and PGM files.
 */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return 1u32;
}

unsafe extern "C" fn get_word_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-word-format PGM files with any maxval */ {
       let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut bufferptr:   *mut U_CHAR =  (*source).iobuffer; let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
         
        let fresh43 = bufferptr;
        bufferptr = bufferptr.offset(1);
         let mut temp:   c_uint =
     ((*fresh43 as c_int) << 8i32) as c_uint;
        let fresh44 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |=  *fresh44 as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        let fresh45 = ptr;
        ptr = ptr.offset(1);
        *fresh45 = *rescale.offset(temp as isize);
        col -=  1
    }
    return 1u32;
}

unsafe extern "C" fn get_word_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-word-format PPM files with any maxval */ {
       let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    
    
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1u64,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut bufferptr:   *mut U_CHAR =  (*source).iobuffer; let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
         
        let fresh46 = bufferptr;
        bufferptr = bufferptr.offset(1);
         let mut temp:   c_uint =
     ((*fresh46 as c_int) << 8i32) as c_uint;
        let fresh47 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |=  *fresh47 as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        let fresh48 = ptr;
        ptr = ptr.offset(1);
        *fresh48 = *rescale.offset(temp as isize);
        let fresh49 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh49 as c_int) << 8i32) as c_uint;
        let fresh50 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |=  *fresh50 as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        let fresh51 = ptr;
        ptr = ptr.offset(1);
        *fresh51 = *rescale.offset(temp as isize);
        let fresh52 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh52 as c_int) << 8i32) as c_uint;
        let fresh53 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |=  *fresh53 as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        let fresh54 = ptr;
        ptr = ptr.offset(1);
        *fresh54 = *rescale.offset(temp as isize);
        col -=  1
    }
    return 1u32;
}
/*
 * Read the file header; return image size and component count.
 */

unsafe extern "C" fn start_input_ppm(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
            let mut current_block_3:  u64;let mut source: ppm_source_ptr = sinfo as ppm_source_ptr; /* subformat discriminator character */
    
    
    
    
    
    
    
    if getc((*source).pub_0.input_file) != 'P' as i32 {
        (*(*cinfo).err).msg_code = JERR_PPM_NOT as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut c:   c_int =  getc((*source).pub_0.input_file);
    
    /* detect unsupported variants (ie, PBM) before trying to read header */
    match c {
        50 => {
            current_block_3 = 13513818773234778473;
        }
        51 => {
            /* it's a text-format PPM file */
            current_block_3 = 1115462442902857658;
        }
        53 => {
            current_block_3 = 1115462442902857658;
        }
        54 => {
            current_block_3 = 12050339815695834491;
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_PPM_NOT as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
            current_block_3 = 13513818773234778473;
        }
    }
    match current_block_3 {
        1115462442902857658 =>
        /* it's a raw-format PGM file */
        {
            current_block_3 = 12050339815695834491;
        }
        _ => {}
    }
    match current_block_3 {
        12050339815695834491 =>
            /* it's a raw-format PPM file */
            {}
        _ => {}
    }
    
    
     let mut w:   c_uint =
     read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535u32); let mut h:   c_uint =
     read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535u32); let mut maxval:   c_uint =
     read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535u32);
    if w <= 0u32 || h <= 0u32 || maxval <= 0u32 {
        /* error check */
        (*(*cinfo).err).msg_code = JERR_PPM_NOT as c_int; /* we always rescale data to this */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cinfo).data_precision = BITS_IN_JSAMPLE;
    (*cinfo).image_width = w;
    (*cinfo).image_height = h;
    (*source).maxval = maxval;
     /* do we need an I/O buffer? */
     /* do we map input buffer onto I/O buffer? */
     let mut need_iobuffer:   boolean =  TRUE; let mut use_raw_buffer:   boolean =  FALSE; let mut need_rescale:   boolean =  TRUE; /* do we need a rescale array? */
    match c {
        50 => {
            /* it's a text-format PGM file */
            if  (*cinfo).in_color_space
                ==  JCS_UNKNOWN
            {
                (*cinfo).in_color_space = JCS_GRAYSCALE
            }
            (*(*cinfo).err).msg_code = JTRC_PGM_TEXT as c_int;
            (*(*cinfo).err).msg_parm.i[0] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1] = h as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1i32
            );
            if  (*cinfo).in_color_space
                ==  JCS_GRAYSCALE
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if  (*cinfo).in_color_space
                ==  JCS_RGB
                ||  (*cinfo).in_color_space
                    >=  JCS_EXT_RGB
                    &&  (*cinfo).in_color_space
                        <=  JCS_EXT_ARGB
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if  (*cinfo).in_color_space
                ==  JCS_CMYK
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            need_iobuffer = FALSE
        }
        51 => {
            /* it's a text-format PPM file */
            if  (*cinfo).in_color_space
                ==  JCS_UNKNOWN
            {
                (*cinfo).in_color_space = JCS_EXT_RGB
            }
            (*(*cinfo).err).msg_code = JTRC_PPM_TEXT as c_int;
            (*(*cinfo).err).msg_parm.i[0] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1] = h as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1i32
            );
            if  (*cinfo).in_color_space
                ==  JCS_RGB
                ||  (*cinfo).in_color_space
                    >=  JCS_EXT_RGB
                    &&  (*cinfo).in_color_space
                        <=  JCS_EXT_ARGB
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if  (*cinfo).in_color_space
                ==  JCS_CMYK
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_rgb_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            need_iobuffer = FALSE
        }
        53 => {
            /* it's a raw-format PGM file */
            if  (*cinfo).in_color_space
                ==  JCS_UNKNOWN
            {
                (*cinfo).in_color_space = JCS_GRAYSCALE
            }
            (*(*cinfo).err).msg_code = JTRC_PGM as c_int;
            (*(*cinfo).err).msg_parm.i[0] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1] = h as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1i32
            );
            if maxval > 255u32 {
                (*source).pub_0.get_pixel_rows = Some(
                    get_word_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if maxval == MAXJSAMPLE as c_uint
                && ::std::mem::size_of::<JSAMPLE>() as c_ulong
                    == ::std::mem::size_of::<U_CHAR>() as c_ulong
                &&  (*cinfo).in_color_space
                    ==  JCS_GRAYSCALE
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_raw_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                );
                use_raw_buffer = TRUE;
                need_rescale = FALSE
            } else if  (*cinfo).in_color_space
                ==  JCS_GRAYSCALE
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_scaled_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if  (*cinfo).in_color_space
                ==  JCS_RGB
                ||  (*cinfo).in_color_space
                    >=  JCS_EXT_RGB
                    &&  (*cinfo).in_color_space
                        <=  JCS_EXT_ARGB
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_gray_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if  (*cinfo).in_color_space
                ==  JCS_CMYK
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_gray_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
        }
        54 => {
            /* it's a raw-format PPM file */
            if  (*cinfo).in_color_space
                ==  JCS_UNKNOWN
            {
                (*cinfo).in_color_space = JCS_EXT_RGB
            }
            (*(*cinfo).err).msg_code = JTRC_PPM as c_int;
            (*(*cinfo).err).msg_parm.i[0] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1] = h as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1i32
            );
            if maxval > 255u32 {
                (*source).pub_0.get_pixel_rows = Some(
                    get_word_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if maxval == MAXJSAMPLE as c_uint
                && ::std::mem::size_of::<JSAMPLE>() as c_ulong
                    == ::std::mem::size_of::<U_CHAR>() as c_ulong
                && ((*cinfo).in_color_space
                    ==  JCS_EXT_RGB
                    ||  (*cinfo).in_color_space
                        ==  JCS_RGB)
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_raw_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                );
                use_raw_buffer = TRUE;
                need_rescale = FALSE
            } else if  (*cinfo).in_color_space
                ==  JCS_RGB
                ||  (*cinfo).in_color_space
                    >=  JCS_EXT_RGB
                    &&  (*cinfo).in_color_space
                        <=  JCS_EXT_ARGB
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else if  (*cinfo).in_color_space
                ==  JCS_CMYK
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_rgb_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code =
                    super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
        }
        _ => {}
    }
    if  (*cinfo).in_color_space
        ==  JCS_RGB
        ||  (*cinfo).in_color_space
            >=  JCS_EXT_RGB
            &&  (*cinfo).in_color_space
                <=  JCS_EXT_ARGB
    {
        (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
    } else if  (*cinfo).in_color_space
        ==  JCS_GRAYSCALE
    {
        (*cinfo).input_components = 1i32
    } else if  (*cinfo).in_color_space
        ==  JCS_CMYK
    {
        (*cinfo).input_components = 4i32
    }
    /* Allocate space for I/O buffer: 1 or 3 bytes or words/pixel. */
    if need_iobuffer != 0 {
        if c == '6' as i32 {
            (*source).buffer_width = w as size_t * 3u64 *
    if maxval <= 255u32 {
                        ::std::mem::size_of::<U_CHAR>() as c_ulong
                    } else {
                        2u64 * ::std::mem::size_of::<U_CHAR>() as c_ulong
                    }
        } else {
            (*source).buffer_width = w as size_t *
    if maxval <= 255u32 {
                    ::std::mem::size_of::<U_CHAR>() as c_ulong
                } else {
                    2u64 * ::std::mem::size_of::<U_CHAR>() as c_ulong
                }
        }
        (*source).iobuffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*source).buffer_width,
        ) as *mut U_CHAR
    }
    /* Create compressor input buffer. */
    if use_raw_buffer != 0 {
        /* For unscaled raw-input case, we can just map it onto the I/O buffer. */
        /* Synthesize a JSAMPARRAY pointer structure */
        (*source).pixrow =  (*source).iobuffer;
        (*source).pub_0.buffer = &mut (*source).pixrow;
        (*source).pub_0.buffer_height = 1u32
    } else {
        /* Need to translate anyway, so make a separate sample buffer. */
        (*source).pub_0.buffer = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            
            w * (*cinfo).input_components as c_uint,
            1u32,
        );
        (*source).pub_0.buffer_height = 1u32
    }
    /* Compute the rescaling array if required. */
    if need_rescale != 0 {
        
          
        /* On 16-bit-int machines we have to be careful of maxval = 65535 */
        (*source).rescale = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (maxval as c_long + 1i64) as c_ulong *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
        ) as *mut JSAMPLE;
        
         let mut half_maxval:   c_long =  ( maxval / 2u32) as c_long; let mut val:   c_long =  0i64;
        while val <= maxval as c_long {
            /* The multiplication here must be done in 32 bits to avoid overflow */
            *(*source).rescale.offset(val as isize) =
                ((val * MAXJSAMPLE as c_long + half_maxval)
                    / maxval as c_long) as JSAMPLE;
            val += 1
        }
    };
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_input_ppm(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
    /* no work */
}
/*
 * The module selection routine for PPM format input.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_read_ppm(
    mut cinfo: j_compress_ptr,
) -> super::cdjpeg::cjpeg_source_ptr {
     
     let mut source:   ppm_source_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<ppm_source_struct>() as c_ulong,
    ) as ppm_source_ptr;
    /* Fill in method ptrs, except get_pixel_rows which start_input sets */
    (*source).pub_0.start_input = Some(
        start_input_ppm
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_ppm
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    return source as super::cdjpeg::cjpeg_source_ptr;
}
/* PPM_SUPPORTED */
