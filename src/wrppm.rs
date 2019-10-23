use libc::{c_uint, c_ulong, c_char, c_long, c_void, c_int, self};

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:22"]
pub mod jmorecfg_h {
    /*
     * jmorecfg.h
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1997, Thomas G. Lane.
     * Modified 1997-2009 by Guido Vollbeding.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009, 2011, 2014-2015, 2018, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains additional configuration options that customize the
     * JPEG software for special applications or support machine-dependent
     * optimizations.  Most users will not need to touch this file.
     */
    /*
     * Maximum number of components (color channels) allowed in JPEG image.
     * To meet the letter of Rec. ITU-T T.81 | ISO/IEC 10918-1, set this to 255.
     * However, darn few applications need more than 4 channels (maybe 5 for CMYK +
     * alpha mask).  We recommend 10 as a reasonable compromise; use 4 if you are
     * really short on memory.  (Each allowed component costs a hundred or so
     * bytes of storage, whether actually used in an image or not.)
     */
    /* maximum number of image components */
    /*
     * Basic data types.
     * You may need to change these if you have a machine with unusual data
     * type sizes; for example, "char" not 8 bits, "short" not 16 bits,
     * or "long" not 32 bits.  We don't care whether "int" is 16 or 32 bits,
     * but it had better be at least 16.
     */
    /* Representation of a single sample (pixel element value).
     * We frequently allocate large arrays of these, so it's important to keep
     * them small.  But if you have memory to burn and access to char or short
     * arrays is very slow on your hardware, you might want to change these.
     */
    /* JSAMPLE should be the smallest type that will hold the values 0..255.
     * You can use a signed char by having GETJSAMPLE mask it with 0xFF.
     */

    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* BITS_IN_JSAMPLE == 8 */
    /* BITS_IN_JSAMPLE == 12 */
    /* Representation of a DCT frequency coefficient.
     * This should be a signed value of at least 16 bits; "short" is usually OK.
     * Again, we allocate large arrays of these, but you can change to int
     * if you have memory to burn and "short" is really slow.
     */

    /* Compressed datastreams are represented as arrays of JOCTET.
     * These must be EXACTLY 8 bits wide, at least once they are written to
     * external storage.  Note that when using the stdio data source/destination
     * managers, this is also the data type passed to fread/fwrite.
     */

    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* These typedefs are used for various table entries and so forth.
     * They must be at least as wide as specified; but making them too big
     * won't cost a huge amount of memory, so we don't provide special
     * extraction code like we did for JSAMPLE.  (In other words, these
     * typedefs live at a different point on the speed/space tradeoff curve.)
     */
    /* UINT8 must hold at least the values 0..255. */

    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* UINT16 must hold at least the values 0..65535. */

    /* Datatype used for image dimensions.  The JPEG standard only supports
     * images up to 64K*64K due to 16-bit fields in SOF markers.  Therefore
     * "unsigned int" is sufficient on all machines.  However, if you need to
     * handle larger images and you don't mind deviating from the spec, you
     * can change this datatype.  (Note that changing this datatype will
     * potentially require modifying the SIMD code.  The x86-64 SIMD extensions,
     * in particular, assume a 32-bit JDIMENSION.)
     */

    /* a tad under 64K to prevent overflows */
    /* These macros are used in all function definitions and extern declarations.
     * You could modify them if you need to change function linkage conventions;
     * in particular, you'll need to do that to make the library a Windows DLL.
     * Another application is to make all functions global for use with debuggers
     * or code profilers that require it.
     */
    /* a function called through method pointers: */
    /* a function used only in its module: */
    /* a function referenced thru EXTERNs: */
    /* a reference to a GLOBAL function: */
    /* Originally, this macro was used as a way of defining function prototypes
     * for both modern compilers as well as older compilers that did not support
     * prototype parameters.  libjpeg-turbo has never supported these older,
     * non-ANSI compilers, but the macro is still included because there is some
     * software out there that uses it.
     */
    /* libjpeg-turbo no longer supports platforms that have far symbols (MS-DOS),
     * but again, some software relies on this macro.
     */
    /*
     * On a few systems, type boolean and/or its values FALSE, TRUE may appear
     * in standard header files.  Or you may have conflicts with application-
     * specific header files that you want to include together with these files.
     * Defining HAVE_BOOLEAN before including jpeglib.h should make it work.
     */

