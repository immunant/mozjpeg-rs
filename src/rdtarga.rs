use libc::c_uchar;use libc::c_ulong;use libc::c_uint;use libc::c_long;use libc::c_int;use libc::c_void;use libc;

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
pub use crate::stdlib::fread;
pub use crate::stdlib::getc;
pub use crate::stdlib::EOF;
/* !HAVE_UNSIGNED_CHAR */
/* HAVE_UNSIGNED_CHAR */
/* Private version of data source object */

pub type tga_source_ptr = *mut _tga_source_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tga_source_struct {
    pub pub_0: super::cdjpeg::cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub colormap: JSAMPARRAY,
    pub whole_image: jvirt_sarray_ptr,
    pub current_row: JDIMENSION,
    pub read_pixel: Option<unsafe extern "C" fn(_: tga_source_ptr) -> ()>,
    pub tga_pixel: [U_CHAR; 4],
    pub pixel_size: c_int,
    pub block_count: c_int,
    pub dup_pixel_count: c_int,
    pub get_pixel_rows: Option<
        unsafe extern "C" fn(
            _: j_compress_ptr,
            _: super::cdjpeg::cjpeg_source_ptr,
        ) -> JDIMENSION,
    >,
}
/*
 * rdtarga.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to read input images in Targa format.
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume input from
 * an ordinary stdio stream.  They further assume that reading begins
 * at the start of the file; start_input may need work if the
 * user interface has already read some data (e.g., to determine that
 * the file is indeed Targa format).
 *
 * Based on code contributed by Lee Daniel Crocker.
 */
/* Macros to deal with unsigned chars as efficiently as compiler allows */

pub type U_CHAR = c_uchar;

pub type tga_source_struct = _tga_source_struct;
/* For expanding 5-bit pixel values to 8-bit with best rounding */

static mut c5to8bits: [UINT8; 32] = [
    0u8,
    8u8,
    16u8,
    25u8,
    33u8,
    41u8,
    49u8,
    58u8,
    66u8,
    74u8,
    82u8,
    90u8,
    99u8,
    107u8,
    115u8,
    123u8,
    132u8,
    140u8,
    148u8,
    156u8,
    165u8,
    173u8,
    181u8,
    189u8,
    197u8,
    206u8,
    214u8,
    222u8,
    230u8,
    239u8,
    247u8,
    255u8,
];

unsafe extern "C" fn read_byte(mut sinfo: tga_source_ptr) -> c_int
/* Read next byte from Targa file */ {
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
    mut sinfo: tga_source_ptr,
    mut cmaplen: c_int,
    mut mapentrysize: c_int,
)
/* Read the colormap from a Targa file */
{
     
    /* Presently only handles 24-bit BGR format */
    if mapentrysize != 24i32 {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_TGA_BADCMAP as c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*sinfo).cinfo as j_common_ptr
        );
    }
     let mut i:   c_int =  0i32;
    while i < cmaplen {
        *(*(*sinfo).colormap.offset(2)).offset(i as isize) =
            read_byte(sinfo) as JSAMPLE;
        *(*(*sinfo).colormap.offset(1)).offset(i as isize) =
            read_byte(sinfo) as JSAMPLE;
        *(*(*sinfo).colormap.offset(0)).offset(i as isize) =
            read_byte(sinfo) as JSAMPLE;
        i += 1
    }
}
/*
 * read_pixel methods: get a single pixel from Targa file into tga_pixel[]
 */

unsafe extern "C" fn read_non_rle_pixel(mut sinfo: tga_source_ptr)
/* Read one Targa pixel from the input file; no RLE expansion */
{
     
     let mut i:   c_int =  0i32;
    while i < (*sinfo).pixel_size {
        (*sinfo).tga_pixel[i as usize] = read_byte(sinfo) as U_CHAR;
        i += 1
    }
}

