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
pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpegint_h::{
    jcopy_sample_rows, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_writer, jpeg_memory_mgr,
    jpeg_progress_mgr, jpeg_scan_info, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, JBLOCK,
    JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
    JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE,
    J_DCT_METHOD,
};
pub use crate::stddef_h::size_t;
use crate::stdlib::memcpy;
use libc::{self, c_int, c_long, c_uint, c_ulong, c_void};

pub type my_prep_ptr = *mut my_prep_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_prep_controller {
    pub pub_0: jpeg_c_prep_controller,
    pub color_buf: [JSAMPARRAY; 10],
    pub rows_to_go: JDIMENSION,
    pub next_buf_row: c_int,
    pub this_row_group: c_int,
    pub next_buf_stop: c_int,
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_prep(mut cinfo: j_compress_ptr, mut pass_mode: J_BUF_MODE) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    if pass_mode as c_uint != JBUF_PASS_THRU as c_int as c_uint {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Initialize total-height counter for detecting bottom of image */
    (*prep).rows_to_go = (*cinfo).image_height;
    /* Mark the conversion buffer empty */
    (*prep).next_buf_row = 0i32;
    /* Preset additional state variables for context mode.
     * These aren't used in non-context mode, so we needn't test which mode.
     */
    (*prep).this_row_group = 0i32;
    /* Set next_buf_stop to stop after two row groups have been read in. */
    (*prep).next_buf_stop = 2i32 * (*cinfo).max_v_samp_factor;
}
/*
 * Expand an image vertically from height input_rows to height output_rows,
 * by duplicating the bottom row.
 */

unsafe extern "C" fn expand_bottom_edge(
    mut image_data: JSAMPARRAY,
    mut num_cols: JDIMENSION,
    mut input_rows: c_int,
    mut output_rows: c_int,
) {
    let mut row: c_int = 0;
    row = input_rows;
    while row < output_rows {
        jcopy_sample_rows(
            image_data,
            input_rows - 1i32,
            image_data,
            row,
            1i32,
            num_cols,
        );
        row += 1
    }
}
/*
 * Process some data in the simple no-context case.
 *
 * Preprocessor output data is counted in "row groups".  A row group
 * is defined to be v_samp_factor sample rows of each component.
 * Downsampling will produce this much data from each max_v_samp_factor
 * input rows.
 */

unsafe extern "C" fn pre_process_data(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut in_row_ctr: *mut JDIMENSION,
    mut in_rows_avail: JDIMENSION,
    mut output_buf: JSAMPIMAGE,
    mut out_row_group_ctr: *mut JDIMENSION,
    mut out_row_groups_avail: JDIMENSION,
) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    let mut numrows: c_int = 0;
    let mut ci: c_int = 0;
    let mut inrows: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    while *in_row_ctr < in_rows_avail && *out_row_group_ctr < out_row_groups_avail {
        /* Do color conversion to fill the conversion buffer. */
        inrows = in_rows_avail.wrapping_sub(*in_row_ctr);
        numrows = (*cinfo).max_v_samp_factor - (*prep).next_buf_row;
        numrows = if (numrows as JDIMENSION) < inrows {
            numrows as JDIMENSION
        } else {
            inrows
        } as c_int;
        Some(
            (*(*cinfo).cconvert)
                .color_convert
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo,
            input_buf.offset(*in_row_ctr as isize),
            (*prep).color_buf.as_mut_ptr(),
            (*prep).next_buf_row as JDIMENSION,
            numrows,
        );
        *in_row_ctr =
            (*in_row_ctr as c_uint).wrapping_add(numrows as c_uint) as JDIMENSION as JDIMENSION;
        (*prep).next_buf_row += numrows;
        (*prep).rows_to_go = ((*prep).rows_to_go as c_uint).wrapping_sub(numrows as c_uint)
            as JDIMENSION as JDIMENSION;
        /* If at bottom of image, pad to fill the conversion buffer. */
        if (*prep).rows_to_go == 0i32 as c_uint && (*prep).next_buf_row < (*cinfo).max_v_samp_factor
        {
            ci = 0i32;
            while ci < (*cinfo).num_components {
                expand_bottom_edge(
                    (*prep).color_buf[ci as usize],
                    (*cinfo).image_width,
                    (*prep).next_buf_row,
                    (*cinfo).max_v_samp_factor,
                );
                ci += 1
            }
            (*prep).next_buf_row = (*cinfo).max_v_samp_factor
        }
        /* If we've filled the conversion buffer, empty it. */
        if (*prep).next_buf_row == (*cinfo).max_v_samp_factor {
            Some(
                (*(*cinfo).downsample)
                    .downsample
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*prep).color_buf.as_mut_ptr(),
                0i32 as JDIMENSION,
                output_buf,
                *out_row_group_ctr,
            );
            (*prep).next_buf_row = 0i32;
            *out_row_group_ctr = (*out_row_group_ctr).wrapping_add(1)
        }
        /* If at bottom of image, pad the output to a full iMCU height.
         * Note we assume the caller is providing a one-iMCU-height output buffer!
         */
        if !((*prep).rows_to_go == 0i32 as c_uint && *out_row_group_ctr < out_row_groups_avail) {
            continue;
        }
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            expand_bottom_edge(
                *output_buf.offset(ci as isize),
                (*compptr).width_in_blocks.wrapping_mul(DCTSIZE as c_uint),
                (*out_row_group_ctr).wrapping_mul((*compptr).v_samp_factor as c_uint) as c_int,
                out_row_groups_avail.wrapping_mul((*compptr).v_samp_factor as c_uint) as c_int,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
        *out_row_group_ctr = out_row_groups_avail;
        break;
        /* can exit outer loop without test */
    }
}
/*
 * Process some data in the context case.
 */

