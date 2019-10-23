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
pub use crate::stddef_h::NULL_0;
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
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_CreateCompress;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_get_int_param;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_c_set_bool_param;
pub use crate::jpeglib_h::jpeg_c_set_float_param;
pub use crate::jpeglib_h::jpeg_c_set_int_param;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_default_colorspace;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_destroy_compress;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_finish_compress;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_mem_dest;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_set_colorspace;
pub use crate::jpeglib_h::jpeg_set_defaults;
pub use crate::jpeglib_h::jpeg_set_quality;
pub use crate::jpeglib_h::jpeg_simple_progression;
pub use crate::jpeglib_h::jpeg_start_compress;
pub use crate::jpeglib_h::jpeg_std_error;
pub use crate::jpeglib_h::jpeg_stdio_dest;
pub use crate::jpeglib_h::jpeg_write_icc_profile;
pub use crate::jpeglib_h::jpeg_write_marker;
pub use crate::jpeglib_h::jpeg_write_scanlines;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_1;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JBOOLEAN_OPTIMIZE_SCANS;
pub use crate::jpeglib_h::JBOOLEAN_OVERSHOOT_DERINGING;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_EOB_OPT;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT_DC;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_Q_OPT;
pub use crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL;
pub use crate::jpeglib_h::JBOOLEAN_USE_SCANS_IN_TRELLIS;
pub use crate::jpeglib_h::JCP_FASTEST;
pub use crate::jpeglib_h::JCP_MAX_COMPRESSION;
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
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1;
pub use crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2;
pub use crate::jpeglib_h::JFLOAT_TRELLIS_DELTA_DC_WEIGHT;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX;
pub use crate::jpeglib_h::JINT_COMPRESS_PROFILE;
pub use crate::jpeglib_h::JINT_DC_SCAN_OPT_MODE;
pub use crate::jpeglib_h::JINT_TRELLIS_FREQ_SPLIT;
pub use crate::jpeglib_h::JINT_TRELLIS_NUM_LOOPS;
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_BOOLEAN_PARAM;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_FLOAT_PARAM;
pub use crate::jpeglib_h::J_INT_PARAM;
pub use crate::src::cdjpeg::cjpeg_source_ptr;
pub use crate::src::cdjpeg::cjpeg_source_struct;
pub use crate::src::cdjpeg::jinit_read_bmp;
pub use crate::src::cdjpeg::jinit_read_gif;
pub use crate::src::cdjpeg::jinit_read_jpeg;
pub use crate::src::cdjpeg::jinit_read_ppm;
pub use crate::src::cdjpeg::jinit_read_targa;
pub use crate::src::cdjpeg::keymatch;
pub use crate::src::cdjpeg::read_quant_tables;
pub use crate::src::cdjpeg::read_scan_script;
pub use crate::src::cdjpeg::read_stdin;
pub use crate::src::cdjpeg::set_quality_ratings;
pub use crate::src::cdjpeg::set_quant_slots;
pub use crate::src::cdjpeg::set_sample_factors;
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
pub use crate::stdlib::atof;
pub use crate::stdlib::atoi;
pub use crate::stdlib::exit;
pub use crate::stdlib::free;
pub use crate::stdlib::malloc;
pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
pub use crate::stdlib::EXIT_FAILURE;
pub use crate::stdlib::EXIT_SUCCESS;

pub use crate::jconfigint_h::BUILD;
pub use crate::jconfigint_h::PACKAGE_NAME;
pub use crate::jconfigint_h::VERSION;
pub use crate::jversion_h::JCOPYRIGHT;
pub use crate::jversion_h::JVERSION;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fprintf;
pub use crate::stdlib::fread;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::getc;
pub use crate::stdlib::sscanf;
pub use crate::stdlib::stderr;
pub use crate::stdlib::stdin;
pub use crate::stdlib::stdout;
pub use crate::stdlib::ungetc;
pub use crate::stdlib::EOF;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;
/*
 * cjpeg.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modified 2003-2011 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2013-2014, 2017, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains a command-line user interface for the JPEG compressor.
 * It should work on any system with Unix- or MS-DOS-style command lines.
 *
 * Two different command line styles are permitted, depending on the
 * compile-time switch TWO_FILE_COMMANDLINE:
 *      cjpeg [options]  inputfile outputfile
 *      cjpeg [options]  [inputfile]
 * In the second style, output is always to standard output, which you'd
 * normally redirect to a file or pipe to some other program.  Input is
 * either from a named file or from standard input (typically redirected).
 * The second style is convenient on Unix but is unhelpful on systems that
 * don't support pipes.  Also, you MUST use the first style if your system
 * doesn't do binary I/O to stdin/stdout.
 * To simplify script writing, the "-outfile" switch is provided.  The syntax
 *      cjpeg [options]  -outfile outputfile  inputfile
 * works regardless of which command line style is used.
 */
/* <stdlib.h> should declare malloc(),free() */
/* command-line reader for Macintosh */
/* Create the add-on message string table. */