    /* in case these macros already exist */
    /* values of boolean */
    /*
     * The remaining options affect code selection within the JPEG library,
     * but they don't need to be visible to most applications using the library.
     * To minimize application namespace pollution, the symbols won't be
     * defined unless JPEG_INTERNALS or JPEG_INTERNAL_OPTIONS has been defined.
     */
    /*
     * These defines indicate whether to include various optional functions.
     * Undefining some of these symbols will produce a smaller but less capable
     * library.  Note that you can leave certain source files out of the
     * compilation/linking process if you've #undef'd the corresponding symbols.
     * (You may HAVE to do that if your compiler doesn't like null source files.)
     */
    /* Capability options common to encoder and decoder: */
    /* slow but accurate integer algorithm */
    /* faster, less accurate integer method */
    /* floating-point: accurate, fast on fast HW */
    /* Encoder capability options: */
    /* Multiple-scan JPEG files? */
    /* Progressive JPEG? (Requires MULTISCAN)*/
    /* Optimization of entropy coding parms? */
    /* Note: if you selected 12-bit data precision, it is dangerous to turn off
     * ENTROPY_OPT_SUPPORTED.  The standard Huffman tables are only good for 8-bit
     * precision, so jchuff.c normally uses entropy optimization to compute
     * usable tables for higher precision.  If you don't want to do optimization,
     * you'll have to supply different default Huffman tables.
     * The exact same statements apply for progressive JPEG: the default tables
     * don't work for progressive mode.  (This may get fixed, however.)
     */
    /* Input image smoothing option? */
    /* Decoder capability options: */
    /* Multiple-scan JPEG files? */
    /* Progressive JPEG? (Requires MULTISCAN)*/
    /* jpeg_save_markers() needed? */
    /* Block smoothing? (Progressive only) */
    /* Output rescaling via IDCT? */
    /* Output rescaling at upsample stage? */
    /* Fast path for sloppy upsampling? */
    /* 1-pass color quantization? */
    /* 2-pass color quantization? */
    /* more capability options later, no doubt */
    /*
     * The RGB_RED, RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE macros are a vestigial
     * feature of libjpeg.  The idea was that, if an application developer needed
     * to compress from/decompress to a BGR/BGRX/RGBX/XBGR/XRGB buffer, they could
     * change these macros, rebuild libjpeg, and link their application statically
     * with it.  In reality, few people ever did this, because there were some
     * severe restrictions involved (cjpeg and djpeg no longer worked properly,
     * compressing/decompressing RGB JPEGs no longer worked properly, and the color
     * quantizer wouldn't work with pixel sizes other than 3.)  Furthermore, since
     * all of the O/S-supplied versions of libjpeg were built with the default
     * values of RGB_RED, RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE, many applications
     * have come to regard these values as immutable.
     *
     * The libjpeg-turbo colorspace extensions provide a much cleaner way of
     * compressing from/decompressing to buffers with arbitrary component orders
     * and pixel sizes.  Thus, we do not support changing the values of RGB_RED,
     * RGB_GREEN, RGB_BLUE, or RGB_PIXELSIZE.  In addition to the restrictions
     * listed above, changing these values will also break the SIMD extensions and
     * the regression tests.
     */
    /* Offset of Red in an RGB scanline element */
    /* Offset of Green */
    /* Offset of Blue */
    /* JSAMPLEs per RGB scanline element */

