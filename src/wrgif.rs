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
pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8};
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
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, FILE, _IO_FILE,
};
use crate::stdlib::{ferror, fflush, fwrite, putc};
use libc::{self, c_char, c_int, c_long, c_uint, c_ulong, c_void};
pub type gif_dest_ptr = *mut gif_dest_struct;
/*
 * wrgif.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to write output images in GIF format.
 *
 **************************************************************************
 * NOTE: to avoid entanglements with Unisys' patent on LZW compression,   *
 * this code has been modified to output "uncompressed GIF" files.        *
 * There is no trace of the LZW algorithm in this file.                   *
 **************************************************************************
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume output to
 * an ordinary stdio stream.
 */
/*
 * This code is loosely based on ppmtogif from the PBMPLUS distribution
 * of Feb. 1991.  That file contains the following copyright notice:
 *    Based on GIFENCODE by David Rowley <mgardi@watdscu.waterloo.edu>.
 *    Lempel-Ziv compression based on "compress" by Spencer W. Thomas et al.
 *    Copyright (C) 1989 by Jef Poskanzer.
 *    Permission to use, copy, modify, and distribute this software and its
 *    documentation for any purpose and without fee is hereby granted, provided
 *    that the above copyright notice appear in all copies and that both that
 *    copyright notice and this permission notice appear in supporting
 *    documentation.  This software is provided "as is" without express or
 *    implied warranty.
 *
 * We are also required to state that
 *    "The Graphics Interchange Format(c) is the Copyright property of
 *    CompuServe Incorporated. GIF(sm) is a Service Mark property of
 *    CompuServe Incorporated."
 */
/* Private version of data destination object */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gif_dest_struct {
    pub pub_0: djpeg_dest_struct,
    pub cinfo: j_decompress_ptr,
    pub n_bits: c_int,
    pub maxcode: c_int,
    pub cur_accum: c_long,
    pub cur_bits: c_int,
    pub ClearCode: c_int,
    pub EOFCode: c_int,
    pub code_counter: c_int,
    pub bytesinpkt: c_int,
    pub packetbuf: [c_char; 256],
}
/* Largest value that will fit in N bits */
/*
 * Routines to package finished data bytes into GIF data blocks.
 * A data block consists of a count byte (1..255) and that many data bytes.
 */
