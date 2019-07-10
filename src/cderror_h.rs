use ::libc;
use libc::c_uint;
pub const JTRC_TGA_MAPPED: C2RustUnnamed_91 = 1040;
pub const JTRC_TGA_GRAY: C2RustUnnamed_91 = 1039;
pub const JTRC_TGA: C2RustUnnamed_91 = 1038;
pub const JERR_TGA_COLORSPACE: C2RustUnnamed_91 = 1037;
pub const JERR_TGA_BADPARMS: C2RustUnnamed_91 = 1036;
/* PPM_SUPPORTED */

/* RLE_SUPPORTED */
pub const JERR_TGA_BADCMAP: C2RustUnnamed_91 = 1035;
pub const JWRN_GIF_NOMOREDATA: C2RustUnnamed_91 = 1026;
pub const JWRN_GIF_ENDCODE: C2RustUnnamed_91 = 1025;
pub const JWRN_GIF_CHAR: C2RustUnnamed_91 = 1024;
pub const JWRN_GIF_BADDATA: C2RustUnnamed_91 = 1023;
pub const JTRC_GIF_NONSQUARE: C2RustUnnamed_91 = 1022;
pub const JTRC_GIF_EXTENSION: C2RustUnnamed_91 = 1021;
pub const JTRC_GIF_BADVERSION: C2RustUnnamed_91 = 1020;
pub const JTRC_GIF: C2RustUnnamed_91 = 1019;
pub const JERR_GIF_NOT: C2RustUnnamed_91 = 1018;
pub const JERR_GIF_IMAGENOTFOUND: C2RustUnnamed_91 = 1017;
pub const JERR_GIF_COLORSPACE: C2RustUnnamed_91 = 1016;
pub const JERR_GIF_CODESIZE: C2RustUnnamed_91 = 1015;
/* BMP_SUPPORTED */
pub const JERR_GIF_BUG: C2RustUnnamed_91 = 1014;
/*
 * cderror.h
 *
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * Modified 2009-2017 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file defines the error and message codes for the cjpeg/djpeg
 * applications.  These strings are not needed as part of the JPEG library
 * proper.
 * Edit this file to add new codes, or to translate the message strings to
 * some other language.
 */

/*
 * To define the enum list of message codes, include this file without
 * defining macro JMESSAGE.  To create a message string table, include it
 * again with a suitable JMESSAGE definition (see jerror.c for an example).
 */

/* First time through, define the enum list */

/* CDERROR_H */

/* JMESSAGE */
pub type C2RustUnnamed_91 = c_uint;
pub const JMSG_LASTADDONCODE: C2RustUnnamed_91 = 1028;
/* JMAKE_ENUM_LIST */

/* Must be first entry! */
pub const JMSG_FIRSTADDONCODE: C2RustUnnamed_91 = 1000;
pub const JERR_UNSUPPORTED_FORMAT: C2RustUnnamed_91 = 1027;
pub const JERR_UNKNOWN_FORMAT: C2RustUnnamed_91 = 1026;
pub const JERR_UNGETC_FAILED: C2RustUnnamed_91 = 1025;
pub const JERR_TOO_MANY_COLORS: C2RustUnnamed_91 = 1024;
/* TARGA_SUPPORTED */
pub const JERR_BAD_CMAP_FILE: C2RustUnnamed_91 = 1023;
/* PPM_SUPPORTED */

/* RLE_SUPPORTED */
pub const JERR_TGA_NOTCOMP: C2RustUnnamed_91 = 1022;
pub const JTRC_PPM_TEXT: C2RustUnnamed_91 = 1021;
pub const JTRC_PPM: C2RustUnnamed_91 = 1020;
pub const JTRC_PGM_TEXT: C2RustUnnamed_91 = 1019;
pub const JTRC_PGM: C2RustUnnamed_91 = 1018;
pub const JERR_PPM_OUTOFRANGE: C2RustUnnamed_91 = 1017;
pub const JERR_PPM_NOT: C2RustUnnamed_91 = 1016;
pub const JERR_PPM_NONNUMERIC: C2RustUnnamed_91 = 1015;
/* BMP_SUPPORTED */

/* GIF_SUPPORTED */
pub const JERR_PPM_COLORSPACE: C2RustUnnamed_91 = 1014;
pub const JTRC_BMP_OS2_MAPPED: C2RustUnnamed_91 = 1013;
pub const JTRC_BMP_OS2: C2RustUnnamed_91 = 1012;
pub const JTRC_BMP_MAPPED: C2RustUnnamed_91 = 1011;
pub const JTRC_BMP: C2RustUnnamed_91 = 1010;
pub const JERR_BMP_OUTOFRANGE: C2RustUnnamed_91 = 1009;
pub const JERR_BMP_NOT: C2RustUnnamed_91 = 1008;
pub const JERR_BMP_EMPTY: C2RustUnnamed_91 = 1007;
pub const JERR_BMP_COMPRESSED: C2RustUnnamed_91 = 1006;
pub const JERR_BMP_COLORSPACE: C2RustUnnamed_91 = 1005;
pub const JERR_BMP_BADPLANES: C2RustUnnamed_91 = 1004;
pub const JERR_BMP_BADHEADER: C2RustUnnamed_91 = 1003;
pub const JERR_BMP_BADDEPTH: C2RustUnnamed_91 = 1002;
pub const JERR_BMP_BADCMAP: C2RustUnnamed_91 = 1001;
