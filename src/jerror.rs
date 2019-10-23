pub use crate::jpeglib_h::C2RustUnnamed_2;
use libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_destroy;
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
pub use crate::stddef_h::NULL;
pub use crate::stdlib::exit;
use crate::stdlib::fprintf;
use crate::stdlib::sprintf;
use crate::stdlib::stderr;
pub use crate::stdlib::EXIT_FAILURE;
// =============== BEGIN jerror_h ================

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
pub type C2RustUnnamed_3 = libc::c_uint;

pub const JMSG_LASTMSGCODE: crate::src::jerror::C2RustUnnamed_3 = 129;

pub const JWRN_BOGUS_ICC: crate::src::jerror::C2RustUnnamed_3 = 128;

pub const JERR_UNSUPPORTED_SUSPEND: crate::src::jerror::C2RustUnnamed_3 = 127;

pub const JERR_BAD_PARAM_VALUE: crate::src::jerror::C2RustUnnamed_3 = 126;

pub const JERR_BAD_PARAM: crate::src::jerror::C2RustUnnamed_3 = 125;

pub const JERR_BAD_CROP_SPEC: crate::src::jerror::C2RustUnnamed_3 = 124;

pub const JWRN_TOO_MUCH_DATA: crate::src::jerror::C2RustUnnamed_3 = 123;

pub const JWRN_NOT_SEQUENTIAL: crate::src::jerror::C2RustUnnamed_3 = 122;

pub const JWRN_MUST_RESYNC: crate::src::jerror::C2RustUnnamed_3 = 121;

pub const JWRN_JPEG_EOF: crate::src::jerror::C2RustUnnamed_3 = 120;

pub const JWRN_JFIF_MAJOR: crate::src::jerror::C2RustUnnamed_3 = 119;

pub const JWRN_HUFF_BAD_CODE: crate::src::jerror::C2RustUnnamed_3 = 118;

pub const JWRN_HIT_MARKER: crate::src::jerror::C2RustUnnamed_3 = 117;

pub const JWRN_EXTRANEOUS_DATA: crate::src::jerror::C2RustUnnamed_3 = 116;

pub const JWRN_BOGUS_PROGRESSION: crate::src::jerror::C2RustUnnamed_3 = 115;

pub const JWRN_ADOBE_XFORM: crate::src::jerror::C2RustUnnamed_3 = 114;

pub const JTRC_XMS_OPEN: crate::src::jerror::C2RustUnnamed_3 = 113;

pub const JTRC_XMS_CLOSE: crate::src::jerror::C2RustUnnamed_3 = 112;

pub const JTRC_UNKNOWN_IDS: crate::src::jerror::C2RustUnnamed_3 = 111;

pub const JTRC_THUMB_RGB: crate::src::jerror::C2RustUnnamed_3 = 110;

pub const JTRC_THUMB_PALETTE: crate::src::jerror::C2RustUnnamed_3 = 109;

pub const JTRC_THUMB_JPEG: crate::src::jerror::C2RustUnnamed_3 = 108;

pub const JTRC_TFILE_OPEN: crate::src::jerror::C2RustUnnamed_3 = 107;

pub const JTRC_TFILE_CLOSE: crate::src::jerror::C2RustUnnamed_3 = 106;

pub const JTRC_SOS_PARAMS: crate::src::jerror::C2RustUnnamed_3 = 105;

pub const JTRC_SOS_COMPONENT: crate::src::jerror::C2RustUnnamed_3 = 104;

pub const JTRC_SOS: crate::src::jerror::C2RustUnnamed_3 = 103;

pub const JTRC_SOI: crate::src::jerror::C2RustUnnamed_3 = 102;

pub const JTRC_SOF_COMPONENT: crate::src::jerror::C2RustUnnamed_3 = 101;

pub const JTRC_SOF: crate::src::jerror::C2RustUnnamed_3 = 100;

pub const JTRC_SMOOTH_NOTIMPL: crate::src::jerror::C2RustUnnamed_3 = 99;

pub const JTRC_RST: crate::src::jerror::C2RustUnnamed_3 = 98;