unsafe extern "C" fn read_rle_pixel(mut sinfo: tga_source_ptr)
/* Read one Targa pixel from the input file, expanding RLE data as needed */
{
     let mut i:  c_int =  0;
    /* Duplicate previously read pixel? */
    if (*sinfo).dup_pixel_count > 0i32 {
        (*sinfo).dup_pixel_count -= 1;
        return;
    }
    /* Time to read RLE block header? */
    (*sinfo).block_count -= 1;
    if (*sinfo).block_count < 0i32 {
        /* decrement pixels remaining in block */
        i = read_byte(sinfo);
        if i & 0x80i32 != 0 {
            /* Start of duplicate-pixel block? */
            (*sinfo).dup_pixel_count = i & 0x7fi32;
            (*sinfo).block_count = 0i32 /* number of dups after this one */
        /* then read new block header */
        } else {
            (*sinfo).block_count = i & 0x7fi32
            /* number of pixels after this one */
        }
    }
    /* Read next pixel */
    i = 0i32;
    while i < (*sinfo).pixel_size {
        (*sinfo).tga_pixel[i as usize] = read_byte(sinfo) as U_CHAR;
        i += 1
    }
}
/*
 * Read one row of pixels.
 *
 * We provide several different versions depending on input file format.
 */

unsafe extern "C" fn get_8bit_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading 8-bit grayscale pixels */ {
      let mut source: tga_source_ptr = sinfo as tga_source_ptr; /* Load next pixel into tga_pixel */
    
    
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
        Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 =  (*source).tga_pixel[0];
        col -=  1
    }
    return 1u32;
}

unsafe extern "C" fn get_8bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading 8-bit colormap indexes */ {
      let mut source: tga_source_ptr = sinfo as tga_source_ptr; /* Load next pixel into tga_pixel */
    
    
    
    let mut colormap: JSAMPARRAY = (*source).colormap;
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
         Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
         let mut t:   c_int =  (*source).tga_pixel[0] as c_int;
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = *(*colormap.offset(0)).offset(t as isize);
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = *(*colormap.offset(1)).offset(t as isize);
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        *fresh3 = *(*colormap.offset(2)).offset(t as isize);
        col -=  1
    }
    return 1u32;
}

unsafe extern "C" fn get_16bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading 16-bit pixels */ {
      let mut source: tga_source_ptr = sinfo as tga_source_ptr; /* Load next pixel into tga_pixel */
    
    
    
    
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
         Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
         let mut t:   c_int =  (*source).tga_pixel[0] as c_int;
        t += ((*source).tga_pixel[1] as c_int) << 8i32;
        /* We expand 5 bit data to 8 bit sample width.
         * The format of the 16-bit (LSB first) input word is
         *     xRRRRRGGGGGBBBBB
         */
        *ptr.offset(2) = c5to8bits[(t & 0x1fi32) as usize];
        t >>= 5i32;
        *ptr.offset(1) = c5to8bits[(t & 0x1fi32) as usize];
        t >>= 5i32;
        *ptr.offset(0) = c5to8bits[(t & 0x1fi32) as usize];
        ptr = ptr.offset(3);
        col -=  1
    }
    return 1u32;
}

unsafe extern "C" fn get_24bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION
/* This version is for reading 24-bit pixels */ {
      let mut source: tga_source_ptr = sinfo as tga_source_ptr; /* Load next pixel into tga_pixel */
     
     let mut ptr:   JSAMPROW =  *(*source).pub_0.buffer.offset(0); let mut col:   JDIMENSION =  (*cinfo).image_width;
    while col > 0u32 {
        Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        *fresh4 =  (*source).tga_pixel[2];
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        *fresh5 =  (*source).tga_pixel[1];
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        *fresh6 =  (*source).tga_pixel[0];
        col -=  1
    }
    return 1u32;
}
/*
 * Targa also defines a 32-bit pixel format with order B,G,R,A.
 * We presently ignore the attribute byte, so the code for reading
 * these pixels is identical to the 24-bit routine above.
 * This works because the actual pixel length is only known to read_pixel.
 */

pub const get_32bit_row: unsafe extern "C" fn(
    _: j_compress_ptr,
    _: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION = get_24bit_row;
/*
 * This method is for re-reading the input data in standard top-down
 * row order.  The entire image has already been read into whole_image
 * with proper conversion of pixel format, but it's in a funny row order.
 */

unsafe extern "C" fn get_memory_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION {
     let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    
     let mut source_row:   JDIMENSION =
      (*cinfo)
        .image_height - (*source).current_row - 1u32;
    /* Fetch that row from virtual array */
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        (*source).whole_image,
        source_row,
        1u32,
        FALSE,
    );
    (*source).current_row =  (*source).current_row + 1;
    return 1u32;
}
/*
 * This method loads the image into whole_image during the first call on
 * get_pixel_rows.  The get_pixel_rows pointer is then adjusted to call
 * get_memory_row on subsequent calls.
 */

