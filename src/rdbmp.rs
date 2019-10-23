use libc::c_uchar;use libc::c_ulong;use libc::c_int;use libc::c_long;use libc::c_ushort;use libc::c_ulonglong;use libc::c_void;use libc::c_uint;use libc;

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:29"]
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
pub use crate::cmyk_h::rgb_to_cmyk;
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
pub use super::cdjpeg::cd_progress_ptr;
pub use super::cdjpeg::cdjpeg_progress_mgr;
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
pub use crate::stdlib::feof;
pub use crate::stdlib::fread;
pub use crate::stdlib::getc;
use crate::stdlib::memcpy;
pub use crate::stdlib::EOF;
/* Private version of data source object */

pub type bmp_source_ptr = *mut _bmp_source_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bmp_source_struct {
    pub pub_0: super::cdjpeg::cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub colormap: JSAMPARRAY,
    pub whole_image: jvirt_sarray_ptr,
    pub source_row: JDIMENSION,
    pub row_width: JDIMENSION,
    pub bits_per_pixel: c_int,
    pub cmap_length: c_int,
    pub use_inversion_array: boolean,
    pub iobuffer: *mut U_CHAR,
}
/*
 * rdbmp.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * Modified 2009-2017 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Modified 2011 by Siarhei Siamashka.
 * Copyright (C) 2015, 2017-2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to read input images in Microsoft "BMP"
 * format (MS Windows 3.x, OS/2 1.x, and OS/2 2.x flavors).
 * Currently, only 8-bit and 24-bit images are supported, not 1-bit or
 * 4-bit (feeding such low-depth images into JPEG would be silly anyway).
 * Also, we don't support RLE-compressed files.
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume input from
 * an ordinary stdio stream.  They further assume that reading begins
 * at the start of the file; start_input may need work if the
 * user interface has already read some data (e.g., to determine that
 * the file is indeed BMP format).
 *
 * This code contributed by James Arthur Boucher.
 */
/* Macros to deal with unsigned chars as efficiently as compiler allows */

pub type U_CHAR = c_uchar;

pub type bmp_source_struct = _bmp_source_struct;
/* I/O buffer (used to buffer a single row from
disk if use_inversion_array == FALSE) */
/* !HAVE_UNSIGNED_CHAR */
/* HAVE_UNSIGNED_CHAR */

static mut alpha_index: [c_int; 17] = [
    -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, 3i32, 3i32,
    0i32, 0i32, -1i32,
];

unsafe extern "C" fn read_byte(mut sinfo: bmp_source_ptr) -> c_int
/* Read next byte from BMP file */ {
     let mut infile: *mut FILE = (*sinfo).pub_0.input_file;
    
     let mut c:   c_int =  getc(infile);
    if c == EOF {
        (*(*(*sinfo).cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*sinfo).cinfo as j_common_ptr
        );
    }
    return c;
}

