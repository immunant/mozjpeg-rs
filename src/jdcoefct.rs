pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jcopy_block_row;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::jround_up;
pub use crate::jpegint_h::jzero_far;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
pub use crate::jpeglib_h::D_MAX_BLOCKS_IN_MCU;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
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
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPEG_ROW_COMPLETED;
pub use crate::jpeglib_h::JPEG_SCAN_COMPLETED;
pub use crate::jpeglib_h::JPEG_SUSPENDED;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use libc;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
// =============== BEGIN jdcoefct_h ================
pub type my_coef_ptr = *mut my_coef_controller;
/*
 * jdcoefct.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 */

/* Block smoothing is only applicable for progressive JPEG, so: */

/* Private buffer controller object */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_coef_controller {
    pub pub_0: jpeg_d_coef_controller,
    pub MCU_ctr: JDIMENSION,
    pub MCU_vert_offset: c_int,
    pub MCU_rows_per_iMCU_row: c_int,
    pub MCU_buffer: [JBLOCKROW; 10],
    pub workspace: *mut JCOEF,
    pub whole_image: [jvirt_barray_ptr; 10],
    pub coef_bits_latch: *mut c_int,
}
pub unsafe extern "C" fn start_iMCU_row(mut cinfo: j_decompress_ptr) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    if (*cinfo).comps_in_scan > 1i32 {
        (*coef).MCU_rows_per_iMCU_row = 1i32
    } else if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint) {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0usize]).v_samp_factor
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0usize]).last_row_height
    }
    (*coef).MCU_ctr = 0i32 as JDIMENSION;
    (*coef).MCU_vert_offset = 0i32;
}
/* we save coef_bits[0..5] */
pub const SAVED_COEFS: c_int = 6i32;
/*
 * Initialize for an input processing pass.
 */
unsafe extern "C" fn start_input_pass(mut cinfo: j_decompress_ptr) {
    (*cinfo).input_iMCU_row = 0i32 as JDIMENSION;
    start_iMCU_row(cinfo);
}
/*
 * Initialize for an output processing pass.
 */
unsafe extern "C" fn start_output_pass(mut cinfo: j_decompress_ptr) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    if !(*coef).pub_0.coef_arrays.is_null() {
        if 0 != (*cinfo).do_block_smoothing && 0 != smoothing_ok(cinfo) {
            (*coef).pub_0.decompress_data = Some(
                decompress_smooth_data
                    as unsafe extern "C" fn(_: j_decompress_ptr, _: JSAMPIMAGE) -> c_int,
            )
        } else {
            (*coef).pub_0.decompress_data = Some(
                decompress_data
                    as unsafe extern "C" fn(_: j_decompress_ptr, _: JSAMPIMAGE) -> c_int,
            )
        }
    }
    (*cinfo).output_iMCU_row = 0i32 as JDIMENSION;
}
/*
 * jdcoefct.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2010, 2015-2016, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains the coefficient buffer controller for decompression.
 * This controller is the top level of the JPEG decompressor proper.
 * The coefficient buffer lies between entropy decoding and inverse-DCT steps.
 *
 * In buffered-image mode, this controller is the interface between
 * input-oriented processing and output-oriented processing.
 * Also, the input side (only) is used when reading a file for transcoding.
 */

/* Forward declarations */

/*
 * Decompress and return some data in the single-pass case.
 * Always attempts to emit one fully interleaved MCU row ("iMCU" row).
 * Input and output must run in lockstep since we have only a one-MCU buffer.
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 *
 * NB: output_buf contains a plane for each component in image,
 * which we index according to the component's SOF position.
 */
