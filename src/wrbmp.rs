use libc;

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:24"]
pub mod jmorecfg_h {

    pub static mut rgb_pixelsize: [libc::c_int; 17] = [
        -1i32,
        -1i32,
        crate::jmorecfg_h::RGB_PIXELSIZE,
        -1i32,
        -1i32,
        -1i32,
        crate::jmorecfg_h::EXT_RGB_PIXELSIZE,
        crate::jmorecfg_h::EXT_RGBX_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGRX_PIXELSIZE,
        crate::jmorecfg_h::EXT_XBGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_XRGB_PIXELSIZE,
        crate::jmorecfg_h::EXT_RGBX_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGRX_PIXELSIZE,
        crate::jmorecfg_h::EXT_XBGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_XRGB_PIXELSIZE,
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
pub use crate::cmyk_h::cmyk_to_rgb;
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
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_calc_output_dimensions;
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
pub use crate::jpeglib_h::jpeg_marker_parser_method;
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
pub use crate::src::cdjpeg::cd_progress_ptr;
pub use crate::src::cdjpeg::cdjpeg_progress_mgr;
pub use crate::src::cdjpeg::djpeg_dest_ptr;
pub use crate::src::cdjpeg::djpeg_dest_struct;
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
pub use crate::src::wrbmp::jmorecfg_h::rgb_pixelsize;
use crate::stdlib::ferror;
use crate::stdlib::fflush;
use crate::stdlib::fwrite;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::putc;

pub type bmp_dest_ptr = *mut bmp_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp_dest_struct {
    pub pub_0: crate::src::cdjpeg::djpeg_dest_struct,
    pub is_os2: crate::jmorecfg_h::boolean,
    pub whole_image: crate::jpeglib_h::jvirt_sarray_ptr,
    pub data_width: crate::jmorecfg_h::JDIMENSION,
    pub row_width: crate::jmorecfg_h::JDIMENSION,
    pub pad_bytes: libc::c_int,
    pub cur_output_row: crate::jmorecfg_h::JDIMENSION,
    pub use_inversion_array: crate::jmorecfg_h::boolean,
    pub iobuffer: *mut crate::jmorecfg_h::JSAMPLE,
}
#[inline(always)]

unsafe extern "C" fn is_big_endian() -> crate::jmorecfg_h::boolean {
     let mut test_value:  libc::c_int =  1i32;
    if *(&mut test_value as *mut libc::c_int as *mut libc::c_char) as libc::c_int != 1i32 {
        return crate::jmorecfg_h::TRUE;
    }
    return crate::jmorecfg_h::FALSE;
}
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 */

unsafe extern "C" fn put_pixel_rows(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
)
/* This version is for writing 24-bit pixels */
{
      let mut outptr:  crate::jpeglib_h::JSAMPROW =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>(); let mut col:  crate::jmorecfg_h::JDIMENSION =  0; let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    
    
    
    
    
    if (*dest).use_inversion_array != 0 {
         let mut image_ptr:   crate::jpeglib_h::JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1u32,
            crate::jmorecfg_h::TRUE,
        );
        (*dest).cur_output_row =  (*dest).cur_output_row + 1;
        outptr = *image_ptr.offset(0)
    } else {
        outptr = (*dest).iobuffer
    }
    /* Transfer data.  Note destination values must be in BGR order
     * (even though Microsoft's own documents say the opposite).
     */
     let mut inptr:   crate::jpeglib_h::JSAMPROW =  *(*dest).pub_0.buffer.offset(0);
    if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_EXT_BGR
    {
        crate::stdlib::memcpy(
            outptr as *mut libc::c_void,
            inptr as *const libc::c_void,
            (*dest).row_width as crate::stddef_h::size_t,
        );
        outptr = outptr.offset(((*cinfo).output_width * 3u32) as isize)
    } else if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB565
    {
        let mut big_endian: crate::jmorecfg_h::boolean = is_big_endian();
        let mut inptr2: *mut libc::c_ushort = inptr as *mut libc::c_ushort;
        col = (*cinfo).output_width;
        while col > 0u32 {
            if big_endian != 0 {
                *outptr.offset(0) =
                    (*inptr2 as libc::c_int >> 5i32 & 0xf8i32) as crate::jmorecfg_h::JSAMPLE;
                *outptr.offset(1) = ((*inptr2 as libc::c_int) << 5i32 & 0xe0i32
                    | *inptr2 as libc::c_int >> 11i32 & 0x1ci32)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr.offset(2) = (*inptr2 as libc::c_int & 0xf8i32) as crate::jmorecfg_h::JSAMPLE
            } else {
                *outptr.offset(0) =
                    ((*inptr2 as libc::c_int) << 3i32 & 0xf8i32) as crate::jmorecfg_h::JSAMPLE;
                *outptr.offset(1) =
                    (*inptr2 as libc::c_int >> 3i32 & 0xfci32) as crate::jmorecfg_h::JSAMPLE;
                *outptr.offset(2) =
                    (*inptr2 as libc::c_int >> 8i32 & 0xf8i32) as crate::jmorecfg_h::JSAMPLE
            }
            outptr = outptr.offset(3);
            inptr2 = inptr2.offset(1);
            col -=  1
        }
    } else if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_CMYK
    {
        col = (*cinfo).output_width;
        while col > 0u32 {
            /* can omit GETJSAMPLE() safely */
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            let mut c: crate::jmorecfg_h::JSAMPLE = *fresh0;
            let fresh1 = inptr;
            inptr = inptr.offset(1);
            let mut m: crate::jmorecfg_h::JSAMPLE = *fresh1;
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            let mut y: crate::jmorecfg_h::JSAMPLE = *fresh2;
            let fresh3 = inptr;
            inptr = inptr.offset(1);
            let mut k: crate::jmorecfg_h::JSAMPLE = *fresh3;
            crate::cmyk_h::cmyk_to_rgb(c, m, y, k, outptr.offset(2), outptr.offset(1), outptr);
            outptr = outptr.offset(3);
            col -=  1
        }
    } else {
        let mut rindex: libc::c_int = crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize];
        let mut gindex: libc::c_int =
            crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize];
        let mut bindex: libc::c_int =
            crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize];
        let mut ps: libc::c_int = rgb_pixelsize[(*cinfo).out_color_space as usize];
        col = (*cinfo).output_width;
        while col > 0u32 {
            /* can omit GETJSAMPLE() safely */
            *outptr.offset(0) = *inptr.offset(bindex as isize);
            *outptr.offset(1) = *inptr.offset(gindex as isize);
            *outptr.offset(2) = *inptr.offset(rindex as isize);
            outptr = outptr.offset(3);
            inptr = inptr.offset(ps as isize);
            col -=  1
        }
    }
     let mut pad:   libc::c_int =  (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0i32) {
            break;
        }
        let fresh4 = outptr;
        outptr = outptr.offset(1);
        *fresh4 = 0u8
    }
    if (*dest).use_inversion_array == 0 {
        crate::stdlib::fwrite(
            (*dest).iobuffer as *const libc::c_void,
            1u64,
            (*dest).row_width as crate::stddef_h::size_t,
            (*dest).pub_0.output_file,
        );
    };
}

