pub use crate::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader,
    jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr, jpeg_resync_to_restart,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
    JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
    JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS,
    JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPEG_EOI, JPOOL_PERMANENT, JQUANT_TBL, JSAMPARRAY,
    JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_long, c_uchar, c_ulong};
/*
 * jdatasrc-tj.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * Modified 2009-2011 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2011, 2016, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains decompression data source routines for the case of
 * reading JPEG data from memory or from a file (or any stdio stream).
 * While these routines are sufficient for most applications,
 * some will want to use a different source manager.
 * IMPORTANT: we assume that fread() will correctly transcribe an array of
 * JOCTETs from 8-bit-wide elements on external storage.  If char is wider
 * than 8 bits on your machine, you may need to do some tweaking.
 */
/* this is not a core library module, so it doesn't define JPEG_INTERNALS */
/*
 * Initialize source --- called by jpeg_read_header
 * before any data is actually read.
 */
unsafe extern "C" fn init_mem_source(mut _cinfo: j_decompress_ptr) {}
/* no work necessary here */
/*
 * Fill the input buffer --- called whenever buffer is emptied.
 *
 * In typical applications, this should read fresh data into the buffer
 * (ignoring the current state of next_input_byte & bytes_in_buffer),
 * reset the pointer & count to the start of the buffer, and return TRUE
 * indicating that the buffer has been reloaded.  It is not necessary to
 * fill the buffer entirely, only to obtain at least one more byte.
 *
 * There is no such thing as an EOF return.  If the end of the file has been
 * reached, the routine has a choice of ERREXIT() or inserting fake data into
 * the buffer.  In most cases, generating a warning message and inserting a
 * fake EOI marker is the best course of action --- this will allow the
 * decompressor to output however much of the image is there.  However,
 * the resulting error message is misleading if the real problem is an empty
 * input file, so we handle that case specially.
 *
 * In applications that need to be able to suspend compression due to input
 * not being available yet, a FALSE return indicates that no more data can be
 * obtained right now, but more may be forthcoming later.  In this situation,
 * the decompressor will return to its caller (with an indication of the
 * number of scanlines it has read, if any).  The application should resume
 * decompression after it has loaded more data into the input buffer.  Note
 * that there are substantial restrictions on the use of suspension --- see
 * the documentation.
 *
 * When suspending, the decompressor will back up to a convenient restart point
 * (typically the start of the current MCU). next_input_byte & bytes_in_buffer
 * indicate where the restart point will be if the current call returns FALSE.
 * Data beyond this point must be rescanned after resumption, so move it to
 * the front of the buffer rather than discarding it.
 */
unsafe extern "C" fn fill_mem_input_buffer(mut cinfo: j_decompress_ptr) -> boolean {
    static mut mybuffer: [JOCTET; 4] = [
        0xffi32 as JOCTET,
        JPEG_EOI as JOCTET,
        0i32 as JOCTET,
        0i32 as JOCTET,
    ];
    (*(*cinfo).err).msg_code = JWRN_JPEG_EOF as c_int;
    (*(*cinfo).err)
        .emit_message
        .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
    (*(*cinfo).src).next_input_byte = mybuffer.as_ptr();
    (*(*cinfo).src).bytes_in_buffer = 2i32 as size_t;
    return TRUE;
}
/*
 * Skip data --- used to skip over a potentially large amount of
 * uninteresting data (such as an APPn marker).
 *
 * Writers of suspendable-input applications must note that skip_input_data
 * is not granted the right to give a suspension return.  If the skip extends
 * beyond the data currently in the buffer, the buffer can be marked empty so
 * that the next read will cause a fill_input_buffer call that can suspend.
 * Arranging for additional bytes to be discarded before reloading the input
 * buffer is the application writer's problem.
 */
unsafe extern "C" fn skip_input_data(mut cinfo: j_decompress_ptr, mut num_bytes: c_long) {
    let mut src: *mut jpeg_source_mgr = (*cinfo).src;
    if num_bytes > 0i32 as c_long {
        while num_bytes > (*src).bytes_in_buffer as c_long {
            num_bytes -= (*src).bytes_in_buffer as c_long;
            (*src).fill_input_buffer.expect("non-null function pointer")(cinfo);
        }
        (*src).next_input_byte = (*src).next_input_byte.offset(num_bytes as size_t as isize);
        (*src).bytes_in_buffer = ((*src).bytes_in_buffer as c_ulong)
            .wrapping_sub(num_bytes as size_t) as size_t as size_t
    };
}
/*
 * An additional method that can be provided by data source modules is the
 * resync_to_restart method for error recovery in the presence of RST markers.
 * For the moment, this source module just uses the default resync method
 * provided by the JPEG library.  That method assumes that no backtracking
 * is possible.
 */
/*
 * Terminate source --- called by jpeg_finish_decompress
 * after all data has been read.  Often a no-op.
 *
 * NB: *not* called by jpeg_abort or jpeg_destroy; surrounding
 * application must deal with any cleanup that should happen even
 * for error exit.
 */
unsafe extern "C" fn term_source(mut _cinfo: j_decompress_ptr) {}
/* no work necessary here */
/*
 * Prepare for input from a supplied memory buffer.
 * The buffer must contain the whole JPEG data.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_src_tj(
    mut cinfo: j_decompress_ptr,
    mut inbuffer: *const c_uchar,
    mut insize: c_ulong,
) {
    let mut src: *mut jpeg_source_mgr = 0 as *mut jpeg_source_mgr;
    if inbuffer.is_null() || insize == 0i32 as c_ulong {
        (*(*cinfo).err).msg_code = JERR_INPUT_EMPTY as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).src.is_null() {
        (*cinfo).src = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ::std::mem::size_of::<jpeg_source_mgr>() as c_ulong,
        ) as *mut jpeg_source_mgr
    } else if (*(*cinfo).src).init_source
        != Some(init_mem_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ())
    {
        (*(*cinfo).err).msg_code = JERR_BUFFER_SIZE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    src = (*cinfo).src;
    (*src).init_source = Some(init_mem_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).fill_input_buffer =
        Some(fill_mem_input_buffer as unsafe extern "C" fn(_: j_decompress_ptr) -> boolean);
    (*src).skip_input_data =
        Some(skip_input_data as unsafe extern "C" fn(_: j_decompress_ptr, _: c_long) -> ());
    (*src).resync_to_restart = Some(
        jpeg_resync_to_restart as unsafe extern "C" fn(_: j_decompress_ptr, _: c_int) -> boolean,
    );
    (*src).term_source = Some(term_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).bytes_in_buffer = insize;
    (*src).next_input_byte = inbuffer as *const JOCTET;
}
