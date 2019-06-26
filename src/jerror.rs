/*
 * jerror.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * Modified 1997-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2014, 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file defines the error and message codes for the JPEG library.
 * Edit this file to add new codes, or to translate the message strings to
 * some other language.
 * A set of error-reporting macros are defined too.  Some applications using
 * the JPEG library may wish to include this file to get the error codes
 * and/or the macros.
 */
/*
 * To define the enum list of message codes, include this file without
 * defining macro JMESSAGE.  To create a message string table, include it
 * again with a suitable JMESSAGE definition (see jerror.c for an example).
 */
/* First time through, define the enum list */
/* JERROR_H */
/* JMESSAGE */
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
pub type C2RustUnnamed_4 = c_uint;
pub use crate::jpeglib_h::C2RustUnnamed_3;
use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
pub const JERR_BAD_STRUCT_SIZE: C2RustUnnamed_4 = 21;
pub const JERR_BAD_LIB_VERSION: C2RustUnnamed_4 = 12;
pub const JERR_CANT_SUSPEND: C2RustUnnamed_4 = 24;
pub const JERR_BAD_STATE: C2RustUnnamed_4 = 20;
pub const JERR_TOO_LITTLE_DATA: C2RustUnnamed_4 = 67;
pub const JMSG_LASTMSGCODE: C2RustUnnamed_4 = 129;
pub const JWRN_BOGUS_ICC: C2RustUnnamed_4 = 128;
pub const JERR_UNSUPPORTED_SUSPEND: C2RustUnnamed_4 = 127;
pub const JERR_BAD_PARAM_VALUE: C2RustUnnamed_4 = 126;
pub const JERR_BAD_PARAM: C2RustUnnamed_4 = 125;
pub const JERR_BAD_CROP_SPEC: C2RustUnnamed_4 = 124;
pub const JWRN_TOO_MUCH_DATA: C2RustUnnamed_4 = 123;
pub const JWRN_NOT_SEQUENTIAL: C2RustUnnamed_4 = 122;
pub const JWRN_MUST_RESYNC: C2RustUnnamed_4 = 121;
pub const JWRN_JPEG_EOF: C2RustUnnamed_4 = 120;
pub const JWRN_JFIF_MAJOR: C2RustUnnamed_4 = 119;
pub const JWRN_HUFF_BAD_CODE: C2RustUnnamed_4 = 118;
pub const JWRN_HIT_MARKER: C2RustUnnamed_4 = 117;
pub const JWRN_EXTRANEOUS_DATA: C2RustUnnamed_4 = 116;
pub const JWRN_BOGUS_PROGRESSION: C2RustUnnamed_4 = 115;
pub const JWRN_ADOBE_XFORM: C2RustUnnamed_4 = 114;
pub const JTRC_XMS_OPEN: C2RustUnnamed_4 = 113;
pub const JTRC_XMS_CLOSE: C2RustUnnamed_4 = 112;
pub const JTRC_UNKNOWN_IDS: C2RustUnnamed_4 = 111;
pub const JTRC_THUMB_RGB: C2RustUnnamed_4 = 110;
pub const JTRC_THUMB_PALETTE: C2RustUnnamed_4 = 109;
pub const JTRC_THUMB_JPEG: C2RustUnnamed_4 = 108;
pub const JTRC_TFILE_OPEN: C2RustUnnamed_4 = 107;
pub const JTRC_TFILE_CLOSE: C2RustUnnamed_4 = 106;
pub const JTRC_SOS_PARAMS: C2RustUnnamed_4 = 105;
pub const JTRC_SOS_COMPONENT: C2RustUnnamed_4 = 104;
pub const JTRC_SOS: C2RustUnnamed_4 = 103;
pub const JTRC_SOI: C2RustUnnamed_4 = 102;
pub const JTRC_SOF_COMPONENT: C2RustUnnamed_4 = 101;
pub const JTRC_SOF: C2RustUnnamed_4 = 100;
pub const JTRC_SMOOTH_NOTIMPL: C2RustUnnamed_4 = 99;
pub const JTRC_RST: C2RustUnnamed_4 = 98;
pub const JTRC_RECOVERY_ACTION: C2RustUnnamed_4 = 97;
pub const JTRC_QUANT_SELECTED: C2RustUnnamed_4 = 96;
pub const JTRC_QUANT_NCOLORS: C2RustUnnamed_4 = 95;
pub const JTRC_QUANT_3_NCOLORS: C2RustUnnamed_4 = 94;
pub const JTRC_QUANTVALS: C2RustUnnamed_4 = 93;
pub const JTRC_PARMLESS_MARKER: C2RustUnnamed_4 = 92;
pub const JTRC_MISC_MARKER: C2RustUnnamed_4 = 91;
pub const JTRC_JFIF_THUMBNAIL: C2RustUnnamed_4 = 90;
pub const JTRC_JFIF_EXTENSION: C2RustUnnamed_4 = 89;
pub const JTRC_JFIF_BADTHUMBNAILSIZE: C2RustUnnamed_4 = 88;
pub const JTRC_JFIF: C2RustUnnamed_4 = 87;
pub const JTRC_HUFFBITS: C2RustUnnamed_4 = 86;
pub const JTRC_EOI: C2RustUnnamed_4 = 85;
pub const JTRC_EMS_OPEN: C2RustUnnamed_4 = 84;
pub const JTRC_EMS_CLOSE: C2RustUnnamed_4 = 83;
pub const JTRC_DRI: C2RustUnnamed_4 = 82;
pub const JTRC_DQT: C2RustUnnamed_4 = 81;
pub const JTRC_DHT: C2RustUnnamed_4 = 80;
pub const JTRC_DAC: C2RustUnnamed_4 = 79;
pub const JTRC_APP14: C2RustUnnamed_4 = 78;
pub const JTRC_APP0: C2RustUnnamed_4 = 77;
pub const JTRC_ADOBE: C2RustUnnamed_4 = 76;
pub const JTRC_16BIT_TABLES: C2RustUnnamed_4 = 75;
pub const JMSG_VERSION: C2RustUnnamed_4 = 74;
pub const JMSG_COPYRIGHT: C2RustUnnamed_4 = 73;
pub const JERR_XMS_WRITE: C2RustUnnamed_4 = 72;
pub const JERR_XMS_READ: C2RustUnnamed_4 = 71;
pub const JERR_WIDTH_OVERFLOW: C2RustUnnamed_4 = 70;
pub const JERR_VIRTUAL_BUG: C2RustUnnamed_4 = 69;
pub const JERR_UNKNOWN_MARKER: C2RustUnnamed_4 = 68;
pub const JERR_TFILE_WRITE: C2RustUnnamed_4 = 66;
pub const JERR_TFILE_SEEK: C2RustUnnamed_4 = 65;
pub const JERR_TFILE_READ: C2RustUnnamed_4 = 64;
pub const JERR_TFILE_CREATE: C2RustUnnamed_4 = 63;
pub const JERR_SOS_NO_SOF: C2RustUnnamed_4 = 62;
pub const JERR_SOI_DUPLICATE: C2RustUnnamed_4 = 61;
pub const JERR_SOF_UNSUPPORTED: C2RustUnnamed_4 = 60;
pub const JERR_SOF_NO_SOS: C2RustUnnamed_4 = 59;
pub const JERR_SOF_DUPLICATE: C2RustUnnamed_4 = 58;
pub const JERR_QUANT_MANY_COLORS: C2RustUnnamed_4 = 57;
pub const JERR_QUANT_FEW_COLORS: C2RustUnnamed_4 = 56;
pub const JERR_QUANT_COMPONENTS: C2RustUnnamed_4 = 55;
pub const JERR_OUT_OF_MEMORY: C2RustUnnamed_4 = 54;
pub const JERR_NO_SOI: C2RustUnnamed_4 = 53;
pub const JERR_NO_QUANT_TABLE: C2RustUnnamed_4 = 52;
pub const JERR_NO_IMAGE: C2RustUnnamed_4 = 51;
pub const JERR_NO_HUFF_TABLE: C2RustUnnamed_4 = 50;
pub const JERR_NO_BACKING_STORE: C2RustUnnamed_4 = 49;
pub const JERR_NOT_COMPILED: C2RustUnnamed_4 = 48;
pub const JERR_NOTIMPL: C2RustUnnamed_4 = 47;
pub const JERR_MODE_CHANGE: C2RustUnnamed_4 = 46;
pub const JERR_MISSING_DATA: C2RustUnnamed_4 = 45;
pub const JERR_MISMATCHED_QUANT_TABLE: C2RustUnnamed_4 = 44;
pub const JERR_INPUT_EOF: C2RustUnnamed_4 = 43;
pub const JERR_INPUT_EMPTY: C2RustUnnamed_4 = 42;
pub const JERR_IMAGE_TOO_BIG: C2RustUnnamed_4 = 41;
pub const JERR_HUFF_MISSING_CODE: C2RustUnnamed_4 = 40;
pub const JERR_HUFF_CLEN_OVERFLOW: C2RustUnnamed_4 = 39;
pub const JERR_FRACT_SAMPLE_NOTIMPL: C2RustUnnamed_4 = 38;
pub const JERR_FILE_WRITE: C2RustUnnamed_4 = 37;
pub const JERR_FILE_READ: C2RustUnnamed_4 = 36;
pub const JERR_EOI_EXPECTED: C2RustUnnamed_4 = 35;
pub const JERR_EMS_WRITE: C2RustUnnamed_4 = 34;
pub const JERR_EMS_READ: C2RustUnnamed_4 = 33;
pub const JERR_EMPTY_IMAGE: C2RustUnnamed_4 = 32;
pub const JERR_DQT_INDEX: C2RustUnnamed_4 = 31;
pub const JERR_DHT_INDEX: C2RustUnnamed_4 = 30;
pub const JERR_DAC_VALUE: C2RustUnnamed_4 = 29;
pub const JERR_DAC_INDEX: C2RustUnnamed_4 = 28;
pub const JERR_CONVERSION_NOTIMPL: C2RustUnnamed_4 = 27;
pub const JERR_COMPONENT_COUNT: C2RustUnnamed_4 = 26;
pub const JERR_CCIR601_NOTIMPL: C2RustUnnamed_4 = 25;
pub const JERR_BUFFER_SIZE: C2RustUnnamed_4 = 23;
pub const JERR_BAD_VIRTUAL_ACCESS: C2RustUnnamed_4 = 22;
pub const JERR_BAD_SCAN_SCRIPT: C2RustUnnamed_4 = 19;
pub const JERR_BAD_SAMPLING: C2RustUnnamed_4 = 18;
pub const JERR_BAD_PROG_SCRIPT: C2RustUnnamed_4 = 17;
pub const JERR_BAD_PROGRESSION: C2RustUnnamed_4 = 16;
pub const JERR_BAD_PRECISION: C2RustUnnamed_4 = 15;
pub const JERR_BAD_POOL_ID: C2RustUnnamed_4 = 14;
pub const JERR_BAD_MCU_SIZE: C2RustUnnamed_4 = 13;
pub const JERR_BAD_LENGTH: C2RustUnnamed_4 = 11;
pub const JERR_BAD_J_COLORSPACE: C2RustUnnamed_4 = 10;
pub const JERR_BAD_IN_COLORSPACE: C2RustUnnamed_4 = 9;
pub const JERR_BAD_HUFF_TABLE: C2RustUnnamed_4 = 8;
pub const JERR_BAD_DCTSIZE: C2RustUnnamed_4 = 7;
pub const JERR_BAD_DCT_COEF: C2RustUnnamed_4 = 6;
pub const JERR_BAD_COMPONENT_ID: C2RustUnnamed_4 = 5;
pub const JERR_BAD_BUFFER_MODE: C2RustUnnamed_4 = 4;
pub const JERR_BAD_ALLOC_CHUNK: C2RustUnnamed_4 = 3;
pub const JERR_BAD_ALIGN_TYPE: C2RustUnnamed_4 = 2;
/* For maintenance convenience, list is alphabetical by message code name */
pub const JERR_ARITH_NOTIMPL: C2RustUnnamed_4 = 1;
/* JMAKE_ENUM_LIST */
/* Must be first entry! */
pub const JMSG_NOMESSAGE: C2RustUnnamed_4 = 0;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jpeglib_h::jpeg_destroy;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::exit;
use crate::stdlib::fprintf;
use crate::stdlib::sprintf;
use crate::stdlib::stderr;
pub use crate::stdlib::EXIT_FAILURE;
/*
 * jerror.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains simple error-reporting and trace-message routines.
 * These are suitable for Unix-like systems and others where writing to
 * stderr is the right thing to do.  Many applications will want to replace
 * some or all of these routines.
 *
 * If you define USE_WINDOWS_MESSAGEBOX in jconfig.h or in the makefile,
 * you get a Windows-specific hack to display error messages in a dialog box.
 * It ain't much, but it beats dropping error messages into the bit bucket,
 * which is what happens to output to stderr under most Windows C compilers.
 *
 * These routines are used by both the compression and decompression code.
 */
