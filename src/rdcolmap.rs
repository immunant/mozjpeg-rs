use libc;

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
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
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
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::jerror::C2RustUnnamed_3;
pub use crate::src::jerror::JERR_ARITH_NOTIMPL;
pub use crate::src::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_LENGTH;
pub use crate::src::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jerror::JERR_BAD_PARAM;
pub use crate::src::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::src::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jerror::JERR_BAD_PRECISION;
pub use crate::src::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jerror::JERR_BAD_STATE;
pub use crate::src::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jerror::JERR_DAC_INDEX;
pub use crate::src::jerror::JERR_DAC_VALUE;
pub use crate::src::jerror::JERR_DHT_INDEX;
pub use crate::src::jerror::JERR_DQT_INDEX;
pub use crate::src::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jerror::JERR_EMS_READ;
pub use crate::src::jerror::JERR_EMS_WRITE;
pub use crate::src::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jerror::JERR_FILE_READ;
pub use crate::src::jerror::JERR_FILE_WRITE;
pub use crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jerror::JERR_INPUT_EOF;
pub use crate::src::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jerror::JERR_MISSING_DATA;
pub use crate::src::jerror::JERR_MODE_CHANGE;
pub use crate::src::jerror::JERR_NOTIMPL;
pub use crate::src::jerror::JERR_NOT_COMPILED;
pub use crate::src::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jerror::JERR_NO_IMAGE;
pub use crate::src::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jerror::JERR_NO_SOI;
pub use crate::src::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jerror::JERR_TFILE_CREATE;
pub use crate::src::jerror::JERR_TFILE_READ;
pub use crate::src::jerror::JERR_TFILE_SEEK;
pub use crate::src::jerror::JERR_TFILE_WRITE;
pub use crate::src::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::src::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jerror::JERR_XMS_READ;
pub use crate::src::jerror::JERR_XMS_WRITE;
pub use crate::src::jerror::JMSG_COPYRIGHT;
pub use crate::src::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jerror::JMSG_NOMESSAGE;
pub use crate::src::jerror::JMSG_VERSION;
pub use crate::src::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jerror::JTRC_ADOBE;
pub use crate::src::jerror::JTRC_APP0;
pub use crate::src::jerror::JTRC_APP14;
pub use crate::src::jerror::JTRC_DAC;
pub use crate::src::jerror::JTRC_DHT;
pub use crate::src::jerror::JTRC_DQT;
pub use crate::src::jerror::JTRC_DRI;
pub use crate::src::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jerror::JTRC_EMS_OPEN;
pub use crate::src::jerror::JTRC_EOI;
pub use crate::src::jerror::JTRC_HUFFBITS;
pub use crate::src::jerror::JTRC_JFIF;
pub use crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jerror::JTRC_MISC_MARKER;
pub use crate::src::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jerror::JTRC_QUANTVALS;
pub use crate::src::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jerror::JTRC_RST;
pub use crate::src::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jerror::JTRC_SOF;
pub use crate::src::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jerror::JTRC_SOI;
pub use crate::src::jerror::JTRC_SOS;
pub use crate::src::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jerror::JTRC_THUMB_RGB;
pub use crate::src::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jerror::JTRC_XMS_OPEN;
pub use crate::src::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jerror::JWRN_BOGUS_ICC;
pub use crate::src::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jerror::JWRN_HIT_MARKER;
pub use crate::src::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jerror::JWRN_JPEG_EOF;
pub use crate::src::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::stdlib::getc;
pub use crate::stdlib::EOF;
/*
 * rdcolmap.c
 *
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file implements djpeg's "-map file" switch.  It reads a source image
 * and constructs a colormap to be supplied to the JPEG decompressor.
 *
 * Currently, these file formats are supported for the map file:
 *   GIF: the contents of the GIF's global colormap are used.
 *   PPM (either text or raw flavor): the entire file is read and
 *      each unique pixel value is entered in the map.
 * Note that reading a large PPM file will be horrendously slow.
 * Typically, a PPM-format map file should contain just one pixel
 * of each desired color.  Such a file can be extracted from an
 * ordinary image PPM file with ppmtomap(1).
 *
 * Rescaling a PPM that has a maxval unequal to MAXJSAMPLE is not
 * currently implemented.
 */