unsafe extern "C" fn put_gray_rows(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
)
/* This version is for grayscale OR quantized color output */
{
      let mut outptr:  crate::jpeglib_h::JSAMPROW =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>(); let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    
    
    
    
    if (*dest).use_inversion_array != 0 {
         let mut image_ptr:   crate::jpeglib_h::JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1u32,
            crate::jmorecfg_h::TRUE,
        );
        (*dest).cur_output_row =  (*dest).cur_output_row + 1;
        outptr = *image_ptr.offset(0)
    } else {
        outptr = (*dest).iobuffer
    }
     let mut inptr:   crate::jpeglib_h::JSAMPROW =  *(*dest).pub_0.buffer.offset(0);
    crate::stdlib::memcpy(
        outptr as *mut libc::c_void,
        inptr as *const libc::c_void,
        (*cinfo).output_width as crate::stddef_h::size_t,
    );
    outptr = outptr.offset((*cinfo).output_width as isize);
     let mut pad:   libc::c_int =  (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0i32) {
            break;
        }
        let fresh5 = outptr;
        outptr = outptr.offset(1);
        *fresh5 = 0u8
    }
    if (*dest).use_inversion_array == 0 {
        crate::stdlib::fwrite(
            (*dest).iobuffer as *const libc::c_void,
            1u64,
            (*dest).row_width as crate::stddef_h::size_t,
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

unsafe extern "C" fn write_bmp_header(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dest: bmp_dest_ptr,
)
/* Write a Windows-style BMP file header, including colormap if needed */
{
    
    
    
    
    
     let mut bmpfileheader:  [libc::c_char; 14] =  [0; 14]; let mut bmpinfoheader:  [libc::c_char; 40] =  [0; 40];   let mut bits_per_pixel:  libc::c_int =  0; let mut cmap_entries:  libc::c_int =  0;
    /* Compute colormap size and total file size */
    if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB
        ||  (*cinfo).out_color_space
            >=  crate::jpeglib_h::JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                <=  crate::jpeglib_h::JCS_EXT_ARGB
    {
        if (*cinfo).quantize_colors != 0 {
            /* Colormapped RGB */
            bits_per_pixel = 8i32;
            cmap_entries = 256i32
        } else {
            /* Unquantized, full color RGB */
            bits_per_pixel = 24i32;
            cmap_entries = 0i32
        }
    } else if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB565
        ||  (*cinfo).out_color_space
            ==  crate::jpeglib_h::JCS_CMYK
    {
        bits_per_pixel = 24i32;
        cmap_entries = 0i32
    } else {
        /* Grayscale output.  We need to fake a 256-entry colormap. */
        bits_per_pixel = 8i32;
        cmap_entries = 256i32
    }
     /* Header and colormap */
     let mut headersize:   libc::c_long =
     (14i32 + 40i32 + cmap_entries * 4i32) as libc::c_long; let mut bfSize:   libc::c_long =
    
        headersize + (*dest).row_width as libc::c_long * (*cinfo).output_height as libc::c_long;
    /* Set unused fields of header to 0 */
    crate::stdlib::memset(
        bmpfileheader.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        bmpinfoheader.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
    );
    /* Fill the file header */
    bmpfileheader[0] = 0x42i8; /* first 2 bytes are ASCII 'B', 'M' */
    bmpfileheader[1] = 0x4di8; /* bfSize */
    bmpfileheader[2] = (bfSize & 0xffi64) as libc::c_char;
    bmpfileheader[(2i32 + 1i32) as usize] =
        (bfSize >> 8i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(2i32 + 2i32) as usize] =
        (bfSize >> 16i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(2i32 + 3i32) as usize] =
        (bfSize >> 24i32 & 0xffi64) as libc::c_char;
    /* we leave bfReserved1 & bfReserved2 = 0 */
    bmpfileheader[10] = (headersize & 0xffi64) as libc::c_char; /* bfOffBits */
    bmpfileheader[(10i32 + 1i32) as usize] =
        (headersize >> 8i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(10i32 + 2i32) as usize] =
        (headersize >> 16i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(10i32 + 3i32) as usize] =
        (headersize >> 24i32 & 0xffi64) as libc::c_char;
    /* Fill the info header (Microsoft calls this a BITMAPINFOHEADER) */
    bmpinfoheader[0] = (40i32 & 0xffi32) as libc::c_char; /* biSize */
    bmpinfoheader[(0i32 + 1i32) as usize] = (40i32 >> 8i32 & 0xffi32) as libc::c_char; /* biWidth */
    bmpinfoheader[4] = ((*cinfo).output_width & 0xffu32) as libc::c_char; /* biHeight */
    bmpinfoheader[(4i32 + 1i32) as usize] =
        ((*cinfo).output_width >> 8i32 & 0xffu32) as libc::c_char; /* biPlanes - must be 1 */
    bmpinfoheader[(4i32 + 2i32) as usize] =
        ((*cinfo).output_width >> 16i32 & 0xffu32) as libc::c_char; /* biBitCount */
    bmpinfoheader[(4i32 + 3i32) as usize] =
        ((*cinfo).output_width >> 24i32 & 0xffu32) as libc::c_char;
    bmpinfoheader[8] = ((*cinfo).output_height & 0xffu32) as libc::c_char;
    bmpinfoheader[(8i32 + 1i32) as usize] =
        ((*cinfo).output_height >> 8i32 & 0xffu32) as libc::c_char;
    bmpinfoheader[(8i32 + 2i32) as usize] =
        ((*cinfo).output_height >> 16i32 & 0xffu32) as libc::c_char;
    bmpinfoheader[(8i32 + 3i32) as usize] =
        ((*cinfo).output_height >> 24i32 & 0xffu32) as libc::c_char;
    bmpinfoheader[12] = (1i32 & 0xffi32) as libc::c_char;
    bmpinfoheader[(12i32 + 1i32) as usize] = (1i32 >> 8i32 & 0xffi32) as libc::c_char;
    bmpinfoheader[14] = (bits_per_pixel & 0xffi32) as libc::c_char;
    bmpinfoheader[(14i32 + 1i32) as usize] = (bits_per_pixel >> 8i32 & 0xffi32) as libc::c_char;
    /* we leave biCompression = 0, for none */
    /* we leave biSizeImage = 0; this is correct for uncompressed data */
    if (*cinfo).density_unit as libc::c_int == 2i32 {
        /* if have density in dots/cm, then */
        bmpinfoheader[24] = (((*cinfo).X_density as libc::c_int * 100i32) as libc::c_long
            & 0xffi64) as libc::c_char; /* XPels/M */
        bmpinfoheader[(24i32 + 1i32) as usize] =
            (((*cinfo).X_density as libc::c_int * 100i32) as libc::c_long >> 8i32
                & 0xffi64) as libc::c_char;
        bmpinfoheader[(24i32 + 2i32) as usize] =
            (((*cinfo).X_density as libc::c_int * 100i32) as libc::c_long >> 16i32
                & 0xffi64) as libc::c_char;
        bmpinfoheader[(24i32 + 3i32) as usize] =
            (((*cinfo).X_density as libc::c_int * 100i32) as libc::c_long >> 24i32
                & 0xffi64) as libc::c_char;
        bmpinfoheader[28] = (((*cinfo).Y_density as libc::c_int * 100i32) as libc::c_long
            & 0xffi64) as libc::c_char;
        bmpinfoheader[(28i32 + 1i32) as usize] =
            (((*cinfo).Y_density as libc::c_int * 100i32) as libc::c_long >> 8i32
                & 0xffi64) as libc::c_char;
        bmpinfoheader[(28i32 + 2i32) as usize] =
            (((*cinfo).Y_density as libc::c_int * 100i32) as libc::c_long >> 16i32
                & 0xffi64) as libc::c_char;
        bmpinfoheader[(28i32 + 3i32) as usize] =
            (((*cinfo).Y_density as libc::c_int * 100i32) as libc::c_long >> 24i32
                & 0xffi64) as libc::c_char
        /* XPels/M */
    } /* biClrUsed */
    bmpinfoheader[32] = (cmap_entries & 0xffi32) as libc::c_char;
    bmpinfoheader[(32i32 + 1i32) as usize] = (cmap_entries >> 8i32 & 0xffi32) as libc::c_char;
    /* we leave biClrImportant = 0 */
    if crate::stdlib::fwrite(
        bmpfileheader.as_mut_ptr() as *const libc::c_void,
        1u64,
        14u64,
        (*dest).pub_0.output_file,
    ) != 14u64
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_FILE_WRITE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if crate::stdlib::fwrite(
        bmpinfoheader.as_mut_ptr() as *const libc::c_void,
        1u64,
        40u64,
        (*dest).pub_0.output_file,
    ) != 40u64
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_FILE_WRITE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if cmap_entries > 0i32 {
        write_colormap(cinfo, dest, cmap_entries, 4i32);
    };
}

unsafe extern "C" fn write_os2_header(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dest: bmp_dest_ptr,
)
/* Write an OS2-style BMP file header, including colormap if needed */
{
    
    
    
    
    
     let mut bmpfileheader:  [libc::c_char; 14] =  [0; 14]; let mut bmpcoreheader:  [libc::c_char; 12] =  [0; 12];   let mut bits_per_pixel:  libc::c_int =  0; let mut cmap_entries:  libc::c_int =  0;
    /* Compute colormap size and total file size */
    if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB
        ||  (*cinfo).out_color_space
            >=  crate::jpeglib_h::JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                <=  crate::jpeglib_h::JCS_EXT_ARGB
    {
        if (*cinfo).quantize_colors != 0 {
            /* Colormapped RGB */
            bits_per_pixel = 8i32;
            cmap_entries = 256i32
        } else {
            /* Unquantized, full color RGB */
            bits_per_pixel = 24i32;
            cmap_entries = 0i32
        }
    } else if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB565
        ||  (*cinfo).out_color_space
            ==  crate::jpeglib_h::JCS_CMYK
    {
        bits_per_pixel = 24i32;
        cmap_entries = 0i32
    } else {
        /* Grayscale output.  We need to fake a 256-entry colormap. */
        bits_per_pixel = 8i32;
        cmap_entries = 256i32
    }
    /* File size */
     /* Header and colormap */
     let mut headersize:   libc::c_long =
     (14i32 + 12i32 + cmap_entries * 3i32) as libc::c_long; let mut bfSize:   libc::c_long =
    
        headersize + (*dest).row_width as libc::c_long * (*cinfo).output_height as libc::c_long;
    /* Set unused fields of header to 0 */
    crate::stdlib::memset(
        bmpfileheader.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        bmpcoreheader.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    /* Fill the file header */
    bmpfileheader[0] = 0x42i8; /* first 2 bytes are ASCII 'B', 'M' */
    bmpfileheader[1] = 0x4di8; /* bfSize */
    bmpfileheader[2] = (bfSize & 0xffi64) as libc::c_char;
    bmpfileheader[(2i32 + 1i32) as usize] =
        (bfSize >> 8i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(2i32 + 2i32) as usize] =
        (bfSize >> 16i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(2i32 + 3i32) as usize] =
        (bfSize >> 24i32 & 0xffi64) as libc::c_char;
    /* we leave bfReserved1 & bfReserved2 = 0 */
    bmpfileheader[10] = (headersize & 0xffi64) as libc::c_char; /* bfOffBits */
    bmpfileheader[(10i32 + 1i32) as usize] =
        (headersize >> 8i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(10i32 + 2i32) as usize] =
        (headersize >> 16i32 & 0xffi64) as libc::c_char;
    bmpfileheader[(10i32 + 3i32) as usize] =
        (headersize >> 24i32 & 0xffi64) as libc::c_char;
    /* Fill the info header (Microsoft calls this a BITMAPCOREHEADER) */
    bmpcoreheader[0] = (12i32 & 0xffi32) as libc::c_char; /* bcSize */
    bmpcoreheader[(0i32 + 1i32) as usize] = (12i32 >> 8i32 & 0xffi32) as libc::c_char; /* bcWidth */
    bmpcoreheader[4] = ((*cinfo).output_width & 0xffu32) as libc::c_char; /* bcHeight */
    bmpcoreheader[(4i32 + 1i32) as usize] =
        ((*cinfo).output_width >> 8i32 & 0xffu32) as libc::c_char; /* bcPlanes - must be 1 */
    bmpcoreheader[6] = ((*cinfo).output_height & 0xffu32) as libc::c_char; /* bcBitCount */
    bmpcoreheader[(6i32 + 1i32) as usize] =
        ((*cinfo).output_height >> 8i32 & 0xffu32) as libc::c_char;
    bmpcoreheader[8] = (1i32 & 0xffi32) as libc::c_char;
    bmpcoreheader[(8i32 + 1i32) as usize] = (1i32 >> 8i32 & 0xffi32) as libc::c_char;
    bmpcoreheader[10] = (bits_per_pixel & 0xffi32) as libc::c_char;
    bmpcoreheader[(10i32 + 1i32) as usize] = (bits_per_pixel >> 8i32 & 0xffi32) as libc::c_char;
    if crate::stdlib::fwrite(
        bmpfileheader.as_mut_ptr() as *const libc::c_void,
        1u64,
        14u64,
        (*dest).pub_0.output_file,
    ) != 14u64
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_FILE_WRITE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if crate::stdlib::fwrite(
        bmpcoreheader.as_mut_ptr() as *const libc::c_void,
        1u64,
        12u64,
        (*dest).pub_0.output_file,
    ) != 12u64
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_FILE_WRITE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dest: bmp_dest_ptr,
    mut map_colors: libc::c_int,
    mut map_entry_size: libc::c_int,
) {
     let mut i:  libc::c_int =  0;let mut colormap: crate::jpeglib_h::JSAMPARRAY = (*cinfo).colormap;
    let mut num_colors: libc::c_int = (*cinfo).actual_number_of_colors;
    let mut outfile: *mut crate::stdlib::FILE = (*dest).pub_0.output_file;
    
    if !colormap.is_null() {
        if (*cinfo).out_color_components == 3i32 {
            /* Normal case with RGB colormap */
            i = 0i32;
            while i < num_colors {
                crate::stdlib::putc(
                    *(*colormap.offset(2)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                crate::stdlib::putc(
                    *(*colormap.offset(1)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                crate::stdlib::putc(
                    *(*colormap.offset(0)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                if map_entry_size == 4i32 {
                    crate::stdlib::putc(0i32, outfile);
                }
                i += 1
            }
        } else {
            /* Grayscale colormap (only happens with grayscale quantization) */
            i = 0i32;
            while i < num_colors {
                crate::stdlib::putc(
                    *(*colormap.offset(0)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                crate::stdlib::putc(
                    *(*colormap.offset(0)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                crate::stdlib::putc(
                    *(*colormap.offset(0)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                if map_entry_size == 4i32 {
                    crate::stdlib::putc(0i32, outfile);
                }
                i += 1
            }
        }
    } else {
        /* If no colormap, must be grayscale data.  Generate a linear "map". */
        i = 0i32;
        while i < 256i32 {
            crate::stdlib::putc(i, outfile);
            crate::stdlib::putc(i, outfile);
            crate::stdlib::putc(i, outfile);
            if map_entry_size == 4i32 {
                crate::stdlib::putc(0i32, outfile);
            }
            i += 1
        }
    }
    /* Pad colormap with zeros to ensure specified number of colormap entries */
    if i > map_colors {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_TOO_MANY_COLORS as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = i;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    while i < map_colors {
        crate::stdlib::putc(0i32, outfile);
        crate::stdlib::putc(0i32, outfile);
        crate::stdlib::putc(0i32, outfile);
        if map_entry_size == 4i32 {
            crate::stdlib::putc(0i32, outfile);
        }
        i += 1
    }
}
/*
 * Startup: write the file header unless the inversion array is being used.
 */

unsafe extern "C" fn start_output_bmp(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    if (*dest).use_inversion_array == 0 {
        /* Write the header and colormap */
        if (*dest).is_os2 != 0 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
    };
}

unsafe extern "C" fn finish_output_bmp(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut outfile: *mut crate::stdlib::FILE = (*dest).pub_0.output_file;
    
    
    
    
    let mut progress: crate::src::cdjpeg::cd_progress_ptr =
        (*cinfo).progress as crate::src::cdjpeg::cd_progress_ptr;
    if (*dest).use_inversion_array != 0 {
        /* Write the header and colormap */
         if (*dest).is_os2 != 0 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
        /* Write the file body from our virtual array */
         let mut row:   crate::jmorecfg_h::JDIMENSION =  (*cinfo).output_height;
        while row > 0u32 {
               if !progress.is_null() {
                (*progress).pub_0.pass_counter =
                    (
                    (*cinfo).output_height - row) as libc::c_long;
                (*progress).pub_0.pass_limit = (*cinfo).output_height as libc::c_long;
                Some(
                    (*progress)
                        .pub_0
                        .progress_monitor
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            
            
             let mut image_ptr:   crate::jpeglib_h::JSAMPARRAY =
     Some(
                (*(*cinfo).mem)
                    .access_virt_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                (*dest).whole_image,
                
                row - 1u32,
                1u32,
                crate::jmorecfg_h::FALSE,
            ); let mut data_ptr:   crate::jpeglib_h::JSAMPROW =  *image_ptr.offset(0); let mut col:   crate::jmorecfg_h::JDIMENSION =  (*dest).row_width;
            while col > 0u32 {
                crate::stdlib::putc(*data_ptr as libc::c_int, outfile);
                data_ptr = data_ptr.offset(1);
                col -=  1
            }
            row -=  1
        }
        if !progress.is_null() {
            (*progress).completed_extra_passes += 1
        }
    }
    /* Make sure we wrote the output file OK */
    crate::stdlib::fflush(outfile);
    if crate::stdlib::ferror(outfile) != 0 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_FILE_WRITE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    };
}
/*
 * The module selection routine for BMP format output.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_write_bmp(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut is_os2: crate::jmorecfg_h::boolean,
    mut use_inversion_array: crate::jmorecfg_h::boolean,
) -> crate::src::cdjpeg::djpeg_dest_ptr {
    
      let mut row_width:  crate::jmorecfg_h::JDIMENSION =  0;
    /* Create module interface object, fill in method pointers */
     let mut dest:   bmp_dest_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<bmp_dest_struct>() as libc::c_ulong,
    ) as bmp_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_bmp
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_bmp
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = ::std::mem::transmute::<
        libc::intptr_t,
        Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
        >,
    >(crate::stddef_h::NULL as libc::intptr_t);
    (*dest).is_os2 = is_os2;
    if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_GRAYSCALE
    {
        (*dest).pub_0.put_pixel_rows = Some(
            put_gray_rows
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::src::cdjpeg::djpeg_dest_ptr,
                    _: crate::jmorecfg_h::JDIMENSION,
                ) -> (),
        )
    } else if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB
        ||  (*cinfo).out_color_space
            >=  crate::jpeglib_h::JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                <=  crate::jpeglib_h::JCS_EXT_ARGB
    {
        if (*cinfo).quantize_colors != 0 {
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        } else {
            (*dest).pub_0.put_pixel_rows = Some(
                put_pixel_rows
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        }
    } else if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB565
        ||  (*cinfo).out_color_space
            ==  crate::jpeglib_h::JCS_CMYK
    {
        (*dest).pub_0.put_pixel_rows = Some(
            put_pixel_rows
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::src::cdjpeg::djpeg_dest_ptr,
                    _: crate::jmorecfg_h::JDIMENSION,
                ) -> (),
        )
    } else {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_COLORSPACE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Calculate output image dimensions so we can allocate space */
    crate::jpeglib_h::jpeg_calc_output_dimensions(cinfo);
    /* Determine width of rows in the BMP file (padded to 4-byte boundary). */
    if  (*cinfo).out_color_space
        ==  crate::jpeglib_h::JCS_RGB565
    {
        row_width =  (*cinfo).output_width * 2u32;
        (*dest).data_width =  (*cinfo).output_width * 3u32;
        (*dest).row_width = (*dest).data_width;
        while row_width & 3u32 != 0u32 {
            row_width +=  1
        }
    } else if (*cinfo).quantize_colors == 0
        && ((*cinfo).out_color_space
            ==  crate::jpeglib_h::JCS_RGB
            ||  (*cinfo).out_color_space
                >=  crate::jpeglib_h::JCS_EXT_RGB
                &&  (*cinfo).out_color_space
                    <=  crate::jpeglib_h::JCS_EXT_ARGB
            ||  (*cinfo).out_color_space
                ==  crate::jpeglib_h::JCS_CMYK)
    {
        row_width =  (*cinfo)
            .output_width * (*cinfo).output_components as libc::c_uint;
        (*dest).data_width =  (*cinfo).output_width * 3u32;
        (*dest).row_width = (*dest).data_width
    } else {
        row_width =  (*cinfo)
            .output_width * (*cinfo).output_components as libc::c_uint;
        (*dest).data_width = row_width;
        (*dest).row_width = (*dest).data_width
    }
    while (*dest).row_width & 3u32 != 0u32 {
        (*dest).row_width =  (*dest).row_width + 1
    }
    (*dest).pad_bytes = ( (*dest).row_width - (*dest).data_width) as libc::c_int;
    if use_inversion_array != 0 {
        /* Allocate space for inversion array, prepare for write pass */
        (*dest).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            crate::jmorecfg_h::FALSE,
            (*dest).row_width,
            (*cinfo).output_height,
            1u32,
        );
        (*dest).cur_output_row = 0u32;
        if !(*cinfo).progress.is_null() {
            let mut progress: crate::src::cdjpeg::cd_progress_ptr =
                (*cinfo).progress as crate::src::cdjpeg::cd_progress_ptr;
            (*progress).total_extra_passes += 1
            /* count file input as separate pass */
        }
    } else {
        (*dest).iobuffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            (*dest).row_width as crate::stddef_h::size_t,
        ) as *mut crate::jmorecfg_h::JSAMPLE
    }
    (*dest).use_inversion_array = use_inversion_array;
    /* Create decompressor output buffer. */
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        row_width,
        1u32,
    );
    (*dest).pub_0.buffer_height = 1u32;
    return dest as crate::src::cdjpeg::djpeg_dest_ptr;
}
/* BMP_SUPPORTED */
