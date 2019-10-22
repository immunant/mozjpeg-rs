use libc;

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
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
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
use crate::stdlib::ferror;
use crate::stdlib::fflush;
use crate::stdlib::fwrite;
use crate::stdlib::memset;
use crate::stdlib::putc;

pub type tga_dest_ptr = *mut tga_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tga_dest_struct {
    pub pub_0: crate::src::cdjpeg::djpeg_dest_struct,
    pub iobuffer: *mut libc::c_char,
    pub buffer_width: crate::jmorecfg_h::JDIMENSION,
}

unsafe extern "C" fn write_header(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut num_colors: libc::c_int,
)
/* Create and write a Targa header */
{
    let mut targaheader: [libc::c_char; 18] = [0; 18];
    /* Set unused fields of header to 0 */
    crate::stdlib::memset(
        targaheader.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong,
    ); /* color map type 1 */
    if num_colors > 0i32 {
        targaheader[1] = 1i32 as libc::c_char;
        targaheader[5] = (num_colors & 0xffi32) as libc::c_char;
        targaheader[6] = (num_colors >> 8i32) as libc::c_char;
        targaheader[7] = 24i32 as libc::c_char
        /* 24 bits per cmap entry */
    } /* Top-down, non-interlaced */
    targaheader[12] = ((*cinfo).output_width & 0xffi32 as libc::c_uint) as libc::c_char; /* image type = uncompressed grayscale */
    targaheader[13] = ((*cinfo).output_width >> 8i32) as libc::c_char;
    targaheader[14] = ((*cinfo).output_height & 0xffi32 as libc::c_uint) as libc::c_char;
    targaheader[15] = ((*cinfo).output_height >> 8i32) as libc::c_char;
    targaheader[17] = 0x20i32 as libc::c_char;
    if (*cinfo).out_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
    {
        targaheader[2] = 3i32 as libc::c_char;
        targaheader[16] = 8i32 as libc::c_char
    /* bits per pixel */
    } else if num_colors > 0i32 {
        targaheader[2] = 1i32 as libc::c_char; /* image type = colormapped RGB */
        targaheader[16] = 8i32 as libc::c_char
    } else {
        targaheader[2] = 2i32 as libc::c_char; /* image type = uncompressed RGB */
        targaheader[16] = 24i32 as libc::c_char
    }
    if crate::stdlib::fwrite(
        targaheader.as_mut_ptr() as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        18i32 as crate::stddef_h::size_t,
        (*dinfo).output_file,
    ) != 18i32 as crate::stddef_h::size_t
    {
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
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 */

unsafe extern "C" fn put_pixel_rows(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
)
/* used for unquantized full-color output */
{
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr; /* RGB to BGR order */
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    inptr = *(*dest).pub_0.buffer.offset(0);
    outptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        *outptr.offset(0) = *inptr.offset(2) as libc::c_int as libc::c_char;
        *outptr.offset(1) = *inptr.offset(1) as libc::c_int as libc::c_char;
        *outptr.offset(2) = *inptr.offset(0) as libc::c_int as libc::c_char;
        inptr = inptr.offset(3);
        outptr = outptr.offset(3);
        col = col.wrapping_sub(1)
    }
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width as crate::stddef_h::size_t,
        (*dest).pub_0.output_file,
    );
}

unsafe extern "C" fn put_gray_rows(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
)
/* used for grayscale OR quantized color output */
{
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    inptr = *(*dest).pub_0.buffer.offset(0);
    outptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        let fresh0 = inptr;
        inptr = inptr.offset(1);
        let fresh1 = outptr;
        outptr = outptr.offset(1);
        *fresh1 = *fresh0 as libc::c_int as libc::c_char;
        col = col.wrapping_sub(1)
    }
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width as crate::stddef_h::size_t,
        (*dest).pub_0.output_file,
    );
}
/*
 * Write some demapped pixel data when color quantization is in effect.
 * For Targa, this is only applied to grayscale data.
 */