/* otherwise can't quantize to supplied map */
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
/*
 * Add a (potentially) new color to the color map.
 */

unsafe extern "C" fn add_map_entry(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut R: libc::c_int,
    mut G: libc::c_int,
    mut B: libc::c_int,
) {
    let mut colormap0: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut colormap1: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(1);
    let mut colormap2: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(2);
    let mut ncolors: libc::c_int = (*cinfo).actual_number_of_colors;
    let mut index: libc::c_int = 0;
    /* Check for duplicate color. */
    index = 0i32;
    while index < ncolors {
        if *colormap0.offset(index as isize) as libc::c_int == R
            && *colormap1.offset(index as isize) as libc::c_int == G
            && *colormap2.offset(index as isize) as libc::c_int == B
        {
            return;
        }
        index += 1
        /* color is already in map */
    }
    /* Check for map overflow. */
    if ncolors >= crate::jmorecfg_h::MAXJSAMPLE + 1i32 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_MANY_COLORS as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* OK, add color to map. */
    *colormap0.offset(ncolors as isize) = R as crate::jmorecfg_h::JSAMPLE;
    *colormap1.offset(ncolors as isize) = G as crate::jmorecfg_h::JSAMPLE;
    *colormap2.offset(ncolors as isize) = B as crate::jmorecfg_h::JSAMPLE;
    (*cinfo).actual_number_of_colors += 1;
}
/*
 * Extract color map from a GIF file.
 */

