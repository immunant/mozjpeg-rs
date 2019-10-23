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












































































































































































































































































































































use std::prelude::v1::*;use crate::stdlib::{fclose, ferror, fopen, fprintf, fread, fwrite, putc,
                    sscanf, stderr, stdin, stdout};use mozjpeg::*;use libc::{c_void, c_uchar, c_ulong, c_ushort, c_long, c_char, c_uint, c_int};use std::prelude::v1;pub use crate::jversion_h::{JCOPYRIGHT, JVERSION};pub use crate::jconfigint_h::{BUILD, PACKAGE_NAME, VERSION};pub use crate::src::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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
                             JERR_EOI_EXPECTED, JERR_FILE_READ,
                             JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL,
                             JERR_HUFF_CLEN_OVERFLOW, JERR_HUFF_MISSING_CODE,
                             JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY,
                             JERR_INPUT_EOF, JERR_MISMATCHED_QUANT_TABLE,
                             JERR_MISSING_DATA, JERR_MODE_CHANGE,
                             JERR_NOTIMPL, JERR_NOT_COMPILED,
                             JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                             JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                             JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                             JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                             JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                             JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                             JERR_SOS_NO_SOF, JERR_TFILE_CREATE,
                             JERR_TFILE_READ, JERR_TFILE_SEEK,
                             JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA,
                             JERR_UNKNOWN_MARKER, JERR_UNSUPPORTED_SUSPEND,
                             JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW,
                             JERR_XMS_READ, JERR_XMS_WRITE, JMSG_COPYRIGHT,
                             JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION,
                             JTRC_16BIT_TABLES, JTRC_ADOBE, JTRC_APP0,
                             JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                             JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
                             JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF,
                             JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
                             JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER,
                             JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
                             JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS,
                             JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION,
                             JTRC_RST, JTRC_SMOOTH_NOTIMPL, JTRC_SOF,
                             JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                             JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS,
                             JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN,
                             JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                             JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                             JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                             JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                             JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE,
                             JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC,
                             JWRN_NOT_SEQUENTIAL, JWRN_TOO_MUCH_DATA};pub use crate::src::cdjpeg::{djpeg_dest_ptr, djpeg_dest_struct,
                             jinit_write_bmp, jinit_write_gif,
                             jinit_write_ppm, jinit_write_targa, keymatch,
                             read_color_map, read_stdin, write_stdout,
                             EXIT_WARNING, READ_BINARY, WRITE_BINARY};pub use crate::cderror_h::{C2RustUnnamed_4, JERR_BAD_CMAP_FILE,
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
                        __off64_t, __off_t, FILE, _IO_FILE, C2RustUnnamed_0,
                        _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit,
                        _ISgraph, _ISlower, _ISprint, _ISpunct, _ISspace,
                        _ISupper, _ISxdigit, __ctype_b_loc, exit, free,
                        realloc, EXIT_FAILURE, EXIT_SUCCESS};pub use crate::jconfig_h::JPEG_LIB_VERSION;pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_CreateDecompress, jpeg_color_deconverter,
                           jpeg_color_quantizer, jpeg_common_struct,
                           jpeg_component_info, jpeg_crop_scanline,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_destroy_decompress,
                           jpeg_entropy_decoder, jpeg_error_mgr,
                           jpeg_finish_decompress, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_parser_method,
                           jpeg_marker_reader, jpeg_marker_struct,
                           jpeg_mem_src, jpeg_memory_mgr, jpeg_progress_mgr,
                           jpeg_read_header, jpeg_read_icc_profile,
                           jpeg_read_scanlines, jpeg_save_markers,
                           jpeg_saved_marker_ptr, jpeg_set_marker_processor,
                           jpeg_skip_scanlines, jpeg_source_mgr,
                           jpeg_start_decompress, jpeg_std_error,
                           jpeg_stdio_src, jpeg_upsampler,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
                           JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB,
                           JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
                           JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
                           JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
                           JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_DEFAULT,
                           JDCT_FASTEST, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
                           JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED,
                           JHUFF_TBL, JPEG_APP0, JPEG_COM, JQUANT_TBL,
                           JSAMPARRAY, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
                           J_DITHER_MODE};pub use crate::stddef_h::{size_t, NULL};

pub type IMAGE_FORMATS = c_uint;

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

