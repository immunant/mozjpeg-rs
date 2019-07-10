use libc;
use libc::c_char;
use libc::c_double;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg-rs/mozjpeg-c/jmorecfg.h:19"]
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
    use crate::jmorecfg_h::EXT_BGRX_BLUE;
    use crate::jmorecfg_h::EXT_BGRX_GREEN;
    use crate::jmorecfg_h::EXT_BGRX_RED;
    use crate::jmorecfg_h::EXT_BGR_BLUE;
    use crate::jmorecfg_h::EXT_BGR_GREEN;
    use crate::jmorecfg_h::EXT_BGR_RED;
    use crate::jmorecfg_h::EXT_RGBX_BLUE;
    use crate::jmorecfg_h::EXT_RGBX_GREEN;
    use crate::jmorecfg_h::EXT_RGBX_RED;
    use crate::jmorecfg_h::EXT_RGB_BLUE;
    use crate::jmorecfg_h::EXT_RGB_GREEN;
    use crate::jmorecfg_h::EXT_RGB_RED;
    use crate::jmorecfg_h::EXT_XBGR_BLUE;
    use crate::jmorecfg_h::EXT_XBGR_GREEN;
    use crate::jmorecfg_h::EXT_XBGR_RED;
    use crate::jmorecfg_h::EXT_XRGB_BLUE;
    use crate::jmorecfg_h::EXT_XRGB_GREEN;
    use crate::jmorecfg_h::EXT_XRGB_RED;
    use crate::jmorecfg_h::RGB_BLUE_5;
    use crate::jmorecfg_h::RGB_GREEN_5;
    use crate::jmorecfg_h::RGB_RED_5;
    use libc::c_int;
    pub static mut rgb_red: [c_int; 17] = [
        -1i32,
        -1i32,
        RGB_RED_5,
        -1i32,
        -1i32,
        -1i32,
        EXT_RGB_RED,
        EXT_RGBX_RED,
        EXT_BGR_RED,
        EXT_BGRX_RED,
        EXT_XBGR_RED,
        EXT_XRGB_RED,
        EXT_RGBX_RED,
        EXT_BGRX_RED,
        EXT_XBGR_RED,
        EXT_XRGB_RED,
        -1i32,
    ];
    pub static mut rgb_blue: [c_int; 17] = [
        -1i32,
        -1i32,
        RGB_BLUE_5,
        -1i32,
        -1i32,
        -1i32,
        EXT_RGB_BLUE,
        EXT_RGBX_BLUE,
        EXT_BGR_BLUE,
        EXT_BGRX_BLUE,
        EXT_XBGR_BLUE,
        EXT_XRGB_BLUE,
        EXT_RGBX_BLUE,
        EXT_BGRX_BLUE,
        EXT_XBGR_BLUE,
        EXT_XRGB_BLUE,
        -1i32,
    ];
    pub static mut rgb_green: [c_int; 17] = [
        -1i32,
        -1i32,
        RGB_GREEN_5,
        -1i32,
        -1i32,
        -1i32,
        EXT_RGB_GREEN,
        EXT_RGBX_GREEN,
        EXT_BGR_GREEN,
        EXT_BGRX_GREEN,
        EXT_XBGR_GREEN,
        EXT_XRGB_GREEN,
        EXT_RGBX_GREEN,
        EXT_BGRX_GREEN,
        EXT_XBGR_GREEN,
        EXT_XRGB_GREEN,
        -1i32,
    ];
}
pub use crate::jdcol565_c::gray_rgb565D_convert_be;
pub use crate::jdcol565_c::gray_rgb565D_convert_le;
pub use crate::jdcol565_c::gray_rgb565_convert_be;
pub use crate::jdcol565_c::gray_rgb565_convert_le;
pub use crate::jdcol565_c::rgb_rgb565D_convert_be;
pub use crate::jdcol565_c::rgb_rgb565D_convert_le;
pub use crate::jdcol565_c::rgb_rgb565_convert_be;
pub use crate::jdcol565_c::rgb_rgb565_convert_le;
pub use crate::jdcol565_c::ycc_rgb565D_convert_be;
pub use crate::jdcol565_c::ycc_rgb565D_convert_le;
pub use crate::jdcol565_c::ycc_rgb565_convert_be;
pub use crate::jdcol565_c::ycc_rgb565_convert_le;
pub use crate::jdcolext_c::gray_extbgr_convert_internal;
pub use crate::jdcolext_c::gray_extbgrx_convert_internal;
pub use crate::jdcolext_c::gray_extrgb_convert_internal;
pub use crate::jdcolext_c::gray_extrgbx_convert_internal;
pub use crate::jdcolext_c::gray_extxbgr_convert_internal;
pub use crate::jdcolext_c::gray_extxrgb_convert_internal;
pub use crate::jdcolext_c::gray_rgb_convert_internal;
pub use crate::jdcolext_c::rgb_extbgr_convert_internal;
pub use crate::jdcolext_c::rgb_extbgrx_convert_internal;
pub use crate::jdcolext_c::rgb_extrgb_convert_internal;
pub use crate::jdcolext_c::rgb_extrgbx_convert_internal;
pub use crate::jdcolext_c::rgb_extxbgr_convert_internal;
pub use crate::jdcolext_c::rgb_extxrgb_convert_internal;
pub use crate::jdcolext_c::rgb_rgb_convert_internal;
pub use crate::jdcolext_c::ycc_extbgr_convert_internal;
pub use crate::jdcolext_c::ycc_extbgrx_convert_internal;
pub use crate::jdcolext_c::ycc_extrgb_convert_internal;
pub use crate::jdcolext_c::ycc_extrgbx_convert_internal;
pub use crate::jdcolext_c::ycc_extxbgr_convert_internal;
pub use crate::jdcolext_c::ycc_extxrgb_convert_internal;
pub use crate::jdcolext_c::ycc_rgb_convert_internal;
pub use crate::jerror::C2RustUnnamed_3;
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
pub use crate::jmorecfg_h::rgb_pixelsize;
pub use crate::jmorecfg_h::CENTERJSAMPLE;
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
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE_5;
pub use crate::jmorecfg_h::RGB_GREEN_5;
pub use crate::jmorecfg_h::RGB_PIXELSIZE_5;
pub use crate::jmorecfg_h::RGB_RED_5;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jcopy_sample_rows;
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
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
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
use crate::jsimd::jsimd_can_ycc_rgb;
use crate::jsimd::jsimd_can_ycc_rgb565;
use crate::jsimd::jsimd_ycc_rgb565_convert;
use crate::jsimd::jsimd_ycc_rgb_convert;
pub use crate::stddef_h::size_t;
pub use jmorecfg_h::rgb_blue;
pub use jmorecfg_h::rgb_green;
pub use jmorecfg_h::rgb_red;
pub type my_cconvert_ptr = *mut my_color_deconverter;
/*
 * jdcolor.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2011 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2009, 2011-2012, 2014-2015, D. R. Commander.
 * Copyright (C) 2013, Linaro Limited.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains output colorspace conversion routines.
 */
