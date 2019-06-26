use libc;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_ushort;
use libc::c_void;
use libc::intptr_t;

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg-rs/mozjpeg-c/jmorecfg.h"]
pub mod jmorecfg_h {

    use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
    use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
    use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
    use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
    use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
    use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
    use crate::jmorecfg_h::RGB_PIXELSIZE;
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

}

pub use crate::cderror_h::C2RustUnnamed_92;
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
pub use crate::cdjpeg::cd_progress_ptr;
pub use crate::cdjpeg::cdjpeg_progress_mgr;
pub use crate::cdjpeg::djpeg_dest_ptr;
pub use crate::cdjpeg::djpeg_dest_struct;
pub use crate::cmyk_h::cmyk_to_rgb;
pub use crate::jerror::C2RustUnnamed_4;
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
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_calc_output_dimensions;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_3;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
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
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
use crate::stdlib::ferror;
use crate::stdlib::fflush;
use crate::stdlib::fwrite;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::putc;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
pub use jmorecfg_h::rgb_pixelsize;
pub type bmp_dest_ptr = *mut bmp_dest_struct;
/*
 * wrbmp.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2013, Linaro Limited.
 * Copyright (C) 2014-2015, 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to write output images in Microsoft "BMP"
 * format (MS Windows 3.x and OS/2 1.x flavors).
 * Either 8-bit colormapped or 24-bit full-color format can be written.
 * No compression is supported.
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume output to
 * an ordinary stdio stream.
 *
 * This code contributed by James Arthur Boucher.
 */
/*
 * To support 12-bit JPEG data, we'd have to scale output down to 8 bits.
 * This is not yet implemented.
 */
/*
 * Since BMP stores scanlines bottom-to-top, we have to invert the image
 * from JPEG's top-to-bottom order.  To do this, we save the outgoing data
 * in a virtual array during put_pixel_row calls, then actually emit the
 * BMP file during finish_output.  The virtual array contains one JSAMPLE per
 * pixel if the output is grayscale or colormapped, three if it is full color.
 */
