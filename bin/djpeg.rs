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


use mozjpeg::*;


pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
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
pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_CreateDecompress;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_crop_scanline;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destroy_decompress;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_finish_decompress;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_mem_src;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_read_header;
pub use crate::jpeglib_h::jpeg_read_icc_profile;
pub use crate::jpeglib_h::jpeg_read_scanlines;
pub use crate::jpeglib_h::jpeg_save_markers;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_set_marker_processor;
pub use crate::jpeglib_h::jpeg_skip_scanlines;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_start_decompress;
pub use crate::jpeglib_h::jpeg_std_error;
pub use crate::jpeglib_h::jpeg_stdio_src;
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
pub use crate::jpeglib_h::JDCT_DEFAULT;
pub use crate::jpeglib_h::JDCT_FASTEST;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JPEG_COM;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::cdjpeg::djpeg_dest_ptr;
pub use crate::src::cdjpeg::djpeg_dest_struct;
pub use crate::src::cdjpeg::jinit_write_bmp;
pub use crate::src::cdjpeg::jinit_write_gif;
pub use crate::src::cdjpeg::jinit_write_ppm;
pub use crate::src::cdjpeg::jinit_write_targa;
pub use crate::src::cdjpeg::keymatch;
pub use crate::src::cdjpeg::read_color_map;
pub use crate::src::cdjpeg::read_stdin;
pub use crate::src::cdjpeg::write_stdout;
pub use crate::src::cdjpeg::EXIT_WARNING;
pub use crate::src::cdjpeg::READ_BINARY;
pub use crate::src::cdjpeg::WRITE_BINARY;
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
pub use crate::stdlib::exit;
use crate::stdlib::fclose;
use crate::stdlib::ferror;
use crate::stdlib::fopen;
use crate::stdlib::fprintf;
use crate::stdlib::fread;
pub use crate::stdlib::free;
use crate::stdlib::fwrite;
use crate::stdlib::putc;
pub use crate::stdlib::realloc;
use crate::stdlib::sscanf;
use crate::stdlib::stderr;
use crate::stdlib::stdin;
use crate::stdlib::stdout;
pub use crate::stdlib::EXIT_FAILURE;
pub use crate::stdlib::EXIT_SUCCESS;

pub use crate::jconfigint_h::BUILD;
pub use crate::jconfigint_h::PACKAGE_NAME;
pub use crate::jconfigint_h::VERSION;
pub use crate::jversion_h::JCOPYRIGHT;
pub use crate::jversion_h::JVERSION;

pub type IMAGE_FORMATS = libc::c_uint;

pub const FMT_TIFF: IMAGE_FORMATS = 6;

pub const FMT_TARGA: IMAGE_FORMATS = 5;

pub const FMT_RLE: IMAGE_FORMATS = 4;

pub const FMT_PPM: IMAGE_FORMATS = 3;

pub const FMT_OS2: IMAGE_FORMATS = 2;

pub const FMT_GIF: IMAGE_FORMATS = 1;

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

static mut progname: *const libc::c_char = 0 as *const libc::c_char;
/* program name for error messages */

static mut icc_filename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* for -icc switch */

static mut outfilename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* for -outfile switch */
#[no_mangle]

pub static mut memsrc: crate::jmorecfg_h::boolean = 0;
/* for -memsrc switch */
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