pub const JTRC_RECOVERY_ACTION: crate::src::jerror::C2RustUnnamed_3 = 97;

pub const JTRC_QUANT_SELECTED: crate::src::jerror::C2RustUnnamed_3 = 96;

pub const JTRC_QUANT_NCOLORS: crate::src::jerror::C2RustUnnamed_3 = 95;

pub const JTRC_QUANT_3_NCOLORS: crate::src::jerror::C2RustUnnamed_3 = 94;

pub const JTRC_QUANTVALS: crate::src::jerror::C2RustUnnamed_3 = 93;

pub const JTRC_PARMLESS_MARKER: crate::src::jerror::C2RustUnnamed_3 = 92;

pub const JTRC_MISC_MARKER: crate::src::jerror::C2RustUnnamed_3 = 91;

pub const JTRC_JFIF_THUMBNAIL: crate::src::jerror::C2RustUnnamed_3 = 90;

pub const JTRC_JFIF_EXTENSION: crate::src::jerror::C2RustUnnamed_3 = 89;

pub const JTRC_JFIF_BADTHUMBNAILSIZE: crate::src::jerror::C2RustUnnamed_3 = 88;

pub const JTRC_JFIF: crate::src::jerror::C2RustUnnamed_3 = 87;

pub const JTRC_HUFFBITS: crate::src::jerror::C2RustUnnamed_3 = 86;

pub const JTRC_EOI: crate::src::jerror::C2RustUnnamed_3 = 85;

pub const JTRC_EMS_OPEN: crate::src::jerror::C2RustUnnamed_3 = 84;

pub const JTRC_EMS_CLOSE: crate::src::jerror::C2RustUnnamed_3 = 83;

pub const JTRC_DRI: crate::src::jerror::C2RustUnnamed_3 = 82;

pub const JTRC_DQT: crate::src::jerror::C2RustUnnamed_3 = 81;

pub const JTRC_DHT: crate::src::jerror::C2RustUnnamed_3 = 80;

pub const JTRC_DAC: crate::src::jerror::C2RustUnnamed_3 = 79;

pub const JTRC_APP14: crate::src::jerror::C2RustUnnamed_3 = 78;

pub const JTRC_APP0: crate::src::jerror::C2RustUnnamed_3 = 77;

pub const JTRC_ADOBE: crate::src::jerror::C2RustUnnamed_3 = 76;

pub const JTRC_16BIT_TABLES: crate::src::jerror::C2RustUnnamed_3 = 75;

pub const JMSG_VERSION: crate::src::jerror::C2RustUnnamed_3 = 74;

pub const JMSG_COPYRIGHT: crate::src::jerror::C2RustUnnamed_3 = 73;

pub const JERR_XMS_WRITE: crate::src::jerror::C2RustUnnamed_3 = 72;

pub const JERR_XMS_READ: crate::src::jerror::C2RustUnnamed_3 = 71;

pub const JERR_WIDTH_OVERFLOW: crate::src::jerror::C2RustUnnamed_3 = 70;

pub const JERR_VIRTUAL_BUG: crate::src::jerror::C2RustUnnamed_3 = 69;

pub const JERR_UNKNOWN_MARKER: crate::src::jerror::C2RustUnnamed_3 = 68;

pub const JERR_TOO_LITTLE_DATA: crate::src::jerror::C2RustUnnamed_3 = 67;

pub const JERR_TFILE_WRITE: crate::src::jerror::C2RustUnnamed_3 = 66;

pub const JERR_TFILE_SEEK: crate::src::jerror::C2RustUnnamed_3 = 65;

pub const JERR_TFILE_READ: crate::src::jerror::C2RustUnnamed_3 = 64;

pub const JERR_TFILE_CREATE: crate::src::jerror::C2RustUnnamed_3 = 63;

pub const JERR_SOS_NO_SOF: crate::src::jerror::C2RustUnnamed_3 = 62;

pub const JERR_SOI_DUPLICATE: crate::src::jerror::C2RustUnnamed_3 = 61;