    use crate::jmorecfg_h::{EXT_RGB_PIXELSIZE, EXT_RGBX_PIXELSIZE,
                        EXT_XBGR_PIXELSIZE, EXT_BGR_PIXELSIZE,
                        EXT_BGRX_PIXELSIZE, EXT_XRGB_PIXELSIZE,
                        RGB_PIXELSIZE};use libc::c_int;pub static mut rgb_pixelsize: [c_int; 17] = [
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





















































































































































































































































































































use crate::stdlib::{ferror, fflush, fprintf, fwrite, memcpy};pub use crate::cderror_h::{C2RustUnnamed_4, JERR_BAD_CMAP_FILE,
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
                           JWRN_GIF_ENDCODE, JWRN_GIF_NOMOREDATA};pub use crate::jpegint_h::{inverse_DCT_method_ptr, JBUF_CRANK_DEST,
                           JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
                           JBUF_SAVE_SOURCE, J_BUF_MODE};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_calc_output_dimensions,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_parser_method,
                           jpeg_marker_reader, jpeg_marker_struct,
                           jpeg_memory_mgr, jpeg_progress_mgr,
                           jpeg_saved_marker_ptr, jpeg_source_mgr,
                           jpeg_upsampler, jvirt_barray_control,
                           jvirt_barray_ptr, jvirt_sarray_control,
                           jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr,
                           JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
                           JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
                           JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
                           J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE};pub use crate::jmorecfg_h::{boolean, rgb_blue, rgb_green, rgb_red,
                            EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE,
                            EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN,
                            EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_RGBX_BLUE,
                            EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED,
                            EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE,
                            EXT_RGB_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN,
                            EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE,
                            EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE, EXT_XRGB_RED,
                            JCOEF, JDIMENSION, JOCTET, JSAMPLE, RGB_BLUE,
                            RGB_GREEN, RGB_PIXELSIZE, RGB_RED, UINT16, UINT8};pub use crate::cmyk_h::cmyk_to_rgb;pub use crate::stddef_h::size_t;pub use crate::jconfig_h::BITS_IN_JSAMPLE;pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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
                        JWRN_TOO_MUCH_DATA};pub use super::cdjpeg::{djpeg_dest_ptr, djpeg_dest_struct};pub use jmorecfg_h::rgb_pixelsize;