unsafe extern "C" fn usage()
/* complain about bad command line */
{
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
) -> libc::c_int
/* Parse optional switches.
 * Returns argv[] index of first file-name argument (== argc if none).
 * Any file names with indexes <= last_file_arg_seen are ignored;
 * they have presumably been processed in a previous iteration.
 * (Pass 0 for last_file_arg_seen on the first or only iteration.)
 * for_real is FALSE on the first (dummy) pass; we may skip any expensive
 * processing.
 */ {
    let mut argn: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Set up default JPEG parameters. */
    requested_fmt = DEFAULT_FMT as IMAGE_FORMATS; /* set default output file format */
    icc_filename = crate::stddef_h::NULL as *mut libc::c_char;
    outfilename = crate::stddef_h::NULL as *mut libc::c_char;
    memsrc = crate::jmorecfg_h::FALSE;
    skip = crate::jmorecfg_h::FALSE;
    crop = crate::jmorecfg_h::FALSE;
    (*(*cinfo).err).trace_level = 0i32;
    /* Scan command line options, adjust parameters */
    argn = 1i32;
    while argn < argc {
        arg = *argv.offset(argn as isize);
        if *arg as libc::c_int != '-' as i32 {
            /* Not a switch, must be a file name argument */
            if !(argn <= last_file_arg_seen) {
                break; /* -outfile applies to just one input file */
            }
            outfilename = crate::stddef_h::NULL as *mut libc::c_char
        /* ignore this name if previously processed */
        /* else done parsing switches */
        } else {
            arg = arg.offset(1); /* advance past switch marker character */
            if crate::src::cdjpeg::keymatch(
                arg,
                b"bmp\x00" as *const u8 as *const libc::c_char,
                1i32,
            ) != 0
            {
                /* BMP output format. */
                requested_fmt = FMT_BMP
            } else if crate::src::cdjpeg::keymatch(
                arg,
                b"colors\x00" as *const u8 as *const libc::c_char,
                1i32,
            ) != 0
                || crate::src::cdjpeg::keymatch(
                    arg,
                    b"colours\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                || crate::src::cdjpeg::keymatch(
                    arg,
                    b"quantize\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                || crate::src::cdjpeg::keymatch(
                    arg,
                    b"quantise\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
            {
                /* Do color quantization. */
                let mut val: libc::c_int = 0;
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
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
            } else if crate::src::cdjpeg::keymatch(
                arg,
                b"dct\x00" as *const u8 as *const libc::c_char,
                2i32,
            ) != 0
            {
                /* Select IDCT algorithm. */
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if crate::src::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"int\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_ISLOW
                } else if crate::src::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"fast\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_IFAST
                } else if crate::src::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"float\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_FLOAT
                } else {
                    usage();
                }
            } else if crate::src::cdjpeg::keymatch(
                arg,
                b"dither\x00" as *const u8 as *const libc::c_char,
                2i32,
            ) != 0
            {
                /* Select dithering algorithm. */
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if crate::src::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"fs\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_FS
                } else if crate::src::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"none\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_NONE
                } else if crate::src::cdjpeg::keymatch(
                    *argv.offset(argn as isize),
                    b"ordered\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_ORDERED
                } else {
                    usage();
                }
            } else if crate::src::cdjpeg::keymatch(
                arg,
                b"debug\x00" as *const u8 as *const libc::c_char,
                1i32,
            ) != 0
                || crate::src::cdjpeg::keymatch(
                    arg,
                    b"verbose\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
            {
                /* Enable debug printouts. */
                /* On first -d, print version identification */
                static mut printed_version: crate::jmorecfg_h::boolean = crate::jmorecfg_h::FALSE;
                if printed_version == 0 {
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
            } else if crate::src::cdjpeg::keymatch(
                arg,
                b"version\x00" as *const u8 as *const libc::c_char,
                4i32,
            ) != 0
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
                if crate::src::cdjpeg::keymatch(
                    arg,
                    b"fast\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                {
                    /* Select recommended processing options for quick-and-dirty output. */
                    (*cinfo).two_pass_quantize = crate::jmorecfg_h::FALSE;
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_ORDERED;
                    if (*cinfo).quantize_colors == 0 {
                        /* don't override an earlier -colors */
                        (*cinfo).desired_number_of_colors = 216i32
                    }
                    (*cinfo).dct_method =
                        crate::jpeglib_h::JDCT_FASTEST as crate::jpeglib_h::J_DCT_METHOD;
                    (*cinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"gif\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                {
                    /* GIF output format. */
                    requested_fmt = FMT_GIF
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"grayscale\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                    || crate::src::cdjpeg::keymatch(
                        arg,
                        b"greyscale\x00" as *const u8 as *const libc::c_char,
                        2i32,
                    ) != 0
                {
                    /* Force monochrome output. */
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_GRAYSCALE
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"rgb\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    /* Force RGB output. */
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_RGB
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"rgb565\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    /* Force RGB565 output. */
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_RGB565
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"icc\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                {
                    /* Set ICC filename. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    icc_filename = *argv.offset(argn as isize);
                    crate::jpeglib_h::jpeg_save_markers(
                        cinfo,
                        crate::jpeglib_h::JPEG_APP0 + 2i32,
                        0xffffi32 as libc::c_uint,
                    );
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"map\x00" as *const u8 as *const libc::c_char,
                    3i32,
                ) != 0
                {
                    /* Quantize to a color map taken from an input file. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if for_real != 0 {
                        /* too expensive to do twice! */
                        /* otherwise can't quantize to supplied map */
                        let mut mapfile: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
                        mapfile = crate::stdlib::fopen(
                            *argv.offset(argn as isize),
                            crate::src::cdjpeg::READ_BINARY.as_ptr(),
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
                        crate::src::cdjpeg::read_color_map(cinfo, mapfile);
                        crate::stdlib::fclose(mapfile);
                        (*cinfo).quantize_colors = crate::jmorecfg_h::TRUE
                    }
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"maxmemory\x00" as *const u8 as *const libc::c_char,
                    3i32,
                ) != 0
                {
                    /* Maximum memory in Kb (or Mb with 'm'). */
                    let mut lval: libc::c_long = 0;
                    let mut ch: libc::c_char = 'x' as i32 as libc::c_char;
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
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
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"nosmooth\x00" as *const u8 as *const libc::c_char,
                    3i32,
                ) != 0
                {
                    /* Suppress fancy upsampling */
                    (*cinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"onepass\x00" as *const u8 as *const libc::c_char,
                    3i32,
                ) != 0
                {
                    /* Use fast one-pass quantization. */
                    (*cinfo).two_pass_quantize = crate::jmorecfg_h::FALSE
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"os2\x00" as *const u8 as *const libc::c_char,
                    3i32,
                ) != 0
                {
                    /* BMP output format (OS/2 flavor). */
                    requested_fmt = FMT_OS2
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"outfile\x00" as *const u8 as *const libc::c_char,
                    4i32,
                ) != 0
                {
                    /* Set output file name. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    outfilename = *argv.offset(argn as isize)
                /* save it away for later use */
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"memsrc\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    /* Use in-memory source manager */
                    memsrc = crate::jmorecfg_h::TRUE
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"pnm\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                    || crate::src::cdjpeg::keymatch(
                        arg,
                        b"ppm\x00" as *const u8 as *const libc::c_char,
                        1i32,
                    ) != 0
                {
                    /* PPM/PGM output format. */
                    requested_fmt = FMT_PPM
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"rle\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                {
                    /* RLE output format. */
                    requested_fmt = FMT_RLE
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"scale\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    /* Scale the output image by a fraction M/N. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
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
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"skip\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
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
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"crop\x00" as *const u8 as *const libc::c_char,
                    2i32,
                ) != 0
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
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    b"targa\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) != 0
                {
                    /* Targa output format. */
                    requested_fmt = FMT_TARGA
                } else {
                    usage();
                    /* bogus switch */
                }
            }
        }
        argn += 1
    }
    return argn;
    /* return index of next arg (file name) */
}
/*
 * Marker processor for COM and interesting APPn markers.
 * This replaces the library's built-in processor, which just skips the marker.
 * We want to print out the marker as text, to the extent possible.
 * Note this code relies on a non-suspending data source.
 */