unsafe extern "C" fn read_colormap(
    mut sinfo: bmp_source_ptr,
    mut cmaplen: c_int,
    mut mapentrysize: c_int,
)
/* Read the colormap from a BMP file */
{
    
     let mut i:  c_int =  0; let mut gray:  c_int =  1i32;
    match mapentrysize {
        3 => {
            /* BGR format (occurs in OS/2 files) */
            i = 0i32;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(1)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(0)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                if *(*(*sinfo).colormap.offset(2)).offset(i as isize) as c_int
                    != *(*(*sinfo).colormap.offset(1)).offset(i as isize) as c_int
                    || *(*(*sinfo).colormap.offset(1)).offset(i as isize) as c_int
                        != *(*(*sinfo).colormap.offset(0)).offset(i as isize) as c_int
                {
                    gray = 0i32
                }
                i += 1
            }
        }
        4 => {
            /* BGR0 format (occurs in MS Windows files) */
            i = 0i32;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(1)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(0)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                read_byte(sinfo);
                if *(*(*sinfo).colormap.offset(2)).offset(i as isize) as c_int
                    != *(*(*sinfo).colormap.offset(1)).offset(i as isize) as c_int
                    || *(*(*sinfo).colormap.offset(1)).offset(i as isize) as c_int
                        != *(*(*sinfo).colormap.offset(0)).offset(i as isize) as c_int
                {
                    gray = 0i32
                }
                i += 1
            }
        }
        _ => {
            (*(*(*sinfo).cinfo).err).msg_code = JERR_BMP_BADCMAP as c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*sinfo).cinfo as j_common_ptr
            );
        }
    }
    if  (*(*sinfo).cinfo).in_color_space
        ==  JCS_UNKNOWN
        && gray != 0
    {
        (*(*sinfo).cinfo).in_color_space = JCS_GRAYSCALE
    }
    if  (*(*sinfo).cinfo).in_color_space
        ==  JCS_GRAYSCALE
        && gray == 0
    {
        (*(*(*sinfo).cinfo).err).msg_code =
            super::jerror::JERR_BAD_IN_COLORSPACE as c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*sinfo).cinfo as j_common_ptr
        );
    };
}
/*
 * Read one row of pixels.
 * The image has been read into the whole_image array, but is otherwise
 * unprocessed.  We must read it out in top-to-bottom row order, and if
 * it is an 8-bit image, we must expand colormapped pixels to 24bit format.
 */