unsafe extern "C" fn read_gif_map(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut infile: *mut crate::stdlib::FILE,
) {
    let mut header: [libc::c_int; 13] = [0; 13];
    let mut i: libc::c_int = 0;
    let mut colormaplen: libc::c_int = 0;
    let mut R: libc::c_int = 0;
    let mut G: libc::c_int = 0;
    let mut B: libc::c_int = 0;
    /* Initial 'G' has already been read by read_color_map */
    /* Read the rest of the GIF header and logical screen descriptor */
    i = 1i32;
    while i < 13i32 {
        header[i as usize] = crate::stdlib::getc(infile);
        if header[i as usize] == crate::stdlib::EOF {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        i += 1
    }
    /* Verify GIF Header */
    if header[1] != 'I' as i32 || header[2] != 'F' as i32 {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* There must be a global color map. */
    if header[10] & 0x80i32 == 0i32 {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* OK, fetch it. */
    colormaplen = 2i32 << (header[10] & 0x7i32);
    i = 0i32;
    while i < colormaplen {
        R = crate::stdlib::getc(infile);
        G = crate::stdlib::getc(infile);
        B = crate::stdlib::getc(infile);
        if R == crate::stdlib::EOF || G == crate::stdlib::EOF || B == crate::stdlib::EOF {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        add_map_entry(
            cinfo,
            R << crate::jconfig_h::BITS_IN_JSAMPLE - 8i32,
            G << crate::jconfig_h::BITS_IN_JSAMPLE - 8i32,
            B << crate::jconfig_h::BITS_IN_JSAMPLE - 8i32,
        );
        i += 1
    }
}
/* Support routines for reading PPM */

unsafe extern "C" fn pbm_getc(mut infile: *mut crate::stdlib::FILE) -> libc::c_int
/* Read next char, skipping over any comments */
/* A comment/newline sequence is returned as a newline */ {
    let mut ch: libc::c_int = 0;
    ch = crate::stdlib::getc(infile);
    if ch == '#' as i32 {
        loop {
            ch = crate::stdlib::getc(infile);
            if !(ch != '\n' as i32 && ch != crate::stdlib::EOF) {
                break;
            }
        }
    }
    return ch;
}

unsafe extern "C" fn read_pbm_integer(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut infile: *mut crate::stdlib::FILE,
) -> libc::c_uint
/* Read an unsigned decimal integer from the PPM file */
/* Swallows one trailing character after the integer */
/* Note that on a 16-bit-int machine, only values up to 64k can be read. */
/* This should not be a problem in practice. */ {
    let mut ch: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    loop
    /* Skip any leading whitespace */
    {
        ch = pbm_getc(infile);
        if ch == crate::stdlib::EOF {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
            break;
        }
    }
    if ch < '0' as i32 || ch > '9' as i32 {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    val = (ch - '0' as i32) as libc::c_uint;
    loop {
        ch = pbm_getc(infile);
        if !(ch >= '0' as i32 && ch <= '9' as i32) {
            break;
        }
        val =  val * 10u32;
        val =  val + (ch - '0' as i32) as libc::c_uint
    }
    return val;
}
/*
 * Extract color map from a PPM file.
 */

unsafe extern "C" fn read_ppm_map(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut infile: *mut crate::stdlib::FILE,
) {
    let mut c: libc::c_int = 0;
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    let mut maxval: libc::c_uint = 0;
    let mut row: libc::c_uint = 0;
    let mut col: libc::c_uint = 0;
    let mut R: libc::c_int = 0;
    let mut G: libc::c_int = 0;
    let mut B: libc::c_int = 0;
    /* Initial 'P' has already been read by read_color_map */
    c = crate::stdlib::getc(infile); /* save format discriminator for a sec */
    /* while we fetch the remaining header info */
    w = read_pbm_integer(cinfo, infile);
    h = read_pbm_integer(cinfo, infile);
    maxval = read_pbm_integer(cinfo, infile);
    if w <= 0u32 || h <= 0u32 || maxval <= 0u32 {
        /* error check */
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For now, we don't support rescaling from an unusual maxval. */
    if maxval != crate::jmorecfg_h::MAXJSAMPLE as libc::c_uint {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    match c {
        51 => {
            /* it's a text-format PPM file */
            row = 0u32;
            while row < h {
                col = 0u32;
                while col < w {
                    R = read_pbm_integer(cinfo, infile) as libc::c_int;
                    G = read_pbm_integer(cinfo, infile) as libc::c_int;
                    B = read_pbm_integer(cinfo, infile) as libc::c_int;
                    add_map_entry(cinfo, R, G, B);
                    col =  col + 1
                }
                row =  row + 1
            }
        }
        54 => {
            /* it's a raw-format PPM file */
            row = 0u32;
            while row < h {
                col = 0u32;
                while col < w {
                    R = crate::stdlib::getc(infile);
                    G = crate::stdlib::getc(infile);
                    B = crate::stdlib::getc(infile);
                    if R == crate::stdlib::EOF || G == crate::stdlib::EOF || B == crate::stdlib::EOF
                    {
                        (*(*cinfo).err).msg_code =
                            crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                        );
                    }
                    add_map_entry(cinfo, R, G, B);
                    col =  col + 1
                }
                row =  row + 1
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
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
/*
 * Main entry point from djpeg.c.
 *  Input: opened input file (from file name argument on command line).
 *  Output: colormap and actual_number_of_colors fields are set in cinfo.
 */
#[no_mangle]

pub unsafe extern "C" fn read_color_map(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut infile: *mut crate::stdlib::FILE,
) {
    /* Allocate space for a color map of maximum supported size. */
    (*cinfo).colormap = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (crate::jmorecfg_h::MAXJSAMPLE + 1i32) as crate::jmorecfg_h::JDIMENSION,
        3u32,
    ); /* initialize map to empty */
    (*cinfo).actual_number_of_colors = 0i32;
    /* Read first byte to determine file format */
    match crate::stdlib::getc(infile) {
        71 => {
            read_gif_map(cinfo, infile);
        }
        80 => {
            read_ppm_map(cinfo, infile);
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BAD_CMAP_FILE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}
/* QUANT_2PASS_SUPPORTED */