/* Private subobject */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_deconverter {
    pub pub_0: jpeg_color_deconverter,
    pub Cr_r_tab: *mut c_int,
    pub Cb_b_tab: *mut c_int,
    pub Cr_g_tab: *mut JLONG,
    pub Cb_g_tab: *mut JLONG,
    pub rgb_y_tab: *mut JLONG,
}
/* *************** YCbCr -> RGB conversion: most common case **************/
/* ***************   RGB -> Y   conversion: less common case **************/
/*
 * YCbCr is defined per CCIR 601-1, except that Cb and Cr are
 * normalized to the range 0..MAXJSAMPLE rather than -0.5 .. 0.5.
 * The conversion equations to be implemented are therefore
 *
 *      R = Y                + 1.40200 * Cr
 *      G = Y - 0.34414 * Cb - 0.71414 * Cr
 *      B = Y + 1.77200 * Cb
 *
 *      Y = 0.29900 * R + 0.58700 * G + 0.11400 * B
 *
 * where Cb and Cr represent the incoming values less CENTERJSAMPLE.
 * (These numbers are derived from TIFF 6.0 section 21, dated 3-June-92.)
 *
 * To avoid floating-point arithmetic, we represent the fractional constants
 * as integers scaled up by 2^16 (about 4 digits precision); we have to divide
 * the products by 2^16, with appropriate rounding, to get the correct answer.
 * Notice that Y, being an integral input, does not contribute any fraction
 * so it need not participate in the rounding.
 *
 * For even more speed, we avoid doing any multiplications in the inner loop
 * by precalculating the constants times Cb and Cr for all possible values.
 * For 8-bit JSAMPLEs this is very reasonable (only 256 entries per table);
 * for 12-bit samples it is still acceptable.  It's not very reasonable for
 * 16-bit samples, but if you want lossless storage you shouldn't be changing
 * colorspace anyway.
 * The Cr=>R and Cb=>B values can be rounded to integers in advance; the
 * values for the G calculation are left scaled up, since we must add them
 * together before rounding.
 */