static mut cdjpeg_message_table: [*const c_char; 47] = [
    ::std::ptr::null::< c_char>(),
    
    b"Unsupported BMP colormap format\x00".as_ptr() as *const c_char,
    
    b"Only 8- and 24-bit BMP files are supported\x00".as_ptr() as *const c_char,
    
    b"Invalid BMP file: bad header length\x00".as_ptr() as *const c_char,
    
    b"Invalid BMP file: biPlanes not equal to 1\x00".as_ptr() as *const c_char,
    
    b"BMP output must be grayscale or RGB\x00".as_ptr() as *const c_char,
    
    b"Sorry, compressed BMPs not yet supported\x00".as_ptr() as *const c_char,
    
    b"Empty BMP image\x00".as_ptr() as *const c_char,
    
    b"Not a BMP file - does not start with BM\x00".as_ptr() as *const c_char,
    
    b"Numeric value out of range in BMP file\x00".as_ptr() as *const c_char,
    
    b"%ux%u 24-bit BMP image\x00".as_ptr() as *const c_char,
    
    b"%ux%u 8-bit colormapped BMP image\x00".as_ptr() as *const c_char,
    
    b"%ux%u 24-bit OS2 BMP image\x00".as_ptr() as *const c_char,
    
    b"%ux%u 8-bit colormapped OS2 BMP image\x00".as_ptr() as *const c_char,
    
    b"GIF output got confused\x00".as_ptr() as *const c_char,
    
    b"Bogus GIF codesize %d\x00".as_ptr() as *const c_char,
    
    b"GIF output must be grayscale or RGB\x00".as_ptr() as *const c_char,
    
    b"Too few images in GIF file\x00".as_ptr() as *const c_char,
    
    b"Not a GIF file\x00".as_ptr() as *const c_char,
    
    b"%ux%ux%d GIF image\x00".as_ptr() as *const c_char,
    
    b"Warning: unexpected GIF version number \'%c%c%c\'\x00".as_ptr() as *const c_char,
    
    b"Ignoring GIF extension block of type 0x%02x\x00".as_ptr() as *const c_char,
    
    b"Caution: nonsquare pixels in input\x00".as_ptr() as *const c_char,
    
    b"Corrupt data in GIF file\x00".as_ptr() as *const c_char,
    
    b"Bogus char 0x%02x in GIF file, ignoring\x00".as_ptr() as *const c_char,
    
    b"Premature end of GIF image\x00".as_ptr() as *const c_char,
    
    b"Ran out of GIF bits\x00".as_ptr() as *const c_char,
    
    b"PPM output must be grayscale or RGB\x00".as_ptr() as *const c_char,
    
    b"Nonnumeric data in PPM file\x00".as_ptr() as *const c_char,
    
    b"Not a PPM/PGM file\x00".as_ptr() as *const c_char,
    
    b"Numeric value out of range in PPM file\x00".as_ptr() as *const c_char,
    
    b"%ux%u PGM image\x00".as_ptr() as *const c_char,
    
    b"%ux%u text PGM image\x00".as_ptr() as *const c_char,
    
    b"%ux%u PPM image\x00".as_ptr() as *const c_char,
    
    b"%ux%u text PPM image\x00".as_ptr() as *const c_char,
    
    b"Unsupported Targa colormap format\x00".as_ptr() as *const c_char,
    
    b"Invalid or unsupported Targa file\x00".as_ptr() as *const c_char,
    
    b"Targa output must be grayscale or RGB\x00".as_ptr() as *const c_char,
    
    b"%ux%u RGB Targa image\x00".as_ptr() as *const c_char,
    
    b"%ux%u grayscale Targa image\x00".as_ptr() as *const c_char,
    
    b"%ux%u colormapped Targa image\x00".as_ptr() as *const c_char,
    
    b"Color map file is invalid or of unsupported format\x00".as_ptr() as *const c_char,
    
    b"Output file format cannot handle %d colormap entries\x00".as_ptr() as *const c_char,
    
    b"ungetc failed\x00".as_ptr() as *const c_char,
    
    b"MozJPEG can\'t read the image (PNG support is disabled in this build)\x00".as_ptr()
        as *const c_char,
    
    b"Unsupported output file format\x00".as_ptr() as *const c_char,
    NULL as *const c_char,
];
/* so can override from CFLAGS in Makefile */

pub const DEFAULT_FMT: c_int = FMT_PPM as c_int;

static mut requested_fmt: IMAGE_FORMATS = FMT_BMP;
/*
 * Argument-parsing code.
 * The switch parser is designed to be useful with DOS-style command line
 * syntax, ie, intermixed switches and file names, where only the switches
 * to the left of a given file name affect processing of that file.
 * The main program in this file doesn't actually use this capability...
 */

static mut progname: *const c_char = ::std::ptr::null::< c_char>();
/* program name for error messages */

static mut icc_filename: *mut c_char = ::std::ptr::null::< c_char>() as *mut c_char;
/* for -icc switch */

static mut outfilename: *mut c_char = ::std::ptr::null::< c_char>() as *mut c_char;
/* for -outfile switch */
#[no_mangle]

pub static mut memsrc: boolean = 0;
/* for -memsrc switch */
#[no_mangle]

pub static mut skip: boolean = 0;
#[no_mangle]

pub static mut crop: boolean = 0;
#[no_mangle]

pub static mut skip_start: JDIMENSION = 0;
#[no_mangle]

pub static mut skip_end: JDIMENSION = 0;
#[no_mangle]

pub static mut crop_x: JDIMENSION = 0;
#[no_mangle]

pub static mut crop_y: JDIMENSION = 0;
#[no_mangle]

pub static mut crop_width: JDIMENSION = 0;
#[no_mangle]

pub static mut crop_height: JDIMENSION = 0;

pub const INPUT_BUF_SIZE: c_int = 4096i32;

