pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JBLOCKROW;
use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jcopy_block_row;
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
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
pub use crate::jpeglib_h::D_MAX_BLOCKS_IN_MCU;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
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
// =============== BEGIN jdcoefct_h ================
pub type my_coef_ptr = *mut crate::src::jdcoefct::my_coef_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_coef_controller {
    pub pub_0: crate::jpeglib_h::jpeg_d_coef_controller,
    pub MCU_ctr: crate::jmorecfg_h::JDIMENSION,
    pub MCU_vert_offset: libc::c_int,
    pub MCU_rows_per_iMCU_row: libc::c_int,
    pub MCU_buffer: [crate::jpeglib_h::JBLOCKROW; 10],
    pub workspace: *mut crate::jmorecfg_h::JCOEF,
    pub whole_image: [crate::jpeglib_h::jvirt_barray_ptr; 10],
    pub coef_bits_latch: *mut libc::c_int,
}

pub unsafe extern "C" fn start_iMCU_row(mut cinfo: crate::jpeglib_h::j_decompress_ptr)
/* Reset within-iMCU-row counters for a new row (input side) */
{
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr;
    /* In an interleaved scan, an MCU row is the same as an iMCU row.
     * In a noninterleaved scan, an iMCU row has v_samp_factor MCU rows.
     * But at the bottom of the image, process only what's left.
     */
    if (*cinfo).comps_in_scan > 1i32 {
        (*coef).MCU_rows_per_iMCU_row = 1i32
    } else if (*cinfo).input_iMCU_row <  (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint
    {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).v_samp_factor
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).last_row_height
    }
    (*coef).MCU_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
    (*coef).MCU_vert_offset = 0i32;
}
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
/* public fields */
/* These variables keep track of the current location of the input side. */
/* cinfo->input_iMCU_row is also used for this. */
/* counts MCUs processed in current row */
/* counts MCU rows within iMCU row */
/* number of such rows needed */
/* The output side's location is represented by cinfo->output_iMCU_row. */
/* In single-pass modes, it's sufficient to buffer just one MCU.
 * We allocate a workspace of D_MAX_BLOCKS_IN_MCU coefficient blocks,
 * and let the entropy decoder write into that workspace each time.
 * In multi-pass modes, this array points to the current MCU's blocks
 * within the virtual arrays; it is used only by the input side.
 */
/* Temporary workspace for one MCU */
/* In multi-pass modes, we need a virtual block array for each component. */
/* When doing block smoothing, we latch coefficient Al values here */

pub const SAVED_COEFS: libc::c_int = 6i32;
/*
 * Initialize for an input processing pass.
 */

unsafe extern "C" fn start_input_pass(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    (*cinfo).input_iMCU_row = 0i32 as crate::jmorecfg_h::JDIMENSION;
    crate::src::jdcoefct::start_iMCU_row(cinfo);
}
/*
 * Initialize for an output processing pass.
 */