/* speediest right-shift on some machines */
pub const SCALEBITS: c_int = 16i32;
pub const ONE_HALF: JLONG = (1i32 as JLONG) << SCALEBITS - 1i32;
/* We allocate one big table for RGB->Y conversion and divide it up into
 * three parts, instead of doing three alloc_small requests.  This lets us
 * use a single table base address, which can be held in a register in the
 * inner loops on many machines (more than can hold all three addresses,
 * anyway).
 */
/* offset to R => Y section */
pub const R_Y_OFF: c_int = 0i32;
/* offset to G => Y section */
pub const G_Y_OFF: c_int = 1i32 * (MAXJSAMPLE + 1i32);
/* etc. */
pub const B_Y_OFF: c_int = 2i32 * (MAXJSAMPLE + 1i32);
pub const TABLE_SIZE: c_int = 3i32 * (MAXJSAMPLE + 1i32);
/* Include inline routines for colorspace extensions */
pub const RGB_RED_4: c_int = EXT_RGB_RED;
pub const RGB_GREEN_4: c_int = EXT_RGB_GREEN;
pub const RGB_BLUE_4: c_int = EXT_RGB_BLUE;
pub const RGB_PIXELSIZE_4: c_int = EXT_RGB_PIXELSIZE;
pub const RGB_RED_2: c_int = EXT_RGBX_RED;
pub const RGB_GREEN_2: c_int = EXT_RGBX_GREEN;
pub const RGB_BLUE_2: c_int = EXT_RGBX_BLUE;
pub const RGB_ALPHA_2: c_int = 3i32;
pub const RGB_PIXELSIZE_2: c_int = EXT_RGBX_PIXELSIZE;
pub const RGB_RED_3: c_int = EXT_BGR_RED;
pub const RGB_GREEN_3: c_int = EXT_BGR_GREEN;
pub const RGB_BLUE_3: c_int = EXT_BGR_BLUE;
pub const RGB_PIXELSIZE_3: c_int = EXT_BGR_PIXELSIZE;
pub const RGB_RED_1: c_int = EXT_BGRX_RED;
pub const RGB_GREEN_1: c_int = EXT_BGRX_GREEN;
pub const RGB_BLUE_1: c_int = EXT_BGRX_BLUE;
pub const RGB_ALPHA_1: c_int = 3i32;
pub const RGB_PIXELSIZE_1: c_int = EXT_BGRX_PIXELSIZE;
pub const RGB_RED_0: c_int = EXT_XBGR_RED;
pub const RGB_GREEN_0: c_int = EXT_XBGR_GREEN;
pub const RGB_BLUE_0: c_int = EXT_XBGR_BLUE;
pub const RGB_ALPHA_0: c_int = 0i32;
pub const RGB_PIXELSIZE_0: c_int = EXT_XBGR_PIXELSIZE;
pub const RGB_RED: c_int = EXT_XRGB_RED;
pub const RGB_GREEN: c_int = EXT_XRGB_GREEN;
pub const RGB_BLUE: c_int = EXT_XRGB_BLUE;
pub const RGB_ALPHA: c_int = 0i32;
pub const RGB_PIXELSIZE: c_int = EXT_XRGB_PIXELSIZE;
/*
 * Initialize tables for YCC->RGB colorspace conversion.
 */
unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut i: c_int = 0;
    let mut x: JLONG = 0;
    (*cconvert).Cr_r_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut c_int;
    (*cconvert).Cb_b_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
    ) as *mut c_int;
    (*cconvert).Cr_g_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    (*cconvert).Cb_g_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1i32) as c_ulong).wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    i = 0i32;
    x = -CENTERJSAMPLE as JLONG;
    while i <= MAXJSAMPLE {
        *(*cconvert).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG * x
                + ((1i32 as JLONG) << 16i32 - 1i32)
                >> 16i32) as c_int;
        *(*cconvert).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * (1i64 << 16i32) as c_double + 0.5f64) as JLONG * x
                + ((1i32 as JLONG) << 16i32 - 1i32)
                >> 16i32) as c_int;
        *(*cconvert).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * x;
        *(*cconvert).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG) * x + ONE_HALF;
        i += 1;
        x += 1
    }
}
/*
 * Convert some rows of samples to the output colorspace.
 */
unsafe extern "C" fn ycc_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            ycc_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            ycc_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            ycc_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            ycc_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            ycc_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            ycc_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            ycc_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/* *************** Cases other than YCbCr -> RGB **************/
/*
 * Initialize for RGB->grayscale colorspace conversion.
 */
unsafe extern "C" fn build_rgb_y_table(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_y_tab: *mut JLONG = 0 as *mut JLONG;
    let mut i: JLONG = 0;
    rgb_y_tab = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (TABLE_SIZE as c_ulong).wrapping_mul(::std::mem::size_of::<JLONG>() as c_ulong),
    ) as *mut JLONG;
    (*cconvert).rgb_y_tab = rgb_y_tab;
    i = 0i32 as JLONG;
    while i <= MAXJSAMPLE as c_long {
        *rgb_y_tab.offset((i + R_Y_OFF as c_long) as isize) =
            (0.29900f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i;
        *rgb_y_tab.offset((i + G_Y_OFF as c_long) as isize) =
            (0.58700f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i;
        *rgb_y_tab.offset((i + B_Y_OFF as c_long) as isize) =
            (0.11400f64 * (1i64 << SCALEBITS) as c_double + 0.5f64) as JLONG * i + ONE_HALF;
        i += 1
    }
}
/*
 * Convert RGB to grayscale.
 */
unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: c_int = 0;
    let mut g: c_int = 0;
    let mut b: c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_y_tab;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh42 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh42;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            r = *inptr0.offset(col as isize) as c_int;
            g = *inptr1.offset(col as isize) as c_int;
            b = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Color conversion for no colorspace change: just copy the data,
 * converting from separate-planes to interleaved representation.
 */
unsafe extern "C" fn null_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr3: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_components: c_int = (*cinfo).num_components;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut ci: c_int = 0;
    if num_components == 3i32 {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh43 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh43;
            col = 0i32 as JDIMENSION;
            while col < num_cols {
                let fresh44 = outptr;
                outptr = outptr.offset(1);
                *fresh44 = *inptr0.offset(col as isize);
                let fresh45 = outptr;
                outptr = outptr.offset(1);
                *fresh45 = *inptr1.offset(col as isize);
                let fresh46 = outptr;
                outptr = outptr.offset(1);
                *fresh46 = *inptr2.offset(col as isize);
                col = col.wrapping_add(1)
            }
        }
    } else if num_components == 4i32 {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
            inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
            inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
            inptr3 = *(*input_buf.offset(3isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh47 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh47;
            col = 0i32 as JDIMENSION;
            while col < num_cols {
                let fresh48 = outptr;
                outptr = outptr.offset(1);
                *fresh48 = *inptr0.offset(col as isize);
                let fresh49 = outptr;
                outptr = outptr.offset(1);
                *fresh49 = *inptr1.offset(col as isize);
                let fresh50 = outptr;
                outptr = outptr.offset(1);
                *fresh50 = *inptr2.offset(col as isize);
                let fresh51 = outptr;
                outptr = outptr.offset(1);
                *fresh51 = *inptr3.offset(col as isize);
                col = col.wrapping_add(1)
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0i32) {
                break;
            }
            ci = 0i32;
            while ci < num_components {
                inptr = *(*input_buf.offset(ci as isize)).offset(input_row as isize);
                outptr = *output_buf;
                col = 0i32 as JDIMENSION;
                while col < num_cols {
                    *outptr.offset(ci as isize) = *inptr.offset(col as isize);
                    outptr = outptr.offset(num_components as isize);
                    col = col.wrapping_add(1)
                }
                ci += 1
            }
            output_buf = output_buf.offset(1isize);
            input_row = input_row.wrapping_add(1)
        }
    };
}
/*
 * Color conversion for grayscale: just copy the data.
 * This also works for YCbCr -> grayscale conversion, in which
 * we just copy the Y (luminance) component and ignore chrominance.
 */
