use libc;
use libc::c_int;
use libc::c_long;
use libc::c_uchar;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg-rs/mozjpeg-c/jmorecfg.h:25"]
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
    use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
    use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
    use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
    use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
    use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
    use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
    use crate::jmorecfg_h::RGB_PIXELSIZE;
    use libc::c_int;
    pub static mut rgb_pixelsize: [c_int; 17] = [
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
}
pub use crate::cderror_h::C2RustUnnamed_91;
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
pub use crate::cdjpeg::cjpeg_source_ptr;
pub use crate::cdjpeg::cjpeg_source_struct;
pub use crate::cmyk_h::rgb_to_cmyk;
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
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
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_PIXELSIZE;
pub use crate::jmorecfg_h::RGB_RED;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::fread;
pub use crate::stdlib::getc;
pub use crate::stdlib::EOF;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
pub use jmorecfg_h::rgb_pixelsize;
pub type ppm_source_ptr = *mut ppm_source_struct;
/* Private version of data source object */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppm_source_struct {
    pub pub_0: cjpeg_source_struct,
    pub iobuffer: *mut U_CHAR,
    pub pixrow: JSAMPROW,
    pub buffer_width: size_t,
    pub rescale: *mut JSAMPLE,
    pub maxval: c_uint,
}
/*
 * rdppm.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2009 by Bill Allombert, Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015-2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to read input images in PPM/PGM format.
 * The extended 2-byte-per-sample raw PPM/PGM formats are supported.
 * The PBMPLUS library is NOT required to compile this software
 * (but it is highly useful as a set of PPM image manipulation programs).
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume input from
 * an ordinary stdio stream.  They further assume that reading begins
 * at the start of the file; start_input may need work if the
 * user interface has already read some data (e.g., to determine that
 * the file is indeed PPM format).
 */