unsafe extern "C" fn decompress_onepass(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* index of current MCU within row */
    let mut MCU_col_num: JDIMENSION = 0;
    let mut last_MCU_col: JDIMENSION = (*cinfo).MCUs_per_row.wrapping_sub(1i32 as c_uint);
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint);
    let mut blkn: c_int = 0;
    let mut ci: c_int = 0;
    let mut xindex: c_int = 0;
    let mut yindex: c_int = 0;
    let mut yoffset: c_int = 0;
    let mut useful_width: c_int = 0;
    let mut output_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut start_col: JDIMENSION = 0;
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num <= last_MCU_col {
            jzero_far(
                (*coef).MCU_buffer[0usize] as *mut c_void,
                ((*cinfo).blocks_in_MCU as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
            );
            if 0 == (*(*cinfo).entropy)
                .decode_mcu
                .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) {
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return JPEG_SUSPENDED;
            }
            if MCU_col_num >= (*(*cinfo).master).first_iMCU_col
                && MCU_col_num <= (*(*cinfo).master).last_iMCU_col
            {
                blkn = 0i32;
                ci = 0i32;
                while ci < (*cinfo).comps_in_scan {
                    compptr = (*cinfo).cur_comp_info[ci as usize];
                    /* Don't bother to IDCT an uninteresting component. */
                    if 0 == (*compptr).component_needed {
                        blkn += (*compptr).MCU_blocks
                    } else {
                        inverse_DCT =
                            (*(*cinfo).idct).inverse_DCT[(*compptr).component_index as usize];
                        useful_width = if MCU_col_num < last_MCU_col {
                            (*compptr).MCU_width
                        } else {
                            (*compptr).last_col_width
                        };
                        output_ptr = (*output_buf.offset((*compptr).component_index as isize))
                            .offset((yoffset * (*compptr).DCT_scaled_size) as isize);
                        start_col = MCU_col_num
                            .wrapping_sub((*(*cinfo).master).first_iMCU_col)
                            .wrapping_mul((*compptr).MCU_sample_width as c_uint);
                        yindex = 0i32;
                        while yindex < (*compptr).MCU_height {
                            if (*cinfo).input_iMCU_row < last_iMCU_row
                                || yoffset + yindex < (*compptr).last_row_height
                            {
                                output_col = start_col;
                                xindex = 0i32;
                                while xindex < useful_width {
                                    inverse_DCT.expect("non-null function pointer")(
                                        cinfo,
                                        compptr,
                                        (*coef).MCU_buffer[(blkn + xindex) as usize] as JCOEFPTR,
                                        output_ptr,
                                        output_col,
                                    );
                                    output_col = (output_col as c_uint)
                                        .wrapping_add((*compptr).DCT_scaled_size as c_uint)
                                        as JDIMENSION
                                        as JDIMENSION;
                                    xindex += 1
                                }
                            }
                            blkn += (*compptr).MCU_width;
                            output_ptr = output_ptr.offset((*compptr).DCT_scaled_size as isize);
                            yindex += 1
                        }
                    }
                    ci += 1
                }
            }
            MCU_col_num = MCU_col_num.wrapping_add(1)
        }
        (*coef).MCU_ctr = 0i32 as JDIMENSION;
        yoffset += 1
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    (*cinfo).input_iMCU_row = (*cinfo).input_iMCU_row.wrapping_add(1);
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        start_iMCU_row(cinfo);
        return JPEG_ROW_COMPLETED;
    }
    (*(*cinfo).inputctl)
        .finish_input_pass
        .expect("non-null function pointer")(cinfo);
    return JPEG_SCAN_COMPLETED;
}
/*
 * Dummy consume-input routine for single-pass operation.
 */
