use libc;

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
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
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
use crate::stdlib::putc;

pub type gif_dest_ptr = *mut gif_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gif_dest_struct {
    pub pub_0: crate::src::cdjpeg::djpeg_dest_struct,
    pub cinfo: crate::jpeglib_h::j_decompress_ptr,
    pub n_bits: libc::c_int,
    pub maxcode: libc::c_int,
    pub cur_accum: libc::c_long,
    pub cur_bits: libc::c_int,
    pub ClearCode: libc::c_int,
    pub EOFCode: libc::c_int,
    pub code_counter: libc::c_int,
    pub bytesinpkt: libc::c_int,
    pub packetbuf: [libc::c_char; 256],
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
        (*dinfo).packetbuf[0] = fresh0 as libc::c_char;
        if crate::stdlib::fwrite(
            (*dinfo).packetbuf.as_mut_ptr() as *const libc::c_void,
            1i32 as crate::stddef_h::size_t,
            (*dinfo).bytesinpkt as crate::stddef_h::size_t,
            (*dinfo).pub_0.output_file,
        ) != (*dinfo).bytesinpkt as crate::stddef_h::size_t
        {
            (*(*(*dinfo).cinfo).err).msg_code = crate::src::jerror::JERR_FILE_WRITE as libc::c_int;
            Some(
                (*(*(*dinfo).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*dinfo).cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        (*dinfo).bytesinpkt = 0i32
    };
}
/* Add a character to current packet; flush to disk if necessary */
/* Routine to convert variable-width codes into a byte stream */

unsafe extern "C" fn output(mut dinfo: gif_dest_ptr, mut code: libc::c_int)
/* Emit a code of n_bits bits */
/* Uses cur_accum and cur_bits to reblock into 8-bit bytes */
{
    (*dinfo).cur_accum |= (code as libc::c_long) << (*dinfo).cur_bits;
    (*dinfo).cur_bits += (*dinfo).n_bits;
    while (*dinfo).cur_bits >= 8i32 {
        (*dinfo).bytesinpkt += 1;
        (*dinfo).packetbuf[(*dinfo).bytesinpkt as usize] =
            ((*dinfo).cur_accum & 0xffi32 as libc::c_long) as libc::c_char;
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

unsafe extern "C" fn compress_init(mut dinfo: gif_dest_ptr, mut i_bits: libc::c_int)
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
    (*dinfo).cur_accum = 0i32 as libc::c_long;
    (*dinfo).cur_bits = 0i32;
    /* GIF specifies an initial Clear code */
    output(dinfo, (*dinfo).ClearCode);
}

unsafe extern "C" fn compress_pixel(mut dinfo: gif_dest_ptr, mut c: libc::c_int)
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
            ((*dinfo).cur_accum & 0xffi32 as libc::c_long) as libc::c_char;
        if (*dinfo).bytesinpkt >= 255i32 {
            flush_packet(dinfo);
        }
    }
    /* Flush the packet buffer */
    flush_packet(dinfo);
}
/* GIF header construction */

unsafe extern "C" fn put_word(mut dinfo: gif_dest_ptr, mut w: libc::c_uint)
/* Emit a 16-bit word, LSB first */
{
    crate::stdlib::putc(
        (w & 0xffi32 as libc::c_uint) as libc::c_int,
        (*dinfo).pub_0.output_file,
    );
    crate::stdlib::putc(
        (w >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int,
        (*dinfo).pub_0.output_file,
    );
}

unsafe extern "C" fn put_3bytes(mut dinfo: gif_dest_ptr, mut val: libc::c_int)
/* Emit 3 copies of same byte value --- handy subr for colormap construction */
{
    crate::stdlib::putc(val, (*dinfo).pub_0.output_file);
    crate::stdlib::putc(val, (*dinfo).pub_0.output_file);
    crate::stdlib::putc(val, (*dinfo).pub_0.output_file);
}

unsafe extern "C" fn emit_header(
    mut dinfo: gif_dest_ptr,
    mut num_colors: libc::c_int,
    mut colormap: crate::jpeglib_h::JSAMPARRAY,
)
/* Output the GIF file header, including color map */
/* If colormap==NULL, synthesize a grayscale colormap */
{
    let mut BitsPerPixel: libc::c_int = 0;
    let mut ColorMapSize: libc::c_int = 0;
    let mut InitCodeSize: libc::c_int = 0;
    let mut FlagByte: libc::c_int = 0;
    let mut cshift: libc::c_int = (*(*dinfo).cinfo).data_precision - 8i32;
    let mut i: libc::c_int = 0;
    if num_colors > 256i32 {
        (*(*(*dinfo).cinfo).err).msg_code = crate::cderror_h::JERR_TOO_MANY_COLORS as libc::c_int;
        (*(*(*dinfo).cinfo).err).msg_parm.i[0] = num_colors;
        Some(
            (*(*(*dinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*dinfo).cinfo as crate::jpeglib_h::j_common_ptr
        );
    }
    /* Compute bits/pixel and related values */
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
    /*
     * Write the GIF header.
     * Note that we generate a plain GIF87 header for maximum compatibility.
     */
    crate::stdlib::putc('G' as i32, (*dinfo).pub_0.output_file);
    crate::stdlib::putc('I' as i32, (*dinfo).pub_0.output_file);
    crate::stdlib::putc('F' as i32, (*dinfo).pub_0.output_file);
    crate::stdlib::putc('8' as i32, (*dinfo).pub_0.output_file);
    crate::stdlib::putc('7' as i32, (*dinfo).pub_0.output_file);
    crate::stdlib::putc('a' as i32, (*dinfo).pub_0.output_file);
    /* Write the Logical Screen Descriptor */
    put_word(dinfo, (*(*dinfo).cinfo).output_width); /* Yes, there is a global color table */
    put_word(dinfo, (*(*dinfo).cinfo).output_height); /* color resolution */
    FlagByte = 0x80i32; /* size of global color table */
    FlagByte |= BitsPerPixel - 1i32 << 4i32; /* Background color index */
    FlagByte |= BitsPerPixel - 1i32; /* Reserved (aspect ratio in GIF89) */
    crate::stdlib::putc(FlagByte, (*dinfo).pub_0.output_file);
    crate::stdlib::putc(0i32, (*dinfo).pub_0.output_file);
    crate::stdlib::putc(0i32, (*dinfo).pub_0.output_file);
    /* Write the Global Color Map */
    /* If the color map is more than 8 bits precision, */
    /* we reduce it to 8 bits by shifting */
    i = 0i32;
    while i < ColorMapSize {
        if i < num_colors {
            if !colormap.is_null() {
                if (*(*dinfo).cinfo).out_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                {
                    /* Normal case: RGB color map */
                    crate::stdlib::putc(
                        *(*colormap.offset(0)).offset(i as isize) as libc::c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    crate::stdlib::putc(
                        *(*colormap.offset(1)).offset(i as isize) as libc::c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    crate::stdlib::putc(
                        *(*colormap.offset(2)).offset(i as isize) as libc::c_int >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                } else {
                    /* Grayscale "color map": possible if quantizing grayscale image */
                    put_3bytes(
                        dinfo,
                        *(*colormap.offset(0)).offset(i as isize) as libc::c_int >> cshift,
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
    crate::stdlib::putc(',' as i32, (*dinfo).pub_0.output_file); /* separator */
    put_word(dinfo, 0i32 as libc::c_uint); /* left/top offset */
    put_word(dinfo, 0i32 as libc::c_uint); /* image size */
    put_word(dinfo, (*(*dinfo).cinfo).output_width);
    put_word(dinfo, (*(*dinfo).cinfo).output_height);
    /* flag byte: not interlaced, no local color map */
    crate::stdlib::putc(0i32, (*dinfo).pub_0.output_file);
    /* Write Initial Code Size byte */
    crate::stdlib::putc(InitCodeSize, (*dinfo).pub_0.output_file);
    /* Initialize for "compression" of image data */
    compress_init(dinfo, InitCodeSize + 1i32);
}
/*
 * Startup: write the file header.
 */

unsafe extern "C" fn start_output_gif(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    if (*cinfo).quantize_colors != 0 {
        emit_header(dest, (*cinfo).actual_number_of_colors, (*cinfo).colormap);
    } else {
        emit_header(
            dest,
            256i32,
            crate::stddef_h::NULL as *mut libc::c_void as crate::jpeglib_h::JSAMPARRAY,
        );
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
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0);
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        compress_pixel(dest, *fresh1 as libc::c_int);
        col =  col - 1
    }
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_output_gif(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    /* Flush "compression" mechanism */
    compress_term(dest);
    /* Write a zero-length data block to end the series */
    crate::stdlib::putc(0i32, (*dest).pub_0.output_file);
    /* Write the GIF terminator mark */
    crate::stdlib::putc(';' as i32, (*dest).pub_0.output_file);
    /* Make sure we wrote the output file OK */
    crate::stdlib::fflush((*dest).pub_0.output_file);
    if crate::stdlib::ferror((*dest).pub_0.output_file) != 0 {
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

unsafe extern "C" fn calc_buffer_dimensions_gif(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::src::cdjpeg::djpeg_dest_ptr {
    let mut dest: gif_dest_ptr = ::std::ptr::null_mut::< gif_dest_struct>();
    /* Create module interface object, fill in method pointers */
    dest = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<gif_dest_struct>() as libc::c_ulong,
    ) as gif_dest_ptr; /* make back link for subroutines */
    (*dest).cinfo = cinfo;
    (*dest).pub_0.start_output = Some(
        start_output_gif
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.put_pixel_rows = Some(
        put_pixel_rows
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_gif
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_gif
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    if (*cinfo).out_color_space as libc::c_uint
        != crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
        && (*cinfo).out_color_space as libc::c_uint
            != crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
    {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_GIF_COLORSPACE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Force quantization if color or if > 8 bits input */
    if (*cinfo).out_color_space as libc::c_uint
        != crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
        || (*cinfo).data_precision > 8i32
    {
        /* Force quantization to at most 256 colors */
        (*cinfo).quantize_colors = crate::jmorecfg_h::TRUE;
        if (*cinfo).desired_number_of_colors > 256i32 {
            (*cinfo).desired_number_of_colors = 256i32
        }
    }
    /* Calculate output image dimensions so we can allocate space */
    crate::jpeglib_h::jpeg_calc_output_dimensions(cinfo);
    if (*cinfo).output_components != 1i32 {
        /* safety check: just one component? */
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_GIF_BUG as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Create decompressor output buffer. */
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (*cinfo).output_width,
        1i32 as crate::jmorecfg_h::JDIMENSION,
    );
    (*dest).pub_0.buffer_height = 1i32 as crate::jmorecfg_h::JDIMENSION;
    return dest as crate::src::cdjpeg::djpeg_dest_ptr;
}
/* GIF_SUPPORTED */
