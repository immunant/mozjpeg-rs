use ::libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
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
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
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
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
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
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memcpy;
pub use crate::stdlib::C2RustUnnamed_0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_mem_destination_mgr {
    pub pub_0: crate::jpeglib_h::jpeg_destination_mgr,
    pub outbuffer: *mut *mut libc::c_uchar,
    pub outsize: *mut libc::c_ulong,
    pub newbuffer: *mut libc::c_uchar,
    pub buffer: *mut crate::jmorecfg_h::JOCTET,
    pub bufsize: crate::stddef_h::size_t,
    pub alloc: crate::jmorecfg_h::boolean,
}

pub type my_mem_dest_ptr = *mut my_mem_destination_mgr;
/*
 * jdatadst-tj.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * Modified 2009-2012 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2011, 2014, 2016, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains compression data destination routines for the case of
 * emitting JPEG data to memory or to a file (or any stdio stream).
 * While these routines are sufficient for most applications,
 * some will want to use a different destination manager.
 * IMPORTANT: we assume that fwrite() will correctly transcribe an array of
 * JOCTETs into 8-bit-wide elements on external storage.  If char is wider
 * than 8 bits on your machine, you may need to do some tweaking.
 */
/* this is not a core library module, so it doesn't define JPEG_INTERNALS */
/* <stdlib.h> should declare malloc(),free() */

pub const OUTPUT_BUF_SIZE: libc::c_int = 4096 as libc::c_int;
/*
 * Initialize destination --- called by jpeg_start_compress
 * before any data is actually written.
 */

unsafe extern "C" fn init_mem_destination(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    /* no work necessary here */
}
/*
 * Empty the output buffer --- called whenever buffer fills up.
 *
 * In typical applications, this should write the entire output buffer
 * (ignoring the current state of next_output_byte & free_in_buffer),
 * reset the pointer & count to the start of the buffer, and return TRUE
 * indicating that the buffer has been dumped.
 *
 * In applications that need to be able to suspend compression due to output
 * overrun, a FALSE return indicates that the buffer cannot be emptied now.
 * In this situation, the compressor will return to its caller (possibly with
 * an indication that it has not accepted all the supplied scanlines).  The
 * application should resume compression after it has made more room in the
 * output buffer.  Note that there are substantial restrictions on the use of
 * suspension --- see the documentation.
 *
 * When suspending, the compressor will back up to a convenient restart point
 * (typically the start of the current MCU). next_output_byte & free_in_buffer
 * indicate where the restart point will be if the current call returns FALSE.
 * Data beyond this point will be regenerated after resumption, so do not
 * write it out when emptying the buffer externally.
 */