pub const JERR_SOF_UNSUPPORTED: crate::src::jerror::C2RustUnnamed_3 = 60;

pub const JERR_SOF_NO_SOS: crate::src::jerror::C2RustUnnamed_3 = 59;

pub const JERR_SOF_DUPLICATE: crate::src::jerror::C2RustUnnamed_3 = 58;

pub const JERR_QUANT_MANY_COLORS: crate::src::jerror::C2RustUnnamed_3 = 57;

pub const JERR_QUANT_FEW_COLORS: crate::src::jerror::C2RustUnnamed_3 = 56;

pub const JERR_QUANT_COMPONENTS: crate::src::jerror::C2RustUnnamed_3 = 55;

pub const JERR_OUT_OF_MEMORY: crate::src::jerror::C2RustUnnamed_3 = 54;

pub const JERR_NO_SOI: crate::src::jerror::C2RustUnnamed_3 = 53;

pub const JERR_NO_QUANT_TABLE: crate::src::jerror::C2RustUnnamed_3 = 52;

pub const JERR_NO_IMAGE: crate::src::jerror::C2RustUnnamed_3 = 51;

pub const JERR_NO_HUFF_TABLE: crate::src::jerror::C2RustUnnamed_3 = 50;

pub const JERR_NO_BACKING_STORE: crate::src::jerror::C2RustUnnamed_3 = 49;

pub const JERR_NOT_COMPILED: crate::src::jerror::C2RustUnnamed_3 = 48;

pub const JERR_NOTIMPL: crate::src::jerror::C2RustUnnamed_3 = 47;

pub const JERR_MODE_CHANGE: crate::src::jerror::C2RustUnnamed_3 = 46;

pub const JERR_MISSING_DATA: crate::src::jerror::C2RustUnnamed_3 = 45;

pub const JERR_MISMATCHED_QUANT_TABLE: crate::src::jerror::C2RustUnnamed_3 = 44;

pub const JERR_INPUT_EOF: crate::src::jerror::C2RustUnnamed_3 = 43;

pub const JERR_INPUT_EMPTY: crate::src::jerror::C2RustUnnamed_3 = 42;

pub const JERR_IMAGE_TOO_BIG: crate::src::jerror::C2RustUnnamed_3 = 41;

pub const JERR_HUFF_MISSING_CODE: crate::src::jerror::C2RustUnnamed_3 = 40;

pub const JERR_HUFF_CLEN_OVERFLOW: crate::src::jerror::C2RustUnnamed_3 = 39;

pub const JERR_FRACT_SAMPLE_NOTIMPL: crate::src::jerror::C2RustUnnamed_3 = 38;

pub const JERR_FILE_WRITE: crate::src::jerror::C2RustUnnamed_3 = 37;

pub const JERR_FILE_READ: crate::src::jerror::C2RustUnnamed_3 = 36;

pub const JERR_EOI_EXPECTED: crate::src::jerror::C2RustUnnamed_3 = 35;

pub const JERR_EMS_WRITE: crate::src::jerror::C2RustUnnamed_3 = 34;

pub const JERR_EMS_READ: crate::src::jerror::C2RustUnnamed_3 = 33;

pub const JERR_EMPTY_IMAGE: crate::src::jerror::C2RustUnnamed_3 = 32;

pub const JERR_DQT_INDEX: crate::src::jerror::C2RustUnnamed_3 = 31;

pub const JERR_DHT_INDEX: crate::src::jerror::C2RustUnnamed_3 = 30;

pub const JERR_DAC_VALUE: crate::src::jerror::C2RustUnnamed_3 = 29;

pub const JERR_DAC_INDEX: crate::src::jerror::C2RustUnnamed_3 = 28;

pub const JERR_CONVERSION_NOTIMPL: crate::src::jerror::C2RustUnnamed_3 = 27;

pub const JERR_COMPONENT_COUNT: crate::src::jerror::C2RustUnnamed_3 = 26;

pub const JERR_CCIR601_NOTIMPL: crate::src::jerror::C2RustUnnamed_3 = 25;

pub const JERR_CANT_SUSPEND: crate::src::jerror::C2RustUnnamed_3 = 24;

