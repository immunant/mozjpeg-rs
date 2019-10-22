pub use super::jerror::{
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
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jround_up, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
    JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY,
    JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_long, c_uint, c_ulong, c_void};

pub type my_post_ptr = *mut my_post_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_post_controller {
    pub pub_0: jpeg_d_post_controller,
    pub whole_image: jvirt_sarray_ptr,
    pub buffer: JSAMPARRAY,
    pub strip_height: JDIMENSION,
    pub starting_row: JDIMENSION,
    pub next_row: JDIMENSION,
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_dpost(mut cinfo: j_decompress_ptr, mut pass_mode: J_BUF_MODE) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    match pass_mode as c_uint {
        0 => {
            if (*cinfo).quantize_colors != 0 {
                /* Single-pass processing with color quantization. */
                (*post).pub_0.post_process_data = Some(
                    post_process_1pass
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPIMAGE,
                            _: *mut JDIMENSION,
                            _: JDIMENSION,
                            _: JSAMPARRAY,
                            _: *mut JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                );
                /* We could be doing buffered-image output before starting a 2-pass
                 * color quantization; in that case, jinit_d_post_controller did not
                 * allocate a strip buffer.  Use the virtual-array buffer as workspace.
                 */
                if (*post).buffer.is_null() {
                    (*post).buffer = Some(
                        (*(*cinfo).mem)
                            .access_virt_sarray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        (*post).whole_image,
                        0i32 as JDIMENSION,
                        (*post).strip_height,
                        TRUE,
                    )
                }
            } else {
                /* For single-pass processing without color quantization,
                 * I have no work to do; just call the upsampler directly.
                 */
                (*post).pub_0.post_process_data = (*(*cinfo).upsample).upsample
            }
        }
        3 => {
            /* First pass of 2-pass quantization */
            if (*post).whole_image.is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*post).pub_0.post_process_data = Some(
                post_process_prepass
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                        _: *mut JDIMENSION,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: *mut JDIMENSION,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
        2 => {
            /* Second pass of 2-pass quantization */
            if (*post).whole_image.is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*post).pub_0.post_process_data = Some(
                post_process_2pass
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                        _: *mut JDIMENSION,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: *mut JDIMENSION,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
        _ => {
            /* QUANT_2PASS_SUPPORTED */
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    (*post).next_row = 0i32 as JDIMENSION;
    (*post).starting_row = (*post).next_row;
}
/* Forward declarations */
/*
 * Process some data in the one-pass (strip buffer) case.
 * This is used for color precision reduction as well as one-pass quantization.
 */

unsafe extern "C" fn post_process_1pass(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    let mut num_rows: JDIMENSION = 0;
    let mut max_rows: JDIMENSION = 0;
    /* Fill the buffer, but not more than what we can dump out in one go. */
    /* Note we rely on the upsampler to detect bottom of image. */
    max_rows = out_rows_avail.wrapping_sub(*out_row_ctr);
    if max_rows > (*post).strip_height {
        max_rows = (*post).strip_height
    }
    num_rows = 0i32 as JDIMENSION;
    Some(
        (*(*cinfo).upsample)
            .upsample
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        input_buf,
        in_row_group_ctr,
        in_row_groups_avail,
        (*post).buffer,
        &mut num_rows,
        max_rows,
    );
    /* Quantize and emit data. */
    Some(
        (*(*cinfo).cquantize)
            .color_quantize
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*post).buffer,
        output_buf.offset(*out_row_ctr as isize),
        num_rows as c_int,
    );
    *out_row_ctr = (*out_row_ctr as c_uint).wrapping_add(num_rows) as JDIMENSION as JDIMENSION;
}
/*
 * Process some data in the first pass of 2-pass quantization.
 */

unsafe extern "C" fn post_process_prepass(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    let mut old_next_row: JDIMENSION = 0;
    let mut num_rows: JDIMENSION = 0;
    /* Reposition virtual buffer if at start of strip. */
    if (*post).next_row == 0i32 as c_uint {
        (*post).buffer = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*post).whole_image,
            (*post).starting_row,
            (*post).strip_height,
            TRUE,
        )
    }
    /* Upsample some data (up to a strip height's worth). */
    old_next_row = (*post).next_row;
    Some(
        (*(*cinfo).upsample)
            .upsample
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        input_buf,
        in_row_group_ctr,
        in_row_groups_avail,
        (*post).buffer,
        &mut (*post).next_row,
        (*post).strip_height,
    );
    /* Allow quantizer to scan new data.  No data is emitted, */
    /* but we advance out_row_ctr so outer loop can tell when we're done. */
    if (*post).next_row > old_next_row {
        num_rows = (*post).next_row.wrapping_sub(old_next_row);
        Some(
            (*(*cinfo).cquantize)
                .color_quantize
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo,
            (*post).buffer.offset(old_next_row as isize),
            NULL as *mut c_void as JSAMPARRAY,
            num_rows as c_int,
        );
        *out_row_ctr = (*out_row_ctr as c_uint).wrapping_add(num_rows) as JDIMENSION as JDIMENSION
    }
    /* Advance if we filled the strip. */
    if (*post).next_row >= (*post).strip_height {
        (*post).starting_row = ((*post).starting_row as c_uint).wrapping_add((*post).strip_height)
            as JDIMENSION as JDIMENSION;
        (*post).next_row = 0i32 as JDIMENSION
    };
}
/*
 * Process some data in the second pass of 2-pass quantization.
 */

unsafe extern "C" fn post_process_2pass(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    let mut num_rows: JDIMENSION = 0;
    let mut max_rows: JDIMENSION = 0;
    /* Reposition virtual buffer if at start of strip. */
    if (*post).next_row == 0i32 as c_uint {
        (*post).buffer = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*post).whole_image,
            (*post).starting_row,
            (*post).strip_height,
            FALSE,
        )
    }
    /* Determine number of rows to emit. */
    num_rows = (*post).strip_height.wrapping_sub((*post).next_row); /* available in strip */
    max_rows = out_rows_avail.wrapping_sub(*out_row_ctr); /* available in output area */
    if num_rows > max_rows {
        num_rows = max_rows
    }
    /* We have to check bottom of image here, can't depend on upsampler. */
    max_rows = (*cinfo).output_height.wrapping_sub((*post).starting_row);
    if num_rows > max_rows {
        num_rows = max_rows
    }
    /* Quantize and emit data. */
    Some(
        (*(*cinfo).cquantize)
            .color_quantize
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*post).buffer.offset((*post).next_row as isize),
        output_buf.offset(*out_row_ctr as isize),
        num_rows as c_int,
    );
    *out_row_ctr = (*out_row_ctr as c_uint).wrapping_add(num_rows) as JDIMENSION as JDIMENSION;
    /* Advance if we filled the strip. */
    (*post).next_row =
        ((*post).next_row as c_uint).wrapping_add(num_rows) as JDIMENSION as JDIMENSION;
    if (*post).next_row >= (*post).strip_height {
        (*post).starting_row = ((*post).starting_row as c_uint).wrapping_add((*post).strip_height)
            as JDIMENSION as JDIMENSION;
        (*post).next_row = 0i32 as JDIMENSION
    };
}
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Compression module initialization routines */
/* Decompression module initialization routines */
/* QUANT_2PASS_SUPPORTED */
/*
 * Initialize postprocessing controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_d_post_controller(
    mut cinfo: j_decompress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut post: my_post_ptr = 0 as *mut my_post_controller; /* flag for no virtual arrays */
    post = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_post_controller>() as c_ulong,
    ) as my_post_ptr; /* flag for no strip buffer */
    (*cinfo).post = post as *mut jpeg_d_post_controller;
    (*post).pub_0.start_pass =
        Some(start_pass_dpost as unsafe extern "C" fn(_: j_decompress_ptr, _: J_BUF_MODE) -> ());
    (*post).whole_image = NULL as jvirt_sarray_ptr;
    (*post).buffer = NULL as JSAMPARRAY;
    /* Create the quantization buffer, if needed */
    if (*cinfo).quantize_colors != 0 {
        /* The buffer strip height is max_v_samp_factor, which is typically
         * an efficient number of rows for upsampling to return.
         * (In the presence of output rescaling, we might want to be smarter?)
         */
        (*post).strip_height = (*cinfo).max_v_samp_factor as JDIMENSION;
        if need_full_buffer != 0 {
            /* Two-pass color quantization: need full-image storage. */
            /* We round up the number of rows to a multiple of the strip height. */
            (*post).whole_image = Some(
                (*(*cinfo).mem)
                    .request_virt_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                FALSE,
                (*cinfo)
                    .output_width
                    .wrapping_mul((*cinfo).out_color_components as c_uint),
                jround_up(
                    (*cinfo).output_height as c_long,
                    (*post).strip_height as c_long,
                ) as JDIMENSION,
                (*post).strip_height,
            )
        /* QUANT_2PASS_SUPPORTED */
        } else {
            /* One-pass color quantization: just make a strip buffer. */
            (*post).buffer = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                (*cinfo)
                    .output_width
                    .wrapping_mul((*cinfo).out_color_components as c_uint),
                (*post).strip_height,
            )
        }
    };
}
