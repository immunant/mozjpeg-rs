










































































































































































































































































use crate::stdlib::{ferror, fflush, fwrite, putc};use libc::{c_uint, c_ulong, c_char, c_long, c_int, c_void, self};pub use crate::cderror_h::{C2RustUnnamed_4, JERR_BAD_CMAP_FILE,
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
                        __off64_t, __off_t, FILE, _IO_FILE};pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE,
                            UINT16, UINT8};pub use crate::stddef_h::{size_t, NULL};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_calc_output_dimensions,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_reader,
                           jpeg_marker_struct, jpeg_memory_mgr,
                           jpeg_progress_mgr, jpeg_saved_marker_ptr,
                           jpeg_source_mgr, jpeg_upsampler,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
                           JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB,
                           JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
                           JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
                           JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
                           JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
                           JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
                           JQUANT_TBL, JSAMPARRAY, JSAMPROW, J_COLOR_SPACE,
                           J_DCT_METHOD, J_DITHER_MODE};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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
                        JERR_EOI_EXPECTED, JERR_FILE_READ, JERR_FILE_WRITE,
                        JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
                        JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG,
                        JERR_INPUT_EMPTY, JERR_INPUT_EOF,
                        JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA,
                        JERR_MODE_CHANGE, JERR_NOTIMPL, JERR_NOT_COMPILED,
                        JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                        JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                        JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                        JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                        JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                        JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                        JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
                        JERR_TFILE_SEEK, JERR_TFILE_WRITE,
                        JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
                        JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG,
                        JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
                        JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE,
                        JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
                        JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                        JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN, JTRC_EOI,
                        JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE,
                        JTRC_JFIF_EXTENSION, JTRC_JFIF_THUMBNAIL,
                        JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER,
                        JTRC_QUANTVALS, JTRC_QUANT_3_NCOLORS,
                        JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED,
                        JTRC_RECOVERY_ACTION, JTRC_RST, JTRC_SMOOTH_NOTIMPL,
                        JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                        JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE,
                        JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                        JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                        JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                        JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                        JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR,
                        JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
                        JWRN_TOO_MUCH_DATA};pub use super::cdjpeg::{djpeg_dest_ptr, djpeg_dest_struct};

