pub type boolean = libc::c_int;
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

/* not HAVE_UNSIGNED_SHORT */

/* HAVE_UNSIGNED_SHORT */

/* INT16 must hold at least the values -32768..32767. */

/* X11/xmd.h correctly defines INT16 */

/* INT32 must hold at least signed 32-bit values.
 *
 * NOTE: The INT32 typedef dates back to libjpeg v5 (1994.)  Integers were
 * sometimes 16-bit back then (MS-DOS), which is why INT32 is typedef'd to
 * long.  It also wasn't common (or at least as common) in 1994 for INT32 to be
 * defined by platform headers.  Since then, however, INT32 is defined in
 * several other common places:
 *
 * Xmd.h (X11 header) typedefs INT32 to int on 64-bit platforms and long on
 * 32-bit platforms (i.e always a 32-bit signed type.)
 *
 * basetsd.h (Win32 header) typedefs INT32 to int (always a 32-bit signed type
 * on modern platforms.)
 *
 * qglobal.h (Qt header) typedefs INT32 to int (always a 32-bit signed type on
 * modern platforms.)
 *
 * This is a recipe for conflict, since "long" and "int" aren't always
 * compatible types.  Since the definition of INT32 has technically been part
 * of the libjpeg API for more than 20 years, we can't remove it, but we do not
 * use it internally any longer.  We instead define a separate type (JLONG)
 * for internal use, which ensures that internal behavior will always be the
 * same regardless of any external headers that may be included.
 */

/* X11/xmd.h correctly defines INT32 */

/* Microsoft defines it in basetsd.h */

/* MinGW is slightly different */

/* Qt defines it in qglobal.h */

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
pub const FALSE: libc::c_int = 0 as libc::c_int;
/* values of boolean */
pub const TRUE: libc::c_int = 1 as libc::c_int;
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

/* not HAVE_UNSIGNED_SHORT */

/* HAVE_UNSIGNED_SHORT */

/* INT16 must hold at least the values -32768..32767. */

/* X11/xmd.h correctly defines INT16 */

/* INT32 must hold at least signed 32-bit values.
 *
 * NOTE: The INT32 typedef dates back to libjpeg v5 (1994.)  Integers were
 * sometimes 16-bit back then (MS-DOS), which is why INT32 is typedef'd to
 * long.  It also wasn't common (or at least as common) in 1994 for INT32 to be
 * defined by platform headers.  Since then, however, INT32 is defined in
 * several other common places:
 *
 * Xmd.h (X11 header) typedefs INT32 to int on 64-bit platforms and long on
 * 32-bit platforms (i.e always a 32-bit signed type.)
 *
 * basetsd.h (Win32 header) typedefs INT32 to int (always a 32-bit signed type
 * on modern platforms.)
 *
 * qglobal.h (Qt header) typedefs INT32 to int (always a 32-bit signed type on
 * modern platforms.)
 *
 * This is a recipe for conflict, since "long" and "int" aren't always
 * compatible types.  Since the definition of INT32 has technically been part
 * of the libjpeg API for more than 20 years, we can't remove it, but we do not
 * use it internally any longer.  We instead define a separate type (JLONG)
 * for internal use, which ensures that internal behavior will always be the
 * same regardless of any external headers that may be included.
 */

/* X11/xmd.h correctly defines INT32 */

/* Microsoft defines it in basetsd.h */

/* MinGW is slightly different */

/* Qt defines it in qglobal.h */

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
pub const EXT_XBGR_PIXELSIZE: libc::c_int = 4 as libc::c_int;
pub const EXT_BGRX_PIXELSIZE: libc::c_int = 4 as libc::c_int;
pub const EXT_RGBX_PIXELSIZE: libc::c_int = 4 as libc::c_int;
pub const EXT_BGR_PIXELSIZE: libc::c_int = 3 as libc::c_int;
pub const EXT_RGB_PIXELSIZE: libc::c_int = 3 as libc::c_int;
pub const RGB_PIXELSIZE_5: libc::c_int = 3 as libc::c_int;
pub const EXT_XRGB_PIXELSIZE: libc::c_int = 4 as libc::c_int;
pub const RGB_BLUE: libc::c_int = 2 as libc::c_int;
pub const EXT_RGB_BLUE: libc::c_int = 2 as libc::c_int;
pub const EXT_BGR_BLUE: libc::c_int = 0 as libc::c_int;
pub const EXT_RGBX_BLUE: libc::c_int = 2 as libc::c_int;
pub const EXT_BGRX_BLUE: libc::c_int = 0 as libc::c_int;
pub const EXT_XBGR_BLUE: libc::c_int = 1 as libc::c_int;
pub const EXT_XRGB_BLUE: libc::c_int = 3 as libc::c_int;
pub const RGB_GREEN: libc::c_int = 1 as libc::c_int;
pub const EXT_RGB_GREEN: libc::c_int = 1 as libc::c_int;
pub const EXT_BGR_GREEN: libc::c_int = 1 as libc::c_int;
pub const EXT_RGBX_GREEN: libc::c_int = 1 as libc::c_int;
pub const EXT_BGRX_GREEN: libc::c_int = 1 as libc::c_int;
pub const EXT_XBGR_GREEN: libc::c_int = 2 as libc::c_int;
pub const EXT_XRGB_GREEN: libc::c_int = 2 as libc::c_int;
pub const RGB_RED: libc::c_int = 0 as libc::c_int;
pub const EXT_RGB_RED: libc::c_int = 0 as libc::c_int;
pub const EXT_BGR_RED: libc::c_int = 2 as libc::c_int;
pub const EXT_RGBX_RED: libc::c_int = 0 as libc::c_int;
pub const EXT_BGRX_RED: libc::c_int = 2 as libc::c_int;
pub const EXT_XBGR_RED: libc::c_int = 3 as libc::c_int;
pub const EXT_XRGB_RED: libc::c_int = 1 as libc::c_int;
pub const MAXJSAMPLE: libc::c_int = 255 as libc::c_int;
pub const CENTERJSAMPLE: libc::c_int = 128 as libc::c_int;
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
pub const RGB_RED_5: libc::c_int = 0 as libc::c_int;
/* Offset of Red in an RGB scanline element */
pub const RGB_GREEN_5: libc::c_int = 1 as libc::c_int;
/* Offset of Green */
pub const RGB_BLUE_5: libc::c_int = 2 as libc::c_int;
pub const JPEG_MAX_DIMENSION: libc::c_long = 65500 as libc::c_long;
pub const MAX_COMPONENTS: libc::c_int = 10 as libc::c_int;
pub type INT16 = libc::c_short;
pub const RGB_PIXELSIZE: libc::c_int = 3 as libc::c_int;
pub type JSAMPLE = libc::c_uchar;
pub type JCOEF = libc::c_short;
pub type JOCTET = libc::c_uchar;
pub type UINT8 = libc::c_uchar;
pub type UINT16 = libc::c_ushort;
pub type JDIMENSION = libc::c_uint;