/* Private version of data destination object */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp_dest_struct {
    pub pub_0: djpeg_dest_struct,
    pub is_os2: boolean,
    pub whole_image: jvirt_sarray_ptr,
    pub data_width: JDIMENSION,
    pub row_width: JDIMENSION,
    pub pad_bytes: c_int,
    pub cur_output_row: JDIMENSION,
    pub use_inversion_array: boolean,
    pub iobuffer: *mut JSAMPLE,
}
#[inline(always)]
unsafe extern "C" fn is_big_endian() -> boolean {
    let mut test_value: c_int = 1i32;
    if *(&mut test_value as *mut c_int as *mut c_char) as c_int != 1i32 {
        return TRUE;
    }
    return FALSE;
}
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 */
unsafe extern "C" fn put_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut image_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut pad: c_int = 0;
    if 0 != (*dest).use_inversion_array {
        image_ptr = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1i32 as JDIMENSION,
            TRUE,
        );
        (*dest).cur_output_row = (*dest).cur_output_row.wrapping_add(1);
        outptr = *image_ptr.offset(0isize)
    } else {
        outptr = (*dest).iobuffer
    }
    inptr = *(*dest).pub_0.buffer.offset(0isize);
    if (*cinfo).out_color_space as c_uint == JCS_EXT_BGR as c_int as c_uint {
        memcpy(
            outptr as *mut c_void,
            inptr as *const c_void,
            (*dest).row_width as size_t,
        );
        outptr = outptr.offset((*cinfo).output_width.wrapping_mul(3i32 as c_uint) as isize)
    } else if (*cinfo).out_color_space as c_uint == JCS_RGB565 as c_int as c_uint {
        let mut big_endian: boolean = is_big_endian();
        let mut inptr2: *mut c_ushort = inptr as *mut c_ushort;
        col = (*cinfo).output_width;
        while col > 0i32 as c_uint {
            if 0 != big_endian {
                *outptr.offset(0isize) = (*inptr2 as c_int >> 5i32 & 0xf8i32) as JSAMPLE;
                *outptr.offset(1isize) = ((*inptr2 as c_int) << 5i32 & 0xe0i32
                    | *inptr2 as c_int >> 11i32 & 0x1ci32)
                    as JSAMPLE;
                *outptr.offset(2isize) = (*inptr2 as c_int & 0xf8i32) as JSAMPLE
            } else {
                *outptr.offset(0isize) = ((*inptr2 as c_int) << 3i32 & 0xf8i32) as JSAMPLE;
                *outptr.offset(1isize) = (*inptr2 as c_int >> 3i32 & 0xfci32) as JSAMPLE;
                *outptr.offset(2isize) = (*inptr2 as c_int >> 8i32 & 0xf8i32) as JSAMPLE
            }
            outptr = outptr.offset(3isize);
            inptr2 = inptr2.offset(1isize);
            col = col.wrapping_sub(1)
        }
    } else if (*cinfo).out_color_space as c_uint == JCS_CMYK as c_int as c_uint {
        col = (*cinfo).output_width;
        while col > 0i32 as c_uint {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            let mut c: JSAMPLE = *fresh0;
            let fresh1 = inptr;
            inptr = inptr.offset(1);
            let mut m: JSAMPLE = *fresh1;
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            let mut y: JSAMPLE = *fresh2;
            let fresh3 = inptr;
            inptr = inptr.offset(1);
            let mut k: JSAMPLE = *fresh3;
            cmyk_to_rgb(
                c,
                m,
                y,
                k,
                outptr.offset(2isize),
                outptr.offset(1isize),
                outptr,
            );
            outptr = outptr.offset(3isize);
            col = col.wrapping_sub(1)
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).out_color_space as usize];
        let mut gindex: c_int = rgb_green[(*cinfo).out_color_space as usize];
        let mut bindex: c_int = rgb_blue[(*cinfo).out_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).out_color_space as usize];
        col = (*cinfo).output_width;
        while col > 0i32 as c_uint {
            *outptr.offset(0isize) = *inptr.offset(bindex as isize);
            *outptr.offset(1isize) = *inptr.offset(gindex as isize);
            *outptr.offset(2isize) = *inptr.offset(rindex as isize);
            outptr = outptr.offset(3isize);
            inptr = inptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    pad = (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0i32) {
            break;
        }
        let fresh4 = outptr;
        outptr = outptr.offset(1);
        *fresh4 = 0i32 as JSAMPLE
    }
    if 0 == (*dest).use_inversion_array {
        fwrite(
            (*dest).iobuffer as *const c_void,
            1i32 as size_t,
            (*dest).row_width as size_t,
            (*dest).pub_0.output_file,
        );
    };
}
unsafe extern "C" fn put_gray_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut image_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut pad: c_int = 0;
    if 0 != (*dest).use_inversion_array {
        image_ptr = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1i32 as JDIMENSION,
            TRUE,
        );
        (*dest).cur_output_row = (*dest).cur_output_row.wrapping_add(1);
        outptr = *image_ptr.offset(0isize)
    } else {
        outptr = (*dest).iobuffer
    }
    inptr = *(*dest).pub_0.buffer.offset(0isize);
    memcpy(
        outptr as *mut c_void,
        inptr as *const c_void,
        (*cinfo).output_width as size_t,
    );
    outptr = outptr.offset((*cinfo).output_width as isize);
    pad = (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0i32) {
            break;
        }
        let fresh5 = outptr;
        outptr = outptr.offset(1);
        *fresh5 = 0i32 as JSAMPLE
    }
    if 0 == (*dest).use_inversion_array {
        fwrite(
            (*dest).iobuffer as *const c_void,
            1i32 as size_t,
            (*dest).row_width as size_t,
            (*dest).pub_0.output_file,
        );
    };
}
/*
 * Finish up at the end of the file.
 *
 * Here is where we really output the BMP file.
 *
 * First, routines to write the Windows and OS/2 variants of the file header.
 */