unsafe extern "C" fn flush_packet(mut dinfo: gif_dest_ptr) {
    if (*dinfo).bytesinpkt > 0i32 {
        let fresh0 = (*dinfo).bytesinpkt;
        (*dinfo).bytesinpkt = (*dinfo).bytesinpkt + 1;
        (*dinfo).packetbuf[0usize] = fresh0 as c_char;
        if fwrite(
            (*dinfo).packetbuf.as_mut_ptr() as *const c_void,
            1i32 as size_t,
            (*dinfo).bytesinpkt as size_t,
            (*dinfo).pub_0.output_file,
        ) != (*dinfo).bytesinpkt as size_t
        {
            (*(*(*dinfo).cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
            (*(*(*dinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer")((*dinfo).cinfo as j_common_ptr);
        }
        (*dinfo).bytesinpkt = 0i32
    };
}
/* Add a character to current packet; flush to disk if necessary */
/* Routine to convert variable-width codes into a byte stream */
unsafe extern "C" fn output(mut dinfo: gif_dest_ptr, mut code: c_int) {
    (*dinfo).cur_accum |= (code as c_long) << (*dinfo).cur_bits;
    (*dinfo).cur_bits += (*dinfo).n_bits;
    while (*dinfo).cur_bits >= 8i32 {
        (*dinfo).bytesinpkt += 1;
        (*dinfo).packetbuf[(*dinfo).bytesinpkt as usize] =
            ((*dinfo).cur_accum & 0xffi32 as c_long) as c_char;
        if (*dinfo).bytesinpkt >= 255i32 {
            flush_packet(dinfo);
        }
        (*dinfo).cur_accum >>= 8i32;
        (*dinfo).cur_bits -= 8i32
    }
}
/* The pseudo-compression algorithm.
 *
 * In this module we simply output each pixel value as a separate symbol;
 * thus, no compression occurs.  In fact, there is expansion of one bit per
 * pixel, because we use a symbol width one bit wider than the pixel width.
 *
 * GIF ordinarily uses variable-width symbols, and the decoder will expect
 * to ratchet up the symbol width after a fixed number of symbols.
 * To simplify the logic and keep the expansion penalty down, we emit a
 * GIF Clear code to reset the decoder just before the width would ratchet up.
 * Thus, all the symbols in the output file will have the same bit width.
 * Note that emitting the Clear codes at the right times is a mere matter of
 * counting output symbols and is in no way dependent on the LZW patent.
 *
 * With a small basic pixel width (low color count), Clear codes will be
 * needed very frequently, causing the file to expand even more.  So this
 * simplistic approach wouldn't work too well on bilevel images, for example.
 * But for output of JPEG conversions the pixel width will usually be 8 bits
 * (129 to 256 colors), so the overhead added by Clear symbols is only about
 * one symbol in every 256.
 */
unsafe extern "C" fn compress_init(mut dinfo: gif_dest_ptr, mut i_bits: c_int) {
    (*dinfo).n_bits = i_bits;
    (*dinfo).maxcode = (1i32 << (*dinfo).n_bits) - 1i32;
    (*dinfo).ClearCode = 1i32 << i_bits - 1i32;
    (*dinfo).EOFCode = (*dinfo).ClearCode + 1i32;
    (*dinfo).code_counter = (*dinfo).ClearCode + 2i32;
    (*dinfo).bytesinpkt = 0i32;
    (*dinfo).cur_accum = 0i32 as c_long;
    (*dinfo).cur_bits = 0i32;
    output(dinfo, (*dinfo).ClearCode);
}
unsafe extern "C" fn compress_pixel(mut dinfo: gif_dest_ptr, mut c: c_int) {
    output(dinfo, c);
    if (*dinfo).code_counter < (*dinfo).maxcode {
        (*dinfo).code_counter += 1
    } else {
        output(dinfo, (*dinfo).ClearCode);
        (*dinfo).code_counter = (*dinfo).ClearCode + 2i32
    };
}
unsafe extern "C" fn compress_term(mut dinfo: gif_dest_ptr) {
    output(dinfo, (*dinfo).EOFCode);
    if (*dinfo).cur_bits > 0i32 {
        (*dinfo).bytesinpkt += 1;
        (*dinfo).packetbuf[(*dinfo).bytesinpkt as usize] =
            ((*dinfo).cur_accum & 0xffi32 as c_long) as c_char;
        if (*dinfo).bytesinpkt >= 255i32 {
            flush_packet(dinfo);
        }
    }
    flush_packet(dinfo);
}
/* GIF header construction */
unsafe extern "C" fn put_word(mut dinfo: gif_dest_ptr, mut w: c_uint) {
    putc((w & 0xffi32 as c_uint) as c_int, (*dinfo).pub_0.output_file);
    putc(
        (w >> 8i32 & 0xffi32 as c_uint) as c_int,
        (*dinfo).pub_0.output_file,
    );
}
unsafe extern "C" fn put_3bytes(mut dinfo: gif_dest_ptr, mut val: c_int) {
    putc(val, (*dinfo).pub_0.output_file);
    putc(val, (*dinfo).pub_0.output_file);
    putc(val, (*dinfo).pub_0.output_file);
}
unsafe extern "C" fn emit_header(
    mut dinfo: gif_dest_ptr,
    mut num_colors: c_int,
    mut colormap: JSAMPARRAY,
) {
    let mut BitsPerPixel: c_int = 0;
    let mut ColorMapSize: c_int = 0;
    let mut InitCodeSize: c_int = 0;
    let mut FlagByte: c_int = 0;
    let mut cshift: c_int = (*(*dinfo).cinfo).data_precision - 8i32;
    let mut i: c_int = 0;
    if num_colors > 256i32 {
        (*(*(*dinfo).cinfo).err).msg_code = JERR_TOO_MANY_COLORS as c_int;
        (*(*(*dinfo).cinfo).err).msg_parm.i[0usize] = num_colors;
        (*(*(*dinfo).cinfo).err)
            .error_exit
            .expect("non-null function pointer")((*dinfo).cinfo as j_common_ptr);
    }
    BitsPerPixel = 1i32;
    while num_colors > 1i32 << BitsPerPixel {
        BitsPerPixel += 1
    }
    ColorMapSize = 1i32 << BitsPerPixel;
    if BitsPerPixel <= 1i32 {
        InitCodeSize = 2i32
    } else {
        InitCodeSize = BitsPerPixel
    }
    putc('G' as i32, (*dinfo).pub_0.output_file);
    putc('I' as i32, (*dinfo).pub_0.output_file);
    putc('F' as i32, (*dinfo).pub_0.output_file);
    putc('8' as i32, (*dinfo).pub_0.output_file);
    putc('7' as i32, (*dinfo).pub_0.output_file);
    putc('a' as i32, (*dinfo).pub_0.output_file);
    put_word(dinfo, (*(*dinfo).cinfo).output_width);
    put_word(dinfo, (*(*dinfo).cinfo).output_height);
    FlagByte = 0x80i32;
    FlagByte |= BitsPerPixel - 1i32 << 4i32;
    FlagByte |= BitsPerPixel - 1i32;
    putc(FlagByte, (*dinfo).pub_0.output_file);
    putc(0i32, (*dinfo).pub_0.output_file);
    putc(0i32, (*dinfo).pub_0.output_file);
    i = 0i32;
    while i < ColorMapSize {
        if i < num_colors {
            if !colormap.is_null() {
                if (*(*dinfo).cinfo).out_color_space as c_uint == JCS_RGB as c_int as c_uint {
                    putc(
                        *(*colormap.offset(0isize)).offset(i as isize) as c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    putc(
                        *(*colormap.offset(1isize)).offset(i as isize) as c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    putc(
                        *(*colormap.offset(2isize)).offset(i as isize) as c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                } else {
                    put_3bytes(
                        dinfo,
                        *(*colormap.offset(0isize)).offset(i as isize) as c_int >> cshift,
                    );
                }
            } else {
                put_3bytes(
                    dinfo,
                    (i * 255i32 + (num_colors - 1i32) / 2i32) / (num_colors - 1i32),
                );
            }
        } else {
            put_3bytes(dinfo, 0i32);
        }
        i += 1
    }
    putc(',' as i32, (*dinfo).pub_0.output_file);
    put_word(dinfo, 0i32 as c_uint);
    put_word(dinfo, 0i32 as c_uint);
    put_word(dinfo, (*(*dinfo).cinfo).output_width);
    put_word(dinfo, (*(*dinfo).cinfo).output_height);
    putc(0i32, (*dinfo).pub_0.output_file);
    putc(InitCodeSize, (*dinfo).pub_0.output_file);
    compress_init(dinfo, InitCodeSize + 1i32);
}
/*
 * Startup: write the file header.
 */
unsafe extern "C" fn start_output_gif(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    if 0 != (*cinfo).quantize_colors {
        emit_header(dest, (*cinfo).actual_number_of_colors, (*cinfo).colormap);
    } else {
        emit_header(dest, 256i32, NULL as *mut c_void as JSAMPARRAY);
    };
}
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 */
unsafe extern "C" fn put_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0isize);
    col = (*cinfo).output_width;
    while col > 0i32 as c_uint {
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        compress_pixel(dest, *fresh1 as c_int);
        col = col.wrapping_sub(1)
    }
}
/*
 * Finish up at the end of the file.
 */
unsafe extern "C" fn finish_output_gif(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    compress_term(dest);
    putc(0i32, (*dest).pub_0.output_file);
    putc(';' as i32, (*dest).pub_0.output_file);
    fflush((*dest).pub_0.output_file);
    if 0 != ferror((*dest).pub_0.output_file) {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/*
 * Re-calculate buffer dimensions based on output dimensions.
 */
unsafe extern "C" fn calc_buffer_dimensions_gif(
    mut _cinfo: j_decompress_ptr,
    mut _dinfo: djpeg_dest_ptr,
) {
}
/*
 * The module selection routine for GIF format output.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_write_gif(mut cinfo: j_decompress_ptr) -> djpeg_dest_ptr {
    let mut dest: gif_dest_ptr = 0 as *mut gif_dest_struct;
    dest = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<gif_dest_struct>() as c_ulong,
    ) as gif_dest_ptr;
    (*dest).cinfo = cinfo;
    (*dest).pub_0.start_output = Some(
        start_output_gif as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.put_pixel_rows = Some(
        put_pixel_rows
            as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr, _: JDIMENSION) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_gif as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_gif
            as unsafe extern "C" fn(_: j_decompress_ptr, _: djpeg_dest_ptr) -> (),
    );
    if (*cinfo).out_color_space as c_uint != JCS_GRAYSCALE as c_int as c_uint
        && (*cinfo).out_color_space as c_uint != JCS_RGB as c_int as c_uint
    {
        (*(*cinfo).err).msg_code = JERR_GIF_COLORSPACE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).out_color_space as c_uint != JCS_GRAYSCALE as c_int as c_uint
        || (*cinfo).data_precision > 8i32
    {
        (*cinfo).quantize_colors = TRUE;
        if (*cinfo).desired_number_of_colors > 256i32 {
            (*cinfo).desired_number_of_colors = 256i32
        }
    }
    jpeg_calc_output_dimensions(cinfo);
    if (*cinfo).output_components != 1i32 {
        (*(*cinfo).err).msg_code = JERR_GIF_BUG as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*dest).pub_0.buffer = (*(*cinfo).mem)
        .alloc_sarray
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (*cinfo).output_width,
        1i32 as JDIMENSION,
    );
    (*dest).pub_0.buffer_height = 1i32 as JDIMENSION;
    return dest as djpeg_dest_ptr;
}
