use libc;

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
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
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
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_PIXELSIZE;
pub use crate::jmorecfg_h::RGB_RED;
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
pub use crate::src::wrppm::jmorecfg_h::rgb_pixelsize;
use crate::stdlib::ferror;
use crate::stdlib::fflush;
use crate::stdlib::fprintf;
use crate::stdlib::fwrite;
use crate::stdlib::memcpy;

pub type ppm_dest_ptr = *mut ppm_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppm_dest_struct {
    pub pub_0: crate::src::cdjpeg::djpeg_dest_struct,
    pub iobuffer: *mut libc::c_char,
    pub pixrow: crate::jpeglib_h::JSAMPROW,
    pub buffer_width: crate::stddef_h::size_t,
    pub samples_per_row: crate::jmorecfg_h::JDIMENSION,
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

pub const BYTESPERSAMPLE: libc::c_int = 1i32;

pub const PPM_MAXVAL: libc::c_int = 255i32;
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 *
 * put_pixel_rows handles the "normal" 8-bit case where the decompressor
 * output buffer is physically the same as the fwrite buffer.
 */

unsafe extern "C" fn put_pixel_rows(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * This code is used when we have to copy the data and apply a pixel
 * format translation.  Typically this only happens in 12-bit mode.
 */

unsafe extern "C" fn copy_pixel_rows(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    ptr = *(*dest).pub_0.buffer.offset(0);
    bufferptr = (*dest).iobuffer;
    crate::stdlib::memcpy(
        bufferptr as *mut libc::c_void,
        ptr as *const libc::c_void,
        (*dest).samples_per_row as crate::stddef_h::size_t,
    );
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Convert extended RGB to RGB.
 */

unsafe extern "C" fn put_rgb(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut rindex: libc::c_int = crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize];
    let mut gindex: libc::c_int = crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize];
    let mut bindex: libc::c_int = crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize];
    let mut ps: libc::c_int = rgb_pixelsize[(*cinfo).out_color_space as usize];
    ptr = *(*dest).pub_0.buffer.offset(0);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        let fresh0 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh0 = *ptr.offset(rindex as isize) as libc::c_char;
        let fresh1 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh1 = *ptr.offset(gindex as isize) as libc::c_char;
        let fresh2 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh2 = *ptr.offset(bindex as isize) as libc::c_char;
        ptr = ptr.offset(ps as isize);
        col =  col - 1
    }
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Convert CMYK to RGB.
 */

unsafe extern "C" fn put_cmyk(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        let mut r: crate::jmorecfg_h::JSAMPLE = 0;
        let mut g: crate::jmorecfg_h::JSAMPLE = 0;
        let mut b: crate::jmorecfg_h::JSAMPLE = 0;
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        let mut c: crate::jmorecfg_h::JSAMPLE = *fresh3;
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        let mut m: crate::jmorecfg_h::JSAMPLE = *fresh4;
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        let mut y: crate::jmorecfg_h::JSAMPLE = *fresh5;
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        let mut k: crate::jmorecfg_h::JSAMPLE = *fresh6;
        crate::cmyk_h::cmyk_to_rgb(c, m, y, k, &mut r, &mut g, &mut b);
        let fresh7 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh7 = r as libc::c_char;
        let fresh8 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh8 = g as libc::c_char;
        let fresh9 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh9 = b as libc::c_char;
        col =  col - 1
    }
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Write some pixel data when color quantization is in effect.
 * We have to demap the color index values to straight data.
 */

unsafe extern "C" fn put_demapped_rgb(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    let mut pixval: libc::c_int = 0;
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut color_map0: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut color_map1: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(1);
    let mut color_map2: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(2);
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        let fresh10 = ptr;
        ptr = ptr.offset(1);
        pixval = *fresh10 as libc::c_int;
        let fresh11 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh11 = *color_map0.offset(pixval as isize) as libc::c_int as libc::c_char;
        let fresh12 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh12 = *color_map1.offset(pixval as isize) as libc::c_int as libc::c_char;
        let fresh13 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh13 = *color_map2.offset(pixval as isize) as libc::c_int as libc::c_char;
        col =  col - 1
    }
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}

unsafe extern "C" fn put_demapped_gray(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    let mut bufferptr: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut color_map: crate::jpeglib_h::JSAMPROW = *(*cinfo).colormap.offset(0);
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    ptr = *(*dest).pub_0.buffer.offset(0);
    bufferptr = (*dest).iobuffer;
    col = (*cinfo).output_width;
    while col > 0i32 as libc::c_uint {
        let fresh14 = ptr;
        ptr = ptr.offset(1);
        let fresh15 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh15 =
            *color_map.offset(*fresh14 as libc::c_int as isize) as libc::c_int as libc::c_char;
        col =  col - 1
    }
    crate::stdlib::fwrite(
        (*dest).iobuffer as *const libc::c_void,
        1i32 as crate::stddef_h::size_t,
        (*dest).buffer_width,
        (*dest).pub_0.output_file,
    );
}
/*
 * Startup: write the file header.
 */