unsafe extern "C" fn pre_process_context(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut in_row_ctr: *mut JDIMENSION,
    mut in_rows_avail: JDIMENSION,
    mut output_buf: JSAMPIMAGE,
    mut out_row_group_ctr: *mut JDIMENSION,
    mut out_row_groups_avail: JDIMENSION,
) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    let mut numrows: c_int = 0;
    let mut ci: c_int = 0;
    let mut buf_height: c_int = (*cinfo).max_v_samp_factor * 3i32;
    let mut inrows: JDIMENSION = 0;
    while *out_row_group_ctr < out_row_groups_avail {
        if *in_row_ctr < in_rows_avail {
            /* Do color conversion to fill the conversion buffer. */
            inrows = in_rows_avail.wrapping_sub(*in_row_ctr);
            numrows = (*prep).next_buf_stop - (*prep).next_buf_row;
            numrows = if (numrows as JDIMENSION) < inrows {
                numrows as JDIMENSION
            } else {
                inrows
            } as c_int;
            Some(
                (*(*cinfo).cconvert)
                    .color_convert
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                input_buf.offset(*in_row_ctr as isize),
                (*prep).color_buf.as_mut_ptr(),
                (*prep).next_buf_row as JDIMENSION,
                numrows,
            );
            /* Pad at top of image, if first time through */
            if (*prep).rows_to_go == (*cinfo).image_height {
                ci = 0i32;
                while ci < (*cinfo).num_components {
                    let mut row: c_int = 0;
                    row = 1i32;
                    while row <= (*cinfo).max_v_samp_factor {
                        jcopy_sample_rows(
                            (*prep).color_buf[ci as usize],
                            0i32,
                            (*prep).color_buf[ci as usize],
                            -row,
                            1i32,
                            (*cinfo).image_width,
                        );
                        row += 1
                    }
                    ci += 1
                }
            }
            *in_row_ctr =
                (*in_row_ctr as c_uint).wrapping_add(numrows as c_uint) as JDIMENSION as JDIMENSION;
            (*prep).next_buf_row += numrows;
            (*prep).rows_to_go = ((*prep).rows_to_go as c_uint).wrapping_sub(numrows as c_uint)
                as JDIMENSION as JDIMENSION
        } else {
            /* Return for more data, unless we are at the bottom of the image. */
            if (*prep).rows_to_go != 0i32 as c_uint {
                break;
            }
            /* When at bottom of image, pad to fill the conversion buffer. */
            if (*prep).next_buf_row < (*prep).next_buf_stop {
                ci = 0i32;
                while ci < (*cinfo).num_components {
                    expand_bottom_edge(
                        (*prep).color_buf[ci as usize],
                        (*cinfo).image_width,
                        (*prep).next_buf_row,
                        (*prep).next_buf_stop,
                    );
                    ci += 1
                }
                (*prep).next_buf_row = (*prep).next_buf_stop
            }
        }
        /* If we've gotten enough data, downsample a row group. */
        if (*prep).next_buf_row == (*prep).next_buf_stop {
            Some(
                (*(*cinfo).downsample)
                    .downsample
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*prep).color_buf.as_mut_ptr(),
                (*prep).this_row_group as JDIMENSION,
                output_buf,
                *out_row_group_ctr,
            );
            *out_row_group_ctr = (*out_row_group_ctr).wrapping_add(1);
            /* Advance pointers with wraparound as necessary. */
            (*prep).this_row_group += (*cinfo).max_v_samp_factor;
            if (*prep).this_row_group >= buf_height {
                (*prep).this_row_group = 0i32
            }
            if (*prep).next_buf_row >= buf_height {
                (*prep).next_buf_row = 0i32
            }
            (*prep).next_buf_stop = (*prep).next_buf_row + (*cinfo).max_v_samp_factor
        }
    }
}
/*
 * Create the wrapped-around downsampling input buffer needed for context mode.
 */

