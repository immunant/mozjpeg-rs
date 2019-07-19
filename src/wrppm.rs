pub use crate::cderror_h::{
    C2RustUnnamed_91, JERR_BAD_CMAP_FILE, JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH, JERR_BMP_BADHEADER,
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
pub use crate::cdjpeg::{djpeg_dest_ptr, djpeg_dest_struct};
pub use crate::cmyk_h::cmyk_to_rgb;
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jerror::{
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
pub use crate::jmorecfg_h::{
    boolean, rgb_blue, rgb_green, rgb_pixelsize, rgb_red, EXT_BGRX_BLUE, EXT_BGRX_GREEN,
    EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE, EXT_BGR_RED,
    EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN,
    EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE,
    EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, JCOEF,
    JDIMENSION, JOCTET, JSAMPLE, RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jpeg_color_deconverter, jpeg_color_quantizer, jpeg_d_coef_controller,
    jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master, jpeg_entropy_decoder,
    jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader, jpeg_upsampler, JBUF_CRANK_DEST,
    JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_calc_output_dimensions, jpeg_common_struct,
    jpeg_component_info, jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_parser_method,
    jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_source_mgr,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL,
    JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
    J_DITHER_MODE,
};
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, FILE, _IO_FILE,
};
use crate::stdlib::{ferror, fflush, fprintf, fwrite, memcpy};
use libc::{self, c_char, c_int, c_long, c_uint, c_ulong, c_void};
pub type ppm_dest_ptr = *mut ppm_dest_struct;
/*
 * When JSAMPLE is the same size as char, we can just fwrite() the
 * decompressed data to the PPM or PGM file.
 */
/* Private version of data destination object */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppm_dest_struct {
    pub pub_0: djpeg_dest_struct,
    pub iobuffer: *mut c_char,
    pub pixrow: JSAMPROW,
    pub buffer_width: size_t,
    pub samples_per_row: JDIMENSION,
}
/*
 * wrppm.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * Modified 2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to write output images in PPM/PGM format.
 * The extended 2-byte-per-sample raw PPM/PGM formats are supported.
 * The PBMPLUS library is NOT required to compile this software
 * (but it is highly useful as a set of PPM image manipulation programs).
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume output to
 * an ordinary stdio stream.
 */
/*
 * For 12-bit JPEG data, we either downscale the values to 8 bits
 * (to write standard byte-per-sample PPM/PGM files), or output
 * nonstandard word-per-sample PPM/PGM files.  Downscaling is done
 * if PPM_NORAWWORD is defined (this can be done in the Makefile
 * or in jconfig.h).
 * (When the core library supports data precision reduction, a cleaner
 * implementation will be to ask for that instead.)
 */
pub const BYTESPERSAMPLE: c_int = 1i32;
pub const PPM_MAXVAL: c_int = 255i32;
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 *
 * put_pixel_rows handles the "normal" 8-bit case where the decompressor
 * output buffer is physically the same as the fwrite buffer.
 */
unsafe extern "C" fn put_pixel_rows(
    mut _cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * This code is used when we have to copy the data and apply a pixel
 * format translation.  Typically this only happens in 12-bit mode.
 */
unsafe extern "C" fn copy_pixel_rows(
    mut _cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut c_char = 0 as *mut c_char;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    ptr = *(*dest).pub_0.buffer.offset(0isize);
    bufferptr = (*dest).iobuffer;
    memcpy(
        bufferptr as *mut c_void,
        ptr as *const c_void,
        (*dest).samples_per_row as size_t,
    );
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Convert extended RGB to RGB.
 */
unsafe extern "C" fn put_rgb(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut c_char = 0 as *mut c_char;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut rindex: c_int = rgb_red[(*cinfo).out_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).out_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).out_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).out_color_space as usize];
    ptr = *(*dest).pub_0.buffer.offset(0isize);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        let fresh0 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh0 = *ptr.offset(rindex as isize) as c_char;
        let fresh1 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh1 = *ptr.offset(gindex as isize) as c_char;
        let fresh2 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh2 = *ptr.offset(bindex as isize) as c_char;
        ptr = ptr.offset(ps as isize);
        col = col.wrapping_sub(1)
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Convert CMYK to RGB.
 */
unsafe extern "C" fn put_cmyk(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut c_char = 0 as *mut c_char;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0isize);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        let mut r: JSAMPLE = 0;
        let mut g: JSAMPLE = 0;
        let mut b: JSAMPLE = 0;
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        let mut c: JSAMPLE = *fresh3;
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        let mut m: JSAMPLE = *fresh4;
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        let mut y: JSAMPLE = *fresh5;
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        let mut k: JSAMPLE = *fresh6;
        cmyk_to_rgb(c, m, y, k, &mut r, &mut g, &mut b);
        let fresh7 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh7 = r as c_char;
        let fresh8 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh8 = g as c_char;
        let fresh9 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh9 = b as c_char;
        col = col.wrapping_sub(1)
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Write some pixel data when color quantization is in effect.
 * We have to demap the color index values to straight data.
 */
unsafe extern "C" fn put_demapped_rgb(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut c_char = 0 as *mut c_char;
    let mut pixval: c_int = 0;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut color_map0: JSAMPROW = *(*cinfo).colormap.offset(0isize);
    let mut color_map1: JSAMPROW = *(*cinfo).colormap.offset(1isize);
    let mut color_map2: JSAMPROW = *(*cinfo).colormap.offset(2isize);
    let mut col: JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0isize);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        let fresh10 = ptr;
        ptr = ptr.offset(1);
        pixval = *fresh10 as c_int;
        let fresh11 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh11 = *color_map0.offset(pixval as isize) as c_int as c_char;
        let fresh12 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh12 = *color_map1.offset(pixval as isize) as c_int as c_char;
        let fresh13 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh13 = *color_map2.offset(pixval as isize) as c_int as c_char;
        col = col.wrapping_sub(1)
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
unsafe extern "C" fn put_demapped_gray(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut c_char = 0 as *mut c_char;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut color_map: JSAMPROW = *(*cinfo).colormap.offset(0isize);
    let mut col: JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0isize);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        let fresh15 = bufferptr;
        bufferptr = bufferptr.offset(1);
        let fresh14 = ptr;
        ptr = ptr.offset(1);
        *fresh15 = *color_map.offset(*fresh14 as c_int as isize) as c_int as c_char;
        col = col.wrapping_sub(1)
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1i32 as size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Startup: write the file header.
 */
unsafe extern "C" fn start_output_ppm(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    match (*cinfo).out_color_space as c_uint {
        1 => {
            fprintf(
                (*dest).pub_0.output_file,
                b"P5\n%ld %ld\n%d\n\x00" as *const u8 as *const c_char,
                (*cinfo).output_width as c_long,
                (*cinfo).output_height as c_long,
                PPM_MAXVAL,
            );
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 4 => {
            fprintf(
                (*dest).pub_0.output_file,
                b"P6\n%ld %ld\n%d\n\x00" as *const u8 as *const c_char,
                (*cinfo).output_width as c_long,
                (*cinfo).output_height as c_long,
                PPM_MAXVAL,
            );
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_PPM_COLORSPACE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}
/*
 * Finish up at the end of the file.
 */
unsafe extern "C" fn finish_output_ppm(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    fflush((*dinfo).output_file);
    if 0 != ferror((*dinfo).output_file) {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/*
 * Re-calculate buffer dimensions based on output dimensions.
 */
unsafe extern "C" fn calc_buffer_dimensions_ppm(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    if (*cinfo).out_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
        (*dest).samples_per_row = (*cinfo)
            .output_width
            .wrapping_mul((*cinfo).out_color_components as c_uint)
    } else {
        (*dest).samples_per_row = (*cinfo).output_width.wrapping_mul(3i32 as c_uint)
    }
    (*dest).buffer_width = ((*dest).samples_per_row as c_ulong).wrapping_mul(
        (BYTESPERSAMPLE as c_ulong).wrapping_mul(::std::mem::size_of::<c_char>() as c_ulong),
    );
}
/*
 * The module selection routine for PPM format output.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_write_ppm(mut cinfo: j_decompress_ptr) -> djpeg_dest_ptr {
    let mut dest: ppm_dest_ptr = 0 as *mut ppm_dest_struct;
    dest = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<ppm_dest_struct>() as c_ulong,
    ) as ppm_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_ppm as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_ppm as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_ppm
            as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    jpeg_calc_output_dimensions(cinfo);
    (*dest)
        .pub_0
        .calc_buffer_dimensions
        .expect("non-null function pointer")(cinfo, dest as djpeg_dest_ptr);
    (*dest).iobuffer = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (*dest).buffer_width,
    ) as *mut c_char;
    if 0 != (*cinfo).quantize_colors
        || BITS_IN_JSAMPLE != 8i32
        || ::std::mem::size_of::<JSAMPLE>() as c_ulong != ::std::mem::size_of::<c_char>() as c_ulong
        || (*cinfo).out_color_space as c_uint != JCS_EXT_RGB as c_int as c_uint
            && (*cinfo).out_color_space as c_uint != JCS_RGB as c_int as c_uint
    {
        (*dest).pub_0.buffer = (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*cinfo)
                .output_width
                .wrapping_mul((*cinfo).output_components as c_uint),
            1i32 as JDIMENSION,
        );
        (*dest).pub_0.buffer_height = 1i32 as JDIMENSION;
        if (*cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint
            || (*cinfo).out_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                && (*cinfo).out_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
        {
            (*dest).pub_0.put_pixel_rows = Some(
                put_rgb
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else if (*cinfo).out_color_space as c_uint == JCS_CMYK as c_int as c_uint {
            (*dest).pub_0.put_pixel_rows = Some(
                put_cmyk
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else if 0 == (*cinfo).quantize_colors {
            (*dest).pub_0.put_pixel_rows = Some(
                copy_pixel_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else if (*cinfo).out_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
            (*dest).pub_0.put_pixel_rows = Some(
                put_demapped_gray
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else {
            (*dest).pub_0.put_pixel_rows = Some(
                put_demapped_rgb
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
    } else {
        (*dest).pixrow = (*dest).iobuffer as JSAMPROW;
        (*dest).pub_0.buffer = &mut (*dest).pixrow;
        (*dest).pub_0.buffer_height = 1i32 as JDIMENSION;
        (*dest).pub_0.put_pixel_rows = Some(
            put_pixel_rows
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: djpeg_dest_ptr,
                    _: JDIMENSION,
                ) -> (),
        )
    }
    return dest as djpeg_dest_ptr;
}