unsafe extern "C" fn usage()
/* complain about bad command line */
{
    fprintf(
        stderr,
        
        b"usage: %s [switches] \x00".as_ptr() as *const c_char,
        progname,
    );
    fprintf(
        stderr,
        
        b"[inputfile]\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"Switches (names may be abbreviated):\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -colors N      Reduce image to no more than N colors\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -fast          Fast, low-quality processing\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -grayscale     Force grayscale output\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -rgb           Force RGB output\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -rgb565        Force RGB565 output\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -scale M/N     Scale output image by fraction M/N, eg, 1/8\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -bmp           Select BMP output format (Windows style)%s\n\x00".as_ptr()
            as *const c_char,
        if DEFAULT_FMT == FMT_BMP as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"  -gif           Select GIF output format%s\n\x00".as_ptr() as *const c_char,
        if DEFAULT_FMT == FMT_GIF as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"  -os2           Select BMP output format (OS/2 style)%s\n\x00".as_ptr()
            as *const c_char,
        if DEFAULT_FMT == FMT_OS2 as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"  -pnm           Select PBMPLUS (PPM/PGM) output format%s\n\x00".as_ptr()
            as *const c_char,
        if DEFAULT_FMT == FMT_PPM as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"  -targa         Select Targa output format%s\n\x00".as_ptr() as *const c_char,
        if DEFAULT_FMT == FMT_TARGA as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"Switches for advanced users:\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -dct int       Use integer DCT method%s\n\x00".as_ptr() as *const c_char,
        if JDCT_DEFAULT == JDCT_ISLOW as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"  -dct fast      Use fast integer DCT (less accurate)%s\n\x00".as_ptr()
            as *const c_char,
        if JDCT_DEFAULT == JDCT_IFAST as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"  -dct float     Use floating-point DCT method%s\n\x00".as_ptr()
            as *const c_char,
        if JDCT_DEFAULT == JDCT_FLOAT as c_int {
            
            b" (default)\x00".as_ptr() as *const c_char
        } else {
            
            b"\x00".as_ptr() as *const c_char
        },
    );
    fprintf(
        stderr,
        
        b"  -dither fs     Use F-S dithering (default)\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -dither none   Don\'t use dithering in quantization\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -dither ordered  Use ordered dither (medium speed, quality)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -icc FILE      Extract ICC profile to FILE\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -map FILE      Map to colors used in named image file\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -nosmooth      Don\'t use high-quality upsampling\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -onepass       Use 1-pass quantization (fast, low quality)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -maxmemory N   Maximum memory to use (in kbytes)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -outfile name  Specify name for output file\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -memsrc        Load input file into memory before decompressing\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -skip Y0,Y1    Decompress all rows except those between Y0 and Y1 (inclusive)\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -crop WxH+X+Y  Decompress only a rectangular subregion of the image\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"                 [requires PBMPLUS (PPM/PGM), GIF, or Targa output format]\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -verbose  or  -debug   Emit debug output\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -version       Print version information and exit\n\x00".as_ptr()
            as *const c_char,
    );
    exit(EXIT_FAILURE);
}