pub type ppm_dest_ptr = *mut ppm_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppm_dest_struct {
    pub pub_0: super::cdjpeg::djpeg_dest_struct,
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
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    fwrite(
        (*dest).iobuffer as *const c_void,
        1u64,
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
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
      let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    
    
    
     let mut ptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0); let mut bufferptr:   *mut c_char =  (*dest).iobuffer;
    memcpy(
        bufferptr as *mut c_void,
        ptr as *const c_void,
        (*dest).samples_per_row as size_t,
    );
    fwrite(
        (*dest).iobuffer as *const c_void,
        1u64,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Convert extended RGB to RGB.
 */

unsafe extern "C" fn put_rgb(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
       let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    
    
    
    let mut rindex: c_int = rgb_red[(*cinfo).out_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).out_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).out_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).out_color_space as usize];
    
    
     let mut ptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0); let mut bufferptr:   *mut c_char =  (*dest).iobuffer; let mut col:   JDIMENSION =  (*cinfo).output_width;
    while col > 0u32 {
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
        col -=  1
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1u64,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Convert CMYK to RGB.
 */

unsafe extern "C" fn put_cmyk(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
       let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    
    
    
    
    
     let mut ptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0); let mut bufferptr:   *mut c_char =  (*dest).iobuffer; let mut col:   JDIMENSION =  (*cinfo).output_width;
    while col > 0u32 {
        
        
         let mut r:  JSAMPLE =  0; let mut g:  JSAMPLE =  0; let mut b:  JSAMPLE =  0;
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
        col -=  1
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1u64,
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
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
       let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    
    
    
    let mut color_map0: JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut color_map1: JSAMPROW = *(*cinfo).colormap.offset(1);
    let mut color_map2: JSAMPROW = *(*cinfo).colormap.offset(2);
    
    
    
     let mut ptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0); let mut bufferptr:   *mut c_char =  (*dest).iobuffer; let mut col:   JDIMENSION =  (*cinfo).output_width;
    while col > 0u32 {
         let fresh10 = ptr;
        ptr = ptr.offset(1);
         let mut pixval:   c_int =  *fresh10 as c_int;
        let fresh11 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh11 =  *color_map0.offset(pixval as isize) as c_char;
        let fresh12 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh12 =  *color_map1.offset(pixval as isize) as c_char;
        let fresh13 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh13 =  *color_map2.offset(pixval as isize) as c_char;
        col -=  1
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1u64,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}

unsafe extern "C" fn put_demapped_gray(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut _rows_supplied: JDIMENSION,
) {
       let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    
    
    let mut color_map: JSAMPROW = *(*cinfo).colormap.offset(0);
    
    
    
     let mut ptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0); let mut bufferptr:   *mut c_char =  (*dest).iobuffer; let mut col:   JDIMENSION =  (*cinfo).output_width;
    while col > 0u32 {
        let fresh14 = ptr;
        ptr = ptr.offset(1);
        let fresh15 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh15 =
            
            *color_map.offset(*fresh14 as c_int as isize) as c_char;
        col -=  1
    }
    fwrite(
        (*dest).iobuffer as *const c_void,
        1u64,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Startup: write the file header.
 */

unsafe extern "C" fn start_output_ppm(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    /* Emit file header */
    match  (*cinfo).out_color_space {
        1 => {
            /* emit header for raw PGM format */
            fprintf(
                (*dest).pub_0.output_file,
                
                b"P5\n%ld %ld\n%d\n\x00".as_ptr() as *const c_char,
                (*cinfo).output_width as c_long,
                (*cinfo).output_height as c_long,
                PPM_MAXVAL,
            );
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 4 => {
            /* emit header for raw PPM format */
            fprintf(
                (*dest).pub_0.output_file,
                
                b"P6\n%ld %ld\n%d\n\x00".as_ptr() as *const c_char,
                (*cinfo).output_width as c_long,
                (*cinfo).output_height as c_long,
                PPM_MAXVAL,
            );
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_PPM_COLORSPACE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_output_ppm(
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

unsafe extern "C" fn calc_buffer_dimensions_ppm(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    if  (*cinfo).out_color_space
        ==  JCS_GRAYSCALE
    {
        (*dest).samples_per_row =  (*cinfo)
            .output_width * (*cinfo).out_color_components as c_uint
    } else {
        (*dest).samples_per_row =  (*cinfo).output_width * 3u32
    }
    (*dest).buffer_width = (*dest).samples_per_row as c_ulong *
    (BYTESPERSAMPLE as c_ulong *
         ::std::mem::size_of::<c_char>() as c_ulong);
}
/*
 * The module selection routine for PPM format output.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_write_ppm(
    mut cinfo: j_decompress_ptr,
) -> super::cdjpeg::djpeg_dest_ptr {
     
    /* Create module interface object, fill in method pointers */
     let mut dest:   ppm_dest_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<ppm_dest_struct>() as c_ulong,
    ) as ppm_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_ppm
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_ppm
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_ppm
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    /* Calculate output image dimensions so we can allocate space */
    jpeg_calc_output_dimensions(cinfo);
    /* Create physical I/O buffer */
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
        (*dest).buffer_width,
    ) as *mut c_char;
    if (*cinfo).quantize_colors != 0
        || BITS_IN_JSAMPLE != 8i32
        || ::std::mem::size_of::<JSAMPLE>() as c_ulong
            != ::std::mem::size_of::<c_char>() as c_ulong
        ||  (*cinfo).out_color_space
            !=  JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                !=  JCS_RGB
    {
        /* When quantizing, we need an output buffer for colormap indexes
         * that's separate from the physical I/O buffer.  We also need a
         * separate buffer if pixel format translation must take place.
         */
        (*dest).pub_0.buffer = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            
            (*cinfo)
                .output_width * (*cinfo).output_components as c_uint,
            1u32,
        );
        (*dest).pub_0.buffer_height = 1u32;
        if  (*cinfo).out_color_space
            ==  JCS_RGB
            ||  (*cinfo).out_color_space
                >=  JCS_EXT_RGB
                &&  (*cinfo).out_color_space
                    <=  JCS_EXT_ARGB
        {
            (*dest).pub_0.put_pixel_rows = Some(
                put_rgb
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else if  (*cinfo).out_color_space
            ==  JCS_CMYK
        {
            (*dest).pub_0.put_pixel_rows = Some(
                put_cmyk
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else if (*cinfo).quantize_colors == 0 {
            (*dest).pub_0.put_pixel_rows = Some(
                copy_pixel_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else if  (*cinfo).out_color_space
            ==  JCS_GRAYSCALE
        {
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
                put_demapped_rgb
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
    } else {
        /* We will fwrite() directly from decompressor output buffer. */
        /* Synthesize a JSAMPARRAY pointer structure */
        (*dest).pixrow = (*dest).iobuffer as JSAMPROW;
        (*dest).pub_0.buffer = &mut (*dest).pixrow;
        (*dest).pub_0.buffer_height = 1u32;
        (*dest).pub_0.put_pixel_rows = Some(
            put_pixel_rows
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: super::cdjpeg::djpeg_dest_ptr,
                    _: JDIMENSION,
                ) -> (),
        )
    }
    return dest as super::cdjpeg::djpeg_dest_ptr;
}
/* PPM_SUPPORTED */
