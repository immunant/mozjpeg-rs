























































































































































































































use crate::stdlib::fread;use libc::{c_uchar, c_ulong, c_int, c_long, c_void, self};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::stddef_h::{size_t, NULL};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_reader,
                           jpeg_marker_struct, jpeg_memory_mgr,
                           jpeg_progress_mgr, jpeg_resync_to_restart,
                           jpeg_saved_marker_ptr, jpeg_source_mgr,
                           jpeg_upsampler, jvirt_barray_control,
                           jvirt_barray_ptr, jvirt_sarray_control,
                           jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr,
                           JBLOCK, JBLOCKARRAY, JBLOCKROW, JCS_CMYK,
                           JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPEG_EOI,
                           JPOOL_PERMANENT, JQUANT_TBL, JSAMPARRAY, JSAMPROW,
                           J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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

pub type my_src_ptr = *mut my_source_mgr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_source_mgr {
    pub pub_0: jpeg_source_mgr,
    pub infile: *mut FILE,
    pub buffer: *mut JOCTET,
    pub start_of_file: boolean,
}

pub const INPUT_BUF_SIZE: c_int = 4096i32;
/* choose an efficiently fread'able size */
/*
 * Initialize source --- called by jpeg_read_header
 * before any data is actually read.
 */

unsafe extern "C" fn init_source(mut cinfo: j_decompress_ptr) {
    let mut src: my_src_ptr = (*cinfo).src as my_src_ptr;
    /* We reset the empty-input-file flag for each image,
     * but we don't clear the input buffer.
     * This is correct behavior for reading a series of images from one source.
     */
    (*src).start_of_file = TRUE;
}

unsafe extern "C" fn init_mem_source(mut cinfo: j_decompress_ptr) {
    /* no work necessary here */
}
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