/* this is not a core library module, so it doesn't define JPEG_INTERNALS */
/* define exit() codes if not provided */
/*
 * Create the message string table.
 * We do this from the master message list in jerror.h by re-reading
 * jerror.h with a suitable definition for macro JMESSAGE.
 * The message table is made an external symbol just in case any applications
 * want to refer to it directly.
 */
#[no_mangle]
pub static mut jpeg_std_message_table: [*const c_char; 130] = [
    b"Bogus message code %d\x00" as *const u8 as *const c_char,
    b"Sorry, arithmetic coding is not implemented\x00" as *const u8 as *const c_char,
    b"ALIGN_TYPE is wrong, please fix\x00" as *const u8 as *const c_char,
    b"MAX_ALLOC_CHUNK is wrong, please fix\x00" as *const u8 as *const c_char,
    b"Bogus buffer control mode\x00" as *const u8 as *const c_char,
    b"Invalid component ID %d in SOS\x00" as *const u8 as *const c_char,
    b"DCT coefficient out of range\x00" as *const u8 as *const c_char,
    b"IDCT output block size %d not supported\x00" as *const u8 as *const c_char,
    b"Bogus Huffman table definition\x00" as *const u8 as *const c_char,
    b"Bogus input colorspace\x00" as *const u8 as *const c_char,
    b"Bogus JPEG colorspace\x00" as *const u8 as *const c_char,
    b"Bogus marker length\x00" as *const u8 as *const c_char,
    b"Wrong JPEG library version: library is %d, caller expects %d\x00" as *const u8
        as *const c_char,
    b"Sampling factors too large for interleaved scan\x00" as *const u8 as *const c_char,
    b"Invalid memory pool code %d\x00" as *const u8 as *const c_char,
    b"Unsupported JPEG data precision %d\x00" as *const u8 as *const c_char,
    b"Invalid progressive parameters Ss=%d Se=%d Ah=%d Al=%d\x00" as *const u8 as *const c_char,
    b"Invalid progressive parameters at scan script entry %d\x00" as *const u8 as *const c_char,
    b"Bogus sampling factors\x00" as *const u8 as *const c_char,
    b"Invalid scan script at entry %d\x00" as *const u8 as *const c_char,
    b"Improper call to JPEG library in state %d\x00" as *const u8 as *const c_char,
    b"JPEG parameter struct mismatch: library thinks size is %u, caller expects %u\x00" as *const u8
        as *const c_char,
    b"Bogus virtual array access\x00" as *const u8 as *const c_char,
    b"Buffer passed to JPEG library is too small\x00" as *const u8 as *const c_char,
    b"Suspension not allowed here\x00" as *const u8 as *const c_char,
    b"CCIR601 sampling not implemented yet\x00" as *const u8 as *const c_char,
    b"Too many color components: %d, max %d\x00" as *const u8 as *const c_char,
    b"Unsupported color conversion request\x00" as *const u8 as *const c_char,
    b"Bogus DAC index %d\x00" as *const u8 as *const c_char,
    b"Bogus DAC value 0x%x\x00" as *const u8 as *const c_char,
    b"Bogus DHT index %d\x00" as *const u8 as *const c_char,
    b"Bogus DQT index %d\x00" as *const u8 as *const c_char,
    b"Empty JPEG image (DNL not supported)\x00" as *const u8 as *const c_char,
    b"Read from EMS failed\x00" as *const u8 as *const c_char,
    b"Write to EMS failed\x00" as *const u8 as *const c_char,
    b"Didn\'t expect more than one scan\x00" as *const u8 as *const c_char,
    b"Input file read error\x00" as *const u8 as *const c_char,
    b"Output file write error --- out of disk space?\x00" as *const u8 as *const c_char,
    b"Fractional sampling not implemented yet\x00" as *const u8 as *const c_char,
    b"Huffman code size table overflow\x00" as *const u8 as *const c_char,
    b"Missing Huffman code table entry\x00" as *const u8 as *const c_char,
    b"Maximum supported image dimension is %u pixels\x00" as *const u8 as *const c_char,
    b"Empty input file\x00" as *const u8 as *const c_char,
    b"Premature end of input file\x00" as *const u8 as *const c_char,
    b"Cannot transcode due to multiple use of quantization table %d\x00" as *const u8
        as *const c_char,
    b"Scan script does not transmit all data\x00" as *const u8 as *const c_char,
    b"Invalid color quantization mode change\x00" as *const u8 as *const c_char,
    b"Not implemented yet\x00" as *const u8 as *const c_char,
    b"Requested feature was omitted at compile time\x00" as *const u8 as *const c_char,
    b"Backing store not supported\x00" as *const u8 as *const c_char,
    b"Huffman table 0x%02x was not defined\x00" as *const u8 as *const c_char,
    b"JPEG datastream contains no image\x00" as *const u8 as *const c_char,
    b"Quantization table 0x%02x was not defined\x00" as *const u8 as *const c_char,
    b"Not a JPEG file: starts with 0x%02x 0x%02x\x00" as *const u8 as *const c_char,
    b"Insufficient memory (case %d)\x00" as *const u8 as *const c_char,
    b"Cannot quantize more than %d color components\x00" as *const u8 as *const c_char,
    b"Cannot quantize to fewer than %d colors\x00" as *const u8 as *const c_char,
    b"Cannot quantize to more than %d colors\x00" as *const u8 as *const c_char,
    b"Invalid JPEG file structure: two SOF markers\x00" as *const u8 as *const c_char,
    b"Invalid JPEG file structure: missing SOS marker\x00" as *const u8 as *const c_char,
    b"Unsupported JPEG process: SOF type 0x%02x\x00" as *const u8 as *const c_char,
    b"Invalid JPEG file structure: two SOI markers\x00" as *const u8 as *const c_char,
    b"Invalid JPEG file structure: SOS before SOF\x00" as *const u8 as *const c_char,
    b"Failed to create temporary file %s\x00" as *const u8 as *const c_char,
    b"Read failed on temporary file\x00" as *const u8 as *const c_char,
    b"Seek failed on temporary file\x00" as *const u8 as *const c_char,
    b"Write failed on temporary file --- out of disk space?\x00" as *const u8 as *const c_char,
    b"Application transferred too few scanlines\x00" as *const u8 as *const c_char,
    b"Unsupported marker type 0x%02x\x00" as *const u8 as *const c_char,
    b"Virtual array controller messed up\x00" as *const u8 as *const c_char,
    b"Image too wide for this implementation\x00" as *const u8 as *const c_char,
    b"Read from XMS failed\x00" as *const u8 as *const c_char,
    b"Write to XMS failed\x00" as *const u8 as *const c_char,
    b"Copyright (C) 1991-2018 The libjpeg-turbo Project and many others\x00" as *const u8
        as *const c_char,
    b"6b  27-Mar-1998\x00" as *const u8 as *const c_char,
    b"Caution: quantization tables are too coarse for baseline JPEG\x00" as *const u8
        as *const c_char,
    b"Adobe APP14 marker: version %d, flags 0x%04x 0x%04x, transform %d\x00" as *const u8
        as *const c_char,
    b"Unknown APP0 marker (not JFIF), length %u\x00" as *const u8 as *const c_char,
    b"Unknown APP14 marker (not Adobe), length %u\x00" as *const u8 as *const c_char,
    b"Define Arithmetic Table 0x%02x: 0x%02x\x00" as *const u8 as *const c_char,
    b"Define Huffman Table 0x%02x\x00" as *const u8 as *const c_char,
    b"Define Quantization Table %d  precision %d\x00" as *const u8 as *const c_char,
    b"Define Restart Interval %u\x00" as *const u8 as *const c_char,
    b"Freed EMS handle %u\x00" as *const u8 as *const c_char,
    b"Obtained EMS handle %u\x00" as *const u8 as *const c_char,
    b"End Of Image\x00" as *const u8 as *const c_char,
    b"        %3d %3d %3d %3d %3d %3d %3d %3d\x00" as *const u8 as *const c_char,
    b"JFIF APP0 marker: version %d.%02d, density %dx%d  %d\x00" as *const u8 as *const c_char,
    b"Warning: thumbnail image size does not match data length %u\x00" as *const u8
        as *const c_char,
    b"JFIF extension marker: type 0x%02x, length %u\x00" as *const u8 as *const c_char,
    b"    with %d x %d thumbnail image\x00" as *const u8 as *const c_char,
    b"Miscellaneous marker 0x%02x, length %u\x00" as *const u8 as *const c_char,
    b"Unexpected marker 0x%02x\x00" as *const u8 as *const c_char,
    b"        %4u %4u %4u %4u %4u %4u %4u %4u\x00" as *const u8 as *const c_char,
    b"Quantizing to %d = %d*%d*%d colors\x00" as *const u8 as *const c_char,
    b"Quantizing to %d colors\x00" as *const u8 as *const c_char,
    b"Selected %d colors for quantization\x00" as *const u8 as *const c_char,
    b"At marker 0x%02x, recovery action %d\x00" as *const u8 as *const c_char,
    b"RST%d\x00" as *const u8 as *const c_char,
    b"Smoothing not supported with nonstandard sampling ratios\x00" as *const u8 as *const c_char,
    b"Start Of Frame 0x%02x: width=%u, height=%u, components=%d\x00" as *const u8 as *const c_char,
    b"    Component %d: %dhx%dv q=%d\x00" as *const u8 as *const c_char,
    b"Start of Image\x00" as *const u8 as *const c_char,
    b"Start Of Scan: %d components\x00" as *const u8 as *const c_char,
    b"    Component %d: dc=%d ac=%d\x00" as *const u8 as *const c_char,
    b"  Ss=%d, Se=%d, Ah=%d, Al=%d\x00" as *const u8 as *const c_char,
    b"Closed temporary file %s\x00" as *const u8 as *const c_char,
    b"Opened temporary file %s\x00" as *const u8 as *const c_char,
    b"JFIF extension marker: JPEG-compressed thumbnail image, length %u\x00" as *const u8
        as *const c_char,
    b"JFIF extension marker: palette thumbnail image, length %u\x00" as *const u8 as *const c_char,
    b"JFIF extension marker: RGB thumbnail image, length %u\x00" as *const u8 as *const c_char,
    b"Unrecognized component IDs %d %d %d, assuming YCbCr\x00" as *const u8 as *const c_char,
    b"Freed XMS handle %u\x00" as *const u8 as *const c_char,
    b"Obtained XMS handle %u\x00" as *const u8 as *const c_char,
    b"Unknown Adobe color transform code %d\x00" as *const u8 as *const c_char,
    b"Inconsistent progression sequence for component %d coefficient %d\x00" as *const u8
        as *const c_char,
    b"Corrupt JPEG data: %u extraneous bytes before marker 0x%02x\x00" as *const u8
        as *const c_char,
    b"Corrupt JPEG data: premature end of data segment\x00" as *const u8 as *const c_char,
    b"Corrupt JPEG data: bad Huffman code\x00" as *const u8 as *const c_char,
    b"Warning: unknown JFIF revision number %d.%02d\x00" as *const u8 as *const c_char,
    b"Premature end of JPEG file\x00" as *const u8 as *const c_char,
    b"Corrupt JPEG data: found marker 0x%02x instead of RST%d\x00" as *const u8 as *const c_char,
    b"Invalid SOS parameters for sequential JPEG\x00" as *const u8 as *const c_char,
    b"Application transferred too many scanlines\x00" as *const u8 as *const c_char,
    b"Invalid crop request\x00" as *const u8 as *const c_char,
    b"Bogus parameter\x00" as *const u8 as *const c_char,
    b"Bogus parameter value\x00" as *const u8 as *const c_char,
    b"I/O suspension not supported in scan optimization\x00" as *const u8 as *const c_char,
    b"Corrupt JPEG data: bad ICC marker\x00" as *const u8 as *const c_char,
    NULL as *const c_char,
];
/*
 * Error exit handler: must not return to caller.
 *
 * Applications may override this if they want to get control back after
 * an error.  Typically one would longjmp somewhere instead of exiting.
 * The setjmp buffer can be made a private field within an expanded error
 * handler object.  Note that the info needed to generate an error message
 * is stored in the error object, so you can generate the message now or
 * later, at your convenience.
 * You should make sure that the JPEG object is cleaned up (with jpeg_abort
 * or jpeg_destroy) at some point.
 */