unsafe extern "C" fn start_output_ppm(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    /* Emit file header */
    match (*cinfo).out_color_space as libc::c_uint {
        1 => {
            /* emit header for raw PGM format */
            crate::stdlib::fprintf(
                (*dest).pub_0.output_file,
                
                b"P5\n%ld %ld\n%d\n\x00".as_ptr() as *const libc::c_char,
                (*cinfo).output_width as libc::c_long,
                (*cinfo).output_height as libc::c_long,
                PPM_MAXVAL,
            );
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 4 => {
            /* emit header for raw PPM format */
            crate::stdlib::fprintf(
                (*dest).pub_0.output_file,
                
                b"P6\n%ld %ld\n%d\n\x00".as_ptr() as *const libc::c_char,
                (*cinfo).output_width as libc::c_long,
                (*cinfo).output_height as libc::c_long,
                PPM_MAXVAL,
            );
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_PPM_COLORSPACE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_output_ppm(
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

unsafe extern "C" fn calc_buffer_dimensions_ppm(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dinfo: crate::src::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: ppm_dest_ptr = dinfo as ppm_dest_ptr;
    if (*cinfo).out_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
    {
        (*dest).samples_per_row =  (*cinfo)
            .output_width * (*cinfo).out_color_components as libc::c_uint
    } else {
        (*dest).samples_per_row =  (*cinfo).output_width * 3i32 as libc::c_uint
    }
    (*dest).buffer_width = (*dest).samples_per_row as libc::c_ulong *
    (BYTESPERSAMPLE as libc::c_ulong *
         ::std::mem::size_of::<libc::c_char>() as libc::c_ulong);
}
/*
 * The module selection routine for PPM format output.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_write_ppm(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::src::cdjpeg::djpeg_dest_ptr {
    let mut dest: ppm_dest_ptr = ::std::ptr::null_mut::< ppm_dest_struct>();
    /* Create module interface object, fill in method pointers */
    dest = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<ppm_dest_struct>() as libc::c_ulong,
    ) as ppm_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_ppm
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_ppm
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_ppm
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::src::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    /* Calculate output image dimensions so we can allocate space */
    crate::jpeglib_h::jpeg_calc_output_dimensions(cinfo);
    /* Create physical I/O buffer */
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
        (*dest).buffer_width,
    ) as *mut libc::c_char;
    if (*cinfo).quantize_colors != 0
        || crate::jconfig_h::BITS_IN_JSAMPLE != 8i32
        || ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong
            != ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        || (*cinfo).out_color_space as libc::c_uint
            != crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
            && (*cinfo).out_color_space as libc::c_uint
                != crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
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
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            
            (*cinfo)
                .output_width * (*cinfo).output_components as libc::c_uint,
            1i32 as crate::jmorecfg_h::JDIMENSION,
        );
        (*dest).pub_0.buffer_height = 1i32 as crate::jmorecfg_h::JDIMENSION;
        if (*cinfo).out_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
            || (*cinfo).out_color_space as libc::c_uint
                >= crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
                && (*cinfo).out_color_space as libc::c_uint
                    <= crate::jpeglib_h::JCS_EXT_ARGB as libc::c_int as libc::c_uint
        {
            (*dest).pub_0.put_pixel_rows = Some(
                put_rgb
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        } else if (*cinfo).out_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
        {
            (*dest).pub_0.put_pixel_rows = Some(
                put_cmyk
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        } else if (*cinfo).quantize_colors == 0 {
            (*dest).pub_0.put_pixel_rows = Some(
                copy_pixel_rows
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        } else if (*cinfo).out_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
        {
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
                put_demapped_rgb
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::src::cdjpeg::djpeg_dest_ptr,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        }
    } else {
        /* We will fwrite() directly from decompressor output buffer. */
        /* Synthesize a JSAMPARRAY pointer structure */
        (*dest).pixrow = (*dest).iobuffer as crate::jpeglib_h::JSAMPROW;
        (*dest).pub_0.buffer = &mut (*dest).pixrow;
        (*dest).pub_0.buffer_height = 1i32 as crate::jmorecfg_h::JDIMENSION;
        (*dest).pub_0.put_pixel_rows = Some(
            put_pixel_rows
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::src::cdjpeg::djpeg_dest_ptr,
                    _: crate::jmorecfg_h::JDIMENSION,
                ) -> (),
        )
    }
    return dest as crate::src::cdjpeg::djpeg_dest_ptr;
}
/* PPM_SUPPORTED */