unsafe extern "C" fn get_8bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading 8-bit colormap indexes */ {
     let mut t:  c_int =  0; let mut inptr:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>();  let mut col:  JDIMENSION =  0;let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut colormap: JSAMPARRAY = (*source).colormap;
    let mut cmaplen: c_int = (*source).cmap_length;
    
    
    
    
    
    if (*source).use_inversion_array != 0 {
        (*source).source_row =  (*source).source_row - 1;
         let mut image_ptr:   JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1u32,
            FALSE,
        );
        inptr = *image_ptr.offset(0)
    } else {
        if !(fread(
            (*source).iobuffer as *mut c_void,
            1u64,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as size_t)
        {
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
        inptr = (*source).iobuffer
    }
     let mut outptr:   JSAMPROW =
     *(*source).pub_0.buffer.offset(0);
    if  (*cinfo).in_color_space
        ==  JCS_GRAYSCALE
    {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            t = *fresh0 as c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            let fresh1 = outptr;
            outptr = outptr.offset(1);
            *fresh1 = *(*colormap.offset(0)).offset(t as isize);
            col -=  1
        }
    } else if  (*cinfo).in_color_space
        ==  JCS_CMYK
    {
        col = (*cinfo).image_width;
        while col > 0u32 {
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            t = *fresh2 as c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            rgb_to_cmyk(
                *(*colormap.offset(0)).offset(t as isize),
                *(*colormap.offset(1)).offset(t as isize),
                *(*colormap.offset(2)).offset(t as isize),
                outptr,
                outptr.offset(1),
                outptr.offset(2),
                outptr.offset(3),
            );
            outptr = outptr.offset(4);
            col -=  1
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: c_int =
            rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh3 = inptr;
                inptr = inptr.offset(1);
                t = *fresh3 as c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                *outptr.offset(rindex as isize) = *(*colormap.offset(0)).offset(t as isize);
                *outptr.offset(gindex as isize) = *(*colormap.offset(1)).offset(t as isize);
                *outptr.offset(bindex as isize) = *(*colormap.offset(2)).offset(t as isize);
                *outptr.offset(aindex as isize) = 0xffu8;
                outptr = outptr.offset(ps as isize);
                col -=  1
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh4 = inptr;
                inptr = inptr.offset(1);
                t = *fresh4 as c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                *outptr.offset(rindex as isize) = *(*colormap.offset(0)).offset(t as isize);
                *outptr.offset(gindex as isize) = *(*colormap.offset(1)).offset(t as isize);
                *outptr.offset(bindex as isize) = *(*colormap.offset(2)).offset(t as isize);
                outptr = outptr.offset(ps as isize);
                col -=  1
            }
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_24bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading 24-bit pixels */ {
     let mut inptr:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>();  let mut col:  JDIMENSION =  0;let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    
    
    
    
    if (*source).use_inversion_array != 0 {
        (*source).source_row =  (*source).source_row - 1;
         let mut image_ptr:   JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1u32,
            FALSE,
        );
        inptr = *image_ptr.offset(0)
    } else {
        if !(fread(
            (*source).iobuffer as *mut c_void,
            1u64,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as size_t)
        {
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
        inptr = (*source).iobuffer
    }
    /* Transfer data.  Note source values are in BGR order
     * (even though Microsoft's own documents say the opposite).
     */
     let mut outptr:   JSAMPROW =
     *(*source).pub_0.buffer.offset(0);
    if  (*cinfo).in_color_space
        ==  JCS_EXT_BGR
    {
        memcpy(
            outptr as *mut c_void,
            inptr as *const c_void,
            (*source).row_width as size_t,
        );
    } else if  (*cinfo).in_color_space
        ==  JCS_CMYK
    {
        col = (*cinfo).image_width;
        while col > 0u32 {
            /* can omit GETJSAMPLE() safely */
            let fresh5 = inptr; /* can omit GETJSAMPLE() safely */
            inptr = inptr.offset(1); /* can omit GETJSAMPLE() safely */
            let mut b: JSAMPLE = *fresh5;
            let fresh6 = inptr;
            inptr = inptr.offset(1);
            let mut g: JSAMPLE = *fresh6;
            let fresh7 = inptr;
            inptr = inptr.offset(1);
            let mut r: JSAMPLE = *fresh7;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr,
                outptr.offset(1),
                outptr.offset(2),
                outptr.offset(3),
            );
            outptr = outptr.offset(4);
            col -=  1
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: c_int =
            rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh8 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh8;
                let fresh9 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh9;
                let fresh10 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh10;
                *outptr.offset(aindex as isize) = 0xffu8;
                outptr = outptr.offset(ps as isize);
                col -=  1
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh11 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh11;
                let fresh12 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh12;
                let fresh13 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh13;
                outptr = outptr.offset(ps as isize);
                col -=  1
            }
        }
    }
    return 1u32;
}

unsafe extern "C" fn get_32bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading 32-bit pixels */ {
     let mut inptr:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>();  let mut col:  JDIMENSION =  0;let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    
    
    
    
    if (*source).use_inversion_array != 0 {
        (*source).source_row =  (*source).source_row - 1;
         let mut image_ptr:   JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1u32,
            FALSE,
        );
        inptr = *image_ptr.offset(0)
    } else {
        if !(fread(
            (*source).iobuffer as *mut c_void,
            1u64,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as size_t)
        {
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
        inptr = (*source).iobuffer
    }
    /* Transfer data.  Note source values are in BGR order
     * (even though Microsoft's own documents say the opposite).
     */
     let mut outptr:   JSAMPROW =
     *(*source).pub_0.buffer.offset(0);
    if  (*cinfo).in_color_space
        ==  JCS_EXT_BGRX
        ||  (*cinfo).in_color_space
            ==  JCS_EXT_BGRA
    {
        memcpy(
            outptr as *mut c_void,
            inptr as *const c_void,
            (*source).row_width as size_t,
        );
    } else if  (*cinfo).in_color_space
        ==  JCS_CMYK
    {
        col = (*cinfo).image_width;
        while col > 0u32 {
            /* can omit GETJSAMPLE() safely */
            let fresh14 = inptr; /* skip the 4th byte (Alpha channel) */
            inptr = inptr.offset(1); /* can omit GETJSAMPLE() safely */
            let mut b: JSAMPLE = *fresh14; /* can omit GETJSAMPLE() safely */
            let fresh15 = inptr; /* skip the 4th byte (Alpha channel) */
            inptr = inptr.offset(1);
            let mut g: JSAMPLE = *fresh15;
            let fresh16 = inptr;
            inptr = inptr.offset(1);
            let mut r: JSAMPLE = *fresh16;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr,
                outptr.offset(1),
                outptr.offset(2),
                outptr.offset(3),
            );
            inptr = inptr.offset(1);
            outptr = outptr.offset(4);
            col -=  1
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: c_int =
            rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh17 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh17;
                let fresh18 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh18;
                let fresh19 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh19;
                let fresh20 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(aindex as isize) = *fresh20;
                outptr = outptr.offset(ps as isize);
                col -=  1
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0u32 {
                let fresh21 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh21;
                let fresh22 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh22;
                let fresh23 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh23;
                inptr = inptr.offset(1);
                outptr = outptr.offset(ps as isize);
                col -=  1
            }
        }
    }
    return 1u32;
}
/*
 * This method loads the image into whole_image during the first call on
 * get_pixel_rows.  The get_pixel_rows pointer is then adjusted to call
 * get_8bit_row, get_24bit_row, or get_32bit_row on subsequent calls.
 */

unsafe extern "C" fn preload_image(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION {
     let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    
    
    
    let mut progress: super::cdjpeg::cd_progress_ptr =
        (*cinfo).progress as super::cdjpeg::cd_progress_ptr;
     let mut row:   JDIMENSION =  0u32;
    while row < (*cinfo).image_height {
          if !progress.is_null() {
            (*progress).pub_0.pass_counter = row as c_long;
            (*progress).pub_0.pass_limit = (*cinfo).image_height as c_long;
            Some(
                (*progress)
                    .pub_0
                    .progress_monitor
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        
         let mut image_ptr:   JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            row,
            1u32,
            TRUE,
        ); let mut out_ptr:   JSAMPROW =  *image_ptr.offset(0);
        if fread(
            out_ptr as *mut c_void,
            1u64,
            (*source).row_width as c_ulong,
            infile,
        ) != (*source).row_width as c_ulong
        {
            if feof(infile) != 0 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            } else {
                (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_READ as c_int;
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
        row +=  1
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1
    }
    /* Set up to read from the virtual array in top-to-bottom order */
    match (*source).bits_per_pixel {
        8 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_8bit_row
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: super::cdjpeg::cjpeg_source_ptr,
                    ) -> JDIMENSION,
            )
        }
        24 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_24bit_row
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: super::cdjpeg::cjpeg_source_ptr,
                    ) -> JDIMENSION,
            )
        }
        32 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_32bit_row
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: super::cdjpeg::cjpeg_source_ptr,
                    ) -> JDIMENSION,
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
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
    (*source).source_row = (*cinfo).image_height;
    /* And read the first row */
    return Some(
        (*source)
            .pub_0
            .get_pixel_rows
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, sinfo);
}
/*
 * Read the file header; return image size and component count.
 */