unsafe extern "C" fn error_exit(mut cinfo: j_common_ptr) {
    (*(*cinfo).err)
        .output_message
        .expect("non-null function pointer")(cinfo);
    jpeg_destroy(cinfo);
    exit(EXIT_FAILURE);
}
/*
 * Actual output of an error or trace message.
 * Applications may override this method to send JPEG messages somewhere
 * other than stderr.
 *
 * On Windows, printing to stderr is generally completely useless,
 * so we provide optional code to produce an error-dialog popup.
 * Most Windows applications will still prefer to override this routine,
 * but if they don't, it'll do something at least marginally useful.
 *
 * NOTE: to use the library in an environment that doesn't support the
 * C stdio library, you may have to delete the call to fprintf() entirely,
 * not just not use this routine.
 */
unsafe extern "C" fn output_message(mut cinfo: j_common_ptr) {
    let mut buffer: [c_char; 200] = [0; 200];
    (*(*cinfo).err)
        .format_message
        .expect("non-null function pointer")(cinfo, buffer.as_mut_ptr());
    fprintf(
        stderr,
        b"%s\n\x00" as *const u8 as *const c_char,
        buffer.as_mut_ptr(),
    );
}
/*
 * Decide whether to emit a trace or warning message.
 * msg_level is one of:
 *   -1: recoverable corrupt-data warning, may want to abort.
 *    0: important advisory messages (always display to user).
 *    1: first level of tracing detail.
 *    2,3,...: successively more detailed tracing messages.
 * An application might override this method if it wanted to abort on warnings
 * or change the policy about which messages to display.
 */