static mut cdjpeg_message_table: [*const libc::c_char; 47] = [
    ::std::ptr::null::< libc::c_char>(),
    
    b"Unsupported BMP colormap format\x00".as_ptr() as *const libc::c_char,
    
    b"Only 8- and 24-bit BMP files are supported\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid BMP file: bad header length\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid BMP file: biPlanes not equal to 1\x00".as_ptr() as *const libc::c_char,
    
    b"BMP output must be grayscale or RGB\x00".as_ptr() as *const libc::c_char,
    
    b"Sorry, compressed BMPs not yet supported\x00".as_ptr() as *const libc::c_char,
    
    b"Empty BMP image\x00".as_ptr() as *const libc::c_char,
    
    b"Not a BMP file - does not start with BM\x00".as_ptr() as *const libc::c_char,
    
    b"Numeric value out of range in BMP file\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u 24-bit BMP image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u 8-bit colormapped BMP image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u 24-bit OS2 BMP image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u 8-bit colormapped OS2 BMP image\x00".as_ptr() as *const libc::c_char,
    
    b"GIF output got confused\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus GIF codesize %d\x00".as_ptr() as *const libc::c_char,
    
    b"GIF output must be grayscale or RGB\x00".as_ptr() as *const libc::c_char,
    
    b"Too few images in GIF file\x00".as_ptr() as *const libc::c_char,
    
    b"Not a GIF file\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%ux%d GIF image\x00".as_ptr() as *const libc::c_char,
    
    b"Warning: unexpected GIF version number \'%c%c%c\'\x00".as_ptr() as *const libc::c_char,
    
    b"Ignoring GIF extension block of type 0x%02x\x00".as_ptr() as *const libc::c_char,
    
    b"Caution: nonsquare pixels in input\x00".as_ptr() as *const libc::c_char,
    
    b"Corrupt data in GIF file\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus char 0x%02x in GIF file, ignoring\x00".as_ptr() as *const libc::c_char,
    
    b"Premature end of GIF image\x00".as_ptr() as *const libc::c_char,
    
    b"Ran out of GIF bits\x00".as_ptr() as *const libc::c_char,
    
    b"PPM output must be grayscale or RGB\x00".as_ptr() as *const libc::c_char,
    
    b"Nonnumeric data in PPM file\x00".as_ptr() as *const libc::c_char,
    
    b"Not a PPM/PGM file\x00".as_ptr() as *const libc::c_char,
    
    b"Numeric value out of range in PPM file\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u PGM image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u text PGM image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u PPM image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u text PPM image\x00".as_ptr() as *const libc::c_char,
    
    b"Unsupported Targa colormap format\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid or unsupported Targa file\x00".as_ptr() as *const libc::c_char,
    
    b"Targa output must be grayscale or RGB\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u RGB Targa image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u grayscale Targa image\x00".as_ptr() as *const libc::c_char,
    
    b"%ux%u colormapped Targa image\x00".as_ptr() as *const libc::c_char,
    
    b"Color map file is invalid or of unsupported format\x00".as_ptr() as *const libc::c_char,
    
    b"Output file format cannot handle %d colormap entries\x00".as_ptr() as *const libc::c_char,
    
    b"ungetc failed\x00".as_ptr() as *const libc::c_char,
    
    b"MozJPEG can\'t read the image (PNG support is disabled in this build)\x00".as_ptr()
        as *const libc::c_char,
    
    b"Unsupported output file format\x00".as_ptr() as *const libc::c_char,
    crate::stddef_h::NULL_0 as *const libc::c_char,
];
/*
 * This routine determines what format the input file is,
 * and selects the appropriate input-reading module.
 *
 * To determine which family of input formats the file belongs to,
 * we may look only at the first byte of the file, since C does not
 * guarantee that more than one character can be pushed back with ungetc.
 * Looking at additional bytes would require one of these approaches:
 *     1) assume we can fseek() the input file (fails for piped input);
 *     2) assume we can push back more than one character (works in
 *        some C implementations, but unportable);
 *     3) provide our own buffering (breaks input readers that want to use
 *        stdio directly, such as the RLE library);
 * or  4) don't put back the data, and modify the input_init methods to assume
 *        they start reading after the start of file (also breaks RLE library).
 * #1 is attractive for MS-DOS but is untenable on Unix.
 *
 * The most portable solution for file types that can't be identified by their
 * first byte is to make the user tell us what they are.  This is also the
 * only approach for "raw" file types that contain only arbitrary values.
 * We presently apply this method for Targa files.  Most of the time Targa
 * files start with 0x00, so we recognize that case.  Potentially, however,
 * a Targa file could start with any byte value (byte 0 is the length of the
 * seldom-used ID field), so we provide a switch to force Targa input mode.
 */

static mut is_targa: crate::jmorecfg_h::boolean = 0;
/* records user -targa switch */

static mut is_jpeg: crate::jmorecfg_h::boolean = 0;

static mut copy_markers: crate::jmorecfg_h::boolean = 0;