/* Portions of this code are based on the PBMPLUS library, which is:
**
** Copyright (C) 1988 by Jef Poskanzer.
**
** Permission to use, copy, modify, and distribute this software and its
** documentation for any purpose and without fee is hereby granted, provided
** that the above copyright notice appear in all copies and that both that
** copyright notice and this permission notice appear in supporting
** documentation.  This software is provided "as is" without express or
** implied warranty.
*/
/* Macros to deal with unsigned chars as efficiently as compiler allows */
pub type U_CHAR = c_uchar;
/* !HAVE_UNSIGNED_CHAR */
/* HAVE_UNSIGNED_CHAR */
static mut alpha_index: [c_int; 17] = [
    -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, 3i32, 3i32,
    0i32, 0i32, -1i32,
];
unsafe extern "C" fn pbm_getc(mut infile: *mut FILE) -> c_int {
    let mut ch: c_int = 0;
    ch = getc(infile);
    if ch == '#' as i32 {
        loop {
            ch = getc(infile);
            if !(ch != '\n' as i32 && ch != EOF) {
                break;
            }
        }
    }
    return ch;
}
unsafe extern "C" fn read_pbm_integer(
    mut cinfo: j_compress_ptr,
    mut infile: *mut FILE,
    mut maxval: c_uint,
) -> c_uint {
    let mut ch: c_int = 0;
    let mut val: c_uint = 0;
    loop {
        ch = pbm_getc(infile);
        if ch == EOF {
            (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
            break;
        }
    }
    if ch < '0' as i32 || ch > '9' as i32 {
        (*(*cinfo).err).msg_code = JERR_PPM_NONNUMERIC as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    val = (ch - '0' as i32) as c_uint;
    loop {
        ch = pbm_getc(infile);
        if !(ch >= '0' as i32 && ch <= '9' as i32) {
            break;
        }
        val = val.wrapping_mul(10i32 as c_uint);
        val = val.wrapping_add((ch - '0' as i32) as c_uint)
    }
    if val > maxval {
        (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return val;
}
/*
 * Read one row of pixels.
 *
 * We provide several different versions depending on input file format.
 * In all cases, input is scaled to the size of JSAMPLE.
 *
 * A really fast path is provided for reading byte/sample raw files with
 * maxval = MAXJSAMPLE, which is the normal case for 8-bit data.
 */
unsafe extern "C" fn get_text_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    ptr = *(*source).pub_0.buffer.offset(0isize);
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_text_gray_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    ptr = *(*source).pub_0.buffer.offset(0isize);
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let ref mut fresh2 = *ptr.offset(gindex as isize);
                let ref mut fresh1 = *ptr.offset(bindex as isize);
                *fresh1 = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *fresh2 = *fresh1;
                *ptr.offset(rindex as isize) = *fresh2;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let ref mut fresh4 = *ptr.offset(gindex as isize);
                let ref mut fresh3 = *ptr.offset(bindex as isize);
                *fresh3 = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *fresh4 = *fresh3;
                *ptr.offset(rindex as isize) = *fresh4;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let ref mut fresh6 = *ptr.offset(gindex as isize);
            let ref mut fresh5 = *ptr.offset(bindex as isize);
            *fresh5 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *fresh6 = *fresh5;
            *ptr.offset(rindex as isize) = *fresh6;
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let ref mut fresh8 = *ptr.offset(gindex as isize);
            let ref mut fresh7 = *ptr.offset(bindex as isize);
            *fresh7 = *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *fresh8 = *fresh7;
            *ptr.offset(rindex as isize) = *fresh8;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_text_gray_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    ptr = *(*source).pub_0.buffer.offset(0isize);
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let mut gray: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            rgb_to_cmyk(
                gray,
                gray,
                gray,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let mut gray_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            rgb_to_cmyk(
                gray_0,
                gray_0,
                gray_0,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_text_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    ptr = *(*source).pub_0.buffer.offset(0isize);
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                *ptr.offset(rindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(gindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(bindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                *ptr.offset(rindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(gindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                *ptr.offset(bindex as isize) = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            *ptr.offset(rindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(gindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(bindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            *ptr.offset(rindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(gindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            *ptr.offset(bindex as isize) =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_text_rgb_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    ptr = *(*source).pub_0.buffer.offset(0isize);
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let mut r: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            let mut g: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            let mut b: JSAMPLE = read_pbm_integer(cinfo, infile, maxval) as JSAMPLE;
            rgb_to_cmyk(
                r,
                g,
                b,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let mut r_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let mut g_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            let mut b_0: JSAMPLE =
                *rescale.offset(read_pbm_integer(cinfo, infile, maxval) as isize);
            rgb_to_cmyk(
                r_0,
                g_0,
                b_0,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_scaled_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ptr = *(*source).pub_0.buffer.offset(0isize);
    bufferptr = (*source).iobuffer;
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let fresh10 = ptr;
        ptr = ptr.offset(1);
        let fresh9 = bufferptr;
        bufferptr = bufferptr.offset(1);
        *fresh10 = *rescale.offset(*fresh9 as c_int as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_gray_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ptr = *(*source).pub_0.buffer.offset(0isize);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let ref mut fresh13 = *ptr.offset(gindex as isize);
                let ref mut fresh12 = *ptr.offset(bindex as isize);
                let fresh11 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *fresh12 = *fresh11;
                *fresh13 = *fresh12;
                *ptr.offset(rindex as isize) = *fresh13;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let ref mut fresh16 = *ptr.offset(gindex as isize);
                let ref mut fresh15 = *ptr.offset(bindex as isize);
                let fresh14 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *fresh15 = *fresh14;
                *fresh16 = *fresh15;
                *ptr.offset(rindex as isize) = *fresh16;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let ref mut fresh19 = *ptr.offset(gindex as isize);
            let ref mut fresh18 = *ptr.offset(bindex as isize);
            let fresh17 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *fresh18 = *rescale.offset(*fresh17 as c_int as isize);
            *fresh19 = *fresh18;
            *ptr.offset(rindex as isize) = *fresh19;
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let ref mut fresh22 = *ptr.offset(gindex as isize);
            let ref mut fresh21 = *ptr.offset(bindex as isize);
            let fresh20 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *fresh21 = *rescale.offset(*fresh20 as c_int as isize);
            *fresh22 = *fresh21;
            *ptr.offset(rindex as isize) = *fresh22;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_gray_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ptr = *(*source).pub_0.buffer.offset(0isize);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh23 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut gray: JSAMPLE = *fresh23;
            rgb_to_cmyk(
                gray,
                gray,
                gray,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh24 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut gray_0: JSAMPLE = *rescale.offset(*fresh24 as c_int as isize);
            rgb_to_cmyk(
                gray_0,
                gray_0,
                gray_0,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
    let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
    let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
    let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
    let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ptr = *(*source).pub_0.buffer.offset(0isize);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh25 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(rindex as isize) = *fresh25;
                let fresh26 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(gindex as isize) = *fresh26;
                let fresh27 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(bindex as isize) = *fresh27;
                *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh28 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(rindex as isize) = *fresh28;
                let fresh29 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(gindex as isize) = *fresh29;
                let fresh30 = bufferptr;
                bufferptr = bufferptr.offset(1);
                *ptr.offset(bindex as isize) = *fresh30;
                ptr = ptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    } else if aindex >= 0i32 {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh31 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(rindex as isize) = *rescale.offset(*fresh31 as c_int as isize);
            let fresh32 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(gindex as isize) = *rescale.offset(*fresh32 as c_int as isize);
            let fresh33 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(bindex as isize) = *rescale.offset(*fresh33 as c_int as isize);
            *ptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh34 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(rindex as isize) = *rescale.offset(*fresh34 as c_int as isize);
            let fresh35 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(gindex as isize) = *rescale.offset(*fresh35 as c_int as isize);
            let fresh36 = bufferptr;
            bufferptr = bufferptr.offset(1);
            *ptr.offset(bindex as isize) = *rescale.offset(*fresh36 as c_int as isize);
            ptr = ptr.offset(ps as isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_rgb_cmyk_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ptr = *(*source).pub_0.buffer.offset(0isize);
    bufferptr = (*source).iobuffer;
    if maxval == MAXJSAMPLE as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh37 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut r: JSAMPLE = *fresh37;
            let fresh38 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut g: JSAMPLE = *fresh38;
            let fresh39 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut b: JSAMPLE = *fresh39;
            rgb_to_cmyk(
                r,
                g,
                b,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh40 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut r_0: JSAMPLE = *rescale.offset(*fresh40 as c_int as isize);
            let fresh41 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut g_0: JSAMPLE = *rescale.offset(*fresh41 as c_int as isize);
            let fresh42 = bufferptr;
            bufferptr = bufferptr.offset(1);
            let mut b_0: JSAMPLE = *rescale.offset(*fresh42 as c_int as isize);
            rgb_to_cmyk(
                r_0,
                g_0,
                b_0,
                ptr,
                ptr.offset(1isize),
                ptr.offset(2isize),
                ptr.offset(3isize),
            );
            ptr = ptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_raw_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_word_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ptr = *(*source).pub_0.buffer.offset(0isize);
    bufferptr = (*source).iobuffer;
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let mut temp: c_uint = 0;
        let fresh43 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh43 as c_int) << 8i32) as c_uint;
        let fresh44 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh44 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh45 = ptr;
        ptr = ptr.offset(1);
        *fresh45 = *rescale.offset(temp as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_word_rgb_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut bufferptr: *mut U_CHAR = 0 as *mut U_CHAR;
    let mut rescale: *mut JSAMPLE = (*source).rescale;
    let mut col: JDIMENSION = 0;
    let mut maxval: c_uint = (*source).maxval;
    if !(fread(
        (*source).iobuffer as *mut c_void,
        1i32 as size_t,
        (*source).buffer_width,
        (*source).pub_0.input_file,
    ) == (*source).buffer_width)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ptr = *(*source).pub_0.buffer.offset(0isize);
    bufferptr = (*source).iobuffer;
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        let mut temp: c_uint = 0;
        let fresh46 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh46 as c_int) << 8i32) as c_uint;
        let fresh47 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh47 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh48 = ptr;
        ptr = ptr.offset(1);
        *fresh48 = *rescale.offset(temp as isize);
        let fresh49 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh49 as c_int) << 8i32) as c_uint;
        let fresh50 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh50 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh51 = ptr;
        ptr = ptr.offset(1);
        *fresh51 = *rescale.offset(temp as isize);
        let fresh52 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp = ((*fresh52 as c_int) << 8i32) as c_uint;
        let fresh53 = bufferptr;
        bufferptr = bufferptr.offset(1);
        temp |= *fresh53 as c_int as c_uint;
        if temp > maxval {
            (*(*cinfo).err).msg_code = JERR_PPM_OUTOFRANGE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh54 = ptr;
        ptr = ptr.offset(1);
        *fresh54 = *rescale.offset(temp as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
/*
 * Read the file header; return image size and component count.
 */
unsafe extern "C" fn start_input_ppm(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut source: ppm_source_ptr = sinfo as ppm_source_ptr;
    let mut c: c_int = 0;
    let mut w: c_uint = 0;
    let mut h: c_uint = 0;
    let mut maxval: c_uint = 0;
    let mut need_iobuffer: boolean = 0;
    let mut use_raw_buffer: boolean = 0;
    let mut need_rescale: boolean = 0;
    if getc((*source).pub_0.input_file) != 'P' as i32 {
        (*(*cinfo).err).msg_code = JERR_PPM_NOT as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    c = getc((*source).pub_0.input_file);
    let mut current_block_3: u64;
    match c {
        50 => {
            /* it's a text-format PPM file */
            current_block_3 = 3943135426467512739;
        }
        51 => {
            current_block_3 = 3943135426467512739;
        }
        53 => {
            current_block_3 = 1115462442902857658;
        }
        54 => {
            current_block_3 = 13513818773234778473;
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_PPM_NOT as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            current_block_3 = 13513818773234778473;
        }
    }
    match current_block_3 {
        3943135426467512739 => {
            /* it's a raw-format PGM file */
            current_block_3 = 1115462442902857658;
        }
        _ => {}
    }
    match current_block_3 {
        1115462442902857658 => {}
        _ => {}
    }
    w = read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535i32 as c_uint);
    h = read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535i32 as c_uint);
    maxval = read_pbm_integer(cinfo, (*source).pub_0.input_file, 65535i32 as c_uint);
    if w <= 0i32 as c_uint || h <= 0i32 as c_uint || maxval <= 0i32 as c_uint {
        (*(*cinfo).err).msg_code = JERR_PPM_NOT as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cinfo).data_precision = BITS_IN_JSAMPLE;
    (*cinfo).image_width = w;
    (*cinfo).image_height = h;
    (*source).maxval = maxval;
    need_iobuffer = TRUE;
    use_raw_buffer = FALSE;
    need_rescale = TRUE;
    match c {
        50 => {
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
                (*cinfo).in_color_space = JCS_GRAYSCALE
            }
            (*(*cinfo).err).msg_code = JTRC_PGM_TEXT as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1usize] = h as c_int;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_gray_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            need_iobuffer = FALSE
        }
        51 => {
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
                (*cinfo).in_color_space = JCS_EXT_RGB
            }
            (*(*cinfo).err).msg_code = JTRC_PPM_TEXT as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1usize] = h as c_int;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_text_rgb_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            need_iobuffer = FALSE
        }
        53 => {
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
                (*cinfo).in_color_space = JCS_GRAYSCALE
            }
            (*(*cinfo).err).msg_code = JTRC_PGM as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1usize] = h as c_int;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if maxval > 255i32 as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_word_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if maxval == MAXJSAMPLE as c_uint
                && ::std::mem::size_of::<JSAMPLE>() as c_ulong
                    == ::std::mem::size_of::<U_CHAR>() as c_ulong
                && (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_raw_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                );
                use_raw_buffer = TRUE;
                need_rescale = FALSE
            } else if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_scaled_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_gray_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_gray_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        54 => {
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
                (*cinfo).in_color_space = JCS_EXT_RGB
            }
            (*(*cinfo).err).msg_code = JTRC_PPM as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = w as c_int;
            (*(*cinfo).err).msg_parm.i[1usize] = h as c_int;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
            if maxval > 255i32 as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_word_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if maxval == MAXJSAMPLE as c_uint
                && ::std::mem::size_of::<JSAMPLE>() as c_ulong
                    == ::std::mem::size_of::<U_CHAR>() as c_ulong
                && ((*cinfo).in_color_space as c_uint == JCS_EXT_RGB as c_int as c_uint
                    || (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint)
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_raw_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                );
                use_raw_buffer = TRUE;
                need_rescale = FALSE
            } else if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*source).pub_0.get_pixel_rows = Some(
                    get_rgb_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*source).pub_0.get_pixel_rows = Some(
                    get_rgb_cmyk_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {}
    }
    if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
        || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
            && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
    {
        (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
    } else if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
        (*cinfo).input_components = 1i32
    } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
        (*cinfo).input_components = 4i32
    }
    if 0 != need_iobuffer {
        if c == '6' as i32 {
            (*source).buffer_width = (w as size_t).wrapping_mul(3i32 as c_ulong).wrapping_mul(
                if maxval <= 255i32 as c_uint {
                    ::std::mem::size_of::<U_CHAR>() as c_ulong
                } else {
                    (2i32 as c_ulong).wrapping_mul(::std::mem::size_of::<U_CHAR>() as c_ulong)
                },
            )
        } else {
            (*source).buffer_width = (w as size_t).wrapping_mul(
                if maxval <= 255i32 as c_uint {
                    ::std::mem::size_of::<U_CHAR>() as c_ulong
                } else {
                    (2i32 as c_ulong).wrapping_mul(::std::mem::size_of::<U_CHAR>() as c_ulong)
                },
            )
        }
        (*source).iobuffer = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*source).buffer_width,
        ) as *mut U_CHAR
    }
    if 0 != use_raw_buffer {
        (*source).pixrow = (*source).iobuffer as JSAMPROW;
        (*source).pub_0.buffer = &mut (*source).pixrow;
        (*source).pub_0.buffer_height = 1i32 as JDIMENSION
    } else {
        (*source).pub_0.buffer = (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            w.wrapping_mul((*cinfo).input_components as c_uint),
            1i32 as JDIMENSION,
        );
        (*source).pub_0.buffer_height = 1i32 as JDIMENSION
    }
    if 0 != need_rescale {
        let mut val: c_long = 0;
        let mut half_maxval: c_long = 0;
        (*source).rescale = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((maxval as c_long + 1i64) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
        ) as *mut JSAMPLE;
        half_maxval = maxval.wrapping_div(2i32 as c_uint) as c_long;
        val = 0i32 as c_long;
        while val <= maxval as c_long {
            *(*source).rescale.offset(val as isize) =
                ((val * MAXJSAMPLE as c_long + half_maxval) / maxval as c_long) as JSAMPLE;
            val += 1
        }
    };
}
/*
 * Finish up at the end of the file.
 */
unsafe extern "C" fn finish_input_ppm(mut _cinfo: j_compress_ptr, mut _sinfo: cjpeg_source_ptr) {}
/* no work */
/*
 * The module selection routine for PPM format input.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_read_ppm(mut cinfo: j_compress_ptr) -> cjpeg_source_ptr {
    let mut source: ppm_source_ptr = 0 as *mut ppm_source_struct;
    source = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<ppm_source_struct>() as c_ulong,
    ) as ppm_source_ptr;
    (*source).pub_0.start_input =
        Some(start_input_ppm as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> ());
    (*source).pub_0.finish_input = Some(
        finish_input_ppm as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> (),
    );
    return source as cjpeg_source_ptr;
}