unsafe extern "C" fn create_context_buffer(mut cinfo: j_compress_ptr) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    let mut rgroup_height: c_int = (*cinfo).max_v_samp_factor;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut true_buffer: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut fake_buffer: JSAMPARRAY = 0 as *mut JSAMPROW;
    /* Grab enough space for fake row pointers for all the components;
     * we need five row groups' worth of pointers for each component.
     */
    fake_buffer = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (((*cinfo).num_components * 5i32 * rgroup_height) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<JSAMPROW>() as c_ulong),
    ) as JSAMPARRAY;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Allocate the actual buffer space (3 row groups) for this component.
         * We make the buffer wide enough to allow the downsampler to edge-expand
         * horizontally within the buffer, if it so chooses.
         */
        true_buffer = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*compptr).width_in_blocks as c_long
                * DCTSIZE as c_long
                * (*cinfo).max_h_samp_factor as c_long
                / (*compptr).h_samp_factor as c_long) as JDIMENSION,
            (3i32 * rgroup_height) as JDIMENSION,
        );
        /* point to space for next component */
        memcpy(
            fake_buffer.offset(rgroup_height as isize) as *mut c_void,
            true_buffer as *const c_void,
            ((3i32 * rgroup_height) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JSAMPROW>() as c_ulong),
        );
        i = 0i32;
        while i < rgroup_height {
            let ref mut fresh0 = *fake_buffer.offset(i as isize);
            *fresh0 = *true_buffer.offset((2i32 * rgroup_height + i) as isize);
            let ref mut fresh1 = *fake_buffer.offset((4i32 * rgroup_height + i) as isize);
            *fresh1 = *true_buffer.offset(i as isize);
            i += 1
        }
        (*prep).color_buf[ci as usize] = fake_buffer.offset(rgroup_height as isize);
        fake_buffer = fake_buffer.offset((5i32 * rgroup_height) as isize);
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/* Copy true buffer row pointers into the middle of the fake row array */
/* Fill in the above and below wraparound pointers */
/* CONTEXT_ROWS_SUPPORTED */
/*
 * Initialize preprocessing controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_prep_controller(
    mut cinfo: j_compress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut prep: my_prep_ptr = 0 as *mut my_prep_controller;
    let mut ci: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if need_full_buffer != 0 {
        /* safety check */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    prep = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_prep_controller>() as c_ulong,
    ) as my_prep_ptr;
    (*cinfo).prep = prep as *mut jpeg_c_prep_controller;
    (*prep).pub_0.start_pass =
        Some(start_pass_prep as unsafe extern "C" fn(_: j_compress_ptr, _: J_BUF_MODE) -> ());
    /* Allocate the color conversion buffer.
     * We make the buffer wide enough to allow the downsampler to edge-expand
     * horizontally within the buffer, if it so chooses.
     */
    if (*(*cinfo).downsample).need_context_rows != 0 {
        /* Set up to provide context rows */
        (*prep).pub_0.pre_process_data = Some(
            pre_process_context
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: JSAMPARRAY,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                    _: JSAMPIMAGE,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                ) -> (),
        );
        create_context_buffer(cinfo);
    } else {
        /* No context, just make it tall enough for one row group */
        (*prep).pub_0.pre_process_data = Some(
            pre_process_data
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: JSAMPARRAY,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                    _: JSAMPIMAGE,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                ) -> (),
        );
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            (*prep).color_buf[ci as usize] = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                ((*compptr).width_in_blocks as c_long
                    * DCTSIZE as c_long
                    * (*cinfo).max_h_samp_factor as c_long
                    / (*compptr).h_samp_factor as c_long) as JDIMENSION,
                (*cinfo).max_v_samp_factor as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
    };
}
