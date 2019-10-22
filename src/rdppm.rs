use libc::{self, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:25"]
pub mod jmorecfg_h {

    use crate::jmorecfg_h::{
        EXT_BGRX_PIXELSIZE, EXT_BGR_PIXELSIZE, EXT_RGBX_PIXELSIZE, EXT_RGB_PIXELSIZE,
        EXT_XBGR_PIXELSIZE, EXT_XRGB_PIXELSIZE, RGB_PIXELSIZE,
    };
    use libc::c_int;
    pub static mut rgb_pixelsize: [c_int; 17] = [
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

pub use super::cdjpeg::{cjpeg_source_ptr, cjpeg_source_struct};
pub use super::jerror::{
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
pub use crate::cderror_h::{
    C2RustUnnamed_4, JERR_BAD_CMAP_FILE, JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH, JERR_BMP_BADHEADER,
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
pub use crate::cmyk_h::rgb_to_cmyk;
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::{
    boolean, rgb_blue, rgb_green, rgb_red, EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE,
    EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_RGBX_BLUE,
    EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN,
    EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE,
    EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, FALSE, JCOEF,
    JDIMENSION, JOCTET, JSAMPLE, MAXJSAMPLE, RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED, TRUE,
    UINT16, UINT8,
};
pub use crate::jpegint_h::{
    JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_struct, jpeg_marker_writer,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_scan_info,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE,
    JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
};
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, fread, getc, EOF, FILE,
    _IO_FILE,
};
pub use jmorecfg_h::rgb_pixelsize;

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
    let mut ch: c_int = 0;
    ch = getc(infile);
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
    let mut ch: c_int = 0;
    let mut val: c_uint = 0;
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
            .expect("non-null function pointer")(cinfo as j_common_ptr);
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
    val = (ch - '0' as i32) as c_uint;
    loop {
        ch = pbm_getc(infile);
        if !(ch >= '0' as i32 && ch <= '9' as i32) {
            break;
        }
        val = val.wrapping_mul(10i32 as c_uint);
        val = val.wrapping_add((ch - '0' as i32) as c_uint)
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
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    ptr = *(*source).pub_0.buffer.offset(0);
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_text_gray_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PGM files with any maxval and
   converting to extended RGB */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    ptr = *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let ref mut fresh1 = *ptr.offset(bindex as isize);
                *fresh1 = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                let ref mut fresh2 = *ptr.offset(gindex as isize);
                *fresh2 = *fresh1;
                *ptr.offset(rindex as isize) = *fresh2;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let ref mut fresh3 = *ptr.offset(bindex as isize);
                *fresh3 = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                let ref mut fresh4 = *ptr.offset(gindex as isize);
                *fresh4 = *fresh3;
                *ptr.offset(rindex as isize) = *fresh4;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let ref mut fresh5 = *ptr.offset(bindex as isize);
            *fresh5 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let ref mut fresh6 = *ptr.offset(gindex as isize);
            *fresh6 = *fresh5;
            *ptr.offset(rindex as isize) = *fresh6;
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let ref mut fresh7 = *ptr.offset(bindex as isize);
            *fresh7 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let ref mut fresh8 = *ptr.offset(gindex as isize);
            *fresh8 = *fresh7;
            *ptr.offset(rindex as isize) = *fresh8;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_text_gray_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PGM files with any maxval and
   converting to CMYK */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    ptr = *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let mut gray: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
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
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
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
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_text_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PPM files with any maxval */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    ptr = *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                *ptr.offset(rindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(gindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(bindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                *ptr.offset(rindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(gindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(bindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            *ptr.offset(rindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(gindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(bindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            *ptr.offset(rindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(gindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(bindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_text_rgb_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading text-format PPM files with any maxval and
   converting to CMYK */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    ptr = *(*source).pub_0.buffer.offset(0);
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let mut r: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            let mut g: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            let mut b: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            rgb_to_cmyk(r, g, b, ptr, ptr.offset(1), ptr.offset(2), ptr.offset(3));
            ptr = ptr.offset(4);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
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
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_scaled_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PGM files with any maxval */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
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
    ptr = *(*source).pub_0.buffer.offset(0);
    bufferptr = (*source).iobuffer;
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let fresh9 = bufferptr;
        bufferptr = bufferptr.offset(1);
        let fresh10 = ptr;
        ptr = ptr.offset(1);
        *fresh10 = *rescale.offset(*fresh9 as c_int as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_gray_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PGM files with any maxval
   and converting to extended RGB */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
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
    ptr = *(*source).pub_0.buffer.offset(0);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh11 = bufferptr;
                bufferptr = bufferptr.offset(1);
                let ref mut fresh12 = *ptr.offset(bindex as isize);
                *fresh12 = *fresh11;
                let ref mut fresh13 = *ptr.offset(gindex as isize);
                *fresh13 = *fresh12;
                *ptr.offset(rindex as isize) = *fresh13;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh14 = bufferptr;
                bufferptr = bufferptr.offset(1);
                let ref mut fresh15 = *ptr.offset(bindex as isize);
                *fresh15 = *fresh14;
                let ref mut fresh16 = *ptr.offset(gindex as isize);
                *fresh16 = *fresh15;
                *ptr.offset(rindex as isize) = *fresh16;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh17 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let ref mut fresh18 = *ptr.offset(bindex as isize);
            *fresh18 = *rescale.offset(*fresh17 as c_int as isize);
            let ref mut fresh19 = *ptr.offset(gindex as isize);
            *fresh19 = *fresh18;
            *ptr.offset(rindex as isize) = *fresh19;
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh20 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let ref mut fresh21 = *ptr.offset(bindex as isize);
            *fresh21 = *rescale.offset(*fresh20 as c_int as isize);
            let ref mut fresh22 = *ptr.offset(gindex as isize);
            *fresh22 = *fresh21;
            *ptr.offset(rindex as isize) = *fresh22;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_gray_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PGM files with any maxval
   and converting to CMYK */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
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
    ptr = *(*source).pub_0.buffer.offset(0);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
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
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh24 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut gray_0: JSAMPLE = *rescale.offset(*fresh24 as c_int as isize);
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
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PPM files with any maxval */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
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
    ptr = *(*source).pub_0.buffer.offset(0);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh25 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(rindex as isize) = *fresh25;
                let fresh26 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(gindex as isize) = *fresh26;
                let fresh27 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(bindex as isize) = *fresh27;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
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
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh31 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(rindex as isize) = *rescale.offset(*fresh31 as c_int as isize);
            let fresh32 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(gindex as isize) = *rescale.offset(*fresh32 as c_int as isize);
            let fresh33 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(bindex as isize) = *rescale.offset(*fresh33 as c_int as isize);
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
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
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_rgb_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-byte-format PPM files with any maxval and
   converting to CMYK */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
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
    ptr = *(*source).pub_0.buffer.offset(0);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
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
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh40 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut r_0: JSAMPLE = *rescale.offset(*fresh40 as c_int as isize);
            let fresh41 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut g_0: JSAMPLE = *rescale.offset(*fresh41 as c_int as isize);
            let fresh42 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut b_0: JSAMPLE = *rescale.offset(*fresh42 as c_int as isize);
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
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
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
        1i32 as size_t,
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
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_word_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-word-format PGM files with any maxval */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
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
    ptr = *(*source).pub_0.buffer.offset(0);
    bufferptr = (*source).iobuffer;
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let mut temp: c_uint = 0;
        let fresh43 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh43 as c_int) << 8i32) as c_uint;
        let fresh44 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh44 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh45 = ptr;
        ptr = ptr.offset(1);
        *fresh45 = *rescale.offset(temp as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}

unsafe extern "C" fn get_word_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading raw-word-format PPM files with any maxval */ {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
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
    ptr = *(*source).pub_0.buffer.offset(0);
    bufferptr = (*source).iobuffer;
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let mut temp: c_uint = 0;
        let fresh46 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh46 as c_int) << 8i32) as c_uint;
        let fresh47 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh47 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh48 = ptr;
        ptr = ptr.offset(1);
        *fresh48 = *rescale.offset(temp as isize);
        let fresh49 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh49 as c_int) << 8i32) as c_uint;
        let fresh50 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh50 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh51 = ptr;
        ptr = ptr.offset(1);
        *fresh51 = *rescale.offset(temp as isize);
        let fresh52 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh52 as c_int) << 8i32) as c_uint;
        let fresh53 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh53 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh54 = ptr;
        ptr = ptr.offset(1);
        *fresh54 = *rescale.offset(temp as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
/*
 * Read the file header; return image size and component count.
 */

unsafe extern "C" fn start_input_ppm(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr; /* subformat discriminator character */
    let mut c: c_int = 0;
    let mut w: c_uint = 0;
    let mut h: c_uint = 0;
    let mut maxval: c_uint = 0;
    let mut need_iobuffer: boolean = 0;
    let mut use_raw_buffer: boolean = 0;
    let mut need_rescale: boolean = 0;
    if getc((*source).pub_0.input_file) != 'P' as i32 {
        (*(*cinfo).err).msg_code = JERR_PPM_NOT as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    c = getc((*source).pub_0.input_file);
    let mut current_block_3: u64;
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
            .expect("non-null function pointer")(cinfo as j_common_ptr);
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
    /* fetch the remaining header info */
    w = read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535i32 as c_uint);
    h = read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535i32 as c_uint);
    maxval = read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535i32 as c_uint);
    if w <= 0i32 as c_uint || h <= 0i32 as c_uint || maxval <= 0i32 as c_uint {
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
    /* initialize flags to most common settings */
    need_iobuffer = TRUE; /* do we need an I/O buffer? */
    use_raw_buffer = FALSE; /* do we map input buffer onto I/O buffer? */
    need_rescale = TRUE; /* do we need a rescale array? */
    match c {
        50 => {
            /* it's a text-format PGM file */
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
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
            .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            need_iobuffer = FALSE
        }
        51 => {
            /* it's a text-format PPM file */
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
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
            .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_rgb_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            need_iobuffer = FALSE
        }
        53 => {
            /* it's a raw-format PGM file */
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
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
            .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if maxval > 255i32 as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_word_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if maxval == MAXJSAMPLE as c_uint
                && ::std::mem::size_of::<JSAMPLE>() as c_ulong
                    == ::std::mem::size_of::<U_CHAR>() as c_ulong
                && (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_raw_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                );
                use_raw_buffer = TRUE;
                need_rescale = FALSE
            } else if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_scaled_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_gray_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_gray_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        54 => {
            /* it's a raw-format PPM file */
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
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
            .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if maxval > 255i32 as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_word_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if maxval == MAXJSAMPLE as c_uint
                && ::std::mem::size_of::<JSAMPLE>() as c_ulong
                    == ::std::mem::size_of::<U_CHAR>() as c_ulong
                && ((*cinfo).in_color_space as c_uint == JCS_EXT_RGB as c_int as c_uint
                    || (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint)
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_raw_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                );
                use_raw_buffer = TRUE;
                need_rescale = FALSE
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_rgb_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {}
    }
    if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
        || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
            && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
    {
        (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
    } else if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
        (*cinfo).input_components = 1i32
    } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
        (*cinfo).input_components = 4i32
    }
    /* Allocate space for I/O buffer: 1 or 3 bytes or words/pixel. */
    if need_iobuffer != 0 {
        if c == '6' as i32 {
            (*source).buffer_width = (w as size_t).wrapping_mul(3i32 as c_ulong).wrapping_mul(
                (if maxval <= 255i32 as c_uint {
                    ::std::mem::size_of::<U_CHAR>() as c_ulong
                } else {
                    (2i32 as c_ulong).wrapping_mul(::std::mem::size_of::<U_CHAR>() as c_ulong)
                }),
            )
        } else {
            (*source).buffer_width = (w as size_t).wrapping_mul(
                (if maxval <= 255i32 as c_uint {
                    ::std::mem::size_of::<U_CHAR>() as c_ulong
                } else {
                    (2i32 as c_ulong).wrapping_mul(::std::mem::size_of::<U_CHAR>() as c_ulong)
                }),
            )
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
        (*source).pixrow = (*source).iobuffer as JSAMPROW;
        (*source).pub_0.buffer = &mut (*source).pixrow;
        (*source).pub_0.buffer_height = 1i32 as JDIMENSION
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
            w.wrapping_mul((*cinfo).input_components as c_uint),
            1i32 as JDIMENSION,
        );
        (*source).pub_0.buffer_height = 1i32 as JDIMENSION
    }
    /* Compute the rescaling array if required. */
    if need_rescale != 0 {
        let mut val: c_long = 0;
        let mut half_maxval: c_long = 0;
        /* On 16-bit-int machines we have to be careful of maxval = 65535 */
        (*source).rescale = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((maxval as c_long + 1i64) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
        ) as *mut JSAMPLE;
        half_maxval = maxval.wrapping_div(2i32 as c_uint) as c_long;
        val = 0i32 as c_long;
        while val <= maxval as c_long {
            /* The multiplication here must be done in 32 bits to avoid overflow */
            *(*source).rescale.offset(val as isize) =
                ((val * MAXJSAMPLE as c_long + half_maxval) / maxval as c_long) as JSAMPLE;
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
    let mut source: ppm_source_ptr = 0 as *mut ppm_source_struct;
    /* Create module interface object */
    source = Some(
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
            as unsafe extern "C" fn(_: j_compress_ptr, _: super::cdjpeg::cjpeg_source_ptr) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_ppm
            as unsafe extern "C" fn(_: j_compress_ptr, _: super::cdjpeg::cjpeg_source_ptr) -> (),
    );
    return source as super::cdjpeg::cjpeg_source_ptr;
}
/* PPM_SUPPORTED */
