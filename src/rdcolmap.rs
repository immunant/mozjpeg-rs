use libc::c_int;use libc::c_uint;pub use crate::cderror_h::C2RustUnnamed_91;
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
pub use crate::jerror::C2RustUnnamed_3;
pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
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
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::getc;
pub use crate::stdlib::EOF;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
use libc;
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
    mut cinfo: j_decompress_ptr,
    mut R: c_int,
    mut G: c_int,
    mut B: c_int,
) {
    let mut colormap0: JSAMPROW = *(*cinfo).colormap.offset(0isize);
    let mut colormap1: JSAMPROW = *(*cinfo).colormap.offset(1isize);
    let mut colormap2: JSAMPROW = *(*cinfo).colormap.offset(2isize);
    let mut ncolors: c_int = (*cinfo).actual_number_of_colors;
    let mut index: c_int = 0;
    index = 0i32;
    while index < ncolors {
        if *colormap0.offset(index as isize) as c_int == R
            && *colormap1.offset(index as isize) as c_int == G
            && *colormap2.offset(index as isize) as c_int == B
        {
            return;
        }
        index += 1
    }
    if ncolors >= MAXJSAMPLE + 1i32 {
        (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = 255i32 + 1i32;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    *colormap0.offset(ncolors as isize) = R as JSAMPLE;
    *colormap1.offset(ncolors as isize) = G as JSAMPLE;
    *colormap2.offset(ncolors as isize) = B as JSAMPLE;
    (*cinfo).actual_number_of_colors += 1;
}
/*
 * Extract color map from a GIF file.
 */
unsafe extern "C" fn read_gif_map(
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) {
    let mut header: [c_int; 13] = [0; 13];
    let mut i: c_int = 0;
    let mut colormaplen: c_int = 0;
    let mut R: c_int = 0;
    let mut G: c_int = 0;
    let mut B: c_int = 0;
    i = 1i32;
    while i < 13i32 {
        header[i as usize] = getc(infile);
        if header[i as usize] == EOF {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        i += 1
    }
    if header[1usize] != 'I' as i32 || header[2usize] != 'F' as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if header[10usize] & 0x80i32 == 0i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    colormaplen = 2i32 << (header[10usize] & 0x7i32);
    i = 0i32;
    while i < colormaplen {
        R = getc(infile);
        G = getc(infile);
        B = getc(infile);
        if R == EOF || G == EOF || B == EOF {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        add_map_entry(
            cinfo,
            R << BITS_IN_JSAMPLE - 8i32,
            G << BITS_IN_JSAMPLE - 8i32,
            B << BITS_IN_JSAMPLE - 8i32,
        );
        i += 1
    }
}
/* Support routines for reading PPM */
unsafe extern "C" fn pbm_getc(mut infile: *mut FILE) -> c_int {
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
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) -> c_uint {
    let mut ch: c_int = 0;
    let mut val: c_uint = 0;
    loop {
        ch = pbm_getc(infile);
        if ch == EOF {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
            break;
        }
    }
    if ch < '0' as i32 || ch > '9' as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        (*(*cinfo).err)
            .error_exit
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
    return val;
}
/*
 * Extract color map from a PPM file.
 */
unsafe extern "C" fn read_ppm_map(
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) {
    let mut c: c_int = 0;
    let mut w: c_uint = 0;
    let mut h: c_uint = 0;
    let mut maxval: c_uint = 0;
    let mut row: c_uint = 0;
    let mut col: c_uint = 0;
    let mut R: c_int = 0;
    let mut G: c_int = 0;
    let mut B: c_int = 0;
    c = getc(infile);
    w = read_pbm_integer(cinfo, infile);
    h = read_pbm_integer(cinfo, infile);
    maxval = read_pbm_integer(cinfo, infile);
    if w <= 0i32 as c_uint || h <= 0i32 as c_uint || maxval <= 0i32 as c_uint {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if maxval != MAXJSAMPLE as c_uint {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    match c {
        51 => {
            row = 0i32 as c_uint;
            while row < h {
                col = 0i32 as c_uint;
                while col < w {
                    R = read_pbm_integer(cinfo, infile) as c_int;
                    G = read_pbm_integer(cinfo, infile) as c_int;
                    B = read_pbm_integer(cinfo, infile) as c_int;
                    add_map_entry(cinfo, R, G, B);
                    col = col.wrapping_add(1)
                }
                row = row.wrapping_add(1)
            }
        }
        54 => {
            row = 0i32 as c_uint;
            while row < h {
                col = 0i32 as c_uint;
                while col < w {
                    R = getc(infile);
                    G = getc(infile);
                    B = getc(infile);
                    if R == EOF || G == EOF || B == EOF
                    {
                        (*(*cinfo).err).msg_code =
                            JERR_BAD_CMAP_FILE as c_int;
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                        );
                    }
                    add_map_entry(cinfo, R, G, B);
                    col = col.wrapping_add(1)
                }
                row = row.wrapping_add(1)
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
/* djpeg support routines (in rdcolmap.c) */
/*
 * Main entry point from djpeg.c.
 *  Input: opened input file (from file name argument on command line).
 *  Output: colormap and actual_number_of_colors fields are set in cinfo.
 */
#[no_mangle]
pub unsafe extern "C" fn read_color_map(
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) {
    (*cinfo).colormap = (*(*cinfo).mem)
        .alloc_sarray
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1i32) as JDIMENSION,
        3i32 as JDIMENSION,
    );
    (*cinfo).actual_number_of_colors = 0i32;
    match getc(infile) {
        71 => {
            read_gif_map(cinfo, infile);
        }
        80 => {
            read_ppm_map(cinfo, infile);
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