unsafe extern "C" fn write_bmp_header(mut cinfo: j_decompress_ptr, mut dest: bmp_dest_ptr) {
    let mut bmpfileheader: [c_char; 14] = [0; 14];
    let mut bmpinfoheader: [c_char; 40] = [0; 40];
    let mut headersize: c_long = 0;
    let mut bfSize: c_long = 0;
    let mut bits_per_pixel: c_int = 0;
    let mut cmap_entries: c_int = 0;
    if (*cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint
        || (*cinfo).out_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
            && (*cinfo).out_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
    {
        if 0 != (*cinfo).quantize_colors {
            bits_per_pixel = 8i32;
            cmap_entries = 256i32
        } else {
            bits_per_pixel = 24i32;
            cmap_entries = 0i32
        }
    } else if (*cinfo).out_color_space as c_uint == JCS_RGB565 as c_int as c_uint
        || (*cinfo).out_color_space as c_uint == JCS_CMYK as c_int as c_uint
    {
        bits_per_pixel = 24i32;
        cmap_entries = 0i32
    } else {
        bits_per_pixel = 8i32;
        cmap_entries = 256i32
    }
    headersize = (14i32 + 40i32 + cmap_entries * 4i32) as c_long;
    bfSize = headersize + (*dest).row_width as c_long * (*cinfo).output_height as c_long;
    memset(
        bmpfileheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 14]>() as c_ulong,
    );
    memset(
        bmpinfoheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 40]>() as c_ulong,
    );
    bmpfileheader[0usize] = 0x42i32 as c_char;
    bmpfileheader[1usize] = 0x4di32 as c_char;
    bmpfileheader[2usize] = (bfSize & 0xffi32 as c_long) as c_char;
    bmpfileheader[(2i32 + 1i32) as usize] = (bfSize >> 8i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(2i32 + 2i32) as usize] = (bfSize >> 16i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(2i32 + 3i32) as usize] = (bfSize >> 24i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[10usize] = (headersize & 0xffi32 as c_long) as c_char;
    bmpfileheader[(10i32 + 1i32) as usize] = (headersize >> 8i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(10i32 + 2i32) as usize] = (headersize >> 16i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(10i32 + 3i32) as usize] = (headersize >> 24i32 & 0xffi32 as c_long) as c_char;
    bmpinfoheader[0usize] = (40i32 & 0xffi32) as c_char;
    bmpinfoheader[(0i32 + 1i32) as usize] = (40i32 >> 8i32 & 0xffi32) as c_char;
    bmpinfoheader[4usize] = ((*cinfo).output_width & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[(4i32 + 1i32) as usize] =
        ((*cinfo).output_width >> 8i32 & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[(4i32 + 2i32) as usize] =
        ((*cinfo).output_width >> 16i32 & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[(4i32 + 3i32) as usize] =
        ((*cinfo).output_width >> 24i32 & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[8usize] = ((*cinfo).output_height & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[(8i32 + 1i32) as usize] =
        ((*cinfo).output_height >> 8i32 & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[(8i32 + 2i32) as usize] =
        ((*cinfo).output_height >> 16i32 & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[(8i32 + 3i32) as usize] =
        ((*cinfo).output_height >> 24i32 & 0xffi32 as c_uint) as c_char;
    bmpinfoheader[12usize] = (1i32 & 0xffi32) as c_char;
    bmpinfoheader[(12i32 + 1i32) as usize] = (1i32 >> 8i32 & 0xffi32) as c_char;
    bmpinfoheader[14usize] = (bits_per_pixel & 0xffi32) as c_char;
    bmpinfoheader[(14i32 + 1i32) as usize] = (bits_per_pixel >> 8i32 & 0xffi32) as c_char;
    if (*cinfo).density_unit as c_int == 2i32 {
        bmpinfoheader[24usize] =
            (((*cinfo).X_density as c_int * 100i32) as c_long & 0xffi32 as c_long) as c_char;
        bmpinfoheader[(24i32 + 1i32) as usize] = (((*cinfo).X_density as c_int * 100i32) as c_long
            >> 8i32
            & 0xffi32 as c_long) as c_char;
        bmpinfoheader[(24i32 + 2i32) as usize] = (((*cinfo).X_density as c_int * 100i32) as c_long
            >> 16i32
            & 0xffi32 as c_long) as c_char;
        bmpinfoheader[(24i32 + 3i32) as usize] = (((*cinfo).X_density as c_int * 100i32) as c_long
            >> 24i32
            & 0xffi32 as c_long) as c_char;
        bmpinfoheader[28usize] =
            (((*cinfo).Y_density as c_int * 100i32) as c_long & 0xffi32 as c_long) as c_char;
        bmpinfoheader[(28i32 + 1i32) as usize] = (((*cinfo).Y_density as c_int * 100i32) as c_long
            >> 8i32
            & 0xffi32 as c_long) as c_char;
        bmpinfoheader[(28i32 + 2i32) as usize] = (((*cinfo).Y_density as c_int * 100i32) as c_long
            >> 16i32
            & 0xffi32 as c_long) as c_char;
        bmpinfoheader[(28i32 + 3i32) as usize] = (((*cinfo).Y_density as c_int * 100i32) as c_long
            >> 24i32
            & 0xffi32 as c_long) as c_char
    }
    bmpinfoheader[32usize] = (cmap_entries & 0xffi32) as c_char;
    bmpinfoheader[(32i32 + 1i32) as usize] = (cmap_entries >> 8i32 & 0xffi32) as c_char;
    if fwrite(
        bmpfileheader.as_mut_ptr() as *const c_void,
        1i32 as size_t,
        14i32 as size_t,
        (*dest).pub_0.output_file,
    ) != 14i32 as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if fwrite(
        bmpinfoheader.as_mut_ptr() as *const c_void,
        1i32 as size_t,
        40i32 as size_t,
        (*dest).pub_0.output_file,
    ) != 40i32 as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if cmap_entries > 0i32 {
        write_colormap(cinfo, dest, cmap_entries, 4i32);
    };
}
unsafe extern "C" fn write_os2_header(mut cinfo: j_decompress_ptr, mut dest: bmp_dest_ptr) {
    let mut bmpfileheader: [c_char; 14] = [0; 14];
    let mut bmpcoreheader: [c_char; 12] = [0; 12];
    let mut headersize: c_long = 0;
    let mut bfSize: c_long = 0;
    let mut bits_per_pixel: c_int = 0;
    let mut cmap_entries: c_int = 0;
    if (*cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint
        || (*cinfo).out_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
            && (*cinfo).out_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
    {
        if 0 != (*cinfo).quantize_colors {
            bits_per_pixel = 8i32;
            cmap_entries = 256i32
        } else {
            bits_per_pixel = 24i32;
            cmap_entries = 0i32
        }
    } else if (*cinfo).out_color_space as c_uint == JCS_RGB565 as c_int as c_uint
        || (*cinfo).out_color_space as c_uint == JCS_CMYK as c_int as c_uint
    {
        bits_per_pixel = 24i32;
        cmap_entries = 0i32
    } else {
        bits_per_pixel = 8i32;
        cmap_entries = 256i32
    }
    headersize = (14i32 + 12i32 + cmap_entries * 3i32) as c_long;
    bfSize = headersize + (*dest).row_width as c_long * (*cinfo).output_height as c_long;
    memset(
        bmpfileheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 14]>() as c_ulong,
    );
    memset(
        bmpcoreheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 12]>() as c_ulong,
    );
    bmpfileheader[0usize] = 0x42i32 as c_char;
    bmpfileheader[1usize] = 0x4di32 as c_char;
    bmpfileheader[2usize] = (bfSize & 0xffi32 as c_long) as c_char;
    bmpfileheader[(2i32 + 1i32) as usize] = (bfSize >> 8i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(2i32 + 2i32) as usize] = (bfSize >> 16i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(2i32 + 3i32) as usize] = (bfSize >> 24i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[10usize] = (headersize & 0xffi32 as c_long) as c_char;
    bmpfileheader[(10i32 + 1i32) as usize] = (headersize >> 8i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(10i32 + 2i32) as usize] = (headersize >> 16i32 & 0xffi32 as c_long) as c_char;
    bmpfileheader[(10i32 + 3i32) as usize] = (headersize >> 24i32 & 0xffi32 as c_long) as c_char;
    bmpcoreheader[0usize] = (12i32 & 0xffi32) as c_char;
    bmpcoreheader[(0i32 + 1i32) as usize] = (12i32 >> 8i32 & 0xffi32) as c_char;
    bmpcoreheader[4usize] = ((*cinfo).output_width & 0xffi32 as c_uint) as c_char;
    bmpcoreheader[(4i32 + 1i32) as usize] =
        ((*cinfo).output_width >> 8i32 & 0xffi32 as c_uint) as c_char;
    bmpcoreheader[6usize] = ((*cinfo).output_height & 0xffi32 as c_uint) as c_char;
    bmpcoreheader[(6i32 + 1i32) as usize] =
        ((*cinfo).output_height >> 8i32 & 0xffi32 as c_uint) as c_char;
    bmpcoreheader[8usize] = (1i32 & 0xffi32) as c_char;
    bmpcoreheader[(8i32 + 1i32) as usize] = (1i32 >> 8i32 & 0xffi32) as c_char;
    bmpcoreheader[10usize] = (bits_per_pixel & 0xffi32) as c_char;
    bmpcoreheader[(10i32 + 1i32) as usize] = (bits_per_pixel >> 8i32 & 0xffi32) as c_char;
    if fwrite(
        bmpfileheader.as_mut_ptr() as *const c_void,
        1i32 as size_t,
        14i32 as size_t,
        (*dest).pub_0.output_file,
    ) != 14i32 as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if fwrite(
        bmpcoreheader.as_mut_ptr() as *const c_void,
        1i32 as size_t,
        12i32 as size_t,
        (*dest).pub_0.output_file,
    ) != 12i32 as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if cmap_entries > 0i32 {
        write_colormap(cinfo, dest, cmap_entries, 3i32);
    };
}
/* Forward declarations */
/*
 * Write the colormap.
 * Windows uses BGR0 map entries; OS/2 uses BGR entries.
 */
unsafe extern "C" fn write_colormap(
    mut cinfo: j_decompress_ptr,
    mut dest: bmp_dest_ptr,
    mut map_colors: c_int,
    mut map_entry_size: c_int,
) {
    let mut colormap: JSAMPARRAY = (*cinfo).colormap;
    let mut num_colors: c_int = (*cinfo).actual_number_of_colors;
    let mut outfile: *mut FILE = (*dest).pub_0.output_file;
    let mut i: c_int = 0;
    if !colormap.is_null() {
        if (*cinfo).out_color_components == 3i32 {
            i = 0i32;
            while i < num_colors {
                putc(
                    *(*colormap.offset(2isize)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(1isize)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0isize)).offset(i as isize) as c_int,
                    outfile,
                );
                if map_entry_size == 4i32 {
                    putc(0i32, outfile);
                }
                i += 1
            }
        } else {
            i = 0i32;
            while i < num_colors {
                putc(
                    *(*colormap.offset(0isize)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0isize)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0isize)).offset(i as isize) as c_int,
                    outfile,
                );
                if map_entry_size == 4i32 {
                    putc(0i32, outfile);
                }
                i += 1
            }
        }
    } else {
        i = 0i32;
        while i < 256i32 {
            putc(i, outfile);
            putc(i, outfile);
            putc(i, outfile);
            if map_entry_size == 4i32 {
                putc(0i32, outfile);
            }
            i += 1
        }
    }
    if i > map_colors {
        (*(*cinfo).err).msg_code = JERR_TOO_MANY_COLORS as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = i;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    while i < map_colors {
        putc(0i32, outfile);
        putc(0i32, outfile);
        putc(0i32, outfile);
        if map_entry_size == 4i32 {
            putc(0i32, outfile);
        }
        i += 1
    }
}
/*
 * Startup: write the file header unless the inversion array is being used.
 */
unsafe extern "C" fn start_output_bmp(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    if 0 == (*dest).use_inversion_array {
        if 0 != (*dest).is_os2 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
    };
}
unsafe extern "C" fn finish_output_bmp(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut outfile: *mut FILE = (*dest).pub_0.output_file;
    let mut image_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut data_ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut row: JDIMENSION = 0;
    let mut col: JDIMENSION = 0;
    let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
    if 0 != (*dest).use_inversion_array {
        if 0 != (*dest).is_os2 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
        row = (*cinfo).output_height;
        while row > 0i32 as c_uint {
            if !progress.is_null() {
                (*progress).pub_0.pass_counter = (*cinfo).output_height.wrapping_sub(row) as c_long;
                (*progress).pub_0.pass_limit = (*cinfo).output_height as c_long;
                (*progress)
                    .pub_0
                    .progress_monitor
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            image_ptr = (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                (*dest).whole_image,
                row.wrapping_sub(1i32 as c_uint),
                1i32 as JDIMENSION,
                FALSE,
            );
            data_ptr = *image_ptr.offset(0isize);
            col = (*dest).row_width;
            while col > 0i32 as c_uint {
                putc(*data_ptr as c_int, outfile);
                data_ptr = data_ptr.offset(1isize);
                col = col.wrapping_sub(1)
            }
            row = row.wrapping_sub(1)
        }
        if !progress.is_null() {
            (*progress).completed_extra_passes += 1
        }
    }
    fflush(outfile);
    if 0 != ferror(outfile) {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/*
 * The module selection routine for BMP format output.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_write_bmp(
    mut cinfo: j_decompress_ptr,
    mut is_os2: boolean,
    mut use_inversion_array: boolean,
) -> djpeg_dest_ptr {
    let mut dest: bmp_dest_ptr = 0 as *mut bmp_dest_struct;
    let mut row_width: JDIMENSION = 0;
    dest = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<bmp_dest_struct>() as c_ulong,
    ) as bmp_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_bmp as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_bmp as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = ::std::mem::transmute::<
        intptr_t,
        Option<unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> ()>,
    >(NULL as intptr_t);
    (*dest).is_os2 = is_os2;
    if (*cinfo).out_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
        (*dest).pub_0.put_pixel_rows = Some(
            put_gray_rows
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: djpeg_dest_ptr,
                    _: JDIMENSION,
                ) -> (),
        )
    } else if (*cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint
        || (*cinfo).out_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
            && (*cinfo).out_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
    {
        if 0 != (*cinfo).quantize_colors {
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else {
            (*dest).pub_0.put_pixel_rows = Some(
                put_pixel_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
    } else if (*cinfo).out_color_space as c_uint == JCS_RGB565 as c_int as c_uint
        || (*cinfo).out_color_space as c_uint == JCS_CMYK as c_int as c_uint
    {
        (*dest).pub_0.put_pixel_rows = Some(
            put_pixel_rows
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: djpeg_dest_ptr,
                    _: JDIMENSION,
                ) -> (),
        )
    } else {
        (*(*cinfo).err).msg_code = JERR_BMP_COLORSPACE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    jpeg_calc_output_dimensions(cinfo);
    if (*cinfo).out_color_space as c_uint == JCS_RGB565 as c_int as c_uint {
        row_width = (*cinfo).output_width.wrapping_mul(2i32 as c_uint);
        (*dest).data_width = (*cinfo).output_width.wrapping_mul(3i32 as c_uint);
        (*dest).row_width = (*dest).data_width;
        while row_width & 3i32 as c_uint != 0i32 as c_uint {
            row_width = row_width.wrapping_add(1)
        }
    } else if 0 == (*cinfo).quantize_colors
        && ((*cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint
            || (*cinfo).out_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                && (*cinfo).out_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            || (*cinfo).out_color_space as c_uint == JCS_CMYK as c_int as c_uint)
    {
        row_width = (*cinfo)
            .output_width
            .wrapping_mul((*cinfo).output_components as c_uint);
        (*dest).data_width = (*cinfo).output_width.wrapping_mul(3i32 as c_uint);
        (*dest).row_width = (*dest).data_width
    } else {
        row_width = (*cinfo)
            .output_width
            .wrapping_mul((*cinfo).output_components as c_uint);
        (*dest).data_width = row_width;
        (*dest).row_width = (*dest).data_width
    }
    while (*dest).row_width & 3i32 as c_uint != 0i32 as c_uint {
        (*dest).row_width = (*dest).row_width.wrapping_add(1)
    }
    (*dest).pad_bytes = (*dest).row_width.wrapping_sub((*dest).data_width) as c_int;
    if 0 != use_inversion_array {
        (*dest).whole_image = (*(*cinfo).mem)
            .request_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            (*dest).row_width,
            (*cinfo).output_height,
            1i32 as JDIMENSION,
        );
        (*dest).cur_output_row = 0i32 as JDIMENSION;
        if !(*cinfo).progress.is_null() {
            let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
            (*progress).total_extra_passes += 1
        }
    } else {
        (*dest).iobuffer = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*dest).row_width as size_t,
        ) as *mut JSAMPLE
    }
    (*dest).use_inversion_array = use_inversion_array;
    (*dest).pub_0.buffer = (*(*cinfo).mem)
        .alloc_sarray
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        row_width,
        1i32 as JDIMENSION,
    );
    (*dest).pub_0.buffer_height = 1i32 as JDIMENSION;
    return dest as djpeg_dest_ptr;
}
