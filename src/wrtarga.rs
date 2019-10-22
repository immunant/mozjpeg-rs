pub use super::cdjpeg::{djpeg_dest_ptr, djpeg_dest_struct};
pub use super::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::cderror_h::{
    C2RustUnnamed_4, JERR_BAD_CMAP_FILE, JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH, JERR_BMP_BADHEADER,
    JERR_BMP_BADPLANES, JERR_BMP_COLORSPACE, JERR_BMP_COMPRESSED, JERR_BMP_EMPTY, JERR_BMP_NOT,
    JERR_BMP_OUTOFRANGE, JERR_GIF_BUG, JERR_GIF_CODESIZE, JERR_GIF_COLORSPACE,
    JERR_GIF_IMAGENOTFOUND, JERR_GIF_NOT, JERR_PPM_COLORSPACE, JERR_PPM_NONNUMERIC, JERR_PPM_NOT,
    JERR_PPM_OUTOFRANGE, JERR_TGA_BADCMAP, JERR_TGA_BADPARMS, JERR_TGA_COLORSPACE,
    JERR_TOO_MANY_COLORS, JERR_UNGETC_FAILED, JERR_UNKNOWN_FORMAT, JERR_UNSUPPORTED_FORMAT,
    JMSG_FIRSTADDONCODE, JMSG_LASTADDONCODE, JTRC_BMP, JTRC_BMP_MAPPED, JTRC_BMP_OS2,
    JTRC_BMP_OS2_MAPPED, JTRC_GIF, JTRC_GIF_BADVERSION, JTRC_GIF_EXTENSION, JTRC_GIF_NONSQUARE,
    JTRC_PGM, JTRC_PGM_TEXT, JTRC_PPM, JTRC_PPM_TEXT, JTRC_TGA, JTRC_TGA_GRAY, JTRC_TGA_MAPPED,
    JWRN_GIF_BADDATA, JWRN_GIF_CHAR, JWRN_GIF_ENDCODE, JWRN_GIF_NOMOREDATA,
};
pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_calc_output_dimensions, jpeg_color_deconverter,
    jpeg_color_quantizer, jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller,
    jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct,
    jpeg_entropy_decoder, jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
    JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
    JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS,
    JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPROW,
    J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, FILE, _IO_FILE,
};
use crate::stdlib::{ferror, fflush, fwrite, memset, putc};
use libc::{self, c_char, c_int, c_uint, c_ulong, c_void};

pub type tga_dest_ptr = *mut tga_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tga_dest_struct {
    pub pub_0: super::cdjpeg::djpeg_dest_struct,
    pub iobuffer: *mut c_char,
    pub buffer_width: JDIMENSION,
}

unsafe extern "C" fn write_header(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut num_colors: c_int,
)
/* Create and write a Targa header */
{
    let mut targaheader: [c_char; 18] = [0; 18];
    /* Set unused fields of header to 0 */
    memset(
        targaheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 18]>() as c_ulong,
    ); /* color map type 1 */
    if num_colors > 0i32 {
        targaheader[1] = 1i32 as c_char;
        targaheader[5] = (num_colors & 0xffi32) as c_char;
        targaheader[6] = (num_colors >> 8i32) as c_char;
        targaheader[7] = 24i32 as c_char
        /* 24 bits per cmap entry */
    } /* Top-down, non-interlaced */
    targaheader[12] = ((*cinfo).output_width & 0xffi32 as c_uint) as c_char; /* image type = uncompressed grayscale */
    targaheader[13] = ((*cinfo).output_width >> 8i32) as c_char;
    targaheader[14] = ((*cinfo).output_height & 0xffi32 as c_uint) as c_char;
    targaheader[15] = ((*cinfo).output_height >> 8i32) as c_char;
    targaheader[17] = 0x20i32 as c_char;
    if (*cinfo).out_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
        targaheader[2] = 3i32 as c_char;
        targaheader[16] = 8i32 as c_char
    /* bits per pixel */
    } else if num_colors > 0i32 {
        targaheader[2] = 1i32 as c_char; /* image type = colormapped RGB */
        targaheader[16] = 8i32 as c_char
    } else {
        targaheader[2] = 2i32 as c_char; /* image type = uncompressed RGB */
        targaheader[16] = 24i32 as c_char
    }
    if fwrite(
        targaheader.as_mut_ptr() as *const c_void,
        1i32 as size_t,
        18i32 as size_t,
        (*dinfo).output_file,
    ) != 18i32 as size_t
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 */

unsafe extern "C" fn put_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
)
/* used for unquantized full-color output */
{
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr; /* RGB to BGR order */
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: *mut c_char = 0 as *mut c_char;
    let mut col: JDIMENSION = 0;
    inptr = *(*dest).pub_0.buffer.offset(0);
    outptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        *outptr.offset(0) = *inptr.offset(2) as c_int as c_char;
        *outptr.offset(1) = *inptr.offset(1) as c_int as c_char;
        *outptr.offset(2) = *inptr.offset(0) as c_int as c_char;
        inptr = inptr.offset(3);
        outptr = outptr.offset(3);
        col = col.wrapping_sub(1)
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width as size_t,
        (*dest).pub_0.output_file,
    );
}