unsafe extern "C" fn preload_image(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) -> JDIMENSION {
     let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    
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
        (*source).pub_0.buffer = Some(
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
        );
        Some((*source).get_pixel_rows.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo, sinfo);
        row +=  1
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1
    }
    /* Set up to read from the virtual array in unscrambled order */
    (*source).pub_0.get_pixel_rows = Some(
        get_memory_row
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> JDIMENSION,
    );
    (*source).current_row = 0u32;
    /* And read the first row */
    return get_memory_row(cinfo, sinfo);
}
/*
 * Read the file header; return image size and component count.
 */

unsafe extern "C" fn start_input_tga(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
     let mut targaheader:  [U_CHAR; 18] =  [0; 18];          let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    
    
    
    
    
    
    
    
    
    
    
    if !(fread(
        targaheader.as_mut_ptr() as *mut c_void,
        1u64,
        18u64,
        (*source).pub_0.input_file,
    ) == 18u64)
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Pretend "15-bit" pixels are 16-bit --- we ignore attribute bit anyway */
    if targaheader[16] as c_int == 15i32 {
        targaheader[16] = 16u8
    } /* Image Descriptor byte */
     /* bit 5 set => top-down */
     /* bits 6/7 are interlace code */
    
    
    
     let mut idlen:   c_int =  targaheader[0] as c_int; let mut cmaptype:   c_int =  targaheader[1] as c_int; let mut subtype:   c_int =  targaheader[2] as c_int; let mut maplen:   c_uint =
      targaheader[5] as c_uint +
    ((((targaheader[(5i32 + 1i32) as usize] as c_uint) << 8i32))); let mut width:   c_uint =
      targaheader[12] as c_uint +
    (((
        (targaheader[(12i32 + 1i32) as usize] as c_uint) << 8i32))); let mut height:   c_uint =
      targaheader[14] as c_uint +
    (((
        (targaheader[(14i32 + 1i32) as usize] as c_uint) << 8i32)));
    (*source).pixel_size = targaheader[16] as c_int >> 3i32;
    
    
     let mut flags:   c_int =  targaheader[17] as c_int; let mut is_bottom_up:   boolean =
     (flags & 0x20i32 == 0i32) as c_int; let mut interlace_type:   c_int =  flags >> 6i32;
    if cmaptype > 1i32
        || (*source).pixel_size < 1i32
        || (*source).pixel_size > 4i32
        || targaheader[16] as c_int & 7i32 != 0i32
        || interlace_type != 0i32
        || width == 0u32
        || height == 0u32
    {
        /* image width/height must be non-zero */
        (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if subtype > 8i32 {
        /* It's an RLE-coded file */
        (*source).read_pixel =
            Some(read_rle_pixel as unsafe extern "C" fn(_: tga_source_ptr) -> ());
        (*source).dup_pixel_count = 0i32;
        (*source).block_count = (*source).dup_pixel_count;
        subtype -= 8i32
    } else {
        /* Non-RLE file */
        (*source).read_pixel =
            Some(read_non_rle_pixel as unsafe extern "C" fn(_: tga_source_ptr) -> ())
    }
     let mut components:   c_int =  3i32; /* until proven different */
    (*cinfo).in_color_space = JCS_RGB;
    match subtype {
        1 => {
            /* Colormapped image */
            if (*source).pixel_size == 1i32 && cmaptype == 1i32 {
                (*source).get_pixel_rows = Some(
                    get_8bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            (*(*cinfo).err).msg_code = JTRC_TGA_MAPPED as c_int;
            (*(*cinfo).err).msg_parm.i[0] = width as c_int;
            (*(*cinfo).err).msg_parm.i[1] = height as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1i32
            );
        }
        2 => {
            /* RGB image */
            match (*source).pixel_size {
                2 => {
                    (*source).get_pixel_rows = Some(
                        get_16bit_row
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: super::cdjpeg::cjpeg_source_ptr,
                            )
                                -> JDIMENSION,
                    )
                }
                3 => {
                    (*source).get_pixel_rows = Some(
                        get_24bit_row
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: super::cdjpeg::cjpeg_source_ptr,
                            )
                                -> JDIMENSION,
                    )
                }
                4 => (*source).get_pixel_rows = Some(get_32bit_row),
                _ => {
                    (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
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
            (*(*cinfo).err).msg_code = JTRC_TGA as c_int;
            (*(*cinfo).err).msg_parm.i[0] = width as c_int;
            (*(*cinfo).err).msg_parm.i[1] = height as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1i32
            );
        }
        3 => {
            /* Grayscale image */
            components = 1i32;
            (*cinfo).in_color_space = JCS_GRAYSCALE;
            if (*source).pixel_size == 1i32 {
                (*source).get_pixel_rows = Some(
                    get_8bit_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: super::cdjpeg::cjpeg_source_ptr,
                        )
                            -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            (*(*cinfo).err).msg_code = JTRC_TGA_GRAY as c_int;
            (*(*cinfo).err).msg_parm.i[0] = width as c_int;
            (*(*cinfo).err).msg_parm.i[1] = height as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1i32
            );
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
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
    if is_bottom_up != 0 {
        /* Create a virtual array to buffer the upside-down image. */
        (*source).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            
            width * components as c_uint,
            height,
            1u32,
        );
        if !(*cinfo).progress.is_null() {
            let mut progress: super::cdjpeg::cd_progress_ptr =
                (*cinfo).progress as super::cdjpeg::cd_progress_ptr;
            (*progress).total_extra_passes += 1
            /* count file input as separate pass */
        }
        /* source->pub.buffer will point to the virtual array. */
        (*source).pub_0.buffer_height = 1u32; /* in case anyone looks at it */
        (*source).pub_0.get_pixel_rows = Some(
            preload_image
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: super::cdjpeg::cjpeg_source_ptr,
                ) -> JDIMENSION,
        )
    } else {
        /* Don't need a virtual array, but do need a one-row input buffer. */
        (*source).whole_image = NULL as jvirt_sarray_ptr;
        (*source).pub_0.buffer = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            
            width * components as c_uint,
            1u32,
        );
        (*source).pub_0.buffer_height = 1u32;
        (*source).pub_0.get_pixel_rows = (*source).get_pixel_rows
    }
    loop {
        let fresh7 = idlen;
        idlen -=  1;
        if !(fresh7 != 0) {
            break;
        }
        /* Throw away ID field */
        read_byte(source);
    }
    if maplen > 0u32 {
        if maplen > 256u32
            ||  targaheader[3] as c_uint +
    (((
                (targaheader[(3i32 + 1i32) as usize] as c_uint) << 8i32))) != 0u32
        {
            (*(*cinfo).err).msg_code = JERR_TGA_BADCMAP as c_int;
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
            maplen,
            3u32,
        );
        /* and read it from the file */
        read_colormap(source, maplen as c_int, targaheader[7] as c_int);
    } else {
        if cmaptype != 0 {
            /* but you promised a cmap! */
            (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        (*source).colormap = NULL as JSAMPARRAY
    }
    (*cinfo).input_components = components;
    (*cinfo).data_precision = 8i32;
    (*cinfo).image_width = width;
    (*cinfo).image_height = height;
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_input_tga(
    mut cinfo: j_compress_ptr,
    mut sinfo: super::cdjpeg::cjpeg_source_ptr,
) {
    /* no work */
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
/*
 * The module selection routine for Targa format input.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_read_targa(
    mut cinfo: j_compress_ptr,
) -> super::cdjpeg::cjpeg_source_ptr {
     
     let mut source:   tga_source_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<tga_source_struct>() as c_ulong,
    ) as tga_source_ptr; /* make back link for subroutines */
    (*source).cinfo = cinfo;
    /* Fill in method ptrs, except get_pixel_rows which start_input sets */
    (*source).pub_0.start_input = Some(
        start_input_tga
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_tga
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: super::cdjpeg::cjpeg_source_ptr,
            ) -> (),
    );
    return source as super::cdjpeg::cjpeg_source_ptr;
}
/* TARGA_SUPPORTED */