unsafe extern "C" fn fill_input_buffer(
    mut cinfo: j_decompress_ptr,
) -> boolean {
     let mut src: my_src_ptr = (*cinfo).src as my_src_ptr;
    
     let mut nbytes:   size_t =
     fread(
        (*src).buffer as *mut c_void,
        1u64,
        4096u64,
        (*src).infile,
    );
    if nbytes <= 0u64 {
        if (*src).start_of_file != 0 {
            /* Treat empty input file as fatal error */
            (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EMPTY as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        (*(*cinfo).err).msg_code = super::jerror::JWRN_JPEG_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
        /* Insert a fake EOI marker */
        *(*src).buffer.offset(0) = 0xffu8;
        *(*src).buffer.offset(1) = JPEG_EOI as JOCTET;
        nbytes = 2u64
    }
    (*src).pub_0.next_input_byte = (*src).buffer;
    (*src).pub_0.bytes_in_buffer = nbytes;
    (*src).start_of_file = FALSE;
    return TRUE;
}

unsafe extern "C" fn fill_mem_input_buffer(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    static mut mybuffer: [JOCTET; 4] = [
        0xffu8,
        JPEG_EOI as JOCTET,
        0u8,
        0u8,
    ];
    /* The whole JPEG data is expected to reside in the supplied memory
     * buffer, so any request for more data beyond the given buffer size
     * is treated as an error.
     */
    (*(*cinfo).err).msg_code = super::jerror::JWRN_JPEG_EOF as c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
    /* Insert a fake EOI marker */
    (*(*cinfo).src).next_input_byte = mybuffer.as_ptr();
    (*(*cinfo).src).bytes_in_buffer = 2u64;
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

unsafe extern "C" fn skip_input_data(
    mut cinfo: j_decompress_ptr,
    mut num_bytes: c_long,
) {
    let mut src: *mut jpeg_source_mgr = (*cinfo).src;
    /* Just a dumb implementation for now.  Could use fseek() except
     * it doesn't work on pipes.  Not clear that being smart is worth
     * any trouble anyway --- large skips are infrequent.
     */
    if num_bytes > 0i64 {
        while num_bytes > (*src).bytes_in_buffer as c_long {
            num_bytes -= (*src).bytes_in_buffer as c_long;
            Some((*src).fill_input_buffer.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo);
            /* note we assume that fill_input_buffer will never return FALSE,
             * so suspension need not be handled.
             */
        }
        (*src).next_input_byte = (*src)
            .next_input_byte
            .offset(num_bytes as isize);
        (*src).bytes_in_buffer = (*src).bytes_in_buffer - num_bytes as size_t
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

unsafe extern "C" fn term_source(mut cinfo: j_decompress_ptr) {
    /* no work necessary here */
}
/*
 * Prepare for input from a stdio stream.
 * The caller must have already opened the stream, and is responsible
 * for closing it after finishing decompression.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_stdio_src(
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) {
     let mut src:  my_src_ptr =  ::std::ptr::null_mut::< my_source_mgr>();
    /* The source object and input buffer are made permanent so that a series
     * of JPEG images can be read from the same file by calling jpeg_stdio_src
     * only before the first one.  (If we discarded the buffer at the end of
     * one image, we'd likely lose the start of the next one.)
     */
    if (*cinfo).src.is_null() {
        /* first time for this JPEG object? */
        (*cinfo).src = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ::std::mem::size_of::<my_source_mgr>() as c_ulong,
        ) as *mut jpeg_source_mgr;
        src = (*cinfo).src as my_src_ptr;
        (*src).buffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            INPUT_BUF_SIZE as c_ulong *
    ::std::mem::size_of::<JOCTET>() as c_ulong,
        ) as *mut JOCTET
    } else if (*(*cinfo).src).init_source
        != Some(init_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ())
    {
        /* It is unsafe to reuse the existing source manager unless it was created
         * by this function.  Otherwise, there is no guarantee that the opaque
         * structure is the right size.  Note that we could just create a new
         * structure, but the old structure would not be freed until
         * jpeg_destroy_decompress() was called.
         */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int; /* use default method */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
        /* forces fill_input_buffer on first read */
    }
    src = (*cinfo).src as my_src_ptr;
    (*src).pub_0.init_source =
        Some(init_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).pub_0.fill_input_buffer = Some(
        fill_input_buffer
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
            ) -> boolean,
    );
    (*src).pub_0.skip_input_data = Some(
        skip_input_data
            as unsafe extern "C" fn(_: j_decompress_ptr, _: c_long) -> (),
    );
    (*src).pub_0.resync_to_restart = Some(
        jpeg_resync_to_restart
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: c_int,
            ) -> boolean,
    );
    (*src).pub_0.term_source =
        Some(term_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).infile = infile;
    (*src).pub_0.bytes_in_buffer = 0u64;
    (*src).pub_0.next_input_byte = NULL as *const JOCTET;
    /* until buffer loaded */
}
/*
 * Prepare for input from a supplied memory buffer.
 * The buffer must contain the whole JPEG data.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_mem_src(
    mut cinfo: j_decompress_ptr,
    mut inbuffer: *const c_uchar,
    mut insize: c_ulong,
) {
     
    if inbuffer.is_null() || insize == 0u64 {
        /* Treat empty input as fatal error */
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EMPTY as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* The source object is made permanent so that a series of JPEG images
     * can be read from the same buffer by calling jpeg_mem_src only before
     * the first one.
     */
    if (*cinfo).src.is_null() {
        /* first time for this JPEG object? */
        (*cinfo).src = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ::std::mem::size_of::<jpeg_source_mgr>() as c_ulong,
        ) as *mut jpeg_source_mgr
    } else if (*(*cinfo).src).init_source
        != Some(
            init_mem_source as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
        )
    {
        /* It is unsafe to reuse the existing source manager unless it was created
         * by this function.
         */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int; /* use default method */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut src:   *mut jpeg_source_mgr =  (*cinfo).src;
    (*src).init_source =
        Some(init_mem_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).fill_input_buffer = Some(
        fill_mem_input_buffer
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
            ) -> boolean,
    );
    (*src).skip_input_data = Some(
        skip_input_data
            as unsafe extern "C" fn(_: j_decompress_ptr, _: c_long) -> (),
    );
    (*src).resync_to_restart = Some(
        jpeg_resync_to_restart
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: c_int,
            ) -> boolean,
    );
    (*src).term_source =
        Some(term_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).bytes_in_buffer = insize;
    (*src).next_input_byte =  inbuffer;
}