unsafe extern "C" fn start_output_pass(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr;
    /* If multipass, check to see whether to use block smoothing on this pass */
    if !(*coef).pub_0.coef_arrays.is_null() {
        if (*cinfo).do_block_smoothing != 0 && smoothing_ok(cinfo) != 0 {
            (*coef).pub_0.decompress_data = Some(
                decompress_smooth_data
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                    ) -> libc::c_int,
            )
        } else {
            (*coef).pub_0.decompress_data = Some(
                decompress_data
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                    ) -> libc::c_int,
            )
        }
    }
    (*cinfo).output_iMCU_row = 0i32 as crate::jmorecfg_h::JDIMENSION;
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> libc::c_int {
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr; /* index of current MCU within row */
    let mut MCU_col_num: crate::jmorecfg_h::JDIMENSION = 0;
    let mut last_MCU_col: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).MCUs_per_row - 1i32 as libc::c_uint;
    let mut last_iMCU_row: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint;
    let mut blkn: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut xindex: libc::c_int = 0;
    let mut yindex: libc::c_int = 0;
    let mut yoffset: libc::c_int = 0;
    let mut useful_width: libc::c_int = 0;
    let mut output_ptr: crate::jpeglib_h::JSAMPARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JSAMPROW>();
    let mut start_col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut output_col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut inverse_DCT: crate::jpegint_h::inverse_DCT_method_ptr = None;
    /* Loop to process as much as one whole iMCU row */
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num <= last_MCU_col {
            /* Try to fetch an MCU.  Entropy decoder expects buffer to be zeroed. */
            crate::jpegint_h::jzero_far(
                (*coef).MCU_buffer[0] as *mut libc::c_void,
                (*cinfo).blocks_in_MCU as libc::c_ulong *
    
                        ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
            );
            if Some(
                (*(*cinfo).entropy)
                    .decode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return crate::jpeglib_h::JPEG_SUSPENDED;
            }
            /* Only perform the IDCT on blocks that are contained within the desired
             * cropping region.
             */
            if MCU_col_num >= (*(*cinfo).master).first_iMCU_col
                && MCU_col_num <= (*(*cinfo).master).last_iMCU_col
            {
                /* Determine where data should go in output_buf and do the IDCT thing.
                 * We skip dummy blocks at the right and bottom edges (but blkn gets
                 * incremented past them!).  Note the inner loop relies on having
                 * allocated the MCU_buffer[] blocks sequentially.
                 */
                blkn = 0i32; /* index of current DCT block within MCU */
                ci = 0i32;
                while ci < (*cinfo).comps_in_scan {
                    compptr = (*cinfo).cur_comp_info[ci as usize];
                    /* Don't bother to IDCT an uninteresting component. */
                    if (*compptr).component_needed == 0 {
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
                        start_col = ( MCU_col_num - (*(*cinfo).master).first_iMCU_col) *
    (*compptr).MCU_sample_width as libc::c_uint;
                        yindex = 0i32;
                        while yindex < (*compptr).MCU_height {
                            if (*cinfo).input_iMCU_row < last_iMCU_row
                                || yoffset + yindex < (*compptr).last_row_height
                            {
                                output_col = start_col;
                                xindex = 0i32;
                                while xindex < useful_width {
                                    Some(inverse_DCT.expect("non-null function pointer"))
                                        .expect("non-null function pointer")(
                                        cinfo,
                                        compptr,
                                        (*coef).MCU_buffer[(blkn + xindex) as usize]
                                            as crate::jpeglib_h::JCOEFPTR,
                                        output_ptr,
                                        output_col,
                                    );
                                    output_col = (output_col as libc::c_uint + (*compptr).DCT_scaled_size as libc::c_uint)
                                        as crate::jmorecfg_h::JDIMENSION
                                        as crate::jmorecfg_h::JDIMENSION;
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
            MCU_col_num =  MCU_col_num + 1
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).MCU_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*cinfo).output_iMCU_row =  (*cinfo).output_iMCU_row + 1;
    (*cinfo).input_iMCU_row =  (*cinfo).input_iMCU_row + 1;
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        crate::src::jdcoefct::start_iMCU_row(cinfo);
        return crate::jpeglib_h::JPEG_ROW_COMPLETED;
    }
    /* Completed the scan */
    Some(
        (*(*cinfo).inputctl)
            .finish_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    return crate::jpeglib_h::JPEG_SCAN_COMPLETED;
}
/*
 * Dummy consume-input routine for single-pass operation.
 */

unsafe extern "C" fn dummy_consume_data(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> libc::c_int {
    return crate::jpeglib_h::JPEG_SUSPENDED;
    /* Always indicate nothing was done */
}
/*
 * Consume input data and store it in the full-image coefficient buffer.
 * We read as much as one fully interleaved MCU row ("iMCU" row) per call,
 * ie, v_samp_factor block rows for each component in the scan.
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 */

unsafe extern "C" fn consume_data(mut cinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int {
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr; /* index of current MCU within row */
    let mut MCU_col_num: crate::jmorecfg_h::JDIMENSION = 0;
    let mut blkn: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut xindex: libc::c_int = 0;
    let mut yindex: libc::c_int = 0;
    let mut yoffset: libc::c_int = 0;
    let mut start_col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut buffer: [crate::jpeglib_h::JBLOCKARRAY; 4] = [::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>(); 4];
    let mut buffer_ptr: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    /* Align the virtual buffers for the components used in this scan. */
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            
            (*cinfo)
                .input_iMCU_row * (*compptr).v_samp_factor as libc::c_uint,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::TRUE,
        );
        ci += 1
        /* Note: entropy decoder expects buffer to be zeroed,
         * but this is handled automatically by the memory manager
         * because we requested a pre-zeroed array.
         */
    }
    /* Loop to process one whole iMCU row */
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num < (*cinfo).MCUs_per_row {
            /* Construct list of pointers to DCT blocks belonging to this MCU */
            blkn = 0i32; /* index of current DCT block within MCU */
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                start_col =  MCU_col_num * (*compptr).MCU_width as libc::c_uint;
                yindex = 0i32;
                while yindex < (*compptr).MCU_height {
                    buffer_ptr = (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                        .offset(start_col as isize);
                    xindex = 0i32;
                    while xindex < (*compptr).MCU_width {
                        let fresh0 = buffer_ptr;
                        buffer_ptr = buffer_ptr.offset(1);
                        let fresh1 = blkn;
                        blkn = blkn + 1;
                        (*coef).MCU_buffer[fresh1 as usize] = fresh0;
                        xindex += 1
                    }
                    yindex += 1
                }
                ci += 1
            }
            /* Try to fetch the MCU. */
            if Some(
                (*(*cinfo).entropy)
                    .decode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return crate::jpeglib_h::JPEG_SUSPENDED;
            }
            MCU_col_num =  MCU_col_num + 1
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).MCU_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*cinfo).input_iMCU_row =  (*cinfo).input_iMCU_row + 1;
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        crate::src::jdcoefct::start_iMCU_row(cinfo);
        return crate::jpeglib_h::JPEG_ROW_COMPLETED;
    }
    /* Completed the scan */
    Some(
        (*(*cinfo).inputctl)
            .finish_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    return crate::jpeglib_h::JPEG_SCAN_COMPLETED;
}
/*
 * Decompress and return some data in the multi-pass case.
 * Always attempts to emit one fully interleaved MCU row ("iMCU" row).
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 *
 * NB: output_buf contains a plane for each component in image.
 */

unsafe extern "C" fn decompress_data(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> libc::c_int {
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr;
    let mut last_iMCU_row: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint;
    let mut block_num: crate::jmorecfg_h::JDIMENSION = 0;
    let mut ci: libc::c_int = 0;
    let mut block_row: libc::c_int = 0;
    let mut block_rows: libc::c_int = 0;
    let mut buffer: crate::jpeglib_h::JBLOCKARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();
    let mut buffer_ptr: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut output_ptr: crate::jpeglib_h::JSAMPARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JSAMPROW>();
    let mut output_col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut inverse_DCT: crate::jpegint_h::inverse_DCT_method_ptr = None;
    /* Force some input to be done if we are getting ahead of the input. */
    while (*cinfo).input_scan_number < (*cinfo).output_scan_number
        || (*cinfo).input_scan_number == (*cinfo).output_scan_number
            && (*cinfo).input_iMCU_row <= (*cinfo).output_iMCU_row
    {
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == crate::jpeglib_h::JPEG_SUSPENDED
        {
            return crate::jpeglib_h::JPEG_SUSPENDED;
        }
    }
    /* OK, output from the virtual arrays. */
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Don't bother to IDCT an uninteresting component. */
        if !((*compptr).component_needed == 0) {
            /* Align the virtual buffer for this component. */
            buffer = Some(
                (*(*cinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                (*coef).whole_image[ci as usize],
                
                (*cinfo)
                    .output_iMCU_row * (*compptr).v_samp_factor as libc::c_uint,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::FALSE,
            );
            /* Count non-dummy DCT block rows in this iMCU row. */
            if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor
            } else {
                /* NB: can't use last_row_height here; it is input-side-dependent! */
                block_rows = ( (*compptr)
                    .height_in_blocks % (*compptr).v_samp_factor as libc::c_uint)
                    as libc::c_int;
                if block_rows == 0i32 {
                    block_rows = (*compptr).v_samp_factor
                }
            }
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            /* Loop over all DCT blocks to be processed. */
            block_row = 0i32;
            while block_row < block_rows {
                buffer_ptr = (*buffer.offset(block_row as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                output_col = 0i32 as crate::jmorecfg_h::JDIMENSION;
                block_num = (*(*cinfo).master).first_MCU_col[ci as usize];
                while block_num <= (*(*cinfo).master).last_MCU_col[ci as usize] {
                    Some(inverse_DCT.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        cinfo,
                        compptr,
                        buffer_ptr as crate::jpeglib_h::JCOEFPTR,
                        output_ptr,
                        output_col,
                    );
                    buffer_ptr = buffer_ptr.offset(1);
                    output_col = (output_col as libc::c_uint + (*compptr).DCT_scaled_size as libc::c_uint)
                        as crate::jmorecfg_h::JDIMENSION
                        as crate::jmorecfg_h::JDIMENSION;
                    block_num =  block_num + 1
                }
                output_ptr = output_ptr.offset((*compptr).DCT_scaled_size as isize);
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
    (*cinfo).output_iMCU_row =  (*cinfo).output_iMCU_row + 1;
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return crate::jpeglib_h::JPEG_ROW_COMPLETED;
    }
    return crate::jpeglib_h::JPEG_SCAN_COMPLETED;
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

pub const Q01_POS: libc::c_int = 1i32;

pub const Q10_POS: libc::c_int = 8i32;

pub const Q20_POS: libc::c_int = 16i32;

pub const Q11_POS: libc::c_int = 9i32;

pub const Q02_POS: libc::c_int = 2i32;
/*
 * Determine whether block smoothing is applicable and safe.
 * We also latch the current states of the coef_bits[] entries for the
 * AC coefficients; otherwise, if the input side of the decompressor
 * advances into a new scan, we might think the coefficients are known
 * more accurately than they really are.
 */

unsafe extern "C" fn smoothing_ok(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr;
    let mut smoothing_useful: crate::jmorecfg_h::boolean = crate::jmorecfg_h::FALSE;
    let mut ci: libc::c_int = 0;
    let mut coefi: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut qtable: *mut crate::jpeglib_h::JQUANT_TBL = ::std::ptr::null_mut::< crate::jpeglib_h::JQUANT_TBL>();
    let mut coef_bits: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut coef_bits_latch: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    if (*cinfo).progressive_mode == 0 || (*cinfo).coef_bits.is_null() {
        return crate::jmorecfg_h::FALSE;
    }
    /* Allocate latch area if not already done */
    if (*coef).coef_bits_latch.is_null() {
        (*coef).coef_bits_latch = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            (*cinfo).num_components as libc::c_ulong *
    (crate::src::jdcoefct::SAVED_COEFS as libc::c_ulong *
         ::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int
    }
    coef_bits_latch = (*coef).coef_bits_latch;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* All components' quantization values must already be latched. */
        qtable = (*compptr).quant_table;
        if qtable.is_null() {
            return crate::jmorecfg_h::FALSE;
        }
        /* Verify DC & first 5 AC quantizers are nonzero to avoid zero-divide. */
        if (*qtable).quantval[0] as libc::c_int == 0i32
            || (*qtable).quantval[Q01_POS as usize] as libc::c_int == 0i32
            || (*qtable).quantval[Q10_POS as usize] as libc::c_int == 0i32
            || (*qtable).quantval[Q20_POS as usize] as libc::c_int == 0i32
            || (*qtable).quantval[Q11_POS as usize] as libc::c_int == 0i32
            || (*qtable).quantval[Q02_POS as usize] as libc::c_int == 0i32
        {
            return crate::jmorecfg_h::FALSE;
        }
        /* DC values must be at least partly known for all components. */
        coef_bits = (*(*cinfo).coef_bits.offset(ci as isize)).as_mut_ptr();
        if *coef_bits.offset(0) < 0i32 {
            return crate::jmorecfg_h::FALSE;
        }
        /* Block smoothing is helpful if some AC coefficients remain inaccurate. */
        coefi = 1i32;
        while coefi <= 5i32 {
            *coef_bits_latch.offset(coefi as isize) = *coef_bits.offset(coefi as isize);
            if *coef_bits.offset(coefi as isize) != 0i32 {
                smoothing_useful = crate::jmorecfg_h::TRUE
            }
            coefi += 1
        }
        coef_bits_latch = coef_bits_latch.offset(crate::src::jdcoefct::SAVED_COEFS as isize);
        ci += 1;
        compptr = compptr.offset(1)
    }
    return smoothing_useful;
}
/*
 * Variant of decompress_data for use when doing block smoothing.
 */

unsafe extern "C" fn decompress_smooth_data(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> libc::c_int {
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr;
    let mut last_iMCU_row: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint;
    let mut block_num: crate::jmorecfg_h::JDIMENSION = 0;
    let mut last_block_column: crate::jmorecfg_h::JDIMENSION = 0;
    let mut ci: libc::c_int = 0;
    let mut block_row: libc::c_int = 0;
    let mut block_rows: libc::c_int = 0;
    let mut access_rows: libc::c_int = 0;
    let mut buffer: crate::jpeglib_h::JBLOCKARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();
    let mut buffer_ptr: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut prev_block_row: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut next_block_row: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut output_ptr: crate::jpeglib_h::JSAMPARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JSAMPROW>();
    let mut output_col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut inverse_DCT: crate::jpegint_h::inverse_DCT_method_ptr = None;
    let mut first_row: crate::jmorecfg_h::boolean = 0;
    let mut last_row: crate::jmorecfg_h::boolean = 0;
    let mut workspace: *mut crate::jmorecfg_h::JCOEF = ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();
    let mut coef_bits: *mut libc::c_int = ::std::ptr::null_mut::< libc::c_int>();
    let mut quanttbl: *mut crate::jpeglib_h::JQUANT_TBL = ::std::ptr::null_mut::< crate::jpeglib_h::JQUANT_TBL>();
    let mut Q00: crate::jpegint_h::JLONG = 0;
    let mut Q01: crate::jpegint_h::JLONG = 0;
    let mut Q02: crate::jpegint_h::JLONG = 0;
    let mut Q10: crate::jpegint_h::JLONG = 0;
    let mut Q11: crate::jpegint_h::JLONG = 0;
    let mut Q20: crate::jpegint_h::JLONG = 0;
    let mut num: crate::jpegint_h::JLONG = 0;
    let mut DC1: libc::c_int = 0;
    let mut DC2: libc::c_int = 0;
    let mut DC3: libc::c_int = 0;
    let mut DC4: libc::c_int = 0;
    let mut DC5: libc::c_int = 0;
    let mut DC6: libc::c_int = 0;
    let mut DC7: libc::c_int = 0;
    let mut DC8: libc::c_int = 0;
    let mut DC9: libc::c_int = 0;
    let mut Al: libc::c_int = 0;
    let mut pred: libc::c_int = 0;
    /* Keep a local variable to avoid looking it up more than once */
    workspace = (*coef).workspace;
    /* Force some input to be done if we are getting ahead of the input. */
    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && (*(*cinfo).inputctl).eoi_reached == 0
    {
        if (*cinfo).input_scan_number == (*cinfo).output_scan_number {
            /* If input is working on current scan, we ordinarily want it to
             * have completed the current row.  But if input scan is DC,
             * we want it to keep one row ahead so that next block row's DC
             * values are up to date.
             */
            let mut delta: crate::jmorecfg_h::JDIMENSION =
                if (*cinfo).Ss == 0i32 { 1i32 } else { 0i32 } as crate::jmorecfg_h::JDIMENSION;
            if (*cinfo).input_iMCU_row >  (*cinfo).output_iMCU_row + delta {
                break;
            }
        }
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == crate::jpeglib_h::JPEG_SUSPENDED
        {
            return crate::jpeglib_h::JPEG_SUSPENDED;
        }
    }
    /* OK, output from the virtual arrays. */
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Don't bother to IDCT an uninteresting component. */
        if !((*compptr).component_needed == 0) {
            /* Count non-dummy DCT block rows in this iMCU row. */
            if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor; /* this and next iMCU row */
                access_rows = block_rows * 2i32;
                last_row = crate::jmorecfg_h::FALSE
            } else {
                /* NB: can't use last_row_height here; it is input-side-dependent! */
                block_rows = ( (*compptr)
                    .height_in_blocks % (*compptr).v_samp_factor as libc::c_uint)
                    as libc::c_int; /* this iMCU row only */
                if block_rows == 0i32 {
                    block_rows = (*compptr).v_samp_factor
                }
                access_rows = block_rows;
                last_row = crate::jmorecfg_h::TRUE
            }
            /* Align the virtual buffer for this component. */
            if (*cinfo).output_iMCU_row > 0i32 as libc::c_uint {
                access_rows += (*compptr).v_samp_factor; /* prior iMCU row too */
                buffer = Some(
                    (*(*cinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    (
                    (*cinfo)
                        .output_iMCU_row - 1i32 as libc::c_uint) *
    (*compptr).v_samp_factor as libc::c_uint,
                    access_rows as crate::jmorecfg_h::JDIMENSION,
                    crate::jmorecfg_h::FALSE,
                ); /* point to current iMCU row */
                buffer = buffer.offset((*compptr).v_samp_factor as isize);
                first_row = crate::jmorecfg_h::FALSE
            } else {
                buffer = Some(
                    (*(*cinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    0i32 as crate::jmorecfg_h::JDIMENSION,
                    access_rows as crate::jmorecfg_h::JDIMENSION,
                    crate::jmorecfg_h::FALSE,
                );
                first_row = crate::jmorecfg_h::TRUE
            }
            /* Fetch component-dependent info */
            coef_bits = (*coef)
                .coef_bits_latch
                .offset((ci * crate::src::jdcoefct::SAVED_COEFS) as isize);
            quanttbl = (*compptr).quant_table;
            Q00 = (*quanttbl).quantval[0] as crate::jpegint_h::JLONG;
            Q01 = (*quanttbl).quantval[Q01_POS as usize] as crate::jpegint_h::JLONG;
            Q10 = (*quanttbl).quantval[Q10_POS as usize] as crate::jpegint_h::JLONG;
            Q20 = (*quanttbl).quantval[Q20_POS as usize] as crate::jpegint_h::JLONG;
            Q11 = (*quanttbl).quantval[Q11_POS as usize] as crate::jpegint_h::JLONG;
            Q02 = (*quanttbl).quantval[Q02_POS as usize] as crate::jpegint_h::JLONG;
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            /* Loop over all DCT blocks to be processed. */
            block_row = 0i32;
            while block_row < block_rows {
                buffer_ptr = (*buffer.offset(block_row as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                if first_row != 0 && block_row == 0i32 {
                    prev_block_row = buffer_ptr
                } else {
                    prev_block_row = *buffer.offset((block_row - 1i32) as isize)
                }
                if last_row != 0 && block_row == block_rows - 1i32 {
                    next_block_row = buffer_ptr
                } else {
                    next_block_row = *buffer.offset((block_row + 1i32) as isize)
                }
                /* We fetch the surrounding DC values using a sliding-register approach.
                 * Initialize all nine here so as to do the right thing on narrow pics.
                 */
                DC3 = (*prev_block_row.offset(0))[0] as libc::c_int;
                DC2 = DC3;
                DC1 = DC2;
                DC6 = (*buffer_ptr.offset(0))[0] as libc::c_int;
                DC5 = DC6;
                DC4 = DC5;
                DC9 = (*next_block_row.offset(0))[0] as libc::c_int;
                DC8 = DC9;
                DC7 = DC8;
                output_col = 0i32 as crate::jmorecfg_h::JDIMENSION;
                last_block_column =  (*compptr)
                    .width_in_blocks - 1i32 as libc::c_uint;
                block_num = (*(*cinfo).master).first_MCU_col[ci as usize];
                while block_num <= (*(*cinfo).master).last_MCU_col[ci as usize] {
                    /* Fetch current DCT block into workspace so we can modify it. */
                    crate::jpegint_h::jcopy_block_row(
                        buffer_ptr,
                        workspace as crate::jpeglib_h::JBLOCKROW,
                        1i32 as crate::jmorecfg_h::JDIMENSION,
                    );
                    /* Update DC values */
                    if block_num < last_block_column {
                        DC3 = (*prev_block_row.offset(1))[0] as libc::c_int;
                        DC6 = (*buffer_ptr.offset(1))[0] as libc::c_int;
                        DC9 = (*next_block_row.offset(1))[0] as libc::c_int
                    }
                    /* Compute coefficient estimates per K.8.
                     * An estimate is applied only if coefficient is still zero,
                     * and is not known to be fully accurate.
                     */
                    /* AC01 */
                    Al = *coef_bits.offset(1);
                    if Al != 0i32 && *workspace.offset(1) as libc::c_int == 0i32 {
                        num = 36i32 as libc::c_long * Q00 * (DC4 - DC6) as libc::c_long;
                        if num >= 0i32 as libc::c_long {
                            pred = (((Q01 << 7i32) + num) / (Q01 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q01 << 7i32) - num) / (Q01 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(1) = pred as crate::jmorecfg_h::JCOEF
                    }
                    /* AC10 */
                    Al = *coef_bits.offset(2);
                    if Al != 0i32 && *workspace.offset(8) as libc::c_int == 0i32 {
                        num = 36i32 as libc::c_long * Q00 * (DC2 - DC8) as libc::c_long;
                        if num >= 0i32 as libc::c_long {
                            pred = (((Q10 << 7i32) + num) / (Q10 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q10 << 7i32) - num) / (Q10 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(8) = pred as crate::jmorecfg_h::JCOEF
                    }
                    /* AC20 */
                    Al = *coef_bits.offset(3);
                    if Al != 0i32 && *workspace.offset(16) as libc::c_int == 0i32 {
                        num = 9i32 as libc::c_long * Q00 * (DC2 + DC8 - 2i32 * DC5) as libc::c_long;
                        if num >= 0i32 as libc::c_long {
                            pred = (((Q20 << 7i32) + num) / (Q20 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q20 << 7i32) - num) / (Q20 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(16) = pred as crate::jmorecfg_h::JCOEF
                    }
                    /* AC11 */
                    Al = *coef_bits.offset(4);
                    if Al != 0i32 && *workspace.offset(9) as libc::c_int == 0i32 {
                        num = 5i32 as libc::c_long * Q00 * (DC1 - DC3 - DC7 + DC9) as libc::c_long;
                        if num >= 0i32 as libc::c_long {
                            pred = (((Q11 << 7i32) + num) / (Q11 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q11 << 7i32) - num) / (Q11 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(9) = pred as crate::jmorecfg_h::JCOEF
                    }
                    /* AC02 */
                    Al = *coef_bits.offset(5);
                    if Al != 0i32 && *workspace.offset(2) as libc::c_int == 0i32 {
                        num = 9i32 as libc::c_long * Q00 * (DC4 + DC6 - 2i32 * DC5) as libc::c_long;
                        if num >= 0i32 as libc::c_long {
                            pred = (((Q02 << 7i32) + num) / (Q02 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                        } else {
                            pred = (((Q02 << 7i32) - num) / (Q02 << 8i32)) as libc::c_int;
                            if Al > 0i32 && pred >= 1i32 << Al {
                                pred = (1i32 << Al) - 1i32
                            }
                            pred = -pred
                        }
                        *workspace.offset(2) = pred as crate::jmorecfg_h::JCOEF
                    }
                    /* OK, do the IDCT */
                    Some(inverse_DCT.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        cinfo, compptr, workspace, output_ptr, output_col,
                    );
                    /* Advance for next column */
                    DC1 = DC2;
                    DC2 = DC3;
                    DC4 = DC5;
                    DC5 = DC6;
                    DC7 = DC8;
                    DC8 = DC9;
                    buffer_ptr = buffer_ptr.offset(1);
                    prev_block_row = prev_block_row.offset(1);
                    next_block_row = next_block_row.offset(1);
                    output_col = (output_col as libc::c_uint + (*compptr).DCT_scaled_size as libc::c_uint)
                        as crate::jmorecfg_h::JDIMENSION
                        as crate::jmorecfg_h::JDIMENSION;
                    block_num =  block_num + 1
                }
                output_ptr = output_ptr.offset((*compptr).DCT_scaled_size as isize);
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
    (*cinfo).output_iMCU_row =  (*cinfo).output_iMCU_row + 1;
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return crate::jpeglib_h::JPEG_ROW_COMPLETED;
    }
    return crate::jpeglib_h::JPEG_SCAN_COMPLETED;
}
/*
 * jpegint.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 1997-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015-2016, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file provides common declarations for the various JPEG modules.
 * These declarations are considered internal to the JPEG library; most
 * applications using the library shouldn't need to include this file.
 */
/* Declarations for both compression & decompression */
/* Operating modes for buffer controllers */
/* Plain stripwise operation */
/* Remaining modes require a full-image buffer to have been created */
/* Run source subobject only, save output */
/* Run dest subobject only, using saved data */
/* Run both subobjects, save output */
/* Requantize */
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */
/* after create_compress */
/* start_compress done, write_scanlines OK */
/* start_compress done, write_raw_data OK */
/* jpeg_write_coefficients done */
/* after create_decompress */
/* reading header markers, no SOS yet */
/* found SOS, ready for start_decompress */
/* reading multiscan file in start_decompress*/
/* performing dummy pass for 2-pass quant */
/* start_decompress done, read_scanlines OK */
/* start_decompress done, read_raw_data OK */
/* expecting jpeg_start_output */
/* looking for SOS/EOI in jpeg_finish_output */
/* reading file in jpeg_read_coefficients */
/* looking for EOI in jpeg_finish_decompress */
/* JLONG must hold at least signed 32-bit values. */
/*
 * Left shift macro that handles a negative operand without causing any
 * sanitizer warnings
 */
/* Declarations for compression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True if pass_startup must be called */
/* True during last pass */
/* Extension parameters */
/* TRUE=optimize progressive coding scans */
/* TRUE=use trellis quantization */
/* TRUE=use trellis quant for DC coefficient */
/* TRUE=optimize for sequences of EOB */
/* TRUE=use lambda weighting table */
/* TRUE=use scans in trellis optimization */
/* TRUE=currently doing trellis-related passes [not exposed] */
/* TRUE=optimize quant table in trellis loop */
/* TRUE=preprocess input to reduce ringing of edges on white background */
/* compression profile */
/* DC scan optimization mode */
/* Quantization table master index */
/* splitting point for frequency in trellis quantization */
/* number of trellis loops */
/* # of entries in scan_info array pertaining to luma (used when optimize_scans is TRUE */
/* maximum value of Al tested when optimizing scans (luma) */
/* maximum value of Al tested when optimizing scans (chroma) */
/* Main buffer control (downsampled-data buffer) */
/* Compression preprocessing (downsampling input buffer control) */
/* Coefficient buffer control */
/* Colorspace conversion */
/* Downsampling */
/* TRUE if need rows above & below */
/* Forward DCT (also controls coefficient quantization) */
/* perhaps this should be an array??? */
/* Entropy encoding */
/* Marker writing */
/* These routines are exported to allow insertion of extra markers */
/* Probably only COM and APPn markers should be written this way */
/* Declarations for decompression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True during 1st pass for 2-pass quant */
/* Partial decompression variables */
/* Input control module */
/* State variables made visible to other modules */
/* True if file has multiple scans */
/* True when EOI has been consumed */
/* Main buffer control (downsampled-data buffer) */
/* Coefficient buffer control */
/* Pointer to array of coefficient virtual arrays, or NULL if none */
/* Decompression postprocessing (color quantization buffer control) */
/* Marker reading & parsing */
/* Read markers until SOS or EOI.
 * Returns same codes as are defined for jpeg_consume_input:
 * JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 */
/* Read a restart marker --- exported for use by entropy decoder only */
/* State of marker reader --- nominally internal, but applications
 * supplying COM or APPn handlers might like to know the state.
 */
/* found SOI? */
/* found SOF? */
/* next restart number expected (0-7) */
/* # of bytes skipped looking for a marker */
/* Entropy decoding */
/* This is here to share code between baseline and progressive decoders; */
/* other modules probably should not use it */
/* set TRUE after emitting warning */
/* Inverse DCT (also performs dequantization) */
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
/* BLOCK_SMOOTHING_SUPPORTED */
/*
 * Initialize coefficient buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_d_coef_controller(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut need_full_buffer: crate::jmorecfg_h::boolean,
) {
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        ::std::ptr::null_mut::< crate::src::jdcoefct::my_coef_controller>();
    coef = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<crate::src::jdcoefct::my_coef_controller>() as libc::c_ulong,
    ) as crate::src::jdcoefct::my_coef_ptr;
    (*cinfo).coef = coef as *mut crate::jpeglib_h::jpeg_d_coef_controller;
    (*coef).pub_0.start_input_pass =
        Some(start_input_pass as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ());
    (*coef).pub_0.start_output_pass = Some(
        start_output_pass as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*coef).coef_bits_latch = crate::stddef_h::NULL as *mut libc::c_int;
    /* Create the coefficient buffer. */
    if need_full_buffer != 0 {
        /* Allocate a full-image virtual array for each component, */
        /* padded to a multiple of samp_factor DCT blocks in each direction. */
        /* Note we ask for a pre-zeroed array. */
        let mut ci: libc::c_int = 0;
        let mut access_rows: libc::c_int = 0;
        let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
            ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            access_rows = (*compptr).v_samp_factor;
            /* If block smoothing could be used, need a bigger window */
            if (*cinfo).progressive_mode != 0 {
                access_rows *= 3i32
            }
            (*coef).whole_image[ci as usize] = Some(
                (*(*cinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                crate::jpeglib_h::JPOOL_IMAGE,
                crate::jmorecfg_h::TRUE,
                crate::jpegint_h::jround_up(
                    (*compptr).width_in_blocks as libc::c_long,
                    (*compptr).h_samp_factor as libc::c_long,
                ) as crate::jmorecfg_h::JDIMENSION,
                crate::jpegint_h::jround_up(
                    (*compptr).height_in_blocks as libc::c_long,
                    (*compptr).v_samp_factor as libc::c_long,
                ) as crate::jmorecfg_h::JDIMENSION,
                access_rows as crate::jmorecfg_h::JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
        (*coef).pub_0.consume_data = Some(
            consume_data
                as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int,
        );
        (*coef).pub_0.decompress_data = Some(
            decompress_data
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPIMAGE,
                ) -> libc::c_int,
        );
        (*coef).pub_0.coef_arrays = (*coef).whole_image.as_mut_ptr()
    } else {
        /* We only need a single-MCU buffer. */
        let mut buffer: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
        let mut i: libc::c_int = 0;
        buffer = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            crate::jpeglib_h::D_MAX_BLOCKS_IN_MCU as libc::c_ulong *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
        ) as crate::jpeglib_h::JBLOCKROW;
        i = 0i32;
        while i < crate::jpeglib_h::D_MAX_BLOCKS_IN_MCU {
            (*coef).MCU_buffer[i as usize] = buffer.offset(i as isize);
            i += 1
        }
        (*coef).pub_0.consume_data = Some(
            dummy_consume_data
                as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int,
        );
        (*coef).pub_0.decompress_data = Some(
            decompress_onepass
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPIMAGE,
                ) -> libc::c_int,
        );
        (*coef).pub_0.coef_arrays = crate::stddef_h::NULL as *mut crate::jpeglib_h::jvirt_barray_ptr
    }
    /* Allocate the workspace buffer */
    (*coef).workspace = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong *
    crate::jpeglib_h::DCTSIZE2 as libc::c_ulong,
    ) as *mut crate::jmorecfg_h::JCOEF;
}