unsafe extern "C" fn dummy_consume_data(mut _cinfo: j_decompress_ptr) -> c_int {
    return JPEG_SUSPENDED;
}
/*
 * Consume input data and store it in the full-image coefficient buffer.
 * We read as much as one fully interleaved MCU row ("iMCU" row) per call,
 * ie, v_samp_factor block rows for each component in the scan.
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 */
unsafe extern "C" fn consume_data(mut cinfo: j_decompress_ptr) -> c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* index of current MCU within row */
    let mut MCU_col_num: JDIMENSION = 0;
    let mut blkn: c_int = 0;
    let mut ci: c_int = 0;
    let mut xindex: c_int = 0;
    let mut yindex: c_int = 0;
    let mut yoffset: c_int = 0;
    let mut start_col: JDIMENSION = 0;
    let mut buffer: [JBLOCKARRAY; 4] = [0 as *mut JBLOCKROW; 4];
    let mut buffer_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = (*(*cinfo).mem)
            .access_virt_barray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            (*cinfo)
                .input_iMCU_row
                .wrapping_mul((*compptr).v_samp_factor as c_uint),
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        ci += 1
    }
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num < (*cinfo).MCUs_per_row {
            blkn = 0i32;
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                start_col = MCU_col_num.wrapping_mul((*compptr).MCU_width as c_uint);
                yindex = 0i32;
                while yindex < (*compptr).MCU_height {
                    buffer_ptr = (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                        .offset(start_col as isize);
                    xindex = 0i32;
                    while xindex < (*compptr).MCU_width {
                        let fresh1 = blkn;
                        blkn = blkn + 1;
                        let fresh0 = buffer_ptr;
                        buffer_ptr = buffer_ptr.offset(1);
                        (*coef).MCU_buffer[fresh1 as usize] = fresh0;
                        xindex += 1
                    }
                    yindex += 1
                }
                ci += 1
            }
            if 0 == (*(*cinfo).entropy)
                .decode_mcu
                .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) {
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return JPEG_SUSPENDED;
            }
            MCU_col_num = MCU_col_num.wrapping_add(1)
        }
        (*coef).MCU_ctr = 0i32 as JDIMENSION;
        yoffset += 1
    }
    (*cinfo).input_iMCU_row = (*cinfo).input_iMCU_row.wrapping_add(1);
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        start_iMCU_row(cinfo);
        return JPEG_ROW_COMPLETED;
    }
    (*(*cinfo).inputctl)
        .finish_input_pass
        .expect("non-null function pointer")(cinfo);
    return JPEG_SCAN_COMPLETED;
}
/*
 * Decompress and return some data in the multi-pass case.
 * Always attempts to emit one fully interleaved MCU row ("iMCU" row).
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 *
 * NB: output_buf contains a plane for each component in image.
 */