unsafe extern "C" fn put_demapped_gray(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut color_map0: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    inptr = *(*dest).pub_0.buffer.offset(0);
    outptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        let fresh2 = inptr;
        inptr = inptr.offset(1);
        let fresh3 = outptr;
        outptr = outptr.offset(1);
        *fresh3 =
            *color_map0.offset(*fresh2 as libc::c_int as isize) as libc::c_int as libc::c_char;
        col = col.wrapping_sub(1)
    }
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width as crate::stddef_h::size_t,
        (*dest).pub_0.output_file,
    );
}
/*
 * Startup: write the file header.
 */

unsafe extern "C" fn start_output_tga(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    let mut num_colors: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut outfile: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    if (*cinfo).out_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
    {
        /* Targa doesn't have a mapped grayscale format, so we will */
        /* demap quantized gray output.  Never emit a colormap. */
        write_header(cinfo, dinfo, 0i32);
        if (*cinfo).quantize_colors != 0 {
            (*dest).pub_0.put_pixel_rows = Some(
                put_demapped_gray
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        } else {
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        }
    } else if (*cinfo).out_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
    {
        if (*cinfo).quantize_colors != 0 {
            /* We only support 8-bit colormap indexes, so only 256 colors */
            num_colors = (*cinfo).actual_number_of_colors;
            if num_colors > 256i32 {
                (*(*cinfo).err).msg_code = crate::cderror_h::JERR_TOO_MANY_COLORS as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0] = num_colors;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            write_header(cinfo, dinfo, num_colors);
            /* Write the colormap.  Note Targa uses BGR byte order */
            outfile = (*dest).pub_0.output_file;
            i = 0i32;
            while i < num_colors {
                crate::stdlib::putc(
                    *(*(*cinfo).colormap.offset(2)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                crate::stdlib::putc(
                    *(*(*cinfo).colormap.offset(1)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                crate::stdlib::putc(
                    *(*(*cinfo).colormap.offset(0)).offset(i as isize) as libc::c_int,
                    outfile,
                );
                i += 1
            }
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        } else {
            write_header(cinfo, dinfo, 0i32);
            (*dest).pub_0.put_pixel_rows = Some(
                put_pixel_rows
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        }
    } else {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_TGA_COLORSPACE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    };
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_output_tga(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    /* Make sure we wrote the output file OK */
    crate::stdlib::fflush((*dinfo).output_file);
    if crate::stdlib::ferror((*dinfo).output_file) != 0 {
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
 * Re-calculate buffer dimensions based on output dimensions.
 */

unsafe extern "C" fn calc_buffer_dimensions_tga(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    (*dest).buffer_width = (*cinfo)
        .output_width
        .wrapping_mul((*cinfo).output_components as libc::c_uint);
}
/*
 * The module selection routine for Targa format output.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_write_targa(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::src::cdjpeg::djpeg_dest_ptr {
    let mut dest: tga_dest_ptr = 0 as *mut tga_dest_struct;
    /* Create module interface object, fill in method pointers */
    dest = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<tga_dest_struct>() as libc::c_ulong,
    ) as tga_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_tga
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_tga
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_tga
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    /* Calculate output image dimensions so we can allocate space */
    crate::jpeglib_h::jpeg_calc_output_dimensions(cinfo);
    /* Create I/O buffer. */
    (*dest)
        .pub_0
        .calc_buffer_dimensions
        .expect("non-null function pointer")(cinfo, dest as crate::src::cdjpeg::djpeg_dest_ptr);
    (*dest).iobuffer = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ((*dest).buffer_width as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    /* Create decompressor output buffer. */
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (*dest).buffer_width,
        1i32 as crate::jmorecfg_h::JDIMENSION,
    );
    (*dest).pub_0.buffer_height = 1i32 as crate::jmorecfg_h::JDIMENSION;
    return dest as crate::src::cdjpeg::djpeg_dest_ptr;
}
/* TARGA_SUPPORTED */