unsafe extern "C" fn start_input_bmp(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
     let mut bmpfileheader:  [U_CHAR; 14] =  [0; 14]; let mut bmpinfoheader:  [U_CHAR; 64] =  [0; 64];   let mut biWidth:  c_int =  0; let mut biHeight:  c_int =  0; let mut biPlanes:  c_ushort =  0; let mut biClrUsed:  c_uint =  0u32; let mut mapentrysize:  c_int =  0i32;  let mut row_width:  JDIMENSION =  0u32;let mut source: bmp_source_ptr = sinfo as bmp_source_ptr; /* 0 indicates no colormap */
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    /* Read and verify the bitmap file header */
    if !(fread(
        bmpfileheader.as_mut_ptr() as *mut c_void,
        1u64,
        14u64,
        (*source).pub_0.input_file,
    ) == 14u64)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if  bmpfileheader[0] as c_int
        + ((bmpfileheader[(0i32 + 1i32) as usize] as c_int)
            << 8i32)
        != 0x4d42i32
    {
        /* 'BM' */
        (*(*cinfo).err).msg_code = JERR_BMP_NOT as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut bfOffBits:   c_uint =
      bmpfileheader[10] as c_uint +
    (((
            (bmpfileheader[(10i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
            (bmpfileheader[(10i32 + 2i32) as usize] as c_uint) << 16i32))) +
    (((
            (bmpfileheader[(10i32 + 3i32) as usize] as c_uint) << 24i32)));
    /* We ignore the remaining fileheader fields */
    /* The infoheader might be 12 bytes (OS/2 1.x), 40 bytes (Windows),
     * or 64 bytes (OS/2 2.x).  Check the first 4 bytes to find out which.
     */
    if !(fread(
        bmpinfoheader.as_mut_ptr() as *mut c_void,
        1u64,
        4u64,
        (*source).pub_0.input_file,
    ) == 4u64)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut headerSize:   c_uint =
      bmpinfoheader[0] as c_uint +
    (((
            (bmpinfoheader[(0i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
            (bmpinfoheader[(0i32 + 2i32) as usize] as c_uint) << 16i32))) +
    (((
            (bmpinfoheader[(0i32 + 3i32) as usize] as c_uint) << 24i32)));
    if headerSize < 12u32 || headerSize > 64u32 {
        (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if !(fread(
        bmpinfoheader.as_mut_ptr().offset(4) as *mut c_void,
        1u64,
        (
        headerSize - 4u32) as size_t,
        (*source).pub_0.input_file,
    ) == ( headerSize - 4u32) as size_t)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    match headerSize {
        12 => {
            /* Decode OS/2 1.x header (Microsoft calls this a BITMAPCOREHEADER) */
            biWidth =  bmpinfoheader[4] as c_int
                + ((bmpinfoheader[(4i32 + 1i32) as usize]
                    as c_int)
                    << 8i32); /* OS/2 uses RGBTRIPLE colormap */
            biHeight =  bmpinfoheader[6] as c_int
                + ((bmpinfoheader[(6i32 + 1i32) as usize]
                    as c_int)
                    << 8i32);
            biPlanes = (bmpinfoheader[8] as c_int
                + ((bmpinfoheader[(8i32 + 1i32) as usize]
                    as c_int)
                    << 8i32)) as c_ushort;
            (*source).bits_per_pixel =  bmpinfoheader[10]
                as c_int
                + ((bmpinfoheader[(10i32 + 1i32) as usize]
                    as c_int)
                    << 8i32);
            match (*source).bits_per_pixel {
                8 => {
                    /* colormapped image */
                    mapentrysize = 3i32;
                    (*(*cinfo).err).msg_code = JTRC_BMP_OS2_MAPPED as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1i32,
                    );
                }
                24 => {
                    /* RGB image */
                    (*(*cinfo).err).msg_code = JTRC_BMP_OS2 as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1i32,
                    );
                }
                _ => {
                    (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
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
        }
        40 | 64 => {
              biWidth = (bmpinfoheader[4] as c_uint +
    (((
                    (bmpinfoheader[(4i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
                    (bmpinfoheader[(4i32 + 2i32) as usize] as c_uint) << 16i32))) +
    (((
                    (bmpinfoheader[(4i32 + 3i32) as usize] as c_uint) << 24i32)))) as c_int;
            biHeight = (bmpinfoheader[8] as c_uint +
    (((
                    (bmpinfoheader[(8i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
                    (bmpinfoheader[(8i32 + 2i32) as usize] as c_uint) << 16i32))) +
    (((
                    (bmpinfoheader[(8i32 + 3i32) as usize] as c_uint) << 24i32)))) as c_int;
            biPlanes = (bmpinfoheader[12] as c_int
                + ((bmpinfoheader[(12i32 + 1i32) as usize]
                    as c_int)
                    << 8i32)) as c_ushort;
            (*source).bits_per_pixel =  bmpinfoheader[14]
                as c_int
                + ((bmpinfoheader[(14i32 + 1i32) as usize]
                    as c_int)
                    << 8i32);
            
            
             let mut biCompression:   c_uint =
      bmpinfoheader[16] as c_uint +
    (((
                    (bmpinfoheader[(16i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
                    (bmpinfoheader[(16i32 + 2i32) as usize] as c_uint)
                        << 16i32))) +
    (((
                    (bmpinfoheader[(16i32 + 3i32) as usize] as c_uint)
                        << 24i32))); let mut biXPelsPerMeter:   c_int =
     (bmpinfoheader[24] as c_uint +
    (((
                    (bmpinfoheader[(24i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
                    (bmpinfoheader[(24i32 + 2i32) as usize] as c_uint)
                        << 16i32))) +
    (((
                    (bmpinfoheader[(24i32 + 3i32) as usize] as c_uint)
                        << 24i32)))) as c_int; let mut biYPelsPerMeter:   c_int =
     (bmpinfoheader[28] as c_uint +
    (((
                    (bmpinfoheader[(28i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
                    (bmpinfoheader[(28i32 + 2i32) as usize] as c_uint)
                        << 16i32))) +
    (((
                    (bmpinfoheader[(28i32 + 3i32) as usize] as c_uint)
                        << 24i32)))) as c_int;
            biClrUsed =  bmpinfoheader[32] as c_uint +
    (((
                    (bmpinfoheader[(32i32 + 1i32) as usize] as c_uint) << 8i32))) +
    (((
                    (bmpinfoheader[(32i32 + 2i32) as usize] as c_uint)
                        << 16i32))) +
    (((
                    (bmpinfoheader[(32i32 + 3i32) as usize] as c_uint)
                        << 24i32)));
            /* biSizeImage, biClrImportant fields are ignored */
            match (*source).bits_per_pixel {
                8 => {
                    mapentrysize = 4i32; /* Windows uses RGBQUAD colormap */
                    (*(*cinfo).err).msg_code = JTRC_BMP_MAPPED as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1i32,
                    );
                }
                24 => {
                    /* colormapped image */
                    /* RGB image */
                    (*(*cinfo).err).msg_code = JTRC_BMP as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1i32,
                    );
                }
                32 => {
                    /* RGB image + Alpha channel */
                    (*(*cinfo).err).msg_code = JTRC_BMP as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1i32,
                    );
                }
                _ => {
                    (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
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
            if biCompression != 0u32 {
                (*(*cinfo).err).msg_code = JERR_BMP_COMPRESSED as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            if biXPelsPerMeter > 0i32 && biYPelsPerMeter > 0i32 {
                /* Set JFIF density parameters from the BMP data */
                (*cinfo).X_density = (biXPelsPerMeter / 100i32) as UINT16;
                (*cinfo).Y_density = (biYPelsPerMeter / 100i32) as UINT16;
                (*cinfo).density_unit = 2u8 /* 100 cm per meter */
                /* dots/cm */
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
            return;
        }
    }
    if biWidth <= 0i32
        || biHeight <= 0i32
        || biWidth as c_long > 0x7fffffffi64
        || biHeight as c_long > 0x7fffffffi64
    {
        (*(*cinfo).err).msg_code = JERR_BMP_EMPTY as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if biPlanes as c_int != 1i32 {
        (*(*cinfo).err).msg_code = JERR_BMP_BADPLANES as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut bPad:   c_int =  ( bfOffBits - (headerSize + 14u32)) as c_int;
    /* Read the colormap, if any */
    if mapentrysize > 0i32 {
        if biClrUsed <= 0u32 {
            biClrUsed = 256u32
        } else if biClrUsed > 256u32 {
            (*(*cinfo).err).msg_code = JERR_BMP_BADCMAP as c_int; /* assume it's 256 */
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        /* Allocate space to store the colormap */
        (*source).colormap = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            biClrUsed,
            3u32,
        );
        (*source).cmap_length = biClrUsed as c_int;
        /* and read it from the file */
        read_colormap(source, biClrUsed as c_int, mapentrysize);
        /* account for size of colormap */
        bPad = (((bPad as c_uint - biClrUsed * mapentrysize as c_uint))) as c_int
    }
    /* Skip any remaining pad bytes */
    if bPad < 0i32 {
        /* incorrect bfOffBits value? */
        (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    loop {
        bPad -= 1;
        if !(bPad >= 0i32) {
            break;
        }
        read_byte(source);
    }
    /* Compute row width in file, including padding to 4-byte boundary */
    match (*source).bits_per_pixel {
        8 => {
            if  (*cinfo).in_color_space
                ==  JCS_UNKNOWN
            {
                (*cinfo).in_color_space = JCS_EXT_RGB
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
            row_width = biWidth as JDIMENSION
        }
        24 => {
            if  (*cinfo).in_color_space
                ==  JCS_UNKNOWN
            {
                (*cinfo).in_color_space = JCS_EXT_BGR
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
                ==  JCS_CMYK
            {
                (*cinfo).input_components = 4i32
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
            row_width = (biWidth * 3i32) as JDIMENSION
        }
        32 => {
            if  (*cinfo).in_color_space
                ==  JCS_UNKNOWN
            {
                (*cinfo).in_color_space = JCS_EXT_BGRA
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
                ==  JCS_CMYK
            {
                (*cinfo).input_components = 4i32
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
            row_width = (biWidth * 4i32) as JDIMENSION
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
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
    while row_width & 3u32 != 0u32 {
        row_width +=  1
    }
    (*source).row_width = row_width;
    if (*source).use_inversion_array != 0 {
        /* Allocate space for inversion array, prepare for preload pass */
        (*source).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            row_width,
            biHeight as JDIMENSION,
            1u32,
        );
        (*source).pub_0.get_pixel_rows = Some(
            preload_image
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: super::cdjpeg::cjpeg_source_ptr,
                ) -> JDIMENSION,
        );
        if !(*cinfo).progress.is_null() {
            let mut progress: super::cdjpeg::cd_progress_ptr =
                (*cinfo).progress as super::cdjpeg::cd_progress_ptr;
            (*progress).total_extra_passes += 1
            /* count file input as separate pass */
        }
    } else {
        (*source).iobuffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            row_width as size_t,
        ) as *mut U_CHAR;
        match (*source).bits_per_pixel {
            8 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_8bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            }
            24 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_24bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            }
            32 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_32bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
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
    }
    /* Ensure that biWidth * cinfo->input_components doesn't exceed the maximum
    value of the JDIMENSION type.  This is only a danger with BMP files, since
    their width and height fields are 32-bit integers. */
    if biWidth as c_ulonglong * (*cinfo).input_components as c_ulonglong
        > 0xffffffffu64
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_WIDTH_OVERFLOW as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Allocate one-row buffer for returned data */
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (biWidth * (*cinfo).input_components) as JDIMENSION,
        1u32,
    );
    (*source).pub_0.buffer_height = 1u32;
    (*cinfo).data_precision = 8i32;
    (*cinfo).image_width = biWidth as JDIMENSION;
    (*cinfo).image_height = biHeight as JDIMENSION;
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_input_bmp(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
    /* no work */
}
/*
 * The module selection routine for BMP format input.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_read_bmp(
    mut cinfo: j_compress_ptr,
    mut use_inversion_array: boolean,
) -> super::cdjpeg::cjpeg_source_ptr {
     
     let mut source:   bmp_source_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<bmp_source_struct>() as c_ulong,
    ) as bmp_source_ptr; /* make back link for subroutines */
    (*source).cinfo = cinfo;
    /* Fill in method ptrs, except get_pixel_rows which start_input sets */
    (*source).pub_0.start_input = Some(
        start_input_bmp
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_bmp
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).use_inversion_array = use_inversion_array;
    return source as super::cdjpeg::cjpeg_source_ptr;
}
/* BMP_SUPPORTED */