unsafe extern "C" fn jpeg_getc(mut cinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_uint
/* Read next byte */ {
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src; /* discount the length word itself */
    if (*datasrc).bytes_in_buffer == 0i32 as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_CANT_SUSPEND as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
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
    if traceit != 0 {
        if (*cinfo).unread_marker == crate::jpeglib_h::JPEG_COM {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"Comment, length %ld:\n\x00" as *const u8 as *const libc::c_char,
                length,
            );
        } else {
            /* assume it is an APPn otherwise */
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
        if traceit != 0 {
            /* Emit the character in a readable form.
             * Nonprintables are converted to \nnn form,
             * while \ is converted to \\.
             * Newlines in CR, CR/LF, or LF form will be printed as one newline.
             */
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
            } else if *(*crate::stdlib::__ctype_b_loc()).offset(ch as libc::c_int as isize)
                as libc::c_int
                & crate::stdlib::_ISprint as libc::c_int as libc::c_ushort as libc::c_int
                != 0
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
    if traceit != 0 {
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
    let mut dest_mgr: crate::src::cdjpeg::djpeg_dest_ptr =
        crate::stddef_h::NULL as crate::src::cdjpeg::djpeg_dest_ptr;
    let mut input_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut output_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut inbuffer: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut insize: libc::c_ulong = 0i32 as libc::c_ulong;
    let mut num_scanlines: crate::jmorecfg_h::JDIMENSION = 0;
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0); /* in case C library doesn't provide it */
    if progname.is_null() || *progname.offset(0) as libc::c_int == 0i32 {
        progname = b"djpeg\x00" as *const u8 as *const libc::c_char
    }
    /* Initialize the JPEG decompression object with default error handling. */
    cinfo.err = crate::jpeglib_h::jpeg_std_error(&mut jerr);
    crate::jpeglib_h::jpeg_CreateDecompress(
        &mut cinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_decompress_struct>() as libc::c_ulong,
    );
    /* Add some application-specific error messages (from cderror.h) */
    jerr.addon_message_table = cdjpeg_message_table.as_ptr();
    jerr.first_addon_message = crate::cderror_h::JMSG_FIRSTADDONCODE as libc::c_int;
    jerr.last_addon_message = crate::cderror_h::JMSG_LASTADDONCODE as libc::c_int;
    /* Insert custom marker processor for COM and APP12.
     * APP12 is used by some digital camera makers for textual info,
     * so we provide the ability to display it as text.
     * If you like, additional APPn marker types can be selected for display,
     * but don't try to override APP0 or APP14 this way (see libjpeg.txt).
     */
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
    /* Scan command line to find file names. */
    /* It is convenient to use just one switch-parsing routine, but the switch
     * values read here are ignored; we will rescan the switches after opening
     * the input file.
     * (Exception: tracing level set here controls verbosity for COM markers
     * found during jpeg_read_header...)
     */
    file_index = parse_switches(&mut cinfo, argc, argv, 0i32, crate::jmorecfg_h::FALSE);
    /* Unix style: expect zero or one file name */
    if file_index < argc - 1i32 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: only one input file\n\x00" as *const u8 as *const libc::c_char,
            progname,
        );
        usage();
    }
    /* TWO_FILE_COMMANDLINE */
    /* Open the input file. */
    if file_index < argc {
        input_file = crate::stdlib::fopen(
            *argv.offset(file_index as isize),
            crate::src::cdjpeg::READ_BINARY.as_ptr(),
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
        /* default input file is stdin */
        input_file = crate::src::cdjpeg::read_stdin()
    }
    /* Open the output file. */
    if !outfilename.is_null() {
        output_file = crate::stdlib::fopen(outfilename, crate::src::cdjpeg::WRITE_BINARY.as_ptr());
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
        /* default output file is stdout */
        output_file = crate::src::cdjpeg::write_stdout()
    }
    /* Specify data source for decompression */
    if memsrc != 0 {
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
            if nbytes < INPUT_BUF_SIZE as libc::c_ulong && crate::stdlib::ferror(input_file) != 0 {
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
    /* Read file header, set default decompression parameters */
    crate::jpeglib_h::jpeg_read_header(&mut cinfo, crate::jmorecfg_h::TRUE);
    /* Adjust default decompression parameters by re-parsing the options */
    file_index = parse_switches(&mut cinfo, argc, argv, 0i32, crate::jmorecfg_h::TRUE);
    /* Initialize the output module now to let it override any crucial
     * option settings (for instance, GIF wants to force color quantization).
     */
    match requested_fmt as libc::c_uint {
        0 => {
            dest_mgr = crate::src::cdjpeg::jinit_write_bmp(
                &mut cinfo,
                crate::jmorecfg_h::FALSE,
                crate::jmorecfg_h::TRUE,
            )
        }
        2 => {
            dest_mgr = crate::src::cdjpeg::jinit_write_bmp(
                &mut cinfo,
                crate::jmorecfg_h::TRUE,
                crate::jmorecfg_h::TRUE,
            )
        }
        1 => dest_mgr = crate::src::cdjpeg::jinit_write_gif(&mut cinfo),
        3 => dest_mgr = crate::src::cdjpeg::jinit_write_ppm(&mut cinfo),
        5 => dest_mgr = crate::src::cdjpeg::jinit_write_targa(&mut cinfo),
        _ => {
            (*cinfo.err).msg_code = crate::cderror_h::JERR_UNSUPPORTED_FORMAT as libc::c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut cinfo as *mut crate::jpeglib_h::jpeg_decompress_struct
                    as crate::jpeglib_h::j_common_ptr,
            );
        }
    }
    (*dest_mgr).output_file = output_file;
    /* Start decompressor */
    crate::jpeglib_h::jpeg_start_decompress(&mut cinfo);
    /* Skip rows */
    if skip != 0 {
        let mut tmp: crate::jmorecfg_h::JDIMENSION = 0;
        /* Decompress a subregion */
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
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        cinfo.output_height = tmp;
        while cinfo.output_scanline < skip_start
        /* Check for valid skip_end.  We cannot check this value until after
         * jpeg_start_decompress() is called.  Note that we have already verified
         * that skip_start <= skip_end.
         */
        /* Write output file header.  This is a hack to ensure that the destination
         * manager creates an output image of the proper size.
         */
        /* Process data */
        {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
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
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
        }
    } else if crop != 0 {
        let mut tmp_0: crate::jmorecfg_h::JDIMENSION = 0;
        /* Normal full-image decompress */
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
            Some(
                (*dest_mgr)
                    .calc_buffer_dimensions
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        } else {
            (*cinfo.err).msg_code = crate::cderror_h::JERR_UNSUPPORTED_FORMAT as libc::c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut cinfo as *mut crate::jpeglib_h::jpeg_decompress_struct
                    as crate::jpeglib_h::j_common_ptr,
            );
        }
        tmp_0 = cinfo.output_height;
        cinfo.output_height = crop_height;
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        cinfo.output_height = tmp_0;
        crate::jpeglib_h::jpeg_skip_scanlines(&mut cinfo, crop_y);
        while cinfo.output_scanline < crop_y.wrapping_add(crop_height) {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
        }
        crate::jpeglib_h::jpeg_skip_scanlines(
            &mut cinfo,
            cinfo
                .output_height
                .wrapping_sub(crop_y)
                .wrapping_sub(crop_height),
        );
    } else {
        /* Check for valid crop dimensions.  We cannot check these values until
         * after jpeg_start_decompress() is called.
         */
        /* Write output file header.  This is a hack to ensure that the destination
         * manager creates an output image of the proper size.
         */
        /* Process data */
        /* Write output file header */
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        /* Process data */
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
        }
    }
    if !icc_filename.is_null() {
        let mut icc_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
        let mut icc_profile: *mut crate::jmorecfg_h::JOCTET = 0 as *mut crate::jmorecfg_h::JOCTET;
        let mut icc_len: libc::c_uint = 0;
        icc_file = crate::stdlib::fopen(icc_filename, crate::src::cdjpeg::WRITE_BINARY.as_ptr());
        if icc_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        if crate::jpeglib_h::jpeg_read_icc_profile(&mut cinfo, &mut icc_profile, &mut icc_len) != 0
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
        } else if (*cinfo.err).msg_code != crate::src::jerror::JWRN_BOGUS_ICC as libc::c_int {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: no ICC profile data in JPEG file\n\x00" as *const u8 as *const libc::c_char,
                progname,
            );
        }
    }
    /* Finish decompression and release memory.
     * I must do it in this order because output module has allocated memory
     * of lifespan JPOOL_IMAGE; it needs to finish before releasing memory.
     */
    Some(
        (*dest_mgr)
            .finish_output
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(&mut cinfo, dest_mgr);
    crate::jpeglib_h::jpeg_finish_decompress(&mut cinfo);
    crate::jpeglib_h::jpeg_destroy_decompress(&mut cinfo);
    /* Close files, if we opened them */
    if input_file != crate::stdlib::stdin {
        crate::stdlib::fclose(input_file);
    }
    if output_file != crate::stdlib::stdout {
        crate::stdlib::fclose(output_file);
    }
    if memsrc != 0 && !inbuffer.is_null() {
        crate::stdlib::free(inbuffer as *mut libc::c_void);
    }
    /* All done. */
    crate::stdlib::exit(if jerr.num_warnings != 0 {
        crate::src::cdjpeg::EXIT_WARNING
    } else {
        crate::stdlib::EXIT_SUCCESS
    });
    /* suppress no-return-value warnings */
}
#[main]
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