unsafe extern "C" fn select_file_type(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut infile: *mut crate::stdlib::FILE,
) -> crate::src::cdjpeg::cjpeg_source_ptr {
     let mut c:  libc::c_int =  0;
    if is_targa != 0 {
        return crate::src::cdjpeg::jinit_read_targa(cinfo);
    }
    c = crate::stdlib::getc(infile);
    if c == crate::stdlib::EOF {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EMPTY as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if crate::stdlib::ungetc(c, infile) == crate::stdlib::EOF {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_UNGETC_FAILED as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    match c {
        66 => return crate::src::cdjpeg::jinit_read_bmp(cinfo, crate::jmorecfg_h::TRUE),
        71 => return crate::src::cdjpeg::jinit_read_gif(cinfo),
        80 => return crate::src::cdjpeg::jinit_read_ppm(cinfo),
        0 => return crate::src::cdjpeg::jinit_read_targa(cinfo),
        255 => {
            is_jpeg = crate::jmorecfg_h::TRUE;
            copy_markers = crate::jmorecfg_h::TRUE;
            return crate::src::cdjpeg::jinit_read_jpeg(cinfo);
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_UNKNOWN_FORMAT as libc::c_int;
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
    return crate::stddef_h::NULL_0 as crate::src::cdjpeg::cjpeg_source_ptr;
    /* suppress compiler warnings */
}
/*
 * Argument-parsing code.
 * The switch parser is designed to be useful with DOS-style command line
 * syntax, ie, intermixed switches and file names, where only the switches
 * to the left of a given file name affect processing of that file.
 * The main program in this file doesn't actually use this capability...
 */

static mut progname: *const libc::c_char = ::std::ptr::null::< libc::c_char>();
/* program name for error messages */

static mut icc_filename: *mut libc::c_char = ::std::ptr::null::< libc::c_char>() as *mut libc::c_char;
/* for -icc switch */

static mut outfilename: *mut libc::c_char = ::std::ptr::null::< libc::c_char>() as *mut libc::c_char;
/* for -outfile switch */
#[no_mangle]

pub static mut memdst: crate::jmorecfg_h::boolean = 0;
/* for -memdst switch */

unsafe extern "C" fn usage()
/* complain about bad command line */
{
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"usage: %s [switches] \x00".as_ptr() as *const libc::c_char,
        progname,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"[inputfile]\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"Switches (names may be abbreviated):\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -quality N[,...]   Compression quality (0..100; 5-95 is most useful range,\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                     default is 75)\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -grayscale     Create monochrome JPEG file\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -rgb           Create RGB JPEG file\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(crate::stdlib::stderr,
            
            b"  -optimize      Optimize Huffman table (smaller file, but slow compression, enabled by default)\n\x00".as_ptr() as *const libc::c_char);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -progressive   Create progressive JPEG file (enabled by default)\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -baseline      Create baseline JPEG file (disable progressive coding)\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -targa         Input file is Targa format (usually not needed)\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -revert        Revert to standard defaults (instead of mozjpeg defaults)\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -fastcrush     Disable progressive scan optimization\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -dc-scan-opt   DC scan optimization mode\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 0 One scan for all components\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 1 One scan per component (default)\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(crate::stdlib::stderr,
            
            b"                 - 2 Optimize between one scan for all components and one scan for 1st component\n\x00".as_ptr() as *const libc::c_char);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                     plus one scan for remaining components\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -notrellis     Disable trellis optimization\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -trellis-dc    Enable trellis optimization of DC coefficients (default)\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -notrellis-dc  Disable trellis optimization of DC coefficients\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -tune-psnr     Tune trellis optimization for PSNR\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -tune-hvs-psnr Tune trellis optimization for PSNR-HVS (default)\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -tune-ssim     Tune trellis optimization for SSIM\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -tune-ms-ssim  Tune trellis optimization for MS-SSIM\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"Switches for advanced users:\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -noovershoot   Disable black-on-white deringing via overshoot\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(crate::stdlib::stderr,
            
            b"  -nojfif        Do not write JFIF. Reduce size in 18 bytes but break standar. No know problems in web use.\n\x00".as_ptr() as *const libc::c_char);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -dct int       Use integer DCT method%s\n\x00".as_ptr() as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_ISLOW as libc::c_int {
            
            b" (default)\x00".as_ptr() as *const libc::c_char
        } else {
            
            b"\x00".as_ptr() as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -dct fast      Use fast integer DCT (less accurate)%s\n\x00".as_ptr()
            as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_IFAST as libc::c_int {
            
            b" (default)\x00".as_ptr() as *const libc::c_char
        } else {
            
            b"\x00".as_ptr() as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -dct float     Use floating-point DCT method%s\n\x00".as_ptr()
            as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_FLOAT as libc::c_int {
            
            b" (default)\x00".as_ptr() as *const libc::c_char
        } else {
            
            b"\x00".as_ptr() as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(crate::stdlib::stderr,
            
            b"  -quant-baseline Use 8-bit quantization table entries for baseline JPEG compatibility\n\x00".as_ptr() as *const libc::c_char);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -quant-table N Use predefined quantization table N:\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 0 JPEG Annex K\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 1 Flat\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 2 Custom, tuned for MS-SSIM\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 3 ImageMagick table by N. Robidoux\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 4 Custom, tuned for PSNR-HVS\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"                 - 5 Table from paper by Klein, Silverstein and Carney\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -icc FILE      Embed ICC profile contained in FILE\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -restart N     Set restart interval in rows, or in blocks with B\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -smooth N      Smooth dithered input (N=1..100 is strength)\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -maxmemory N   Maximum memory to use (in kbytes)\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -outfile name  Specify name for output file\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -memdst        Compress to memory instead of file (useful for benchmarking)\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -verbose  or  -debug   Emit debug output\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -version       Print version information and exit\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"Switches for wizards:\n\x00".as_ptr() as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -qtables FILE  Use quantization tables given in FILE\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -qslots N[,...]    Set component quantization tables\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -sample HxV[,...]  Set component sampling factors\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        
        b"  -scans FILE    Create multi-scan JPEG per script FILE\n\x00".as_ptr()
            as *const libc::c_char,
    );
    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
}

unsafe extern "C" fn parse_switches(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
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
     /* saves -quality parm if any */
     /* saves -qtables filename if any */
     /* saves -qslots parm if any */
     let mut argn:  libc::c_int =  0; let mut force_baseline:  crate::jmorecfg_h::boolean =  0; let mut simple_progressive:  crate::jmorecfg_h::boolean =  0; /* saves -sample parm if any */
    let mut qualityarg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char; /* saves -scans parm if any */
    let mut qtablefile: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut qslotsarg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut samplearg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut scansarg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    /* Set up default JPEG parameters. */
    force_baseline = crate::jmorecfg_h::FALSE; /* by default, allow 16-bit quantizers */
    simple_progressive = if (*cinfo).num_scans == 0i32 {
        crate::jmorecfg_h::FALSE
    } else {
        crate::jmorecfg_h::TRUE
    };
    is_targa = crate::jmorecfg_h::FALSE;
    icc_filename = crate::stddef_h::NULL_0 as *mut libc::c_char;
    outfilename = crate::stddef_h::NULL_0 as *mut libc::c_char;
    memdst = crate::jmorecfg_h::FALSE;
    (*(*cinfo).err).trace_level = 0i32;
    /* Scan command line options, adjust parameters */
    argn = 1i32;
    while argn < argc {
         let mut arg:  *mut libc::c_char =  ::std::ptr::null_mut::< libc::c_char>();arg = *argv.offset(argn as isize);
        if *arg as libc::c_int != '-' as i32 {
            /* Not a switch, must be a file name argument */
            if !(argn <= last_file_arg_seen) {
                break; /* -outfile applies to just one input file */
            }
            outfilename = crate::stddef_h::NULL_0 as *mut libc::c_char
        /* ignore this name if previously processed */
        /* else done parsing switches */
        } else {
            arg = arg.offset(1); /* advance past switch marker character */
            if crate::src::cdjpeg::keymatch(
                arg,
                
                b"arithmetic\x00".as_ptr() as *const libc::c_char,
                1i32,
            ) != 0
            {
                /* Use arithmetic coding. */
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    
                    b"%s: sorry, arithmetic coding not supported\n\x00".as_ptr()
                        as *const libc::c_char,
                    progname,
                );
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            } else {
                if crate::src::cdjpeg::keymatch(
                    arg,
                    
                    b"baseline\x00".as_ptr() as *const libc::c_char,
                    1i32,
                ) != 0
                {
                    /* Force baseline-compatible output (8-bit quantizer values). */
                    force_baseline = crate::jmorecfg_h::TRUE;
                    /* Disable multiple scans */
                    simple_progressive = crate::jmorecfg_h::FALSE;
                    (*cinfo).num_scans = 0i32;
                    (*cinfo).scan_info =
                        crate::stddef_h::NULL_0 as *const crate::jpeglib_h::jpeg_scan_info
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    
                    b"dct\x00".as_ptr() as *const libc::c_char,
                    2i32,
                ) != 0
                {
                    /* Select DCT algorithm. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            
                            b"%s: missing argument for dct\n\x00".as_ptr()
                                as *const libc::c_char,
                            progname,
                        );
                        usage();
                    }
                    if crate::src::cdjpeg::keymatch(
                        *argv.offset(argn as isize),
                        
                        b"int\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                    {
                        (*cinfo).dct_method = crate::jpeglib_h::JDCT_ISLOW
                    } else if crate::src::cdjpeg::keymatch(
                        *argv.offset(argn as isize),
                        
                        b"fast\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                    {
                        (*cinfo).dct_method = crate::jpeglib_h::JDCT_IFAST
                    } else if crate::src::cdjpeg::keymatch(
                        *argv.offset(argn as isize),
                        
                        b"float\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                    {
                        (*cinfo).dct_method = crate::jpeglib_h::JDCT_FLOAT
                    } else {
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            
                            b"%s: invalid argument for dct\n\x00".as_ptr()
                                as *const libc::c_char,
                            progname,
                        );
                        usage();
                    }
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    
                    b"debug\x00".as_ptr() as *const libc::c_char,
                    1i32,
                ) != 0
                    || crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"verbose\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                {
                    /* Enable debug printouts. */
                    /* On first -d, print version identification */
                    static mut printed_version: crate::jmorecfg_h::boolean =
                        crate::jmorecfg_h::FALSE;
                    if printed_version == 0 {
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            
                            b"%s version %s (build %s)\n\x00".as_ptr() as *const libc::c_char,
                            crate::jconfigint_h::PACKAGE_NAME.as_ptr(),
                            crate::jconfigint_h::VERSION.as_ptr(),
                            crate::jconfigint_h::BUILD.as_ptr(),
                        );
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            
                            b"%s\n\n\x00".as_ptr() as *const libc::c_char,
                            crate::jversion_h::JCOPYRIGHT.as_ptr(),
                        );
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            
                            b"Emulating The Independent JPEG Group\'s software, version %s\n\n\x00".as_ptr() as *const libc::c_char,
                            crate::jversion_h::JVERSION.as_ptr(),
                        );
                        printed_version = crate::jmorecfg_h::TRUE
                    }
                    (*(*cinfo).err).trace_level += 1
                } else if crate::src::cdjpeg::keymatch(
                    arg,
                    
                    b"version\x00".as_ptr() as *const libc::c_char,
                    4i32,
                ) != 0
                {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        
                        b"%s version %s (build %s)\n\x00".as_ptr() as *const libc::c_char,
                        crate::jconfigint_h::PACKAGE_NAME.as_ptr(),
                        crate::jconfigint_h::VERSION.as_ptr(),
                        crate::jconfigint_h::BUILD.as_ptr(),
                    );
                    crate::stdlib::exit(crate::stdlib::EXIT_SUCCESS);
                } else {
                    if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"fastcrush\x00".as_ptr() as *const libc::c_char,
                        4i32,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_OPTIMIZE_SCANS,
                            crate::jmorecfg_h::FALSE,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"grayscale\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                        || crate::src::cdjpeg::keymatch(
                            arg,
                            
                            b"greyscale\x00".as_ptr() as *const libc::c_char,
                            2i32,
                        ) != 0
                    {
                        /* Force a monochrome JPEG file to be generated. */
                        crate::jpeglib_h::jpeg_set_colorspace(
                            cinfo,
                            crate::jpeglib_h::JCS_GRAYSCALE,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"rgb\x00".as_ptr() as *const libc::c_char,
                        3i32,
                    ) != 0
                    {
                        /* Force an RGB JPEG file to be generated. */
                        crate::jpeglib_h::jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_RGB);
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"lambda1\x00".as_ptr() as *const libc::c_char,
                        7i32,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            crate::stdlib::atof(*argv.offset(argn as isize)) as libc::c_float,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"lambda2\x00".as_ptr() as *const libc::c_char,
                        7i32,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            crate::stdlib::atof(*argv.offset(argn as isize)) as libc::c_float,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"icc\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                    {
                        /* Set ICC filename. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        icc_filename = *argv.offset(argn as isize)
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"maxmemory\x00".as_ptr() as *const libc::c_char,
                        3i32,
                    ) != 0
                    {
                        
                         let mut lval:  libc::c_long =  0; let mut ch:  libc::c_char =   'x' as libc::c_char;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if crate::stdlib::sscanf(
                            *argv.offset(argn as isize),
                            
                            b"%ld%c\x00".as_ptr() as *const libc::c_char,
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
                        
                        b"dc-scan-opt\x00".as_ptr() as *const libc::c_char,
                        3i32,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                
                                b"%s: missing argument for dc-scan-opt\n\x00".as_ptr()
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_DC_SCAN_OPT_MODE,
                            crate::stdlib::atoi(*argv.offset(argn as isize)),
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"optimize\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                        || crate::src::cdjpeg::keymatch(
                            arg,
                            
                            b"optimise\x00".as_ptr() as *const libc::c_char,
                            1i32,
                        ) != 0
                    {
                        /* Enable entropy parm optimization. */
                        (*cinfo).optimize_coding = crate::jmorecfg_h::TRUE
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"outfile\x00".as_ptr() as *const libc::c_char,
                        4i32,
                    ) != 0
                    {
                        /* Set output file name. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                
                                b"%s: missing argument for outfile\n\x00".as_ptr()
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        outfilename = *argv.offset(argn as isize)
                    /* save it away for later use */
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"progressive\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                    {
                        /* Select simple progressive mode. */
                        simple_progressive = crate::jmorecfg_h::TRUE
                    /* We must postpone execution until num_components is known. */
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"memdst\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                    {
                        /* Use in-memory destination manager */
                        memdst = crate::jmorecfg_h::TRUE
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"quality\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                    {
                        /* Quality ratings (quantization table scaling factors). */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                
                                b"%s: missing argument for quality\n\x00".as_ptr()
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        qualityarg = *argv.offset(argn as isize)
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"qslots\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                    {
                        /* Quantization table slot numbers. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        qslotsarg = *argv.offset(argn as isize)
                    /* Must delay setting qslots until after we have processed any
                     * colorspace-determining switches, since jpeg_set_colorspace sets
                     * default quant table numbers.
                     */
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"qtables\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                    {
                        /* Quantization tables fetched from file. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        qtablefile = *argv.offset(argn as isize)
                    /* We postpone actually reading the file in case -quality comes later. */
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"quant-table\x00".as_ptr() as *const libc::c_char,
                        7i32,
                    ) != 0
                    {
                         let mut val:  libc::c_int =  0;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        val = crate::stdlib::atoi(*argv.offset(argn as isize));
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            val,
                        );
                        if crate::jpeglib_h::jpeg_c_get_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                        ) != val
                        {
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                
                                b"%s: %d is invalid argument for quant-table\n\x00".as_ptr()
                                    as *const libc::c_char,
                                progname,
                                val,
                            );
                            usage();
                        }
                        crate::jpeglib_h::jpeg_set_quality(cinfo, 75i32, crate::jmorecfg_h::TRUE);
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"quant-baseline\x00".as_ptr() as *const libc::c_char,
                        7i32,
                    ) != 0
                    {
                        /* Force quantization table to meet baseline requirements */
                        force_baseline = crate::jmorecfg_h::TRUE
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"restart\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                    {
                        
                         let mut lval_0:  libc::c_long =  0; let mut ch_0:  libc::c_char =   'x' as libc::c_char;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if crate::stdlib::sscanf(
                            *argv.offset(argn as isize),
                            
                            b"%ld%c\x00".as_ptr() as *const libc::c_char,
                            &mut lval_0 as *mut libc::c_long,
                            &mut ch_0 as *mut libc::c_char,
                        ) < 1i32
                        {
                            usage();
                        }
                        if lval_0 < 0i64 || lval_0 > 65535i64 {
                            usage();
                        }
                        if ch_0 as libc::c_int == 'b' as i32 || ch_0 as libc::c_int == 'B' as i32 {
                            (*cinfo).restart_interval = lval_0 as libc::c_uint;
                            (*cinfo).restart_in_rows = 0i32
                        /* else prior '-restart n' overrides me */
                        } else {
                            (*cinfo).restart_in_rows = lval_0 as libc::c_int
                            /* restart_interval will be computed during startup */
                        }
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"revert\x00".as_ptr() as *const libc::c_char,
                        3i32,
                    ) != 0
                    {
                        /* revert to old JPEG default */
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_COMPRESS_PROFILE,
                            crate::jpeglib_h::JCP_FASTEST as libc::c_int,
                        );
                        crate::jpeglib_h::jpeg_set_defaults(cinfo);
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"sample\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                    {
                        /* Set sampling factors. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        samplearg = *argv.offset(argn as isize)
                    /* Must delay setting sample factors until after we have processed any
                     * colorspace-determining switches, since jpeg_set_colorspace sets
                     * default sampling factors.
                     */
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"scans\x00".as_ptr() as *const libc::c_char,
                        4i32,
                    ) != 0
                    {
                        /* Set scan script. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        scansarg = *argv.offset(argn as isize)
                    /* We must postpone reading the file in case -progressive appears. */
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"smooth\x00".as_ptr() as *const libc::c_char,
                        2i32,
                    ) != 0
                    {
                         let mut val_0:  libc::c_int =  0;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if crate::stdlib::sscanf(
                            *argv.offset(argn as isize),
                            
                            b"%d\x00".as_ptr() as *const libc::c_char,
                            &mut val_0 as *mut libc::c_int,
                        ) != 1i32
                        {
                            usage();
                        }
                        if val_0 < 0i32 || val_0 > 100i32 {
                            usage();
                        }
                        (*cinfo).smoothing_factor = val_0
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"targa\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                    {
                        /* Input file is Targa format. */
                        is_targa = crate::jmorecfg_h::TRUE
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"notrellis-dc\x00".as_ptr() as *const libc::c_char,
                        11i32,
                    ) != 0
                    {
                        /* disable trellis quantization */
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT_DC,
                            crate::jmorecfg_h::FALSE,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"notrellis\x00".as_ptr() as *const libc::c_char,
                        1i32,
                    ) != 0
                    {
                        /* disable trellis quantization */
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT,
                            crate::jmorecfg_h::FALSE,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"trellis-dc-ver-weight\x00".as_ptr() as *const libc::c_char,
                        12i32,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                
                                b"%s: missing argument for trellis-dc-ver-weight\n\x00".as_ptr()
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_TRELLIS_DELTA_DC_WEIGHT,
                            crate::stdlib::atof(*argv.offset(argn as isize)) as libc::c_float,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"trellis-dc\x00".as_ptr() as *const libc::c_char,
                        9i32,
                    ) != 0
                    {
                        /* enable DC trellis quantization */
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT_DC,
                            crate::jmorecfg_h::TRUE,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"tune-psnr\x00".as_ptr() as *const libc::c_char,
                        6i32,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            1i32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            9f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            0f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::FALSE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(cinfo, 75i32, crate::jmorecfg_h::TRUE);
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"tune-ssim\x00".as_ptr() as *const libc::c_char,
                        6i32,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            1i32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            11.5f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            12.75f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::FALSE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(cinfo, 75i32, crate::jmorecfg_h::TRUE);
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"tune-ms-ssim\x00".as_ptr() as *const libc::c_char,
                        6i32,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            3i32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            12f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            13f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::TRUE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(cinfo, 75i32, crate::jmorecfg_h::TRUE);
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"tune-hvs-psnr\x00".as_ptr() as *const libc::c_char,
                        6i32,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            3i32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            14.75f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            16.5f32,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::TRUE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(cinfo, 75i32, crate::jmorecfg_h::TRUE);
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"noovershoot\x00".as_ptr() as *const libc::c_char,
                        11i32,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_OVERSHOOT_DERINGING,
                            crate::jmorecfg_h::FALSE,
                        );
                    } else if crate::src::cdjpeg::keymatch(
                        arg,
                        
                        b"nojfif\x00".as_ptr() as *const libc::c_char,
                        6i32,
                    ) != 0
                    {
                        (*cinfo).write_JFIF_header = 0i32
                    } else {
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            
                            b"%s: unknown option \'%s\'\n\x00".as_ptr() as *const libc::c_char,
                            progname,
                            arg,
                        );
                        usage();
                        /* bogus switch */
                    }
                }
            }
        }
        argn += 1
    }
    /* Post-switch-scanning cleanup */
    if for_real != 0 {
        /* Set quantization tables for selected quality. */
        /* Some or all may be overridden if -qtables is present. */
        if !qualityarg.is_null() {
            /* process -quality if it was present */
            if crate::src::cdjpeg::set_quality_ratings(cinfo, qualityarg, force_baseline) == 0 {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    
                    b"%s: can\'t set quality ratings\n\x00".as_ptr() as *const libc::c_char,
                    progname,
                );
                usage();
            }
        }
        if !qtablefile.is_null() {
            /* process -qtables if it was present */
            if crate::src::cdjpeg::read_quant_tables(cinfo, qtablefile, force_baseline) == 0 {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    
                    b"%s: can\'t read qtable file\n\x00".as_ptr() as *const libc::c_char,
                    progname,
                );
                usage();
            }
        }
        if !qslotsarg.is_null() {
            /* process -qslots if it was present */
            if crate::src::cdjpeg::set_quant_slots(cinfo, qslotsarg) == 0 {
                usage();
            }
        }
        /* set_quality_ratings sets default subsampling, so the explicit
        subsampling must be set after it */
        if !samplearg.is_null() {
            /* process -sample if it was present */
            if crate::src::cdjpeg::set_sample_factors(cinfo, samplearg) == 0 {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    
                    b"%s: can\'t set sample factors\n\x00".as_ptr() as *const libc::c_char,
                    progname,
                );
                usage();
            }
        }
        if simple_progressive != 0 {
            /* process -progressive; -scans can override */
            crate::jpeglib_h::jpeg_simple_progression(cinfo);
        }
        if !scansarg.is_null() {
            /* process -scans if it was present */
            if crate::src::cdjpeg::read_scan_script(cinfo, scansarg) == 0 {
                usage();
            }
        }
    }
    return argn;
    /* return index of next arg (file name) */
}
/*
 * The main program.
 */

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
     let mut file_index:  libc::c_int =  0; let mut src_mgr:  crate::src::cdjpeg::cjpeg_source_ptr =
    
        ::std::ptr::null_mut::< crate::src::cdjpeg::cjpeg_source_struct>(); let mut input_file:  *mut crate::stdlib::FILE =
     ::std::ptr::null_mut::< crate::stdlib::FILE>(); let mut icc_len:  libc::c_long =  0i64; let mut outsize:  libc::c_ulong =  0u64;let mut cinfo: crate::jpeglib_h::jpeg_compress_struct =
        crate::jpeglib_h::jpeg_compress_struct {
            err: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_error_mgr>(),
            mem: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_memory_mgr>(),
            progress: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_progress_mgr>(),
            client_data: ::std::ptr::null_mut::< libc::c_void>(),
            is_decompressor: 0,
            global_state: 0,
            dest: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_destination_mgr>(),
            image_width: 0,
            image_height: 0,
            input_components: 0,
            in_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            input_gamma: 0.,
            data_precision: 0,
            num_components: 0,
            jpeg_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            comp_info: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>(),
            quant_tbl_ptrs: [::std::ptr::null_mut::< crate::jpeglib_h::JQUANT_TBL>(); 4],
            dc_huff_tbl_ptrs: [::std::ptr::null_mut::< crate::jpeglib_h::JHUFF_TBL>(); 4],
            ac_huff_tbl_ptrs: [::std::ptr::null_mut::< crate::jpeglib_h::JHUFF_TBL>(); 4],
            arith_dc_L: [0; 16],
            arith_dc_U: [0; 16],
            arith_ac_K: [0; 16],
            num_scans: 0,
            scan_info: ::std::ptr::null::< crate::jpeglib_h::jpeg_scan_info>(),
            raw_data_in: 0,
            arith_code: 0,
            optimize_coding: 0,
            CCIR601_sampling: 0,
            smoothing_factor: 0,
            dct_method: crate::jpeglib_h::JDCT_ISLOW,
            restart_interval: 0,
            restart_in_rows: 0,
            write_JFIF_header: 0,
            JFIF_major_version: 0,
            JFIF_minor_version: 0,
            density_unit: 0,
            X_density: 0,
            Y_density: 0,
            write_Adobe_marker: 0,
            next_scanline: 0,
            progressive_mode: 0,
            max_h_samp_factor: 0,
            max_v_samp_factor: 0,
            total_iMCU_rows: 0,
            comps_in_scan: 0,
            cur_comp_info: [::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>(); 4],
            MCUs_per_row: 0,
            MCU_rows_in_scan: 0,
            blocks_in_MCU: 0,
            MCU_membership: [0; 10],
            Ss: 0,
            Se: 0,
            Ah: 0,
            Al: 0,
            master: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_comp_master>(),
            main: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_c_main_controller>(),
            prep: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_c_prep_controller>(),
            coef: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_c_coef_controller>(),
            marker: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_marker_writer>(),
            cconvert: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_color_converter>(),
            downsample: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_downsampler>(),
            fdct: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_forward_dct>(),
            entropy: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_entropy_encoder>(),
            script_space: ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_scan_info>(),
            script_space_size: 0,
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
        jpeg_message_table: ::std::ptr::null::< *const libc::c_char>(),
        last_jpeg_message: 0,
        addon_message_table: ::std::ptr::null::< *const libc::c_char>(),
        first_addon_message: 0,
        last_addon_message: 0,
    };
    
    
    
    
    let mut icc_profile: *mut crate::jmorecfg_h::JOCTET =
        crate::stddef_h::NULL_0 as *mut crate::jmorecfg_h::JOCTET;
    
    let mut output_file: *mut crate::stdlib::FILE =
        crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut outbuffer: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    
    
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0); /* in case C library doesn't provide it */
    if progname.is_null() || *progname.offset(0) as libc::c_int == 0i32 {
        progname =  b"cjpeg\x00".as_ptr() as *const libc::c_char
    }
    /* Initialize the JPEG compression object with default error handling. */
    cinfo.err = crate::jpeglib_h::jpeg_std_error(&mut jerr);
    crate::jpeglib_h::jpeg_CreateCompress(
        &mut cinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong,
    );
    /* Add some application-specific error messages (from cderror.h) */
    jerr.addon_message_table = cdjpeg_message_table.as_ptr();
    jerr.first_addon_message = crate::cderror_h::JMSG_FIRSTADDONCODE as libc::c_int;
    jerr.last_addon_message = crate::cderror_h::JMSG_LASTADDONCODE as libc::c_int;
    /* Initialize JPEG parameters.
     * Much of this may be overridden later.
     * In particular, we don't yet know the input file's color space,
     * but we need to provide some value for jpeg_set_defaults() to work.
     */
    cinfo.in_color_space = crate::jpeglib_h::JCS_RGB; /* arbitrary guess */
    crate::jpeglib_h::jpeg_set_defaults(&mut cinfo);
    /* Scan command line to find file names.
     * It is convenient to use just one switch-parsing routine, but the switch
     * values read here are ignored; we will rescan the switches after opening
     * the input file.
     */
    file_index = parse_switches(&mut cinfo, argc, argv, 0i32, crate::jmorecfg_h::FALSE);
    /* Unix style: expect zero or one file name */
    if file_index < argc - 1i32 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            
            b"%s: only one input file\n\x00".as_ptr() as *const libc::c_char,
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
                
                b"%s: can\'t open %s\n\x00".as_ptr() as *const libc::c_char,
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
                
                b"%s: can\'t open %s\n\x00".as_ptr() as *const libc::c_char,
                progname,
                outfilename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
    } else if memdst == 0 {
        /* default output file is stdout */
        output_file = crate::src::cdjpeg::write_stdout()
    }
    if !icc_filename.is_null() {
         let mut icc_file:  *mut crate::stdlib::FILE =
     ::std::ptr::null_mut::< crate::stdlib::FILE>();icc_file = crate::stdlib::fopen(icc_filename, crate::src::cdjpeg::READ_BINARY.as_ptr());
        if icc_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                
                b"%s: can\'t open %s\n\x00".as_ptr() as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        if crate::stdlib::fseek(icc_file, 0i64, crate::stdlib::SEEK_END) < 0i32
            || {
                icc_len = crate::stdlib::ftell(icc_file);
                (icc_len) < 1i64
            }
            || crate::stdlib::fseek(icc_file, 0i64, crate::stdlib::SEEK_SET) < 0i32
        {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                
                b"%s: can\'t determine size of %s\n\x00".as_ptr() as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        icc_profile =
            crate::stdlib::malloc(icc_len as libc::c_ulong) as *mut crate::jmorecfg_h::JOCTET;
        if icc_profile.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                
                b"%s: can\'t allocate memory for ICC profile\n\x00".as_ptr()
                    as *const libc::c_char,
                progname,
            );
            crate::stdlib::fclose(icc_file);
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        if crate::stdlib::fread(
            icc_profile as *mut libc::c_void,
            icc_len as libc::c_ulong,
            1u64,
            icc_file,
        ) < 1u64
        {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                
                b"%s: can\'t read ICC profile from %s\n\x00".as_ptr() as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::free(icc_profile as *mut libc::c_void);
            crate::stdlib::fclose(icc_file);
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        crate::stdlib::fclose(icc_file);
    }
    /* Figure out the input file format, and set up to read it. */
    src_mgr = select_file_type(&mut cinfo, input_file);
    (*src_mgr).input_file = input_file;
    /* Read the input file header to obtain file size & colorspace. */
    Some((*src_mgr).start_input.expect("non-null function pointer"))
        .expect("non-null function pointer")(&mut cinfo, src_mgr);
    /* Now that we know input colorspace, fix colorspace-dependent defaults */
    crate::jpeglib_h::jpeg_default_colorspace(&mut cinfo);
    /* Adjust default compression parameters by re-parsing the options */
    file_index = parse_switches(&mut cinfo, argc, argv, 0i32, crate::jmorecfg_h::TRUE);
    /* Specify data destination for compression */
    if memdst != 0 {
        crate::jpeglib_h::jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);
    } else {
        crate::jpeglib_h::jpeg_stdio_dest(&mut cinfo, output_file);
    }
    /* Start compressor */
    crate::jpeglib_h::jpeg_start_compress(&mut cinfo, crate::jmorecfg_h::TRUE);
    /* Copy metadata */
    if copy_markers != 0 {
         let mut marker:  crate::jpeglib_h::jpeg_saved_marker_ptr =
    
            ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_marker_struct>();
        /* In the current implementation, we don't actually need to examine the
         * option flag here; we just copy everything that got saved.
         * But to avoid confusion, we do not output JFIF and Adobe APP14 markers
         * if the encoder library already wrote one.
         */
        marker = (*src_mgr).marker_list; /* reject duplicate JFIF */
        while !marker.is_null() {
            if !(cinfo.write_JFIF_header != 0
                && (*marker).marker as libc::c_int == crate::jpeglib_h::JPEG_APP0
                && (*marker).data_length >= 5u32
                && *(*marker).data.offset(0) as libc::c_int == 0x4ai32
                && *(*marker).data.offset(1) as libc::c_int == 0x46i32
                && *(*marker).data.offset(2) as libc::c_int == 0x49i32
                && *(*marker).data.offset(3) as libc::c_int == 0x46i32
                && *(*marker).data.offset(4) as libc::c_int == 0i32)
            {
                if !(cinfo.write_Adobe_marker != 0
                    && (*marker).marker as libc::c_int == crate::jpeglib_h::JPEG_APP0 + 14i32
                    && (*marker).data_length >= 5u32
                    && *(*marker).data.offset(0) as libc::c_int == 0x41i32
                    && *(*marker).data.offset(1) as libc::c_int == 0x64i32
                    && *(*marker).data.offset(2) as libc::c_int == 0x6fi32
                    && *(*marker).data.offset(3) as libc::c_int == 0x62i32
                    && *(*marker).data.offset(4) as libc::c_int == 0x65i32)
                {
                    crate::jpeglib_h::jpeg_write_marker(
                        &mut cinfo,
                        (*marker).marker as libc::c_int,
                        (*marker).data,
                        (*marker).data_length,
                    ); /* reject duplicate Adobe */
                }
            }
            marker = (*marker).next
        }
    }
    if !icc_profile.is_null() {
        crate::jpeglib_h::jpeg_write_icc_profile(&mut cinfo, icc_profile, icc_len as libc::c_uint);
    }
    /* Process data */
    while cinfo.next_scanline < cinfo.image_height {
         let mut num_scanlines:  crate::jmorecfg_h::JDIMENSION =  0;num_scanlines = Some(
            (*src_mgr)
                .get_pixel_rows
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(&mut cinfo, src_mgr);
        crate::jpeglib_h::jpeg_write_scanlines(&mut cinfo, (*src_mgr).buffer, num_scanlines);
    }
    /* Finish compression and release memory */
    Some((*src_mgr).finish_input.expect("non-null function pointer"))
        .expect("non-null function pointer")(&mut cinfo, src_mgr);
    crate::jpeglib_h::jpeg_finish_compress(&mut cinfo);
    crate::jpeglib_h::jpeg_destroy_compress(&mut cinfo);
    /* Close files, if we opened them */
    if input_file != crate::stdlib::stdin {
        crate::stdlib::fclose(input_file);
    }
    if output_file != crate::stdlib::stdout && !output_file.is_null() {
        crate::stdlib::fclose(output_file);
    }
    if memdst != 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            
            b"Compressed size:  %lu bytes\n\x00".as_ptr() as *const libc::c_char,
            outsize,
        );
        if !outbuffer.is_null() {
            crate::stdlib::free(outbuffer as *mut libc::c_void);
        }
    }
    if !icc_profile.is_null() {
        crate::stdlib::free(icc_profile as *mut libc::c_void);
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
     let mut args:  Vec<*mut libc::c_char> =  Vec::new();
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
            
            args.as_mut_ptr(),
        ))
    }
}