unsafe extern "C" fn parse_switches(
    mut cinfo: j_decompress_ptr,
    mut argc: c_int,
    mut argv: *mut *mut c_char,
    mut last_file_arg_seen: c_int,
    mut for_real: boolean,
) -> c_int
/* Parse optional switches.
 * Returns argv[] index of first file-name argument (== argc if none).
 * Any file names with indexes <= last_file_arg_seen are ignored;
 * they have presumably been processed in a previous iteration.
 * (Pass 0 for last_file_arg_seen on the first or only iteration.)
 * for_real is FALSE on the first (dummy) pass; we may skip any expensive
 * processing.
 */ {
    
     
    /* Set up default JPEG parameters. */
    requested_fmt = DEFAULT_FMT as IMAGE_FORMATS; /* set default output file format */
    icc_filename = NULL as *mut c_char;
    outfilename = NULL as *mut c_char;
    memsrc = FALSE;
    skip = FALSE;
    crop = FALSE;
    (*(*cinfo).err).trace_level = 0i32;
     let mut argn:   c_int =  1i32;
    while argn < argc {
          let mut arg:   *mut c_char =  *argv.offset(argn as isize);
        if *arg as c_int != '-' as i32 {
            /* Not a switch, must be a file name argument */
            if !(argn <= last_file_arg_seen) {
                break; /* -outfile applies to just one input file */
            }
            outfilename = NULL as *mut c_char
        /* ignore this name if previously processed */
        /* else done parsing switches */
        } else {
            arg = arg.offset(1); /* advance past switch marker character */
            if keymatch(
                arg,
                
                b"bmp\x00".as_ptr() as *const c_char,
                1i32,
            ) != 0
            {
                /* BMP output format. */
                requested_fmt = FMT_BMP
            } else if keymatch(
                arg,
                
                b"colors\x00".as_ptr() as *const c_char,
                1i32,
            ) != 0
                || keymatch(
                    arg,
                    
                    b"colours\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                || keymatch(
                    arg,
                    
                    b"quantize\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                || keymatch(
                    arg,
                    
                    b"quantise\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
            {
                 let mut val:  c_int =  0;
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    
                    b"%d\x00".as_ptr() as *const c_char,
                    &mut val as *mut c_int,
                ) != 1i32
                {
                    usage();
                }
                (*cinfo).desired_number_of_colors = val;
                (*cinfo).quantize_colors = TRUE
            } else if keymatch(
                arg,
                
                b"dct\x00".as_ptr() as *const c_char,
                2i32,
            ) != 0
            {
                /* Select IDCT algorithm. */
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if keymatch(
                    *argv.offset(argn as isize),
                    
                    b"int\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                {
                    (*cinfo).dct_method = JDCT_ISLOW
                } else if keymatch(
                    *argv.offset(argn as isize),
                    
                    b"fast\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dct_method = JDCT_IFAST
                } else if keymatch(
                    *argv.offset(argn as isize),
                    
                    b"float\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dct_method = JDCT_FLOAT
                } else {
                    usage();
                }
            } else if keymatch(
                arg,
                
                b"dither\x00".as_ptr() as *const c_char,
                2i32,
            ) != 0
            {
                /* Select dithering algorithm. */
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if keymatch(
                    *argv.offset(argn as isize),
                    
                    b"fs\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dither_mode = JDITHER_FS
                } else if keymatch(
                    *argv.offset(argn as isize),
                    
                    b"none\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dither_mode = JDITHER_NONE
                } else if keymatch(
                    *argv.offset(argn as isize),
                    
                    b"ordered\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    (*cinfo).dither_mode = JDITHER_ORDERED
                } else {
                    usage();
                }
            } else if keymatch(
                arg,
                
                b"debug\x00".as_ptr() as *const c_char,
                1i32,
            ) != 0
                || keymatch(
                    arg,
                    
                    b"verbose\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
            {
                /* Enable debug printouts. */
                /* On first -d, print version identification */
                static mut printed_version: boolean = FALSE;
                if printed_version == 0 {
                    fprintf(
                        stderr,
                        
                        b"%s version %s (build %s)\n\x00".as_ptr() as *const c_char,
                        PACKAGE_NAME.as_ptr(),
                        VERSION.as_ptr(),
                        BUILD.as_ptr(),
                    );
                    fprintf(
                        stderr,
                        
                        b"%s\n\n\x00".as_ptr() as *const c_char,
                        JCOPYRIGHT.as_ptr(),
                    );
                    fprintf(
                        stderr,
                        
                        b"Emulating The Independent JPEG Group\'s software, version %s\n\n\x00".as_ptr() as *const c_char,
                        JVERSION.as_ptr(),
                    );
                    printed_version = TRUE
                }
                (*(*cinfo).err).trace_level += 1
            } else if keymatch(
                arg,
                
                b"version\x00".as_ptr() as *const c_char,
                4i32,
            ) != 0
            {
                fprintf(
                    stderr,
                    
                    b"%s version %s (build %s)\n\x00".as_ptr() as *const c_char,
                    PACKAGE_NAME.as_ptr(),
                    VERSION.as_ptr(),
                    BUILD.as_ptr(),
                );
                exit(EXIT_SUCCESS);
            } else {
                if keymatch(
                    arg,
                    
                    b"fast\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                {
                    /* Select recommended processing options for quick-and-dirty output. */
                    (*cinfo).two_pass_quantize = FALSE;
                    (*cinfo).dither_mode = JDITHER_ORDERED;
                    if (*cinfo).quantize_colors == 0 {
                        /* don't override an earlier -colors */
                        (*cinfo).desired_number_of_colors = 216i32
                    }
                    (*cinfo).dct_method =
                        JDCT_FASTEST as J_DCT_METHOD;
                    (*cinfo).do_fancy_upsampling = FALSE
                } else if keymatch(
                    arg,
                    
                    b"gif\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                {
                    /* GIF output format. */
                    requested_fmt = FMT_GIF
                } else if keymatch(
                    arg,
                    
                    b"grayscale\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                    || keymatch(
                        arg,
                        
                        b"greyscale\x00".as_ptr() as *const c_char,
                        2i32,
                    ) != 0
                {
                    /* Force monochrome output. */
                    (*cinfo).out_color_space = JCS_GRAYSCALE
                } else if keymatch(
                    arg,
                    
                    b"rgb\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    /* Force RGB output. */
                    (*cinfo).out_color_space = JCS_RGB
                } else if keymatch(
                    arg,
                    
                    b"rgb565\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    /* Force RGB565 output. */
                    (*cinfo).out_color_space = JCS_RGB565
                } else if keymatch(
                    arg,
                    
                    b"icc\x00".as_ptr() as *const c_char,
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
                    jpeg_save_markers(
                        cinfo,
                        JPEG_APP0 + 2i32,
                        0xffffu32,
                    );
                } else if keymatch(
                    arg,
                    
                    b"map\x00".as_ptr() as *const c_char,
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
                         
                         let mut mapfile:   *mut FILE =
     fopen(
                            *argv.offset(argn as isize),
                            READ_BINARY.as_ptr(),
                        );
                        if mapfile.is_null() {
                            fprintf(
                                stderr,
                                
                                b"%s: can\'t open %s\n\x00".as_ptr() as *const c_char,
                                progname,
                                *argv.offset(argn as isize),
                            );
                            exit(EXIT_FAILURE);
                        }
                        read_color_map(cinfo, mapfile);
                        fclose(mapfile);
                        (*cinfo).quantize_colors = TRUE
                    }
                } else if keymatch(
                    arg,
                    
                    b"maxmemory\x00".as_ptr() as *const c_char,
                    3i32,
                ) != 0
                {
                    
                     let mut lval:  c_long =  0; let mut ch:  c_char =   'x' as c_char;
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if sscanf(
                        *argv.offset(argn as isize),
                        
                        b"%ld%c\x00".as_ptr() as *const c_char,
                        &mut lval as *mut c_long,
                        &mut ch as *mut c_char,
                    ) < 1i32
                    {
                        usage();
                    }
                    if ch as c_int == 'm' as i32 || ch as c_int == 'M' as i32 {
                        lval *= 1000i64
                    }
                    (*(*cinfo).mem).max_memory_to_use = lval * 1000i64
                } else if keymatch(
                    arg,
                    
                    b"nosmooth\x00".as_ptr() as *const c_char,
                    3i32,
                ) != 0
                {
                    /* Suppress fancy upsampling */
                    (*cinfo).do_fancy_upsampling = FALSE
                } else if keymatch(
                    arg,
                    
                    b"onepass\x00".as_ptr() as *const c_char,
                    3i32,
                ) != 0
                {
                    /* Use fast one-pass quantization. */
                    (*cinfo).two_pass_quantize = FALSE
                } else if keymatch(
                    arg,
                    
                    b"os2\x00".as_ptr() as *const c_char,
                    3i32,
                ) != 0
                {
                    /* BMP output format (OS/2 flavor). */
                    requested_fmt = FMT_OS2
                } else if keymatch(
                    arg,
                    
                    b"outfile\x00".as_ptr() as *const c_char,
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
                } else if keymatch(
                    arg,
                    
                    b"memsrc\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    /* Use in-memory source manager */
                    memsrc = TRUE
                } else if keymatch(
                    arg,
                    
                    b"pnm\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                    || keymatch(
                        arg,
                        
                        b"ppm\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                {
                    /* PPM/PGM output format. */
                    requested_fmt = FMT_PPM
                } else if keymatch(
                    arg,
                    
                    b"rle\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                {
                    /* RLE output format. */
                    requested_fmt = FMT_RLE
                } else if keymatch(
                    arg,
                    
                    b"scale\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    /* Scale the output image by a fraction M/N. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if sscanf(
                        *argv.offset(argn as isize),
                        
                        b"%u/%u\x00".as_ptr() as *const c_char,
                        &mut (*cinfo).scale_num as *mut c_uint,
                        &mut (*cinfo).scale_denom as *mut c_uint,
                    ) != 2i32
                    {
                        usage();
                    }
                } else if keymatch(
                    arg,
                    
                    b"skip\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if sscanf(
                        *argv.offset(argn as isize),
                        
                        b"%u,%u\x00".as_ptr() as *const c_char,
                        &mut skip_start as *mut JDIMENSION,
                        &mut skip_end as *mut JDIMENSION,
                    ) != 2i32
                        || skip_start > skip_end
                    {
                        usage();
                    }
                    skip = TRUE
                } else if keymatch(
                    arg,
                    
                    b"crop\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                     let mut c:  c_char =  0;
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if sscanf(
                        *argv.offset(argn as isize),
                        
                        b"%u%c%u+%u+%u\x00".as_ptr() as *const c_char,
                        &mut crop_width as *mut JDIMENSION,
                        &mut c as *mut c_char,
                        &mut crop_height as *mut JDIMENSION,
                        &mut crop_x as *mut JDIMENSION,
                        &mut crop_y as *mut JDIMENSION,
                    ) != 5i32
                        || c as c_int != 'X' as i32 && c as c_int != 'x' as i32
                        || crop_width < 1u32
                        || crop_height < 1u32
                    {
                        usage();
                    }
                    crop = TRUE
                } else if keymatch(
                    arg,
                    
                    b"targa\x00".as_ptr() as *const c_char,
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

unsafe extern "C" fn jpeg_getc(mut cinfo: j_decompress_ptr) -> c_uint
/* Read next byte */ {
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src; /* discount the length word itself */
    if (*datasrc).bytes_in_buffer == 0u64 {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = JERR_CANT_SUSPEND as c_int;
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
    (*datasrc).bytes_in_buffer =  (*datasrc).bytes_in_buffer - 1;
    let fresh0 = (*datasrc).next_input_byte;
    (*datasrc).next_input_byte = (*datasrc).next_input_byte.offset(1);
    return *fresh0 as c_uint;
}

unsafe extern "C" fn print_text_marker(
    mut cinfo: j_decompress_ptr,
) -> boolean {
     let mut traceit: boolean =
        ((*(*cinfo).err).trace_level >= 1i32) as c_int;
    
    
    
     let mut length:   c_long =  (jpeg_getc(cinfo) << 8i32) as c_long;
    length += jpeg_getc(cinfo) as c_long;
    length -= 2i64;
    if traceit != 0 {
        if (*cinfo).unread_marker == JPEG_COM {
            fprintf(
                stderr,
                
                b"Comment, length %ld:\n\x00".as_ptr() as *const c_char,
                length,
            );
        } else {
            /* assume it is an APPn otherwise */
            fprintf(
                stderr,
                
                b"APP%d, length %ld:\n\x00".as_ptr() as *const c_char,
                (*cinfo).unread_marker - JPEG_APP0,
                length,
            );
        }
    }
    loop {
         length -= 1;
        if !(length >= 0i64) {
            break;
        }
         let mut ch:   c_uint =  jpeg_getc(cinfo);
        if traceit != 0 {
            /* Emit the character in a readable form.
             * Nonprintables are converted to \nnn form,
             * while \ is converted to \\.
             * Newlines in CR, CR/LF, or LF form will be printed as one newline.
             */
             let mut lastch:  c_uint =  0u32;if ch ==  '\r' as c_uint {
                fprintf(
                    stderr,
                    
                    b"\n\x00".as_ptr() as *const c_char,
                );
            } else if ch ==  '\n' as c_uint {
                if lastch !=  '\r' as c_uint {
                    fprintf(
                        stderr,
                        
                        b"\n\x00".as_ptr() as *const c_char,
                    );
                }
            } else if ch ==  '\\' as c_uint {
                fprintf(
                    stderr,
                    
                    b"\\\\\x00".as_ptr() as *const c_char,
                );
            } else if *(*__ctype_b_loc()).offset(ch as c_int as isize)
                as c_int
                &  _ISprint as c_ushort as c_int
                != 0
            {
                putc(ch as c_int, stderr);
            } else {
                fprintf(
                    stderr,
                    
                    b"\\%03o\x00".as_ptr() as *const c_char,
                    ch,
                );
            }
            lastch = ch
        }
    }
    if traceit != 0 {
        fprintf(
            stderr,
            
            b"\n\x00".as_ptr() as *const c_char,
        );
    }
    return TRUE;
}
/*
 * The main program.
 */

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
      let mut input_file:  *mut FILE =
     ::std::ptr::null_mut::< FILE>(); let mut output_file:  *mut FILE =
     ::std::ptr::null_mut::< FILE>(); let mut num_scanlines:  JDIMENSION =  0;let mut cinfo: jpeg_decompress_struct =
        jpeg_decompress_struct{err:  ::std::ptr::null_mut::< jpeg_error_mgr>(),
                       mem:  ::std::ptr::null_mut::< jpeg_memory_mgr>(),
                       progress:  ::std::ptr::null_mut::< jpeg_progress_mgr>(),
                       client_data:  ::std::ptr::null_mut::< c_void>(),
                       is_decompressor:  0,
                       global_state:  0,
                       src:  ::std::ptr::null_mut::< jpeg_source_mgr>(),
                       image_width:  0,
                       image_height:  0,
                       num_components:  0,
                       jpeg_color_space:  JCS_UNKNOWN,
                       out_color_space:  JCS_UNKNOWN,
                       scale_num:  0,
                       scale_denom:  0,
                       output_gamma:  0.,
                       buffered_image:  0,
                       raw_data_out:  0,
                       dct_method:  JDCT_ISLOW,
                       do_fancy_upsampling:  0,
                       do_block_smoothing:  0,
                       quantize_colors:  0,
                       dither_mode:  JDITHER_NONE,
                       two_pass_quantize:  0,
                       desired_number_of_colors:  0,
                       enable_1pass_quant:  0,
                       enable_external_quant:  0,
                       enable_2pass_quant:  0,
                       output_width:  0,
                       output_height:  0,
                       out_color_components:  0,
                       output_components:  0,
                       rec_outbuf_height:  0,
                       actual_number_of_colors:  0,
                       colormap:  ::std::ptr::null_mut::< JSAMPROW>(),
                       output_scanline:  0,
                       input_scan_number:  0,
                       input_iMCU_row:  0,
                       output_scan_number:  0,
                       output_iMCU_row:  0,
                       coef_bits:  ::std::ptr::null_mut::< [c_int; 64]>(),
                       quant_tbl_ptrs:
                            [::std::ptr::null_mut::< JQUANT_TBL>(); 4],
                       dc_huff_tbl_ptrs:
                            [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                       ac_huff_tbl_ptrs:
                            [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                       data_precision:  0,
                       comp_info:
                            ::std::ptr::null_mut::< jpeg_component_info>(),
                       progressive_mode:  0,
                       arith_code:  0,
                       arith_dc_L:  [0; 16],
                       arith_dc_U:  [0; 16],
                       arith_ac_K:  [0; 16],
                       restart_interval:  0,
                       saw_JFIF_marker:  0,
                       JFIF_major_version:  0,
                       JFIF_minor_version:  0,
                       density_unit:  0,
                       X_density:  0,
                       Y_density:  0,
                       saw_Adobe_marker:  0,
                       Adobe_transform:  0,
                       CCIR601_sampling:  0,
                       marker_list:
                            ::std::ptr::null_mut::< jpeg_marker_struct>(),
                       max_h_samp_factor:  0,
                       max_v_samp_factor:  0,
                       min_DCT_scaled_size:  0,
                       total_iMCU_rows:  0,
                       sample_range_limit:  ::std::ptr::null_mut::< JSAMPLE>(),
                       comps_in_scan:  0,
                       cur_comp_info:
                            [::std::ptr::null_mut::< jpeg_component_info>(); 4],
                       MCUs_per_row:  0,
                       MCU_rows_in_scan:  0,
                       blocks_in_MCU:  0,
                       MCU_membership:  [0; 10],
                       Ss:  0,
                       Se:  0,
                       Ah:  0,
                       Al:  0,
                       unread_marker:  0,
                       master:  ::std::ptr::null_mut::< jpeg_decomp_master>(),
                       main:  ::std::ptr::null_mut::< jpeg_d_main_controller>(),
                       coef:  ::std::ptr::null_mut::< jpeg_d_coef_controller>(),
                       post:  ::std::ptr::null_mut::< jpeg_d_post_controller>(),
                       inputctl:
                            ::std::ptr::null_mut::< jpeg_input_controller>(),
                       marker:  ::std::ptr::null_mut::< jpeg_marker_reader>(),
                       entropy:
                            ::std::ptr::null_mut::< jpeg_entropy_decoder>(),
                       idct:  ::std::ptr::null_mut::< jpeg_inverse_dct>(),
                       upsample:  ::std::ptr::null_mut::< jpeg_upsampler>(),
                       cconvert:
                            ::std::ptr::null_mut::< jpeg_color_deconverter>(),
                       cquantize:
                            ::std::ptr::null_mut::< jpeg_color_quantizer>(),};
    let mut jerr: jpeg_error_mgr = jpeg_error_mgr{error_exit:  None,
               emit_message:  None,
               output_message:  None,
               format_message:  None,
               reset_error_mgr:  None,
               msg_code:  0,
               msg_parm:  C2RustUnnamed_2{i:  [0; 8],},
               trace_level:  0,
               num_warnings:  0,
               jpeg_message_table:  ::std::ptr::null::< *const c_char>(),
               last_jpeg_message:  0,
               addon_message_table:  ::std::ptr::null::< *const c_char>(),
               first_addon_message:  0,
               last_addon_message:  0,};
    
    let mut dest_mgr: djpeg_dest_ptr =
        NULL as djpeg_dest_ptr;
    
    
    let mut inbuffer: *mut c_uchar = NULL as *mut c_uchar;
    
    
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0); /* in case C library doesn't provide it */
    if progname.is_null() || *progname.offset(0) as c_int == 0i32 {
        progname =  b"djpeg\x00".as_ptr() as *const c_char
    }
    /* Initialize the JPEG decompression object with default error handling. */
    cinfo.err = jpeg_std_error(&mut jerr);
    jpeg_CreateDecompress(
        &mut cinfo,
        JPEG_LIB_VERSION,
        ::std::mem::size_of::<jpeg_decompress_struct>() as c_ulong,
    );
    /* Add some application-specific error messages (from cderror.h) */
    jerr.addon_message_table = cdjpeg_message_table.as_ptr();
    jerr.first_addon_message = JMSG_FIRSTADDONCODE as c_int;
    jerr.last_addon_message = JMSG_LASTADDONCODE as c_int;
    /* Insert custom marker processor for COM and APP12.
     * APP12 is used by some digital camera makers for textual info,
     * so we provide the ability to display it as text.
     * If you like, additional APPn marker types can be selected for display,
     * but don't try to override APP0 or APP14 this way (see libjpeg.txt).
     */
    jpeg_set_marker_processor(
        &mut cinfo,
        JPEG_COM,
        Some(
            print_text_marker
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                ) -> boolean,
        ),
    );
    jpeg_set_marker_processor(
        &mut cinfo,
        JPEG_APP0 + 12i32,
        Some(
            print_text_marker
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                ) -> boolean,
        ),
    );
    /* Scan command line to find file names. */
    /* It is convenient to use just one switch-parsing routine, but the switch
     * values read here are ignored; we will rescan the switches after opening
     * the input file.
     * (Exception: tracing level set here controls verbosity for COM markers
     * found during jpeg_read_header...)
     */
     let mut file_index:   c_int =
     parse_switches(&mut cinfo, argc, argv, 0i32, FALSE);
    /* Unix style: expect zero or one file name */
    if file_index < argc - 1i32 {
        fprintf(
            stderr,
            
            b"%s: only one input file\n\x00".as_ptr() as *const c_char,
            progname,
        );
        usage();
    }
    /* TWO_FILE_COMMANDLINE */
    /* Open the input file. */
    if file_index < argc {
        input_file = fopen(
            *argv.offset(file_index as isize),
            READ_BINARY.as_ptr(),
        );
        if input_file.is_null() {
            fprintf(
                stderr,
                
                b"%s: can\'t open %s\n\x00".as_ptr() as *const c_char,
                progname,
                *argv.offset(file_index as isize),
            );
            exit(EXIT_FAILURE);
        }
    } else {
        /* default input file is stdin */
        input_file = read_stdin()
    }
    /* Open the output file. */
    if !outfilename.is_null() {
        output_file = fopen(outfilename, WRITE_BINARY.as_ptr());
        if output_file.is_null() {
            fprintf(
                stderr,
                
                b"%s: can\'t open %s\n\x00".as_ptr() as *const c_char,
                progname,
                outfilename,
            );
            exit(EXIT_FAILURE);
        }
    } else {
        /* default output file is stdout */
        output_file = write_stdout()
    }
    /* Specify data source for decompression */
    if memsrc != 0 {
         let mut insize:  c_ulong =  0u64;
        loop {
             inbuffer = realloc(
                inbuffer as *mut c_void,
                
                insize + INPUT_BUF_SIZE as c_ulong,
            ) as *mut c_uchar;
            if inbuffer.is_null() {
                fprintf(
                    stderr,
                    
                    b"%s: memory allocation failure\n\x00".as_ptr() as *const c_char,
                    progname,
                );
                exit(EXIT_FAILURE);
            }
             let mut nbytes:   size_t =
     fread(
                &mut *inbuffer.offset(insize as isize) as *mut c_uchar as *mut c_void,
                1u64,
                4096u64,
                input_file,
            );
            if nbytes < INPUT_BUF_SIZE as c_ulong && ferror(input_file) != 0 {
                if file_index < argc {
                    fprintf(
                        stderr,
                        
                        b"%s: can\'t read from %s\n\x00".as_ptr() as *const c_char,
                        progname,
                        *argv.offset(file_index as isize),
                    );
                } else {
                    fprintf(
                        stderr,
                        
                        b"%s: can\'t read from stdin\n\x00".as_ptr() as *const c_char,
                        progname,
                    );
                }
            }
            insize +=  nbytes;
            if !(nbytes == INPUT_BUF_SIZE as c_ulong) {
                break;
            }
        }
        fprintf(
            stderr,
            
            b"Compressed size:  %lu bytes\n\x00".as_ptr() as *const c_char,
            insize,
        );
        jpeg_mem_src(&mut cinfo, inbuffer, insize);
    } else {
        jpeg_stdio_src(&mut cinfo, input_file);
    }
    /* Read file header, set default decompression parameters */
    jpeg_read_header(&mut cinfo, TRUE);
    /* Adjust default decompression parameters by re-parsing the options */
    file_index = parse_switches(&mut cinfo, argc, argv, 0i32, TRUE);
    /* Initialize the output module now to let it override any crucial
     * option settings (for instance, GIF wants to force color quantization).
     */
    match  requested_fmt {
        0 => {
            dest_mgr = jinit_write_bmp(
                &mut cinfo,
                FALSE,
                TRUE,
            )
        }
        2 => {
            dest_mgr = jinit_write_bmp(
                &mut cinfo,
                TRUE,
                TRUE,
            )
        }
        1 => dest_mgr = jinit_write_gif(&mut cinfo),
        3 => dest_mgr = jinit_write_ppm(&mut cinfo),
        5 => dest_mgr = jinit_write_targa(&mut cinfo),
        _ => {
            (*cinfo.err).msg_code = JERR_UNSUPPORTED_FORMAT as c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut cinfo as *mut jpeg_decompress_struct
                    as j_common_ptr,
            );
        }
    }
    (*dest_mgr).output_file = output_file;
    /* Start decompressor */
    jpeg_start_decompress(&mut cinfo);
    /* Skip rows */
    if skip != 0 {
         
        /* Decompress a subregion */
        if skip_end >  cinfo.output_height - 1u32 {
            fprintf(
                stderr,
                
                b"%s: skip region exceeds image height %d\n\x00".as_ptr()
                    as *const c_char,
                progname,
                cinfo.output_height,
            );
            exit(EXIT_FAILURE);
        }
         let mut tmp:   JDIMENSION =  cinfo.output_height;
        cinfo.output_height = cinfo.output_height -
    (
            skip_end - skip_start + 1u32);
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
            num_scanlines = jpeg_read_scanlines(
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
        jpeg_skip_scanlines(
            &mut cinfo,
            
            skip_end - skip_start + 1u32,
        );
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = jpeg_read_scanlines(
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
         
        /* Normal full-image decompress */
        if  crop_x + crop_width > cinfo.output_width
            ||  crop_y + crop_height > cinfo.output_height
        {
            fprintf(
                stderr,
                
                b"%s: crop dimensions exceed image dimensions %d x %d\n\x00".as_ptr()
                    as *const c_char,
                progname,
                cinfo.output_width,
                cinfo.output_height,
            );
            exit(EXIT_FAILURE);
        }
        jpeg_crop_scanline(&mut cinfo, &mut crop_x, &mut crop_width);
        if (*dest_mgr).calc_buffer_dimensions.is_some() {
            Some(
                (*dest_mgr)
                    .calc_buffer_dimensions
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        } else {
            (*cinfo.err).msg_code = JERR_UNSUPPORTED_FORMAT as c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut cinfo as *mut jpeg_decompress_struct
                    as j_common_ptr,
            );
        }
         let mut tmp_0:   JDIMENSION =  cinfo.output_height;
        cinfo.output_height = crop_height;
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        cinfo.output_height = tmp_0;
        jpeg_skip_scanlines(&mut cinfo, crop_y);
        while cinfo.output_scanline <  crop_y + crop_height {
            num_scanlines = jpeg_read_scanlines(
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
        jpeg_skip_scanlines(
            &mut cinfo,
            
            cinfo
                .output_height - crop_y - crop_height,
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
            num_scanlines = jpeg_read_scanlines(
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
        
        
          let mut icc_profile:  *mut JOCTET =
     ::std::ptr::null_mut::< JOCTET>(); let mut icc_len:  c_uint =  0;
         let mut icc_file:   *mut FILE =
     fopen(icc_filename, WRITE_BINARY.as_ptr());
        if icc_file.is_null() {
            fprintf(
                stderr,
                
                b"%s: can\'t open %s\n\x00".as_ptr() as *const c_char,
                progname,
                icc_filename,
            );
            exit(EXIT_FAILURE);
        }
        if jpeg_read_icc_profile(&mut cinfo, &mut icc_profile, &mut icc_len) != 0
        {
            if fwrite(
                icc_profile as *const c_void,
                icc_len as c_ulong,
                1u64,
                icc_file,
            ) < 1u64
            {
                fprintf(
                    stderr,
                    
                    b"%s: can\'t read ICC profile from %s\n\x00".as_ptr()
                        as *const c_char,
                    progname,
                    icc_filename,
                );
                free(icc_profile as *mut c_void);
                fclose(icc_file);
                exit(EXIT_FAILURE);
            }
            free(icc_profile as *mut c_void);
            fclose(icc_file);
        } else if (*cinfo.err).msg_code != JWRN_BOGUS_ICC as c_int {
            fprintf(
                stderr,
                
                b"%s: no ICC profile data in JPEG file\n\x00".as_ptr() as *const c_char,
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
    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);
    /* Close files, if we opened them */
    if input_file != stdin {
        fclose(input_file);
    }
    if output_file != stdout {
        fclose(output_file);
    }
    if memsrc != 0 && !inbuffer.is_null() {
        free(inbuffer as *mut c_void);
    }
    /* All done. */
    exit(if jerr.num_warnings != 0 {
        EXIT_WARNING
    } else {
        EXIT_SUCCESS
    });
    /* suppress no-return-value warnings */
}
#[main]
pub fn main() {
     let mut args:  Vec<*mut c_char> =  Vec::new();
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
            (args.len() - 1) as c_int,
            
            args.as_mut_ptr(),
        ))
    }
}