pub type gif_dest_ptr = *mut gif_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gif_dest_struct {
    pub pub_0: super::cdjpeg::djpeg_dest_struct,
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

unsafe extern "C" fn flush_packet(mut dinfo: gif_dest_ptr)
/* flush any accumulated data */
{
    if (*dinfo).bytesinpkt > 0i32 {
        /* never write zero-length packet */
        let fresh0 = (*dinfo).bytesinpkt;
        (*dinfo).bytesinpkt = (*dinfo).bytesinpkt + 1;
        (*dinfo).packetbuf[0] = fresh0 as c_char;
        if fwrite(
            (*dinfo).packetbuf.as_mut_ptr() as *const c_void,
            1u64,
            (*dinfo).bytesinpkt as size_t,
            (*dinfo).pub_0.output_file,
        ) != (*dinfo).bytesinpkt as size_t
        {
            (*(*(*dinfo).cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
            Some(
                (*(*(*dinfo).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*dinfo).cinfo as j_common_ptr
            );
        }
        (*dinfo).bytesinpkt = 0i32
    };
}
/* Add a character to current packet; flush to disk if necessary */
/* Routine to convert variable-width codes into a byte stream */

unsafe extern "C" fn output(mut dinfo: gif_dest_ptr, mut code: c_int)
/* Emit a code of n_bits bits */
/* Uses cur_accum and cur_bits to reblock into 8-bit bytes */
{
    (*dinfo).cur_accum |= (code as c_long) << (*dinfo).cur_bits;
    (*dinfo).cur_bits += (*dinfo).n_bits;
    while (*dinfo).cur_bits >= 8i32 {
        (*dinfo).bytesinpkt += 1;
        (*dinfo).packetbuf[(*dinfo).bytesinpkt as usize] =
            ((*dinfo).cur_accum & 0xffi64) as c_char;
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

unsafe extern "C" fn compress_init(mut dinfo: gif_dest_ptr, mut i_bits: c_int)
/* Initialize pseudo-compressor */
{
    /* init all the state variables */
    (*dinfo).n_bits = i_bits;
    (*dinfo).maxcode = (1i32 << (*dinfo).n_bits) - 1i32;
    (*dinfo).ClearCode = 1i32 << i_bits - 1i32;
    (*dinfo).EOFCode = (*dinfo).ClearCode + 1i32;
    (*dinfo).code_counter = (*dinfo).ClearCode + 2i32;
    /* init output buffering vars */
    (*dinfo).bytesinpkt = 0i32;
    (*dinfo).cur_accum = 0i64;
    (*dinfo).cur_bits = 0i32;
    /* GIF specifies an initial Clear code */
    output(dinfo, (*dinfo).ClearCode);
}

unsafe extern "C" fn compress_pixel(mut dinfo: gif_dest_ptr, mut c: c_int)
/* Accept and "compress" one pixel value.
 * The given value must be less than n_bits wide.
 */
{
    /* Output the given pixel value as a symbol. */
    output(dinfo, c);
    /* Issue Clear codes often enough to keep the reader from ratcheting up
     * its symbol size.
     */
    if (*dinfo).code_counter < (*dinfo).maxcode {
        (*dinfo).code_counter += 1
    } else {
        output(dinfo, (*dinfo).ClearCode);
        (*dinfo).code_counter = (*dinfo).ClearCode + 2i32
        /* reset the counter */
    };
}

unsafe extern "C" fn compress_term(mut dinfo: gif_dest_ptr)
/* Clean up at end */
{
    /* Send an EOF code */
    output(dinfo, (*dinfo).EOFCode);
    /* Flush the bit-packing buffer */
    if (*dinfo).cur_bits > 0i32 {
        (*dinfo).bytesinpkt += 1;
        (*dinfo).packetbuf[(*dinfo).bytesinpkt as usize] =
            ((*dinfo).cur_accum & 0xffi64) as c_char;
        if (*dinfo).bytesinpkt >= 255i32 {
            flush_packet(dinfo);
        }
    }
    /* Flush the packet buffer */
    flush_packet(dinfo);
}
/* GIF header construction */

unsafe extern "C" fn put_word(mut dinfo: gif_dest_ptr, mut w: c_uint)
/* Emit a 16-bit word, LSB first */
{
    putc(
        (w & 0xffu32) as c_int,
        (*dinfo).pub_0.output_file,
    );
    putc(
        (w >> 8i32 & 0xffu32) as c_int,
        (*dinfo).pub_0.output_file,
    );
}

unsafe extern "C" fn put_3bytes(mut dinfo: gif_dest_ptr, mut val: c_int)
/* Emit 3 copies of same byte value --- handy subr for colormap construction */
{
    putc(val, (*dinfo).pub_0.output_file);
    putc(val, (*dinfo).pub_0.output_file);
    putc(val, (*dinfo).pub_0.output_file);
}

unsafe extern "C" fn emit_header(
    mut dinfo: gif_dest_ptr,
    mut num_colors: c_int,
    mut colormap: JSAMPARRAY,
)
/* Output the GIF file header, including color map */
/* If colormap==NULL, synthesize a grayscale colormap */
{
    
    
    
       let mut InitCodeSize:  c_int =  0;  
    let mut cshift: c_int = (*(*dinfo).cinfo).data_precision - 8i32;
    
    if num_colors > 256i32 {
        (*(*(*dinfo).cinfo).err).msg_code = JERR_TOO_MANY_COLORS as c_int;
        (*(*(*dinfo).cinfo).err).msg_parm.i[0] = num_colors;
        Some(
            (*(*(*dinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*dinfo).cinfo as j_common_ptr
        );
    }
    /* Compute bits/pixel and related values */
     let mut BitsPerPixel:   c_int =  1i32;
    while num_colors > 1i32 << BitsPerPixel {
        BitsPerPixel += 1
    }
     let mut ColorMapSize:   c_int =  1i32 << BitsPerPixel;
    if BitsPerPixel <= 1i32 {
        InitCodeSize = 2i32
    } else {
        InitCodeSize = BitsPerPixel
    }
    /*
     * Write the GIF header.
     * Note that we generate a plain GIF87 header for maximum compatibility.
     */
    putc('G' as i32, (*dinfo).pub_0.output_file);
    putc('I' as i32, (*dinfo).pub_0.output_file);
    putc('F' as i32, (*dinfo).pub_0.output_file);
    putc('8' as i32, (*dinfo).pub_0.output_file);
    putc('7' as i32, (*dinfo).pub_0.output_file);
    putc('a' as i32, (*dinfo).pub_0.output_file);
    /* Write the Logical Screen Descriptor */
    put_word(dinfo, (*(*dinfo).cinfo).output_width); /* Yes, there is a global color table */
    put_word(dinfo, (*(*dinfo).cinfo).output_height); /* color resolution */
     let mut FlagByte:   c_int =  0x80i32; /* size of global color table */
    FlagByte |= BitsPerPixel - 1i32 << 4i32; /* Background color index */
    FlagByte |= BitsPerPixel - 1i32; /* Reserved (aspect ratio in GIF89) */
    putc(FlagByte, (*dinfo).pub_0.output_file);
    putc(0i32, (*dinfo).pub_0.output_file);
    putc(0i32, (*dinfo).pub_0.output_file);
    /* Write the Global Color Map */
    /* If the color map is more than 8 bits precision, */
    /* we reduce it to 8 bits by shifting */
     let mut i:   c_int =  0i32;
    while i < ColorMapSize {
        if i < num_colors {
            if !colormap.is_null() {
                if  (*(*dinfo).cinfo).out_color_space
                    ==  JCS_RGB
                {
                    /* Normal case: RGB color map */
                    putc(
                        *(*colormap.offset(0)).offset(i as isize) as c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    putc(
                        *(*colormap.offset(1)).offset(i as isize) as c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    putc(
                        *(*colormap.offset(2)).offset(i as isize) as c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                } else {
                    /* Grayscale "color map": possible if quantizing grayscale image */
                    put_3bytes(
                        dinfo,
                        *(*colormap.offset(0)).offset(i as isize) as c_int >> cshift,
                    );
                }
            } else {
                /* Create a grayscale map of num_colors values, range 0..255 */
                put_3bytes(
                    dinfo,
                    (i * 255i32 + (num_colors - 1i32) / 2i32) / (num_colors - 1i32),
                );
            }
        } else {
            /* fill out the map to a power of 2 */
            put_3bytes(dinfo, 0i32);
        }
        i += 1
    }
    /* Write image separator and Image Descriptor */
    putc(',' as i32, (*dinfo).pub_0.output_file); /* separator */
    put_word(dinfo, 0u32); /* left/top offset */
    put_word(dinfo, 0u32); /* image size */
    put_word(dinfo, (*(*dinfo).cinfo).output_width);
    put_word(dinfo, (*(*dinfo).cinfo).output_height);
    /* flag byte: not interlaced, no local color map */
    putc(0i32, (*dinfo).pub_0.output_file);
    /* Write Initial Code Size byte */
    putc(InitCodeSize, (*dinfo).pub_0.output_file);
    /* Initialize for "compression" of image data */
    compress_init(dinfo, InitCodeSize + 1i32);
}
/*
 * Startup: write the file header.
 */

unsafe extern "C" fn start_output_gif(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    if (*cinfo).quantize_colors != 0 {
        emit_header(dest, (*cinfo).actual_number_of_colors, (*cinfo).colormap);
    } else {
        emit_header(
            dest,
            256i32,
            
            NULL as JSAMPARRAY,
        );
    };
}
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 */

unsafe extern "C" fn put_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
      let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    
    
    
     let mut ptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0); let mut col:   JDIMENSION =  (*cinfo).output_width;
    while col > 0u32 {
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        compress_pixel(dest, *fresh1 as c_int);
        col -=  1
    }
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_output_gif(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    /* Flush "compression" mechanism */
    compress_term(dest);
    /* Write a zero-length data block to end the series */
    putc(0i32, (*dest).pub_0.output_file);
    /* Write the GIF terminator mark */
    putc(';' as i32, (*dest).pub_0.output_file);
    /* Make sure we wrote the output file OK */
    fflush((*dest).pub_0.output_file);
    if ferror((*dest).pub_0.output_file) != 0 {
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

unsafe extern "C" fn calc_buffer_dimensions_gif(
    mut _cinfo: j_decompress_ptr,
    mut _dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
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
 * The module selection routine for GIF format output.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_write_gif(
    mut cinfo: j_decompress_ptr,
) -> super::cdjpeg::djpeg_dest_ptr {
     
    /* Create module interface object, fill in method pointers */
     let mut dest:   gif_dest_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<gif_dest_struct>() as c_ulong,
    ) as gif_dest_ptr; /* make back link for subroutines */
    (*dest).cinfo = cinfo;
    (*dest).pub_0.start_output = Some(
        start_output_gif
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.put_pixel_rows = Some(
        put_pixel_rows
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
                _: JDIMENSION,
            ) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_gif
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_gif
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    if  (*cinfo).out_color_space
        !=  JCS_GRAYSCALE
        &&  (*cinfo).out_color_space
            !=  JCS_RGB
    {
        (*(*cinfo).err).msg_code = JERR_GIF_COLORSPACE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Force quantization if color or if > 8 bits input */
    if  (*cinfo).out_color_space
        !=  JCS_GRAYSCALE
        || (*cinfo).data_precision > 8i32
    {
        /* Force quantization to at most 256 colors */
        (*cinfo).quantize_colors = TRUE;
        if (*cinfo).desired_number_of_colors > 256i32 {
            (*cinfo).desired_number_of_colors = 256i32
        }
    }
    /* Calculate output image dimensions so we can allocate space */
    jpeg_calc_output_dimensions(cinfo);
    if (*cinfo).output_components != 1i32 {
        /* safety check: just one component? */
        (*(*cinfo).err).msg_code = JERR_GIF_BUG as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Create decompressor output buffer. */
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (*cinfo).output_width,
        1u32,
    );
    (*dest).pub_0.buffer_height = 1u32;
    return dest as super::cdjpeg::djpeg_dest_ptr;
}
/* GIF_SUPPORTED */