unsafe extern "C" fn empty_mem_output_buffer(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut nextsize: crate::stddef_h::size_t = 0;
    let mut nextbuffer: *mut crate::jmorecfg_h::JOCTET = 0 as *mut crate::jmorecfg_h::JOCTET;
    let mut dest: my_mem_dest_ptr = (*cinfo).dest as my_mem_dest_ptr;
    if (*dest).alloc == 0 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BUFFER_SIZE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Try to allocate new buffer with double size */
    nextsize = (*dest)
        .bufsize
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    nextbuffer = crate::stdlib::malloc(nextsize) as *mut crate::jmorecfg_h::JOCTET;
    if nextbuffer.is_null() {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_OUT_OF_MEMORY as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = 10 as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    crate::stdlib::memcpy(
        nextbuffer as *mut libc::c_void,
        (*dest).buffer as *const libc::c_void,
        (*dest).bufsize,
    );
    if !(*dest).newbuffer.is_null() {
        crate::stdlib::free((*dest).newbuffer as *mut libc::c_void);
    }
    (*dest).newbuffer = nextbuffer;
    (*dest).pub_0.next_output_byte = nextbuffer.offset((*dest).bufsize as isize);
    (*dest).pub_0.free_in_buffer = (*dest).bufsize;
    (*dest).buffer = nextbuffer;
    (*dest).bufsize = nextsize;
    return crate::jmorecfg_h::TRUE;
}
/*
 * Terminate destination --- called by jpeg_finish_compress
 * after all data has been written.  Usually needs to flush buffer.
 *
 * NB: *not* called by jpeg_abort or jpeg_destroy; surrounding
 * application must deal with any cleanup that should happen even
 * for error exit.
 */

unsafe extern "C" fn term_mem_destination(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut dest: my_mem_dest_ptr = (*cinfo).dest as my_mem_dest_ptr;
    if (*dest).alloc != 0 {
        *(*dest).outbuffer = (*dest).buffer
    }
    *(*dest).outsize = (*dest).bufsize.wrapping_sub((*dest).pub_0.free_in_buffer);
}
/*
 * Prepare for output to a memory buffer.
 * The caller may supply an own initial buffer with appropriate size.
 * Otherwise, or when the actual data output exceeds the given size,
 * the library adapts the buffer size as necessary.
 * The standard library functions malloc/free are used for allocating
 * larger memory, so the buffer is available to the application after
 * finishing compression, and then the application is responsible for
 * freeing the requested memory.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_mem_dest_tj(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut outbuffer: *mut *mut libc::c_uchar,
    mut outsize: *mut libc::c_ulong,
    mut alloc: crate::jmorecfg_h::boolean,
) {
    let mut reused: crate::jmorecfg_h::boolean = crate::jmorecfg_h::FALSE;
    let mut dest: my_mem_dest_ptr = 0 as *mut my_mem_destination_mgr;
    if outbuffer.is_null() || outsize.is_null() {
        /* sanity check */
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BUFFER_SIZE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* The destination object is made permanent so that multiple JPEG images
     * can be written to the same buffer without re-executing jpeg_mem_dest.
     */
    if (*cinfo).dest.is_null() {
        /* first time for this JPEG object? */
        (*cinfo).dest = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_PERMANENT,
            ::std::mem::size_of::<my_mem_destination_mgr>() as libc::c_ulong,
        ) as *mut crate::jpeglib_h::jpeg_destination_mgr;
        dest = (*cinfo).dest as my_mem_dest_ptr;
        (*dest).newbuffer = crate::stddef_h::NULL as *mut libc::c_uchar;
        (*dest).buffer = crate::stddef_h::NULL as *mut crate::jmorecfg_h::JOCTET
    } else if (*(*cinfo).dest).init_destination
        != Some(
            init_mem_destination as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
        )
    {
        /* It is unsafe to reuse the existing destination manager unless it was
         * created by this function.
         */
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BUFFER_SIZE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    dest = (*cinfo).dest as my_mem_dest_ptr;
    (*dest).pub_0.init_destination = Some(
        init_mem_destination as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
    );
    (*dest).pub_0.empty_output_buffer = Some(
        empty_mem_output_buffer
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    (*dest).pub_0.term_destination = Some(
        term_mem_destination as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
    );
    if (*dest).buffer == *outbuffer && !(*outbuffer).is_null() && alloc != 0 {
        reused = crate::jmorecfg_h::TRUE
    }
    (*dest).outbuffer = outbuffer;
    (*dest).outsize = outsize;
    (*dest).alloc = alloc;
    if (*outbuffer).is_null() || *outsize == 0 as libc::c_int as libc::c_ulong {
        if alloc != 0 {
            /* Allocate initial buffer */
            *outbuffer =
                crate::stdlib::malloc(OUTPUT_BUF_SIZE as libc::c_ulong) as *mut libc::c_uchar;
            (*dest).newbuffer = *outbuffer;
            if (*dest).newbuffer.is_null() {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_OUT_OF_MEMORY as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = 10 as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            *outsize = OUTPUT_BUF_SIZE as libc::c_ulong
        } else {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BUFFER_SIZE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    (*dest).buffer = *outbuffer;
    (*dest).pub_0.next_output_byte = (*dest).buffer;
    if reused == 0 {
        (*dest).bufsize = *outsize
    }
    (*dest).pub_0.free_in_buffer = (*dest).bufsize;
}