unsafe extern "C" fn decompress_data(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint);
    let mut block_num: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut block_row: c_int = 0;
    let mut block_rows: c_int = 0;
    let mut buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut buffer_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut output_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    while (*cinfo).input_scan_number < (*cinfo).output_scan_number
        || (*cinfo).input_scan_number == (*cinfo).output_scan_number
            && (*cinfo).input_iMCU_row <= (*cinfo).output_iMCU_row
    {
        if (*(*cinfo).inputctl)
            .consume_input
            .expect("non-null function pointer")(cinfo)
            == JPEG_SUSPENDED
        {
            return JPEG_SUSPENDED;
        }
    }
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Don't bother to IDCT an uninteresting component. */
        if !(0 == (*compptr).component_needed) {
            buffer = (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                (*coef).whole_image[ci as usize],
                (*cinfo)
                    .output_iMCU_row
                    .wrapping_mul((*compptr).v_samp_factor as c_uint),
                (*compptr).v_samp_factor as JDIMENSION,
                FALSE,
            );
            if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor
            } else {
                block_rows = (*compptr)
                    .height_in_blocks
                    .wrapping_rem((*compptr).v_samp_factor as c_uint)
                    as c_int;
                if block_rows == 0i32 {
                    block_rows = (*compptr).v_samp_factor
                }
            }
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            block_row = 0i32;
            while block_row < block_rows {
                buffer_ptr = (*buffer.offset(block_row as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                output_col = 0i32 as JDIMENSION;
                block_num = (*(*cinfo).master).first_MCU_col[ci as usize];
                while block_num <= (*(*cinfo).master).last_MCU_col[ci as usize] {
                    inverse_DCT.expect("non-null function pointer")(
                        cinfo,
                        compptr,
                        buffer_ptr as JCOEFPTR,
                        output_ptr,
                        output_col,
                    );
                    buffer_ptr = buffer_ptr.offset(1isize);
                    output_col = (output_col as c_uint)
                        .wrapping_add((*compptr).DCT_scaled_size as c_uint)
                        as JDIMENSION as JDIMENSION;
                    block_num = block_num.wrapping_add(1)
                }
                output_ptr = output_ptr.offset((*compptr).DCT_scaled_size as isize);
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return JPEG_ROW_COMPLETED;
    }
    return JPEG_SCAN_COMPLETED;
}
/* D_MULTISCAN_FILES_SUPPORTED */

/*
 * This code applies interblock smoothing as described by section K.8
 * of the JPEG standard: the first 5 AC coefficients are estimated from
 * the DC values of a DCT block and its 8 neighboring blocks.
 * We apply smoothing only for progressive JPEG decoding, and only if
 * the coefficients it can estimate are not yet known to full precision.
 */

/* Natural-order array positions of the first 5 zigzag-order coefficients */
pub const Q01_POS: c_int = 1i32;
pub const Q10_POS: c_int = 8i32;
pub const Q20_POS: c_int = 16i32;
pub const Q11_POS: c_int = 9i32;
pub const Q02_POS: c_int = 2i32;
/*
 * Determine whether block smoothing is applicable and safe.
 * We also latch the current states of the coef_bits[] entries for the
 * AC coefficients; otherwise, if the input side of the decompressor
 * advances into a new scan, we might think the coefficients are known
 * more accurately than they really are.
 */
unsafe extern "C" fn smoothing_ok(mut cinfo: j_decompress_ptr) -> boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut smoothing_useful: boolean = FALSE;
    let mut ci: c_int = 0;
    let mut coefi: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut qtable: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut coef_bits: *mut c_int = 0 as *mut c_int;
    let mut coef_bits_latch: *mut c_int = 0 as *mut c_int;
    if 0 == (*cinfo).progressive_mode || (*cinfo).coef_bits.is_null() {
        return FALSE;
    }
    if (*coef).coef_bits_latch.is_null() {
        (*coef).coef_bits_latch = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*cinfo).num_components as c_ulong).wrapping_mul(
                (SAVED_COEFS as c_ulong).wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong),
            ),
        ) as *mut c_int
    }
    coef_bits_latch = (*coef).coef_bits_latch;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        qtable = (*compptr).quant_table;
        if qtable.is_null() {
            return FALSE;
        }
        if (*qtable).quantval[0usize] as c_int == 0i32
            || (*qtable).quantval[Q01_POS as usize] as c_int == 0i32
            || (*qtable).quantval[Q10_POS as usize] as c_int == 0i32
            || (*qtable).quantval[Q20_POS as usize] as c_int == 0i32
            || (*qtable).quantval[Q11_POS as usize] as c_int == 0i32
            || (*qtable).quantval[Q02_POS as usize] as c_int == 0i32
        {
            return FALSE;
        }
        coef_bits = (*(*cinfo).coef_bits.offset(ci as isize)).as_mut_ptr();
        if *coef_bits.offset(0isize) < 0i32 {
            return FALSE;
        }
        coefi = 1i32;
        while coefi <= 5i32 {
            *coef_bits_latch.offset(coefi as isize) = *coef_bits.offset(coefi as isize);
            if *coef_bits.offset(coefi as isize) != 0i32 {
                smoothing_useful = TRUE
            }
            coefi += 1
        }
        coef_bits_latch = coef_bits_latch.offset(SAVED_COEFS as isize);
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    return smoothing_useful;
}
/*
 * Variant of decompress_data for use when doing block smoothing.
 */