unsafe extern "C" fn grayscale_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    jcopy_sample_rows(
        *input_buf.offset(0isize),
        input_row as c_int,
        output_buf,
        0i32,
        num_rows,
        (*cinfo).output_width,
    );
}
/*
 * Convert grayscale to RGB
 */
unsafe extern "C" fn gray_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            gray_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            gray_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            gray_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            gray_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            gray_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            gray_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            gray_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/*
 * Convert plain RGB to extended RGB
 */
unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    match (*cinfo).out_color_space as c_uint {
        6 => {
            rgb_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            rgb_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            rgb_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            rgb_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            rgb_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            rgb_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            rgb_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
/*
 * Adobe-style YCCK->CMYK conversion.
 * We convert YCbCr to R=1-C, G=1-M, and B=1-Y using the same
 * conversion as above, while passing K (black) unchanged.
 * We assume build_ycc_rgb_table has been called.
 */
unsafe extern "C" fn ycck_cmyk_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: c_int = 0;
    let mut cb: c_int = 0;
    let mut cr: c_int = 0;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inptr3: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2isize)).offset(input_row as isize);
        inptr3 = *(*input_buf.offset(3isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh52 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh52;
        col = 0i32 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as c_int;
            cb = *inptr1.offset(col as isize) as c_int;
            cr = *inptr2.offset(col as isize) as c_int;
            *outptr.offset(0isize) =
                *range_limit.offset((MAXJSAMPLE - (y + *Crrtab.offset(cr as isize))) as isize);
            *outptr.offset(1isize) = *range_limit.offset(
                (MAXJSAMPLE
                    - (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16i32)
                        as c_int)) as isize,
            );
            *outptr.offset(2isize) =
                *range_limit.offset((MAXJSAMPLE - (y + *Cbbtab.offset(cb as isize))) as isize);
            *outptr.offset(3isize) = *inptr3.offset(col as isize);
            outptr = outptr.offset(4isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * RGB565 conversion
 */
/* Declarations for ordered dithering
 *
 * We use a 4x4 ordered dither array packed into 32 bits.  This array is
 * sufficent for dithering RGB888 to RGB565.
 */
pub const DITHER_MASK: c_int = 0x3i32;
pub(crate) static mut dither_matrix: [JLONG; 4] = [
    0x8020ai32 as JLONG,
    0xc040e06i32 as JLONG,
    0x30b0109i32 as JLONG,
    0xf070d05i32 as JLONG,
];
#[inline(always)]
unsafe extern "C" fn is_big_endian() -> boolean {
    let mut test_value: c_int = 1i32;
    if *(&mut test_value as *mut c_int as *mut c_char) as c_int != 1i32 {
        return TRUE;
    }
    return FALSE;
}
/* Include inline routines for RGB565 conversion */
unsafe extern "C" fn ycc_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        ycc_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn ycc_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        ycc_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn rgb_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        rgb_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn rgb_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        rgb_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn gray_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        gray_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn gray_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    if 0 != is_big_endian() {
        gray_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
/*
 * Empty method for start_pass.
 */
unsafe extern "C" fn start_pass_dcolor(mut cinfo: j_decompress_ptr) {}
/* no work needed */
/*
 * Module initialization routine for output colorspace conversion.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_color_deconverter(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = 0 as *mut my_color_deconverter;
    let mut ci: c_int = 0;
    cconvert = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_color_deconverter>() as c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut jpeg_color_deconverter;
    (*cconvert).pub_0.start_pass =
        Some(start_pass_dcolor as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    match (*cinfo).jpeg_color_space as c_uint {
        1 => {
            if (*cinfo).num_components != 1i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 3 => {
            if (*cinfo).num_components != 3i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 | 5 => {
            if (*cinfo).num_components != 4i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).num_components < 1i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    match (*cinfo).out_color_space as c_uint {
        1 => {
            (*cinfo).out_color_components = 1i32;
            if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint
                || (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                ci = 1i32;
                while ci < (*cinfo).num_components {
                    (*(*cinfo).comp_info.offset(ci as isize)).component_needed = FALSE;
                    ci += 1
                }
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_gray_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                build_rgb_y_table(cinfo);
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            (*cinfo).out_color_components = rgb_pixelsize[(*cinfo).out_color_space as usize];
            if (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                if 0 != jsimd_can_ycc_rgb() {
                    (*cconvert).pub_0.color_convert = Some(
                        jsimd_ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        ycc_rgb_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    );
                    build_ycc_rgb_table(cinfo);
                }
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                if rgb_red[(*cinfo).out_color_space as usize] == 0i32
                    && rgb_green[(*cinfo).out_color_space as usize] == 1i32
                    && rgb_blue[(*cinfo).out_color_space as usize] == 2i32
                    && rgb_pixelsize[(*cinfo).out_color_space as usize] == 3i32
                {
                    (*cconvert).pub_0.color_convert = Some(
                        null_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                }
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        16 => {
            (*cinfo).out_color_components = 3i32;
            if (*cinfo).dither_mode as c_uint == JDITHER_NONE as c_int as c_uint {
                if (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                    if 0 != jsimd_can_ycc_rgb565() {
                        (*cconvert).pub_0.color_convert = Some(
                            jsimd_ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: JSAMPIMAGE,
                                    _: JDIMENSION,
                                    _: JSAMPARRAY,
                                    _: c_int,
                                ) -> (),
                        )
                    } else {
                        (*cconvert).pub_0.color_convert = Some(
                            ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    _: j_decompress_ptr,
                                    _: JSAMPIMAGE,
                                    _: JDIMENSION,
                                    _: JSAMPARRAY,
                                    _: c_int,
                                ) -> (),
                        );
                        build_ycc_rgb_table(cinfo);
                    }
                } else if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                    (*cconvert).pub_0.color_convert = Some(
                        gray_rgb565_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb565_convert
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: JSAMPIMAGE,
                                _: JDIMENSION,
                                _: JSAMPARRAY,
                                _: c_int,
                            ) -> (),
                    )
                } else {
                    (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    ycc_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_RGB as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb565D_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 => {
            (*cinfo).out_color_components = 4i32;
            if (*cinfo).jpeg_color_space as c_uint == JCS_YCCK as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    ycck_cmyk_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).out_color_space as c_uint == (*cinfo).jpeg_color_space as c_uint {
                (*cinfo).out_color_components = (*cinfo).num_components;
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    if 0 != (*cinfo).quantize_colors {
        (*cinfo).output_components = 1i32
    } else {
        (*cinfo).output_components = (*cinfo).out_color_components
    };
}