unsafe extern "C" fn put_gray_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
)
/* used for grayscale OR quantized color output */
{
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: *mut c_char = 0 as *mut c_char;
    let mut col: JDIMENSION = 0;
    inptr = *(*dest).pub_0.buffer.offset(0);
    outptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        let fresh0 = inptr;
        inptr = inptr.offset(1);
        let fresh1 = outptr;
        outptr = outptr.offset(1);
        *fresh1 = *fresh0 as c_int as c_char;
        col = col.wrapping_sub(1)
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width as size_t,
        (*dest).pub_0.output_file,
    );
}
/*
 * Write some demapped pixel data when color quantization is in effect.
 * For Targa, this is only applied to grayscale data.
 */

unsafe extern "C" fn put_demapped_gray(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
) {
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: *mut c_char = 0 as *mut c_char;
    let mut color_map0: JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut col: JDIMENSION = 0;
    inptr = *(*dest).pub_0.buffer.offset(0);
    outptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        let fresh2 = inptr;
        inptr = inptr.offset(1);
        let fresh3 = outptr;
        outptr = outptr.offset(1);
        *fresh3 = *color_map0.offset(*fresh2 as c_int as isize) as c_int as c_char;
        col = col.wrapping_sub(1)
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width as size_t,
        (*dest).pub_0.output_file,
    );
}
/*
 * Startup: write the file header.
 */

unsafe extern "C" fn start_output_tga(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    let mut num_colors: c_int = 0;
    let mut i: c_int = 0;
    let mut outfile: *mut FILE = 0 as *mut FILE;
    if (*cinfo).out_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
        /* Targa doesn't have a mapped grayscale format, so we will */
        /* demap quantized gray output.  Never emit a colormap. */
        write_header(cinfo, dinfo, 0i32);
        if (*cinfo).quantize_colors != 0 {
            (*dest).pub_0.put_pixel_rows = Some(
                put_demapped_gray
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else {
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
    } else if (*cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint {
        if (*cinfo).quantize_colors != 0 {
            /* We only support 8-bit colormap indexes, so only 256 colors */
            num_colors = (*cinfo).actual_number_of_colors;
            if num_colors > 256i32 {
                (*(*cinfo).err).msg_code = JERR_TOO_MANY_COLORS as c_int;
                (*(*cinfo).err).msg_parm.i[0] = num_colors;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            write_header(cinfo, dinfo, num_colors);
            /* Write the colormap.  Note Targa uses BGR byte order */
            outfile = (*dest).pub_0.output_file;
            i = 0i32;
            while i < num_colors {
                putc(
                    *(*(*cinfo).colormap.offset(2)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*(*cinfo).colormap.offset(1)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*(*cinfo).colormap.offset(0)).offset(i as isize) as c_int,
                    outfile,
                );
                i += 1
            }
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else {
            write_header(cinfo, dinfo, 0i32);
            (*dest).pub_0.put_pixel_rows = Some(
                put_pixel_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
    } else {
        (*(*cinfo).err).msg_code = JERR_TGA_COLORSPACE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_output_tga(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    /* Make sure we wrote the output file OK */
    fflush((*dinfo).output_file);
    if ferror((*dinfo).output_file) != 0 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/*
 * Re-calculate buffer dimensions based on output dimensions.
 */

unsafe extern "C" fn calc_buffer_dimensions_tga(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: tga_dest_ptr = dinfo as tga_dest_ptr;
    (*dest).buffer_width = (*cinfo)
        .output_width
        .wrapping_mul((*cinfo).output_components as c_uint);
}
/*
 * The module selection routine for Targa format output.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_write_targa(
    mut cinfo: j_decompress_ptr,
) -> super::cdjpeg::djpeg_dest_ptr {
    let mut dest: tga_dest_ptr = 0 as *mut tga_dest_struct;
    /* Create module interface object, fill in method pointers */
    dest = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<tga_dest_struct>() as c_ulong,
    ) as tga_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_tga
            as unsafe extern "C" fn(_: j_decompress_ptr, _: super::cdjpeg::djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_tga
            as unsafe extern "C" fn(_: j_decompress_ptr, _: super::cdjpeg::djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_tga
            as unsafe extern "C" fn(_: j_decompress_ptr, _: super::cdjpeg::djpeg_dest_ptr) -> (),
    );
    /* Calculate output image dimensions so we can allocate space */
    jpeg_calc_output_dimensions(cinfo);
    /* Create I/O buffer. */
    (*dest)
        .pub_0
        .calc_buffer_dimensions
        .expect("non-null function pointer")(cinfo, dest as super::cdjpeg::djpeg_dest_ptr);
    (*dest).iobuffer = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((*dest).buffer_width as c_ulong).wrapping_mul(::std::mem::size_of::<c_char>() as c_ulong),
    ) as *mut c_char;
    /* Create decompressor output buffer. */
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (*dest).buffer_width,
        1i32 as JDIMENSION,
    );
    (*dest).pub_0.buffer_height = 1i32 as JDIMENSION;
    return dest as super::cdjpeg::djpeg_dest_ptr;
}
/* TARGA_SUPPORTED */