unsafe extern "C" fn emit_message(mut cinfo: j_common_ptr, mut msg_level: c_int) {
    let mut err: *mut jpeg_error_mgr = (*cinfo).err;
    if msg_level < 0i32 {
        if (*err).num_warnings == 0i32 as c_long || (*err).trace_level >= 3i32 {
            (*err).output_message.expect("non-null function pointer")(cinfo);
        }
        (*err).num_warnings += 1
    } else if (*err).trace_level >= msg_level {
        (*err).output_message.expect("non-null function pointer")(cinfo);
    };
}
/*
 * Format a message string for the most recent JPEG error or message.
 * The message is stored into buffer, which should be at least JMSG_LENGTH_MAX
 * characters.  Note that no '\n' character is added to the string.
 * Few applications should need to override this method.
 */
unsafe extern "C" fn format_message(mut cinfo: j_common_ptr, mut buffer: *mut c_char) {
    let mut err: *mut jpeg_error_mgr = (*cinfo).err;
    let mut msg_code: c_int = (*err).msg_code;
    let mut msgtext: *const c_char = NULL as *const c_char;
    let mut msgptr: *const c_char = 0 as *const c_char;
    let mut ch: c_char = 0;
    let mut isstring: boolean = 0;
    if msg_code > 0i32 && msg_code <= (*err).last_jpeg_message {
        msgtext = *(*err).jpeg_message_table.offset(msg_code as isize)
    } else if !(*err).addon_message_table.is_null()
        && msg_code >= (*err).first_addon_message
        && msg_code <= (*err).last_addon_message
    {
        msgtext = *(*err)
            .addon_message_table
            .offset((msg_code - (*err).first_addon_message) as isize)
    }
    if msgtext.is_null() {
        (*err).msg_parm.i[0usize] = msg_code;
        msgtext = *(*err).jpeg_message_table.offset(0isize)
    }
    isstring = FALSE;
    msgptr = msgtext;
    loop {
        let fresh0 = msgptr;
        msgptr = msgptr.offset(1);
        ch = *fresh0;
        if !(ch as c_int != '\u{0}' as i32) {
            break;
        }
        if !(ch as c_int == '%' as i32) {
            continue;
        }
        if *msgptr as c_int == 's' as i32 {
            isstring = TRUE
        }
        break;
    }
    if 0 != isstring {
        sprintf(buffer, msgtext, (*err).msg_parm.s.as_mut_ptr());
    } else {
        sprintf(
            buffer,
            msgtext,
            (*err).msg_parm.i[0usize],
            (*err).msg_parm.i[1usize],
            (*err).msg_parm.i[2usize],
            (*err).msg_parm.i[3usize],
            (*err).msg_parm.i[4usize],
            (*err).msg_parm.i[5usize],
            (*err).msg_parm.i[6usize],
            (*err).msg_parm.i[7usize],
        );
    };
}
/*
 * Reset error state variables at start of a new image.
 * This is called during compression startup to reset trace/error
 * processing to default state, without losing any application-specific
 * method pointers.  An application might possibly want to override
 * this method if it has additional error processing state.
 */
