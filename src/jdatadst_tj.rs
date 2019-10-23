







































































































































































































use crate::stdlib::{free, malloc, memcpy};use libc::{c_uchar, c_ulong, c_int, c_void, self};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::stddef_h::{size_t, NULL};pub use crate::jpeglib_h::{j_common_ptr, j_compress_ptr,
                           jpeg_c_coef_controller, jpeg_c_main_controller,
                           jpeg_c_prep_controller, jpeg_color_converter,
                           jpeg_common_struct, jpeg_comp_master,
                           jpeg_component_info, jpeg_compress_struct,
                           jpeg_destination_mgr, jpeg_downsampler,
                           jpeg_entropy_encoder, jpeg_error_mgr,
                           jpeg_forward_dct, jpeg_marker_writer,
                           jpeg_memory_mgr, jpeg_progress_mgr, jpeg_scan_info,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
                           JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB,
                           JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
                           JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
                           JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
                           JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
                           JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_PERMANENT,
                           JQUANT_TBL, JSAMPARRAY, JSAMPROW, J_COLOR_SPACE,
                           J_DCT_METHOD};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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
                        JWRN_TOO_MUCH_DATA};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_mem_destination_mgr {
    pub pub_0: jpeg_destination_mgr,
    pub outbuffer: *mut *mut c_uchar,
    pub outsize: *mut c_ulong,
    pub newbuffer: *mut c_uchar,
    pub buffer: *mut JOCTET,
    pub bufsize: size_t,
    pub alloc: boolean,
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

pub const OUTPUT_BUF_SIZE: c_int = 4096i32;
/*
 * Initialize destination --- called by jpeg_start_compress
 * before any data is actually written.
 */

unsafe extern "C" fn init_mem_destination(mut _cinfo: j_compress_ptr) {
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
    mut cinfo: j_compress_ptr,
) -> boolean {
    
      
    let mut dest: my_mem_dest_ptr = (*cinfo).dest as my_mem_dest_ptr;
    if (*dest).alloc == 0 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
     let mut nextsize:   size_t =   (*dest).bufsize * 2u64; let mut nextbuffer:   *mut JOCTET =
     malloc(nextsize) as *mut JOCTET;
    if nextbuffer.is_null() {
        (*(*cinfo).err).msg_code = super::jerror::JERR_OUT_OF_MEMORY as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 10i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    memcpy(
        nextbuffer as *mut c_void,
        (*dest).buffer as *const c_void,
        (*dest).bufsize,
    );
    if !(*dest).newbuffer.is_null() {
        free((*dest).newbuffer as *mut c_void);
    }
    (*dest).newbuffer = nextbuffer;
    (*dest).pub_0.next_output_byte = nextbuffer.offset((*dest).bufsize as isize);
    (*dest).pub_0.free_in_buffer = (*dest).bufsize;
    (*dest).buffer = nextbuffer;
    (*dest).bufsize = nextsize;
    return TRUE;
}
/*
 * Terminate destination --- called by jpeg_finish_compress
 * after all data has been written.  Usually needs to flush buffer.
 *
 * NB: *not* called by jpeg_abort or jpeg_destroy; surrounding
 * application must deal with any cleanup that should happen even
 * for error exit.
 */

unsafe extern "C" fn term_mem_destination(mut cinfo: j_compress_ptr) {
    let mut dest: my_mem_dest_ptr = (*cinfo).dest as my_mem_dest_ptr;
    if (*dest).alloc != 0 {
        *(*dest).outbuffer = (*dest).buffer
    }
    *(*dest).outsize =  (*dest).bufsize - (*dest).pub_0.free_in_buffer;
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
    mut cinfo: j_compress_ptr,
    mut outbuffer: *mut *mut c_uchar,
    mut outsize: *mut c_ulong,
    mut alloc: boolean,
) {
     let mut dest:  my_mem_dest_ptr =
     ::std::ptr::null_mut::< my_mem_destination_mgr>();let mut reused: boolean = FALSE;
    
    if outbuffer.is_null() || outsize.is_null() {
        /* sanity check */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
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
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ::std::mem::size_of::<my_mem_destination_mgr>() as c_ulong,
        ) as *mut jpeg_destination_mgr;
        dest = (*cinfo).dest as my_mem_dest_ptr;
        (*dest).newbuffer = NULL as *mut c_uchar;
        (*dest).buffer = NULL as *mut JOCTET
    } else if (*(*cinfo).dest).init_destination
        != Some(
            init_mem_destination as unsafe extern "C" fn(_: j_compress_ptr) -> (),
        )
    {
        /* It is unsafe to reuse the existing destination manager unless it was
         * created by this function.
         */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    dest = (*cinfo).dest as my_mem_dest_ptr;
    (*dest).pub_0.init_destination = Some(
        init_mem_destination as unsafe extern "C" fn(_: j_compress_ptr) -> (),
    );
    (*dest).pub_0.empty_output_buffer = Some(
        empty_mem_output_buffer
            as unsafe extern "C" fn(
                _: j_compress_ptr,
            ) -> boolean,
    );
    (*dest).pub_0.term_destination = Some(
        term_mem_destination as unsafe extern "C" fn(_: j_compress_ptr) -> (),
    );
    if (*dest).buffer == *outbuffer && !(*outbuffer).is_null() && alloc != 0 {
        reused = TRUE
    }
    (*dest).outbuffer = outbuffer;
    (*dest).outsize = outsize;
    (*dest).alloc = alloc;
    if (*outbuffer).is_null() || *outsize == 0u64 {
        if alloc != 0 {
            /* Allocate initial buffer */
            *outbuffer =
                malloc(OUTPUT_BUF_SIZE as c_ulong) as *mut c_uchar;
            (*dest).newbuffer = *outbuffer;
            if (*dest).newbuffer.is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_OUT_OF_MEMORY as c_int;
                (*(*cinfo).err).msg_parm.i[0] = 10i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            *outsize = OUTPUT_BUF_SIZE as c_ulong
        } else {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
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