pub const JERR_BUFFER_SIZE: crate::src::jerror::C2RustUnnamed_3 = 23;

pub const JERR_BAD_VIRTUAL_ACCESS: crate::src::jerror::C2RustUnnamed_3 = 22;

pub const JERR_BAD_STRUCT_SIZE: crate::src::jerror::C2RustUnnamed_3 = 21;

pub const JERR_BAD_STATE: crate::src::jerror::C2RustUnnamed_3 = 20;

pub const JERR_BAD_SCAN_SCRIPT: crate::src::jerror::C2RustUnnamed_3 = 19;

pub const JERR_BAD_SAMPLING: crate::src::jerror::C2RustUnnamed_3 = 18;

pub const JERR_BAD_PROG_SCRIPT: crate::src::jerror::C2RustUnnamed_3 = 17;

pub const JERR_BAD_PROGRESSION: crate::src::jerror::C2RustUnnamed_3 = 16;

pub const JERR_BAD_PRECISION: crate::src::jerror::C2RustUnnamed_3 = 15;

pub const JERR_BAD_POOL_ID: crate::src::jerror::C2RustUnnamed_3 = 14;

pub const JERR_BAD_MCU_SIZE: crate::src::jerror::C2RustUnnamed_3 = 13;

pub const JERR_BAD_LIB_VERSION: crate::src::jerror::C2RustUnnamed_3 = 12;

pub const JERR_BAD_LENGTH: crate::src::jerror::C2RustUnnamed_3 = 11;

pub const JERR_BAD_J_COLORSPACE: crate::src::jerror::C2RustUnnamed_3 = 10;

pub const JERR_BAD_IN_COLORSPACE: crate::src::jerror::C2RustUnnamed_3 = 9;

pub const JERR_BAD_HUFF_TABLE: crate::src::jerror::C2RustUnnamed_3 = 8;

pub const JERR_BAD_DCTSIZE: crate::src::jerror::C2RustUnnamed_3 = 7;

pub const JERR_BAD_DCT_COEF: crate::src::jerror::C2RustUnnamed_3 = 6;

pub const JERR_BAD_COMPONENT_ID: crate::src::jerror::C2RustUnnamed_3 = 5;

pub const JERR_BAD_BUFFER_MODE: crate::src::jerror::C2RustUnnamed_3 = 4;

pub const JERR_BAD_ALLOC_CHUNK: crate::src::jerror::C2RustUnnamed_3 = 3;

pub const JERR_BAD_ALIGN_TYPE: crate::src::jerror::C2RustUnnamed_3 = 2;
/* Must be first entry! */
/* For maintenance convenience, list is alphabetical by message code name */

pub const JERR_ARITH_NOTIMPL: crate::src::jerror::C2RustUnnamed_3 = 1;
/* JMAKE_ENUM_LIST */

pub const JMSG_NOMESSAGE: crate::src::jerror::C2RustUnnamed_3 = 0;
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