unsafe extern "C" fn decompress_smooth_data(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint);
    let mut block_num: JDIMENSION = 0;
    let mut last_block_column: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut block_row: c_int = 0;
    let mut block_rows: c_int = 0;
    let mut access_rows: c_int = 0;
    let mut buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut buffer_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut prev_block_row: JBLOCKROW = 0 as *mut JBLOCK;
    let mut next_block_row: JBLOCKROW = 0 as *mut JBLOCK;
    let mut output_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    let mut first_row: boolean = 0;
    let mut last_row: boolean = 0;
    let mut workspace: *mut JCOEF = 0 as *mut JCOEF;
    let mut coef_bits: *mut c_int = 0 as *mut c_int;
    let mut quanttbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut Q00: JLONG = 0;
    let mut Q01: JLONG = 0;
    let mut Q02: JLONG = 0;
    let mut Q10: JLONG = 0;
    let mut Q11: JLONG = 0;
    let mut Q20: JLONG = 0;
    let mut num: JLONG = 0;
    let mut DC1: c_int = 0;
    let mut DC2: c_int = 0;
    let mut DC3: c_int = 0;
    let mut DC4: c_int = 0;
    let mut DC5: c_int = 0;
    let mut DC6: c_int = 0;
    let mut DC7: c_int = 0;
    let mut DC8: c_int = 0;
    let mut DC9: c_int = 0;
    let mut Al: c_int = 0;
    let mut pred: c_int = 0;
    workspace = (*coef).workspace;
    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && 0 == (*(*cinfo).inputctl).eoi_reached
    {
        if (*cinfo).input_scan_number == (*cinfo).output_scan_number {
            /* If input is working on current scan, we ordinarily want it to
             * have completed the current row.  But if input scan is DC,
             * we want it to keep one row ahead so that next block row's DC
             * values are up to date.
             */
            let mut delta: JDIMENSION =
                (if (*cinfo).Ss == 0i32 { 1i32 } else { 0i32 }) as JDIMENSION;
            if (*cinfo).input_iMCU_row > (*cinfo).output_iMCU_row.wrapping_add(delta) {
                break;
            }
        }
        if (*(*cinfo).inputctl)
            .consume_input
            .expect("non-null function pointer")(cinfo)
            == JPEG_SUSPENDED
        {
            return JPEG_SUSPENDED;
        }
    }
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Don't bother to IDCT an uninteresting component. */
        if !(0 == (*compptr).component_needed) {
            if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor;
                access_rows = block_rows * 2i32;
                last_row = FALSE
            } else {
                block_rows = (*compptr)
                    .height_in_blocks
                    .wrapping_rem((*compptr).v_samp_factor as c_uint)
                    as c_int;
                if block_rows == 0i32 {
                    block_rows = (*compptr).v_samp_factor
                }
                access_rows = block_rows;
                last_row = TRUE
            }
            if (*cinfo).output_iMCU_row > 0i32 as c_uint {
                access_rows += (*compptr).v_samp_factor;
                buffer = (*(*cinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    (*cinfo)
                        .output_iMCU_row
                        .wrapping_sub(1i32 as c_uint)
                        .wrapping_mul((*compptr).v_samp_factor as c_uint),
                    access_rows as JDIMENSION,
                    FALSE,
                );
                buffer = buffer.offset((*compptr).v_samp_factor as isize);
                first_row = FALSE
            } else {
                buffer = (*(*cinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    0i32 as JDIMENSION,
                    access_rows as JDIMENSION,
                    FALSE,
                );
                first_row = TRUE
            }
            coef_bits = (*coef).coef_bits_latch.offset((ci * SAVED_COEFS) as isize);
            quanttbl = (*compptr).quant_table;
            Q00 = (*quanttbl).quantval[0usize] as JLONG;
            Q01 = (*quanttbl).quantval[Q01_POS as usize] as JLONG;
            Q10 = (*quanttbl).quantval[Q10_POS as usize] as JLONG;
            Q20 = (*quanttbl).quantval[Q20_POS as usize] as JLONG;
            Q11 = (*quanttbl).quantval[Q11_POS as usize] as JLONG;
            Q02 = (*quanttbl).quantval[Q02_POS as usize] as JLONG;
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            block_row = 0i32;
            while block_row < block_rows {
                buffer_ptr = (*buffer.offset(block_row as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                if 0 != first_row && block_row == 0i32 {
                    prev_block_row = buffer_ptr
                } else {
                    prev_block_row = *buffer.offset((block_row - 1i32) as isize)
                }
                if 0 != last_row && block_row == block_rows - 1i32 {
                    next_block_row = buffer_ptr
                } else {
                    next_block_row = *buffer.offset((block_row + 1i32) as isize)
                }
                DC3 = (*prev_block_row.offset(0isize))[0usize] as c_int;
                DC2 = DC3;
                DC1 = DC2;
                DC6 = (*buffer_ptr.offset(0isize))[0usize] as c_int;
                DC5 = DC6;
                DC4 = DC5;
                DC9 = (*next_block_row.offset(0isize))[0usize] as c_int;
                DC8 = DC9;
                DC7 = DC8;
                output_col = 0i32 as JDIMENSION;
                last_block_column = (*compptr).width_in_blocks.wrapping_sub(1i32 as c_uint);
                block_num = (*(*cinfo).master).first_MCU_col[ci as usize];
                while block_num <= (*(*cinfo).master).last_MCU_col[ci as usize] {
                    jcopy_block_row(buffer_ptr, workspace as JBLOCKROW, 1i32 as JDIMENSION);
                    if block_num < last_block_column {
                        DC3 = (*prev_block_row.offset(1isize))[0usize] as c_int;
                        DC6 = (*buffer_ptr.offset(1isize))[0usize] as c_int;
                        DC9 = (*next_block_row.offset(1isize))[0usize] as c_int
                    }
                    Al = *coef_bits.offset(1isize);
                    if Al != 0i32 && *workspace.offset(1isize) as c_int == 0i32 {
                        num = 36i32 as c_long * Q00 * (DC4 - DC6) as c_long;
                        if num >= 0i32 as c_long {
                            pred = (((Q01 << 7i32) + num) / (Q01 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q01 << 7i32) - num) / (Q01 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(1isize) = pred as JCOEF
                    }
                    Al = *coef_bits.offset(2isize);
                    if Al != 0i32 && *workspace.offset(8isize) as c_int == 0i32 {
                        num = 36i32 as c_long * Q00 * (DC2 - DC8) as c_long;
                        if num >= 0i32 as c_long {
                            pred = (((Q10 << 7i32) + num) / (Q10 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q10 << 7i32) - num) / (Q10 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(8isize) = pred as JCOEF
                    }
                    Al = *coef_bits.offset(3isize);
                    if Al != 0i32 && *workspace.offset(16isize) as c_int == 0i32 {
                        num = 9i32 as c_long * Q00 * (DC2 + DC8 - 2i32 * DC5) as c_long;
                        if num >= 0i32 as c_long {
                            pred = (((Q20 << 7i32) + num) / (Q20 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q20 << 7i32) - num) / (Q20 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(16isize) = pred as JCOEF
                    }
                    Al = *coef_bits.offset(4isize);
                    if Al != 0i32 && *workspace.offset(9isize) as c_int == 0i32 {
                        num = 5i32 as c_long * Q00 * (DC1 - DC3 - DC7 + DC9) as c_long;
                        if num >= 0i32 as c_long {
                            pred = (((Q11 << 7i32) + num) / (Q11 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q11 << 7i32) - num) / (Q11 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(9isize) = pred as JCOEF
                    }
                    Al = *coef_bits.offset(5isize);
                    if Al != 0i32 && *workspace.offset(2isize) as c_int == 0i32 {
                        num = 9i32 as c_long * Q00 * (DC4 + DC6 - 2i32 * DC5) as c_long;
                        if num >= 0i32 as c_long {
                            pred = (((Q02 << 7i32) + num) / (Q02 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q02 << 7i32) - num) / (Q02 << 8i32)) as c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(2isize) = pred as JCOEF
                    }
                    inverse_DCT.expect("non-null function pointer")(
                        cinfo, compptr, workspace, output_ptr, output_col,
                    );
                    DC1 = DC2;
                    DC2 = DC3;
                    DC4 = DC5;
                    DC5 = DC6;
                    DC7 = DC8;
                    DC8 = DC9;
                    buffer_ptr = buffer_ptr.offset(1isize);
                    prev_block_row = prev_block_row.offset(1isize);
                    next_block_row = next_block_row.offset(1isize);
                    output_col = (output_col as c_uint)
                        .wrapping_add((*compptr).DCT_scaled_size as c_uint)
                        as JDIMENSION as JDIMENSION;
                    block_num = block_num.wrapping_add(1)
                }
                output_ptr = output_ptr.offset((*compptr).DCT_scaled_size as isize);
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return JPEG_ROW_COMPLETED;
    }
    return JPEG_SCAN_COMPLETED;
}
/* BLOCK_SMOOTHING_SUPPORTED */

/*
 * Initialize coefficient buffer controller.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_d_coef_controller(
    mut cinfo: j_decompress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut coef: my_coef_ptr = 0 as *mut my_coef_controller;
    coef = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_coef_controller>() as c_ulong,
    ) as my_coef_ptr;
    (*cinfo).coef = coef as *mut jpeg_d_coef_controller;
    (*coef).pub_0.start_input_pass =
        Some(start_input_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*coef).pub_0.start_output_pass =
        Some(start_output_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*coef).coef_bits_latch = NULL as *mut c_int;
    if 0 != need_full_buffer {
        let mut ci: c_int = 0;
        let mut access_rows: c_int = 0;
        let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            access_rows = (*compptr).v_samp_factor;
            if 0 != (*cinfo).progressive_mode {
                access_rows *= 3i32
            }
            (*coef).whole_image[ci as usize] = (*(*cinfo).mem)
                .request_virt_barray
                .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                TRUE,
                jround_up(
                    (*compptr).width_in_blocks as c_long,
                    (*compptr).h_samp_factor as c_long,
                ) as JDIMENSION,
                jround_up(
                    (*compptr).height_in_blocks as c_long,
                    (*compptr).v_samp_factor as c_long,
                ) as JDIMENSION,
                access_rows as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1isize)
        }
        (*coef).pub_0.consume_data =
            Some(consume_data as unsafe extern "C" fn(_: j_decompress_ptr) -> c_int);
        (*coef).pub_0.decompress_data = Some(
            decompress_data as unsafe extern "C" fn(_: j_decompress_ptr, _: JSAMPIMAGE) -> c_int,
        );
        (*coef).pub_0.coef_arrays = (*coef).whole_image.as_mut_ptr()
    } else {
        let mut buffer: JBLOCKROW = 0 as *mut JBLOCK;
        let mut i: c_int = 0;
        buffer = (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (D_MAX_BLOCKS_IN_MCU as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
        ) as JBLOCKROW;
        i = 0i32;
        while i < D_MAX_BLOCKS_IN_MCU {
            (*coef).MCU_buffer[i as usize] = buffer.offset(i as isize);
            i += 1
        }
        (*coef).pub_0.consume_data =
            Some(dummy_consume_data as unsafe extern "C" fn(_: j_decompress_ptr) -> c_int);
        (*coef).pub_0.decompress_data = Some(
            decompress_onepass as unsafe extern "C" fn(_: j_decompress_ptr, _: JSAMPIMAGE) -> c_int,
        );
        (*coef).pub_0.coef_arrays = NULL as *mut jvirt_barray_ptr
    }
    (*coef).workspace = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (::std::mem::size_of::<JCOEF>() as c_ulong).wrapping_mul(DCTSIZE2 as c_ulong),
    ) as *mut JCOEF;
}