unsafe extern "C" fn reset_error_mgr(mut cinfo: j_common_ptr) {
    (*(*cinfo).err).num_warnings = 0i32 as c_long;
    (*(*cinfo).err).msg_code = 0i32;
}
/* Originally, this macro was used as a way of defining function prototypes
 * for both modern compilers as well as older compilers that did not support
 * prototype parameters.  libjpeg-turbo has never supported these older,
 * non-ANSI compilers, but the macro is still included because there is some
 * software out there that uses it.
 */
/* Default error-management setup */
/*
 * Fill in the standard error-handling methods in a jpeg_error_mgr object.
 * Typical call is:
 *      struct jpeg_compress_struct cinfo;
 *      struct jpeg_error_mgr err;
 *
 *      cinfo.err = jpeg_std_error(&err);
 * after which the application may override some of the methods.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_std_error(mut err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr {
    (*err).error_exit = Some(error_exit as unsafe extern "C" fn(_: j_common_ptr) -> ());
    (*err).emit_message =
        Some(emit_message as unsafe extern "C" fn(_: j_common_ptr, _: c_int) -> ());
    (*err).output_message = Some(output_message as unsafe extern "C" fn(_: j_common_ptr) -> ());
    (*err).format_message =
        Some(format_message as unsafe extern "C" fn(_: j_common_ptr, _: *mut c_char) -> ());
    (*err).reset_error_mgr = Some(reset_error_mgr as unsafe extern "C" fn(_: j_common_ptr) -> ());
    (*err).trace_level = 0i32;
    (*err).num_warnings = 0i32 as c_long;
    (*err).msg_code = 0i32;
    (*err).jpeg_message_table = jpeg_std_message_table.as_ptr();
    (*err).last_jpeg_message = JMSG_LASTMSGCODE as c_int - 1i32;
    (*err).addon_message_table = NULL as *const *const c_char;
    (*err).first_addon_message = 0i32;
    (*err).last_addon_message = 0i32;
    return err;
}