pub static mut jpeg_std_message_table: [*const libc::c_char; 130] = [
    
    b"Bogus message code %d\x00".as_ptr() as *const libc::c_char,
    
    b"Sorry, arithmetic coding is not implemented\x00".as_ptr() as *const libc::c_char,
    
    b"ALIGN_TYPE is wrong, please fix\x00".as_ptr() as *const libc::c_char,
    
    b"MAX_ALLOC_CHUNK is wrong, please fix\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus buffer control mode\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid component ID %d in SOS\x00".as_ptr() as *const libc::c_char,
    
    b"DCT coefficient out of range\x00".as_ptr() as *const libc::c_char,
    
    b"IDCT output block size %d not supported\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus Huffman table definition\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus input colorspace\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus JPEG colorspace\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus marker length\x00".as_ptr() as *const libc::c_char,
    
    b"Wrong JPEG library version: library is %d, caller expects %d\x00".as_ptr()
        as *const libc::c_char,
    
    b"Sampling factors too large for interleaved scan\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid memory pool code %d\x00".as_ptr() as *const libc::c_char,
    
    b"Unsupported JPEG data precision %d\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid progressive parameters Ss=%d Se=%d Ah=%d Al=%d\x00".as_ptr()
        as *const libc::c_char,
    
    b"Invalid progressive parameters at scan script entry %d\x00".as_ptr()
        as *const libc::c_char,
    
    b"Bogus sampling factors\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid scan script at entry %d\x00".as_ptr() as *const libc::c_char,
    
    b"Improper call to JPEG library in state %d\x00".as_ptr() as *const libc::c_char,
    
    b"JPEG parameter struct mismatch: library thinks size is %u, caller expects %u\x00".as_ptr()
        as *const libc::c_char,
    
    b"Bogus virtual array access\x00".as_ptr() as *const libc::c_char,
    
    b"Buffer passed to JPEG library is too small\x00".as_ptr() as *const libc::c_char,
    
    b"Suspension not allowed here\x00".as_ptr() as *const libc::c_char,
    
    b"CCIR601 sampling not implemented yet\x00".as_ptr() as *const libc::c_char,
    
    b"Too many color components: %d, max %d\x00".as_ptr() as *const libc::c_char,
    
    b"Unsupported color conversion request\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus DAC index %d\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus DAC value 0x%x\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus DHT index %d\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus DQT index %d\x00".as_ptr() as *const libc::c_char,
    
    b"Empty JPEG image (DNL not supported)\x00".as_ptr() as *const libc::c_char,
    
    b"Read from EMS failed\x00".as_ptr() as *const libc::c_char,
    
    b"Write to EMS failed\x00".as_ptr() as *const libc::c_char,
    
    b"Didn\'t expect more than one scan\x00".as_ptr() as *const libc::c_char,
    
    b"Input file read error\x00".as_ptr() as *const libc::c_char,
    
    b"Output file write error --- out of disk space?\x00".as_ptr() as *const libc::c_char,
    
    b"Fractional sampling not implemented yet\x00".as_ptr() as *const libc::c_char,
    
    b"Huffman code size table overflow\x00".as_ptr() as *const libc::c_char,
    
    b"Missing Huffman code table entry\x00".as_ptr() as *const libc::c_char,
    
    b"Maximum supported image dimension is %u pixels\x00".as_ptr() as *const libc::c_char,
    
    b"Empty input file\x00".as_ptr() as *const libc::c_char,
    
    b"Premature end of input file\x00".as_ptr() as *const libc::c_char,
    
    b"Cannot transcode due to multiple use of quantization table %d\x00".as_ptr()
        as *const libc::c_char,
    
    b"Scan script does not transmit all data\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid color quantization mode change\x00".as_ptr() as *const libc::c_char,
    
    b"Not implemented yet\x00".as_ptr() as *const libc::c_char,
    
    b"Requested feature was omitted at compile time\x00".as_ptr() as *const libc::c_char,
    
    b"Backing store not supported\x00".as_ptr() as *const libc::c_char,
    
    b"Huffman table 0x%02x was not defined\x00".as_ptr() as *const libc::c_char,
    
    b"JPEG datastream contains no image\x00".as_ptr() as *const libc::c_char,
    
    b"Quantization table 0x%02x was not defined\x00".as_ptr() as *const libc::c_char,
    
    b"Not a JPEG file: starts with 0x%02x 0x%02x\x00".as_ptr() as *const libc::c_char,
    
    b"Insufficient memory (case %d)\x00".as_ptr() as *const libc::c_char,
    
    b"Cannot quantize more than %d color components\x00".as_ptr() as *const libc::c_char,
    
    b"Cannot quantize to fewer than %d colors\x00".as_ptr() as *const libc::c_char,
    
    b"Cannot quantize to more than %d colors\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid JPEG file structure: two SOF markers\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid JPEG file structure: missing SOS marker\x00".as_ptr() as *const libc::c_char,
    
    b"Unsupported JPEG process: SOF type 0x%02x\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid JPEG file structure: two SOI markers\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid JPEG file structure: SOS before SOF\x00".as_ptr() as *const libc::c_char,
    
    b"Failed to create temporary file %s\x00".as_ptr() as *const libc::c_char,
    
    b"Read failed on temporary file\x00".as_ptr() as *const libc::c_char,
    
    b"Seek failed on temporary file\x00".as_ptr() as *const libc::c_char,
    
    b"Write failed on temporary file --- out of disk space?\x00".as_ptr()
        as *const libc::c_char,
    
    b"Application transferred too few scanlines\x00".as_ptr() as *const libc::c_char,
    
    b"Unsupported marker type 0x%02x\x00".as_ptr() as *const libc::c_char,
    
    b"Virtual array controller messed up\x00".as_ptr() as *const libc::c_char,
    
    b"Image too wide for this implementation\x00".as_ptr() as *const libc::c_char,
    
    b"Read from XMS failed\x00".as_ptr() as *const libc::c_char,
    
    b"Write to XMS failed\x00".as_ptr() as *const libc::c_char,
    
    b"Copyright (C) 1991-2018 The libjpeg-turbo Project and many others\x00".as_ptr()
        as *const libc::c_char,
    
    b"6b  27-Mar-1998\x00".as_ptr() as *const libc::c_char,
    
    b"Caution: quantization tables are too coarse for baseline JPEG\x00".as_ptr()
        as *const libc::c_char,
    
    b"Adobe APP14 marker: version %d, flags 0x%04x 0x%04x, transform %d\x00".as_ptr()
        as *const libc::c_char,
    
    b"Unknown APP0 marker (not JFIF), length %u\x00".as_ptr() as *const libc::c_char,
    
    b"Unknown APP14 marker (not Adobe), length %u\x00".as_ptr() as *const libc::c_char,
    
    b"Define Arithmetic Table 0x%02x: 0x%02x\x00".as_ptr() as *const libc::c_char,
    
    b"Define Huffman Table 0x%02x\x00".as_ptr() as *const libc::c_char,
    
    b"Define Quantization Table %d  precision %d\x00".as_ptr() as *const libc::c_char,
    
    b"Define Restart Interval %u\x00".as_ptr() as *const libc::c_char,
    
    b"Freed EMS handle %u\x00".as_ptr() as *const libc::c_char,
    
    b"Obtained EMS handle %u\x00".as_ptr() as *const libc::c_char,
    
    b"End Of Image\x00".as_ptr() as *const libc::c_char,
    
    b"        %3d %3d %3d %3d %3d %3d %3d %3d\x00".as_ptr() as *const libc::c_char,
    
    b"JFIF APP0 marker: version %d.%02d, density %dx%d  %d\x00".as_ptr() as *const libc::c_char,
    
    b"Warning: thumbnail image size does not match data length %u\x00".as_ptr()
        as *const libc::c_char,
    
    b"JFIF extension marker: type 0x%02x, length %u\x00".as_ptr() as *const libc::c_char,
    
    b"    with %d x %d thumbnail image\x00".as_ptr() as *const libc::c_char,
    
    b"Miscellaneous marker 0x%02x, length %u\x00".as_ptr() as *const libc::c_char,
    
    b"Unexpected marker 0x%02x\x00".as_ptr() as *const libc::c_char,
    
    b"        %4u %4u %4u %4u %4u %4u %4u %4u\x00".as_ptr() as *const libc::c_char,
    
    b"Quantizing to %d = %d*%d*%d colors\x00".as_ptr() as *const libc::c_char,
    
    b"Quantizing to %d colors\x00".as_ptr() as *const libc::c_char,
    
    b"Selected %d colors for quantization\x00".as_ptr() as *const libc::c_char,
    
    b"At marker 0x%02x, recovery action %d\x00".as_ptr() as *const libc::c_char,
    
    b"RST%d\x00".as_ptr() as *const libc::c_char,
    
    b"Smoothing not supported with nonstandard sampling ratios\x00".as_ptr()
        as *const libc::c_char,
    
    b"Start Of Frame 0x%02x: width=%u, height=%u, components=%d\x00".as_ptr()
        as *const libc::c_char,
    
    b"    Component %d: %dhx%dv q=%d\x00".as_ptr() as *const libc::c_char,
    
    b"Start of Image\x00".as_ptr() as *const libc::c_char,
    
    b"Start Of Scan: %d components\x00".as_ptr() as *const libc::c_char,
    
    b"    Component %d: dc=%d ac=%d\x00".as_ptr() as *const libc::c_char,
    
    b"  Ss=%d, Se=%d, Ah=%d, Al=%d\x00".as_ptr() as *const libc::c_char,
    
    b"Closed temporary file %s\x00".as_ptr() as *const libc::c_char,
    
    b"Opened temporary file %s\x00".as_ptr() as *const libc::c_char,
    
    b"JFIF extension marker: JPEG-compressed thumbnail image, length %u\x00".as_ptr()
        as *const libc::c_char,
    
    b"JFIF extension marker: palette thumbnail image, length %u\x00".as_ptr()
        as *const libc::c_char,
    
    b"JFIF extension marker: RGB thumbnail image, length %u\x00".as_ptr()
        as *const libc::c_char,
    
    b"Unrecognized component IDs %d %d %d, assuming YCbCr\x00".as_ptr() as *const libc::c_char,
    
    b"Freed XMS handle %u\x00".as_ptr() as *const libc::c_char,
    
    b"Obtained XMS handle %u\x00".as_ptr() as *const libc::c_char,
    
    b"Unknown Adobe color transform code %d\x00".as_ptr() as *const libc::c_char,
    
    b"Inconsistent progression sequence for component %d coefficient %d\x00".as_ptr()
        as *const libc::c_char,
    
    b"Corrupt JPEG data: %u extraneous bytes before marker 0x%02x\x00".as_ptr()
        as *const libc::c_char,
    
    b"Corrupt JPEG data: premature end of data segment\x00".as_ptr() as *const libc::c_char,
    
    b"Corrupt JPEG data: bad Huffman code\x00".as_ptr() as *const libc::c_char,
    
    b"Warning: unknown JFIF revision number %d.%02d\x00".as_ptr() as *const libc::c_char,
    
    b"Premature end of JPEG file\x00".as_ptr() as *const libc::c_char,
    
    b"Corrupt JPEG data: found marker 0x%02x instead of RST%d\x00".as_ptr()
        as *const libc::c_char,
    
    b"Invalid SOS parameters for sequential JPEG\x00".as_ptr() as *const libc::c_char,
    
    b"Application transferred too many scanlines\x00".as_ptr() as *const libc::c_char,
    
    b"Invalid crop request\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus parameter\x00".as_ptr() as *const libc::c_char,
    
    b"Bogus parameter value\x00".as_ptr() as *const libc::c_char,
    
    b"I/O suspension not supported in scan optimization\x00".as_ptr() as *const libc::c_char,
    
    b"Corrupt JPEG data: bad ICC marker\x00".as_ptr() as *const libc::c_char,
    crate::stddef_h::NULL as *const libc::c_char,
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

unsafe extern "C" fn error_exit(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    /* Always display the message */
    Some(
        (*(*cinfo).err)
            .output_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Let the memory manager delete any temp files before we die */
    crate::jpeglib_h::jpeg_destroy(cinfo);
    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
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

unsafe extern "C" fn output_message(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut buffer: [libc::c_char; 200] = [0; 200];
    /* Create the message */
    Some(
        (*(*cinfo).err)
            .format_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, buffer.as_mut_ptr());
      eprintln!("{:}",
          unsafe {
    std::ffi::CStr::from_ptr(buffer.as_mut_ptr() as
                                 *const libc::c_char).to_str().unwrap()
});
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

unsafe extern "C" fn emit_message(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut msg_level: libc::c_int,
) {
    let mut err: *mut crate::jpeglib_h::jpeg_error_mgr = (*cinfo).err;
    if msg_level < 0i32 {
        /* It's a warning message.  Since corrupt files may generate many warnings,
         * the policy implemented here is to show only the first warning,
         * unless trace_level >= 3.
         */
        if (*err).num_warnings == 0i32 as libc::c_long || (*err).trace_level >= 3i32 {
            Some((*err).output_message.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo);
        }
        /* Always count warnings in num_warnings. */
        (*err).num_warnings += 1
    } else if (*err).trace_level >= msg_level {
        Some((*err).output_message.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo);
    };
}
/* It's a trace message.  Show it if trace_level >= msg_level. */
/*
 * Format a message string for the most recent JPEG error or message.
 * The message is stored into buffer, which should be at least JMSG_LENGTH_MAX
 * characters.  Note that no '\n' character is added to the string.
 * Few applications should need to override this method.
 */

unsafe extern "C" fn format_message(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut buffer: *mut libc::c_char,
) {
    let mut err: *mut crate::jpeglib_h::jpeg_error_mgr = (*cinfo).err;
    let mut msg_code: libc::c_int = (*err).msg_code;
    let mut msgtext: *const libc::c_char = crate::stddef_h::NULL as *const libc::c_char;
    let mut msgptr: *const libc::c_char = ::std::ptr::null::< libc::c_char>();
    let mut ch: libc::c_char = 0;
    let mut isstring: crate::jmorecfg_h::boolean = 0;
    /* Look up message string in proper table */
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
    /* Defend against bogus message number */
    if msgtext.is_null() {
        (*err).msg_parm.i[0] = msg_code;
        msgtext = *(*err).jpeg_message_table.offset(0)
    }
    /* Check for string parameter, as indicated by %s in the message text */
    isstring = crate::jmorecfg_h::FALSE;
    msgptr = msgtext;
    loop {
        let fresh0 = msgptr;
        msgptr = msgptr.offset(1);
        ch = *fresh0;
        if !(ch as libc::c_int != '\u{0}' as i32) {
            break;
        }
        if !(ch as libc::c_int == '%' as i32) {
            continue;
        }
        if *msgptr as libc::c_int == 's' as i32 {
            isstring = crate::jmorecfg_h::TRUE
        }
        break;
    }
    /* Format the message into the passed buffer */
    if isstring != 0 {
        crate::stdlib::sprintf(buffer, msgtext, (*err).msg_parm.s.as_mut_ptr());
    } else {
        crate::stdlib::sprintf(
            buffer,
            msgtext,
            (*err).msg_parm.i[0],
            (*err).msg_parm.i[1],
            (*err).msg_parm.i[2],
            (*err).msg_parm.i[3],
            (*err).msg_parm.i[4],
            (*err).msg_parm.i[5],
            (*err).msg_parm.i[6],
            (*err).msg_parm.i[7],
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

unsafe extern "C" fn reset_error_mgr(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    (*(*cinfo).err).num_warnings = 0i32 as libc::c_long;
    /* trace_level is not reset since it is an application-supplied parameter */
    (*(*cinfo).err).msg_code = 0i32;
    /* may be useful as a flag for "no error" */
}
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

pub unsafe extern "C" fn jpeg_std_error(
    mut err: *mut crate::jpeglib_h::jpeg_error_mgr,
) -> *mut crate::jpeglib_h::jpeg_error_mgr {
    (*err).error_exit =
        Some(error_exit as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()); /* default = no tracing */
    (*err).emit_message = Some(
        emit_message
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> (),
    ); /* no warnings emitted yet */
    (*err).output_message =
        Some(output_message as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()); /* may be useful as a flag for "no error" */
    (*err).format_message = Some(
        format_message
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: *mut libc::c_char) -> (),
    );
    (*err).reset_error_mgr =
        Some(reset_error_mgr as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*err).trace_level = 0i32;
    (*err).num_warnings = 0i32 as libc::c_long;
    (*err).msg_code = 0i32;
    /* Initialize message table pointers */
    (*err).jpeg_message_table = jpeg_std_message_table.as_ptr(); /* for safety */
    (*err).last_jpeg_message = crate::src::jerror::JMSG_LASTMSGCODE as libc::c_int - 1i32;
    (*err).addon_message_table = crate::stddef_h::NULL as *const *const libc::c_char;
    (*err).first_addon_message = 0i32;
    (*err).last_addon_message = 0i32;
    return err;
}
