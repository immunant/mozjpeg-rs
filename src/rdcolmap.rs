





































































































































































































































































use libc::{c_int, c_uint, self};pub use crate::cderror_h::{C2RustUnnamed_4, JERR_BAD_CMAP_FILE,
                           JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH,
                           JERR_BMP_BADHEADER, JERR_BMP_BADPLANES,
                           JERR_BMP_COLORSPACE, JERR_BMP_COMPRESSED,
                           JERR_BMP_EMPTY, JERR_BMP_NOT, JERR_BMP_OUTOFRANGE,
                           JERR_GIF_BUG, JERR_GIF_CODESIZE,
                           JERR_GIF_COLORSPACE, JERR_GIF_IMAGENOTFOUND,
                           JERR_GIF_NOT, JERR_PPM_COLORSPACE,
                           JERR_PPM_NONNUMERIC, JERR_PPM_NOT,
                           JERR_PPM_OUTOFRANGE, JERR_TGA_BADCMAP,
                           JERR_TGA_BADPARMS, JERR_TGA_COLORSPACE,
                           JERR_TOO_MANY_COLORS, JERR_UNGETC_FAILED,
                           JERR_UNKNOWN_FORMAT, JERR_UNSUPPORTED_FORMAT,
                           JMSG_FIRSTADDONCODE, JMSG_LASTADDONCODE, JTRC_BMP,
                           JTRC_BMP_MAPPED, JTRC_BMP_OS2, JTRC_BMP_OS2_MAPPED,
                           JTRC_GIF, JTRC_GIF_BADVERSION, JTRC_GIF_EXTENSION,
                           JTRC_GIF_NONSQUARE, JTRC_PGM, JTRC_PGM_TEXT,
                           JTRC_PPM, JTRC_PPM_TEXT, JTRC_TGA, JTRC_TGA_GRAY,
                           JTRC_TGA_MAPPED, JWRN_GIF_BADDATA, JWRN_GIF_CHAR,
                           JWRN_GIF_ENDCODE, JWRN_GIF_NOMOREDATA};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE, getc, EOF};pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE,
                            MAXJSAMPLE, UINT16, UINT8};pub use crate::stddef_h::size_t;pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_reader,
                           jpeg_marker_struct, jpeg_memory_mgr,
                           jpeg_progress_mgr, jpeg_saved_marker_ptr,
                           jpeg_source_mgr, jpeg_upsampler,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
                           JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB,
                           JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
                           JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
                           JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
                           JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
                           JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
                           JQUANT_TBL, JSAMPARRAY, JSAMPROW, J_COLOR_SPACE,
                           J_DCT_METHOD, J_DITHER_MODE};pub use crate::jconfig_h::BITS_IN_JSAMPLE;pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
                        JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
                        JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID,
                        JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
                        JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE,
                        JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
                        JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION,
                        JERR_BAD_MCU_SIZE, JERR_BAD_PARAM,
                        JERR_BAD_PARAM_VALUE, JERR_BAD_POOL_ID,
                        JERR_BAD_PRECISION, JERR_BAD_PROGRESSION,
                        JERR_BAD_PROG_SCRIPT, JERR_BAD_SAMPLING,
                        JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE,
                        JERR_BAD_STRUCT_SIZE, JERR_BAD_VIRTUAL_ACCESS,
                        JERR_BUFFER_SIZE, JERR_CANT_SUSPEND,
                        JERR_CCIR601_NOTIMPL, JERR_COMPONENT_COUNT,
                        JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX,
                        JERR_DAC_VALUE, JERR_DHT_INDEX, JERR_DQT_INDEX,
                        JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE,
                        JERR_EOI_EXPECTED, JERR_FILE_READ, JERR_FILE_WRITE,
                        JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
                        JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG,
                        JERR_INPUT_EMPTY, JERR_INPUT_EOF,
                        JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA,
                        JERR_MODE_CHANGE, JERR_NOTIMPL, JERR_NOT_COMPILED,
                        JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                        JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                        JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                        JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                        JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                        JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                        JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
                        JERR_TFILE_SEEK, JERR_TFILE_WRITE,
                        JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
                        JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG,
                        JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
                        JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE,
                        JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
                        JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                        JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN, JTRC_EOI,
                        JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE,
                        JTRC_JFIF_EXTENSION, JTRC_JFIF_THUMBNAIL,
                        JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER,
                        JTRC_QUANTVALS, JTRC_QUANT_3_NCOLORS,
                        JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED,
                        JTRC_RECOVERY_ACTION, JTRC_RST, JTRC_SMOOTH_NOTIMPL,
                        JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                        JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE,
                        JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                        JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                        JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                        JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                        JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR,
                        JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
                        JWRN_TOO_MUCH_DATA};
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
     let mut colormap0: JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut colormap1: JSAMPROW = *(*cinfo).colormap.offset(1);
    let mut colormap2: JSAMPROW = *(*cinfo).colormap.offset(2);
    let mut ncolors: c_int = (*cinfo).actual_number_of_colors;
    
     let mut index:   c_int =  0i32;
    while index < ncolors {
        if *colormap0.offset(index as isize) as c_int == R
            && *colormap1.offset(index as isize) as c_int == G
            && *colormap2.offset(index as isize) as c_int == B
        {
            return;
        }
        index += 1
        /* color is already in map */
    }
    /* Check for map overflow. */
    if ncolors >= MAXJSAMPLE + 1i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_MANY_COLORS as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* OK, add color to map. */
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
    
    
    
    
    
     let mut header:  [c_int; 13] =  [0; 13];  
     let mut i:   c_int =  1i32;
    while i < 13i32 {
        header[i as usize] = getc(infile);
        if header[i as usize] == EOF {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        i += 1
    }
    /* Verify GIF Header */
    if header[1] != 'I' as i32 || header[2] != 'F' as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* There must be a global color map. */
    if header[10] & 0x80i32 == 0i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut colormaplen:   c_int =  2i32 << (header[10] & 0x7i32);
    i = 0i32;
    while i < colormaplen {
           
        
         let mut R:   c_int =  getc(infile); let mut G:   c_int =  getc(infile); let mut B:   c_int =  getc(infile);
        if R == EOF || G == EOF || B == EOF {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
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
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
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
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
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
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
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
    return val;
}
/*
 * Extract color map from a PPM file.
 */

unsafe extern "C" fn read_ppm_map(
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) {
    
    
    
    
    
    
    
    
         let mut row:  c_uint =  0; let mut col:  c_uint =  0; let mut R:  c_int =  0; let mut G:  c_int =  0; let mut B:  c_int =  0;
     /* save format discriminator for a sec */
    
    
     let mut c:   c_int =  getc(infile); let mut w:   c_uint =  read_pbm_integer(cinfo, infile); let mut h:   c_uint =  read_pbm_integer(cinfo, infile); let mut maxval:   c_uint =  read_pbm_integer(cinfo, infile);
    if w <= 0u32 || h <= 0u32 || maxval <= 0u32 {
        /* error check */
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For now, we don't support rescaling from an unusual maxval. */
    if maxval != MAXJSAMPLE as c_uint {
        (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    match c {
        51 => {
            /* it's a text-format PPM file */
            row = 0u32;
            while row < h {
                col = 0u32;
                while col < w {
                    R = read_pbm_integer(cinfo, infile) as c_int;
                    G = read_pbm_integer(cinfo, infile) as c_int;
                    B = read_pbm_integer(cinfo, infile) as c_int;
                    add_map_entry(cinfo, R, G, B);
                    col +=  1
                }
                row +=  1
            }
        }
        54 => {
            /* it's a raw-format PPM file */
            row = 0u32;
            while row < h {
                col = 0u32;
                while col < w {
                    R = getc(infile);
                    G = getc(infile);
                    B = getc(infile);
                    if R == EOF || G == EOF || B == EOF
                    {
                        (*(*cinfo).err).msg_code =
                            JERR_BAD_CMAP_FILE as c_int;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                        );
                    }
                    add_map_entry(cinfo, R, G, B);
                    col +=  1
                }
                row +=  1
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
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
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) {
    /* Allocate space for a color map of maximum supported size. */
    (*cinfo).colormap = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1i32) as JDIMENSION,
        3u32,
    ); /* initialize map to empty */
    (*cinfo).actual_number_of_colors = 0i32;
    /* Read first byte to determine file format */
    match getc(infile) {
        71 => {
            read_gif_map(cinfo, infile);
        }
        80 => {
            read_ppm_map(cinfo, infile);
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_CMAP_FILE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
/* QUANT_2PASS_SUPPORTED */
